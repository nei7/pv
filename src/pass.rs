use crate::crypto::{self, aes_decrypt, aes_encrypt, generate_encryption_key};
use crate::errors::PasswordError;
use serde::{Deserialize, Serialize};
use serde_json::{self, Error};
use std::fs::File;
use std::io::{
    BufReader, Cursor, Error as IoError, ErrorKind as IoKind, Read, Seek, SeekFrom, Write,
};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Clone)]
pub struct Password {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Schema {
    passwords: Vec<Password>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            passwords: Vec::new(),
        }
    }
}

impl Password {
    pub fn new(name: String, password: String) -> Password {
        Password {
            name: name,
            password: password,
        }
    }
}

pub struct PasswordStore {
    key: Vec<u8>,
    salt: [u8; 32],
    schema: Schema,
}

impl PasswordStore {
    pub fn new(master_password: &str) -> Result<PasswordStore, PasswordError> {
        let salt = crypto::generate_random_salt()?;

        let key = crypto::generate_encryption_key(master_password, &salt);

        Ok(PasswordStore {
            key: key,
            salt: salt,
            schema: Schema::new(),
        })
    }

    pub fn save_store(&self, file: &mut File) -> Result<(), PasswordError> {
        let json = serde_json::to_string(&self.schema).map_err(|_| PasswordError::InvalidJson)?;

        let iv = crypto::generate_random_iv()?;

        let encrypted = aes_encrypt(json.as_bytes(), &self.key, &iv)
            .map_err(|_| PasswordError::EncryptionError)?;

        file.seek(SeekFrom::Start(0))
            .and_then(|_| file.set_len(0))?;

        file.write_all(&self.salt)?;
        file.write_all(&iv)?;
        file.write_all(&encrypted.as_ref())?;
        file.sync_all()?;

        Ok(())
    }

    pub fn load_store(
        master_password: String,
        file: &File,
    ) -> Result<PasswordStore, PasswordError> {
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .map_err(|err| PasswordError::Io(err))?;

        let mut reader = Cursor::new(buffer);

        let mut salt: [u8; 32] = [0u8; 32];
        reader.read(&mut salt).and_then(|num_bytes| {
            if num_bytes == 32 {
                Ok(())
            } else {
                Err(IoError::new(IoKind::Other, "unexpected eof"))
            }
        })?;

        let mut iv: [u8; 16] = [0u8; 16];
        reader.read(&mut iv).and_then(|num_bytes| {
            if num_bytes == 16 {
                Ok(())
            } else {
                Err(IoError::new(IoKind::Other, "unexpected eof"))
            }
        })?;

        let mut blob: Vec<u8> = Vec::new();
        reader.read_to_end(&mut blob)?;

        let key = generate_encryption_key(master_password.as_str(), &salt);
        let paswords = match aes_decrypt(blob.deref(), key.as_ref(), &iv) {
            Ok(decrypted) => {
                let encoded = String::from_utf8_lossy(decrypted.as_ref()).into_owned();
                let s: Result<Schema, Error> = serde_json::from_str(encoded.as_str());
                match s {
                    Ok(json) => json.passwords,
                    Err(_) => return Err(PasswordError::InvalidJson),
                }
            }
            Err(_) => return Err(PasswordError::DecryptionError),
        };

        Ok(PasswordStore {
            key: key,
            salt: salt,
            schema: Schema {
                passwords: paswords,
            },
        })
    }

    pub fn add_password(&mut self, password: Password) -> Result<(), PasswordError> {
        self.schema.passwords.push(password);
        Ok(())
    }

    pub fn has_password(&self, name: &str) -> bool {
        self.get_password(name).is_some()
    }

    pub fn get_password(&self, name: &str) -> Option<Password> {
        let password = self
            .schema
            .passwords
            .clone()
            .into_iter()
            .find(|password| password.name == name)?;

        Some(password)
    }

    pub fn get_all_passwords(&self) -> Vec<&Password> {
        self.schema.passwords.iter().collect()
    }

    pub fn delete_password(&mut self, name: &str) -> Result<(), PasswordError> {
        let is_removed = self
            .schema
            .passwords
            .iter()
            .position(|pass| pass.name == name)
            .map(|i| self.schema.passwords.remove(i))
            .is_some();

        if !is_removed {
            return Err(PasswordError::NotFoundError);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::pass::{self, Password};

    #[test]
    fn test_create_password_store() {
        let store = pass::PasswordStore::new("123").unwrap();
        assert_eq!(store.get_all_passwords().len(), 0)
    }

    #[test]
    fn test_add_password() {
        let mut store = pass::PasswordStore::new("123").unwrap();

        let pass = Password::new("e".to_string(), "123".to_string());
        assert_eq!(pass.name, "e");
        assert_eq!(pass.password, "123");

        assert!(store.add_password(pass).is_ok());

        let added_password = store.get_password("e").unwrap();
        assert_eq!(added_password.name, "e");
        assert_eq!(added_password.password, "123");

        assert_eq!(store.get_all_passwords().len(), 1);
    }

    #[test]
    fn test_delete_password() {
        let mut store = pass::PasswordStore::new("123").unwrap();
        let pass = Password::new("e".to_string(), "123".to_string());
        assert!(store.add_password(pass).is_ok());
        assert_eq!(store.get_all_passwords().len(), 1);

        store.delete_password("e").unwrap();

        assert_eq!(store.get_all_passwords().len(), 0);
    }
}
