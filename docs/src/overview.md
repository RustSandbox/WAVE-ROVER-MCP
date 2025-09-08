# Overview

## System Architecture

The LLM Robot Commander consists of three main components:

1. **MCP Server** (This Project): A Rust-based server that implements the Model Context Protocol
2. **LLM Client**: Any MCP-compatible client (Claude Desktop, MLStudio, etc.)
3. **Robot Hardware**: A network-connected robot that accepts HTTP commands

## Communication Flow

```
User Input → LLM Client → MCP Server → Robot Hardware
                ↑                         ↓
          Sensor Data ← MCP Server ← Robot Response
```

## ⚙️ How the Magic Happens

**The Spark of an Idea:** You give a command in plain English, like "Drive backward at about half speed."

**The Interpreter:** An LLM, acting as the brain, receives your command. It understands your intent and knows the robot's capabilities by consulting the tools this server provides.

**The Connection:** The LLM selects the perfect tool for the job—in this case, `move_backward`—and extracts the necessary details, like a speed of 0.5.

**The Message:** A request is sent from the LLM's client to our Rust server. This is where the magic is handed off to pure performance.

**Action!:** Our server instantly constructs the correct JSON command (`{"T":1,"L":-0.5,"R":-0.5}`) and beams it to the robot over the local network.

**The Feedback Loop:** After moving, the robot reports back with its sensor data (IMU). This tells the LLM not just that the action was done, but how it went.

**Closing the Loop:** The server relays this success message and the rich sensor data back to the LLM, completing the conversation and readying it for your next command.

## Benefits

### ✨ Why You'll Love This Project

**Intuitive Control**: Command your robot with words, not code. Unlock robotics for artists, educators, and creators.

**See What the Robot Sees**: The sensor feedback loop gives the LLM—and you—real-time context, opening the door for smarter, more adaptive behaviors.

**Endlessly Extensible**: The foundation is solid. Adding new tools for turning, gripping, or reading new sensors is straightforward and encouraged!

## Requirements

- **Rust Toolchain**: For building and running the server
- **Robot Hardware**: A network-connected robot at `192.168.4.1`
- **MCP Client**: Any application that supports the Model Context Protocol