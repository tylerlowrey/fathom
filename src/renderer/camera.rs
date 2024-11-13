use bevy::math::Mat4;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Camera {
    pub transform: Mat4
}