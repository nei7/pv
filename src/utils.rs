use std::process::Command;

#[cfg(any(windows, target_os = "macos"))]
pub fn copy_to_clipboard(s: &SafeString) -> Result<(), ()> {
    use clipboard::ClipboardContext;
    use clipboard::ClipboardProvider;

    let mut context: ClipboardContext = ClipboardProvider::new().map_err(|_| ())?;
    context.set_contents(s.deref().to_owned()).map_err(|_| ())?;
    Ok(())
}

#[cfg(all(unix, not(target_os = "macos")))]
pub fn copy_to_clipboard(password: &str) -> bool {
    let shell = format!(
        "echo  '{}' | {} -sel clip 2> /dev/null",
        password, "/bin/xclip"
    );

    match Command::new("sh").args(&["-c", shell.as_str()]).status() {
        Ok(_) => true,
        Err(_) => false,
    }
}
