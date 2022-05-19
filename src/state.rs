use std::any::type_name;

use cosmwasm_std::{Addr, StdError, StdResult, Storage};
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// TODO:
// * Whitelist of projects that can cross

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    /// The address of the destination contract on Secret Network
    secret_address: Addr,
    /// The address of the user that sent the token to the bridge
    sender: Addr,
    /// Whether the token has been successfully bridged to Secret Network
    is_bridged: bool,
}

/// mapping of (contract_address, token_id) -> TokenInfo
pub const TOKENS: Map<(&Addr, &str), TokenInfo> = Map::new("tokens_locked");

/*
 *
 * Auth State
 *
 */

/// Key to access the list of admins
/// * `ADMINS`: Vec\<CanonicalAddr>
pub const ADMINS_KEY: &[u8] = b"admins";
/// Key to access the list of operators
/// * `OPERATORS`: Vec\<CanonicalAddr>
pub const OPERATORS_KEY: &[u8] = b"operators";

/*
 *
 *  Taken from Stashh's bridge escrow contract
 *
 */

/// Returns StdResult<()> resulting from saving an item to storage
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item should go to
/// * `key` - a byte slice representing the key to access the stored item
/// * `value` - a reference to the item to store
pub fn save<T: Serialize>(storage: &mut dyn Storage, key: &[u8], value: &T) -> StdResult<()> {
    storage.set(
        key,
        &bincode2::serialize(value).map_err(|e| StdError::serialize_err(type_name::<T>(), e))?,
    );
    Ok(())
}

/// Returns StdResult<T> from retrieving the item with the specified key.  Returns a
/// StdError::NotFound if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn load<T: DeserializeOwned>(storage: &mut dyn Storage, key: &[u8]) -> StdResult<T> {
    bincode2::deserialize(
        &storage
            .get(key)
            .ok_or_else(|| StdError::not_found(type_name::<T>()))?,
    )
    .map_err(|e| StdError::parse_err(type_name::<T>(), e))
}
