
# Rust Blockchain Project

Project rust_chain, is a simple, educational blockchain implementation in Rust. It demonstrates the fundamental concepts of blockchain technology, including creating blocks, managing a blockchain, and calculating Merkle proofs. This project is designed for educational purposes and serves as a foundation for understanding how blockchains work under the hood.

## Features

- Basic blockchain structure, block creation, chain verificaction
- Proof of Work (PoW) consensus algorithm [Maybe TBD]
- Transaction management and processing
- CLI for interacting with the blockchain [Or Web UI TBD]
- Simple tests of chain integrity and Merkle proofs

## Getting Started

### Prerequisites

- Rust programming environment ([Rustup](https://rustup.rs/))
- Cargo (comes with Rustup installation)
  
- I used Eclipse on Ubuntu. You'll have to use apt or your favorite package manager tool to install the Rust development packages. I used synaptic.

### Installation

1. Clone the repository:

git clone https://github.com/rickenator/rust_chain.git

2. Navigate to the project directory:

cd rust_chain

3. Build the project:

cargo build --release


In Eclipse, install the Rust package from the market. The "Run" button will save files, compile and run your project. 

### Running the Application

To start the blockchain node and interact with the CLI:

cargo run


## Usage

In main() the run_test() function will be called. You can modify the tests there as desired. Currently as configured, a file blockchain.json will be written when the blockchain goes out of scope at the end of the test. Subsequent runs of rust_chain will load this file and restore the previous state, so if you want to start from fresh, delete this file. 

There are a number of print_json() that have been commented out for brevity from the console, and at some point of course we can have a better debugging log, but this is fine for now. 

The mempool is hard coded so that after 4 transactions are added, a new block is created and the pool is drained by 4. In the current test, there are 8 additions and 1 extra "dangling" addition, that makes 2 blocks and an singleton transaction that gets properly persisted. Then if rust_chain is executed 4 times, there should be 8 more blocks and an additional block made of the previously dangling transactions with an empty pool.

## Architecture

This project is structured as follows:

- `src/main.rs`: Entry point for the CLI application.
- `src/blockchain.rs`: Contains the core blockchain logic, including block creation and the Merkle tree algorithm.
- `src/transaction.rs`: Defines the transaction structure and how transactions are processed.
- `src/block.rs`: Defines the block structure and how blocks are created and validated.
- `src/merkle_proof.rs`: Implements the Merkle verifier against the tree.

## Contributing

Contributions are welcome! Please feel free to submit pull requests, report bugs, and suggest features.

## License

This project is licensed under the GPLv2 License - see the LICENSE file for details.

## Acknowledgments

- Special thanks to ChatGPT and Copilot. I knew not a line of Rust nor anything of depth about Blockchains prior to taking on this project. This took about 48 hours of work. I didn't understand a lot going on, but that was the idea. I could piece things together chunks at a time and get it to verifyably work. Afterwards I did a deep dive into what the language was doing, how a blockchain works and what it could be used for. A good learning experience, and overall I'd say Rust is very cool and want to learn more.


