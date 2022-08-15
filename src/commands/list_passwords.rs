use crate::pass;

pub fn exec(store: &mut pass::PasswordStore) -> Result<(), i32> {
    let passwords = store.get_all_passwords();
    if passwords.is_empty() {
        println!("Currently there are no passwords added");
    } else {
        for (i, password) in passwords.iter().enumerate() {
            println!("{}. {}", i + 1, password.name)
        }
    }

    Ok(())
}
