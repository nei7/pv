use crate::cli;
use crate::pass;
use dialoguer::{theme::ColorfulTheme, Password};
use std::fs::File;
use std::path::PathBuf;

pub fn init() -> Result<(), i32> {
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
    let mut password_file = match File::open(&path) {
        Err(_) => File::create(&path).unwrap(),
        Ok(f) => f,
    };

    match store.save_store(&mut password_file) {
        Ok(_) => println!("Info: your password vault has been successfully initialized"),
        Err(e) => {
            println!("Error: {}", e);
            return Err(1);
        }
    }

    Ok(())
}

pub fn list_passwords(store: &mut pass::PasswordStore) -> Result<(), i32> {}
