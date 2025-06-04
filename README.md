# RTS Galaxy - Revolutionary Real-Time Strategy Game

A next-generation RTS game featuring a unique 50-level zoom system spanning from individual units to the observable universe, built with Rust and modern game development practices.

## 🌌 Unique Features

### Revolutionary Zoom System
- **50 Discrete Zoom Levels**: Navigate seamlessly from 1-meter unit scale to 93-billion light-year cosmic scale
- **Mathematical Precision**: Logarithmic scaling with zoom factor F ≈ 3.55
- **Smart LOD System**: Automatic level-of-detail adjustments for optimal performance
- **Contextual UI**: Interface adapts to current scale for optimal usability

### Advanced Gameplay
- **Multi-Scale Strategy**: Command individual soldiers to galactic fleets
- **Resource Management**: Minerals and energy with automated worker AI
- **Unit Production**: 4 distinct unit types with unique roles and abilities
- **Base Building**: Strategic construction with defensive capabilities
- **AI Opponents**: Challenging AI with dynamic difficulty adjustment

### Technical Innovation
- **3D Animation System**: Modular animation architecture with separate files
- **Network Architecture**: Scalable multiplayer with spatial partitioning
- **Audio Engine**: Dynamic 3D audio with distance-based attenuation
- **Asset Streaming**: Efficient loading based on zoom level and proximity

## 🚀 Quick Start

### Prerequisites
```bash
# Install Rust (latest stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Git
sudo apt install git  # Ubuntu/Debian
brew install git      # macOS
```

### Installation
```bash
# Clone repository
git clone https://github.com/yourusername/rts-galaxy.git
cd rts-galaxy/RTS_GALAXY

# Build and run (release mode recommended)
cargo run --release

# Development build
cargo run
```

### First Launch
1. **Main Menu**: Use arrow keys or mouse to navigate
2. **Start Game**: Press Enter or click "Start Game"
3. **Tutorial**: Follow on-screen instructions for basic controls
4. **Zoom Navigation**: Use mouse wheel or +/- keys to explore scales

## 🎮 Controls

### Camera & Navigation
| Input | Action |
|-------|--------|
| **WASD** | Move camera |
| **Arrow Keys** | Alternative camera movement |
| **Mouse Wheel** | Zoom in/out through 50 levels |
| **+/-** | Keyboard zoom controls |
| **H/Home** | Return to headquarters |
| **Space** | Center on selected units |

### Unit Management
| Input | Action |
|-------|--------|
| **Left Click** | Select unit/building |
| **Right Click** | Move/attack command |
| **Ctrl+Click** | Add to selection |
| **Shift+Click** | Remove from selection |
| **Drag** | Box select multiple units |
| **Double-Click** | Select all units of same type |

### Building & Production
| Input | Action |
|-------|--------|
| **B** | Open build menu |
| **1** | Train worker (from HQ) |
| **2** | Train fighter (from barracks) |
| **3** | Train ranger (from barracks) |
| **4** | Train tank (from factory) |
| **Tab** | Cycle through buildings |

### Interface
| Input | Action |
|-------|--------|
| **Escape** | Main menu/pause |
| **F1** | Toggle debug information |
| **F11** | Toggle fullscreen |
| **Enter** | Chat (multiplayer) |
| **Ctrl+S** | Quick save |

## 📐 Zoom System Mathematics

### Scale Calculation
```rust
Scale(level) = base_scale × zoom_factor^(level-1)
```

Where:
- `base_scale = 1.0` meter
- `zoom_factor = 3.55` (derived from universe scale)
- `level = 1 to 50`

### Scale Examples
| Level | Scale | Real-World Reference |
|-------|-------|---------------------|
| 1-3 | 1-35m | Individual units, small buildings |
| 7-9 | 1-35km | Cities, large military bases |
| 15 | ~1.4M km | Continental operations |
| 25 | ~600 LY | Stellar neighborhood |
| 35 | ~2.5M LY | Galactic clusters |
| 50 | 93B LY | Observable universe |

## 🏗️ Architecture

### Project Structure
```
src/
├── main.rs              # Entry point and main game loop
├── game/                # Core game logic
│   ├── state.rs         # Game state management
│   ├── zoom.rs          # Multi-scale zoom system
│   ├── rendering.rs     # Rendering pipeline
│   ├── commands.rs      # Game command system
│   └── screens.rs       # Game screen management
├── entity/              # Game entities
│   ├── unit.rs          # Unit implementations
│   ├── building.rs      # Building system
│   ├── player.rs        # Player management
│   └── types.rs         # Core entity types
├── ui/                  # User interface
│   ├── game_ui.rs       # In-game interface
│   └── menu/            # Menu systems
├── resources/           # Asset management
│   ├── manager.rs       # Resource loading
│   └── model3d.rs       # 3D model system
├── network/             # Multiplayer networking
│   ├── client.rs        # Network client
│   └── messages.rs      # Network protocols
├── audio/               # Audio system
│   └── manager.rs       # Audio management
└── ai/                  # AI controllers
    ├── controller.rs    # Main AI logic
    └── behaviors.rs     # AI behavior trees
```

