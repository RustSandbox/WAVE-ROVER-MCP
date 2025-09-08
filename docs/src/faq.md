# FAQ

## General Questions

### Q: What is LLM Robot Commander?
**A:** LLM Robot Commander is a Rust-based server that implements the Model Context Protocol (MCP) to enable natural language control of robots. It acts as a bridge between LLM clients (like Claude) and robot hardware.

### Q: What robots are supported?
**A:** Currently, the system supports any robot that:
- Has a network connection (WiFi/Ethernet)
- Accepts HTTP requests with JSON payloads
- Implements the expected command protocol (movement and IMU data)

The default configuration expects robots at IP `192.168.4.1`, but this can be customized.

### Q: Do I need programming experience to use this?
**A:** For basic usage, no programming is required. You simply:
1. Build and configure the server
2. Set up your MCP client
3. Give natural language commands to your robot

For customization and adding new features, Rust programming knowledge is helpful.

## Setup and Installation

### Q: What are the system requirements?
**A:** You need:
- **Rust toolchain** (1.70+)
- **Network connection** to your robot
- **MCP-compatible client** (Claude Desktop, MLStudio, etc.)
- **Operating System**: Windows, macOS, or Linux

### Q: How do I install Rust?
**A:** Use rustup (recommended):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Q: Where do I put the MCP configuration?
**A:** For Claude Desktop:
- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **Linux**: `~/.config/claude/claude_desktop_config.json`

### Q: The executable path in my configuration doesn't work. What's wrong?
**A:** Ensure you use the absolute path:
```json
{
  "mcpServers": {
    "RobotCommandServer": {
      "command": "/full/path/to/your/project/target/release/robot-mcp",
      "args": []
    }
  }
}
```

Common mistakes:
- Using relative paths like `./target/release/robot-mcp`
- Missing the executable name
- Wrong file permissions

## Robot Communication

### Q: My robot is at a different IP address. How do I change it?
**A:** Edit the `ROVER_IP` constant in `src/main.rs`:
```rust
const ROVER_IP: &str = "192.168.1.100"; // Your robot's IP
```
Then rebuild: `cargo build --release`

### Q: What if my robot uses a different protocol than HTTP?
**A:** You'll need to modify the `command_to_robot` function in `src/main.rs` to support your robot's protocol (TCP, UDP, Serial, etc.).

### Q: How do I find my robot's IP address?
**A:** Try these methods:
1. Check robot's display/web interface
2. Use network scanning: `nmap -sn 192.168.1.0/24`
3. Check your router's connected devices list
4. Use the robot manufacturer's app

### Q: The robot commands work but IMU data looks wrong. What should I check?
**A:** Verify:
1. **Data format**: Should be `accel_x:0.12,accel_y:-0.05,accel_z:9.81,gyro_x:0.01,gyro_y:0.02,gyro_z:-0.001`
2. **IMU calibration**: Many IMUs need calibration after power-on
3. **Mounting orientation**: IMU should be properly aligned with robot chassis
4. **Units**: Accelerometer in m/s², gyroscope in rad/s

## MCP and Client Issues

### Q: How do I know if the MCP server is working?
**A:** Check these indicators:
1. Server starts without errors and shows "waiting for requests"
2. MCP client shows the server as connected
3. Robot tools appear in your client's available tools
4. You can successfully call tools

### Q: My MCP client shows connection errors. What should I check?
**A:** Verify:
1. **Executable path**: Must be absolute and correct
2. **File permissions**: Executable should have execute permissions
3. **Dependencies**: All Rust dependencies are installed
4. **Client logs**: Check your MCP client's error logs

### Q: Tools are not appearing in my MCP client. Why?
**A:** This could be due to:
1. **Server registration**: Server might not be properly registered
2. **Client restart**: Some clients need restart after configuration changes
3. **Tool compilation**: Tools might not be compiled correctly
4. **Protocol version**: Ensure client and server use compatible MCP versions

### Q: What LLM clients are supported?
**A:** Any client that supports the Model Context Protocol (MCP), including:
- Claude Desktop
- MLStudio
- Custom MCP implementations
- Any client following MCP 2024-11-05 specification

## Commands and Usage

### Q: What commands can I give to the robot?
**A:** Natural language examples:
- "Move forward slowly" → `move_forward` with low speed
- "Go backward at half speed" → `move_backward` with 0.5 speed
- "Stop the robot" → `stop` command
- "What's the robot's status?" → `stop` (returns IMU data)

