# Asset System Documentation

## Overview

The Galaxy RTS asset system is designed for modular, efficient loading and management of 3D models, animations, textures, sounds, and fonts. The system supports individual animation files, manifest-driven configuration, and runtime optimization.

## Core Components

### ResourceManager
The `ResourceManager` is the central hub for all asset operations:

```rust
// Loading assets
let mut resource_manager = ResourceManager::new();
resource_manager.load_resources().await;

// Accessing assets
let worker_model = resource_manager.get_model("worker");
let click_sound = resource_manager.get_sound("button_click");
let ui_texture = resource_manager.get_texture("button_normal");
```

### Asset Manifest System
All assets are defined in `assets/asset_manifest.json`:

```json
{
  "models": {
    "units": [
      {
        "name": "worker",
        "file": "models/units/worker/worker.glb",
        "scale": 1.0,
        "default_animation": "idle",
        "animations": [
          {
            "name": "idle",
            "file": "models/units/worker/animations/idle.glb",
            "duration": 2.0,
            "loop": true,
            "speed": 1.0
          }
        ]
      }
    ]
  }
}
```

## Animation System

### Individual Animation Files
Each animation is stored as a separate `.glb` file:

```
worker/
├── worker.glb              # Base model
└── animations/
    ├── idle.glb           # Individual animations
    ├── walking.glb
    ├── gathering_minerals.glb
    └── building.glb
```

### Benefits
1. **Modularity**: Update animations independently
2. **Memory Efficiency**: Load only needed animations
3. **Version Control**: Track individual animation changes
4. **Artist Workflow**: Multiple artists can work simultaneously

### Animation Loading
```rust
// Check if animation exists
if model.has_animation("walking") {
    // Play animation
    model.draw_with_animation(
        position,
        rotation,
        scale,
        "walking",
        animation_time
    );
}
```

## Asset Categories

### 3D Models
- **Units**: Character models with complete animation sets
- **Buildings**: Structures with construction and operational animations
- **Resources**: Harvestable resource nodes with ambient animations
- **Effects**: Visual effects like explosions and particles

### Textures
- **UI Elements**: Buttons, panels, icons organized by function
- **Materials**: PBR textures for 3D models
- **Effects**: Particle and special effect textures

### Audio
- **Sound Effects**: Categorized by usage (UI, units, buildings, game)
- **Music**: Background tracks for different game states
- **Voice**: Unit acknowledgments and narrator audio

### Fonts
- **UI Fonts**: Interface text rendering
- **Title Fonts**: Large display text
- **Special Fonts**: Decorative or thematic typography

## File Organization

### Directory Structure
```
assets/
├── models/
│   ├── units/
│   │   ├── worker/
│   │   │   ├── worker.glb
│   │   │   └── animations/
│   │   │       ├── idle.glb
│   │   │       ├── walking.glb
│   │   │       └── ...
│   │   └── ...
│   ├── buildings/
│   ├── resources/
│   └── effects/
├── textures/
│   └── ui/
│       ├── buttons/
│       ├── panels/
│       └── ...
├── sounds/
│   ├── units/
│   ├── buildings/
│   ├── ui/
│   └── game/
├── music/
├── fonts/
└── asset_manifest.json
```

### Naming Conventions
- **Files**: lowercase with underscores (`worker_idle.glb`)
- **Assets**: descriptive names matching usage (`button_click`)
- **Directories**: singular nouns (`model/`, `texture/`)

## Model Requirements

### Technical Specifications
- **Format**: glTF Binary (.glb) format only
- **Polygon Count**:
  - Units: 500-2000 triangles
  - Buildings: 1000-5000 triangles
  - Effects: 200-1000 triangles
- **Textures**: 1024×1024 max, embedded in glTF
- **Scale**: 1 Blender unit = 1 game meter
- **Origin**: Centered at base for units and buildings

### Animation Requirements
- **Frame Rate**: 30fps for all animations
- **Duration**: As specified in manifest
- **Looping**: Seamless loops where appropriate
- **Naming**: Match manifest animation names exactly

