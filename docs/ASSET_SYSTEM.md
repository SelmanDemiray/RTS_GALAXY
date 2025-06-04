# RTS Galaxy Asset System Documentation

The RTS Galaxy asset system is designed to handle complex 3D models, animations, textures, and audio assets efficiently across all zoom levels from individual units to cosmic structures.

## Asset Organization Philosophy

### Modular Animation System
Each unit and building has its own folder containing:
- **Main Model**: The base 3D model file (e.g., `worker.glb`)
- **Animation Folder**: Individual animation files in separate `.glb` files
- **Texture Assets**: Organized by resolution and purpose

### Benefits of Separation
1. **Independent Updates**: Modify animations without affecting base models
2. **Memory Efficiency**: Load only needed animations for current game state
3. **Version Control**: Track changes to individual components
4. **Collaboration**: Multiple artists can work on different animations
5. **Performance**: Stream assets based on zoom level and visibility

## Directory Structure

```
assets/
├── models/
│   ├── units/
│   │   └── [unit_name]/
│   │       ├── [unit_name].glb              # Base model
│   │       └── animations/
│   │           ├── idle.glb                 # Animation files
│   │           ├── walking.glb
│   │           └── ...
│   ├── buildings/
│   │   └── [building_name]/
│   │       ├── [building_name].glb          # Base model
│   │       └── animations/
│   │           ├── idle.glb
│   │           ├── construction.glb
│   │           └── ...
│   ├── resources/
│   ├── effects/
│   └── terrain/
├── textures/
│   ├── ui/
│   │   ├── buttons/
│   │   ├── panels/
│   │   └── icons/
│   └── environment/
├── sounds/
│   ├── units/
│   ├── buildings/
│   ├── ui/
│   └── game/
├── music/
└── fonts/
```

## Asset Manifest System

### Manifest Structure
The `asset_manifest.json` file defines all assets and their properties:

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

### Animation Metadata
Each animation entry contains:
- **name**: Unique identifier for the animation
- **file**: Path to the animation .glb file
- **duration**: Length of animation in seconds
- **loop**: Whether animation should repeat
- **speed**: Playback speed multiplier

## 3D Model Specifications

### Technical Requirements

#### File Format
- **Primary Format**: glTF Binary (.glb)
- **Fallback**: glTF (.gltf) with external assets
- **Creation Tool**: Blender 2.8+ recommended

#### Polygon Limits
```rust
pub enum ModelComplexity {
    Simple,    // 100-500 triangles   (distant view)
    Medium,    // 500-2000 triangles  (normal view)
    Detailed,  // 2000-8000 triangles (close view)
    HighDetail,// 8000+ triangles     (cinematic)
}
```

#### Texture Specifications
- **Resolution**: 1024×1024 for main models, 512×512 for details
- **Format**: PNG or JPG embedded in glTF
- **Maps Required**:
  - Diffuse (base color)
  - Normal (surface detail)
  - Metallic-Roughness (material properties)
  - Emissive (self-illumination, optional)

#### Model Origins and Scale
- **Units**: Origin at base/feet, 1 Blender unit = 1 meter
- **Buildings**: Origin at foundation center
- **Effects**: Origin at effect center
- **Human Scale Reference**: Worker ~1.8 meters tall

### Level-of-Detail (LOD) System

#### Automatic LOD Selection
```rust
impl Model3D {
    pub fn get_appropriate_lod(&self, distance: f32, zoom_level: i32) -> ModelLOD {
        match (distance, zoom_level) {
            (d, z) if d < 50.0 && z <= 5 => ModelLOD::High,
            (d, z) if d < 200.0 && z <= 15 => ModelLOD::Medium,
            (d, z) if d < 1000.0 && z <= 30 => ModelLOD::Low,
            _ => ModelLOD::Icon,
        }
    }
}
```

#### LOD Variants
Each major model can have multiple detail levels:
- `model_high.glb` - Full detail for close viewing
- `model_medium.glb` - Simplified for tactical view
- `model_low.glb` - Basic shape for strategic view
- `model_icon.glb` - Simple representation for cosmic view

## Animation System

### Animation Categories

#### Unit Animations

