use crate::cli;
use crate::pass;
use dialoguer::{theme::ColorfulTheme, Password};

pub fn exec(args: cli::NameParam, store: &mut pass::PasswordStore) -> Result<(), i32> {
    if !store.has_password(&args.name) {
        println!("password doesn't exist");
        return Err(1);
    }

    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Type your new password:")
        .with_confirmation("Repeat password", "Error: passwords don't match")
        .interact()
        .map_err(|_| 1)?;

    store
        .change_password(&args.name, &password)
        .map_err(|err| {
            println!("Error: {}", err);
            1
        })?;

    Ok(())
}
