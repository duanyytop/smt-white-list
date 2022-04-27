use crate::entries::helper::{
    generate_claim_key, generate_claim_value, generate_cota_index, generate_define_key,
    generate_define_value, generate_hold_key, generate_hold_value, generate_withdrawal_key,
    generate_withdrawal_key_v1, generate_withdrawal_value, generate_withdrawal_value_v1,
};
use crate::models::claim::ClaimDb;
use crate::models::hold::HoldDb;
use crate::models::withdrawal::WithdrawDb;
use crate::{get_all_cota_by_lock_hash, DefineDb};
use cota_smt::smt::{H256, SMT};
use std::collections::HashMap;

pub fn generate_mysql_smt_root(lock_hash: [u8; 32]) -> H256 {
    let mut smt = SMT::default();
    let (defines, holds, withdrawals, claims) = get_all_cota_by_lock_hash(lock_hash);

    for define_db in defines {
        let DefineDb {
            cota_id,
            total,
            issued,
            configure,
        } = define_db;
        let (_, key) = generate_define_key(cota_id);
        let (_, value) =
            generate_define_value(total.to_be_bytes(), issued.to_be_bytes(), configure);
        smt.update(key, value).expect("SMT update leave error");
    }
    for hold_db in holds {
        let HoldDb {
            cota_id,
            token_index,
            configure,
            state,
            characteristic,
        } = hold_db;
        let (_, key) = generate_hold_key(cota_id, token_index);
        let (_, value) = generate_hold_value(configure, state, characteristic);
        smt.update(key, value).expect("SMT update leave error");
    }
    let mut withdrawal_map: HashMap<Vec<u8>, u8> = HashMap::new();
    for withdrawal_db in withdrawals {
        let WithdrawDb {
            cota_id,
            token_index,
            configure,
            state,
            characteristic,
            receiver_lock_script,
            out_point,
            version,
        } = withdrawal_db;
        let (key, value) = if version == 0 {
            (
                generate_withdrawal_key(cota_id, token_index).1,
                generate_withdrawal_value(
                    configure,
                    state,
                    characteristic,
                    &receiver_lock_script,
                    out_point,
                )
                .1,
            )
        } else {
            (
                generate_withdrawal_key_v1(cota_id, token_index, out_point).1,
                generate_withdrawal_value_v1(
                    configure,
                    state,
                    characteristic,
                    &receiver_lock_script,
                )
                .1,
            )
        };
        withdrawal_map.insert(generate_cota_index(cota_id, token_index), version);
        smt.update(key, value).expect("SMT update leave error");
    }
    for claim_db in claims {
        let ClaimDb {
            cota_id,
            token_index,
            out_point,
        } = claim_db;
        let version = withdrawal_map
            .get(&*generate_cota_index(cota_id, token_index))
            .cloned()
            .unwrap_or_default();
        let (_, key) = generate_claim_key(cota_id, token_index, out_point);
        let (_, value) = generate_claim_value(version);
        smt.update(key, value).expect("SMT update leave error");
    }
    *smt.root()
}
