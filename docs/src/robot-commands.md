# Robot Commands

This section describes the JSON command protocol used to communicate with the robot hardware.

## Protocol Overview

The robot accepts HTTP GET requests with JSON payloads as query parameters:

```
http://192.168.4.1/js?json={COMMAND_JSON}
```

## Command Types

### Movement Commands (T: 1)

Control robot movement using differential drive (left and right motor speeds).

#### Forward Movement
```json
{
  "T": 1,
  "L": 0.8,
  "R": 0.8
}
```

#### Backward Movement
```json
{
  "T": 1,
  "L": -0.5,
  "R": -0.5
}
```

#### Turning (Differential Speeds)
```json
// Turn right
{
  "T": 1,
  "L": 0.6,
  "R": 0.2
}

// Turn left
{
  "T": 1,
  "L": 0.2,
  "R": 0.6
}

// Spin in place (right)
{
  "T": 1,
  "L": 0.5,
  "R": -0.5
}
```

#### Parameters

- **T**: Command type (always `1` for movement)
- **L**: Left motor speed (`-1.0` to `1.0`)
  - Positive values: forward direction
  - Negative values: backward direction
  - `0.0`: stopped
- **R**: Right motor speed (`-1.0` to `1.0`)
  - Same range and meaning as left motor

### Sensor Commands (T: 126)

Retrieve IMU (Inertial Measurement Unit) sensor data.

```json
{
  "T": 126
}
```

#### Response Format

The robot returns IMU data as a comma-separated string:
```
accel_x:0.12,accel_y:-0.05,accel_z:9.81,gyro_x:0.01,gyro_y:0.02,gyro_z:-0.001
```

#### Data Fields

- **accel_x**: Acceleration in X-axis (m/s²)
- **accel_y**: Acceleration in Y-axis (m/s²)
- **accel_z**: Acceleration in Z-axis (m/s²) - typically ~9.81 due to gravity
- **gyro_x**: Angular velocity around X-axis (rad/s)
- **gyro_y**: Angular velocity around Y-axis (rad/s)
- **gyro_z**: Angular velocity around Z-axis (rad/s)

## Implementation in the Server

### Command Structure Definitions

```rust
#[derive(Serialize)]
pub struct GoForward {
    #[serde(rename = "T")]
    command: u8,        // Always 1
    #[serde(rename = "L")]
    left_speed: f32,
    #[serde(rename = "R")]
    right_speed: f32,
}

#[derive(Serialize)]
pub struct IMUData {
    #[serde(rename = "T")]
    command: u8,        // Always 126
}
```

### HTTP Communication

```rust
fn command_to_robot<T: Serialize>(command: T) -> Result<String, String> {
    let json = serde_json::to_string(&command).unwrap();
    let url = format!("http://{}/js?json={}", ROVER_IP, json);
    
    match reqwest::blocking::get(&url) {
        Ok(response) => Ok(response.text().unwrap_or("No response".to_string())),
        Err(_) => Ok("Robot not responding".to_string()),
    }
}
```

## Testing Robot Commands

You can test robot commands directly using curl:

### Test Movement
```bash
# Move forward at 50% speed
curl "http://192.168.4.1/js?json=%7B%22T%22%3A1%2C%22L%22%3A0.5%2C%22R%22%3A0.5%7D"

# Move backward at 30% speed
curl "http://192.168.4.1/js?json=%7B%22T%22%3A1%2C%22L%22%3A-0.3%2C%22R%22%3A-0.3%7D"

# Stop (zero speed)
curl "http://192.168.4.1/js?json=%7B%22T%22%3A1%2C%22L%22%3A0.0%2C%22R%22%3A0.0%7D"
```

### Test IMU Data
```bash
# Get sensor data
curl "http://192.168.4.1/js?json=%7B%22T%22%3A126%7D"
```

## Robot Hardware Requirements

Your robot must implement:

1. **HTTP Server**: Listening on port 80
2. **Endpoint**: `/js` accepting GET requests
3. **JSON Parameter**: `json` query parameter
4. **Movement Response**: Acknowledge movement commands
5. **Sensor Response**: Return IMU data in the specified format

## Expected Responses

### Successful Movement
```
OK
```

### Successful IMU Retrieval
```
accel_x:0.02,accel_y:-0.01,accel_z:9.82,gyro_x:0.00,gyro_y:0.00,gyro_z:0.00
```

### Error Conditions
- **Network timeout**: No response
- **Invalid JSON**: Error message from robot
- **Hardware failure**: Robot-specific error message