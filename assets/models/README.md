# 3D Model Requirements for RTS Game

This document outlines the technical and artistic requirements for all 3D models used in our RTS game.

## Directory Structure
```
models/
├── units/
│   ├── worker/
│   │   ├── worker.glb
│   │   └── animations/
│   │       ├── idle.glb
│   │       ├── walking.glb
│   │       ├── gathering_minerals.glb
│   │       ├── gathering_energy.glb
│   │       ├── building.glb
│   │       ├── carrying_resources.glb
│   │       └── dying.glb
│   ├── fighter/
│   │   ├── fighter.glb
│   │   └── animations/
│   │       ├── idle.glb
│   │       ├── walking.glb
│   │       ├── running.glb
│   │       ├── melee_attack.glb
│   │       ├── blocking.glb
│   │       ├── victory_pose.glb
│   │       └── dying.glb
│   ├── ranger/
│   │   ├── ranger.glb
│   │   └── animations/
│   │       ├── idle.glb
│   │       ├── walking.glb
│   │       ├── running.glb
│   │       ├── aiming.glb
│   │       ├── shooting.glb
│   │       ├── reloading.glb
│   │       └── dying.glb
│   └── tank/
│       ├── tank.glb
│       └── animations/
│           ├── idle.glb
│           ├── walking.glb
│           ├── turret_rotate.glb
│           ├── firing_cannon.glb
│           ├── damaged_idle.glb
│           └── dying.glb
├── buildings/
│   ├── headquarters/
│   │   ├── headquarters.glb
│   │   └── animations/
│   │       ├── idle.glb
│   │       ├── construction.glb
│   │       ├── command_active.glb
│   │       ├── communication.glb
│   │       ├── emergency_mode.glb
│   │       ├── shield_up.glb
│   │       ├── damaged.glb
│   │       └── destroyed.glb
│   ├── barracks/
│   │   ├── barracks.glb
│   │   └── animations/
│   │       ├── idle.glb
│   │       ├── construction.glb
│   │       ├── training.glb
│   │       ├── deployment.glb
│   │       ├── drill_mode.glb
│   │       ├── damaged.glb
│   │       └── destroyed.glb
│   ├── war_factory/
│   │   ├── war_factory.glb
│   │   └── animations/
│   │       ├── idle.glb
│   │       ├── construction.glb
│   │       ├── manufacturing.glb
│   │       ├── assembly_line.glb
│   │       ├── heavy_production.glb
│   │       ├── vehicle_rollout.glb
│   │       ├── damaged.glb
│   │       └── destroyed.glb
│   ├── energy_plant/
│   │   ├── energy_plant.glb
│   │   └── animations/
│   │       ├── idle.glb
│   │       ├── construction.glb
│   │       ├── power_generation.glb
│   │       ├── energy_surge.glb
│   │       ├── cooling_cycle.glb
│   │       ├── overload.glb
│   │       ├── damaged.glb
│   │       └── destroyed.glb
│   └── defense_turret/
│       ├── defense_turret.glb
│       └── animations/
│           ├── idle.glb
│           ├── construction.glb
│           ├── scanning.glb
│           ├── targeting.glb
│           ├── firing.glb
│           ├── reloading.glb
│           ├── turret_rotate.glb
│           ├── damaged.glb
│           └── destroyed.glb
├── resources/
│   ├── minerals/
│   │   ├── minerals.glb
│   │   └── animations/
│   │       ├── idle.glb
│   │       └── crystal_glow.glb
│   └── energy/
│       ├── energy.glb
│       └── animations/
│           ├── idle.glb
│           └── energy_pulse.glb
└── effects/
    └── explosion/
        ├── explosion.glb
        └── animations/
            └── explode.glb
```

## Technical Requirements

### File Format
- All models must be exported as glTF Binary (.glb)
- Use Blender version 2.8 or higher for model creation

### Model Specifications
- Polygon count:
  - Units: 500-2000 triangles
  - Buildings: 1000-5000 triangles
  - Effects: 200-1000 triangles
- Textures:
  - Resolution: 1024×1024 for main models, 512×512 for smaller assets
  - Format: PNG or JPG embedded in glTF
  - Maps required: Diffuse, Normal, Metallic-Roughness (combined)
- UV Mapping:
  - All models must be properly UV unwrapped
  - Utilize texture space efficiently
- Origin Point:
  - Units: Centered at the base/feet
  - Buildings: Centered at the base/foundation
  - Effects: Centered at origin
- Scale:
  - 1 Blender unit = 1 meter in game
  - Worker unit should be approximately 1.8 units tall
  - Buildings scaled appropriately to units

### Animation Requirements
- Each animation is a separate .glb file in the animations/ subfolder
- Animation frame rate: 30fps
- Loop seamlessly where appropriate
- Animation duration as specified in asset_manifest.json

## Animation Organization

### Benefits of Separate Animation Files
1. **Modularity**: Each animation can be updated independently
2. **Memory Management**: Load only needed animations at runtime
3. **Version Control**: Better tracking of animation changes
4. **Collaboration**: Multiple artists can work on different animations
5. **Maintenance**: Easier to fix or update specific animations

### Animation Categories by Type

#### Worker Animations
- **idle.glb**: Standing ready animation
- **walking.glb**: Normal movement
- **gathering_minerals.glb**: Mining animation
- **gathering_energy.glb**: Energy collection animation
- **building.glb**: Construction work animation
- **carrying_resources.glb**: Walking while carrying resources
- **dying.glb**: Death sequence

#### Combat Unit Animations
- **idle.glb**: Alert standing position
- **walking.glb**: Normal movement
- **running.glb**: Fast movement
- **attack animations**: Varies by unit type (melee_attack, shooting, firing_cannon)
- **special animations**: Unit-specific (blocking, aiming, reloading, victory_pose)
- **dying.glb**: Death sequence

#### Building Animations
- **idle.glb**: Normal operational state
- **construction.glb**: Being built animation
- **working animations**: Function-specific (training, manufacturing, power_generation)
- **combat animations**: For defensive buildings (scanning, targeting, firing)
- **damage states**: damaged.glb, destroyed.glb

### Implementation Notes
- Models and animations are loaded separately by the ResourceManager
- Animation files should contain only the animation data, not the full model
- Use consistent naming conventions as specified in the manifest
- All placeholder files will be replaced with actual 3D models and animations
