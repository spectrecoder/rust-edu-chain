pub mod blockchain;
pub mod transaction;
pub mod block;
pub mod merkle_proof;
pub mod utils;
use std::fs::File;
use std::path::Path;
use std::io::Read;

pub const MAX_TRANSACTIONS_PER_BLOCK: usize = 4;





// Run test
fn run_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running test");
    let mut blockchain = blockchain::Blockchain::new();

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
