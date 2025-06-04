# 3D Game Assets with Animation System

This directory contains all 3D models and media assets for the RTS game, organized by type. Each unit and building includes specific animations for enhanced visual feedback.

## Directory Structure

- **models/**
  - **units/**: 3D models for units (.glb format) with animations
  - **buildings/**: 3D models for buildings (.glb format) with animations
  - **resources/**: 3D models for resource nodes (.glb format)
  - **terrain/**: 3D terrain models and elements (.glb format)
  - **props/**: 3D models for props and decorations (.glb format)
  - **effects/**: 3D models for visual effects (.glb format)
- **textures/**
  - **ui/**: User interface elements and icons (2D)
- **sounds/**: Sound effect files (.ogg or .wav format)
- **music/**: Background music files (.ogg format)
- **fonts/**: Game fonts (.ttf format)

## Animation System

Each unit type has specific animations that provide visual feedback for player actions:

### Worker Unit Animations
| Animation Name | Duration | Loop | Description | Triggered When |
|----------------|----------|------|-------------|----------------|
| idle | 2.0s | Yes | Standing still, breathing | No action |
| walking | 1.0s | Yes | Basic movement | Moving to destination |
| gathering_minerals | 1.5s | Yes | Mining/collecting crystals | At mineral node |
| gathering_energy | 1.5s | Yes | Siphoning energy | At energy node |
| building | 2.0s | Yes | Construction work | Building structures |
| carrying_resources | 1.2s | Yes | Walking with resource load | Moving with resources |
| dying | 1.2s | No | Death animation | Health reaches 0 |

### Fighter Unit Animations
| Animation Name | Duration | Loop | Description | Triggered When |
|----------------|----------|------|-------------|----------------|
| idle | 2.0s | Yes | Combat ready stance | No action |
| walking | 1.0s | Yes | Marching movement | Moving slowly |
| running | 0.8s | Yes | Quick movement | Moving at speed |
| melee_attack | 0.6s | No | Sword/weapon swing | Attacking enemy |
| blocking | 1.0s | Yes | Defensive posture | Under attack |
| victory_pose | 2.0s | No | Celebration stance | After victory |
| dying | 1.2s | No | Death animation | Health reaches 0 |

### Ranger Unit Animations
| Animation Name | Duration | Loop | Description | Triggered When |
|----------------|----------|------|-------------|----------------|
| idle | 2.0s | Yes | Alert scanning stance | No action |
| walking | 1.0s | Yes | Patrol movement | Moving slowly |
| running | 0.8s | Yes | Quick reposition | Moving at speed |
| aiming | 0.5s | Yes | Taking aim | Targeting enemy |
| shooting | 0.4s | No | Firing weapon | Attacking enemy |
| reloading | 1.0s | No | Weapon reload | After multiple shots |
| dying | 1.2s | No | Death animation | Health reaches 0 |

### Tank Unit Animations
| Animation Name | Duration | Loop | Description | Triggered When |
|----------------|----------|------|-------------|----------------|
| idle | 3.0s | Yes | Engine idling | No action |
| walking | 1.5s | Yes | Tank movement | Moving |
| turret_rotate | 2.0s | Yes | Turret turning | Targeting |
| firing_cannon | 1.0s | No | Cannon blast | Attacking |
| damaged_idle | 3.0s | Yes | Smoking/sparking | Below 50% health |
| dying | 2.0s | No | Explosion sequence | Health reaches 0 |

### Building Animations
| Animation Name | Duration | Loop | Description | Triggered When |
|----------------|----------|------|-------------|----------------|
| idle | 3-4s | Yes | Normal operation | Default state |
| construction | 2.0s | Yes | Being built | Under construction |
| working | 2.5s | Yes | Active production | Training units |
| damaged | 2.0s | Yes | Smoking/fire | Below 50% health |
| production | 3.0s | Yes | Manufacturing | Creating units (HQ) |
| upgrading | 5.0s | Yes | Improvement work | During upgrades |

## Creating Animated Models in Blender

### Animation Requirements
1. **Frame Rate**: 30 FPS for all animations
2. **Bone Setup**: Use armatures for character animations
3. **Animation Naming**: Exact names as specified in tables above
4. **Root Motion**: Keep root bone at origin for position control
5. **Loop Compatibility**: Ensure looping animations start/end seamlessly

### Blender Animation Workflow
1. Create base model with proper proportions
2. Add armature with appropriate bone hierarchy
3. Weight paint vertices to bones
4. Create animation actions for each required animation
5. Name actions exactly as specified in manifest
6. Test loops and transitions
7. Export with animations included

### Export Settings for Animated Models
- Format: glTF Binary (.glb)
- Include: Selected Objects
- Transform: Y Up
- Geometry: Apply Modifiers: On
- Animation: 
  - Animation Mode: Actions
  - Export all actions: On
  - NLA Strips: Off
  - Force Sample Animations: On
  - Group by NLA Track: Off

## Implementation

To add new 3D assets:
1. Create the model in Blender
2. Export to .glb format
3. Place the file in the appropriate models directory
4. Update the asset manifest (see below)

## Resource Manifest

The `asset_manifest.json` file in this directory maintains a registry of all 
game assets and their properties. When adding new assets, please update this
file to ensure the ResourceManager properly loads them.

## Creating Models in Blender

When creating models in Blender for the game:

1. Use appropriate scale (1 Blender unit = 1 meter in game)
2. Apply all transformations before export
3. Export as glTF Binary (.glb) format
4. Set the correct orientation (Y-up for Blender exports)
5. Include materials and textures in the export
6. For animated models, include armatures and animations

## Export Settings for Blender

When exporting from Blender to glTF:

- Format: glTF Binary (.glb)
- Include: Selected Objects
- Transform: Y Up
- Geometry:
  - Apply Modifiers: On
  - UVs: On
  - Normals: On
  - Tangents: On
  - Vertex Colors: On
  - Materials: On
  - Textures: On
  - Animations: On (for animated assets)

## Animation Integration

### Code Integration
- Animations are loaded via `ResourceManager` from `asset_manifest.json`
- Unit animation state is managed by `UnitAnimation` struct
- Visual feedback provided through `Model3D.draw_with_animation()`
- State transitions handled automatically based on unit actions

### Performance Considerations
- All animations are preloaded during game initialization
- Animation blending provides smooth transitions
- LOD system can disable animations at distant zoom levels
- Animation culling for off-screen units

### Testing Animations
1. Place units in-game
2. Order various actions (move, attack, gather)
3. Verify correct animations play
4. Check animation loops are smooth
5. Test state transitions
6. Verify animation performance

## Asset Creation Guidelines

### Visual Consistency
- Maintain consistent art style across all units
- Use similar color palettes for faction identification
- Ensure readable silhouettes at game camera distance
- Test visibility at various zoom levels

### Technical Standards
- Keep polygon count within specified limits
- Optimize texture usage
- Test animations at target framerate
- Ensure proper UV mapping for all models

This animation system provides rich visual feedback to players and enhances the overall game experience through expressive character movement and actions.
