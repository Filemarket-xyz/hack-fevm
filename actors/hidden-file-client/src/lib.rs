mod utils;

use hidden_file_crypto;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct JsKeyPair {
    private: String,
    public: String
}

#[wasm_bindgen]
impl JsKeyPair {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        alert("Creating keypair");
        let keypair = hidden_file_crypto::create_keypair();
        let private_str = hidden_file_crypto::serialize_private(keypair.private);
        let public_str = hidden_file_crypto::serialize_pub(keypair.public);
        JsKeyPair {private: private_str, public: public_str}
    }

    #[wasm_bindgen(getter)]
    pub fn get_public(&self) -> String {
        self.public.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_private(&self) -> String {
        self.private.clone()
    }
}
