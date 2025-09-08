# Building from Source

## Development Setup

### Prerequisites
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git for version control
- A robot compatible with the command protocol

### Clone and Build
```bash
git clone https://github.com/your-username/robot-command-mcp.git
cd robot-command-mcp
cargo build
```

### Development Build
```bash
cargo build
./target/debug/robot-mcp
```

### Release Build
```bash
cargo build --release
./target/release/robot-mcp
```

## Project Structure

```
robot-command-mcp/
├── Cargo.toml          # Rust project configuration
├── Cargo.lock          # Dependency lock file
├── src/
│   └── main.rs         # Main application code
├── docs/               # mdBook documentation
└── target/             # Build artifacts
    ├── debug/          # Development builds
    └── release/        # Optimized builds
```

## Dependencies

Key dependencies defined in `Cargo.toml`:

```toml
[dependencies]
rmcp = "0.1.0"          # Model Context Protocol implementation
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Customization

### Changing Robot IP
Edit the `ROVER_IP` constant in `src/main.rs`:
```rust
const ROVER_IP: &str = "192.168.1.100"; // Your robot's IP
```

### Adding New Tools
1. Define the command structure:
```rust
#[derive(Serialize)]
pub struct TurnLeft {
    #[serde(rename = "T")]
    command: u8,
    #[serde(rename = "angle")]
    angle: f32,
}
```

2. Add the tool to the server:
```rust
#[tool(description = "Turn robot left by specified angle")]
async fn turn_left(&self, Parameters(angle): Parameters<Angle>) 
    -> Result<CallToolResult, ErrorData> {
    // Implementation
}
```

### Modifying Command Protocol
To support different robot APIs, modify the `command_to_robot` function:
```rust
fn command_to_robot<T: Serialize>(command: T) -> Result<String, String> {
    let json = serde_json::to_string(&command).unwrap();
    // Modify URL format as needed
    let url = format!("http://{}/api/v2/command", ROVER_IP);
    // Change to POST request if needed
    // ...
}
```

## Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
# Test with a mock robot
cargo test -- --test-threads 1
```

### Manual Testing
```bash
# Start the server
cargo run

# In another terminal, test MCP protocol
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/debug/robot-mcp
```

## Cross-Compilation

### For Raspberry Pi (ARM64)
```bash
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu --release
```

### For Windows
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu --release
```

## Optimization

### Release Optimizations
The release build includes optimizations defined in `Cargo.toml`:
```toml
[profile.release]
opt-level = 3           # Maximum optimization
lto = true             # Link-time optimization
codegen-units = 1      # Better optimization
panic = 'abort'        # Smaller binary size
```

### Binary Size Reduction
```bash
# Strip debug symbols
strip target/release/robot-mcp

# Use cargo-strip
cargo install cargo-strip
cargo strip --release
```

### Performance Profiling
```bash
# Install profiling tools
cargo install cargo-profdata
cargo install cargo-show-asm

# Profile the application
cargo profdata run --release
```

## Debugging

### Enable Debug Logging
```bash
RUST_LOG=debug cargo run
```

### Use Debugger
```bash
# With GDB
rust-gdb target/debug/robot-mcp

# With LLDB
rust-lldb target/debug/robot-mcp
```

### Analyze Dependencies
```bash
# Show dependency tree
cargo tree

# Check for outdated dependencies
cargo outdated

# Audit for security issues
cargo audit
```

## Continuous Integration

### GitHub Actions Example
```yaml
name: Build and Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - run: cargo test --all-features
    - run: cargo build --release
```

## Documentation

### Generate Rust Docs
```bash
cargo doc --open
```

### Build This Documentation
```bash
cd docs
mdbook build
mdbook serve
```