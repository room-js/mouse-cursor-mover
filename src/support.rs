use std::process::Command;

const SUPPORT_URL: &str = "https://buymeacoffee.com/roomjs";

pub fn open_support_url() {
    #[cfg(target_os = "macos")]
    let result = Command::new("open").arg(SUPPORT_URL).spawn();

    #[cfg(target_os = "linux")]
    let result = Command::new("xdg-open").arg(SUPPORT_URL).spawn();

    #[cfg(target_os = "windows")]
    let result = Command::new("cmd")
        .args(["/C", "start", "", SUPPORT_URL])
        .spawn();

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    let result: std::io::Result<std::process::Child> = Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "Opening URLs is not supported on this platform",
    ));

    if let Err(err) = result {
        eprintln!("Failed to open support URL: {err}");
    }
}
