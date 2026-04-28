use crate::EnhancedCamera;
use avian3d::prelude::*;
use bevy::prelude::*;

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
