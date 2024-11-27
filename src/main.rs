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
    let transactions: Vec<Transaction> = Transaction::create_transactions();

    // Add new blocks to the blockchain
    println!("\nMining new blocks...");
    blockchain.add_block(transactions.clone());
    //blockchain.add_block(transactions.clone());
    //blockchain.add_block(transactions.clone());

    // Verify blockchain integrity
    if blockchain.is_valid() {
        blockchain.display();
    } else {
        eprintln!("The blockchain has been corrupted!");
    }
}
