# Common Issues

## Server Issues

### Server Won't Start

**Problem**: The server executable doesn't start or exits immediately.

**Solutions**:
1. Check Rust installation: `rustc --version`
2. Verify executable permissions: `chmod +x target/release/robot-mcp`
3. Check for missing dependencies: `cargo build --release`
4. Look for error messages in stderr output

### Server Starts but MCP Client Can't Connect

**Problem**: Server runs but MCP client shows connection errors.

**Solutions**:
1. Verify the executable path in your MCP client configuration
2. Check that the path is absolute, not relative
3. Ensure the server executable exists and is accessible
4. Check MCP client logs for specific error messages

**Example Configuration (Claude Desktop)**:
```json
{
  "mcpServers": {
    "RobotCommandServer": {
      "command": "/Users/your-username/projects/robot-command-mcp/target/release/robot-mcp",
      "args": []
    }
  }
}
```

## Robot Communication Issues

### Robot Not Responding

**Problem**: All robot commands return "Robot not responding".

**Solutions**:
1. **Check Network Connection**:
   ```bash
   ping 192.168.4.1
   ```
2. **Verify Robot IP**: Ensure robot is actually at `192.168.4.1`
3. **Test Robot API Directly**:
   ```bash
   curl "http://192.168.4.1/js?json=%7B%22T%22%3A126%7D"
   ```
4. **Check Firewall**: Ensure no firewall is blocking HTTP requests
5. **Verify Robot Power**: Make sure robot is powered on and running

### Inconsistent Robot Responses

**Problem**: Robot sometimes responds, sometimes doesn't.

**Solutions**:
1. **Check WiFi Signal Strength**: Poor connection can cause timeouts
2. **Verify Robot Firmware**: Ensure robot API is stable
3. **Check Network Load**: Heavy network traffic can cause delays
4. **Monitor Robot Logs**: If available, check robot-side error logs

### Invalid IMU Data

**Problem**: IMU data looks incorrect or contains NaN values.

**Solutions**:
1. **Calibrate IMU**: Many IMUs need calibration after startup
2. **Check Mounting**: Ensure IMU is properly mounted and oriented
3. **Verify Data Format**: Ensure robot returns data in expected format:
   ```
   accel_x:0.12,accel_y:-0.05,accel_z:9.81,gyro_x:0.01,gyro_y:0.02,gyro_z:-0.001
   ```
4. **Temperature Issues**: IMUs can be sensitive to temperature changes

## MCP Client Issues

### Tools Not Appearing

**Problem**: MCP client doesn't show available robot tools.

**Solutions**:
1. **Restart MCP Client**: Sometimes clients need restart after configuration changes
2. **Check Server Registration**: Verify server is properly registered in client config
3. **Verify Tool Registration**: Ensure tools are properly defined with `#[tool]` macro
4. **Check Client Logs**: Look for tool discovery errors

### Tool Calls Failing

**Problem**: Tool calls return errors or don't execute.

**Solutions**:
1. **Parameter Validation**: Ensure parameters match expected schema
2. **Check Error Messages**: Look at specific error returned by tool
3. **Verify Speed Ranges**: Ensure speed parameters are between -1.0 and 1.0
4. **Network Issues**: Check if robot communication is working

## Configuration Issues

### Wrong Robot IP Address

**Problem**: Your robot uses a different IP address than `192.168.4.1`.

**Solutions**:
1. **Find Robot IP**: Use network scanning tools to find your robot
2. **Update Server Code**: Modify `ROVER_IP` constant in `src/main.rs`:
   ```rust
   const ROVER_IP: &str = "192.168.1.100"; // Your robot's IP
   ```
3. **Rebuild Server**: `cargo build --release`
4. **Update Client Config**: Point to new executable

### Path Issues

**Problem**: MCP client can't find the executable.

**Common Path Mistakes**:
- Using relative paths: `./target/release/robot-mcp` ❌
- Missing executable name: `/path/to/project/` ❌
- Wrong filename: `/path/to/project/target/release/robot-command` ❌

**Correct Path Format**:
```json
{
  "command": "/Users/username/projects/robot-command-mcp/target/release/robot-mcp"
}
```

## Performance Issues

### Slow Response Times

**Problem**: Robot commands take a long time to complete.

**Solutions**:
1. **Network Latency**: Check ping times to robot
2. **Robot Processing**: Some robots may be slow to process commands
3. **IMU Reading**: IMU data retrieval might be slow
4. **HTTP Timeouts**: Consider adjusting reqwest timeout settings

### Memory Usage

**Problem**: Server uses too much memory.

**Solutions**:
1. **Check for Memory Leaks**: Monitor memory usage over time
2. **Optimize Builds**: Use `cargo build --release` for production
3. **Profile Code**: Use tools like `valgrind` or `heaptrack`

## Debugging Tips

### Enable Detailed Logging

The server uses `tracing` for logging. To see more details:
```bash
RUST_LOG=debug ./target/release/robot-mcp
```

### Test Individual Components

1. **Test Robot API**:
   ```bash
   curl -v "http://192.168.4.1/js?json={\"T\":126}"
   ```

2. **Test Server Locally**:
   ```bash
   echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/release/robot-mcp
   ```

### Check Dependencies

Ensure all required dependencies are available:
```bash
cargo tree
cargo check
```

## Getting Help

If you're still experiencing issues:

1. **Check GitHub Issues**: Look for similar problems in the project repository
2. **Create Detailed Report**: Include error messages, configuration, and steps to reproduce
3. **Provide Environment Info**: OS, Rust version, robot model, etc.
4. **Test Isolation**: Try to isolate whether the issue is server, client, or robot related