use rust_decimal::Decimal; // NOTE: maybe use float and ok??
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: Decimal,
    pub fee: Decimal,
    pub message: String,
}

impl Transaction {
    pub fn new(
        sender: String,
        receiver: String,
        amount: Decimal,
        fee: Decimal,
        message: String,
    ) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            fee,
            message,
        }
    }

    pub fn display(&self) -> String {
        format!(
            "Sender: {}
        Receiver: {}
        Amount: {}
        Fee: {}
        Message: {}\n",
            self.sender, self.receiver, self.amount, self.fee, self.message
        )
    }

    pub fn create_transactions() -> Vec<Transaction> {
        let mut transactions = Vec::new();
        let transaction1 = Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            Decimal::new(10, 1),
            Decimal::new(1, 1),
            String::new(),
        );
        let transaction2 = Transaction::new(
            "KLM S.L.".to_string(),
            "Pablo".to_string(),
            Decimal::new(25000, 2),
            Decimal::new(2, 2),
            String::new(),
        );
        let transaction3 = Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            Decimal::new(2315, 2),
            Decimal::new(2, 1),
            String::new(),
        );
        transactions.push(transaction1);
        transactions.push(transaction2);
        transactions.push(transaction3);
        transactions
    }
}
