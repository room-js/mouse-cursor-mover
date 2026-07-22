use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIcon, TrayIconBuilder,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

// ── macOS: CoreGraphics mouse control ────────────────────────────────────────

#[cfg(target_os = "macos")]
mod mouse {
    use std::ffi::c_void;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGPoint {
        x: f64,
        y: f64,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGSize {
        width: f64,
        height: f64,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGRect {
        origin: CGPoint,
        size: CGSize,
    }

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGEventCreateMouseEvent(
            source: *const c_void,
            mouse_type: u32,
            cursor_position: CGPoint,
            mouse_button: u32,
        ) -> *mut c_void;
        fn CGEventPost(tap_location: u32, event: *mut c_void);
        fn CGMainDisplayID() -> u32;
        fn CGDisplayBounds(display: u32) -> CGRect;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFRelease(cf: *const c_void);
    }

    /// Move the mouse cursor to an absolute screen position.
    pub fn move_cursor(x: f64, y: f64) {
        unsafe {
            let event = CGEventCreateMouseEvent(
                std::ptr::null(),
                5, // kCGEventMouseMoved
                CGPoint { x, y },
                0, // kCGMouseButtonLeft
            );
            if !event.is_null() {
                CGEventPost(0, event); // kCGHIDEventTap = 0
                CFRelease(event);
            }
        }
    }

    /// Return the width and height of the main display in points.
    pub fn screen_size() -> (f64, f64) {
        unsafe {
            let id = CGMainDisplayID();
            let b = CGDisplayBounds(id);
            (b.size.width, b.size.height)
        }
    }
}

// ── Minimal LCG pseudo-random number generator ───────────────────────────────

struct Lcg(u64);

impl Lcg {
    /// Seed from the current wall-clock nanoseconds.
    fn seeded() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        Self(seed ^ 0x517cc1b727220a95)
    }

