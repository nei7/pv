use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, scrypt, symmetriccipher};
use rand_core::{OsRng, RngCore};
use std::io::Result as IoResult;
use std::ops::DerefMut;

pub fn aes_encrypt(
    data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor =
        aes::cbc_encryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

        for b in write_buffer.take_read_buffer().take_remaining() {
            final_result.push(*b);
        }

        match result {
            BufferResult::BufferOverflow => {}
            BufferResult::BufferUnderflow => break,
        }
    }

    Ok(final_result)
}

pub fn aes_decrypt(
    encrypted_data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor =
        aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;

        for b in write_buffer.take_read_buffer().take_remaining() {
            final_result.push(*b);
        }

        match result {
            BufferResult::BufferOverflow => {}
            BufferResult::BufferUnderflow => break,
        }
    }

    Ok(final_result)
}

pub fn generate_encryption_key(master_password: &str, salt: &[u8]) -> Vec<u8> {
    let mut output = Vec::<u8>::with_capacity(32);
    for _ in 0..32 {
        output.push(0u8);
    }

    scrypt::scrypt(
        master_password.as_bytes(),
        &salt,
        &scrypt::ScryptParams::new(12, 8, 1),
        output.deref_mut(),
    );

    output
}

pub fn generate_random_iv() -> IoResult<[u8; 16]> {
    let mut bytes: [u8; 16] = [0; 16];
    OsRng.fill_bytes(&mut bytes);
    Ok(bytes)
}

pub fn generate_random_salt() -> IoResult<[u8; 32]> {
    let mut bytes: [u8; 32] = [0; 32];
    OsRng.fill_bytes(&mut bytes);
    Ok(bytes)
}

#[cfg(test)]
mod test {
    use crate::crypto;

    #[test]
    fn test_generate_random_iv() {
        assert_eq!(crypto::generate_random_iv().unwrap().len(), 16);
    }

    #[test]
    fn test_generate_random_salt() {
        assert_eq!(crypto::generate_random_salt().unwrap().len(), 32);
    }

    #[test]
    fn test_generate_encryption_key() {
        assert_eq!(
            crypto::generate_encryption_key("hi", &crypto::generate_random_salt().unwrap()).len(),
            32
        )
    }
}
