# Galaxy-Scale Zoom System Documentation

## Overview

The Galaxy RTS features a unique 50-level zoom system that allows seamless navigation from individual units (1 meter scale) to the entire observable universe (93 billion light-years). This system is mathematically accurate and provides smooth transitions between scales.

## Mathematical Foundation

### Scale Calculation
The zoom system uses a logarithmic progression:

```
Scale(level) = base_scale × zoom_factor^(level-1)
```

Where:
- `base_scale = 1.0` (1 meter at level 1)
- `zoom_factor ≈ 3.55` (calculated from universe diameter)
- `max_level = 50`

### Zoom Factor Derivation
```
zoom_factor = (D_universe / base_scale)^(1/(max_level-1))
zoom_factor = (8.8×10²⁶ / 1.0)^(1/49) ≈ 3.55
```

## Zoom Levels

### Level Categories

| Levels | Scale Range | Description | Use Cases |
|--------|-------------|-------------|-----------|
| 1-3    | 1m - 35m    | Unit Detail | Individual unit management |
| 4-6    | 35m - 450m  | Small Village | Local base building |
| 7-9    | 450m - 5.7km | Town/City | Regional overview |
| 10-12  | 5.7km - 72km | Region | Continental strategy |
| 13-15  | 72km - 910km | Continent | Planetary politics |
| 16-18  | 910km - 11.5Mkm | Planet Surface | System overview |
| 19-21  | 11.5Mkm - 145Mkm | Planetary System | Local star management |
| 22-24  | 145Mkm - 1.8Gkm | Solar Neighborhood | Regional space control |
| 25-27  | 1.8Gkm - 23GLY | Nearby Stars | Stellar politics |
| 28-30  | 23GLY - 290GLY | Local Cluster | Galactic regions |
| 31-33  | 290GLY - 3.7kLY | Spiral Arm | Galaxy management |
| 34-36  | 3.7kLY - 47kLY | Galaxy | Intergalactic strategy |
| 37-39  | 47kLY - 590kLY | Local Group | Cluster politics |
| 40-42  | 590kLY - 7.4MLY | Supercluster | Cosmic regions |
| 43-49  | 7.4MLY - 74GLY | Cosmic Web | Universal strategy |
| 50     | 93GLY | Observable Universe | Complete overview |

## Implementation

### ZoomSystem Structure
```rust
pub struct ZoomSystem {
    pub current_level: i32,
    pub target_level: i32,
    pub interpolation_progress: f32,
    pub home_position: Vec2,
    pub base_scale: f64,
    pub zoom_factor: f64,
    pub max_level: i32,
}
```

### Core Methods

#### Zoom Navigation
```rust
// Zoom in one level
zoom_system.zoom_in();

// Zoom out one level
zoom_system.zoom_out();

// Jump to specific level
zoom_system.set_zoom_level(25);

// Return to headquarters
let home_pos = zoom_system.go_home();
```

#### Scale Calculation
```rust
// Get scale for any level
let scale = zoom_system.get_scale_for_level(level);

// Get current interpolated scale
let current_scale = zoom_system.get_current_scale();
```

#### Information Display
```rust
// Get human-readable level name
let label = zoom_system.get_zoom_label();
// "Level 15: Continent"

// Get scale description
let description = zoom_system.get_zoom_description();
// "~910 km - Continental scale"
```

## User Interface Integration

### Controls
- **Mouse Wheel**: Scroll to zoom in/out
- **+/- Keys**: Keyboard zoom controls
- **Keypad +/-**: Alternative zoom controls
- **H/Home Key**: Return to headquarters at optimal level

### Visual Feedback
```rust
// Display current zoom information
let zoom_info = format!(
    "{}\n{}",
    game_state.zoom_system.get_zoom_label(),
    game_state.zoom_system.get_zoom_description()
);
draw_text(&zoom_info, 10.0, screen_height() - 60.0, 16.0, WHITE);
```

### Camera Integration
```rust
// Adjust camera movement speed based on zoom level
let zoom_scale = zoom_system.get_current_scale() as f32;
let camera_speed = base_speed * (zoom_scale / 1000.0).clamp(0.1, 50.0);
```

## Smooth Transitions

### Interpolation System
The system provides smooth transitions between zoom levels:

```rust
pub fn update(&mut self, dt: f32) {
    if self.interpolation_progress < 1.0 {
        self.interpolation_progress += self.interpolation_speed * dt;
        if self.interpolation_progress >= 1.0 {
            self.current_level = self.target_level;
        }
    }
}
```

### Logarithmic Interpolation
Scale interpolation uses logarithmic blending for natural feel:

```rust
let log_current = current_scale.ln();
let log_target = target_scale.ln();
let log_interpolated = log_current + (log_target - log_current) * progress;
let interpolated_scale = log_interpolated.exp();
```

