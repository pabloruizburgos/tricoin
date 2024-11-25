mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    print!("Creation of the blockchain: ");
    blockchain.display();

    println!("Mining new block...");
    blockchain.add_block("Transaction of the first <real> block".to_string());

    // Verify blockchain integrity
    if blockchain.is_valid() {
        blockchain.display();
    } else {
        eprintln!("The blockchain has been corrupted!");
    }
}
