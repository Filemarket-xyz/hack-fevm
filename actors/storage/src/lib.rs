mod blockstore;
mod crypto;

use crate::blockstore::Blockstore;
use crate::crypto::{EncriptedFileContent, EncriptedPassword, SalePrivateKey, SalePublicKey};

use cid::multihash::Code;
use cid::Cid;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{to_vec, CborStore, RawBytes, DAG_CBOR};
use fvm_sdk as sdk;
use fvm_sdk::NO_DATA_BLOCK_ID;
use fvm_shared::ActorID;
use fvm_shared;
use fvm_shared::econ::TokenAmount;
use fvm_shared::address::Address;

macro_rules! abort {
    ($code:ident, $msg:literal $(, $ex:expr)*) => {
        fvm_sdk::vm::abort(
            fvm_shared::error::ExitCode::$code.value(),
            Some(format!($msg, $($ex,)*).as_str()),
        )
    };
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct State {
    pub seller: Address,
    pub consumer: Option<Address>,
    pub word: String,
    pub ciphered_file_content: EncriptedFileContent,
    pub cost: TokenAmount,
    pub consumer_pub: Option<SalePublicKey>,
    pub ciphered_encoding_key: Option<EncriptedPassword>,
    pub is_finished: bool
}

impl State {
    pub fn load() -> Self {
        // First, load the current state root.
        let root = match sdk::sself::root() {
            Ok(root) => root,
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get root: {:?}", err),
        };

        // Load the actor state from the state tree.
        match Blockstore.get_cbor::<Self>(&root) {
            Ok(Some(state)) => state,
            Ok(None) => abort!(USR_ILLEGAL_STATE, "state does not exist"),
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get state: {}", err),
        }
    }

    pub fn save(&self) -> Cid {
        let serialized = match to_vec(self) {
            Ok(s) => s,
            Err(err) => abort!(USR_SERIALIZATION, "failed to serialize state: {:?}", err),
        };
        let cid = match sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice())
        {
            Ok(cid) => cid,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store initial state: {:}", err),
        };
        if let Err(err) = sdk::sself::set_root(&cid) {
            abort!(USR_ILLEGAL_STATE, "failed to set root ciid: {:}", err);
        }
        cid
    }
}

#[no_mangle]
pub fn invoke(params: u32) -> u32 {
    // Conduct method dispatch. Handle input parameters and return data.
    let ret: Option<RawBytes> = match sdk::message::method_number() {
        1 => constructor(params),
        2 => buy_file(params),
        3 => share_access(params),
        4 => finish_sale(),
        5 => complain(params),
        _ => abort!(USR_UNHANDLED_MESSAGE, "unrecognized method"),
    };

    match ret {
        None => NO_DATA_BLOCK_ID,
        Some(v) => match sdk::ipld::put_block(DAG_CBOR, v.bytes()) {
            Ok(id) => id,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store return value: {}", err),
        },
    }
}

#[derive(Debug, Deserialize_tuple)]
struct InitialParams {
    pub seller: Address,
    pub word: String,
    pub ciphered_file_content: EncriptedFileContent,
    pub cost: TokenAmount
}

pub fn constructor(params: u32) -> Option<RawBytes> {
    const INIT_ACTOR_ADDR: ActorID = 1;

    if sdk::message::caller() != INIT_ACTOR_ADDR {
        abort!(USR_FORBIDDEN, "constructor invoked by non-init actor");
    }

    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);
    let params: InitialParams = params.deserialize().unwrap();

    let state = State {
        seller: params.seller,
        consumer: None,
        word: params.word,
        ciphered_file_content: params.ciphered_file_content,
        cost: params.cost,
        consumer_pub: None,
        ciphered_encoding_key: None,
        is_finished: false
    };

    state.save();
    None
}

#[derive(Debug, Deserialize_tuple)]
struct BuyParams {
    pub consumer_pub: SalePublicKey
}

pub fn buy_file(params: u32) -> Option<RawBytes> {
    let mut state = State::load();

    if let Some(_) = state.consumer_pub {
        abort!(USR_FORBIDDEN, "File allready buyed");
    }

    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);
    let params: BuyParams = params.deserialize().unwrap();

    let caller = sdk::message::caller();
    let address = Address::new_id(100);
    let send_params = RawBytes::default();

    let _receipt = fvm_sdk::send::send(
        &address,
        2,
        send_params,
        state.cost.clone(),
    ).unwrap();

    state.consumer_pub = Some(params.consumer_pub);
    state.consumer = Some(Address::new_id(caller));

    state.save();
    None
}

#[derive(Debug, Deserialize_tuple)]
struct ShareParams {
    pub ciphered_encoding_key: EncriptedPassword
}

fn share_access(params: u32) -> Option<RawBytes> {
    let mut state = State::load();

    let caller = sdk::message::caller();
    let caller_address = Address::new_id(caller);

    if caller_address != state.seller {
        abort!(USR_FORBIDDEN, "Only owner may share access");
    }

    if state.consumer_pub.is_none() || state.ciphered_encoding_key.is_some() {
        abort!(USR_FORBIDDEN, "Wrong state");
    }

    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);
    let params: ShareParams = params.deserialize().unwrap();

    state.ciphered_encoding_key = Some(params.ciphered_encoding_key);

    state.save();
    None
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone)]
struct WithdrawParams {
    provider_or_client: Address,
    amount: TokenAmount
}

#[derive(Debug, Deserialize_tuple)]
struct ComplainParams {
    pub user_priv_key: SalePrivateKey
}

fn complain(params: u32) -> Option<RawBytes> {
    let mut state = State::load();

    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);
    let params: ComplainParams = params.deserialize().unwrap();
    
    let encoded_password = state.ciphered_encoding_key.as_ref().unwrap();

    let consumer_pub = state.consumer_pub.as_ref().unwrap();
    let file_content_result = crypto::decript(
        &params.user_priv_key,
        &consumer_pub,
        &encoded_password,
        state.ciphered_file_content.clone()
    );

    if let Ok(file_content) = file_content_result {
        if file_content.contains(&state.word) {
            return  None;
        }   
    }

    let send_params = WithdrawParams {
        provider_or_client: state.consumer.unwrap(),
        amount: state.cost.clone()
    };
    let send_params = RawBytes::serialize(send_params).unwrap();

    let market_address = Address::new_id(100);

    let _receipt = fvm_sdk::send::send(
        &market_address,
        3,
        send_params,
        TokenAmount::from_atto(0)
    ).unwrap();

    state.is_finished = true;

    state.save();
    None
}

fn finish_sale() -> Option<RawBytes> {
    let mut state = State::load();

    if state.ciphered_encoding_key.is_none() {
        abort!(USR_FORBIDDEN, "Wrong state");
    }

    let caller = sdk::message::caller();
    let caller_address = Address::new_id(caller);

    if caller_address != state.consumer.unwrap() {
        abort!(USR_FORBIDDEN, "Only consumer may finish sale");
    }

    let send_params = WithdrawParams {
        provider_or_client: state.seller,
        amount: state.cost.clone()
    };
    let send_params = RawBytes::serialize(send_params).unwrap();

    let market_address = Address::new_id(100);

    let _receipt = fvm_sdk::send::send(
        &market_address,
        3,
        send_params,
        TokenAmount::from_atto(0)
    ).unwrap();

    state.is_finished = true;

    state.save();
    None
}
