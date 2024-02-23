mod merkle_proof;

use merkle_proof::MerkleProof;
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
        let merkle_root = self.calculate_merkle_root().unwrap_or_else(|| vec![0; 32]); 
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
                .collect::<Vec<[u8; 32]>>(); 
        }

        Some(leaf_hashes.first()?.to_vec()) // Convert the first (and only) hash array to Vec<u8>
    }

    pub fn generate_merkle_path(&self, transaction_hash: &Vec<u8>) -> Option<Vec<(Vec<u8>, bool)>> {
        let transaction_hashes = self
            .transactions
            .iter()
            .map(|tx| tx.calculate_hash())
            .collect::<Vec<_>>();
        let mut tree_layers = vec![transaction_hashes]; // The bottom layer of the tree

        // Build the tree, layer by layer
        while tree_layers.last().unwrap().len() > 1 {
            let prev_layer = tree_layers.last().unwrap();
            let new_layer = prev_layer
                .chunks(2)
                .map(|chunk| {
                    let left = &chunk[0];
                    let right = chunk.get(1).unwrap_or(left); // Handle odd number of elements

                    // Create a new Vec<u8> and extend it with the bytes from left and right
                    let mut combined = Vec::new();
                    combined.extend_from_slice(left);
                    combined.extend_from_slice(right);

                    // Hash the combined vector
                    Block::hash_function(&combined)
                })
                .collect::<Vec<_>>();
            tree_layers.push(new_layer);
        }

        // Find the transaction index in the bottom layer
        let index = tree_layers[0]
            .iter()
            .position(|hash| hash == transaction_hash)?;
        let mut path = Vec::new();
        let mut current_index = index;

        // Collect the sibling hashes and direction for each layer
        for layer in tree_layers.iter().take(tree_layers.len() - 1) {
            let is_right_sibling = current_index % 2 == 1;
            let sibling_index = if is_right_sibling {
                current_index - 1
            } else {
                current_index + 1
            };
            let sibling_hash = layer.get(sibling_index).cloned().unwrap_or_default();

            path.push((sibling_hash, is_right_sibling));
            current_index /= 2; // Move up to the next layer
        }

        Some(path)
    }

    fn hash_function(data: &[u8]) -> Vec<u8> {
        Sha256::digest(data).to_vec()
    }

    pub fn construct_merkle_tree(&self) -> Vec<u8> {
        let mut layer = self
            .transactions
            .iter()
            .map(|tx| tx.calculate_hash())
            .collect::<Vec<_>>();

        while layer.len() > 1 {
            layer = Self::construct_merkle_layer(&layer);
        }

        layer.first().cloned().unwrap_or_else(|| vec![])
    }

    fn construct_merkle_layer(current_layer: &[Vec<u8>]) -> Vec<Vec<u8>> {
        current_layer
            .chunks(2)
            .map(|chunk| {
                let left = &chunk[0];
                let right = chunk.get(1).unwrap_or(left);

                let mut hasher = Sha256::new();
                hasher.update(left);
                hasher.update(right);
                hasher.finalize().to_vec()
            })
            .collect()
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
            previous_hash: self
                .previous_hash
                .as_ref()
                .map_or_else(String::new, |hash| to_hex_string(hash)),
            // Do the same for hash and merkle_root if they are also Option<Vec<u8>>
            hash: self
                .hash
                .as_ref()
                .map_or_else(String::new, |hash| to_hex_string(hash)),
            merkle_root: self
                .merkle_root
                .as_ref()
                .map_or_else(String::new, |root| to_hex_string(root)),
        }
    }

    // Debug print function for a Block
    pub fn debug_print(&self) {
        println!("Block ID: {}", self.id);
        println!("Timestamp: {}", self.timestamp);
        // Handle Option<Vec<u8>> for previous_hash, merkle_root, and hash
        println!(
            "Previous Hash: {}",
            self.previous_hash
                .as_ref()
                .map_or_else(|| "None".to_string(), |hash| to_hex_string(hash))
        );
        println!(
            "Merkle Root: {}",
            self.merkle_root
                .as_ref()
                .map_or_else(|| "None".to_string(), |root| to_hex_string(root))
        );
        println!(
            "Hash: {}",
            self.hash
                .as_ref()
                .map_or_else(|| "None".to_string(), |hash| to_hex_string(hash))
        );

        println!("Transactions: {:?}", self.transactions);
    }
}

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

fn to_hex_string(bytes: &Vec<u8>) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}

// Run test
fn run_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running test");
    let mut blockchain = Blockchain::new();

    blockchain.load_from_file("./blockchain.json")?;
    println!("Blockchain loaded from file");
    //blockchain.print_json();

    println!("Begin Transactions to mempool");

    // Add 2 * MAX_TRANSACTIONS_PER_BLOCK transactions to the mempool
    blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 5);
    blockchain.add_transaction("Charlie".to_string(), "Dana".to_string(), 3);
    blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 5);
    blockchain.add_transaction("Charlie".to_string(), "Dana".to_string(), 3);
    blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 5);
    blockchain.add_transaction("Charlie".to_string(), "Dana".to_string(), 3);
    blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 5);
    blockchain.add_transaction("Fred".to_string(), "Barney".to_string(), 3);

    // Create a dangling transaction that should be persisted, it won't create a block
    blockchain.add_transaction("George".to_string(), "Henry".to_string(), 5);

    // Validation check
    assert!(
        blockchain.validate_chain(),
        "Blockchain should be valid after adding transactions and creating blocks."
    );

    // Generate a Merkle proof for a transaction
    // Select a transaction hash for which to generate a Merkle proof
    // For simplicity, using the hash of the first transaction in the first non-genesis block
    let transaction_hash = blockchain.chain[1].transactions[0].calculate_hash();

    // Generate a Merkle proof for the selected transaction
    let merkle_proof = blockchain
        .generate_merkle_proof(&transaction_hash)
        .expect("Merkle proof should be generated");

    // Verify the Merkle proof
    let merkle_root_option = blockchain.chain[1].merkle_root.clone(); // Get the Merkle root of the block containing the transaction

    if let Some(merkle_root) = merkle_root_option {
        assert!(
            merkle_proof.verify(&merkle_root),
            "Merkle proof should be valid"
        );
    } else {
        // Handle the case where the Merkle root is not set (e.g., panic or assert with a specific message)
        panic!("Merkle root is not available for the block.");
    }

    println!("Merkle proof is valid");

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
