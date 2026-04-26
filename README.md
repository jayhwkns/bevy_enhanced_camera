# Bevy Enhanced Camera

A minimal, configurable, general purpose first person and third person camera
library for Bevy.
Natively uses [bevy_enhanced_input](https://github.com/simgine/bevy_enhanced_input).
If your project doesn't use bevy_enhanced_input, you should consider using
[bevy_third_person_camera](https://github.com/The-DevBlog/bevy_third_person_camera)
instead.

With bevy_enhanced_input, the user is responsible for configuring bindings,
sensitivity, etc.
A minimal setup is shown below.

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
