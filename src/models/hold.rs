use super::helper::parse_lock_hash;
use crate::models::helper::PAGE_SIZE;
use crate::schema::hold_cota_nft_kv_pairs::dsl::hold_cota_nft_kv_pairs;
use crate::schema::hold_cota_nft_kv_pairs::*;
use crate::utils::parse_bytes_n;
use crate::POOL;
use diesel::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Debug)]
struct HoldCotaNft {
    pub cota_id: String,
    pub token_index: u32,
    pub state: u8,
    pub configure: u8,
    pub characteristic: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct HoldDb {
    pub cota_id: [u8; 20],
    pub token_index: [u8; 4],
    pub state: u8,
    pub configure: u8,
    pub characteristic: [u8; 20],
}

pub fn get_hold_cota_by_lock_hash(lock_hash_: [u8; 32]) -> Vec<HoldDb> {
    let conn = &POOL.clone().get().expect("Mysql pool connection error");
    let (lock_hash_hex, lock_hash_crc_) = parse_lock_hash(lock_hash_);
    let mut holds: Vec<HoldDb> = vec![];
    let mut page: i64 = 0;
    loop {
        let holds_page: Vec<HoldDb> = hold_cota_nft_kv_pairs
            .select(get_selection())
            .filter(lock_hash_crc.eq(lock_hash_crc_))
            .filter(lock_hash.eq(lock_hash_hex.clone()))
            .limit(PAGE_SIZE)
            .offset(PAGE_SIZE * page)
            .load::<HoldCotaNft>(conn)
            .map(parse_hold_cota_nfts)
            .expect("Query hold error");
        let length = holds_page.len();
        holds.extend(holds_page);
        if length < (PAGE_SIZE as usize) {
            break;
        }
        page += 1;
    }
    holds
}

fn parse_hold_cota_nfts(holds: Vec<HoldCotaNft>) -> Vec<HoldDb> {
    holds.into_iter().map(parse_hold_cota_nft).collect()
}

fn parse_hold_cota_nft(hold: HoldCotaNft) -> HoldDb {
    HoldDb {
        cota_id: parse_bytes_n::<20>(hold.cota_id),
        token_index: hold.token_index.to_be_bytes(),
        state: hold.state,
        configure: hold.configure,
        characteristic: parse_bytes_n::<20>(hold.characteristic),
    }
}

fn get_selection() -> (cota_id, token_index, configure, state, characteristic) {
    (cota_id, token_index, configure, state, characteristic)
}
