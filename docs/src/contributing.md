# Contributing

We welcome contributions to the LLM Robot Commander project! This guide will help you get started.

## Ways to Contribute

- **Bug Reports**: Report issues you encounter
- **Feature Requests**: Suggest new capabilities
- **Code Contributions**: Fix bugs or add features
- **Documentation**: Improve or expand documentation
- **Testing**: Help test on different platforms and robots

## Development Setup

### 1. Fork and Clone
```bash
# Fork the repository on GitHub
git clone https://github.com/your-username/robot-command-mcp.git
cd robot-command-mcp
```

### 2. Set Up Development Environment
```bash
# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development tools
cargo install cargo-fmt
cargo install cargo-clippy
cargo install cargo-audit
```

### 3. Build and Test
```bash
# Build the project
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy
```

## Making Changes

### 1. Create a Feature Branch
```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes
- Write clean, well-documented code
- Follow existing code style and patterns
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes
```bash
# Run all tests
cargo test

# Test with a real robot if possible
cargo build --release
./target/release/robot-mcp
```

### 4. Commit Changes
```bash
git add .
git commit -m "Add descriptive commit message"
```

## Code Style

### Rust Formatting
Use `rustfmt` for consistent formatting:
```bash
cargo fmt
```

### Naming Conventions
- **Functions**: `snake_case`
- **Variables**: `snake_case` 
- **Types**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`

### Documentation
- Document all public functions and types
- Use clear, descriptive names
- Include examples where helpful

```rust
/// Sends a movement command to the robot and retrieves IMU data
/// 
/// # Arguments
/// * `command` - A serializable command structure
/// 
/// # Returns
/// * `Ok(String)` - Robot response message
/// * `Err(String)` - Error description
/// 
/// # Examples
/// ```
/// let forward_cmd = GoForward::new(0.5);
/// let result = command_to_robot(forward_cmd)?;
/// ```
fn command_to_robot<T: Serialize>(command: T) -> Result<String, String> {
    // Implementation
}
```

## Adding New Features

### New Robot Commands

1. Define the command structure:
```rust
#[derive(Serialize)]
pub struct TurnRight {
    #[serde(rename = "T")]
    command: u8,
    #[serde(rename = "angle")]
    angle: f32,
}

impl TurnRight {
    pub fn new(angle: f32) -> Self {
        Self {
            command: 2, // New command type
            angle,
        }
    }
}
```

2. Add parameter type:
```rust
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct Angle {
    #[schemars(description = "Angle in degrees")]
    angle: f32,
}
```

3. Implement the tool:
```rust
#[tool(description = "Turn robot right by specified angle")]
async fn turn_right(&self, Parameters(Angle{angle}): Parameters<Angle>) 
    -> Result<CallToolResult, ErrorData> {
    let command = TurnRight::new(angle);
    let result = command_to_robot(command);
    let imu_data = retrieve_imu_data().unwrap();
    let return_message = format!(
        "Robot turned right {}°. IMU data: {:?}", 
        angle, imu_data
    );
    Ok(CallToolResult::success(vec![Content::text(return_message)]))
}
```

### New Sensor Types

1. Define sensor command:
```rust
#[derive(Serialize)]
pub struct CameraData {
    #[serde(rename = "T")]
    command: u8, // e.g., 127
}
```

2. Add retrieval function:
```rust
pub fn retrieve_camera_data() -> Result<String, String> {
    let camera_command = CameraData::new();
    command_to_robot(camera_command)
}
```

3. Update existing tools to include new sensor data

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_forward_creation() {
        let command = GoForward::new(0.5);
        assert_eq!(command.command, 1);
        assert_eq!(command.left_speed, 0.5);
        assert_eq!(command.right_speed, 0.5);
    }

    #[test]
    fn test_go_backward_negative_speed() {
        let command = GoBackward::new(0.3);
        assert_eq!(command.left_speed, -0.3);
        assert_eq!(command.right_speed, -0.3);
    }
}
```

### Integration Tests
Create tests in `tests/` directory:
```rust
// tests/integration_test.rs
use robot_command_mcp::*;

#[tokio::test]
async fn test_server_initialization() {
    let server = RobotControlsServer::new();
    let info = server.get_info();
    assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
}
```

### Manual Testing Checklist
- [ ] Server starts without errors
- [ ] MCP client can connect
- [ ] All tools are discovered
- [ ] Robot commands work correctly
- [ ] IMU data is retrieved properly
- [ ] Error handling works as expected

## Documentation

### Code Documentation
- Use `///` for documentation comments
- Document all public APIs
- Include examples where helpful
- Explain complex algorithms

### mdBook Documentation
Update relevant sections in `docs/src/`:
- Add new tools to `tools.md`
- Update architecture diagrams if needed
- Add troubleshooting entries for new features

### README Updates
Keep the main README current with:
- New features and capabilities
- Updated installation instructions
- Changed requirements

## Submission Guidelines

### Pull Request Process

1. **Update Documentation**: Ensure all documentation reflects your changes
2. **Add Tests**: Include tests for new functionality
3. **Check CI**: Ensure all tests pass
4. **Update CHANGELOG**: Document your changes
5. **Submit PR**: Create a pull request with a clear description

### PR Description Template
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings introduced
```

## Code Review Process

1. **Automated Checks**: CI runs tests, formatting, and linting
2. **Maintainer Review**: Project maintainers review code and design
3. **Feedback Integration**: Address review comments
4. **Final Approval**: Maintainer approves and merges

## Release Process

### Versioning
We use [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### Release Checklist
- [ ] Update version in `Cargo.toml`
- [ ] Update CHANGELOG.md
- [ ] Create release tag
- [ ] Build and test release binaries
- [ ] Update documentation

## Getting Help

- **Discussion**: Use GitHub Discussions for questions
- **Issues**: Report bugs via GitHub Issues
- **Chat**: Join project chat/Discord if available
- **Email**: Contact maintainers for sensitive topics

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation

Thank you for contributing to LLM Robot Commander!