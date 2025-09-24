use macroquad::prelude::*;

/// Transform component for position, rotation, and scale
#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Transform {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
    
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }
    
    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }
    
    pub fn translate(&mut self, delta: Vec2) {
        self.position += delta;
    }
    
    pub fn rotate(&mut self, delta_rotation: f32) {
        self.rotation += delta_rotation;
    }
    
    pub fn forward(&self) -> Vec2 {
        Vec2::new(self.rotation.cos(), self.rotation.sin())
    }
    
    pub fn right(&self) -> Vec2 {
        Vec2::new(-self.rotation.sin(), self.rotation.cos())
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}