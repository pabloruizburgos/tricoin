# tricoin
This is a simple introduction made by me to understand **blockchain**. *"If you don't get your hands dirty you aren't learning fully".*

Made in Rust (just like Solana), tricoin is a "fake" cryptocurrency built based on the principles of Bitcoin (the first crypto launched publicly). The idea is to use the same concepts used in the cration on Bitcoin explained, so the grasp of what really is a crypto is as clear as water.

### Introduction to blockchain
A cryptocurrency really isn't much different from a digital currency (of course, there are different types...), so the key behind them is really this technology called **blockchain**. As its name suggests, it is a "chain" of "blocks". Now, the next logic question is *"What is a block then?"*. A block isn't a "coin", it is really how we save any transaction made with the actual coins (money) of this economy.

Every block needs to be identified, so we define an *index*, which usually is just the position of the block in the chain. Also the time when the block was created is a relevant information, so a *timestamp* is created. Then we need the *data*, this is all the information that defines transactions, such as sender, receiver, amount, date of when it took place and any other information relevant in a transaction. But we need to "connect" the blocks to form the chain! This is done by making a *hash* of the information inside a block (also used to identify blocks), and also including the *hash of the previous block* we now have blocks connected. As simple as that.

### How a block is created
Here is when the concept of *"mining"* appears. Bitcoin was made so it was scarce like gold (only 21,000,000 BTC will ever exist). This is said to be a way to control the inflation on an economy, as we cannot "create" bitcoins out of thin air. To guarantee a certain amount of computational power behind the blockchain network (we will talk about this in the next section) there is introduced a "Proof of Work" (PoW). What it does is make sure that, before adding a new block to the blockchain, this block satisfies a certain condition. In Bitcoin, the condition is that the hash of the block should start with a certain number of zeros. The number of leading zeros is set by a variable *difficulty*. At the start, the so called "genesis block" hash, with a difficulty of 1, was: 000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f. Now the difficulty is much higher.

So to create a block the only thing to be done (after defining the variables of course) is just finding the hash that satisfies the difficulty. Since a hash produces the same output given the same input, we introduce a *nonce* (just a number) that we can modify to get a different hash every time until a valid one is found. Once a block is mined, the other users on the blockchain validate it and it is then added to the blockchain.

*"We talked about "mining a block" but the usual term is "mining bitcoin"?"* Well, as a way to incentivize the collaboration to create blocks and make the blockchain larger and, consequently, better safeguarded, a reward is given to anyone that creates a new block (finds a valid hash :) ). This started being a high reward of 50 BTC per block (at the time almost $0) and now, at the time of March 2025, it's 6.25 BTC (around $480,000). The times when the reward is made smaller is called "halving" (as the reward price is halved each time this event occurs).

### Decentralized?
After understanding the previous concepts, a question about who forms the blockchain should have arised already. The idea behind bitcoin is to have a decentralized economic system. This is done by making each person inside the economy (owning bitcoin is enough) a participant in the **bitcoin network**. Instead of having a central bank that handles the balance and transactions made, the blockchain is like a book where a copy of every movement of bitcoins is written. Every user of the network has a copy of this book so, in order to make a new transaction (that would be included in a block), at least 51% of the people that have this book have to "accept" it. This is a fantastic way of avoiding fraud as well!

No central bank needed, each person is participant of the blockchain and serves as a judge for it.

---
This code just defines a block from scratch, how a new one is mined and its incorporations on the blockchain, but made in local with no other nodes to be included in the "tricoin network". Feel free to make any suggestions or improvements on it!!!

## 'tricoin-lite' branch
This is the same exact approach followed on the main branch, except the fact that the handling of transactions is almost omitted. Here, we just take the 'data' related to a certain block and treat it as a simple string. This helps us understand the functions purely of the block without diving too deep in the transactions operations linked to each block. Having less functions (and not splitting a block into the block header and the transactions data itself), a block is treated as a whole.
