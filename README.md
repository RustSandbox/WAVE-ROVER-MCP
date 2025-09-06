# LLM Robot Commander
<p align="center">
<em>Giving Robots a Voice. Your Voice.</em>
</p>
![image info]](./logo4.jpg)

Welcome to the LLM Robot Commander project! This isn't just a codebase; it's an experiment in the future of human-robot interaction. We're building a lightweight, high-performance Rust server that acts as a bridge between the boundless creativity of Large Language Models (LLMs) and the tangible actions of a physical robot.

Our **mission** is to tear down the barriers of complex code and proprietary software, allowing anyone to command a robot using the most natural interface there is: language.

## 🚀 The Vision:
A Conversation with Your Robot
Imagine telling your rover, "Go explore the garden and let me know what you see," and watching it happen. That's the future we're building. This project takes your natural language intent, translates it through an LLM into a precise command, and sends it to your robot in real-time.

This server is designed to control a simple, customizable rover, turning it from a programmable machine into an intelligent partner.

## ⚙️ How the Magic Happens
The architecture is designed for simplicity and power, creating a seamless conversation between you, the LLM, and your robot.

**The Spark of an Idea:** You give a command in plain English, like "Drive backward at about half speed."

**The Interpreter:** An LLM, acting as the brain, receives your command. It understands your intent and knows the robot's capabilities by consulting the tools this server provides.

**The Connection:** The LLM selects the perfect tool for the job—in this case, move_backward—and extracts the necessary details, like a speed of 0.5.

**The Message:** A request is sent from the LLM's client to our Rust server. This is where the magic is handed off to pure performance.

**Action!:** Our server instantly constructs the correct JSON command ({"T":1,"L":-0.5,"R":-0.5}) and beams it to the robot over the local network.

**The Feedback Loop:** But it doesn't stop there. After moving, the robot reports back with its sensor data (IMU). This tells the LLM not just that the action was done, but how it went.

**Closing the Loop:** The server relays this success message and the rich sensor data back to the LLM, completing the conversation and readying it for your next command.

### ✨ Why You'll Love This Project
Intuitive Control: Command your robot with words, not code. Unlock robotics for artists, educators, and creators.


**See What the Robot Sees:** The sensor feedback loop gives the LLM—and you—real-time context, opening the door for smarter, more adaptive behaviors.

**Endlessly Extensible:** The foundation is solid. Adding new tools for turning, gripping, or reading new sensors is straightforward and encouraged!

### 📋 Getting Started:
Your Journey Begins Here
Ready to bring your robot to life? Here’s what you’ll need.

**A Rust Toolchain:** The heart of our project. Install it easily via rustup. (https://www.rust-lang.org/tools/install)

**Your Robotic Companion:** A robot on your network ready to listen for commands at http://192.168.4.1. We've included the expected JSON API below to help you get its firmware ready.

**An MCP Client:** The application that connects your LLM to this server.

#### 📦 Installation & First Run
Clone Your Copy:

`git clone [https://github.com/your-username/your-repo.git](https://github.com/your-username/your-repo.git)
cd your-repo`

Build the Executable:

`cargo build --release`

This will create the executable at target/release/robot-mcp.

### 🔌 Connecting to an MCP Client
To allow an application like Claude Desktop or MLStudio to manage and communicate with this server, you need to add it to your client's configuration. Here is an example configuration snippet:

`"RobotCommandServer": {
"command": "/path/to/your/project/target/release/robot-mcp",
"args": []
}`

**Important:** Make sure you replace /path/to/your/project/ with the absolute path to the robot-mcp executable on your system.



#### 🤖 Robot JSON API
This server expects the robot to expose an HTTP endpoint that accepts a json query parameter. The structure of the JSON payload determines the action.

Move Forward/Backward:

T: Command type (integer, 1 for movement).

L: Left motor speed (float). Positive for forward, negative for backward.

R: Right motor speed (float). Positive for forward, negative for backward.

Example Forward: http://192.168.4.1/js?json={"T":1,"L":0.8,"R":0.8}

Example Backward: http://192.168.4.1/js?json={"T":1,"L":-0.5,"R":-0.5}

#### Get IMU Data:

T: Command type (integer, 126 for IMU data).

Example: http://192.168.4.1/js?json={"T":126}

## 🤝 Contributing
Contributions are welcome! If you'd like to improve the project, please follow these steps:

#### Fork the repository.

Create a new branch (`git checkout -b feature/your-feature-name`).

Make your changes.

Commit your changes (`git commit -m 'Add some amazing feature'`).

Push to the branch (`git push origin feature/your-feature-name`).

Open a Pull Request.

# 📄 License
This project is licensed under the MIT License 