# Meow Auto Clicker 3.0

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

[中文说明](./README_zh_CN.md)  

An advanced automatic mouse clicker written in Rust with a sleek GUI interface. Perfect for automation tasks, gaming, and repetitive clicking operations.

## 🚀 Features

### 🖱️ Mouse Control
- **Multiple Click Types**: Support for single click and double click operations
- **Left/Right Click**: Choose between left or right mouse button clicking
- **Custom Coordinates**: Intelligent mode allows recording and playback of precise mouse movements

### ⚡ Clicking Modes
- **Ordinary Mode**: Simple automated clicking with fixed intervals
- **Intelligent Mode**: Record and replay complex mouse movement sequences
  - Record mouse positions and clicks
  - Save and load recording sessions
  - Custom sleep intervals between actions

### 🎯 Hotkey Support
- **Customizable Hotkeys**: Set your preferred key combinations for:
  - Start/Stop clicking operations
  - Record mouse movements (Intelligent Mode only)
- **Smart Debouncing**: Prevents accidental multiple triggers

### ⚙️ Advanced Settings
- **Click Count Control**: Set exact number of clicks to perform
- **Time Interval Configuration**: Adjust delay between clicks (milliseconds)
- **Sound Effects**: Enable/disable audio feedback for recording actions
- **Persistence**: Automatic saving of settings and recordings

### 📊 Real-time Status
- **Visual Indicators**: Color-coded status showing clicking, recording, or stopped states
- **Record Counter**: Track number of recorded actions in Intelligent Mode

## 🛠️ Installation

### Prerequisites
- Rust and Cargo installed on your system

### Building from Source
```bash
# Clone the repository
git clone git@github.com:jcglqmoyx/clicker.git
cd clicker

# Build the project
cargo build --release

# Run the application
cargo run --release
```

The built binary will be available at `target/release/clicker`

## 📖 Usage

1. **Launch the Application**: Run the executable to open the GUI
2. **Configure Settings**:
   - Select mouse button (Left/Right)
   - Choose click type (Single/Double)
   - Set click count and time interval
3. **Set Hotkeys**: Configure your preferred key combinations
4. **Choose Mode**:
   - **Ordinary Mode**: For simple repetitive clicking
   - **Intelligent Mode**: For complex sequences (record movements first)
5. **Start Clicking**: Use the hotkey to start/stop the clicking process

### Intelligent Mode Workflow
1. Enable "Smart Click" checkbox
2. Position your mouse where you want to record the first click
3. Press the record hotkey to capture the position and click
4. Repeat for each step in your sequence
5. Use the start hotkey to replay the entire sequence

## 🏗️ Technical Details

### Built With
- **Rust**: Systems programming language for performance and safety
- **FLTK**: Lightweight GUI toolkit for cross-platform compatibility
- **DeviceQuery**: Keyboard and mouse input handling
- **Enigo**: Cross-platform input automation
- **Rusqlite**: SQLite database for persistence
- **Rodio**: Audio playback for sound effects

### Architecture
The application follows a modular architecture with clear separation of concerns:
- **GUI Layer**: FLTK-based interface components
- **Business Logic**: Global state management and click operations
- **Persistence**: SQLite database for storing settings and recordings
- **Input Handling**: Device query and Enigo for input automation

## 🔧 Configuration

Settings are automatically saved to a local SQLite database in the application data directory. You can:
- Clear recorded sequences
- Save/Load specific recording sessions
- Customize all operational parameters

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests for:
- Bug fixes
- New features
- Documentation improvements
- Performance optimizations

## 📄 License

This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)**.