mod block;
mod blockchain;
//mod transaction;
mod utils;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    print!("Creation of the blockchain:");
    blockchain.display();

    println!("\nMining new blocks...");
    blockchain.add_block("Transaction of the first <real> block".to_string());

    // Add two more blocks
    blockchain.add_block("Pablo stole 100bitcoins :c".to_string());
    blockchain.add_block("Transaction of Santander 26-11-2024".to_string());

    // Verify blockchain integrity
    if blockchain.is_valid() {
        blockchain.display();
    } else {
        eprintln!("The blockchain has been corrupted!");
    }
}
