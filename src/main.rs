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
use crate::models::withdrawal::load_withdraw_cota_count_by_lock_hash;
use cota_smt::smt::blake2b_256;
use dotenv::dotenv;
use lazy_static::lazy_static;
use log::info;
use std::fs::File;
use std::io::Write;

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

    let mut file = File::create("whitelist.txt").expect("create failed");

    info!("White list server start");

    let mut white_list_len = 0;
    let lock_scripts = get_all_script_hashes();
    let length = lock_scripts.len();
    info!("CoTA users count: {}", length);

    for (index, lock_script) in lock_scripts.into_iter().enumerate() {
        info!("Scan progress: {}/{}", index, length);
        let lock_hash = blake2b_256(lock_script.clone());
        let claim_count = load_claim_cota_count_by_lock_hash(lock_hash);
        if claim_count > 1 {
            continue;
        }
        if claim_count == 0 && load_withdraw_cota_count_by_lock_hash(lock_hash) > 0 {
            continue;
        }
        let cota_cell_smt_root = get_cota_smt_root(&lock_script).await;
        if let Some(cell_smt_root) = cota_cell_smt_root {
            let db_smt_root = generate_mysql_smt_root(lock_hash);
            if db_smt_root.as_slice() != cell_smt_root.as_slice() {
                white_list_len += 1;
                info!("White list index: {}", white_list_len);
                file.write_all(hex::encode(&lock_hash).as_bytes())
                    .expect("write lock hash failed");
                file.write_all(" / ".as_bytes())
                    .expect("write separate failed");
                file.write_all(hex::encode(&cell_smt_root).as_bytes())
                    .expect("write cell smt root failed");
                file.write_all(" / ".as_bytes())
                    .expect("write separate failed");
                file.write_all(hex::encode(db_smt_root.as_slice()).as_bytes())
                    .expect("write db smt root failed");
                file.write_all("\n".as_bytes()).expect("write \n failed");
            }
        }
    }
    info!("white list length: {}", white_list_len);
}
