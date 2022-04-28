#![feature(test)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::entries::entries::generate_mysql_smt_root;
use crate::indexer::index::get_cota_smt_root;
use crate::models::claim::load_claim_cota_count_by_lock_hash;
use crate::models::define::DefineDb;
use crate::models::get_all_cota_by_lock_hash;
use crate::models::helper::{init_connection_pool, SqlConnectionPool};
use crate::models::scripts::get_all_script_hashes;
use cota_smt::smt::blake2b_256;
use dotenv::dotenv;
use lazy_static::lazy_static;
use log::info;

pub mod entries;
pub mod indexer;
pub mod models;
pub mod schema;
pub mod utils;

lazy_static! {
    static ref POOL: SqlConnectionPool = init_connection_pool();
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    env_logger::Builder::from_default_env()
        .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Millis))
        .init();

    info!("White list server start");

    let mut white_list_len = 0;
    let lock_scripts = get_all_script_hashes();
    let length = lock_scripts.len();
    info!("CoTA users count: {}", length);

    for (index, lock_script) in lock_scripts.into_iter().enumerate() {
        info!("Scan progress: {}/{}", index, length);
        let lock_hash = blake2b_256(lock_script.clone());
        if load_claim_cota_count_by_lock_hash(lock_hash) != 1 {
            continue;
        }
        let cota_cell_smt_root = get_cota_smt_root(&lock_script).await;
        if let Some(cell_smt_root) = cota_cell_smt_root {
            let db_smt_root = generate_mysql_smt_root(lock_hash);
            if db_smt_root.as_slice() != cell_smt_root.as_slice() {
                white_list_len += 1;
                info!("White list index: {}", white_list_len);
                info!("Lock hash: {:?}'s smt root error, live cell smt root: {:?} and db smt root: {:?}",
                    hex::encode(&lock_hash) , hex::encode(&cell_smt_root), hex::encode(db_smt_root.as_slice()));
            }
        }
    }
    info!("white list length: {}", white_list_len);
}
