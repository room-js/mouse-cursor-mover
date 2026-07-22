use std::sync::OnceLock;

use tray_icon::Icon as TrayIconImage;

const RUNNING_ICON_PNG_2X: &[u8] = include_bytes!("../assets/icon-running@2x.png");
const STOPPED_ICON_PNG_2X: &[u8] = include_bytes!("../assets/icon-stopped@2x.png");

fn required_tray_icon_from_bytes(bytes: &[u8], state_label: &str) -> TrayIconImage {
    let img = image::load_from_memory(bytes).unwrap_or_else(|err| {
        panic!(
            "Failed to decode embedded {} tray icon: {}",
            state_label, err
        )
    });
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    TrayIconImage::from_rgba(rgba.into_raw(), w, h).unwrap_or_else(|err| {
        panic!(
            "Failed to instantiate embedded {} tray icon: {}",
            state_label, err
        )
    })
}

pub fn tray_icon_for_state(running: bool) -> TrayIconImage {
    static RUNNING_ICON: OnceLock<TrayIconImage> = OnceLock::new();
    static STOPPED_ICON: OnceLock<TrayIconImage> = OnceLock::new();

    if running {
        RUNNING_ICON.get_or_init(|| required_tray_icon_from_bytes(RUNNING_ICON_PNG_2X, "running"))
    } else {
        STOPPED_ICON.get_or_init(|| required_tray_icon_from_bytes(STOPPED_ICON_PNG_2X, "stopped"))
    }
    .clone()
}
