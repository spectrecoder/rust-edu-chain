use crate::block::Block;
use crate::transaction::Transaction;
use crate::merkle_proof::MerkleProof;
use crate::MAX_TRANSACTIONS_PER_BLOCK;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;


#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    mempool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Self::create_genesis_block()],
            mempool: Vec::new(),
        }
    }

    fn create_genesis_block() -> Block {
        // Create a block with fixed data and empty previous_hash
        let timestamp = chrono::Utc::now().timestamp();

        let mut genesis_block = Block {
            id: 0,
            timestamp,
            previous_hash: None,
            hash: None,
            merkle_root: None,
            transactions: Vec::new(),
        };

        genesis_block.merkle_root = genesis_block.calculate_merkle_root();

        // Finally, calculate the hash of the genesis block including its Merkle root
        genesis_block.hash = genesis_block.calculate_hash();

        genesis_block
    }

    fn is_valid_block(&self, block: &Block) -> bool {
        block.calculate_hash() == block.hash
    }

    // Helper methods:

    pub fn get_chain(&self) -> &Vec<Block> {
        self.chain.as_ref()
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn get_block_by_id(&self, id: u32) -> Option<&Block> {
        self.chain.iter().find(|block| block.id == id)
    }

    pub fn get_block_height(&self) -> u32 {
        self.chain.len() as u32 - 1
    }

    pub fn validate_chain(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate().skip(1) {
            let prev_block = self.chain.get(i - 1).unwrap();

            let hash = &prev_block.hash;
            match block.previous_hash != *hash || !self.is_valid_block(block) {
                true => return false,
                false => (),
            }
        }
        true
    }

    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if std::path::Path::new(path).exists() {
            let data = std::fs::read_to_string(path)?;
            if !data.trim().is_empty() {
                // Check if the file is not just whitespace
                *self = serde_json::from_str(&data)?;
            }
            // If the file is empty or only contains whitespace, do nothing
        }
        Ok(())
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn add_transaction(&mut self, sender: String, receiver: String, amount: u64) {
        let transaction = Transaction::new(sender, receiver, amount);

        // Add the new transaction to the mempool
        self.mempool.push(transaction);

        // Check if the mempool has reached the threshold to create a new block
        if self.mempool.len() >= MAX_TRANSACTIONS_PER_BLOCK {
            self.create_block_from_mempool();
        }
    }

    fn create_block_from_mempool(&mut self) {
        assert!(
            self.mempool.len() >= MAX_TRANSACTIONS_PER_BLOCK,
            "Not enough transactions in the mempool to create a block"
        );

        // Assuming you have a method to get the previous block's hash
        let previous_hash = Some(self.get_latest_block_hash());
        let timestamp = chrono::Utc::now().timestamp();
        let transactions = self
            .mempool
            .drain(..MAX_TRANSACTIONS_PER_BLOCK)
            .collect::<Vec<Transaction>>();

        let block_hash = Some(vec![0, 32]); // Placeholder

        let mut new_block = Block {
            id: self.chain.len() as u32,
            timestamp,
            transactions,
            previous_hash,
            hash: block_hash, // This should be calculated based on block content
            merkle_root: None,
        };

        new_block.merkle_root = new_block.calculate_merkle_root();

        new_block.hash = new_block.calculate_hash();

        // print_json method for Block
        //new_block.print_json().unwrap();

        self.chain.push(new_block);
    }

    fn get_latest_block_hash(&self) -> Vec<u8> {
        if let Some(block) = self.chain.last() {
            // Check if the block has a hash and clone it if present
            if let Some(hash) = &block.hash {
                hash.clone()
            } else {
                // Return a default hash if the block doesn't have one
                vec![0; 32] // Assuming SHA-256 or similar
            }
        } else {
            // Return a default hash if there are no blocks in the chain
            vec![0; 32] // 32 bytes of 0s, assuming SHA-256 or similar
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        // Serialize the blockchain to a pretty-printed JSON string
        let json = serde_json::to_string_pretty(&self)?;
        println!("{}", json);
        Ok(())
    }

    // Find a transaction within a block, identify its path to the Merkle root,
    // and collect sibling hashes along this path to verify the transaction is
    // on in the block.
    pub fn generate_merkle_proof(&self, transaction_hash: &Vec<u8>) -> Option<MerkleProof> {
        // Iterate through the blockchain to find the block containing the transaction
        for block in &self.chain {
            // Check if the block contains the transaction
            if block
                .transactions
                .iter()
                .any(|tx| tx.calculate_hash() == *transaction_hash)
            {
                // Generate the Merkle path for that transaction
                if let Some(path) = block.generate_merkle_path(transaction_hash) {
                    // Construct and return the MerkleProof object
                    return Some(MerkleProof {
                        leaf: transaction_hash.clone(),
                        path,
                    });
                }
                break;
            }
        }
        None
    }
}

impl Drop for Blockchain {
    fn drop(&mut self) {
        // Print a message that this is the final Blockchain
        //println!("Final Blockchain instance:");
        //self.print_json();

        // Print a message when the Blockchain instance is dropped and saved to a file
        println!("Dropping Blockchain instance pesrsistently to blockchain.json.");
        // Save the blockchain to a file before dropping the instance

        // Attempt to save to file here
        let _ = self.save_to_file("./blockchain.json");
    }
}