##### Movement Animations
- **idle**: Default standing/hovering pose
- **walking**: Normal speed movement
- **running**: Fast movement for combat units
- **turning**: Rotation animations for large units

##### Work Animations
- **gathering_minerals**: Mining and collection
- **gathering_energy**: Energy harvesting
- **building**: Construction work
- **carrying_resources**: Moving with cargo

##### Combat Animations
- **attacking**: Varies by unit type:
  - `melee_attack` for fighters
  - `shooting` for rangers
  - `firing_cannon` for tanks
- **blocking**: Defensive poses
- **dying**: Death sequences
- **victory_pose**: Celebration animations

##### Special Animations
- **reloading**: Weapon maintenance
- **special_ability**: Unique unit abilities
- **damaged_idle**: Wounded state animations

#### Building Animations

##### Construction Phases
- **foundation**: Initial building placement
- **framework**: Structural assembly
- **completion**: Finishing details
- **operational**: Normal working state

##### Operational Animations
- **idle**: Normal functioning state
- **working**: Active production/operation
- **power_up**: Activation sequences
- **power_down**: Deactivation sequences

##### Production Animations
- **training**: Unit production in barracks
- **manufacturing**: Vehicle assembly
- **research**: Technology development
- **resource_processing**: Material refinement

##### Combat Animations (Defense Buildings)
- **scanning**: Target acquisition
- **targeting**: Weapon aiming
- **firing**: Weapon discharge
- **reloading**: Ammunition replacement
- **turret_rotate**: Weapon positioning

##### Damage States
- **damaged**: Partially destroyed appearance
- **critical**: Severely damaged state
- **destroyed**: Final destruction sequence
- **repair**: Restoration animations

### Animation Implementation

#### Animation State Machine
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum UnitAnimationState {
    Idle,
    Walking,
    Running,
    Attacking,
    Gathering,
    Building,
    Dying,
    Special,
}

pub struct UnitAnimation {
    pub current_state: UnitAnimationState,
    pub animation_time: f32,
    pub animation_speed: f32,
    pub loop_animation: bool,
    pub transition_time: f32,
    pub next_state: Option<UnitAnimationState>,
}
```

#### Animation Blending
```rust
impl UnitAnimation {
    pub fn blend_to_state(&mut self, new_state: UnitAnimationState, blend_time: f32) {
        self.next_state = Some(new_state);
        self.transition_time = blend_time;
    }
    
    pub fn update(&mut self, dt: f32) {
        if let Some(next_state) = &self.next_state {
            self.transition_time -= dt;
            if self.transition_time <= 0.0 {
                self.current_state = next_state.clone();
                self.next_state = None;
                self.animation_time = 0.0;
            }
        }
        
        self.animation_time += dt * self.animation_speed;
    }
}
```

## Resource Management System

### Asset Loading Strategy

#### Lazy Loading
```rust
pub struct ResourceManager {
    loaded_models: HashMap<String, Model3D>,
    loading_queue: VecDeque<AssetRequest>,
    cache_size_limit: usize,
    current_cache_size: usize,
}

impl ResourceManager {
    pub async fn load_asset_async(&mut self, asset_name: &str) -> Result<(), LoadError> {
        if !self.loaded_models.contains_key(asset_name) {
            let model = self.load_model_from_file(asset_name).await?;
            self.cache_model(asset_name.to_string(), model);
        }
        Ok(())
    }
}
```

#### Streaming System
- **Predictive Loading**: Load assets before they're needed
- **Distance-Based**: Load higher detail as camera approaches
- **Zoom-Based**: Load appropriate detail level for current zoom
- **Memory Management**: Unload distant or unused assets

#### Cache Management
```rust
pub enum CacheStrategy {
    LRU,           // Least Recently Used
    Distance,      // Based on distance from camera
    Importance,    // Based on asset importance
    Frequency,     // Based on usage frequency
}
```

### Memory Optimization

#### Asset Compression
- **Texture Compression**: DXT/BC compression for GPU efficiency
- **Mesh Compression**: Quantized vertex data
- **Animation Compression**: Keyframe reduction

#### Instancing System
```rust
pub struct InstancedModel {
    pub base_model: Model3D,
    pub instances: Vec<ModelInstance>,
    pub instance_buffer: Buffer,
}

