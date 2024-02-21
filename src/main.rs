pub struct Block {
    pub id: u32,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: Option<Vec<u8>>,
    pub hash: Option<Vec<u8>>,
}

impl Block {
    fn calculate_hash(&self) -> Option<Vec<u8>> {
        // <--- Return Option<Vec<u8>>
        // Use a SHA-256 library to calculate the hash of the block data
        let timestamp_bytes = self.timestamp.to_le_bytes();

        let mut data_to_hash = Vec::new();
        // Correctly handle previous_hash:
        if let Some(previous_hash) = &self.previous_hash {
            data_to_hash.extend_from_slice(previous_hash);
        }
        data_to_hash.extend_from_slice(&self.id.to_le_bytes());
        data_to_hash.extend_from_slice(&timestamp_bytes); // Reference timestamp_bytes directly
        data_to_hash.extend_from_slice(self.data.as_bytes());

        let digest = sha256::digest(&data_to_hash);
        let digest: Vec<u8> = digest.into_bytes();
        Some(digest) // Return Some(digest)
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

fn main() {
    let mut blockchain = Blockchain::new();

    // Add some blocks
    // Use the get_new_block function before adding a block
    let new_block = blockchain.get_new_block("Block 1 Hello World!".to_string());
    let block_to_add = match new_block {
        Some(block) => block,
        None => {
            return println!("[Error] Couldn't create new block");
        }
    };

	blockchain.add_block(block_to_add).unwrap();
	
    // Print the latest block data
    println!("Latest block data: {}", blockchain.get_latest_block().data);

	let new_block = blockchain.get_new_block("Block 2 This is a test!".to_string());
    let block_to_add = match new_block {
        Some(block) => block,
        None => {
            return println!("[Error] Couldn't create new block");
        }
    };

    blockchain.add_block(block_to_add).unwrap();

    // Print the latest block data
    println!("Latest block data: {}", blockchain.get_latest_block().data);

    // Verify chain integrity
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
        Block {
            id: 0,
            timestamp,
            data: String::from("Genesis Block"),
            previous_hash: None,
            hash: None,
        }
    }

    pub fn get_new_block(&self, data: String) -> Option<Block> {
        // Access the last block directly:
        let prev_block = self.chain.last().unwrap();

        let timestamp = chrono::Utc::now().timestamp();
        let new_block = Block {
            id: prev_block.id + 1,
            timestamp,
            data,
            previous_hash: prev_block.hash.clone(),
            hash: None,
        };
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
        Some(self.get_block_by_id(id)?.data.clone())
    }

    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }
}
