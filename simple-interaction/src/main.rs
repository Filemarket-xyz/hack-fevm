use fvm_ipld_encoding::RawBytes;
use base64;
use fvm_ipld_encoding::{
    tuple::{Deserialize_tuple, Serialize_tuple},
    strict_bytes,
};
use dotenvy;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, web::Query, Responder, web::Data, web::Bytes};
use std::env;
use serde::{Deserialize, Serialize};
use async_process::Command;

pub struct Config {
    pub path: String,
    pub lotus_path: String,
    pub lotus_miner_path: String,
    pub code_cid: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct InvokeCountMatchesRequest {
    pub address: String,
    pub word: String,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct ConstructorParams {
    #[serde(with = "strict_bytes")]
    pub contents: Vec<u8>,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct WordParams {
    #[serde(with = "strict_bytes")]
    pub word: Vec<u8>,
}

pub async fn invoke_count_matches   (
    que: Query<InvokeCountMatchesRequest>,
    cfg: Data<Config>,
) -> impl Responder  {
    let _word_params = WordParams {
        word: que.word.as_bytes().to_vec(),
    };
    let encoded_params = base64::encode_config(RawBytes::serialize(_word_params).unwrap().bytes(), base64::STANDARD);
    let _result = Command::new(&cfg.path)
        .env("LOTUS_PATH", &cfg.lotus_path)
        .env("LOTUS_MINER_PATH", &cfg.lotus_miner_path)
        .env("LOTUS_SKIP_GENESIS_CHECK", "_yes")
        .env("CGO_CFLAGS_ALLOW", "-D__BLST_PORTABLE__")
        .env("CGO_CFLAGS", "-D__BLST_PORTABLE__")
        .args(["chain", "invoke", &que.address, "3", &encoded_params])
        .output()
        .await
        .expect("failed to execute");
    let res_s = String::from_utf8(_result.stdout).unwrap();
    let split = res_s.split("\n");
    let lines = split.collect::<Vec<&str>>();
    let last = lines[lines.len()-2];
    println!("what? {:?}", _result.status);
    println!("aaa? {:?}", String::from_utf8(_result.stderr));
    let decoded = base64::decode(last).unwrap();
    format!("{:}", String::from_utf8(decoded).unwrap())
}

pub async fn create_actor(
    bytes: Bytes,
     cfg: Data<Config>
    ) -> impl Responder {
    let constructor_params = ConstructorParams {
        contents: bytes.to_vec(),
    };
    println!("kekes: {:?}", String::from_utf8(bytes.to_vec()));
    let encoded_params = base64::encode_config(RawBytes::serialize(constructor_params).unwrap().bytes(), base64::STANDARD);
    let _result = Command::new(&cfg.path)
        .env("LOTUS_PATH", &cfg.lotus_path)
        .env("LOTUS_MINER_PATH", &cfg.lotus_miner_path)
        .env("LOTUS_SKIP_GENESIS_CHECK", "_yes")
        .env("CGO_CFLAGS_ALLOW", "-D__BLST_PORTABLE__")
        .env("CGO_CFLAGS", "-D__BLST_PORTABLE__")
        .args(["chain", "create-actor", &cfg.code_cid, &encoded_params])
        .output()
        .await
        .expect("failed to execute");
    let res_s = String::from_utf8(_result.stdout).unwrap();
    let split = res_s.split("\n");
    let lines = split.collect::<Vec<&str>>();
    let last = lines[lines.len()-3];
    println!("what? {:?}", _result.status);
    println!("aaa? {:?}", String::from_utf8(_result.stderr));
    let trimmed = last.strip_prefix("Robust Address: ").unwrap();
    format!("{:}", trimmed)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("env error (file .env not found)");

    let config_data = web::Data::new(Config {
        path: env::var("LOTUS_EXECUTABLE").expect("env err (LOTUS_EXECUTABLE)"),
        lotus_path: env::var("LOTUS_PATH").expect("env err (LOTUS_PATH)"),
        lotus_miner_path: env::var("LOTUS_MINER_PATH").expect("env err (LOTUS_MINER_PATH)"),
        code_cid: env::var("CODE_CID").expect("env err (CODE_CID)"),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(config_data.clone())
            .route("/api/invoke_count_matches", web::post().to(invoke_count_matches))
            .route("/api/create_actor", web::post().to(create_actor))
    })
    .keep_alive(std::time::Duration::new(10, 0))
    .bind((
        "0.0.0.0",
        9300,
    ))?
    .run()
    .await
}