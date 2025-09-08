# Testing

This guide covers testing strategies and procedures for the LLM Robot Commander.

## Testing Overview

The project includes multiple testing levels:

1. **Unit Tests**: Test individual functions and components
2. **Integration Tests**: Test component interactions
3. **MCP Protocol Tests**: Test MCP compliance
4. **Hardware Tests**: Test with real robot hardware
5. **End-to-End Tests**: Test complete user workflows

## Running Tests

### Basic Test Commands

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in release mode
cargo test --release
```

### Test Categories

```bash
# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration

# Run only doc tests
cargo test --doc
```

## Unit Tests

### Command Structure Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_forward_serialization() {
        let command = GoForward::new(0.7);
        let json = serde_json::to_string(&command).unwrap();
        assert_eq!(json, r#"{"T":1,"L":0.7,"R":0.7}"#);
    }

    #[test]
    fn test_go_backward_negative_speed() {
        let command = GoBackward::new(0.5);
        assert_eq!(command.left_speed, -0.5);
        assert_eq!(command.right_speed, -0.5);
    }

    #[test]
    fn test_imu_command_serialization() {
        let command = IMUData::new();
        let json = serde_json::to_string(&command).unwrap();
        assert_eq!(json, r#"{"T":126}"#);
    }

    #[test]
    fn test_speed_parameter_validation() {
        // Test valid speeds
        assert!(validate_speed(0.0).is_ok());
        assert!(validate_speed(0.5).is_ok());
        assert!(validate_speed(1.0).is_ok());
        
        // Test invalid speeds
        assert!(validate_speed(-0.1).is_err());
        assert!(validate_speed(1.1).is_err());
    }
}
```

### HTTP Communication Tests

```rust
#[cfg(test)]
mod http_tests {
    use super::*;
    use mockito;

    #[test]
    fn test_successful_robot_command() {
        let mut server = mockito::Server::new();
        let mock = server.mock("GET", "/js")
            .match_query(mockito::Matcher::UrlEncoded("json".into(), r#"{"T":1,"L":0.5,"R":0.5}"#.into()))
            .with_status(200)
            .with_body("OK")
            .create();

        let command = GoForward::new(0.5);
        let result = command_to_robot_with_url(command, &server.url());
        
        mock.assert();
        assert_eq!(result.unwrap(), "OK");
    }

    #[test]
    fn test_robot_not_responding() {
        // Test with invalid URL to simulate network failure
        let command = GoForward::new(0.5);
        let result = command_to_robot_with_url(command, "http://invalid.url");
        
        assert_eq!(result.unwrap(), "Robot not responding");
    }
}
```

## Integration Tests

### MCP Server Tests

Create `tests/mcp_integration.rs`:

```rust
use robot_command_mcp::*;
use rmcp::model::*;
use tokio_test;

#[tokio::test]
async fn test_server_initialization() {
    let server = RobotControlsServer::new();
    let info = server.get_info();
    
    assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
    assert!(info.capabilities.tools.is_some());
}

#[tokio::test]
async fn test_tool_discovery() {
    let server = RobotControlsServer::new();
    let tools = server.list_tools().await;
    
    assert!(tools.len() >= 3);
    
    let tool_names: Vec<&str> = tools.iter()
        .map(|t| t.name.as_str())
        .collect();
    
    assert!(tool_names.contains(&"move_forward"));
    assert!(tool_names.contains(&"move_backward"));
    assert!(tool_names.contains(&"stop"));
}

#[tokio::test]
async fn test_move_forward_tool() {
    let server = RobotControlsServer::new();
    let params = serde_json::json!({
        "speed": 0.5
    });
    
    // Note: This will fail without a real robot
    // Consider mocking HTTP requests for CI
    let result = server.call_tool("move_forward", params).await;
    
    // In a real test environment with mock robot:
    // assert!(result.is_ok());
}
```

### Mock Robot Server

For CI/CD, create a mock robot server:

