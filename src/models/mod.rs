use crate::models::claim::{get_claim_cota_by_lock_hash, ClaimDb};
use crate::models::define::{get_define_cota_by_lock_hash, DefineDb};
use crate::models::hold::{get_hold_cota_by_lock_hash, HoldDb};
use crate::models::withdrawal::{get_withdrawal_cota_by_lock_hash, WithdrawDb};

pub(crate) mod claim;
pub(crate) mod define;
pub mod helper;
pub(crate) mod hold;
pub(crate) mod scripts;
pub(crate) mod withdrawal;

pub fn get_all_cota_by_lock_hash(
    lock_hash: [u8; 32],
) -> (Vec<DefineDb>, Vec<HoldDb>, Vec<WithdrawDb>, Vec<ClaimDb>) {
    let defines = get_define_cota_by_lock_hash(lock_hash);
    let holds = get_hold_cota_by_lock_hash(lock_hash);
    let withdrawals = get_withdrawal_cota_by_lock_hash(lock_hash);
    let claims = get_claim_cota_by_lock_hash(lock_hash);
    (defines, holds, withdrawals, claims)
}
