# Installation

## Prerequisites

Before you begin, ensure you have the following installed:

### Rust Toolchain

Install Rust using rustup (the recommended way):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Verify your installation:
```bash
rustc --version
cargo --version
```

### Required Dependencies

The project uses the following key dependencies (automatically handled by Cargo):

- `rmcp`: Model Context Protocol implementation
- `reqwest`: HTTP client for robot communication
- `serde`: JSON serialization/deserialization
- `tokio`: Async runtime
- `tracing`: Logging

## Building the Project

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/robot-command-mcp.git
cd robot-command-mcp
```

### 2. Build the Executable

For development builds:
```bash
cargo build
```

For optimized release builds:
```bash
cargo build --release
```

This will create the executable at:
- Development: `target/debug/robot-mcp`
- Release: `target/release/robot-mcp`

### 3. Verify the Build

Test that the server starts correctly:
```bash
./target/release/robot-mcp
```

The server should start and wait for MCP protocol messages.

## Configuration

### Robot IP Address

By default, the server expects your robot to be available at `192.168.4.1`. If your robot uses a different IP address, you'll need to modify the `ROVER_IP` constant in `src/main.rs`:

```rust
const ROVER_IP: &str = "192.168.4.1"; // Change this to your robot's IP
```

Then rebuild the project:
```bash
cargo build --release
```

### Environment Variables

Currently, the server uses hardcoded configuration. Future versions may support environment variable configuration.

## Troubleshooting Build Issues

### Common Problems

**Rust not found**: Ensure Rust is properly installed and `~/.cargo/bin` is in your PATH.

**Compilation errors**: Make sure you're using a recent version of Rust:
```bash
rustup update
```

**Network dependencies**: If you're behind a corporate firewall, you may need to configure Cargo to use a proxy.

### Getting Help

If you encounter build issues:

1. Check that you have the latest version of Rust
2. Try cleaning and rebuilding: `cargo clean && cargo build --release`
3. Check the [troubleshooting section](troubleshooting.md) for common issues
4. Open an issue on the project's GitHub repository