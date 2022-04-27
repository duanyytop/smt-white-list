use super::helper::parse_lock_hash;
use crate::models::helper::PAGE_SIZE;
use crate::schema::define_cota_nft_kv_pairs::dsl::*;
use crate::utils::parse_bytes_n;
use crate::POOL;
use diesel::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Debug)]
struct DefineCotaNft {
    pub cota_id: String,
    pub total: u32,
    pub issued: u32,
    pub configure: u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DefineDb {
    pub cota_id: [u8; 20],
    pub total: u32,
    pub issued: u32,
    pub configure: u8,
}

pub fn get_define_cota_by_lock_hash(lock_hash_: [u8; 32]) -> Vec<DefineDb> {
    let conn = &POOL.clone().get().expect("Mysql pool connection error");
    let (lock_hash_hex, lock_hash_crc_) = parse_lock_hash(lock_hash_);
    let mut page: i64 = 0;
    let mut defines: Vec<DefineDb> = Vec::new();
    loop {
        let defines_page = define_cota_nft_kv_pairs
            .select(get_selection())
            .filter(lock_hash_crc.eq(lock_hash_crc_))
            .filter(lock_hash.eq(lock_hash_hex.clone()))
            .limit(PAGE_SIZE)
            .offset(PAGE_SIZE * page)
            .load::<DefineCotaNft>(conn)
            .map(parse_define_cota_nft)
            .expect("Query define error");
        let length = defines_page.len();
        defines.extend(defines_page);
        if length < (PAGE_SIZE as usize) {
            break;
        }
        page += 1;
    }
    defines
}

fn parse_define_cota_nft(defines: Vec<DefineCotaNft>) -> Vec<DefineDb> {
    defines
        .into_iter()
        .map(|define| DefineDb {
            cota_id: parse_bytes_n::<20>(define.cota_id),
            total: define.total,
            issued: define.issued,
            configure: define.configure,
        })
        .collect()
}

fn get_selection() -> (cota_id, total, issued, configure) {
    (cota_id, total, issued, configure)
}
