mod utils;

use wasm_bindgen::prelude::*;
// use tendermint_rpc::{Client as TendermintClient};
// use tendermint_rpc::endpoint::abci_info;
// use tendermint::abci;
// use tendermint::error::Error;
use async_trait::async_trait;
use namada::ledger::queries::{Client, EncodedResponseQuery};

// struct WebRpcClient;

// https://docs.rs/tendermint-rpc/latest/tendermint_rpc/trait.Client.html
// impl TendermintClient for WebRpcClient {
//     // should implement 
// }

// impl Client for WebRpcClient { 
//     // should implement request
//     // simple_request has a default implementation
// }


// once I have implemented it, I used it like Jeremy in CLI
//
// Sub::TxTransfer(TxTransfer(args)) => {
//     let client =
//         HttpClient::new(args.tx.ledger_address.clone())
//             .unwrap();
//     let args = args.to_sdk(&mut ctx);
//     tx::submit_transfer::<HttpClient, CliWalletUtils, _>(
//         &client,
//         &mut ctx.wallet,
//         &mut ctx.shielded,
//         args,
//     )
//     .await?;
// }


// impl Client for WebRpcClient {
//     async fn request(
//         &self,
//         path: String,
//         data: Option<Vec<u8>>,
//         height: Option<BlockHeight>,
//         prove: bool,
//     ) -> Result<EncodedResponseQuery, Self::Error> {

//         // I use js call here that does the fetching of the data
//         // simple, just returns the byte array
//         let response = EncodedResponseQuery {
//             data: vec![0],
//             info: String::from("response info"),
//             proof: None,
//         };
//         Ok(response)
//     }
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

async fn get_abci_info() -> String{
    String::from("get_abci_info")
}

#[wasm_bindgen]
pub async fn greet() {
    alert("Hello, tendermint-rs-in-web!");
}

#[wasm_bindgen]
pub async fn get_abci_info_from_rust() -> Result<JsValue, JsValue> {
    let _abci_info = get_abci_info();
    Ok(JsValue::from_str("aaa"))
}