```rust
// tests/mock_robot.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

pub struct MockRobot {
    state: Arc<RwLock<RobotState>>,
}

#[derive(Clone)]
struct RobotState {
    last_command: Option<String>,
    imu_data: String,
}

impl MockRobot {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(RobotState {
                last_command: None,
                imu_data: "accel_x:0.0,accel_y:0.0,accel_z:9.81,gyro_x:0.0,gyro_y:0.0,gyro_z:0.0".to_string(),
            })),
        }
    }

    pub async fn start_server(&self, port: u16) {
        let state = self.state.clone();
        
        let route = warp::path("js")
            .and(warp::query::<HashMap<String, String>>())
            .and(warp::any().map(move || state.clone()))
            .and_then(handle_command);

        warp::serve(route)
            .run(([127, 0, 0, 1], port))
            .await;
    }
}

async fn handle_command(
    query: HashMap<String, String>,
    state: Arc<RwLock<RobotState>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(json) = query.get("json") {
        let mut robot_state = state.write().await;
        robot_state.last_command = Some(json.clone());
        
        // Parse command and return appropriate response
        if json.contains(r#""T":126"#) {
            // IMU request
            Ok(robot_state.imu_data.clone())
        } else {
            // Movement command
            Ok("OK".to_string())
        }
    } else {
        Ok("Error: No JSON parameter".to_string())
    }
}
```

## Hardware Testing

### Test with Real Robot

```bash
# Ensure robot is connected and at correct IP
ping 192.168.4.1

# Build and run server
cargo build --release
./target/release/robot-mcp
```

### Manual Test Sequence

1. **Basic Connection**:
   ```bash
   curl "http://192.168.4.1/js?json=%7B%22T%22%3A126%7D"
   ```

2. **Movement Commands**:
   ```bash
   # Forward
   curl "http://192.168.4.1/js?json=%7B%22T%22%3A1%2C%22L%22%3A0.3%2C%22R%22%3A0.3%7D"
   
   # Backward
   curl "http://192.168.4.1/js?json=%7B%22T%22%3A1%2C%22L%22%3A-0.3%2C%22R%22%3A-0.3%7D"
   
   # Stop
   curl "http://192.168.4.1/js?json=%7B%22T%22%3A1%2C%22L%22%3A0.0%2C%22R%22%3A0.0%7D"
   ```

3. **MCP Protocol Test**:
   ```bash
   echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/release/robot-mcp
   ```

### Hardware Test Checklist

- [ ] Robot responds to ping
- [ ] HTTP endpoint accepts requests
- [ ] Movement commands work correctly
- [ ] IMU data is returned in correct format
- [ ] Robot stops when commanded
- [ ] No network timeouts or errors
- [ ] Sensor data looks reasonable

## Performance Testing

### Load Testing

```rust
#[tokio::test]
async fn test_concurrent_commands() {
    let server = RobotControlsServer::new();
    let mut handles = vec![];
    
    // Send multiple commands concurrently
    for i in 0..10 {
        let server_clone = server.clone();
        let handle = tokio::spawn(async move {
            let params = serde_json::json!({
                "speed": 0.1 * i as f32
            });
            server_clone.call_tool("move_forward", params).await
        });
        handles.push(handle);
    }
    
    // Wait for all commands to complete
    for handle in handles {
        let result = handle.await.unwrap();
        // Verify results...
    }
}
```

### Memory Testing

```bash
# Monitor memory usage during testing
valgrind --tool=memcheck --leak-check=full ./target/debug/robot-mcp

# Or use cargo-profdata
cargo install cargo-profdata
cargo profdata run
```

## Continuous Integration

### GitHub Actions Configuration

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    
    - name: Check formatting
      run: cargo fmt --check
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Run tests
      run: cargo test --all-features
    
    - name: Run integration tests
      run: cargo test --test '*' -- --test-threads=1
      
    - name: Check docs
      run: cargo doc --no-deps
```

### Test Coverage

```bash
# Install cargo-tarpaulin for coverage
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out Html

# View coverage report
open tarpaulin-report.html
```

## Test Data Management

### Test Fixtures

Create test data files in `tests/fixtures/`:

```
tests/
├── fixtures/
│   ├── valid_commands.json
│   ├── invalid_commands.json
│   └── sample_imu_data.txt
└── integration/
    ├── mcp_tests.rs
    └── robot_tests.rs
```

### Environment Setup

```bash
# Set test environment variables
export RUST_LOG=debug
export TEST_ROBOT_IP=127.0.0.1:3000

# Run tests with environment
cargo test
```

## Debugging Test Failures

### Common Issues

1. **Robot Not Available**: Use mock robot for CI
2. **Network Timeouts**: Increase timeout values for slow networks
3. **Flaky Tests**: Add retry logic for network operations
4. **Race Conditions**: Use proper synchronization in async tests

### Debug Tools

```bash
# Run tests with debug output
RUST_LOG=debug cargo test -- --nocapture

# Run single test with backtrace
RUST_BACKTRACE=1 cargo test test_name

# Use debugger
rust-gdb target/debug/deps/robot_command_mcp-<hash>
```