mod block;
mod blockchain;
mod transaction;
mod utils;

use blockchain::Blockchain;
use transaction::Transaction;

// TODO: add transactions instead of data directly
fn main() {
    let mut blockchain = Blockchain::new();
    print!("Creation of the blockchain:");
    blockchain.display();

    // Add transactions
    let transaction1 = Transaction::create_transactions();
    let transaction2 = vec![Transaction::new(
        "Bob".to_string(),
        "Eve".to_string(),
        21.34,
        0.01,
        "Regalo cumple Sebas".to_string(),
    )];

    let transaction3: Vec<Transaction> = vec![Transaction::new(
        "Sophia".to_string(),
        "Marco".to_string(),
        10.00,
        0.01,
        String::new(),
    )];
    // Add new blocks to the blockchain
    println!("\nMining new blocks...");
    blockchain.add_block(transaction1.clone());
    blockchain.add_block(transaction2);
    blockchain.add_block(transaction3);

    // Verify blockchain integrity
    if blockchain.is_valid() {
        blockchain.display();
    } else {
        eprintln!("The blockchain has been corrupted!");
    }
}
