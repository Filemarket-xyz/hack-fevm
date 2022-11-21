mod utils;

use hidden_file_crypto;
use wasm_bindgen::prelude::*;

use log::{Level, info};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub struct JsKeyPair {
    private: String,
    public: String
}

#[wasm_bindgen]
impl JsKeyPair {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_log::init_with_level(Level::Debug).unwrap();

        info!("Creating keypair");
        let keypair = hidden_file_crypto::create_keypair();
        let private_str = hidden_file_crypto::serialize_private(keypair.private);
        let public_str = hidden_file_crypto::serialize_pub(keypair.public);
        JsKeyPair {private: private_str, public: public_str}
    }

    #[wasm_bindgen(getter)]
    pub fn public(&self) -> String {
        self.public.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn private(&self) -> String {
        self.private.clone()
    }
}
