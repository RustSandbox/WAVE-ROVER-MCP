# Available Tools

The LLM Robot Commander provides three main tools for robot control. Each tool is designed to be intuitive for LLM clients to understand and use.

## Tool Overview

| Tool Name | Purpose | Parameters | Returns |
|-----------|---------|------------|---------|
| `move_forward` | Move robot forward | `speed: f32` | Status + IMU data |
| `move_backward` | Move robot backward | `speed: f32` | Status + IMU data |
| `stop` | Stop robot movement | None | IMU data |

## move_forward

Move the robot forward at a specified speed.

### Parameters

- **speed** (`f32`): Speed of movement (0.0 to 1.0)
  - `0.0`: No movement
  - `0.5`: Half speed
  - `1.0`: Full speed

### Example Usage

```json
{
  "method": "tools/call",
  "params": {
    "name": "move_forward",
    "arguments": {
      "speed": 0.7
    }
  }
}
```

### Generated Robot Command

```json
{
  "T": 1,
  "L": 0.7,
  "R": 0.7
}
```

### Response Format

```
reaction of Robot is "OK"
. IMU data of robot after this move forward is "accel_x:0.12,accel_y:-0.05,accel_z:9.81,gyro_x:0.01,gyro_y:0.02,gyro_z:-0.001"
```

## move_backward

Move the robot backward at a specified speed.

### Parameters

- **speed** (`f32`): Speed of movement (0.0 to 1.0)
  - Speed is automatically converted to negative values for backward movement

### Example Usage

```json
{
  "method": "tools/call",
  "params": {
    "name": "move_backward",
    "arguments": {
      "speed": 0.4
    }
  }
}
```

### Generated Robot Command

```json
{
  "T": 1,
  "L": -0.4,
  "R": -0.4
}
```

### Response Format

```
reaction of Robot is "OK"
. IMU data of robot after this move backward is "accel_x:-0.08,accel_y:0.03,accel_z:9.79,gyro_x:-0.01,gyro_y:0.01,gyro_z:0.002"
```

## stop

Stop the robot and return current sensor status.

### Parameters

None required.

### Example Usage

```json
{
  "method": "tools/call",
  "params": {
    "name": "stop",
    "arguments": {}
  }
}
```

### Behavior

- Stops current robot movement
- Immediately retrieves and returns IMU sensor data
- No explicit stop command is sent to robot (relies on natural deceleration)

### Response Format

```
accel_x:0.02,accel_y:-0.01,accel_z:9.82,gyro_x:0.00,gyro_y:0.00,gyro_z:0.00
```

## Natural Language Examples

Here are examples of how an LLM might interpret natural language and select tools:

| User Command | Tool Selected | Parameters |
|--------------|---------------|------------|
| "Move forward slowly" | `move_forward` | `speed: 0.3` |
| "Go backward at half speed" | `move_backward` | `speed: 0.5` |
| "Full speed ahead!" | `move_forward` | `speed: 1.0` |
| "Stop the robot" | `stop` | None |
| "What's the robot's current status?" | `stop` | None |
| "Move back a little" | `move_backward` | `speed: 0.2` |

## Error Handling

All tools implement comprehensive error handling:

### Network Errors
If the robot is unreachable:
```
reaction of Robot is "Robot not responding"
. IMU data of robot after this move forward is "Robot not responding"
```

### Invalid Parameters
- Speed values are clamped to valid ranges
- Invalid JSON is handled gracefully
- Missing parameters return clear error messages

## Tool Registration

Tools are automatically registered via the `#[tool_router]` macro:

```rust
#[tool_router]
impl RobotControlsServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
    // ... tool implementations
}
```

This ensures all tools are discoverable by MCP clients and properly documented with their descriptions and parameter schemas.