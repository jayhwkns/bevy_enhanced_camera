//! Helper functions and config scructs for physics feature.
use crate::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

impl EnhancedCamera {
    /// Returns a camera that with no `physics_config`, which will phase
    /// through all colliders.
    pub fn noclip() -> Self {
        EnhancedCamera {
            physics_config: None,
            ..Default::default()
        }
    }

    pub fn which_ignores_physics_layers(mut self, layers: impl Into<LayerMask>) -> Self {
        let Some(ref mut physics_config) = self.physics_config else {
            warn!(
                "Attempted to call `which_ignores_physics_layers` on an EnhancedCamera with no physics config"
            );
            return self;
        };
        physics_config.ignore_layers = layers.into();
        self
    }
}

pub struct CameraPhysicsConfig {
    /// How large to make the radius of the spherecast that prevents the
    /// camera from phasing through the environment.
    ///
    /// `None` will ignore colliders.
    pub spherecast_radius: f32,
    /// Which layers the camera should ignore and phase through. An object will
    /// only be ignored if *all* of its layers are ignored here (not just one).
    ///
    /// Usually, this should include a layer from the target's `CollisionLayers`
    /// `membership` field.
    pub ignore_layers: LayerMask,
}

impl Default for CameraPhysicsConfig {
    fn default() -> Self {
        CameraPhysicsConfig {
            spherecast_radius: 0.05,
            ignore_layers: LayerMask::NONE,
        }
    }
}

/// The sum of all `RotateCamera` values each frame.
#[derive(Component, Default)]
pub(crate) struct AccumulatedCameraInput(Vec2);

/// Adds `RotateCamera` value to entity's `AccumulatedCameraInput`.
pub(crate) fn accumulate_input(
    event: On<Fire<RotateCamera>>,
    mut accumulated_inputs: Query<&mut AccumulatedCameraInput>,
) {
    let Ok(mut acc) = accumulated_inputs.get_mut(event.context) else {
        warn!("RotateCamera event fired on an entity without EnhancedCamera");
        return;
    };
    acc.0 += event.value;
}

pub(crate) fn clear_accumulated_input(accumulated_inputs: Query<&mut AccumulatedCameraInput>) {
    for mut acc in accumulated_inputs {
        acc.0 = Vec2::ZERO;
    }
}

/// Rotates and moves camera while handling collisions according to
/// `AccumulatedCameraInput` in a fixed timestep.
///
/// References:
/// - [Bevy Ahoy](https://github.com/janhohenheim/bevy_ahoy/blob/main/src/camera.rs)
pub(crate) fn rotate_move_camera(
    cameras: Query<(
        Option<&crate::Targeting>,
        &mut Transform,
        &EnhancedCamera,
        &AccumulatedCameraInput,
    )>,
    transforms: Query<&GlobalTransform>,
    spatial_query: SpatialQuery,
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
