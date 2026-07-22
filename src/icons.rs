use std::{path::Path, sync::OnceLock};

use image::ImageReader;
use tray_icon::Icon as TrayIconImage;

fn load_tray_icon_from_png(path: &Path) -> Option<TrayIconImage> {
    let img = ImageReader::open(path).ok()?.decode().ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    TrayIconImage::from_rgba(rgba.into_raw(), w, h).ok()
}

fn load_first_existing_tray_icon(paths: &[&str]) -> Option<TrayIconImage> {
    for path in paths {
        if let Some(icon) = load_tray_icon_from_png(Path::new(path)) {
            return Some(icon);
        }
    }
    None
}

fn required_tray_icon_from_paths(paths: &[&str], state_label: &str) -> TrayIconImage {
    load_first_existing_tray_icon(paths).unwrap_or_else(|| {
        panic!(
            "Missing or invalid {} tray icon. Expected one of: {}",
            state_label,
            paths.join(", ")
        )
    })
}

pub fn tray_icon_for_state(running: bool) -> TrayIconImage {
    static RUNNING_ICON: OnceLock<TrayIconImage> = OnceLock::new();
    static STOPPED_ICON: OnceLock<TrayIconImage> = OnceLock::new();

    if running {
        RUNNING_ICON.get_or_init(|| {
            required_tray_icon_from_paths(
                &["assets/icon-running@2x.png", "assets/icon-running.png"],
                "running",
            )
        })
    } else {
        STOPPED_ICON.get_or_init(|| {
            required_tray_icon_from_paths(
                &["assets/icon-stopped@2x.png", "assets/icon-stopped.png"],
                "stopped",
            )
        })
    }
    .clone()
}