pub struct ModelInstance {
    pub transform: Mat4,
    pub color_tint: Vec4,
    pub animation_frame: f32,
}
```

## Audio Asset System

### Audio Categories

#### Sound Effects
```rust
pub struct SoundInfo {
    pub name: String,
    pub file: String,
    pub volume: f32,
    pub pitch_variation: f32,
    pub spatial: bool,  // 3D positioned sound
}
```

#### Music System
```rust
pub struct MusicTrack {
    pub name: String,
    pub file: String,
    pub loop_start: f32,
    pub loop_end: f32,
    pub fade_in_time: f32,
    pub fade_out_time: f32,
}
```

#### Dynamic Audio
- **Adaptive Music**: Changes based on game state
- **3D Spatial Audio**: Positioned sounds in game world
- **Environmental Effects**: Reverb and echo based on location

### Audio Loading and Playback

#### Streaming Audio
```rust
pub struct AudioManager {
    sound_cache: HashMap<String, Sound>,
    music_player: MusicPlayer,
    spatial_mixer: SpatialAudioMixer,
}

impl AudioManager {
    pub fn play_3d_sound(&mut self, sound_name: &str, position: Vec3, volume: f32) {
        if let Some(sound) = self.sound_cache.get(sound_name) {
            self.spatial_mixer.play_at_position(sound, position, volume);
        }
    }
}
```

## UI Asset Management

### Texture Organization
```
textures/ui/
├── buttons/
│   ├── button_normal.png
│   ├── button_hover.png
│   └── button_pressed.png
├── panels/
│   ├── panel_background.png
│   └── panel_frame.png
├── icons/
│   ├── unit_icons/
│   ├── building_icons/
│   └── resource_icons/
└── hud/
    ├── minimap_frame.png
    ├── health_bar.png
    └── selection_circle.png
```

### Scalable UI Assets
```rust
pub struct UITexture {
    pub texture: Texture2D,
    pub nine_patch: Option<NinePatch>,
    pub scaling_mode: ScalingMode,
}

pub enum ScalingMode {
    Stretch,
    NinePatch,
    Tile,
    KeepAspect,
}
```

## Performance Considerations

### Loading Performance
- **Async Loading**: Non-blocking asset loading
- **Progressive Loading**: Load base models first, details later
- **Batch Loading**: Load related assets together

### Runtime Performance
- **Asset Pooling**: Reuse common assets
- **GPU Memory Management**: Efficient texture and mesh uploading
- **Culling Systems**: Don't process invisible assets

### Platform Optimization
- **Mobile Considerations**: Lower resolution assets for mobile devices
- **Console Optimization**: Platform-specific asset formats
- **PC Flexibility**: Configurable quality settings

## Development Workflow

### Asset Creation Pipeline
1. **Concept Art**: Design and style guides
2. **3D Modeling**: Create base models in Blender
3. **Animation**: Create individual animation files
4. **Texturing**: Create and optimize textures
5. **Export**: Export to glTF format
6. **Integration**: Add to asset manifest
7. **Testing**: Verify in-game appearance and performance

### Version Control
- **Asset Versioning**: Track changes to individual assets
- **Manifest Updates**: Version the asset manifest file
- **Backwards Compatibility**: Handle asset format changes

### Quality Assurance
- **Automated Testing**: Verify asset loading and display
- **Performance Testing**: Check memory usage and frame rates
- **Visual Validation**: Ensure assets display correctly

## Future Enhancements

### Planned Features
1. **Procedural Assets**: Generate detail on-demand
2. **AI-Enhanced LOD**: Machine learning for optimal detail levels
3. **Real-time Asset Streaming**: Stream assets from cloud
4. **User-Generated Content**: Tools for community asset creation

### Technical Improvements
1. **Advanced Compression**: Better compression algorithms
2. **GPU-Accelerated Loading**: Use GPU for asset processing
3. **Predictive Caching**: AI-driven asset pre-loading
4. **Cross-Platform Assets**: Unified assets across all platforms

---

*The RTS Galaxy asset system provides the foundation for rich, detailed 3D content that scales seamlessly from intimate unit details to vast cosmic structures.*
