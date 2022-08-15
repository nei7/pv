use crate::{cli, pass};

pub fn exec(args: cli::NameParam, store: &mut pass::PasswordStore) -> Result<(), i32> {
    match store.delete_password(args.name.as_str()) {
        Ok(_) => {
            println!("Password deleted successfully");
            return Ok(());
        }
        Err(e) => {
            println!("{}", e);
            return Err(1);
        }
    }
}
