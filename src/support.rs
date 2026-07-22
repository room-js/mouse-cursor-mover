use std::process::Command;

const SUPPORT_URL: &str = "https://buymeacoffee.com/roomjs";

pub fn open_support_url() {
    if let Err(err) = Command::new("open").arg(SUPPORT_URL).spawn() {
        eprintln!("Failed to open support URL: {err}");
    }
}
