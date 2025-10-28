// greedy scheduler logic
use crate::lock_table::LockTable;
use crate::priority::PriorityQueue;
use crate::types::SchedulerTxn;

pub struct Scheduler {
    pub lock_table: LockTable,
    pub queue: PriorityQueue,
    pub attempted_txns: Vec<SchedulerTxn>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            lock_table: LockTable::new(),
            queue: PriorityQueue::new(10),
            attempted_txns: Vec::new(),
        }
    }

    pub fn add_txn_to_queue(&mut self, txn: SchedulerTxn) -> Result<(), String> {
        self.queue.push(txn)
    }

    pub fn add_txn_to_slot(&mut self) {
        while self.queue.len() > 0 {
            let txn = self.queue.pop().unwrap();

            // check conflict
            if self.lock_table.can_lock(&txn.write_lock, &txn.read_lock) {
                // schedule it
                if self
                    .lock_table
                    .lock(&txn.write_lock, &txn.read_lock)
                    .is_err()
                {
                    self.attempted_txns.push(txn);
                }
            } else {
                self.attempted_txns.push(txn);
            }
        }
    }
}
