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
                    Spawn(Binding::mouse_motion()),
                    // Controller support
                    Axial::right_stick(),
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

#### Fixed Timestep

If you notice that the gameplay's motion appears very jagged, you may be
running the target's movement system in `Update` (or similar) while the
camera is updated on `FixedPreUpdate` when the physics feature is enabled.

The weakness of this is that `Time<Fixed>` is 64 Hz by default, so if the game
is running above 64fps, the character and camera will move less smoothly than
the game's framerate and the displayed transforms might be slightly inaccurate
without interpolation (which has not been implemented yet).

Currently, there is no way to have a physical third-person camera running in a
fixed timestep and a non-physical first-person camera running on update, though
it is a planned feature.

<img width="498" height="390" alt="bevy_enhanced_camera" src="https://github.com/user-attachments/assets/26288e5f-556c-4106-bed3-d058a543353f" />

## Version Table

| bevy  | bevy_enhanced_camera |
|-------|----------------------|
| 0.18  | 0.1-0.3, main        |
