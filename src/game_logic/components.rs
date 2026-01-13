use ecs_core::Component;

use crate::graphics::TextureHandle;

pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Component for Position {}

pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Component for Velocity {}

pub struct Sprite {
    pub texture_name: String,
}
impl Component for Sprite {}
