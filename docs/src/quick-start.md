# Quick Start

Get up and running with LLM Robot Commander in just a few steps!

## Step 1: Build the Server

```bash
# Clone and build
git clone https://github.com/your-username/robot-command-mcp.git
cd robot-command-mcp
cargo build --release
```

## Step 2: Prepare Your Robot

Ensure your robot:
- Is connected to the same network
- Has IP address `192.168.4.1` (or modify the server configuration)
- Accepts HTTP requests with JSON payloads
- Implements the expected [Robot API](robot-commands.md)

## Step 3: Configure Your MCP Client

Add the server to your MCP client configuration. For Claude Desktop, add this to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "RobotCommandServer": {
      "command": "/path/to/your/project/target/release/robot-mcp",
      "args": []
    }
  }
}
```

**Important**: Replace `/path/to/your/project/` with the absolute path to your project directory.

### Common MCP Client Locations

**Claude Desktop (macOS)**:
```
~/Library/Application Support/Claude/claude_desktop_config.json
```

**Claude Desktop (Windows)**:
```
%APPDATA%\Claude\claude_desktop_config.json
```

**Claude Desktop (Linux)**:
```
~/.config/claude/claude_desktop_config.json
```

## Step 4: Test the Connection

1. Start your MCP client (e.g., Claude Desktop)
2. The server should automatically connect
3. Try a simple command like: "Move the robot forward slowly"

## Example Commands

Once everything is connected, you can give natural language commands:

- **"Move forward at half speed"** → Robot moves forward with speed 0.5
- **"Go backward slowly"** → Robot moves backward at low speed
- **"Stop the robot"** → Robot stops and returns current sensor data
- **"What are the current IMU readings?"** → Returns sensor data

## Verification

### Check Server Status

The server logs to stderr, so you should see messages like:
```
Starting Robot commander MCP server
Service initialized, waiting for requests...
```

### Test Robot Communication

You can manually test your robot's API:
```bash
# Test forward movement
curl "http://192.168.4.1/js?json={\"T\":1,\"L\":0.5,\"R\":0.5}"

# Test IMU data retrieval
curl "http://192.168.4.1/js?json={\"T\":126}"
```

## Next Steps

- Learn more about the [available tools](tools.md)
- Understand the [robot communication protocol](robot-communication.md)
- Check out the [troubleshooting guide](troubleshooting.md) if you run into issues

## Quick Troubleshooting

**Server won't start**: Check that the executable has the right permissions and Rust dependencies are installed.

**Robot not responding**: Verify the robot's IP address and that it's on the same network.

**MCP client can't connect**: Ensure the path in your client configuration points to the correct executable location.