## Integration with Game Systems

### Rendering Pipeline
```rust
// In game rendering loop
for unit in &game_state.units {
    if let Some(model) = resource_manager.get_model(&unit.model_name) {
        let animation_name = unit.get_current_animation();
        model.draw_with_animation(
            unit.position,
            unit.rotation,
            unit.scale,
            animation_name,
            unit.animation_time
        );
    }
}
```

### Audio Integration
```rust
// Playing context-aware audio
audio_manager.play_selection_sound(&resource_manager, &game_state);
audio_manager.play_music("battle_theme", &resource_manager, &game_state);
```

### UI Textures
```rust
// Accessing UI textures
if let Some(button_texture) = resource_manager.get_texture("button_normal") {
    draw_texture(button_texture, x, y, WHITE);
}
```

## Performance Optimization

### Loading Strategy
- **Async Loading**: Non-blocking asset loading during initialization
- **Priority Loading**: Critical assets loaded first
- **Lazy Loading**: Optional assets loaded on demand
- **Memory Management**: Automatic cleanup of unused assets

### Runtime Optimization
- **Animation Caching**: Frequently used animations kept in memory
- **Level-of-Detail**: Different models for different zoom levels
- **Culling**: Only render visible objects
- **Batching**: Group similar draw calls

## Development Workflow

### Adding New Assets

1. **Create Asset Files**:
   ```bash
   # Place model files
   assets/models/units/new_unit/new_unit.glb
   assets/models/units/new_unit/animations/idle.glb
   ```

2. **Update Manifest**:
   ```json
   {
     "name": "new_unit",
     "file": "models/units/new_unit/new_unit.glb",
     "animations": [
       {
         "name": "idle",
         "file": "models/units/new_unit/animations/idle.glb",
         "duration": 2.0,
         "loop": true
       }
     ]
   }
   ```

3. **Reference in Code**:
   ```rust
   let model = resource_manager.get_model("new_unit");
   ```

### Asset Hot-Reloading (Development)
```rust
// Enable hot-reloading in debug builds
#[cfg(debug_assertions)]
resource_manager.enable_hot_reload();
```

### Asset Validation
```bash
# Validate asset manifest
cargo run --bin validate_assets

# Check for missing files
cargo run --bin check_assets
```

## Troubleshooting

### Common Issues

**Model not loading:**
- Check file path in manifest
- Verify .glb file format
- Ensure file permissions are correct

**Animation not playing:**
- Verify animation name matches manifest
- Check animation duration and loop settings
- Ensure animation file exists

**Texture not displaying:**
- Check texture format (PNG/JPG)
- Verify texture is embedded in glTF
- Check UV mapping in 3D software

**Audio not playing:**
- Verify audio file format (.ogg/.wav)
- Check audio file path in manifest
- Ensure system audio is working

### Performance Issues

**Slow loading:**
- Reduce texture sizes
- Optimize polygon counts
- Use compressed audio formats

**High memory usage:**
- Enable asset cleanup
- Reduce number of loaded animations
- Use texture streaming

### Asset Creation Guidelines

**For Artists:**
- Use Blender 2.8+ for model creation
- Export with embedded textures
- Test models in game before finalizing
- Follow naming conventions strictly

**For Developers:**
- Update manifest for all new assets
- Test asset loading in development builds
- Validate file formats before committing
- Document any new asset requirements

## Future Enhancements

### Planned Features
- **Asset Streaming**: Dynamic loading based on game area
- **Compression**: Advanced compression for smaller file sizes
- **Procedural Assets**: Runtime generation of certain assets
- **Asset Bundles**: Grouped loading for related assets

### Optimization Goals
- **Faster Loading**: Parallel loading of multiple assets
- **Lower Memory**: More efficient memory management
- **Better Quality**: Higher resolution assets with LOD systems
- **Cross-Platform**: Optimized assets for different platforms
