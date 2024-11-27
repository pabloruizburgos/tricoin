# tricoin
Introduction to blockchain Bitcoin-like in Rust.

### simpler-version branch
_What is it?_ 
This is the same exact approach followed on the main branch, except the fact that the handling of transactions is almost omitted. Here, we just take the 'data' related to a certain block and treat it as a simple string. This helps us understand the functions purely of the block without diving too deep in the transactions operations linked to each block. Having less functions (and not splitting a block into the block header and the transactions data itself), a block is treated as a whole.
