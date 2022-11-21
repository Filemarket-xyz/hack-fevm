use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
    Nonce, // Or `Aes128Gcm`
};
use core::str;
use fvm_ipld_encoding::tuple::Deserialize_tuple;
use fvm_ipld_encoding::RawBytes;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::{PaddingScheme, RsaPrivateKey, RsaPublicKey};

pub enum DecriptError {
    DecriptPub,
    DecriptPrivate,
    WrongPub,
    DecriptKey,
    DecriptionFile,
    NonUtf8Content,
    SerializeDecriptData,
}

pub type EncriptedFileContent = String;
pub type EncriptedPassword = Vec<u8>;
pub type SalePrivateKey = String;
pub type SalePublicKey = String;

#[derive(Debug, Deserialize_tuple)]
struct EncriptionParams {
    key: String,
    nonce: String,
}

pub fn decript(
    private: &SalePrivateKey,
    public: &SalePublicKey,
    encripted_password: &EncriptedPassword,
    encripted_file_content: EncriptedFileContent,
) -> Result<String, DecriptError> {
    let public_key =
        RsaPublicKey::from_public_key_pem(public).map_err(|_x| DecriptError::DecriptPub)?;
    let private_key =
        RsaPrivateKey::from_pkcs8_pem(private).map_err(|_x| DecriptError::DecriptPrivate)?;
    if RsaPublicKey::from(&private_key) != public_key {
        return Err(DecriptError::WrongPub);
    }
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let decripted_data = private_key
        .decrypt(padding, encripted_password)
        .map_err(|_x| DecriptError::DecriptKey)?;
    let decripted_data_bytes = RawBytes::new(decripted_data);
    let decripted_data_serialized: EncriptionParams = decripted_data_bytes
        .deserialize()
        .map_err(|_x| DecriptError::SerializeDecriptData)?;
    decript_content(encripted_file_content, decripted_data_serialized)
}

fn decript_content(
    s: EncriptedFileContent,
    params: EncriptionParams,
) -> Result<String, DecriptError> {
    let cipher = Aes256Gcm::new(params.key.as_bytes().into());
    let nonce = Nonce::from_slice(params.nonce.as_bytes()); // 96-bits; unique per message
    let plaintext = cipher
        .decrypt(nonce, s.as_ref())
        .map_err(|_x| DecriptError::DecriptionFile)?;
    str::from_utf8(&plaintext)
        .map_err(|_| DecriptError::NonUtf8Content)
        .map(|x| x.to_string())
}
