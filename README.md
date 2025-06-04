# Galaxy RTS - Galactic Scale Real-Time Strategy Game

A revolutionary real-time strategy game built with Rust and macroquad that features seamless zoom from individual units to the entire observable universe, complete 3D asset pipeline, and advanced networking capabilities.

## 🌌 Unique Features

### Galaxy-Scale Zoom System
- **50 Zoom Levels**: From individual units (1 meter) to the observable universe (93 billion light-years)
- **Logarithmic Scaling**: Mathematically accurate zoom progression with factor ~3.55
- **Smooth Transitions**: Interpolated zoom with real-time level descriptions
- **Home Navigation**: Instant return to your headquarters with 'H' key

### Advanced 3D Asset System
- **Modular Animations**: Each unit/building has separate animation files for optimal loading
- **Asset Manifest**: JSON-driven asset management with detailed animation specifications
- **Resource Manager**: Efficient loading and caching of models, textures, sounds, and fonts
- **Animation States**: Context-aware animations based on unit behavior

### Galactic Network Architecture
- **Sector-Based Partitioning**: Hierarchical spatial organization for massive multiplayer support
- **Delta Compression**: Efficient network updates with bitfield change tracking
- **Load Balancing**: Dynamic server migration for optimal performance
- **Real-time Events**: Combat events and inter-server communication

## 🎮 Gameplay Features

### Core RTS Mechanics
- **Multiple Unit Types**: Workers, Fighters, Rangers, Tanks with unique abilities
- **Building System**: Headquarters, Barracks, Factories, Energy Plants, Defense Turrets
- **Resource Management**: Minerals and Energy with automated worker AI
- **Combat System**: Real-time combat with range, damage, and cooldown mechanics

### AI System
- **Intelligent Opponents**: AI that manages resources, builds structures, and plans attacks
- **Behavioral Trees**: Modular AI decision making for different strategies
- **Adaptive Difficulty**: AI adjusts tactics based on player performance

### Enhanced UI/UX
- **Minimap**: Real-time overview with unit positions and camera view
- **Selection System**: Individual and group unit selection with visual feedback
- **Audio Integration**: Comprehensive sound effects and background music
- **Settings Management**: Configurable audio, video, and control options

## 🚀 Getting Started

### Prerequisites
- Rust 1.70 or higher
- macroquad dependencies for your platform

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/galaxy_rts.git
cd galaxy_rts/RTS_GALAXY

# Build and run
cargo run --release
```

### Quick Start Guide

1. **Main Menu**: Start with the main menu to access game settings or begin playing
2. **Basic Controls**:
   - `WASD` or Arrow Keys: Move camera
   - `Mouse Wheel` or `+/-`: Zoom in/out through 50 levels
   - `H` or `Home`: Return to headquarters
   - `Left Click`: Select units
   - `Right Click`: Move/Attack command
   - `Click + Drag`: Group selection

3. **Zoom Navigation**:
   - Start at Level 15 (Continental scale)
   - Zoom in to see individual units (Levels 1-3)
   - Zoom out to see planetary systems (Levels 19-21)
   - Explore galactic scales (Levels 34-36)
   - View the entire universe (Level 50)

## 📁 Project Structure

```
RTS_GALAXY/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── game/                   # Core game logic
│   │   ├── state.rs           # Game state management
│   │   ├── zoom.rs            # Galaxy-scale zoom system
│   │   ├── rendering.rs       # Game rendering pipeline
│   │   └── ...
│   ├── entity/                # Game entities (units, buildings, players)
│   ├── resources/             # Asset management system
│   ├── network/               # Networking and multiplayer
│   ├── audio/                 # Audio management
│   ├── ai/                    # AI controllers and behaviors
│   └── ui/                    # User interface components
├── assets/                    # Game assets (see Asset System below)
├── Cargo.toml                # Rust dependencies
└── README.md                 # This file
```

## 🎨 Asset System

### Directory Structure
```
assets/
├── models/                    # 3D models and animations
│   ├── units/                # Unit models with animation subfolders
│   ├── buildings/            # Building models with animation subfolders
│   ├── resources/            # Resource node models
│   └── effects/              # Visual effect models
├── textures/                 # UI textures organized by category
├── sounds/                   # Sound effects by category
├── music/                    # Background music files
├── fonts/                    # Game fonts
└── asset_manifest.json      # Asset configuration and metadata
```

### Asset Loading System
The game uses a sophisticated asset management system:

1. **Manifest-Driven**: All assets defined in `asset_manifest.json`
2. **Lazy Loading**: Assets loaded on-demand for optimal memory usage
3. **Animation Management**: Individual animation files for modularity
4. **Resource Caching**: Efficient memory management with automatic cleanup

### Creating New Assets
To add new assets:

1. Place files in appropriate `assets/` subdirectory
2. Update `asset_manifest.json` with asset metadata
3. Reference assets by name in code using `ResourceManager`

Example animation entry:
```json
{
  "name": "worker_idle",
  "file": "models/units/worker/animations/idle.glb",
  "duration": 2.0,
  "loop": true,
  "speed": 1.0
}
```

## 🌐 Network Architecture

### Sector System
- **Hierarchical Partitioning**: Universe divided into sectors at multiple scales
- **Dynamic Load Balancing**: Sectors can migrate between servers
- **Spatial Optimization**: Players only receive updates for relevant sectors

### Message Types
- **Unit Updates**: Position, health, and state changes with delta compression
- **Combat Events**: Real-time battle notifications
- **Server Sync**: Inter-server communication for seamless experience
- **Player Management**: Join/leave and authentication

### Scalability Features
- **Compressed Updates**: Bitfield change tracking reduces bandwidth
- **Batch Operations**: Multiple commands processed together
- **Priority Systems**: Critical updates prioritized over routine data

## 🎵 Audio System

### Sound Categories
- **UI Sounds**: Menu interactions and interface feedback
- **Unit Sounds**: Movement, combat, and selection audio
- **Building Sounds**: Construction, operation, and destruction
- **Ambient**: Background music and environmental audio

### Features
- **Volume Controls**: Separate settings for sound effects and music
- **Mute Options**: Individual mute controls for different audio types
- **Context-Aware**: Sounds triggered by appropriate game events
- **Audio Manager**: Centralized audio control with resource integration

## 🤖 AI System

### AI Behaviors
- **Resource Management**: Automatic worker assignment to resource nodes
- **Base Building**: Strategic placement of buildings and defenses
- **Military Planning**: Unit production and attack coordination
- **Adaptive Tactics**: AI adjusts strategy based on player actions

### Implementation
- **Behavior Trees**: Modular decision-making system
- **State Machines**: Unit and building AI states
- **Pathfinding**: Efficient movement calculation
- **Performance Optimized**: AI updates run at appropriate intervals

## 🔧 Configuration

### Game Settings
- **Audio**: Volume controls for music and sound effects
- **Graphics**: Quality settings and performance options
- **Controls**: Customizable key bindings
- **Network**: Connection settings for multiplayer

### Development Configuration
- **Debug Mode**: Additional logging and development tools
- **Asset Hot-Reloading**: Real-time asset updates during development
- **Performance Monitoring**: Frame rate and memory usage tracking

## 🛠️ Development

### Building from Source
```bash
# Development build (faster compilation)
cargo run

