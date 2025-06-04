# 3D Model Requirements for RTS Game

This document outlines the technical and artistic requirements for all 3D models used in our RTS game.

## Technical Requirements

### File Format
- All models must be exported as glTF Binary (.glb)
- Use Blender version 2.8 or higher for model creation

### Model Specifications
- Polygon count:
  - Units: 500-2000 triangles
  - Buildings: 1000-5000 triangles
  - Environment: As optimized as possible
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
  - Props: Centered at the base
- Scale:
  - 1 Blender unit = 1 meter in game
  - Worker unit should be approximately 1.8 units tall
  - Buildings scaled appropriately to units

### Animation Requirements
- Animated models should include:
  - Idle animation
  - Movement animation
  - Action animation (attack, gather, etc.)
- Animation frame rate: 30fps
- Loop seamlessly where appropriate

## Visual Style Guidelines

### Art Direction
- Low-poly stylized look
- Clean, readable silhouettes
- Strong color differentiation between unit types
- Cohesive faction aesthetics

### Factions
- Human Faction:
  - Clean, geometric designs
  - Blue/white/silver color scheme
  - Angular, technological features
- Alien Faction:
  - Organic, curved forms
  - Red/purple/green color scheme
  - Asymmetrical designs

### Unit Types
- Worker:
  - Smaller scale
  - Utility-focused design
  - Visible gathering tools
- Fighter:
  - Medium scale
  - Obvious weapons
  - Dynamic, action-ready pose
- Ranger:
  - Medium scale
  - Ranged weapon prominent
  - Lighter armor than fighter
- Tank:
  - Larger scale
  - Heavy armor elements
  - Powerful weapons

### Building Types
- Headquarters:
  - Largest structure
  - Central command aesthetics
  - Faction-defining architecture
- Barracks:
  - Military training elements
  - Medium-sized structure
  - Obvious entrance/exit for units
- Resource Structures:
  - Visual connection to resource type
  - Processing/storage visual elements

## File Naming and Organization

### Naming Convention
- Format: `[faction]_[type]_[variant].glb`
- Examples:
  - `human_worker_01.glb`
  - `alien_barracks_damaged.glb`

### Directory Structure
