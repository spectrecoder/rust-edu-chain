use sha2::{Sha256, Digest};


pub struct Block {
    pub id: u32,
    pub timestamp: i64,
    pub data: Vec<String>,
    pub previous_hash: Option<Vec<u8>>,
    pub hash: Option<Vec<u8>>,
    pub merkle_root: Option<Vec<u8>>, 
}

impl Block {
    fn calculate_hash(&self) -> Option<Vec<u8>> {
    
        // <--- Return Option<Vec<u8>>
        // Use a SHA-256 library to calculate the hash of the block data
        let timestamp_bytes = self.timestamp.to_le_bytes();
		let merkle_root = self.calculate_merkle_root().unwrap_or_else(Vec::new);
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
        if self.data.is_empty() {
            return None;
        }

        let mut leaf_hashes = self.data.iter()
            .map(|item| {
                let digest = Sha256::digest(item.as_bytes());
                digest.to_vec()
            })
            .collect::<Vec<_>>();

        while leaf_hashes.len() > 1 {
            if leaf_hashes.len() % 2 != 0 { // Ensure even number of leaves
                leaf_hashes.push(leaf_hashes.last().unwrap().clone());
            }

            leaf_hashes = leaf_hashes.chunks(2)
                .map(|chunk| {
                    let mut hasher = Sha256::new();
                    hasher.update(&chunk[0]);
                    hasher.update(&chunk[1]);
                    hasher.finalize().to_vec()
                })
                .collect();
        }

        leaf_hashes.first().cloned()
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

fn main() {
    let mut blockchain = Blockchain::new();

    // Example data for the first block
    let data_to_add = vec![
        "Transaction 1: Alice sends Bob 5 coins".to_string(),
        "Transaction 2: Charlie sends Dana 3 coins".to_string(),
    ];

    // Attempt to create and add the first new block
    if let Some(new_block) = blockchain.get_new_block(data_to_add) {
        println!("New block created with ID: {}", new_block.id);
        blockchain.add_block(new_block).unwrap(); // Assuming this method exists
    } else {
        println!("[Error] Couldn't create new block");
    }

    // Assuming get_latest_block() returns a reference or a copy of the latest block
    // and that the Block data is now a Vec<String>, requiring joining for display
    println!("Latest block data: {:?}", blockchain.get_latest_block().data.join(", "));

    // Example data for the second block
    let data_for_second_block = vec![
        "Block 2 This is a test!".to_string(),
    ];

    // Attempt to create and add the second new block
    if let Some(new_block) = blockchain.get_new_block(data_for_second_block) {
        println!("New block created with ID: {}", new_block.id);
        blockchain.add_block(new_block).unwrap(); // Assuming this method exists
    } else {
        println!("[Error] Couldn't create new block");
    }

    // Print the latest block data
    println!("Latest block data: {:?}", blockchain.get_latest_block().data.join(", "));

    // Validate chain integrity
    if blockchain.validate_chain() {
        println!("Chain is valid!");
    } else {
        println!("Chain is invalid!");
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Self::create_genesis_block()],
        }
    }

    fn create_genesis_block() -> Block {
        // Create a block with fixed data and empty previous_hash
        let timestamp = chrono::Utc::now().timestamp();
        let genesis_data = vec![String::from("Genesis Block")];
        
        
        let mut genesis_block = Block {
            id: 0,
            timestamp,
            data: genesis_data,
            previous_hash: None,
            hash: None,
            merkle_root: None,
        };
        
        genesis_block.merkle_root = genesis_block.calculate_merkle_root();

    	// Finally, calculate the hash of the genesis block including its Merkle root
    	genesis_block.hash = genesis_block.calculate_hash();
    	
    	genesis_block

    }

    pub fn get_new_block(&self, data: Vec<String>) -> Option<Block> {
        // Access the last block directly:
        let prev_block = self.chain.last().unwrap();

        let timestamp = chrono::Utc::now().timestamp();
        
        let mut new_block = Block {
            id: prev_block.id + 1,
            timestamp,
            data,
            previous_hash: prev_block.hash.clone(),
            hash: None,
            merkle_root: None,
        };
        
        new_block.merkle_root = new_block.calculate_merkle_root();

        // Calculate the hash for the new block, including its Merkle root.
        new_block.hash = new_block.calculate_hash();

        Some(new_block)
    }

    pub fn add_block(&mut self, mut block: Block) -> Result<(), String> {
        let _prev_block = self.chain.last().unwrap();

        if block.hash.is_none() {
            block.hash = block.calculate_hash();
        }

        if self.is_valid_block(&block) {
            self.chain.push(block);
            Ok(())
        } else {
            Err(String::from("Invalid block"))
        }
    }

    fn is_valid_block(&self, block: &Block) -> bool {
        block.calculate_hash() == block.hash
    }

    // New methods:

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
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

    pub fn get_block_data(&self, id: u32) -> Option<String> {
    	// Attempt to get the block by its ID
    	let block = self.get_block_by_id(id)?;

    	// Join the Vec<String> data into a single String, separating entries with a delimiter
    	// Here, we use a newline character for readability, but you could use ", " or any other delimiter
    	let data_as_string = block.data.join("\n");

    	Some(data_as_string)
	}

    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }
}