    /// Return a uniform f64 in [0, 1).
    fn next_f64(&mut self) -> f64 {
        self.0 = self.0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

// ── Tray icon images ──────────────────────────────────────────────────────────

/// Build a 22×22 RGBA tray icon.
///
/// * `running = true`  → solid green filled circle  
/// * `running = false` → gray outline ring
fn make_icon(running: bool) -> tray_icon::Icon {
    const N: usize = 22;
    let mut px = vec![0u8; N * N * 4];
    let c = N as f64 / 2.0;
    let r_outer = c - 1.5;
    let r_inner = r_outer - 2.5;

    for row in 0..N {
        for col in 0..N {
            let dx = col as f64 + 0.5 - c;
            let dy = row as f64 + 0.5 - c;
            let d = (dx * dx + dy * dy).sqrt();
            let i = (row * N + col) * 4;

            let (r, g, b, a): (u8, u8, u8, u8) = if running {
                if d <= r_outer {
                    (34, 197, 94, 255) // green-500
                } else {
                    (0, 0, 0, 0)
                }
            } else {
                if d <= r_outer && d >= r_inner {
                    (156, 163, 175, 255) // gray-400 ring
                } else {
                    (0, 0, 0, 0)
                }
            };

            px[i..i + 4].copy_from_slice(&[r, g, b, a]);
        }
    }

    tray_icon::Icon::from_rgba(px, N as u32, N as u32).unwrap()
}

// ── App state ─────────────────────────────────────────────────────────────────

struct App {
    tray: Option<TrayIcon>,
    start_item: Option<MenuItem>,
    stop_item: Option<MenuItem>,
    quit_item: Option<MenuItem>,
    running: Arc<Mutex<bool>>,
}

impl App {
    fn new() -> Self {
        Self {
            tray: None,
            start_item: None,
            stop_item: None,
            quit_item: None,
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Spawn a background thread that moves the cursor every 5 seconds
    /// while the `running` flag is true.
    fn spawn_mover(&self) {
        let flag = self.running.clone();
        thread::spawn(move || {
            let mut rng = Lcg::seeded();

            #[cfg(target_os = "macos")]
            let (sw, sh) = mouse::screen_size();
            #[cfg(not(target_os = "macos"))]
            let (sw, sh) = (1920.0_f64, 1080.0_f64);

            loop {
                thread::sleep(Duration::from_secs(5));

                // Stop if the flag was cleared while we were sleeping.
                if !*flag.lock().unwrap() {
                    break;
                }

                let x = rng.next_f64() * sw;
                let y = rng.next_f64() * sh;

                #[cfg(target_os = "macos")]
                mouse::move_cursor(x, y);

                // On non-macOS platforms (dev/test builds) just suppress the
                // unused-variable warning.
                #[cfg(not(target_os = "macos"))]
                let _ = (x, y);
            }
        });
    }

    /// Update the running flag, tray icon, tooltip, and menu-item states.
    fn set_running(&mut self, running: bool) {
        *self.running.lock().unwrap() = running;

        if let Some(item) = &self.start_item {
            item.set_enabled(!running);
        }
        if let Some(item) = &self.stop_item {
            item.set_enabled(running);
        }
        if let Some(tray) = &self.tray {
            let _ = tray.set_icon(Some(make_icon(running)));
            let tooltip = if running {
                "Mouse Cursor Mover \u{2013} Running"
            } else {
                "Mouse Cursor Mover \u{2013} Stopped"
            };
            let _ = tray.set_tooltip(Some(tooltip));
        }
    }
}

// ── winit ApplicationHandler ──────────────────────────────────────────────────

impl ApplicationHandler for App {
    /// Called once when the event loop starts (desktop) or resumes (mobile).
    /// Create the tray icon here so the macOS run loop is already running.
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        if self.tray.is_some() {
            return; // already initialised
        }

        let start = MenuItem::new("Start", true, None);
        let stop = MenuItem::new("Stop", false, None);
        let quit = MenuItem::new("Quit", true, None);

        let menu = Menu::new();
        let _ = menu.append_items(&[&start, &stop, &quit]);

        self.tray = Some(
            TrayIconBuilder::new()
                .with_menu(Box::new(menu))
                .with_icon(make_icon(false))
                .with_tooltip("Mouse Cursor Mover \u{2013} Stopped")
                .build()
                .unwrap(),
        );

        // Store menu items so we can compare IDs and toggle enabled state.
        self.start_item = Some(start);
        self.stop_item = Some(stop);
        self.quit_item = Some(quit);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
        // No windows in this app.
    }

    /// Poll for menu events each iteration and handle them.
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        while let Ok(event) = MenuEvent::receiver().try_recv() {
            let is_start = self
                .start_item
                .as_ref()
                .map_or(false, |i| i.id() == &event.id);
            let is_stop = self
                .stop_item
                .as_ref()
                .map_or(false, |i| i.id() == &event.id);
            let is_quit = self
                .quit_item
                .as_ref()
                .map_or(false, |i| i.id() == &event.id);

            if is_start {
                self.set_running(true);
                self.spawn_mover();
            } else if is_stop {
                self.set_running(false);
            } else if is_quit {
                event_loop.exit();
            }
        }

        // Wake up every 100 ms to stay responsive without burning CPU.
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            Instant::now() + Duration::from_millis(100),
        ));
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    // On macOS: hide the app from the Dock, disable the default application
    // menu bar, and make it an Accessory app (menu-bar only).
    #[cfg(target_os = "macos")]
    let event_loop = {
        use winit::platform::macos::{ActivationPolicy, EventLoopBuilderExtMacOS};
        let mut builder = EventLoop::builder();
        builder
            .with_activation_policy(ActivationPolicy::Accessory)
            .with_default_menu(false);
        builder.build().unwrap()
    };

    #[cfg(not(target_os = "macos"))]
    let event_loop = EventLoop::new().unwrap();

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
