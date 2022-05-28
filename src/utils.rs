use cosmwasm_std::{Addr, CanonicalAddr, StdResult, Storage};

use crate::state::{ADMINS, IS_COLL_PAUSED, IS_PAUSED, OPERS};

pub fn check_is_paused(store: &dyn Storage, coll_addr: Addr) -> StdResult<bool> {
    let is_paused = IS_PAUSED.load(store)?;
    if !is_paused {
        // Only return false when is_paused and is_coll_paused are false
        let is_coll_paused = IS_COLL_PAUSED.may_load(store, coll_addr)?;
        return Ok(is_coll_paused.unwrap_or(false));
    }
    Ok(true)
}

pub fn check_is_operator(store: &dyn Storage, sender_raw: CanonicalAddr) -> StdResult<bool> {
    let opers = OPERS.load(store)?;
    if !opers.contains(&sender_raw) {
        // Allow admins to update too
        return check_is_admin(store, sender_raw);
    }
    Ok(true)
}

pub fn check_is_admin(store: &dyn Storage, sender_raw: CanonicalAddr) -> StdResult<bool> {
    let admins = ADMINS.load(store)?;
    if !admins.contains(&sender_raw) {
        return Ok(false);
    }
    Ok(true)
}
