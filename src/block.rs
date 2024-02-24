use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use crate::utils::to_hex_string;
use sha2::{Digest, Sha256};


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
    pub fn calculate_hash(&self) -> Option<Vec<u8>> {
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

    pub fn calculate_merkle_root(&self) -> Option<Vec<u8>> {
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
