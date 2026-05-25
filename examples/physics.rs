//! Demonstrates the physics feature by introducing a plane that the camera
//! can collide with.
//!
//! This example also demonstrates how to make the camera ignore the player's
//! collider if it has one.
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_camera::prelude::*;
use bevy_enhanced_input::prelude::*;

const MOUSE_SENSITIVITY: f32 = 0.15;
const CONTROLLER_SENSITIVITY: f32 = 100.0;

// Note that the default physics layer is 0b1, so the camera ignore layer must
// be different.
const CAMERA_IGNORE: u32 = 0b10;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EnhancedInputPlugin,
            EnhancedCameraPlugin::default(),
            PhysicsPlugins::default(),
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
            EnhancedCamera::default().which_ignores_physics_layers(CAMERA_IGNORE),
            CameraContext,
            actions!(CameraContext[
                (
                    Action::<RotateCamera>::new(),
                    Bindings::spawn((
                        Spawn((Binding::mouse_motion(), Scale::splat(MOUSE_SENSITIVITY))),
                        Axial::right_stick().with((
                            DeadZone::default(),
                            Scale::splat(CONTROLLER_SENSITIVITY),
                            Negate::y(),
                            DeltaScale::default(),
                        )),
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
        Collider::cuboid(1.0, 1.0, 1.0),
        CollisionLayers::new(CAMERA_IGNORE, LayerMask::ALL),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        TargetOf(camera),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(3.0, 8.0, 5.0),
    ));

    // plane
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(10.0, 0.0, 10.0),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(5.0, 5.0)))),
        MeshMaterial3d(materials.add(Color::srgb_u8(100, 230, 100))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

/// For bevy_enhanced_input, a context must be defined.
///
/// This context just means that the camera can be controlled when it is on the
/// same entity as EnhancedCamera.
#[derive(Component)]
struct CameraContext;
