#![doc = include_str!("../README.md")]
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

#[cfg(feature = "cursor_utils")]
mod cursor_utils;

#[cfg(feature = "physics")]
mod physics;

/// Initializes camera control systems.
///
/// Depends on EnhancedInputPlugin for observers to be triggered.
pub struct EnhancedCameraPlugin;

impl Plugin for EnhancedCameraPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "physics")]
        app.add_observer(physics::accumulate_input).add_systems(
            // Run in fixed pre-update so inputs that depend on camera rotation
            // can be accurate to most up-to-date camera angle.
            FixedPreUpdate,
            (
                physics::rotate_move_camera,
                physics::clear_accumulated_input,
            )
                .chain(),
        );

        #[cfg(not(feature = "physics"))]
        app.add_observer(apply_rotation)
            .add_systems(PreUpdate, move_cameras);

        #[cfg(feature = "cursor_utils")]
        app.add_plugins(cursor_utils::CursorUtilsPlugin);
    }
}

/// Entity targetted by camera.
#[derive(Component)]
#[relationship(relationship_target = Targeting)]
pub struct TargetOf(pub Entity);

/// Container for camera target
#[derive(Component)]
#[relationship_target(relationship = TargetOf)]
pub struct Targeting(Entity);

#[cfg(not(feature = "physics"))]
#[derive(Component)]
pub struct EnhancedCamera {
    /// Maximum pitch (vertical angle) magnitude in **radians**.
    pub max_pitch: f32,
    /// Maximum distance to follow target by
    pub follow_dist: f32,
}

#[cfg(feature = "physics")]
#[derive(Component)]
#[require(physics::AccumulatedCameraInput)]
pub struct EnhancedCamera {
    /// Maximum pitch (vertical angle) magnitude in **radians**.
    pub max_pitch: f32,
    /// Maximum distance to follow target by
    pub follow_dist: f32,
    /// Determines how to handle collisions.
    ///
    /// Phases through everything when `None`.
    pub physics_config: Option<physics::CameraPhysicsConfig>,
}

impl Default for EnhancedCamera {
    fn default() -> Self {
        EnhancedCamera {
            max_pitch: 85.0_f32.to_radians(),
            follow_dist: 5.0,
            #[cfg(feature = "physics")]
            physics_config: Some(physics::CameraPhysicsConfig::default()),
        }
    }
}

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct RotateCamera;

/// Applies rotation input to camera.
#[cfg(not(feature = "physics"))]
fn apply_rotation(
    event: On<Fire<RotateCamera>>,
    mut cameras: Query<(&mut Transform, &EnhancedCamera, Option<&Targeting>), Without<TargetOf>>,
    transforms: Query<&GlobalTransform, Without<EnhancedCamera>>,
) {
    let Ok((mut transform, camera, target)) = cameras.get_mut(event.context) else {
        warn!(
            "RotateCamera fired by entity {}, who is not valid in observer's query.",
            event.context
        );
        return;
    };

    let (mut yaw, mut pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

    let delta_rotation = event.value; // in degrees
    yaw += -delta_rotation.x.to_radians();
    pitch += -delta_rotation.y.to_radians();
    pitch = pitch.clamp(-camera.max_pitch, camera.max_pitch);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    // Sync rotation
    let Some(target) = target else {
        return;
    };
    let Ok(target_transform) = transforms.get(target.0) else {
        warn!("Camera target entity {} doesn't have transform.", target.0);
        return;
    };

    transform.translation = target_transform.translation() + transform.back() * camera.follow_dist;
}

/// Repositions camera around target
#[cfg(not(feature = "physics"))]
fn move_cameras(
    cameras: Query<(&Targeting, &mut Transform, &EnhancedCamera)>,
    transforms: Query<&GlobalTransform, Without<EnhancedCamera>>,
) {
    for (target, mut transform, camera) in cameras {
        let Ok(target_transform) = transforms.get(target.0) else {
            warn!("Camera target entity {} doesn't have transform.", target.0);
            return;
        };

        transform.translation =
            target_transform.translation() + transform.back() * camera.follow_dist;
    }
}

/// Re-exports for common use-cases.
pub mod prelude {
    #[cfg(feature = "cursor_utils")]
    pub use crate::cursor_utils::{LockCursor, UnlockCursor};
    #[cfg(feature = "physics")]
    pub use crate::physics::CameraPhysicsConfig;
    pub use crate::{EnhancedCamera, EnhancedCameraPlugin, RotateCamera, TargetOf};
}
