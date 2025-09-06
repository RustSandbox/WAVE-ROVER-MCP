use std::result;
use rmcp::handler::server::tool::{ ToolRouter};
use rmcp::model::{CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo};
use rmcp::transport::stdio;
use rmcp::{schemars, tool, tool_handler, tool_router, ErrorData, ServerHandler, ServiceExt};
use rmcp::handler::server::wrapper::Parameters;
use serde::Deserialize;
use tracing::info;
use serde::Serialize;
use serde_json::to_string;
const ROVER_IP: &str = "192.168.4.1";

fn send_command(json: &str) -> String {
	let url = format!("http://{}/js?json={}", ROVER_IP, json);

	match reqwest::blocking::get(&url) {
		Ok(response) => response.text().unwrap_or("No response".to_string()),
		Err(e) => format!("Error: {}", e),
	}
}
#[derive(Serialize)]
pub struct GoForward{
	#[serde(rename = "T")]
	command: u8, // Always 1
	#[serde(rename = "L")]
	left_speed: f32,
	#[serde(rename = "R")]
	right_speed: f32,
}
impl GoForward{
	fn new( speed: f32) -> Self{
		Self{
			command:1,
			left_speed: speed,
			right_speed: speed
		}
	}
}
#[derive(Serialize)]
pub struct IMUData{
	#[serde(rename = "T")]
	command: u8,
}
impl IMUData{
	pub fn new() -> Self{
		Self{
			command:126
		}
	}
}

pub fn retrieve_imu_data() ->Result<String,String>{
	let retrieve_imu_data_command = IMUData::new();
	command_to_robot(retrieve_imu_data_command)
}
#[derive(Serialize)]
pub struct GoBackward {
	#[serde(rename = "T")]
	command: u8, // Always 1
	#[serde(rename = "L")]
	left_speed: f32,
	#[serde(rename = "R")]
	right_speed: f32,
}
impl GoBackward {
	fn new( speed: f32) -> Self{
		Self{
			command:1,
			left_speed: -speed,
			right_speed: -speed,
		}
	}
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct Speed{
	#[schemars(description = " Speed of movement of Robot")]
	speed: f32,
}
fn command_to_robot<T:Serialize>(command: T)-> Result<String,String>{
	let json= serde_json::to_string(&command).unwrap();
	let url = format!("http://{}/js?json={}", ROVER_IP, json);
	match reqwest::blocking::get(&url) {
		Ok(response) => Ok(response.text().unwrap_or("No response".to_string())),
		Err(_) => Ok("Robot not responding".to_string()),
	}
}
#[derive(Clone)]
struct RobotControlsServer {
	tool_router: ToolRouter<Self>,
}



#[tool_router]
impl RobotControlsServer {
	fn new() -> Self {
		Self {
			tool_router: Self::tool_router(),
		}
	}

	#[tool(description = "command robot to move forward with a given speed and return IMU data")]
	async fn move_forward(&self, Parameters(Speed{speed:s}): Parameters<Speed>) -> Result<CallToolResult, ErrorData> {
		let command=GoForward::new(s);
		let result = command_to_robot(command);
		let imu_data = retrieve_imu_data().unwrap();
		let return_message = format!("reaction of Robot is {:?}\n. IMU data of robot after this move forward is {:?}",
		                             result.unwrap(), imu_data);
		Ok(CallToolResult::success(vec![Content::text(return_message)]))
	}
	#[tool(description = "command robot to move backward with a given speed and return IMU data")]
	async fn move_backward(&self, Parameters(Speed{speed:s}): Parameters<Speed>) -> Result<CallToolResult, ErrorData> {
		let command= GoBackward::new(s);
		let result = command_to_robot(command);
		let imu_data = retrieve_imu_data().unwrap();
		let return_message = format!("reaction of Robot is {:?}\n. IMU data of robot after this move backward is {:?}",
		                             result.unwrap(), imu_data);

		Ok(CallToolResult::success(vec![Content::text(return_message)]))
	}

	#[tool(description = "Command Robot to stop")]
	async fn stop(&self) -> Result<CallToolResult, ErrorData> {
		Ok(CallToolResult::success(vec![Content::text(retrieve_imu_data().unwrap())]))
	}
}

#[tool_handler]
impl ServerHandler for RobotControlsServer {
	fn get_info(&self) -> ServerInfo {
		info!("Providing server info");
		ServerInfo {
			protocol_version: ProtocolVersion::V_2024_11_05,
			capabilities: ServerCapabilities::builder().enable_tools().build(),
			server_info: Implementation::from_build_env(),
			instructions: Some("this server allow command robot to move forward or stop".to_string()),
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Initialize tracing to stderr so it appears in the client logs
	tracing_subscriber::fmt()
		.with_writer(std::io::stderr)
		.init();

	info!("Starting Robot commander MCP server");

	// Create the server instance
	let server = RobotControlsServer::new();

	// Serve indefinitely - this will keep the process alive
	info!("About to start serving...");
	let _service = server.serve(stdio()).await?;
	info!("Service initialized, waiting for requests...");

	// Wait for termination signal (Ctrl+C)
	tokio::signal::ctrl_c().await?;
	info!("Received shutdown signal");

	info!("Server shutting down");
	Ok(())
}
