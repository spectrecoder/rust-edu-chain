use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: u64, // or whatever type 
    pub hash: Vec<u8>,
}

impl Transaction {
	pub fn new(sender: String, receiver: String, amount: u64 ) -> Self {
        let transaction = Transaction {
            sender: sender.clone(),
            receiver: receiver.clone(),
            amount: amount,
            hash: Vec::new(), // Temporary placeholder
        };
        let hash = transaction.calculate_hash(); // Calculate the hash based on current content

		println!("Sender: {}, Receiver: {}, Amount: {}, Transaction hash: {:#?}", sender, receiver, amount, hash);

        // Return the transaction with its hash field correctly populated
        Transaction { hash, ..transaction }
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        let transaction_data = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(transaction_data.as_bytes());
        hasher.finalize().to_vec()
    }

    pub fn hash(&self) -> &Vec<u8> {
        &self.hash
    }
}
