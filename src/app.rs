use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use global_hotkey::{
    hotkey::{Code as HotKeyCode, HotKey, Modifiers as HotKeyModifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use tray_icon::{
    menu::{
        accelerator::{Accelerator, Code, Modifiers},
        Menu, MenuEvent, MenuItem,
    },
    TrayIcon, TrayIconBuilder,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

use crate::icons::tray_icon_for_state;
use crate::mover;
use crate::support;

struct App {
    tray: Option<TrayIcon>,
    toggle_item: Option<MenuItem>,
    support_item: Option<MenuItem>,
    quit_item: Option<MenuItem>,
    hotkey_manager: Option<GlobalHotKeyManager>,
    hotkey: Option<HotKey>,
    last_toggle_at: Option<Instant>,
    running: Arc<Mutex<bool>>,
}

impl App {
    fn new() -> Self {
        Self {
            tray: None,
            toggle_item: None,
            support_item: None,
            quit_item: None,
            hotkey_manager: None,
            hotkey: None,
            last_toggle_at: None,
            running: Arc::new(Mutex::new(false)),
        }
    }

    fn ensure_hotkey_registered(&mut self) {
        if self.hotkey_manager.is_some() {
            return;
        }

        let manager = match GlobalHotKeyManager::new() {
            Ok(mgr) => mgr,
            Err(err) => {
                eprintln!("Failed to create global hotkey manager: {err}");
                return;
            }
        };

        let hotkey = HotKey::new(
            Some(HotKeyModifiers::SUPER | HotKeyModifiers::ALT),
            HotKeyCode::KeyS,
        );

        if let Err(err) = manager.register(hotkey) {
            eprintln!("Failed to register global hotkey Cmd+Option+S: {err}");
            return;
        }

        self.hotkey_manager = Some(manager);
        self.hotkey = Some(hotkey);
    }

    fn set_running(&mut self, running: bool) {
        *self.running.lock().unwrap() = running;

        if let Some(item) = &self.toggle_item {
            if running {
                item.set_text("Stop");
            } else {
                item.set_text("Start");
            }
        }

        if let Some(tray) = &self.tray {
            let _ = tray.set_icon_with_as_template(Some(tray_icon_for_state(running)), true);
            let tooltip = if running {
                "Mouse Cursor Mover \u{2013} Running"
            } else {
                "Mouse Cursor Mover \u{2013} Stopped"
            };
            let _ = tray.set_tooltip(Some(tooltip));
        }
    }

    fn toggle_running(&mut self) {
        let next = !*self.running.lock().unwrap();
        self.set_running(next);
        if next {
            mover::spawn_mover(self.running.clone());
        }
    }

    fn toggle_running_debounced(&mut self) {
        let now = Instant::now();
        let debounce_window = Duration::from_millis(250);

        if self
            .last_toggle_at
            .is_some_and(|last| now.duration_since(last) < debounce_window)
        {
            return;
        }

        self.last_toggle_at = Some(now);
        self.toggle_running();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        if self.tray.is_some() {
            return;
        }

        self.ensure_hotkey_registered();

        let toggle = MenuItem::new(
            "Start",
            true,
            Some(Accelerator::new(
                Some(Modifiers::SUPER | Modifiers::ALT),
                Code::KeyS,
            )),
        );
        let support = MenuItem::new("Support project", true, None);
        let quit = MenuItem::new("Quit", true, None);

        let menu = Menu::new();
        let _ = menu.append_items(&[&toggle, &support, &quit]);

        self.tray = Some(
            TrayIconBuilder::new()
                .with_menu(Box::new(menu))
                .with_icon(tray_icon_for_state(false))
                .with_icon_as_template(true)
                .with_tooltip("Mouse Cursor Mover \u{2013} Stopped")
                .build()
                .unwrap(),
        );

        self.toggle_item = Some(toggle);
        self.support_item = Some(support);
        self.quit_item = Some(quit);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.state == HotKeyState::Pressed
                && self
                    .hotkey
                    .as_ref()
                    .is_some_and(|hotkey| hotkey.id() == event.id)
            {
                self.toggle_running_debounced();
            }
        }

        while let Ok(event) = MenuEvent::receiver().try_recv() {
            let is_toggle = self
                .toggle_item
                .as_ref()
                .is_some_and(|i| i.id() == &event.id);
            let is_support = self
                .support_item
                .as_ref()
                .is_some_and(|i| i.id() == &event.id);
            let is_quit = self.quit_item.as_ref().is_some_and(|i| i.id() == &event.id);

            if is_toggle {
                self.toggle_running_debounced();
            } else if is_support {
                support::open_support_url();
            } else if is_quit {
                event_loop.exit();
            }
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(
            Instant::now() + Duration::from_millis(100),
        ));
    }
}

pub fn run() {
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
