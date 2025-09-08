# System Design

## Architecture Overview

The LLM Robot Commander follows a three-tier architecture designed for simplicity, performance, and extensibility:

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   LLM Client    │    │   MCP Server     │    │  Robot Hardware │
│  (Claude, etc.) │◄──►│  (This Project)  │◄──►│  (HTTP API)     │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Component Breakdown

### 1. MCP Server (Core Component)

**Technology**: Rust with async/await
**Responsibilities**:
- Implements Model Context Protocol (MCP)
- Provides tool interfaces for robot control
- Handles HTTP communication with robot hardware
- Manages sensor data collection and reporting
- Error handling and logging

**Key Modules**:
```rust
// Tool definitions and handlers
#[tool_router]
impl RobotControlsServer {
    #[tool(description = "command robot to move forward with a given speed")]
    async fn move_forward(&self, ...) -> Result<CallToolResult, ErrorData>
    
    #[tool(description = "command robot to move backward with a given speed")]
    async fn move_backward(&self, ...) -> Result<CallToolResult, ErrorData>
    
    #[tool(description = "Command Robot to stop")]
    async fn stop(&self) -> Result<CallToolResult, ErrorData>
}
```

### 2. LLM Client

**Examples**: Claude Desktop, MLStudio, or any MCP-compatible client
**Responsibilities**:
- Natural language processing and understanding
- Tool selection based on user intent
- Parameter extraction and validation
- Result presentation to the user

### 3. Robot Hardware

**Interface**: HTTP REST API
**Expected Location**: `192.168.4.1`
**Responsibilities**:
- Physical movement execution
- Sensor data collection (IMU)
- Status reporting

## Data Flow

### Command Execution Flow

1. **User Input**: Natural language command
2. **LLM Processing**: Intent recognition and tool selection
3. **MCP Call**: Tool invocation with parameters
4. **JSON Generation**: Server creates robot-specific command
5. **HTTP Request**: Command sent to robot
6. **Robot Execution**: Physical action performed
7. **Sensor Retrieval**: IMU data collected
8. **Response Assembly**: Status and sensor data combined
9. **Result Return**: Information sent back to LLM
10. **User Presentation**: Natural language response

### Example Data Transformation

```
"Move forward slowly" → 
move_forward(speed: 0.3) → 
{"T":1,"L":0.3,"R":0.3} → 
HTTP GET to robot → 
Robot response + IMU data → 
"Robot moved forward successfully. Current IMU reading: ..."
```

## Design Patterns

### Tool Pattern

Each robot capability is exposed as an MCP tool:
```rust
#[tool(description = "...")]
async fn tool_name(&self, parameters: Parameters<InputType>) 
    -> Result<CallToolResult, ErrorData>
```

Benefits:
- Clear separation of concerns
- Easy to add new capabilities
- Automatic parameter validation
- Consistent error handling

### Command Pattern

Robot commands are structured as serializable data:
```rust
#[derive(Serialize)]
pub struct GoForward {
    #[serde(rename = "T")]
    command: u8,
    #[serde(rename = "L")]
    left_speed: f32,
    #[serde(rename = "R")]
    right_speed: f32,
}
```

Benefits:
- Type safety
- Consistent serialization
- Easy to extend with new command types

### Feedback Loop Pattern

Every action includes sensor data retrieval:
```rust
let result = command_to_robot(command);
let imu_data = retrieve_imu_data().unwrap();
let return_message = format!("reaction: {:?}\nIMU data: {:?}", result, imu_data);
```

Benefits:
- Rich context for LLM decision-making
- Real-time robot status awareness
- Enhanced user feedback

## Error Handling

The system implements multiple layers of error handling:

1. **Network Level**: HTTP request failures
2. **Protocol Level**: Invalid JSON responses
3. **Application Level**: Robot command failures
4. **MCP Level**: Tool execution errors

## Scalability Considerations

### Current Limitations
- Single robot support (hardcoded IP)
- Synchronous robot communication
- Basic sensor data (IMU only)

### Extension Points
- Multiple robot support
- Async robot communication
- Additional sensor types
- Complex command sequences
- State persistence