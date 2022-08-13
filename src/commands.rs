use crate::cli;
use crate::errors::PasswordError;
use crate::pass;
use crate::utils;
use dialoguer::{theme::ColorfulTheme, Password};

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

pub fn add_password(args: cli::AddCommand, store: &mut pass::PasswordStore) -> Result<(), i32> {
    if store.has_password(args.name.as_str()) {
        println!("Error: {}", PasswordError::ConflictError);
        return Err(1);
    }

    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Enter password for {}: ", args.name))
        .with_confirmation("Repeat password", "Error: passwords don't match")
        .interact()
        .map_err(|_| 1)?;

    store
        .add_password(pass::Password::new(args.name, password))
        .map_err(|e| {
            println!("Error: {}", e);
            return 1;
        })
}

pub fn get_password(args: cli::GetCommand, store: &mut pass::PasswordStore) -> Result<(), i32> {
    let password = match store.get_password(&args.name) {
        Some(pass) => pass,
        None => return Err(1),
    };

    if utils::copy_to_clipboard(&password.password).is_err() {
        println!("{}", password.password)
    }

    println!("password copied to clipoard");

    Ok(())
}
