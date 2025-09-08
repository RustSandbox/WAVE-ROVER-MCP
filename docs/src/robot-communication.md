# Robot Communication

This document explains the low-level communication between the MCP server and robot hardware.

## Communication Stack

```
MCP Server (Rust)
       ↓
HTTP Client (reqwest)
       ↓
Network (WiFi/Ethernet)
       ↓
Robot HTTP Server
       ↓
Robot Firmware
       ↓
Hardware (Motors/Sensors)
```

## Network Configuration

### Default Setup
- **Robot IP**: `192.168.4.1`
- **Protocol**: HTTP (port 80)
- **Method**: GET requests
- **Format**: JSON in query parameter

### Alternative Configurations

If your robot uses a different setup, you'll need to modify the server code:

```rust
// Change IP address
const ROVER_IP: &str = "192.168.1.100";

// For different ports
let url = format!("http://{}:8080/api?json={}", ROVER_IP, json);

// For POST requests (requires code changes)
let client = reqwest::Client::new();
let response = client.post(&url).json(&command).send().await?;
```

## HTTP Request Format

### URL Structure
```
http://192.168.4.1/js?json={ENCODED_JSON}
```

### Example Request
```http
GET /js?json=%7B%22T%22%3A1%2C%22L%22%3A0.5%2C%22R%22%3A0.5%7D HTTP/1.1
Host: 192.168.4.1
User-Agent: reqwest/0.11.22
```

The JSON `{"T":1,"L":0.5,"R":0.5}` is URL-encoded as `%7B%22T%22%3A1%2C%22L%22%3A0.5%2C%22R%22%3A0.5%7D`.

## Error Handling

### Network-Level Errors

```rust
match reqwest::blocking::get(&url) {
    Ok(response) => {
        match response.text() {
            Ok(text) => text,
            Err(_) => "Invalid response format".to_string()
        }
    },
    Err(e) => {
        // Connection timeout, DNS resolution failure, etc.
        format!("Network error: {}", e)
    }
}
```

### Common Error Conditions

1. **Connection Refused**: Robot is off or not listening on port 80
2. **Timeout**: Robot is slow to respond or network issues
3. **DNS Resolution**: IP address is incorrect
4. **Invalid Response**: Robot returns non-text data

## Response Handling

### Successful Movement Command
```
Request:  {"T":1,"L":0.8,"R":0.8}
Response: "OK"
```

### Successful IMU Request
```
Request:  {"T":126}
Response: "accel_x:0.12,accel_y:-0.05,accel_z:9.81,gyro_x:0.01,gyro_y:0.02,gyro_z:-0.001"
```

### Error Responses
```
Request:  {"T":1,"L":2.0,"R":2.0}  // Invalid speed
Response: "Error: Speed out of range"
```

## Performance Considerations

### Request Timing

The current implementation uses blocking HTTP requests:
```rust
// Synchronous - blocks until response
let response = reqwest::blocking::get(&url)?;
```

For better performance, consider async requests:
```rust
// Asynchronous - non-blocking
let response = reqwest::get(&url).await?;
```

### Timeout Configuration

Default timeouts can be configured:
```rust
let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(5))
    .build()?;
```

### Connection Pooling

For frequent requests, connection reuse helps performance:
```rust
static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .pool_max_idle_per_host(10)
        .build()
        .expect("Failed to create HTTP client")
});
```

## Security Considerations

### Network Security
- Robot operates on local network (192.168.x.x)
- No authentication currently implemented
- HTTP (not HTTPS) - suitable for local networks only

### Access Control
- Any device on the network can control the robot
- Consider implementing authentication for production use
- Rate limiting may be needed for busy networks

## Robot Firmware Requirements

Your robot must implement:

### HTTP Server
```c
// Example server setup (pseudo-code)
void setup() {
    WiFi.begin("network", "password");
    server.on("/js", handleCommand);
    server.begin();
}

void handleCommand() {
    String json = server.arg("json");
    // Parse and execute command
    server.send(200, "text/plain", "OK");
}
```

### JSON Parsing
```c
void parseCommand(String json) {
    JSONVar command = JSON.parse(json);
    
    int type = command["T"];
    if (type == 1) {
        float left = command["L"];
        float right = command["R"];
        setMotorSpeeds(left, right);
    } else if (type == 126) {
        sendIMUData();
    }
}
```

### IMU Data Format
```c
void sendIMUData() {
    float ax, ay, az, gx, gy, gz;
    readIMU(&ax, &ay, &az, &gx, &gy, &gz);
    
    String response = "accel_x:" + String(ax, 2) +
                     ",accel_y:" + String(ay, 2) +
                     ",accel_z:" + String(az, 2) +
                     ",gyro_x:" + String(gx, 3) +
                     ",gyro_y:" + String(gy, 3) +
                     ",gyro_z:" + String(gz, 3);
    
    server.send(200, "text/plain", response);
}
```

## Testing Communication

### Manual Testing
```bash
# Test robot connection
ping 192.168.4.1

# Test API endpoint
curl "http://192.168.4.1/js?json=%7B%22T%22%3A126%7D"

# Test with verbose output
curl -v "http://192.168.4.1/js?json=%7B%22T%22%3A1%2C%22L%22%3A0.5%2C%22R%22%3A0.5%7D"
```

### Network Debugging
```bash
# Check if port 80 is open
nmap -p 80 192.168.4.1

# Monitor network traffic
tcpdump -i en0 host 192.168.4.1

# Check routing
traceroute 192.168.4.1
```

### Robot-Side Debugging
- Enable debug logging in robot firmware
- Monitor serial output during commands
- Check WiFi signal strength
- Verify power supply stability during movement