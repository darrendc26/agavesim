// SimTransaction, account metadata, results
#![allow(dead_code)]
use solana_sdk::pubkey::Pubkey;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct SchedulerTxn {
    pub id: String,

    pub priority: u64,

    pub write_lock: HashSet<Pubkey>,

    pub read_lock: HashSet<Pubkey>,

    pub compute_units: u64,
}
