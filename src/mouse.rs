#[cfg(target_os = "macos")]
mod imp {
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
        fn CGEventCreate(source: *const c_void) -> *mut c_void;
        fn CGEventGetLocation(event: *mut c_void) -> CGPoint;
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

    pub fn move_cursor(x: f64, y: f64) {
        unsafe {
            let event = CGEventCreateMouseEvent(std::ptr::null(), 5, CGPoint { x, y }, 0);
            if !event.is_null() {
                CGEventPost(0, event);
                CFRelease(event);
            }
        }
    }

    pub fn cursor_position() -> Option<(f64, f64)> {
        unsafe {
            let event = CGEventCreate(std::ptr::null());
            if event.is_null() {
                return None;
            }

            let p = CGEventGetLocation(event);
            CFRelease(event);
            Some((p.x, p.y))
        }
    }

    pub fn screen_size() -> (f64, f64) {
        unsafe {
            let id = CGMainDisplayID();
            let b = CGDisplayBounds(id);
            (b.size.width, b.size.height)
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod imp {
    pub fn move_cursor(_x: f64, _y: f64) {}

    pub fn cursor_position() -> Option<(f64, f64)> {
        None
    }

    pub fn screen_size() -> (f64, f64) {
        (1920.0, 1080.0)
    }
}

pub use imp::{cursor_position, move_cursor, screen_size};
