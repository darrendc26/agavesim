// priority queue / fee heuristics
use crate::types::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
pub struct PriorityTxn {
    txn: SchedulerTxn,
    insert_order: u64,
}

impl PartialEq for PriorityTxn {
    fn eq(&self, other: &Self) -> bool {
        self.txn.priority == other.txn.priority && self.insert_order == other.insert_order
    }
}

impl Eq for PriorityTxn {}

impl PartialOrd for PriorityTxn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityTxn {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.txn.priority.cmp(&other.txn.priority) {
            Ordering::Equal => self.insert_order.cmp(&other.insert_order),
            other => other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PriorityQueue {
    heap: BinaryHeap<PriorityTxn>,
    insert_counter: u64,
    max_size: u64,
}

impl PriorityQueue {
    pub fn new(max_size: u64) -> Self {
        Self {
            heap: BinaryHeap::new(),
            insert_counter: 0,
            max_size,
        }
    }

    pub fn push(&mut self, txn: SchedulerTxn) -> Result<(), String> {
        if self.heap.len() >= self.max_size as usize {
            return Err("Priority queue is full".to_string());
        }

        let priority_txn = PriorityTxn {
            txn,
            insert_order: self.insert_counter,
        };

        self.heap.push(priority_txn);
        self.insert_counter += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<SchedulerTxn> {
        self.heap.pop().map(|txn| txn.txn)
    }

    pub fn peek(&self) -> Option<&SchedulerTxn> {
        self.heap.peek().map(|txn| &txn.txn)
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }
}
