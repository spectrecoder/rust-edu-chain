// merkle_proof.rs
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MerkleProof {
    pub leaf: Vec<u8>,
    pub path: Vec<(Vec<u8>, bool)>, // (hash, is_right)
}

impl MerkleProof {
    pub fn new(leaf: Vec<u8>, path: Vec<(Vec<u8>, bool)>) -> Self {
        MerkleProof { leaf, path }
    }

    // Implement the method to verify the proof against a given Merkle root
    pub fn verify(&self, merkle_root: &Vec<u8>) -> bool {
        let mut current_hash = self.leaf.clone();
        for (hash, is_right) in &self.path {
            // Combine the current hash with the next hash in the path
            let combined = if *is_right {
                // If the current node is supposed to be on the right, append it to the hash from the path
                [hash.as_slice(), current_hash.as_slice()].concat()
            } else {
                // If the current node is on the left, append the path hash to it
                [current_hash.as_slice(), hash.as_slice()].concat()
            };

            // Hash the combined pair to get the new current hash
            current_hash = MerkleProof::hash_function(&combined);
        }

        // Check if the final hash matches the provided Merkle root
        &current_hash == merkle_root
    }

    pub fn hash_function(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
