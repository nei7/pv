use crate::cli;
use crate::errors::PasswordError;
use crate::pass;
use dialoguer::{theme::ColorfulTheme, Password};

pub fn exec(args: cli::AddCommand, store: &mut pass::PasswordStore) -> Result<(), i32> {
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
