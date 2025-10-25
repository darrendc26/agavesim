// parse JSON txn format → SimTransaction
#![allow(non_snake_case)]
use crate::types::SanitizedTxn;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTxn {
    pub jsonrpc: String,
    pub result: ResultData,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultData {
    pub blockTime: u64,
    pub meta: MetaData,
    pub slot: u64,
    pub transaction: TxnData,
    pub version: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    pub computeUnitsConsumed: u64,
    pub costUnits: u64,
    pub err: Option<String>,
    pub fee: u64,
    pub innerInstructions: Vec<Value>,
    pub loadedAddresses: LoadedAddresses,
    pub logMessages: Vec<String>,
    pub postBalances: Vec<u64>,
    pub postTokenBalances: Vec<serde_json::Value>,
    pub preBalances: Vec<u64>,
    pub preTokenBalances: Vec<serde_json::Value>,
    pub rewards: Vec<serde_json::Value>,
    pub status: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedAddresses {
    pub readonly: Vec<String>,
    pub writable: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxnData {
    pub message: Message,
    pub signatures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(deserialize_with = "deserialize_pubkeys")]
    pub accountKeys: Vec<Pubkey>,
    pub addressTableLookups: Vec<serde_json::Value>,
    pub header: Header,
    pub instructions: Vec<Instruction>,
    pub recentBlockhash: String,
}

// Custom deserializer for Vec<Pubkey> from base58 strings
fn deserialize_pubkeys<'de, D>(deserializer: D) -> Result<Vec<Pubkey>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Vec<String> = Vec::deserialize(deserializer)?;
    strings
        .into_iter()
        .map(|s| Pubkey::from_str(&s).map_err(serde::de::Error::custom))
        .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub numReadonlySignedAccounts: u8,
    pub numReadonlyUnsignedAccounts: u8,
    pub numRequiredSignatures: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub accounts: Vec<u8>,
    pub data: String,
    pub programIdIndex: u8,
    #[serde(default)]
    pub stackHeight: Option<u8>,
}

pub fn create_sanitized_txn(json_txn: RawTxn) -> SanitizedTxn {
    let id = json_txn.result.transaction.signatures[0].to_string();
    let priority = json_txn.result.meta.fee;

    // Parse static account keys from the message
    let account_keys = &json_txn.result.transaction.message.accountKeys;
    let header = &json_txn.result.transaction.message.header;

    // Calculate writable and readonly accounts from static keys
    // Writable = all accounts except readonly signed and readonly unsigned
    let num_writable_signed = header.numRequiredSignatures - header.numReadonlySignedAccounts;
    let num_writable_unsigned = account_keys.len() as u8
        - header.numRequiredSignatures
        - header.numReadonlyUnsignedAccounts;
    let num_writable = num_writable_signed + num_writable_unsigned;

    let mut write_lock = HashSet::new();
    let mut read_lock = HashSet::new();

    // Add static writable accounts
    for i in 0..num_writable as usize {
        write_lock.insert(account_keys[i]);
    }

    // Add static readonly accounts
    for i in num_writable as usize..account_keys.len() {
        read_lock.insert(account_keys[i]);
    }

    // Add writable accounts from address lookup tables (loadedAddresses)
    for writable_addr in &json_txn.result.meta.loadedAddresses.writable {
        if let Ok(pubkey) = Pubkey::from_str(writable_addr) {
            write_lock.insert(pubkey);
        }
    }

    // Add readonly accounts from address lookup tables (loadedAddresses)
    for readonly_addr in &json_txn.result.meta.loadedAddresses.readonly {
        if let Ok(pubkey) = Pubkey::from_str(readonly_addr) {
            read_lock.insert(pubkey);
        }
    }

    let compute_units = json_txn.result.meta.computeUnitsConsumed;

    SanitizedTxn {
        id,
        priority,
        write_lock,
        read_lock,
        compute_units,
    }
}
