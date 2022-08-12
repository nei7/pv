use std::fmt::Display;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum PasswordError {
    DecryptionError,
    EncryptionError,
    NotFoundError,
    EmptyPasswordError,
    InvalidJson,
    Io(IoError),
}

impl From<IoError> for PasswordError {
    fn from(err: IoError) -> PasswordError {
        PasswordError::Io(err)
    }
}

impl Display for PasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &PasswordError::DecryptionError => f.write_str("decryption error"),
            &PasswordError::EmptyPasswordError => f.write_str("password can't be empty"),
            &PasswordError::EncryptionError => f.write_str("encryption error"),
            &PasswordError::InvalidJson => f.write_str("invalid json"),
            &PasswordError::Io(ref e) => e.fmt(f),
            &PasswordError::NotFoundError => f.write_str("password not found"),
        }
    }
}
