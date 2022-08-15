use crate::cli;
use crate::pass;
use dialoguer::{theme::ColorfulTheme, Password};

pub fn exec() -> Result<(), i32> {
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Type your master password: ")
        .with_confirmation("Repeat master password", "Error: passwords don't match")
        .interact()
        .map_err(|_| 1)?;

    let store = match pass::PasswordStore::new(password.as_str()) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {}", e);
            return Err(1);
        }
    };

    let path = cli::get_password_file_path();

    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    options.write(true);
    options.create(true);

    let mut file = options.open(path).map_err(|_| 1)?;

    match store.save_store(&mut file) {
        Ok(_) => println!("Info: your password vault has been successfully initialized"),
        Err(e) => {
            println!("Error: {}", e);
            return Err(1);
        }
    }

    Ok(())
}