### Core Systems

#### Zoom System
- **Smooth Transitions**: Logarithmic interpolation between levels
- **Performance Optimization**: Automatic LOD switching
- **Context Awareness**: UI adapts to current scale
- **Home Navigation**: Quick return to base operations

#### Entity System
- **Component-Based**: Modular entity composition
- **Animation Integration**: Seamless 3D animation system
- **State Management**: Efficient entity state tracking
- **Collision Detection**: Spatial partitioning for performance

#### Network Architecture
- **Sector-Based**: Hierarchical space partitioning
- **Delta Compression**: Bandwidth-optimized updates
- **Load Balancing**: Dynamic server assignment
- **Fault Tolerance**: Graceful connection handling

## 🎯 Gameplay Guide

### Resource Management
1. **Minerals**: Primary construction resource
   - Gathered by workers from mineral nodes
   - Required for all buildings and most units
   - Storage capacity increases with depots

2. **Energy**: Power and advanced technology
   - Generated by energy plants
   - Required for advanced units and abilities
   - Powers defensive systems

### Unit Types & Roles

#### Worker
- **Role**: Resource gathering and construction
- **Health**: 50 HP
- **Speed**: 80 units/second
- **Special**: Can build all structures
- **Cost**: 50 minerals

#### Fighter
- **Role**: Melee combat specialist
- **Health**: 80 HP
- **Attack**: 25 damage (melee)
- **Range**: 40 units
- **Cost**: 75 minerals

#### Ranger
- **Role**: Ranged combat unit
- **Health**: 60 HP
- **Attack**: 30 damage (ranged)
- **Range**: 80 units
- **Cost**: 100 minerals

#### Tank
- **Role**: Heavy assault vehicle
- **Health**: 150 HP
- **Attack**: 50 damage (explosive)
- **Range**: 50 units
- **Cost**: 200 minerals, 50 energy

### Building Types

#### Headquarters
- **Function**: Main base, trains workers
- **Health**: 500 HP
- **Provides**: 20 population
- **Special**: Game ends if destroyed

#### Barracks
- **Function**: Trains fighters and rangers
- **Health**: 200 HP
- **Cost**: 150 minerals
- **Requirements**: None

#### Factory
- **Function**: Produces tanks and vehicles
- **Health**: 250 HP
- **Cost**: 200 minerals, 100 energy
- **Requirements**: Barracks

#### Defense Turret
- **Function**: Automated base defense
- **Health**: 150 HP
- **Attack**: 35 damage (auto-targeting)
- **Range**: 100 units
- **Cost**: 120 minerals

### Strategic Tips

#### Early Game (Zoom Levels 1-10)
1. Train 3-5 workers immediately
2. Build resource depot near mineral nodes
3. Establish defensive perimeter
4. Scout for enemy positions

#### Mid Game (Zoom Levels 10-25)
1. Expand to secondary resource sites
2. Build balanced military forces
3. Establish forward operating bases
4. Research advanced technologies

#### Late Game (Zoom Levels 25+)
1. Control key strategic locations
2. Mass production of advanced units
3. Coordinate multi-front operations
4. Prepare for galactic expansion

## 🔧 Development

### Building from Source
```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Performance profiling
cargo build --release
perf record ./target/release/rts_galaxy
```

### Adding New Content

#### New Unit Types
1. Define in `src/entity/types.rs`
2. Add stats to `src/entity/unit.rs`
3. Create 3D model in `assets/models/units/`
4. Update AI behavior in `src/ai/behaviors.rs`
5. Add training logic to appropriate building

#### New Buildings
1. Add type to `src/entity/building.rs`
2. Implement construction in `src/game/state.rs`
3. Create model and animations
4. Update UI building menu
5. Add to AI construction priorities

#### New Zoom Levels
The system supports 1-50 levels by default. To extend:
1. Modify `MAX_ZOOM_LEVEL` in `src/game/zoom.rs`
2. Update scale descriptions
3. Adjust LOD thresholds
4. Test performance at extreme scales

### Contributing
1. Fork the repository
2. Create feature branch: `git checkout -b feature-name`
3. Follow Rust coding standards: `cargo fmt`
4. Run tests: `cargo test`
5. Submit pull request with detailed description

