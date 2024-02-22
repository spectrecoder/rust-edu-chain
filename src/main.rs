use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const MAX_TRANSACTIONS_PER_BLOCK: usize = 4;


#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub id: u32,
    pub timestamp: i64,
    pub previous_hash: Option<Vec<u8>>,
    pub hash: Option<Vec<u8>>,
    pub merkle_root: Option<Vec<u8>>,
    pub transactions: Vec<Transaction>,
}

impl Block {
    fn calculate_hash(&self) -> Option<Vec<u8>> {
        // <--- Return Option<Vec<u8>>
        // Use a SHA-256 library to calculate the hash of the block data
        let timestamp_bytes = self.timestamp.to_le_bytes();
        let merkle_root = self.calculate_merkle_root().unwrap_or_else(|| vec![0; 32]); // Or vec![] depending on your hash function and design
        let mut data_to_hash = Vec::new();

        // Correctly handle previous_hash:
        if let Some(previous_hash) = &self.previous_hash {
            data_to_hash.extend_from_slice(previous_hash);
        }

        data_to_hash.extend_from_slice(&self.id.to_le_bytes());
        data_to_hash.extend_from_slice(&timestamp_bytes); // Reference timestamp_bytes directly
        data_to_hash.extend_from_slice(&merkle_root);

        // Use a SHA-256 library to calculate the hash of the aggregated data.
        let mut hasher = Sha256::new();
        hasher.update(data_to_hash);
        let digest = hasher.finalize();

        Some(digest.to_vec())
    }
     
    fn calculate_merkle_root(&self) -> Option<Vec<u8>> {
        if self.transactions.is_empty() {
            return None;
        }

        let mut leaf_hashes = self
            .transactions
            .iter()
            .map(|transaction| {
                let transaction_data = serde_json::to_string(transaction).unwrap();
                let mut hasher = Sha256::new();
                hasher.update(transaction_data.as_bytes());
                let hash_result = hasher.finalize();
                let hash_array: [u8; 32] = hash_result.into();
                hash_array
            })
            .collect::<Vec<[u8; 32]>>();

        while leaf_hashes.len() > 1 {
            if leaf_hashes.len() % 2 != 0 {
                leaf_hashes.push(*leaf_hashes.last().unwrap());
            }

            leaf_hashes = leaf_hashes
                .chunks(2)
                .map(|chunk| {
                    let mut hasher = Sha256::new();
                    hasher.update(&chunk[0]);
                    hasher.update(&chunk[1]);
                    let hash_result = hasher.finalize();
                    hash_result.into()
                })
                .collect::<Vec<[u8; 32]>>(); // Correctly collecting into a Vec<[u8; 32]>
        }

        Some(leaf_hashes.first()?.to_vec()) // Convert the first (and only) hash array to Vec<u8>
    
    }

	pub fn print_json(&self) -> serde_json::Result<()> {
        let serializable_block = self.to_serializable();
        let json = serde_json::to_string_pretty(&serializable_block)?;
        println!("{}", json);
        Ok(())
    }
    
    fn to_serializable(&self) -> SerializableBlock {
        SerializableBlock {
            id: self.id,
            timestamp: self.timestamp,
            transactions: self.transactions.clone(),
            // Check if previous_hash is Some, then convert to hex, else default to an empty string
            previous_hash: self.previous_hash.as_ref().map_or_else(String::new, |hash| to_hex_string(hash)),
            // Do the same for hash and merkle_root if they are also Option<Vec<u8>>
            hash: self.hash.as_ref().map_or_else(String::new, |hash| to_hex_string(hash)),
            merkle_root: self.merkle_root.as_ref().map_or_else(String::new, |root| to_hex_string(root)),
        }
    }

    // Debug print function for a Block
	pub fn debug_print(&self) {
        println!("Block ID: {}", self.id);
        println!("Timestamp: {}", self.timestamp);
        // Handle Option<Vec<u8>> for previous_hash, merkle_root, and hash
        println!("Previous Hash: {}", self.previous_hash.as_ref().map_or_else(|| "None".to_string(), |hash| to_hex_string(hash)));
        println!("Merkle Root: {}", self.merkle_root.as_ref().map_or_else(|| "None".to_string(), |root| to_hex_string(root)));
        println!("Hash: {}", self.hash.as_ref().map_or_else(|| "None".to_string(), |hash| to_hex_string(hash)));
        // Assuming Transaction implements Debug for {:?} printing
        println!("Transactions: {:?}", self.transactions);
    }
}

// Define a struct for serialization that uses hex strings for byte arrays
#[derive(Serialize)]
pub struct SerializableBlock {
    id: u32,
    timestamp: i64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    merkle_root: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: u64, // or any other relevant fields
}

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

    // New methods:

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
        let transaction = Transaction {
            sender,
            receiver,
            amount,
        };

        // Add the new transaction to the mempool
        self.mempool.push(transaction);

        // Check if the mempool has reached the threshold to create a new block
        if self.mempool.len() >= MAX_TRANSACTIONS_PER_BLOCK {
            self.create_block_from_mempool();
        }
    }

    fn create_block_from_mempool(&mut self) {
    
    	assert!(self.mempool.len() >= MAX_TRANSACTIONS_PER_BLOCK, "Not enough transactions in the mempool to create a block");

        // Assuming you have a method to get the previous block's hash
        let previous_hash = Some(self.get_latest_block_hash());
        let timestamp = chrono::Utc::now().timestamp();
        let transactions = self.mempool.drain(..MAX_TRANSACTIONS_PER_BLOCK).collect::<Vec<Transaction>>();

        // Assuming you have implemented a method to calculate a new block's hash
        let block_hash = Some(vec![0,32]); // Placeholder

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
        new_block.print_json().unwrap();

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
}

impl Drop for Blockchain {
    fn drop(&mut self) {
        // Print a message when the Blockchain instance is dropped and saved to a file
        println!("Dropping Blockchain instance pesrsistently to blockchain.json.");
        // Save the blockchain to a file before dropping the instance
        // Attempt to save to file here
        let _ = self.save_to_file("./blockchain.json");
    }
}

fn to_hex_string(bytes: &Vec<u8>) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect::<String>()
}


// Run test
fn run_test() -> Result<(), Box<dyn std::error::Error>> {
	println!("Running test");
    let mut blockchain = Blockchain::new();

    blockchain.load_from_file("./blockchain.json")?;

    // Add 2 * MAX_TRANSACTIONS_PER_BLOCK transactions to the mempool
    blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 5);
    blockchain.add_transaction("Charlie".to_string(), "Dana".to_string(), 3);
	blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 5);
	blockchain.add_transaction("Charlie".to_string(), "Dana".to_string(), 3);
	blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 5);
	blockchain.add_transaction("Charlie".to_string(), "Dana".to_string(), 3);
	blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 5);
	blockchain.add_transaction("Charlie".to_string(), "Dana".to_string(), 3);
	             
	// Example validation check
	assert!(blockchain.validate_chain(), "Blockchain should be valid after adding transactions and creating blocks.");

    println!("Test completed");
    Ok(())
}



fn main() {
    if let Err(e) = run_test() {
        println!("Error running test: {}", e);
    }

    // Print the contents of the blockchain.json file
    let path = Path::new("blockchain.json");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}
