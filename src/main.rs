mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let blockchain = Blockchain::new();
    blockchain.display();
}
