//! Demonstrates how to use a child entity as a target to offset a
//! third-person camera.
#[cfg(feature = "physics")]
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_camera::prelude::*;
use bevy_enhanced_input::prelude::*;

const MOUSE_SENSITIVITY: f32 = 0.15;
const CONTROLLER_SENSITIVITY: f32 = 0.9;

const OFFSET: Vec3 = Vec3::new(0.0, 0.5, 1.0);

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, EnhancedInputPlugin, EnhancedCameraPlugin))
        .add_input_context::<CameraContext>()
        .add_systems(Startup, setup);

    // When physics is disabled, camera rotation is immediately handled on
    // input and movement occurs on update, so motion will be smooth if player
    // is moved on update.
    #[cfg(not(feature = "physics"))]
    app.add_systems(Update, move_player);

    // When physics is enabled, the player should be moved in a fixed timestep
    // or else motion will appear jagged.
    #[cfg(feature = "physics")]
    app.add_plugins(PhysicsPlugins::default())
        .add_systems(FixedUpdate, move_player);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
                        Axial::right_stick().with((Scale::splat(CONTROLLER_SENSITIVITY), DeadZone::default(), Negate::y()))
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

    let cube = commands
        .spawn((
            Player,
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();

    // target offset
    commands.spawn((
        Transform::from_translation(OFFSET),
        TargetOf(camera),
        ChildOf(cube),
    ));

    // plane for reference
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(5.0, 5.0)))),
        MeshMaterial3d(materials.add(Color::srgb_u8(100, 230, 100))),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));

    // light
    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));
}

fn move_player(time: Res<Time>, mut player: Single<&mut Transform, With<Player>>) {
    const RADIUS: f32 = 3.0;
    let elapsed = time.elapsed_secs();

    player.translation = Vec3::new(elapsed.sin(), 0.0, elapsed.cos()) * RADIUS;
}

/// For bevy_enhanced_input, a context must be defined.
///
/// This context just means that the camera can be controlled when it is on the
/// same entity as EnhancedCamera.
#[derive(Component)]
struct CameraContext;

#[derive(Component)]
struct Player;
