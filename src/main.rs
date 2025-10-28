// entry: drives scheduler + bank + svm runner

mod loader;
mod lock_table;
mod priority;
mod scheduler;
mod txn_json;
mod types;
use loader::create_sanitized_txn;
use lock_table::LockTable;
use priority::PriorityQueue;
use scheduler::Scheduler;
use txn_json::{JSON1, JSON2, JSON3, JSON4};
fn main() {
    let json_txn: loader::RawTxn = serde_json::from_str(JSON1).unwrap();
    let json_txn_2: loader::RawTxn = serde_json::from_str(JSON2).unwrap();
    let json_txn_3: loader::RawTxn = serde_json::from_str(JSON3).unwrap();
    let json_txn_4: loader::RawTxn = serde_json::from_str(JSON4).unwrap();
    let txn = create_sanitized_txn(json_txn);
    let txn_2 = create_sanitized_txn(json_txn_2);
    let txn_3 = create_sanitized_txn(json_txn_3);
    let txn_4 = create_sanitized_txn(json_txn_4);

    println!("{:?}\n", txn);
    println!("{:?}\n", txn_2);
    println!("{:?}\n", txn_3);
    println!("{:?}\n", txn_4);
    println!("=========================================================================\n");

    let mut queue = PriorityQueue::new(10);
    queue.push(txn_2.clone()).unwrap();
    queue.push(txn.clone()).unwrap();
    queue.push(txn_3.clone()).unwrap();
    queue.push(txn_4.clone()).unwrap();

    let mut scheduler = Scheduler::new();
    scheduler.add_txn_to_queue(txn_2.clone()).unwrap();
    scheduler.add_txn_to_queue(txn.clone()).unwrap();
    scheduler.add_txn_to_queue(txn_3.clone()).unwrap();
    scheduler.add_txn_to_queue(txn_4.clone()).unwrap();
    scheduler.add_txn_to_slot();

    println!("Queue: {:?}\n", scheduler.queue);
    println!("Write Lock Table: {:?}\n", scheduler.lock_table.write_lock);
    println!("Read Lock Table: {:?}\n", scheduler.lock_table.read_lock);

    for txn in scheduler.attempted_txns {
        println!("Conflicting Txn: {:?}\n", txn);
    }
}
