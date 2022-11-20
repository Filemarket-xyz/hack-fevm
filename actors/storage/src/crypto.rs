use rsa::{RsaPublicKey, RsaPrivateKey, PaddingScheme};
use rsa::pkcs8::{DecodePublicKey, DecodePrivateKey};
use core::str;

pub enum DecriptError {
    DecriptPub,
    DecriptPrivate,
    WrongPub,
    DecriptKey,
    NonUtf8Content
}

pub type EncriptedFileContent = Vec<u8>;
pub type EncriptedPassword = Vec<u8>;
pub type SalePrivateKey = String;
pub type SalePublicKey = String;

pub fn decript(private: &SalePrivateKey, public: &SalePublicKey, encripted_password: &EncriptedPassword, encripted_file_content: EncriptedFileContent) -> Result<String, DecriptError> {
    let public_key = RsaPublicKey::from_public_key_pem(public).map_err(|_x| DecriptError::DecriptPub)?;
    let private_key = RsaPrivateKey::from_pkcs8_pem(private).map_err(|_x| DecriptError::DecriptPrivate)?;
    if RsaPublicKey::from(&private_key) != public_key {
        return Err(DecriptError::WrongPub)
    }
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let password = private_key.decrypt(padding, encripted_password).map_err(|_x| DecriptError::DecriptKey)?;
    let decoded_content = xor(encripted_file_content, &password);
    str::from_utf8(&decoded_content).map_err(|_| DecriptError::NonUtf8Content).map(|x| x.to_string())
}

fn xor(s: EncriptedFileContent, key: &EncriptedPassword) -> Vec<u8> {
    let mut b = key.iter().cycle();
    s.into_iter().map(|x| x ^ b.next().unwrap()).collect()
}
