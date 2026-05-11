//! A minimal 3D third-person example with a cube in a void.
use bevy::prelude::*;
use bevy_enhanced_camera::prelude::*;
use bevy_enhanced_input::prelude::*;

const MOUSE_SENSITIVITY: f32 = 0.15;
const CONTROLLER_SENSITIVITY: f32 = 0.9;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EnhancedInputPlugin, EnhancedCameraPlugin))
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_input_context::<CameraContext>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
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

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        TargetOf(camera),
    ));

    // light
    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));

    // frame-pace
    use bevy_framepace::Limiter;
    settings.limiter = Limiter::from_framerate(60.0);
}

/// For bevy_enhanced_input, a context must be defined.
///
/// This context just means that the camera can be controlled when it is on the
/// same entity as EnhancedCamera.
#[derive(Component)]
struct CameraContext;
