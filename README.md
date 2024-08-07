# Cart

# Description
Cart is an in-memory database implemented in Rust that runs in the terminal. It utilizes a special container to store elements, providing very high read and write speeds. The database uses the extremely fast and safe hashing function known as MurmurHash One-At-A-Time.

# Requirements
Rust programming language installed on your system

# Usage
- Clone the repository: git clone <repository-url>
- Navigate to the project directory: cd Cart
- Compile the Rust program: cargo build --release
- Run the database in the terminal: cargo run --release

# Features
- In-memory database running in the terminal environment.
- Special container structure optimized for high read and write speeds.
- Utilizes MurmurHash One-At-A-Time hashing function for efficient and secure data storage.

# Performance
- Cart is designed to offer blazing fast read and write speeds for in-memory data operations.
- The use of MurmurHash One-At-A-Time ensures data integrity and security while maintaining exceptional performance.

# Implementation
- The database is implemented in Rust, leveraging its performance and safety features.
- The special container structure and hashing function are carefully designed to optimize speed and reliability.

# Future Development
- Implement additional features such as indexing and querying capabilities.
- Enhance error handling and data validation mechanisms.

# License
This project is licensed under the MIT License. See the LICENSE file for details.
