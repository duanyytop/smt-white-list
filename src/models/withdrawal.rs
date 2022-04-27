use super::helper::parse_lock_hash;
use crate::models::helper::PAGE_SIZE;
use crate::models::scripts::get_script_map_by_ids;
use crate::schema::withdraw_cota_nft_kv_pairs::dsl::withdraw_cota_nft_kv_pairs;
use crate::schema::withdraw_cota_nft_kv_pairs::*;
use crate::utils::parse_bytes_n;
use crate::POOL;
use diesel::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct WithdrawCotaNft {
    pub cota_id: String,
    pub token_index: u32,
    pub out_point: String,
    pub state: u8,
    pub configure: u8,
    pub characteristic: String,
    pub receiver_lock_script_id: i64,
    pub version: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WithdrawDb {
    pub cota_id: [u8; 20],
    pub token_index: [u8; 4],
    pub out_point: [u8; 24],
    pub state: u8,
    pub configure: u8,
    pub characteristic: [u8; 20],
    pub receiver_lock_script: Vec<u8>,
    pub version: u8,
}

pub fn get_withdrawal_cota_by_lock_hash(lock_hash_: [u8; 32]) -> Vec<WithdrawDb> {
    let conn = &POOL.clone().get().expect("Mysql pool connection error");
    let (lock_hash_hex, lock_hash_crc_) = parse_lock_hash(lock_hash_);
    let mut withdraw_nfts: Vec<WithdrawCotaNft> = vec![];
    let mut page: i64 = 0;
    loop {
        let withdrawals_page: Vec<WithdrawCotaNft> = withdraw_cota_nft_kv_pairs
            .select(get_selection())
            .filter(lock_hash_crc.eq(lock_hash_crc_))
            .filter(lock_hash.eq(lock_hash_hex.clone()))
            .limit(PAGE_SIZE)
            .offset(PAGE_SIZE * page)
            .load::<WithdrawCotaNft>(conn)
            .expect("Query withdraw error");
        let length = withdrawals_page.len();
        withdraw_nfts.extend(withdrawals_page);
        if length < (PAGE_SIZE as usize) {
            break;
        }
        page += 1;
    }
    parse_withdraw_db(withdraw_nfts)
}

fn parse_withdraw_db(withdrawals: Vec<WithdrawCotaNft>) -> Vec<WithdrawDb> {
    if withdrawals.is_empty() {
        return vec![];
    }
    let receiver_lock_script_ids: Vec<i64> = withdrawals
        .iter()
        .map(|withdrawal| withdrawal.receiver_lock_script_id)
        .collect();
    let mut withdraw_db_vec: Vec<WithdrawDb> = vec![];
    let script_map = get_script_map_by_ids(receiver_lock_script_ids);
    for withdrawal in withdrawals {
        let lock_script = script_map
            .get(&withdrawal.receiver_lock_script_id)
            .unwrap()
            .clone();
        withdraw_db_vec.push(WithdrawDb {
            cota_id: parse_bytes_n::<20>(withdrawal.cota_id),
            token_index: withdrawal.token_index.to_be_bytes(),
            configure: withdrawal.configure,
            state: withdrawal.state,
            characteristic: parse_bytes_n::<20>(withdrawal.characteristic),
            receiver_lock_script: lock_script,
            out_point: parse_bytes_n::<24>(withdrawal.out_point),
            version: withdrawal.version,
        })
    }
    withdraw_db_vec
}

fn get_selection() -> (
    cota_id,
    token_index,
    out_point,
    state,
    configure,
    characteristic,
    receiver_lock_script_id,
    version,
) {
    (
        cota_id,
        token_index,
        out_point,
        state,
        configure,
        characteristic,
        receiver_lock_script_id,
        version,
    )
}
