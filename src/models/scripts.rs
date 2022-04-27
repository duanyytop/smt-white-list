use crate::models::helper::PAGE_SIZE;
use crate::schema::scripts::dsl::scripts;
use crate::schema::scripts::*;
use crate::schema::scripts::{args, code_hash, hash_type};
use crate::utils::{parse_bytes, parse_bytes_n};
use crate::POOL;
use cota_smt::ckb_types::packed::{Byte32, BytesBuilder, ScriptBuilder};
use cota_smt::ckb_types::prelude::*;
use cota_smt::molecule::prelude::Byte;
use diesel::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Script {
    pub id: i64,
    pub code_hash: String,
    pub hash_type: u8,
    pub args: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScriptDb {
    pub id: i64,
    pub code_hash: [u8; 32],
    pub hash_type: u8,
    pub args: Vec<u8>,
}

pub fn get_script_map_by_ids(script_ids: Vec<i64>) -> HashMap<i64, Vec<u8>> {
    let conn = &POOL.clone().get().expect("Mysql pool connection error");
    let mut scripts_dbs: Vec<ScriptDb> = vec![];
    let script_ids_subs: Vec<&[i64]> = script_ids.chunks(PAGE_SIZE as usize).collect();
    for script_ids_sub in script_ids_subs.into_iter() {
        let scripts_db = scripts
            .select((id, code_hash, hash_type, args))
            .filter(id.eq_any(script_ids_sub))
            .load::<Script>(conn)
            .map(parse_script)
            .expect("Query script error");
        scripts_dbs.extend(scripts_db);
    }
    let scripts_: Vec<(i64, Vec<u8>)> = scripts_dbs
        .iter()
        .map(|script_db| (script_db.id, generate_script_vec(script_db)))
        .collect();
    let script_map: HashMap<i64, Vec<u8>> = scripts_.into_iter().collect();
    script_map
}

fn parse_script(scripts_: Vec<Script>) -> Vec<ScriptDb> {
    scripts_
        .into_iter()
        .map(|script| ScriptDb {
            id: script.id,
            code_hash: parse_bytes_n::<32>(script.code_hash),
            hash_type: script.hash_type,
            args: parse_bytes(script.args),
        })
        .collect()
}

fn generate_script_vec(script_db: &ScriptDb) -> Vec<u8> {
    let args_bytes: Vec<Byte> = script_db.args.iter().map(|v| Byte::from(*v)).collect();
    let script = ScriptBuilder::default()
        .code_hash(Byte32::from_slice(&script_db.code_hash[..]).unwrap())
        .hash_type(Byte::from(script_db.hash_type))
        .args(BytesBuilder::default().set(args_bytes).build())
        .build();
    script.as_slice().to_vec()
}
