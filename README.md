
# Rust Blockchain Project

This project rust_chain, is a simple, educational blockchain implementation in Rust. It demonstrates the fundamental concepts of blockchain technology, including creating blocks, managing a blockchain, and calculating Merkle proofs. This project is designed for educational purposes and serves as a foundation for understanding how blockchains work under the hood.

## Features

- Basic blockchain structure and block creation
- Proof of Work (PoW) consensus algorithm [Maybe TBD]
- Transaction management and processing
- CLI for interacting with the blockchain [Or Web UI TBD]
- Simple tests of chain integrity and Merkle proofs

## Getting Started

### Prerequisites

- Rust programming environment ([Rustup](https://rustup.rs/))
- Cargo (comes with Rustup installation)
  
- I used Eclipse on Ubuntu.

### Installation

1. Clone the repository:

git clone https://github.com/rickenator/rust_chain.git

2. Navigate to the project directory:

cd rust_chain

3. Build the project:

cargo build --release


### Running the Application

To start the blockchain node and interact with the CLI:

cargo run


## Usage

[TBD]

## Architecture

This project is structured as follows:

- `src/main.rs`: Entry point for the CLI application.
- `src/blockchain.rs`: Contains the core blockchain logic, including block creation and the Merkle tree algorithm.
- `src/transaction.rs`: Defines the transaction structure and how transactions are processed.
- `src/block.rs`: Defines the block structure and how blocks are created and validated.
- `src/merkle_proof.rs`: Implements the Merlke verifier against the tree.

## Contributing

Contributions are welcome! Please feel free to submit pull requests, report bugs, and suggest features.

## License

This project is licensed under the GPLv2 License - see the LICENSE file for details.

## Acknowledgments

- Special thanks to ChatGPT and Copilot. I knew not a line of Rust nor anything of depth about Blockchains prior to taking on this project. This took about 48 hours of work. I didn't understand a lot going on, but that was the idea. I could piece things together chunks at a time and get it to verifyably work. Afterwards I did a deep dive into what the language was doing, how a blockchain works and what it could be used for. A good learning experience, and overall I'd say Rust is very cool and want to learn more.


