#![doc = include_str!("../README.md")]
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

#[cfg(feature = "cursor_utils")]
mod cursor_utils;

/// Initializes camera control systems.
///
/// Depends on EnhancedInputPlugin for observers to be triggered.
pub struct EnhancedCameraPlugin;

impl Plugin for EnhancedCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(apply_rotation);
        #[cfg(feature = "cursor_utils")]
        app.add_plugins(cursor_utils::CursorUtilsPlugin);
    }
}

/// Entity targetted by camera.
#[derive(Component)]
#[relationship(relationship_target = Targeting)]
pub struct TargetOf(pub Entity);

/// Container for camera target
///
/// NOTE: Impl's Deref so entity can be immutably accessed, though I am not
/// certain that this is the intended way to use one-to-one relationships.
#[derive(Component, Deref)]
#[relationship_target(relationship = TargetOf)]
pub struct Targeting(Entity);

#[derive(Component)]
pub struct EnhancedCamera {
    /// Maximum pitch (vertical angle) magnitude in **radians**.
    pub max_pitch: f32,
    /// Maximum distance to follow target by
    pub follow_dist: f32,
}

impl Default for EnhancedCamera {
    fn default() -> Self {
        EnhancedCamera {
            max_pitch: 85.0_f32.to_radians(),
            follow_dist: 5.0,
        }
    }
}

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct RotateCamera;

/// Responds to camera rotation input by rotating camera.
///
/// Is not responsible for handling sensitivity or scaling with delta time,
/// that should be done by input modifiers from bevy_enhanced_input.
///
/// References:
/// - [Bevy Ahoy](https://github.com/janhohenheim/bevy_ahoy/blob/main/src/camera.rs)
fn apply_rotation(
    event: On<Fire<RotateCamera>>,
    mut cameras: Query<(Option<&Targeting>, &mut Transform, &EnhancedCamera), Without<TargetOf>>,
    transforms: Query<&Transform, With<TargetOf>>,
) {
    let Ok((target, mut transform, camera)) = cameras.get_mut(event.context) else {
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

    // Orbit target
    if let Some(target) = target {
        let Ok(target_transform) = transforms.get(**target) else {
            warn!("Camera target entity {} doesn't have transform.", **target);
            return;
        };
        transform.translation =
            target_transform.translation + transform.back() * camera.follow_dist;
    }
}

/// Re-exports for common use-cases.
pub mod prelude {
    #[cfg(feature = "cursor_utils")]
    pub use crate::cursor_utils::{LockCursor, UnlockCursor};
    pub use crate::{EnhancedCamera, EnhancedCameraPlugin, RotateCamera, TargetOf};
}
