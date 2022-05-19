use cosmwasm_std::Addr;
use cw721::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Initial admin addresses
    pub admins: Vec<String>,
    /// Initial operator addresses
    pub operators: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Admin messages
    /// Update the contract's admins
    UpdateAdmins {
        /// The addresses to add
        add: Option<Vec<String>>,
        /// The addresses to remove
        remove: Option<Vec<String>>,
    },

    /// Update the contract's operators
    UpdateOperators {
        /// The addresses to add
        add: Option<Vec<String>>,
        /// The addresses to remove
        remove: Option<Vec<String>>,
    },

    /// Transfer ownership of NFT to the new owner
    /// * contract_address, token_id is the key for our NFTs
    ReleaseNft {
        /// Address of the recipient
        recipient: String,
        /// The contract address for the NFT
        contract_address: String,
        /// The token_id for the NFT
        token_id: String,
    },

    /// Accept cw721 NFT
    /// * https://docs.cosmwasm.com/cw-plus/0.9.0/cw721/spec/#receiver
    ReceiveNft(Cw721ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns all of the contract's admins.
    /// `start_after` and `limit` are required for pagination.
    AllAdmins {
        /// The last admin address returned in the previous query
        start_after: Option<String>,
        /// The maximum number of addresses to return
        limit: Option<u32>,
    },

    /// Returns all of the contract's operators.
    /// `start_after` and `limit` are required for pagination.
    AllOperators {
        /// The last admin address returned in the previous query
        start_after: Option<String>,
        /// The maximum number of addresses to return
        limit: Option<u32>,
    },

    /// Return all of the NFTs stored in the contract
    /// `start_after` and `limit` are required for pagination.
    AllNfts {
        /// The last admin address returned in the previous query
        start_after: Option<String>,
        /// The maximum number of addresses to return
        limit: Option<u32>,
    },
}
