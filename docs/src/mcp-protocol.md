# MCP Protocol

The Model Context Protocol (MCP) is the communication standard used between the LLM client and our robot command server.

## What is MCP?

MCP enables LLM applications to securely access external tools and data sources. It provides:

- **Standardized Communication**: Consistent message format across all tools
- **Security**: Controlled access to external resources
- **Extensibility**: Easy addition of new capabilities
- **Type Safety**: Well-defined schemas for all interactions

## Server Implementation

Our robot server implements the MCP server specification:

```rust
#[tool_handler]
impl ServerHandler for RobotControlsServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("this server allow command robot to move forward or stop".to_string()),
        }
    }
}
```

## Protocol Messages

### Server Info Request

When a client connects, it requests server information:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "Claude",
      "version": "1.0"
    }
  }
}
```

### Server Info Response

The server responds with its capabilities:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": {}
    },
    "serverInfo": {
      "name": "robot-mcp",
      "version": "0.1.0"
    },
    "instructions": "this server allow command robot to move forward or stop"
  }
}
```

### Tools List Request

Client requests available tools:

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list"
}
```

### Tools List Response

Server provides tool definitions:

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name": "move_forward",
        "description": "command robot to move forward with a given speed and return IMU data",
        "inputSchema": {
          "type": "object",
          "properties": {
            "speed": {
              "type": "number",
              "description": "Speed of movement of Robot"
            }
          },
          "required": ["speed"]
        }
      },
      {
        "name": "move_backward",
        "description": "command robot to move backward with a given speed and return IMU data",
        "inputSchema": {
          "type": "object",
          "properties": {
            "speed": {
              "type": "number",
              "description": "Speed of movement of Robot"
            }
          },
          "required": ["speed"]
        }
      },
      {
        "name": "stop",
        "description": "Command Robot to stop",
        "inputSchema": {
          "type": "object",
          "properties": {}
        }
      }
    ]
  }
}
```

### Tool Call Request

Client calls a specific tool:

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "move_forward",
    "arguments": {
      "speed": 0.5
    }
  }
}
```

### Tool Call Response

Server returns execution results:

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "reaction of Robot is \"OK\"\\n. IMU data of robot after this move forward is \"accel_x:0.12,accel_y:-0.05,accel_z:9.81,gyro_x:0.01,gyro_y:0.02,gyro_z:-0.001\""
      }
    ],
    "isError": false
  }
}
```

## Tool Definition Macros

The server uses Rust macros to automatically generate MCP-compliant tool definitions:

### Tool Router Macro

```rust
#[tool_router]
impl RobotControlsServer {
    // Automatically generates tool registry and routing
}
```

### Tool Definition Macro

```rust
#[tool(description = "command robot to move forward with a given speed and return IMU data")]
async fn move_forward(&self, Parameters(Speed{speed:s}): Parameters<Speed>) 
    -> Result<CallToolResult, ErrorData> {
    // Tool implementation
}
```

### Parameter Schema Generation

```rust
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct Speed {
    #[schemars(description = " Speed of movement of Robot")]
    speed: f32,
}
```

The `JsonSchema` derive automatically generates the JSON schema for tool parameters.

## Transport Layer

The server uses standard I/O (stdin/stdout) for communication:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = RobotControlsServer::new();
    let _service = server.serve(stdio()).await?;
    tokio::signal::ctrl_c().await?;
    Ok(())
}
```

This allows the server to work with any MCP client that can launch processes and communicate via stdin/stdout.

## Error Handling

MCP defines standard error responses:

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "error": {
    "code": -32000,
    "message": "Robot not responding",
    "data": {
      "type": "INTERNAL_ERROR"
    }
  }
}
```

The server maps internal errors to appropriate MCP error codes for proper client handling.