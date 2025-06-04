# Advanced 3D Model System for RTS Galaxy

A comprehensive 3D asset pipeline for real-time strategy gameplay with advanced animation systems, level-of-detail optimization, and cosmic-scale rendering support.

## Table of Contents
- [Directory Structure](#directory-structure)
- [Technical Specifications](#technical-specifications)
- [Animation System](#animation-system)
- [Level of Detail (LOD) System](#level-of-detail-lod-system)
- [Cosmic Scale Requirements](#cosmic-scale-requirements)
- [Asset Pipeline](#asset-pipeline)
- [Quality Standards](#quality-standards)
- [Performance Guidelines](#performance-guidelines)
- [Integration with Game Systems](#integration-with-game-systems)

## Directory Structure

```
models/
├── units/                      # Combat and utility units
│   ├── worker/                 # Resource gathering unit
│   │   ├── worker.glb          # Base model (LOD 0)
│   │   ├── worker_lod1.glb     # Medium detail
│   │   ├── worker_lod2.glb     # Low detail
│   │   ├── worker_icon.glb     # Icon representation
│   │   └── animations/         # Individual animation files
│   │       ├── idle.glb        # Base idle animation
│   │       ├── walking.glb     # Standard movement
│   │       ├── running.glb     # Fast movement
│   │       ├── gathering_minerals.glb
│   │       ├── gathering_energy.glb
│   │       ├── gathering_exotic.glb    # Advanced resources
│   │       ├── building.glb    # Construction work
│   │       ├── building_large.glb      # Large structure construction
│   │       ├── carrying_resources.glb
│   │       ├── carrying_heavy.glb      # Heavy resource transport
│   │       ├── repairing.glb   # Unit/building repair
│   │       ├── upgrading.glb   # Structure enhancement
│   │       ├── celebration.glb # Victory/completion
│   │       ├── damaged.glb     # Injured state
│   │       └── dying.glb       # Death sequence
│   ├── fighter/                # Melee combat unit
│   │   ├── fighter.glb
│   │   ├── fighter_lod1.glb
│   │   ├── fighter_lod2.glb
│   │   ├── fighter_icon.glb
│   │   └── animations/
│   │       ├── idle.glb
│   │       ├── idle_alert.glb  # Combat ready stance
│   │       ├── walking.glb
│   │       ├── running.glb
│   │       ├── charging.glb    # Sprint attack
│   │       ├── melee_attack.glb
│   │       ├── combo_attack.glb        # Multi-hit sequence
│   │       ├── power_attack.glb        # Heavy strike
│   │       ├── blocking.glb
│   │       ├── dodge_left.glb
│   │       ├── dodge_right.glb
│   │       ├── parry.glb       # Counter-attack
│   │       ├── stunned.glb     # Incapacitated state
│   │       ├── victory_pose.glb
│   │       ├── retreat.glb     # Tactical withdrawal
│   │       ├── formation_march.glb     # Group movement
│   │       └── dying.glb
│   ├── ranger/                 # Ranged combat unit
│   │   ├── ranger.glb
│   │   ├── ranger_lod1.glb
│   │   ├── ranger_lod2.glb
│   │   ├── ranger_icon.glb
│   │   └── animations/
│   │       ├── idle.glb
│   │       ├── idle_scanning.glb       # Surveillance mode
│   │       ├── walking.glb
│   │       ├── running.glb
│   │       ├── crouching.glb   # Stealth position
│   │       ├── aiming.glb
│   │       ├── aiming_precision.glb    # Sniper mode
│   │       ├── shooting.glb
│   │       ├── rapid_fire.glb  # Burst mode
│   │       ├── grenade_throw.glb       # Explosive attack
│   │       ├── reloading.glb
│   │       ├── reloading_fast.glb      # Quick reload
│   │       ├── taking_cover.glb        # Defensive stance
│   │       ├── overwatch.glb   # Area monitoring
│   │       ├── spotting.glb    # Target identification
│   │       └── dying.glb
│   ├── tank/                   # Heavy armored unit
│   │   ├── tank.glb
│   │   ├── tank_lod1.glb
│   │   ├── tank_lod2.glb
│   │   ├── tank_icon.glb
│   │   └── animations/
│   │       ├── idle.glb
│   │       ├── idle_powered.glb        # Engine running
│   │       ├── moving_slow.glb         # Careful advance
│   │       ├── moving_fast.glb         # Full speed
│   │       ├── reversing.glb   # Retreat movement
│   │       ├── turret_rotate_left.glb
│   │       ├── turret_rotate_right.glb
│   │       ├── elevation_up.glb        # Cannon adjustment
│   │       ├── elevation_down.glb
│   │       ├── firing_main.glb         # Primary weapon
│   │       ├── firing_secondary.glb    # Machine gun
│   │       ├── firing_special.glb      # Artillery mode
│   │       ├── reloading_main.glb
│   │       ├── smoke_deploy.glb        # Defensive screen
│   │       ├── damaged_idle.glb
│   │       ├── damaged_smoking.glb     # Heavy damage
│   │       ├── emergency_repair.glb    # Field maintenance
│   │       └── dying.glb
│   ├── aircraft/               # Aerial units
│   │   ├── interceptor/        # Fast fighter
│   │   ├── bomber/             # Heavy assault
│   │   └── transport/          # Unit carrier
│   └── naval/                  # Water-based units
│       ├── destroyer/          # Combat vessel
│       ├── carrier/            # Aircraft platform
│       └── submarine/          # Stealth unit
├── buildings/                  # Structures and installations
│   ├── headquarters/           # Command center
│   │   ├── headquarters.glb
│   │   ├── headquarters_lod1.glb
│   │   ├── headquarters_lod2.glb
│   │   ├── headquarters_icon.glb
│   │   └── animations/
│   │       ├── idle.glb
│   │       ├── idle_night.glb  # Evening illumination
│   │       ├── construction.glb
│   │       ├── construction_phase2.glb # Advanced building
│   │       ├── construction_phase3.glb # Near completion
│   │       ├── command_active.glb      # Operations mode
│   │       ├── command_emergency.glb   # Crisis management
│   │       ├── communication.glb       # Data transmission
│   │       ├── communication_long.glb  # Long-range contact
│   │       ├── research_mode.glb       # Technology development
│   │       ├── production_mode.glb     # Unit manufacturing
│   │       ├── shield_charging.glb     # Defense preparation
│   │       ├── shield_active.glb       # Protection enabled
│   │       ├── shield_overload.glb     # System strain
│   │       ├── power_surge.glb         # Energy spike
│   │       ├── maintenance.glb         # Routine upkeep
│   │       ├── upgrade_tech.glb        # Enhancement process
│   │       ├── damaged.glb
│   │       ├── damaged_sparking.glb    # Electrical damage
│   │       ├── emergency_power.glb     # Backup systems
│   │       ├── evacuation.glb  # Personnel withdrawal
│   │       └── destroyed.glb
│   ├── production/             # Manufacturing facilities
│   │   ├── barracks/           # Infantry training
│   │   │   ├── barracks.glb
│   │   │   ├── barracks_lod1.glb
│   │   │   ├── barracks_lod2.glb
│   │   │   ├── barracks_icon.glb
│   │   │   └── animations/
│   │   │       ├── idle.glb
│   │   │       ├── construction.glb
│   │   │       ├── training_basic.glb  # Standard recruitment
│   │   │       ├── training_advanced.glb # Elite forces
│   │   │       ├── training_special.glb # Specialized units
│   │   │       ├── deployment.glb      # Unit graduation
│   │   │       ├── drill_formation.glb # Group training
│   │   │       ├── drill_combat.glb    # Combat practice
│   │   │       ├── drill_endurance.glb # Fitness training
│   │   │       ├── ceremony.glb        # Formal events
│   │   │       ├── inspection.glb      # Quality review
│   │   │       ├── upgrade_facility.glb # Building enhancement
│   │   │       ├── damaged.glb
│   │   │       └── destroyed.glb
│   │   ├── war_factory/        # Vehicle production
│   │   │   ├── war_factory.glb
│   │   │   ├── war_factory_lod1.glb
│   │   │   ├── war_factory_lod2.glb
│   │   │   ├── war_factory_icon.glb
│   │   │   └── animations/
│   │   │       ├── idle.glb
│   │   │       ├── construction.glb
│   │   │       ├── manufacturing_light.glb # Small vehicles
│   │   │       ├── manufacturing_heavy.glb # Large vehicles
│   │   │       ├── manufacturing_air.glb   # Aircraft assembly
│   │   │       ├── assembly_line.glb       # Automated production
│   │   │       ├── assembly_custom.glb     # Specialized builds
│   │   │       ├── quality_control.glb     # Testing phase
│   │   │       ├── painting.glb        # Finishing touches
│   │   │       ├── vehicle_rollout.glb     # Delivery
│   │   │       ├── retooling.glb       # Production change
│   │   │       ├── maintenance_bay.glb     # Repair services
│   │   │       ├── research_prototype.glb  # Experimental units
│   │   │       ├── damaged.glb
│   │   │       └── destroyed.glb
│   │   ├── shipyard/           # Naval construction
│   │   ├── aerospace_facility/ # Spacecraft production
│   │   └── drone_factory/      # Automated unit production
│   ├── infrastructure/         # Support systems
│   │   ├── energy_plant/       # Power generation
│   │   │   ├── energy_plant.glb
│   │   │   ├── energy_plant_lod1.glb
│   │   │   ├── energy_plant_lod2.glb
│   │   │   ├── energy_plant_icon.glb
│   │   │   └── animations/
│   │   │       ├── idle.glb
│   │   │       ├── construction.glb
│   │   │       ├── startup_sequence.glb    # Initial activation
│   │   │       ├── power_generation_low.glb # Minimal output
│   │   │       ├── power_generation_med.glb # Standard output
│   │   │       ├── power_generation_high.glb # Maximum output
│   │   │       ├── energy_surge.glb        # Power spike
│   │   │       ├── energy_discharge.glb    # Excess release
│   │   │       ├── cooling_cycle.glb       # Temperature control
│   │   │       ├── cooling_emergency.glb   # Overheat prevention
│   │   │       ├── maintenance_routine.glb # Regular upkeep
│   │   │       ├── maintenance_major.glb   # Extensive repair
│   │   │       ├── efficiency_boost.glb    # Performance enhancement
│   │   │       ├── overload_warning.glb    # System stress
│   │   │       ├── overload_critical.glb   # Failure imminent
│   │   │       ├── shutdown_safe.glb       # Controlled stop
│   │   │       ├── shutdown_emergency.glb  # Crisis stop
│   │   │       ├── damaged.glb
│   │   │       └── destroyed.glb
│   │   ├── resource_depot/     # Storage facility
│   │   ├── refinery/           # Resource processing
│   │   ├── communications/     # Network hub
│   │   └── transport_hub/      # Logistics center
│   ├── research/               # Technology development
│   │   ├── research_lab/       # Basic research
│   │   ├── tech_center/        # Advanced research
│   │   ├── observatory/        # Space monitoring
│   │   └── quantum_lab/        # Exotic technology
│   ├── defense/                # Protective structures
│   │   ├── defense_turret/     # Automated weapons
│   │   │   ├── defense_turret.glb
│   │   │   ├── defense_turret_lod1.glb
│   │   │   ├── defense_turret_lod2.glb
│   │   │   ├── defense_turret_icon.glb
│   │   │   └── animations/
│   │   │       ├── idle.glb
│   │   │       ├── idle_powered.glb        # System online
│   │   │       ├── construction.glb
│   │   │       ├── calibration.glb         # Initial setup
│   │   │       ├── scanning_area.glb       # Patrol mode
│   │   │       ├── scanning_focused.glb    # Target search
│   │   │       ├── target_acquired.glb     # Lock-on
│   │   │       ├── targeting_lead.glb      # Prediction aiming
│   │   │       ├── firing_single.glb       # Single shot
│   │   │       ├── firing_burst.glb        # Rapid fire
│   │   │       ├── firing_continuous.glb   # Sustained fire
│   │   │       ├── reloading_quick.glb     # Fast reload
│   │   │       ├── reloading_full.glb      # Complete reload
│   │   │       ├── turret_rotate_slow.glb  # Careful tracking
│   │   │       ├── turret_rotate_fast.glb  # Rapid response
│   │   │       ├── elevation_tracking.glb  # Vertical aim
│   │   │       ├── cooling_system.glb      # Heat management
│   │   │       ├── self_diagnostic.glb     # System check
│   │   │       ├── upgrade_weapons.glb     # Armament enhancement
│   │   │       ├── upgrade_sensors.glb     # Detection improvement
│   │   │       ├── malfunction.glb         # System error
│   │   │       ├── damaged.glb
│   │   │       └── destroyed.glb
│   │   ├── shield_generator/   # Energy barriers
│   │   ├── missile_defense/    # Anti-projectile system
│   │   ├── bunker/             # Fortified position
│   │   └── wall_segments/      # Perimeter barriers
│   └── special/                # Unique structures
│       ├── portal/             # Teleportation gate
│       ├── monument/           # Victory marker
│       ├── space_elevator/     # Orbital access
│       └── dyson_sphere/       # Cosmic megastructure
├── resources/                  # Harvestable materials
│   ├── basic/                  # Standard resources
│   │   ├── minerals/           # Common materials
│   │   │   ├── minerals.glb
│   │   │   ├── minerals_depleted.glb   # Low reserves
│   │   │   └── animations/
│   │   │       ├── idle.glb
│   │   │       ├── crystal_glow.glb    # Natural luminescence
│   │   │       ├── extraction.glb      # Being harvested
│   │   │       └── depletion.glb       # Resource exhaustion
│   │   ├── energy/             # Power sources
│   │   │   ├── energy.glb
│   │   │   ├── energy_depleted.glb
│   │   │   └── animations/
│   │   │       ├── idle.glb
│   │   │       ├── energy_pulse.glb    # Power fluctuation
│   │   │       ├── extraction.glb
│   │   │       └── overload.glb        # Dangerous discharge
│   ├── rare/                   # Valuable materials
│   │   ├── rare_earth/         # Special minerals
│   │   ├── exotic_matter/      # Advanced resources
│   │   └── quantum_crystals/   # Unique materials
│   └── cosmic/                 # Universal-scale resources
│       ├── dark_matter/        # Invisible substance
│       ├── solar_plasma/       # Stellar energy
│       └── neutronium/         # Ultra-dense matter
├── terrain/                    # Environmental objects
│   ├── planetary/              # Surface features
│   │   ├── rocks/              # Natural obstacles
│   │   ├── vegetation/         # Plant life
│   │   ├── water_bodies/       # Rivers, lakes
│   │   └── atmospheric/        # Weather effects
│   ├── space/                  # Cosmic environment
│   │   ├── asteroids/          # Space rocks
│   │   ├── debris/             # Wreckage
│   │   ├── stations/           # Orbital platforms
│   │   └── phenomena/          # Space anomalies
│   └── megastructures/         # Massive constructs
│       ├── ring_worlds/        # Orbital habitats
│       ├── sphere_segments/    # Dyson sphere parts
│       └── gate_networks/      # Transport systems
├── effects/                    # Visual phenomena
│   ├── explosions/             # Destruction effects
│   │   ├── small_explosion/    # Infantry weapons
│   │   ├── large_explosion/    # Vehicle weapons
│   │   ├── nuclear_blast/      # Massive destruction
│   │   └── cosmic_explosion/   # Stellar events
│   ├── energy/                 # Power manifestations
│   │   ├── shields/            # Protective barriers
│   │   ├── beams/              # Directed energy
│   │   └── fields/             # Area effects
│   ├── atmospheric/            # Environmental effects
│   │   ├── weather/            # Climate phenomena
│   │   ├── dust_storms/        # Planetary conditions
│   │   └── solar_flares/       # Stellar activity
│   └── technological/          # Artificial effects
│       ├── holograms/          # Projection systems
│       ├── teleportation/      # Transport effects
│       └── time_distortion/    # Temporal anomalies
└── props/                      # Decorative elements
    ├── furniture/              # Interior objects
    ├── machinery/              # Industrial equipment
    ├── vehicles/               # Transport devices
    └── artifacts/              # Historical items
```

## Technical Specifications

### File Format Standards
- **Primary Format**: glTF Binary (.glb) for all 3D assets
- **Fallback Support**: OBJ + MTL for legacy compatibility
- **Animation Format**: Embedded glTF animations or separate .glb files
- **Texture Format**: Embedded PNG/JPG with fallback external references

### Polygon Budget Guidelines
```
Unit Types:
├── Infantry Units:     800-1,500 triangles (LOD 0)
├── Vehicle Units:      1,200-2,500 triangles (LOD 0)
├── Aircraft Units:     1,000-2,000 triangles (LOD 0)
└── Naval Units:        1,500-3,000 triangles (LOD 0)

Building Types:
├── Small Structures:   1,000-2,000 triangles (LOD 0)
├── Medium Buildings:   2,000-4,000 triangles (LOD 0)
├── Large Facilities:   4,000-8,000 triangles (LOD 0)
└── Megastructures:     8,000-15,000 triangles (LOD 0)

LOD Reduction:
├── LOD 1 (Medium):     50% of LOD 0 triangles
├── LOD 2 (Low):        25% of LOD 0 triangles
└── LOD 3 (Icon):       100-200 triangles maximum
```

### Texture Specifications
```
Resolution Standards:
├── High Detail (LOD 0):    2048x2048 (hero units/buildings)
├── Standard Detail:        1024x1024 (regular assets)
├── Medium Detail (LOD 1):  512x512 (distant viewing)
└── Low Detail (LOD 2+):    256x256 (far/icon viewing)

Channel Usage:
├── Diffuse/Albedo:         RGB channels (base color)
├── Normal Maps:            RGB channels (surface detail)
├── Metallic-Roughness:     R=Metallic, G=Roughness, B=Unused, A=AO
├── Emission:               RGB channels (self-illumination)
└── Utility Maps:           Custom data (team colors, damage, etc.)
```

## Animation System

### Animation Categories

#### Movement Animations
```
Basic Locomotion:
├── idle:               2-4 second loop, subtle movement
├── walking:            1.0-1.5 second loop, steady pace
├── running:            0.8-1.2 second loop, urgent movement
├── charging:           0.6-1.0 second loop, aggressive advance
└── retreating:         1.2-1.8 second loop, tactical withdrawal

Specialized Movement:
├── crouching:          Static pose with breathing
├── crawling:           Stealth locomotion
├── climbing:           Vertical movement
├── swimming:           Aquatic locomotion
└── flying:             Aerial maneuvering
```

#### Combat Animations
```
Melee Combat:
├── attack_light:       0.4-0.6 second, quick strike
├── attack_heavy:       0.8-1.2 second, powerful blow
├── combo_sequence:     1.5-2.5 second, multi-hit chain
├── block:              0.2-0.4 second, defensive stance
├── parry:              0.3-0.5 second, counter-defense
└── dodge:              0.3-0.6 second, evasive maneuver

Ranged Combat:
├── aim:                Static pose with micro-movements
├── fire_single:        0.1-0.3 second, single shot
├── fire_burst:         0.5-0.8 second, multiple shots
├── fire_continuous:    Loop, sustained fire
├── reload_quick:       0.8-1.2 second, fast reload
├── reload_full:        1.5-2.5 second, complete reload
└── scope:              0.5-1.0 second, precision aiming
```

#### Work Animations
```
Resource Gathering:
├── gather_minerals:    1.5-2.0 second loop, mining action
├── gather_energy:      1.2-1.8 second loop, energy collection
├── gather_rare:        2.0-3.0 second loop, careful extraction
└── gather_processing:  2.5-4.0 second loop, refinement work

Construction:
├── build_foundation:   2.0-3.0 second loop, ground preparation
├── build_framework:    1.8-2.5 second loop, structure assembly
├── build_details:      1.5-2.2 second loop, finishing work
├── repair:             1.0-1.5 second loop, maintenance work
└── upgrade:            2.5-4.0 second loop, enhancement work
```

### Animation Quality Standards

#### Technical Requirements
- **Frame Rate**: 30 FPS minimum, 60 FPS for hero animations
- **Bone Count**: Maximum 64 bones per character (mobile compatibility)
- **Animation Compression**: Use keyframe reduction for file size optimization
- **Root Motion**: Support for both in-place and root motion animations
- **Blending**: All animations must support smooth transitions

#### Artistic Guidelines
- **Timing**: Follow principles of animation (anticipation, follow-through)
- **Exaggeration**: Slightly exaggerated for gameplay clarity
- **Consistency**: Maintain character personality across all animations
- **Readability**: Clear silhouettes and recognizable actions
- **Polish**: Secondary animation (cloth, hair, accessories)

## Level of Detail (LOD) System

### Automatic LOD Selection
```rust
// Pseudo-code for LOD selection
fn select_lod(zoom_level: i32, screen_size: f32) -> LODLevel {
    match zoom_level {
        1..=5 => LODLevel::High,     // Close-up tactical view
        6..=15 => LODLevel::Medium,  // Regional view
        16..=30 => LODLevel::Low,    // Strategic view
        31..=50 => LODLevel::Icon,   // Cosmic scale
    }
}
```

### LOD Variants Required
```
LOD 0 (High Detail):
├── Full geometry and textures
├── All animation bones active
├── Particle effects enabled
├── Dynamic lighting interaction
└── Used for zoom levels 1-5

LOD 1 (Medium Detail):
├── 50% polygon reduction
├── Simplified bone hierarchy
├── Reduced particle effects
├── Baked lighting where possible
└── Used for zoom levels 6-15

LOD 2 (Low Detail):
├── 75% polygon reduction
├── Minimal bone count
├── Static textures only
├── No particle effects
└── Used for zoom levels 16-30

LOD 3 (Icon):
├── Simple geometric shapes
├── No animation bones
├── Single diffuse texture
├── Representational only
└── Used for zoom levels 31-50
```

## Cosmic Scale Requirements

### Scale-Appropriate Detail
```
Unit Scale (Levels 1-10):
├── Individual components visible
├── Personal equipment details
├── Facial features (heroes)
├── Material wear and weathering
└── Dynamic shadows and lighting

Regional Scale (Levels 11-20):
├── Simplified details
├── Color-coded team identification
├── Basic shape recognition
├── Movement patterns visible
└── Reduced visual effects

Planetary Scale (Levels 21-35):
├── Icon representation
├── Formation indicators
├── Strategic overlays
├── Minimal geometry
└── Performance optimization

Cosmic Scale (Levels 36-50):
├── Abstract symbols
├── Data visualization
├── Network connections
├── Conceptual representation
└── Maximum performance
```

### Render Distance Culling
```rust
// Distance-based rendering decisions
fn should_render(object_type: ObjectType, zoom_level: i32) -> bool {
    let render_distance = match object_type {
        ObjectType::Unit => match zoom_level {
            1..=15 => true,
            16..=25 => false, // Units not visible at planetary scale
            _ => false
        },
        ObjectType::Building => match zoom_level {
            1..=20 => true,
            21..=30 => false, // Buildings merge into cities
            _ => false
        },
        ObjectType::Megastructure => match zoom_level {
            15..=45 => true,
            _ => false
        }
    };
    render_distance
}
```

## Asset Pipeline

### Creation Workflow
```
1. Concept Design
   ├── Gameplay requirements analysis
   ├── Art style guide reference
   ├── Technical constraint review
   └── Scale consideration planning

2. 3D Modeling
   ├── High-poly sculpting (ZBrush/Blender)
   ├── Low-poly retopology
   ├── UV mapping optimization
   └── LOD variant creation

3. Texturing
   ├── PBR material creation
   ├── Detail texture painting
   ├── Optimization for target resolution
   └── Variant textures (team colors, damage)

4. Rigging & Animation
   ├── Bone hierarchy setup
   ├── Skinning and weight painting
   ├── Animation keyframing
   └── Export optimization

5. Integration
   ├── glTF export with proper settings
   ├── Asset manifest registration
   ├── In-game testing and iteration
   └── Performance validation
```

### Quality Assurance Checklist
```
□ Model adheres to polygon budget
□ All textures properly sized and formatted
□ UV mapping has no overlapping issues
□ Animations loop seamlessly where required
□ LOD variants properly nested
□ Team color support implemented
□ Performance metrics within targets
□ Visual consistency with art direction
□ Gameplay readability confirmed
□ File size optimization completed
```

## Performance Guidelines

### Optimization Targets
```
Frame Rate Targets:
├── Desktop (High):     120+ FPS at 1080p
├── Desktop (Medium):   60+ FPS at 1080p
├── Mobile (High):      60+ FPS at 720p
└── Mobile (Low):       30+ FPS at 480p

Memory Usage:
├── Total Asset Pool:   ≤ 2GB (all LODs)
├── Active LOD Set:     ≤ 512MB (current zoom)
├── Animation Cache:    ≤ 128MB (active units)
└── Texture Streaming:  ≤ 256MB (visible range)

Draw Call Limits:
├── Units per Frame:    1000-5000 (depending on LOD)
├── Buildings per Frame: 200-1000 (depending on LOD)
├── Effects per Frame:  100-500 (based on complexity)
└── UI Elements:        50-200 (overlay components)
```

### Streaming Strategy
```rust
// Asset streaming based on zoom level
struct AssetStreamer {
    current_zoom: i32,
    loaded_assets: HashSet<String>,
    loading_queue: VecDeque<AssetRequest>,
}

impl AssetStreamer {
    fn update_for_zoom(&mut self, new_zoom: i32) {
        if abs(new_zoom - self.current_zoom) > 3 {
            self.queue_lod_transition(new_zoom);
        }
    }
    
    fn queue_lod_transition(&mut self, target_zoom: i32) {
        let required_lod = determine_lod_for_zoom(target_zoom);
        let required_assets = get_assets_for_lod(required_lod);
        
        for asset in required_assets {
            if !self.loaded_assets.contains(&asset) {
                self.loading_queue.push_back(AssetRequest::new(asset));
            }
        }
    }
}
```

## Integration with Game Systems

### Animation State Machine
```rust
#[derive(Debug, Clone)]
pub enum UnitAnimationState {
    // Movement states
    Idle,
    Walking,
    Running,
    
    // Combat states
    Aiming,
    Attacking,
    Reloading,
    TakingDamage,
    
    // Work states
    Gathering,
    Building,
    Repairing,
    
    // Special states
    Celebrating,
    Dying,
    Spawning,
}

pub struct AnimationController {
    current_state: UnitAnimationState,
    next_state: Option<UnitAnimationState>,
    transition_time: f32,
    animation_speed: f32,
    blend_factor: f32,
}

impl AnimationController {
    pub fn transition_to(&mut self, new_state: UnitAnimationState) {
        if self.can_transition_to(&new_state) {
            self.next_state = Some(new_state);
            self.transition_time = 0.2; // 200ms blend time
        }
    }
    
    fn can_transition_to(&self, target: &UnitAnimationState) -> bool {
        use UnitAnimationState::*;
        match (&self.current_state, target) {
            (Dying, _) => false, // Death is final
            (_, Dying) => true,  // Always can die
            (Attacking, Reloading) => true,
            (Reloading, Attacking) => true,
            _ => true, // Most transitions allowed
        }
    }
}
```

### Resource Management Integration
```rust
pub struct Model3DManager {
    models: HashMap<String, Model3D>,
    animations: HashMap<String, AnimationClip>,
    current_lod_level: LODLevel,
    zoom_system: Weak<ZoomSystem>,
}

impl Model3DManager {
    pub fn update_lod_for_zoom(&mut self, zoom_level: i32) {
        let new_lod = self.calculate_lod_for_zoom(zoom_level);
        if new_lod != self.current_lod_level {
            self.transition_to_lod(new_lod);
        }
    }
    
    pub fn get_model_for_zoom(&self, model_name: &str, zoom: i32) -> Option<&Model3D> {
        let lod_suffix = match self.calculate_lod_for_zoom(zoom) {
            LODLevel::High => "",
            LODLevel::Medium => "_lod1",
            LODLevel::Low => "_lod2",
            LODLevel::Icon => "_icon",
        };
        
        let full_name = format!("{}{}", model_name, lod_suffix);
        self.models.get(&full_name).or_else(|| {
            // Fallback to base model if LOD variant not available
            self.models.get(model_name)
        })
    }
}
```

### Rendering Pipeline Integration
```rust
pub fn render_units_at_scale(
    units: &[Unit],
    camera: &Camera,
    zoom_system: &ZoomSystem,
    model_manager: &Model3DManager,
) {
    let zoom_level = zoom_system.current_level;
    let scale = zoom_system.get_current_scale();
    
    for unit in units {
        if !should_render_unit(unit, camera, scale) {
            continue;
        }
        
        let model = model_manager.get_model_for_zoom(&unit.type_name(), zoom_level);
        if let Some(model) = model {
            let animation = get_current_animation(unit);
            render_animated_model(model, &unit.transform, animation);
        } else {
            // Fallback to procedural rendering
            render_unit_fallback(unit, zoom_level);
        }
    }
}

fn should_render_unit(unit: &Unit, camera: &Camera, scale: f64) -> bool {
    // Frustum culling
    if !camera.frustum.contains_point(unit.position) {
        return false;
    }
    
    // Scale-based culling
    let apparent_size = unit.bounding_radius / scale as f32;
    apparent_size > 0.001 // Don't render sub-pixel objects
}
```

---

This enhanced 3D model system provides the foundation for a scalable, high-performance RTS game that can seamlessly transition between intimate tactical combat and cosmic-scale strategic operations. The modular animation system ensures that each unit and building can express complex behaviors while maintaining optimal performance across all zoom levels.
