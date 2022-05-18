use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::{Map, U8Key, Item};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::error::ContractError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
struct TokenInfo {
    /// The address of the destination contract on Secret Network
    secret_address: Addr,
    /// The address of the user that sent the token to the bridge
    sender: Addr,
    /// Whether the token has been successfully bridged to Secret Network
    is_bridged: bool,
}

/// mapping of (contract_address, token_id) -> TokenInfo
pub const TOKENS: Map<(&Addr, &str), TokenInfo> = Map::new("tokens_locked");


/**
 * Admin State
 */

/// mapping of u8 -> admin_address
pub const ADMINS: Map<U8Key, Addr> = Map::new("admins");
/// counter used to create U8Keys
pub const ADMIN_COUNTER: Item<u8> = Item::new("admin_counter");
/// function to grab the next key for ADMINS
pub fn next_admin_counter(store: &mut dyn Storage) -> Result<u8, ContractError> {
    let id = ADMIN_COUNTER.may_load(store)?.unwrap_or_default() + 1;
    ADMIN_COUNTER.save(store, &id);
    Ok(id)
}

/**
 * Operator State
 */

pub const OPERATORS: Map<U8Key, Addr> = Map::new("admins");
/// counter used to create U8Keys
pub const OPERATOR_COUNTER: Item<u8> = Item::new("admin_counter");
/// function to grab the next key for OPERATORS
pub fn next_operator_counter(store: &mut dyn Storage) -> Result<u8, ContractError> {
    let id = OPERATOR_COUNTER.may_load(store)?.unwrap_or_default() + 1;
    OPERATOR_COUNTER.save(store, &id);
    Ok(id)
}


// TODO: 
// * Whitelist of projects that can cross
