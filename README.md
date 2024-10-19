## chat

### Installation

#### Install Script (Coming Soon)

#### Building from Source
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update

# Clone the repository
git clone https://github.com/7791372/chat.git
cd chat

# Build the project
cargo build --release --bin client
cargo build --release --bin server (only build server if you are going to be hosting)

# Run the binary
./target/release/client
./target/release/server (this will open a server others can connect to)
```

---

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.