#![doc = include_str!("../README.md")]
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

#[cfg(feature = "physics")]
use avian3d::prelude::*;

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
        app.add_observer(apply_rotation)
            .add_systems(PreUpdate, move_cameras);

        #[cfg(feature = "cursor_utils")]
        app.add_plugins(cursor_utils::CursorUtilsPlugin);
    }
}

/// Entity targeted by camera.
#[derive(Component)]
#[relationship(relationship_target = Targeting)]
pub struct TargetOf(pub Entity);

/// Container for camera target
#[derive(Component)]
#[relationship_target(relationship = TargetOf)]
pub struct Targeting(Entity);

#[derive(Component)]
pub struct EnhancedCamera {
    /// Maximum pitch (vertical angle) magnitude in **radians**.
    pub max_pitch: f32,
    /// Maximum distance to follow target by
    pub follow_dist: f32,
    /// Determines how to handle collisions.
    ///
    /// Phases through everything when `None`.
    #[cfg(feature = "physics")]
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
fn apply_rotation(
    event: On<Fire<RotateCamera>>,
    mut cameras: Query<(&mut Transform, &EnhancedCamera, Option<&Targeting>), Without<TargetOf>>,
    transforms: Query<&GlobalTransform, Without<EnhancedCamera>>,
    #[cfg(feature = "physics")] spatial_query: SpatialQuery,
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

    transform.translation = orbit_pos(
        camera,
        &transform,
        target_transform.translation(),
        #[cfg(feature = "physics")]
        &spatial_query,
    );
}

/// Repositions camera around target
fn move_cameras(
    cameras: Query<(&Targeting, &mut Transform, &EnhancedCamera)>,
    transforms: Query<&GlobalTransform, Without<EnhancedCamera>>,
    #[cfg(feature = "physics")] spatial_query: SpatialQuery,
) {
    for (target, mut transform, camera) in cameras {
        let Ok(target_transform) = transforms.get(target.0) else {
            warn!("Camera target entity {} doesn't have transform.", target.0);
            continue;
        };
        transform.translation = orbit_pos(
            camera,
            &transform,
            target_transform.translation(),
            #[cfg(feature = "physics")]
            &spatial_query,
        );
    }
}

/// Calculates the orbital position of a camera around its target.
///
/// Handles collisions when physics is enabled.
fn orbit_pos(
    camera: &EnhancedCamera,
    camera_transform: &Transform,
    target_translation: Vec3,
    #[cfg(feature = "physics")] spatial_query: &SpatialQuery,
) -> Vec3 {
    let back = camera_transform.back();
    let mut new_pos = target_translation + back * camera.follow_dist;

    #[cfg(feature = "physics")]
    {
        let Some(physics_config) = &camera.physics_config else {
            return new_pos;
        };
        let radius = physics_config.spherecast_radius;
        let ignore_layers = physics_config.ignore_layers;

        let shape = Collider::sphere(radius);
        let origin = target_translation;
        let rotation = Quat::default();
        let direction = back;

        let config = ShapeCastConfig::from_max_distance(camera.follow_dist - radius);
        let filter = SpatialQueryFilter::from_mask(!ignore_layers);
        if let Some(first_hit) =
            spatial_query.cast_shape(&shape, origin, rotation, direction, &config, &filter)
        {
            new_pos = origin + back * first_hit.distance
        }
    }

    new_pos
}

/// Re-exports for common use-cases.
pub mod prelude {
    #[cfg(feature = "cursor_utils")]
    pub use crate::cursor_utils::{LockCursor, UnlockCursor};
    #[cfg(feature = "physics")]
    pub use crate::physics::CameraPhysicsConfig;
    pub use crate::{EnhancedCamera, EnhancedCameraPlugin, RotateCamera, TargetOf};
}