# Release build (optimized performance)
cargo run --release

# Run tests
cargo test

# Check code formatting
cargo fmt

# Lint code
cargo clippy
```

### Adding New Features

1. **Game Logic**: Add to appropriate module in `src/game/`
2. **Entities**: Define new units/buildings in `src/entity/`
3. **UI Components**: Create interfaces in `src/ui/`
4. **Assets**: Follow asset system guidelines
5. **Tests**: Add unit tests for new functionality

### Performance Optimization
- Use `cargo run --release` for performance testing
- Profile with `cargo flamegraph` for bottleneck identification
- Monitor memory usage with built-in game profiler
- Optimize asset loading for target platforms

## 📋 Requirements

### System Requirements
- **OS**: Windows, macOS, or Linux
- **RAM**: 4GB minimum, 8GB recommended
- **Graphics**: OpenGL 3.3 compatible
- **Storage**: 2GB for full asset library

### Development Requirements
- **Rust**: 1.70+ with cargo
- **IDE**: VS Code with rust-analyzer recommended
- **Tools**: git, optional audio/graphics editing software

## 🚀 Future Roadmap

### Planned Features
- [ ] **Advanced Graphics**: PBR shading and particle effects
- [ ] **Expanded Universe**: More unit types and building options
- [ ] **Campaign Mode**: Single-player story campaign
- [ ] **Mod Support**: Lua scripting for custom content
- [ ] **VR Support**: Virtual reality gameplay mode

### Performance Goals
- [ ] **Optimization**: Support for 10,000+ simultaneous units
- [ ] **Networking**: 1000+ players per server cluster
- [ ] **Cross-Platform**: Mobile and console versions

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines
- Follow Rust naming conventions
- Add tests for new functionality
- Update documentation for API changes
- Ensure assets follow the manifest system

## 🆘 Troubleshooting

### Common Issues

**Game won't start:**
- Ensure Rust 1.70+ is installed
- Check that all dependencies are available
- Verify graphics drivers support OpenGL 3.3

**Poor performance:**
- Run with `cargo run --release`
- Reduce graphics settings in game menu
- Close other applications to free memory

**Audio issues:**
- Check system audio settings
- Verify audio files are in correct format
- Ensure game audio is not muted

**Asset loading errors:**
- Verify `asset_manifest.json` syntax
- Check that referenced files exist
- Ensure proper file permissions

### Getting Help
- Check the Issues tab on GitHub
- Review documentation in `docs/` folder
- Join community discussions
- Contact developers through project channels

---

**Ready to command your galactic empire? Download and start building your space-faring civilization today!**
