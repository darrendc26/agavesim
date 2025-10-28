// greedy scheduler logic
use crate::types::SchedulerTxn;

pub type Slot = Vec<SchedulerTxn>;
pub type Schedule = Vec<Slot>;

pub struct SchedulerConfig {
    pub max_cu_per_slot: u64,
    pub max_txns_per_slot: usize,
}
