use ckb_jsonrpc_types::{BlockNumber, CellOutput, JsonBytes, OutPoint, Uint32};
use ckb_types::packed::Script;
use ckb_types::prelude::Entity;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::env;

const MAINNET_COTA_CODE_HASH: &str =
    "0x1122a4fb54697cf2e6e3a96c9d80fd398a936559b90954c6e88eb7ba0cf652df";

pub async fn get_cota_smt_root(lock_script: &[u8]) -> Option<[u8; 32]> {
    let ckb_indexer_url = env::var("CKB_INDEXER").expect("CKB_INDEXER must be set");

    let mut req_json = Map::new();
    req_json.insert("id".to_owned(), json!("1"));
    req_json.insert("jsonrpc".to_owned(), json!("2.0"));
    req_json.insert("method".to_owned(), json!("get_cells"));
    req_json.insert("params".to_owned(), generate_params(lock_script));

    let client = reqwest::Client::new();

    let resp = client
        .post(ckb_indexer_url)
        .json(&req_json)
        .send()
        .await
        .expect("CKB Indexer rpc error");
    let output = resp
        .json::<jsonrpc_core::response::Output>()
        .await
        .expect("CKB Indexer rpc error");

    let result: CellPagination = match output {
        jsonrpc_core::response::Output::Success(success) => {
            serde_json::from_value::<CellPagination>(success.result).expect("Parse response error")
        }
        jsonrpc_core::response::Output::Failure(_f) => {
            panic!("Response error")
        }
    };
    if result.objects.is_empty() {
        return None;
    }
    let cell_data = result.objects.first().unwrap().output_data.as_bytes();
    match cell_data.len() {
        1 => None,
        33 => {
            let mut ret = [0u8; 32];
            ret.copy_from_slice(&cell_data[1..]);
            Some(ret)
        }
        _ => panic!("CoTA cell data error"),
    }
}

fn generate_params(lock_script: &[u8]) -> Value {
    let lock = Script::from_slice(lock_script).expect("Lock script format error");
    let hash_type = match lock.hash_type().into() {
        0u8 => "data",
        1u8 => "type",
        2u8 => "data1",
        _ => "0",
    };
    let code_hash = MAINNET_COTA_CODE_HASH;

    json!([
        {
            "script": {
                "code_hash": format!("0x{}", hex::encode(lock.code_hash().as_slice())),
                "hash_type": hash_type,
                "args": format!("0x{}", hex::encode(lock.args().raw_data())),
            },
            "script_type": "lock",
            "filter": {
                "script": {
                    "code_hash": code_hash,
                    "hash_type": "type",
                    "args": "0x",
                },
            }
        },
        "asc",
        "0x1"
    ])
}

#[derive(Deserialize)]
struct Cell {
    #[serde(skip_deserializing)]
    _output: CellOutput,
    output_data: JsonBytes,
    #[serde(skip_deserializing)]
    _out_point: OutPoint,
    #[serde(skip_deserializing)]
    _block_number: BlockNumber,
    #[serde(skip_deserializing)]
    _tx_index: Uint32,
}

#[derive(Deserialize)]
struct CellPagination {
    objects: Vec<Cell>,
    #[serde(skip_deserializing)]
    _last_cursor: JsonBytes,
}