### Q: How does the system interpret speed values?
**A:** The LLM interprets natural language and converts it to numeric speeds:
- "slowly" → ~0.2-0.3
- "half speed" → ~0.5  
- "quickly" or "fast" → ~0.8-1.0
- "full speed" → 1.0

### Q: Can I control multiple robots?
**A:** Currently, the system supports one robot per server instance. For multiple robots, you could:
1. Run multiple server instances with different IPs
2. Modify the code to support robot selection
3. Use a robot multiplexer

### Q: How do I add new robot capabilities?
**A:** You can extend the system by:
1. Defining new command structures
2. Adding new MCP tools
3. Updating the robot communication protocol
4. See the [Contributing Guide](contributing.md) for details

## Troubleshooting

### Q: The robot says "Robot not responding". What does this mean?
**A:** This indicates network communication issues:
1. **Check connection**: `ping 192.168.4.1`
2. **Verify robot power**: Ensure robot is on and ready
3. **Test directly**: `curl "http://192.168.4.1/js?json=%7B%22T%22%3A126%7D"`
4. **Check firewall**: Ensure no blocking of HTTP traffic

### Q: Commands work sometimes but fail other times. Why?
**A:** Intermittent failures usually indicate:
1. **Poor WiFi signal**: Move robot closer to router
2. **Network congestion**: Try during off-peak hours
3. **Robot processing issues**: Robot may be overloaded
4. **Power issues**: Low battery can cause instability

### Q: How do I enable debug logging?
**A:** Set the environment variable:
```bash
RUST_LOG=debug ./target/release/robot-mcp
```

### Q: The server builds but won't start. What should I check?
**A:** Common issues:
1. **Missing dependencies**: Run `cargo build --release` again
2. **File permissions**: `chmod +x target/release/robot-mcp`
3. **Port conflicts**: Check if another service is using the same resources
4. **Rust version**: Ensure you have Rust 1.70+

## Performance and Limitations

### Q: How fast can I send commands?
**A:** The system can handle multiple commands per second, but consider:
1. **Robot processing time**: Physical robots need time to execute commands
2. **Network latency**: WiFi adds delay
3. **Safety**: Rapid commands might be unsafe for the robot

### Q: Are there any security concerns?
**A:** Current considerations:
1. **Local network**: System assumes trusted local network
2. **No authentication**: Robot API has no built-in security
3. **HTTP**: Uses unencrypted HTTP (fine for local networks)
4. **Access control**: Any device on network can control robot

### Q: Can this work over the internet?
**A:** Not recommended without modifications:
1. **Security**: No authentication or encryption
2. **Latency**: Internet delays affect real-time control
3. **Firewall**: Most home networks block incoming connections
4. **Safety**: Remote robot control raises safety concerns

## Development and Customization

### Q: How do I modify the system for my specific robot?
**A:** Key areas to customize:
1. **IP address**: Change `ROVER_IP` constant
2. **Command format**: Modify command structures
3. **Protocol**: Update `command_to_robot` function
4. **New tools**: Add tools for robot-specific features

### Q: Can I use this with Arduino/Raspberry Pi robots?
**A:** Yes! Your robot just needs:
1. **HTTP server**: Simple web server on port 80
2. **JSON parsing**: Parse the command JSON
3. **Motor control**: Execute movement commands
4. **Sensor reading**: Return IMU data in expected format

### Q: How do I contribute to the project?
**A:** See the [Contributing Guide](contributing.md) for:
1. **Development setup**
2. **Code style guidelines** 
3. **Submission process**
4. **Testing requirements**

## Getting Help

### Q: Where can I get help if I'm stuck?
**A:** Resources:
1. **Documentation**: This mdbook covers most scenarios
2. **GitHub Issues**: Report bugs or ask questions
3. **Code Examples**: Check the source code for examples
4. **Community**: Join project discussions if available

### Q: How do I report a bug?
**A:** When reporting issues, include:
1. **Error messages**: Copy exact error text
2. **Configuration**: Your MCP client config
3. **Environment**: OS, Rust version, robot model
4. **Steps**: How to reproduce the issue
5. **Logs**: Debug output if available

### Q: Is commercial use allowed?
**A:** Check the project's LICENSE file for specific terms. Most open-source projects allow commercial use under certain conditions.