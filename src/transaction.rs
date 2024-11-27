// use rust_decimal::Decimal;  NOTE: if financial operations supported, may come as handy
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
    pub fee: f32,
    pub message: String,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f32, fee: f32, message: String) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            fee,
            message,
        }
    }

    /*
     * This function transforms the transaction into a concatenatted string of all the info about
     * it and returns it just like that, for it to be hashed later.
     */
    pub fn get_transaction_as_string(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.sender, self.receiver, self.amount, self.fee, self.message
        )
    }

    pub fn display(&self) {
        print!(
            "   
        Sender: {}
        Receiver: {}
        Amount: {}
        Fee: {}
        Message: {}\n",
            self.sender, self.receiver, self.amount, self.fee, self.message
        );
    }

    pub fn create_transactions() -> Vec<Transaction> {
        let mut transactions = Vec::new();
        let transaction1 = Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            10.00,
            0.01,
            String::new(),
        );
        let transaction2 = Transaction::new(
            "KLM S.L.".to_string(),
            "Pablo".to_string(),
            250.00,
            0.20,
            String::new(),
        );
        let transaction3 = Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            43.15,
            0.02,
            String::new(),
        );
        transactions.push(transaction1);
        transactions.push(transaction2);
        transactions.push(transaction3);
        transactions
    }
}