### Code Standards
- **Formatting**: Use `cargo fmt` for consistent style
- **Linting**: Run `cargo clippy` and fix warnings
- **Documentation**: Document all public APIs
- **Testing**: Add tests for new functionality
- **Performance**: Profile code for bottlenecks

## 🎵 Assets

### 3D Models
- **Format**: glTF Binary (.glb)
- **Animations**: Separate files per animation
- **Optimization**: LOD variants for different zoom levels
- **Textures**: PBR materials with 1024x1024 resolution

### Audio
- **Effects**: WAV/OGG format, 44.1kHz
- **Music**: Stereo OGG, looped tracks
- **3D Audio**: Distance-based attenuation
- **Dynamic**: Adaptive music based on game state

### UI Textures
- **Resolution**: 1024x1024 for UI panels
- **Format**: PNG with transparency
- **Scaling**: Vector-based where possible
- **Accessibility**: High contrast for visibility

## 📊 Performance

### System Requirements

#### Minimum
- **OS**: Windows 10, macOS 10.14, or Linux
- **CPU**: Dual-core 2.5GHz
- **RAM**: 4GB
- **GPU**: DirectX 11 compatible
- **Storage**: 2GB available space

#### Recommended
- **OS**: Latest OS version
- **CPU**: Quad-core 3.0GHz+
- **RAM**: 8GB+
- **GPU**: Dedicated graphics card
- **Storage**: 4GB+ (SSD recommended)

### Performance Features
- **LOD System**: Automatic quality adjustment
- **Frustum Culling**: Only render visible objects
- **Batch Rendering**: Efficient draw calls
- **Asset Streaming**: Load on demand
- **Multi-threading**: Parallel processing where possible

### Optimization Tips
- Lower zoom levels require more detail processing
- Reduce unit count for better performance
- Use medium quality settings on older hardware
- Close other applications for maximum performance

## 🌐 Multiplayer

### Network Features
- **Player Capacity**: Up to 8 players
- **Latency Compensation**: Client-side prediction
- **Anti-Cheat**: Server-side validation
- **Reconnection**: Automatic connection recovery

### Game Modes
- **Skirmish**: Free-for-all combat
- **Team Battle**: Coordinated team strategy
- **Cooperative**: Players vs AI
- **Custom**: User-defined scenarios

### Hosting Games
```bash
# Start dedicated server
cargo run --bin server --release

# Connect to server
cargo run --release -- --connect 192.168.1.100:8080
```

## 🔮 Roadmap

### Version 1.0 (Current)
- [x] Core RTS gameplay
- [x] 50-level zoom system
- [x] Single-player AI
- [x] Basic multiplayer
- [ ] Campaign mode
- [ ] Map editor

### Version 1.1 (Planned)
- [ ] Advanced AI behaviors
- [ ] More unit types (air units, naval)
- [ ] Technology research system
- [ ] Custom maps and scenarios
- [ ] Mod support framework

### Version 2.0 (Future)
- [ ] VR compatibility
- [ ] Procedural galaxy generation
- [ ] Massive multiplayer (16+ players)
- [ ] Cross-platform mobile support
- [ ] Steam Workshop integration

## 🐛 Troubleshooting

### Common Issues

**Game won't start:**
```bash
# Check Rust installation
rustc --version

# Update to latest stable
rustup update stable

# Clean rebuild
cargo clean && cargo build --release
```

**Poor performance:**
- Lower graphics settings in main menu
- Reduce maximum unit count
- Close background applications
- Update graphics drivers

**Network connection issues:**
- Check firewall settings
- Ensure port 8080 is open
- Verify server address
- Test with local network first

**Audio problems:**
- Check system audio settings
- Verify audio device selection
- Update audio drivers
- Test with different audio format

### Debug Mode
```bash
# Enable debug mode for detailed logging
RUST_LOG=debug cargo run --features debug
```

### Getting Help
- **Discord**: [RTS Galaxy Community](https://discord.gg/rtsgalaxy)
- **GitHub Issues**: Report bugs and request features
- **Wiki**: Comprehensive documentation and guides
- **Forums**: Community discussions and strategies

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### Third-Party Licenses
- **macroquad**: MIT License
- **serde**: MIT OR Apache-2.0
- **tokio**: MIT License

## 🙏 Acknowledgments

- **macroquad team**: Excellent Rust game engine
- **Rust community**: Language and ecosystem support
- **Contributors**: All community contributors
- **Testers**: Alpha and beta testing participants
- **Artists**: 3D models and audio assets

---

**Ready to command the galaxy? Download RTS Galaxy today!**

*From commanding individual soldiers to orchestrating galactic empires, every scale of strategy awaits.*
