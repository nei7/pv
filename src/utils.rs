use std::process::Command;

#[cfg(all(unix, not(target_os = "macos")))]
pub fn copy_to_clipboard(password: &str) -> Result<(), ()> {
    let shell = format!("printf '%s' {} | {} 2> /dev/null", password, "/bin/xclip");

    Command::new("sh")
        .args(&["-c", shell.as_str()])
        .status()
        .map(|_| ())
        .map_err(|_| ())
}
