// conflict tracking & locking
use solana_sdk::{account, pubkey::Pubkey};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct LockTable {
    pub write_lock: HashSet<Pubkey>,
    pub read_lock: HashSet<Pubkey>,
}

impl LockTable {
    pub fn new() -> Self {
        Self {
            write_lock: HashSet::new(),
            read_lock: HashSet::new(),
        }
    }

    pub fn can_lock(
        &self,
        write_accounts: &HashSet<Pubkey>,
        read_accounts: &HashSet<Pubkey>,
    ) -> bool {
        for account in write_accounts {
            if self.write_lock.contains(account) {
                return false;
            }
            if self.read_lock.contains(account) {
                return false;
            }
        }

        for account in read_accounts {
            if self.write_lock.contains(account) {
                return false;
            }
        }

        true
    }

    pub fn lock(
        &mut self,
        write_accounts: &HashSet<Pubkey>,
        read_accounts: &HashSet<Pubkey>,
    ) -> Result<(), String> {
        if !self.can_lock(write_accounts, read_accounts) {
            return Err("Cannot lock".to_string());
        }

        for account in write_accounts {
            self.write_lock.insert(*account);
        }

        for account in read_accounts {
            self.read_lock.insert(*account);
        }
        Ok(())
    }

    pub fn clear_lock(&mut self) {
        self.write_lock.clear();
        self.read_lock.clear();
    }
}
