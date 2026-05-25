//! A minimal 3D third-person example with a cube in a void.
use bevy::prelude::*;
use bevy_enhanced_camera::prelude::*;
use bevy_enhanced_input::prelude::*;

const MOUSE_SENSITIVITY: f32 = 0.15;
// Controller sensitivity will be much higher due to delta scale
const CONTROLLER_SENSITIVITY: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EnhancedInputPlugin,
            EnhancedCameraPlugin::default(),
        ))
        .add_input_context::<CameraContext>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    let camera = commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(-5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            EnhancedCamera::default(),
            CameraContext,
            actions!(CameraContext[
                (
                    Action::<RotateCamera>::new(),
                    Bindings::spawn((
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
                        ))
                    )),
                ),
                (
                    Action::<LockCursor>::new(),
                    bindings![MouseButton::Left]
                ),
                (
                    Action::<UnlockCursor>::new(),
                    bindings!(KeyCode::Escape)
                )
            ]),
        ))
        .id();

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        TargetOf(camera),
    ));

    // light
    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));
}

/// For bevy_enhanced_input, a context must be defined.
///
/// This context just means that the camera can be controlled when it is on the
/// same entity as EnhancedCamera.
#[derive(Component)]
struct CameraContext;
