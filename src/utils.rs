use std::process::Command;

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
