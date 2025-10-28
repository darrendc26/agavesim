// entry: drives scheduler + bank + svm runner

mod loader;
mod priority;
mod scheduler;
mod txn_json;
mod types;
use loader::create_sanitized_txn;
use priority::PriorityQueue;
use txn_json::{JSON1, JSON2};

fn main() {
    let json_txn: loader::RawTxn = serde_json::from_str(JSON1).unwrap();
    let json_txn_2: loader::RawTxn = serde_json::from_str(JSON2).unwrap();

    let txn = create_sanitized_txn(json_txn);
    let txn_2 = create_sanitized_txn(json_txn_2);

    println!("{:?}", txn);
    println!("{:?}", txn_2);
    println!("=========================================================================");

    let mut queue = PriorityQueue::new(10);
    queue.push(txn_2).unwrap();
    queue.push(txn).unwrap();

    println!("{:?}", queue.len());
    println!("{:?}", queue.peek());
    println!("{:?}", queue.pop());
}
