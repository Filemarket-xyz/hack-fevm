#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm,
    Nonce,
};
use core::str;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

type NonceSimple = [u8; 12];
type Secret = [u8; 32];
type SharedSecret = Vec<u8>;

pub const NONCE: &NonceSimple = b"unique nonce";

#[derive(Debug, PartialEq)]
pub struct EncodeFileParams<'a> {
    pub msg: &'a str,
    pub key: &'a Secret,
}

#[derive(Debug, PartialEq)]
pub struct DecodeFileParams<'a> {
    pub ciphertext: Vec<u8>,
    pub key: &'a Secret,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EncriptError,
    DecriptError,
    NonUtf8,
    WrongKey,
    DecriptPub,
    KeyEncrypt,
    DecriptPrivate,
    KeyDecript,
}

#[derive(Debug, PartialEq)]
pub struct Keypair {
    pub private: RsaPrivateKey,
    pub public: RsaPublicKey,
}

pub fn create_keypair() -> Keypair {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    Keypair {
        private: private_key,
        public: public_key,
    }
}

pub fn serialize_pub(public: RsaPublicKey) -> String {
    public.to_public_key_pem(LineEnding::default()).unwrap()
}

pub fn serialize_private(private: RsaPrivateKey) -> String {
    String::from(private.to_pkcs8_pem(LineEnding::default()).unwrap().as_str())
}

pub fn share_secret<'a>(key: &Secret, rsa_pub_str: &'a str) -> Result<SharedSecret, Error> {
    let mut rng = rand::thread_rng();
    let public_key =
        RsaPublicKey::from_public_key_pem(&rsa_pub_str).map_err(|_x| Error::DecriptPub)?;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    public_key
        .encrypt(&mut rng, padding, key)
        .map_err(|_| Error::KeyEncrypt)
}

pub fn read_secret(shared_secret: &SharedSecret, rsa_priv_str: &str) -> Result<Secret, Error> {
    let private_key =
        RsaPrivateKey::from_pkcs8_pem(&rsa_priv_str).map_err(|_x| Error::DecriptPrivate)?;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let mut sized_v = [0u8; 32];
    let v = private_key
        .decrypt(padding, &shared_secret)
        .map_err(|_x| Error::KeyDecript)?;
    let mut t = 0;
    for i in v {
        sized_v[t] = i;
        t += 1;
    }
    Ok(sized_v)
}

pub fn encode_msg(params: EncodeFileParams) -> Result<Vec<u8>, Error> {
    let cipher = Aes256Gcm::new_from_slice(params.key).map_err(|_| Error::WrongKey)?;
    let nonce = Nonce::from_slice(NONCE); // 96-bits; unique per message
    let bytes = cipher
        .encrypt(nonce, params.msg.as_ref())
        .map_err(|_| Error::EncriptError)?;
    Ok(bytes)
}

pub fn decode_msg(params: DecodeFileParams) -> Result<Vec<u8>, Error> {
    let cipher = Aes256Gcm::new_from_slice(params.key).map_err(|_| Error::WrongKey)?;
    let nonce = Nonce::from_slice(NONCE); // 96-bits; unique per message
    cipher
        .decrypt(nonce, params.ciphertext.as_ref())
        .map_err(|_| Error::DecriptError)
}

pub fn gen_key() -> Secret {
    let mut sized_v = [0u8; 32];
    let v = Aes256Gcm::generate_key(OsRng).to_vec();
    let mut t = 0;
    for i in v {
        sized_v[t] = i;
        t += 1;
    }

    sized_v
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use std::fs;

    const KEY: Secret = [1u8; 32];

    fn chipher_text() -> Vec<u8> {
        vec![
            50, 11, 229, 207, 205, 109, 39, 30, 108, 223, 129, 191, 80, 58, 29, 142, 202, 59, 174,
            57, 88, 89, 217, 132, 238, 12, 135, 156, 128, 221,
        ]
    }

    fn shared_secret_data() -> Vec<u8> {
        vec![
            126, 237, 230, 198, 90, 245, 102, 202, 117, 171, 66, 81, 32, 185, 49, 255, 153, 239,
            150, 48, 196, 22, 84, 48, 224, 36, 159, 204, 105, 106, 186, 184, 133, 201, 112, 17, 84,
            36, 121, 75, 73, 85, 227, 16, 221, 66, 78, 254, 182, 154, 235, 197, 153, 201, 0, 70,
            78, 70, 61, 182, 23, 238, 19, 16, 50, 217, 53, 192, 16, 82, 110, 184, 46, 22, 146, 209,
            158, 212, 7, 140, 81, 226, 80, 20, 126, 226, 225, 72, 188, 41, 2, 93, 22, 201, 36, 10,
            169, 123, 207, 193, 23, 122, 45, 10, 138, 166, 45, 192, 140, 134, 242, 85, 35, 141,
            133, 235, 62, 249, 108, 234, 0, 206, 197, 45, 41, 89, 97, 196, 98, 203, 38, 126, 0, 23,
            197, 29, 152, 51, 145, 193, 206, 188, 69, 35, 219, 117, 47, 62, 123, 233, 55, 80, 40,
            170, 39, 10, 138, 206, 218, 48, 3, 89, 215, 239, 223, 53, 206, 167, 67, 133, 144, 143,
            63, 164, 19, 145, 209, 212, 156, 71, 221, 181, 240, 153, 122, 6, 55, 214, 4, 185, 9,
            222, 161, 85, 57, 206, 243, 153, 129, 148, 74, 72, 197, 222, 193, 172, 158, 88, 173,
            55, 143, 9, 1, 200, 208, 16, 2, 58, 165, 3, 69, 254, 20, 243, 7, 232, 246, 246, 235,
            204, 61, 225, 18, 193, 23, 185, 29, 166, 167, 97, 183, 62, 45, 2, 97, 75, 148, 193,
            193, 119, 31, 7, 43, 237, 210, 136,
        ]
    }

    fn text<'a>() -> &'a str {
        "f example text"
    }

    #[test]
    fn should_encode_msg() {
        let result = encode_msg(EncodeFileParams {
            msg: text(),
            key: &KEY,
        });
        assert_eq!(result, Ok(chipher_text()));
    }

    #[test]
    fn should_gen_key() {
        let result = gen_key();
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn should_decode_msg() {
        let result = decode_msg(DecodeFileParams {
            ciphertext: chipher_text(),
            key: &KEY,
        })
        .unwrap();
        assert_eq!(str::from_utf8(&result).unwrap(), text());
    }

    #[test]
    fn should_share_secret() {
        let contents = fs::read_to_string("../priv/test/public.pem")
            .expect("Should have been able to read the file");
        let result = share_secret(&KEY, &contents);

        assert_eq!(result.unwrap().len(), 256);
    }

    #[test]
    fn should_read_secret() {
        let contents = fs::read_to_string("../priv/test/key.pem")
            .expect("Should have been able to read the file");
        let result = read_secret(&shared_secret_data(), &contents);
        assert_eq!(result.unwrap(), KEY);
    }
}
