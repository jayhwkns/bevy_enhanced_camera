#![doc = include_str!("../README.md")]
#[cfg(feature = "physics")]
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_schedule_utils::ScheduleStatePlugin;

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
        app.add_observer(accumulate_input).add_systems(
            // Run in fixed pre-update so inputs that depend on camera rotation
            // can be accurate to most up-to-date camera angle.
            FixedPreUpdate,
            (
                apply_rotation,
                clear_accumulated_input
                    .run_if(bevy_schedule_utils::fixed_pre_update_ran_this_frame),
            )
                .chain(),
        );

        #[cfg(feature = "cursor_utils")]
        app.add_plugins(cursor_utils::CursorUtilsPlugin);

        if !app.is_plugin_added::<ScheduleStatePlugin>() {
            app.add_plugins(ScheduleStatePlugin);
        }
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
#[derive(Component)]
#[relationship_target(relationship = TargetOf)]
pub struct Targeting(Entity);

#[derive(Component)]
#[require(AccumulatedCameraInput)]
pub struct EnhancedCamera {
    /// Maximum pitch (vertical angle) magnitude in **radians**.
    pub max_pitch: f32,
    /// Maximum distance to follow target by
    pub follow_dist: f32,
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

#[derive(Component, Default)]
struct AccumulatedCameraInput(Vec2);

fn accumulate_input(
    event: On<Fire<RotateCamera>>,
    mut accumulated_inputs: Query<&mut AccumulatedCameraInput>,
) {
    let Ok(mut acc) = accumulated_inputs.get_mut(event.context) else {
        warn!("RotateCamera event fired on an entity without EnhancedCamera");
        return;
    };
    acc.0 += event.value;
}

fn clear_accumulated_input(accumulated_inputs: Query<&mut AccumulatedCameraInput>) {
    for mut acc in accumulated_inputs {
        acc.0 = Vec2::ZERO;
    }
}

/// Responds to camera rotation input by rotating camera.
///
/// Is not responsible for handling sensitivity or scaling with delta time,
/// that should be done by input modifiers from bevy_enhanced_input.
///
/// References:
/// - [Bevy Ahoy](https://github.com/janhohenheim/bevy_ahoy/blob/main/src/camera.rs)
fn apply_rotation(
    cameras: Query<(
        Option<&Targeting>,
        &mut Transform,
        &EnhancedCamera,
        &AccumulatedCameraInput,
    )>,
    transforms: Query<&GlobalTransform>,
    #[cfg(feature = "physics")] spatial_query: SpatialQuery,
) {
    for (target, mut transform, camera, accumulated_input) in cameras {
        let (mut yaw, mut pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        yaw += -accumulated_input.0.x.to_radians();
        pitch += -accumulated_input.0.y.to_radians();
        pitch = pitch.clamp(-camera.max_pitch, camera.max_pitch);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

        // Orbit target
        if let Some(target) = target {
            let Ok(target_transform) = transforms.get(target.0) else {
                warn!("Camera target entity {} doesn't have transform.", target.0);
                return;
            };

            let new_pos = target_transform.translation() + transform.back() * camera.follow_dist;

            // Cast to ensure camera doesn't go through colliders
            #[cfg(feature = "physics")]
            let new_pos = if let Some(physics_config) = &camera.physics_config {
                let radius = physics_config.spherecast_radius;
                let ignore_layers = physics_config.ignore_layers;

                let shape = Collider::sphere(radius);
                let origin = target_transform.translation();
                let rotation = Quat::default();
                let direction = transform.back();

                let config = ShapeCastConfig::from_max_distance(camera.follow_dist - radius);
                let filter = SpatialQueryFilter::from_mask(!ignore_layers);

                if let Some(first_hit) =
                    spatial_query.cast_shape(&shape, origin, rotation, direction, &config, &filter)
                {
                    origin + transform.back() * first_hit.distance
                } else {
                    new_pos
                }
            } else {
                new_pos
            };

            transform.translation = new_pos;
        }
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
