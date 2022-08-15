use crate::cli;
use crate::pass;
use crate::utils;

pub fn exec(args: cli::GetCommand, store: &mut pass::PasswordStore) -> Result<(), i32> {
    let password = match store.get_password(&args.name) {
        Some(pass) => pass,
        None => return Err(1),
    };

    if !utils::copy_to_clipboard(&password.password) {
        println!("{}", password.password)
    }

    println!("password copied to clipoard");

    Ok(())
}
