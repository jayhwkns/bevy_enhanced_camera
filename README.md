# Bevy Enhanced Camera

A minimal, configurable, general purpose first person and third person camera
library for Bevy.
Natively uses [bevy_enhanced_input](https://github.com/simgine/bevy_enhanced_input).
If your project doesn't use bevy_enhanced_input, you should consider using
[bevy_third_person_camera](https://github.com/The-DevBlog/bevy_third_person_camera)
instead.

With bevy_enhanced_input, the user is responsible for configuring bindings,
sensitivity, etc.

#### Minimal Example

```rust
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_enhanced_camera::prelude::*;

const MOUSE_SENSITIVITY: f32 = 0.15;
// Controller sensitivity will be much higher due to delta scale
const CONTROLLER_SENSITIVITY: f32 = 100.0;

#[derive(Component)]
struct CameraContext;

let mut app = App::new();
app.add_plugins((EnhancedInputPlugin, EnhancedCameraPlugin))
    .add_input_context::<CameraContext>()
    .finish();

let camera = app.world_mut().spawn((
        Camera3d::default(),
        Transform::default(),
        EnhancedCamera::default(),
        CameraContext,
        actions!(CameraContext[
            (
                Action::<RotateCamera>::new(),
                Bindings::spawn((
                    // Mouse support
                    Spawn((Binding::mouse_motion(), Scale::splat(MOUSE_SENSITIVITY))),
                    // Sticks require some special handling.
                    Axial::right_stick().with((
                        // Always put DeadZone FIRST or the scale will interfere.
                        DeadZone::default(),
                        Scale::splat(CONTROLLER_SENSITIVITY),
                        Negate::y(),
                        // DeltaScale is necessary because the stick affects
                        // angular velocity, not the angle itself (unlike
                        // when using the mouse).
                        DeltaScale::default(),
                    )),
                )),
            ),
        ]),
    ))
    .id();

app.world_mut().spawn((
    Transform::from_xyz(0.0, 0.0, 0.0),
    TargetOf(camera),
));
```

## Cargo Features

To keep the core of this crate simple, many features must be manually included
via your project's `Cargo.toml`.

### Cursor Utils

Included by default.
Adds `LockCursor` and `UnlockCursor` actions to allow users to
ergonomically keep the cursor from flying out of the window.

### Physics

Uses [Avian](https://github.com/avianphysics/avian) to prevent the camera from
phasing through terrain.
See physics example in repository for a basic setup.
Requires `PhysicsPlugins` in app or will panic otherwise.

If you are using this crate for a first-person camera, it is best to leave
physics disabled and let your character controller handle collisions.

<img width="498" height="390" alt="bevy_enhanced_camera" src="https://github.com/user-attachments/assets/26288e5f-556c-4106-bed3-d058a543353f" />

## Version Table

| bevy  | bevy_enhanced_camera |
|-------|----------------------|
| 0.18  | 0.1-0.3, main        |