## Performance Considerations

### Level-of-Detail (LOD)
Different zoom levels can trigger different rendering modes:

```rust
match zoom_system.current_level {
    1..=5 => render_high_detail_units(),
    6..=15 => render_medium_detail(),
    16..=30 => render_strategic_view(),
    31..=50 => render_galactic_overview(),
}
```

### Culling Optimization
Objects outside zoom-appropriate scales are culled:

```rust
let current_scale = zoom_system.get_current_scale();
let object_size = calculate_object_size();

// Only render if object is visible at current scale
if object_size > current_scale * 0.1 && object_size < current_scale * 10.0 {
    render_object();
}
```

## Gameplay Integration

### Strategic Levels
Different zoom levels enable different strategic decisions:

- **Levels 1-6**: Tactical unit micromanagement
- **Levels 7-12**: Base construction and resource management
- **Levels 13-18**: Regional expansion and defense
- **Levels 19-30**: Interplanetary colonization
- **Levels 31-50**: Galactic empire management

### Home Position
The system tracks your headquarters position:

```rust
// Set home when HQ is built
zoom_system.set_home_position(Vec2::new(hq_x, hq_y));

// Return home with optimal zoom
let home_position = zoom_system.go_home(); // Sets level 8
camera.position = home_position;
```

## Advanced Features

### Contextual Zoom
Automatic zoom adjustment based on selection:

```rust
fn auto_zoom_to_selection(zoom_system: &mut ZoomSystem, selection: &[Unit]) {
    let optimal_level = calculate_optimal_zoom_for_units(selection);
    zoom_system.set_zoom_level(optimal_level);
}
```

### Bookmark System (Future)
Save interesting zoom positions:

```rust
// Future feature
struct ZoomBookmark {
    name: String,
    level: i32,
    position: Vec2,
}
```

### Mini-Map Integration
Show current zoom level on minimap:

```rust
fn draw_zoom_indicator(zoom_system: &ZoomSystem, minimap_rect: Rect) {
    let level_height = minimap_rect.h / 50.0;
    let indicator_y = minimap_rect.y + (zoom_system.current_level as f32) * level_height;
    draw_rectangle(minimap_rect.x, indicator_y, minimap_rect.w, 2.0, YELLOW);
}
```

## Configuration

### Customizable Parameters
```rust
impl ZoomSystem {
    pub fn with_custom_params(
        base_scale: f64,
        max_level: i32,
        universe_diameter: f64
    ) -> Self {
        let zoom_factor = (universe_diameter / base_scale).powf(1.0 / (max_level - 1) as f64);
        // ... initialize with custom parameters
    }
}
```

### Speed Settings
```rust
// Adjust interpolation speed
zoom_system.interpolation_speed = 2.0; // Faster transitions
zoom_system.interpolation_speed = 0.5; // Slower transitions
```

## Scientific Accuracy

### Real-World Scales
The zoom levels correspond to real astronomical and terrestrial scales:

- **Levels 1-15**: Earth-based scales from human to continental
- **Levels 16-18**: Planetary scales (Earth diameter ≈ 12,742 km)
- **Levels 19-21**: Solar system (Neptune orbit ≈ 9 billion km)
- **Levels 22-30**: Local stellar neighborhood
- **Levels 31-36**: Milky Way galaxy (≈ 100,000 light-years)
- **Levels 37-50**: Intergalactic and cosmic scales

### Educational Value
The system can serve as an educational tool for understanding cosmic scales and the vast differences between microscopic and universal measurements.

## Troubleshooting

### Common Issues

**Zoom feels too fast/slow:**
```rust
// Adjust interpolation speed
zoom_system.interpolation_speed = 1.5; // Adjust as needed
```

**Camera movement too sensitive at high zoom:**
```rust
// Implement zoom-appropriate camera speed
let adjusted_speed = base_speed * zoom_scale_factor.clamp(0.1, 10.0);
```

**Level descriptions unclear:**
```rust
// Customize level descriptions
match level {
    1..=3 => "Tactical View",
    4..=9 => "Base Management",
    // ... customize as needed
}
```

### Performance Tips
- Use LOD systems for different zoom levels
- Implement frustum culling based on zoom scale
- Cache scale calculations for frequently accessed levels
- Consider using different rendering pipelines for extreme scales

## Future Enhancements

### Planned Features
- **Smooth LOD Transitions**: Gradual quality changes during zoom
- **Zoom History**: Navigate back through zoom sequence
- **Contextual UI**: Different interfaces for different scales
- **Procedural Detail**: Generate appropriate detail for each scale
- **VR Integration**: 3D zoom navigation in virtual reality

The galaxy-scale zoom system provides an unparalleled strategic gaming experience, allowing players to command everything from individual soldiers to galactic empires with seamless navigation between all scales of existence.
