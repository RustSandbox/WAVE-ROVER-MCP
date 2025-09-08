# IMU Data

The Inertial Measurement Unit (IMU) provides crucial sensor data about the robot's movement and orientation.

## Data Format

IMU data is returned as a comma-separated string:
```
accel_x:0.12,accel_y:-0.05,accel_z:9.81,gyro_x:0.01,gyro_y:0.02,gyro_z:-0.001
```

## Data Fields

### Accelerometer Data (m/s²)

- **accel_x**: Linear acceleration along the X-axis
- **accel_y**: Linear acceleration along the Y-axis  
- **accel_z**: Linear acceleration along the Z-axis (includes gravity ~9.81 m/s²)

### Gyroscope Data (rad/s)

- **gyro_x**: Angular velocity around the X-axis (roll rate)
- **gyro_y**: Angular velocity around the Y-axis (pitch rate)
- **gyro_z**: Angular velocity around the Z-axis (yaw rate)

## Coordinate System

The IMU typically uses a right-handed coordinate system:

```
        Y (forward)
        ^
        |
        |
Z (up)  |
   ^    |
   |    |
   |    +------> X (right)
   |   /
   |  /
   | /
   |/
```

### Movement Interpretation

- **Forward/Backward**: Changes in `accel_y`
- **Left/Right**: Changes in `accel_x`  
- **Up/Down**: Changes in `accel_z`
- **Turning**: Changes in `gyro_z`
- **Tilting**: Changes in `gyro_x` and `gyro_y`

## Typical Values

### At Rest (Stationary Robot)
```
accel_x:0.02,accel_y:-0.01,accel_z:9.82,gyro_x:0.00,gyro_y:0.00,gyro_z:0.00
```

- Small accelerometer values due to sensor noise
- Z-acceleration ~9.81 m/s² due to gravity
- Near-zero gyroscope values

### During Forward Movement
```
accel_x:0.05,accel_y:0.85,accel_z:9.78,gyro_x:0.01,gyro_y:0.02,gyro_z:0.01
```

- Positive Y-acceleration (forward direction)
- Small changes in other axes due to vibration
- Small gyroscope values if moving straight

### During Turning
```
accel_x:0.15,accel_y:0.45,accel_z:9.80,gyro_x:0.02,gyro_y:0.01,gyro_z:0.35
```

- Non-zero Z-gyroscope (turning motion)
- X-acceleration from centripetal force
- Combined forward and rotational motion

## Data Collection

### Automatic Collection
IMU data is automatically collected after each movement command:
```rust
let result = command_to_robot(command);
let imu_data = retrieve_imu_data().unwrap();
let return_message = format!(
    "reaction of Robot is {:?}\n. IMU data of robot after this move is {:?}",
    result.unwrap(), 
    imu_data
);
```

### Manual Collection
Use the `stop` tool to get current IMU data without moving:
```json
{
  "method": "tools/call",
  "params": {
    "name": "stop",
    "arguments": {}
  }
}
```

## Data Quality Considerations

### Sensor Noise
- IMU sensors always have some noise
- Values may fluctuate slightly even when stationary
- Filter values if high precision is needed

### Calibration
- IMUs may need calibration after power-on
- Zero-point drift can occur over time
- Temperature affects sensor accuracy

### Mounting Orientation
- IMU must be properly aligned with robot chassis
- Sensor orientation affects data interpretation
- Consistent mounting is crucial for accurate readings

## Data Processing

### Parsing IMU Data
```rust
fn parse_imu_data(data: &str) -> Result<IMUReading, String> {
    let mut values = HashMap::new();
    
    for pair in data.split(',') {
        let parts: Vec<&str> = pair.split(':').collect();
        if parts.len() == 2 {
            let key = parts[0];
            let value: f32 = parts[1].parse()
                .map_err(|_| format!("Invalid number: {}", parts[1]))?;
            values.insert(key, value);
        }
    }
    
    Ok(IMUReading {
        accel_x: values.get("accel_x").copied().unwrap_or(0.0),
        accel_y: values.get("accel_y").copied().unwrap_or(0.0),
        accel_z: values.get("accel_z").copied().unwrap_or(0.0),
        gyro_x: values.get("gyro_x").copied().unwrap_or(0.0),
        gyro_y: values.get("gyro_y").copied().unwrap_or(0.0),
        gyro_z: values.get("gyro_z").copied().unwrap_or(0.0),
    })
}
```

### Filtering and Smoothing
```rust
// Simple moving average filter
struct IMUFilter {
    history: VecDeque<IMUReading>,
    window_size: usize,
}

impl IMUFilter {
    fn add_reading(&mut self, reading: IMUReading) -> IMUReading {
        self.history.push_back(reading);
        if self.history.len() > self.window_size {
            self.history.pop_front();
        }
        
        // Calculate average
        let mut sum = IMUReading::default();
        for r in &self.history {
            sum.accel_x += r.accel_x;
            sum.accel_y += r.accel_y;
            // ... sum other fields
        }
        
        let count = self.history.len() as f32;
        IMUReading {
            accel_x: sum.accel_x / count,
            accel_y: sum.accel_y / count,
            // ... average other fields
        }
    }
}
```

## Applications

### Motion Detection
```rust
fn detect_motion(imu: &IMUReading) -> bool {
    let accel_magnitude = (
        imu.accel_x.powi(2) + 
        imu.accel_y.powi(2) + 
        (imu.accel_z - 9.81).powi(2)
    ).sqrt();
    
    let gyro_magnitude = (
        imu.gyro_x.powi(2) + 
        imu.gyro_y.powi(2) + 
        imu.gyro_z.powi(2)
    ).sqrt();
    
    accel_magnitude > 0.5 || gyro_magnitude > 0.1
}
```

### Orientation Estimation
```rust
fn estimate_tilt(imu: &IMUReading) -> (f32, f32) {
    let roll = imu.accel_y.atan2(imu.accel_z);
    let pitch = (-imu.accel_x).atan2(
        (imu.accel_y.powi(2) + imu.accel_z.powi(2)).sqrt()
    );
    (roll.to_degrees(), pitch.to_degrees())
}
```

### Collision Detection
```rust
fn detect_collision(imu: &IMUReading) -> bool {
    let impact_threshold = 20.0; // m/s²
    let total_accel = (
        imu.accel_x.powi(2) + 
        imu.accel_y.powi(2) + 
        imu.accel_z.powi(2)
    ).sqrt();
    
    total_accel > impact_threshold
}
```