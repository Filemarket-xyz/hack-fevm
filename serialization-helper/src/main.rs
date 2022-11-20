use fvm_ipld_encoding::RawBytes;
use base64;
use fvm_ipld_encoding::{
    tuple::{Deserialize_tuple, Serialize_tuple},
    strict_bytes,
};
use std::fs;
use std::env;

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct WordParams {
    #[serde(with = "strict_bytes")]
    pub word: Vec<u8>,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct ConstructorParams {
    #[serde(with = "strict_bytes")]
    pub contents: Vec<u8>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read(&args[2]).unwrap();
    let constructor_params = ConstructorParams {
        contents: contents,
    };
    println!("constructor params {:?}", constructor_params);
    println!("constructor params {:?}", base64::encode_config(RawBytes::serialize(constructor_params).unwrap().bytes(), base64::STANDARD));

    let _word_params = WordParams {
        word: args[1].as_bytes().to_vec(),
    };
    println!("word params {:?}", _word_params);
    println!("word params {:?}", base64::encode_config(RawBytes::serialize(_word_params).unwrap().bytes(), base64::STANDARD));
}
