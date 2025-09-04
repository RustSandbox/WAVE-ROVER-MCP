use rmcp::handler::server::tool::{ ToolRouter};
use rmcp::model::{CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo};
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::transport::stdio;
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler, ServiceExt};
use serde::Deserialize;
use std::fs;
use tracing::info;



use rmcp::handler::server::wrapper::Parameters;

const ROVER_IP: &str = "192.168.4.1";

fn send_command(json: &str) -> String {
	let url = format!("http://{}/js?json={}", ROVER_IP, json);

	match reqwest::blocking::get(&url) {
		Ok(response) => response.text().unwrap_or("No response".to_string()),
		Err(e) => format!("Error: {}", e),
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

	#[tool(description = "command robot to move forward")]
	async fn forward(&self,/* param: Parameters<StoreRequest>*/) -> Result<CallToolResult, ErrorData> {
		let command = r#"{"T":1,"L":0.25,"R":0.25}"#;
		send_command(command);

		Ok(CallToolResult::success(vec![Content::text("Moving forward ...")]))
	}
	#[tool(description = "Get a memory about user")]
	async fn stop(&self) -> Result<CallToolResult, ErrorData> {
		let command = r#"{"T":1,"L":0,"R":0}"#;
		send_command(command);

		Ok(CallToolResult::success(vec![Content::text("Stopping robot ...")]))
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
