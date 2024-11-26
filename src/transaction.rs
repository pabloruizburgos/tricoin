pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u8,
    pub fee: u8,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u8, fee: u8) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            fee,
        }
    }

    pub fn display(&self) {
        print!(
            "Sender: {}
            Receiver: {}
            Amount: {}
            Fee: {}\n",
            self.sender, self.receiver, self.amount, self.fee
        );
    }
}
