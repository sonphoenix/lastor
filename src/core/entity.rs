// src/core/entity.rs
use macroquad::prelude::*;
use crate::{math::Transform, input::InputManager};

/// The trait that all game objects must implement
pub trait Entity {
    /// Update the entity's logic (called every frame)
    fn update(&mut self, dt: f32);
    
    /// Draw the entity (called every frame after update)
    fn draw(&self);
    
    /// Update with input access - override this for entities that need input
    fn update_with_input(&mut self, dt: f32, _input: &InputManager) {
        // Default implementation just calls regular update (ignores input)
        // Override this method in your entities to use input
        self.update(dt);
    }
    
    /// Get read-only access to this entity's transform (if it has one)
    fn get_transform(&self) -> Option<&Transform> {
        None
    }
    
    /// Get mutable access to this entity's transform (if it has one)
    fn get_transform_mut(&mut self) -> Option<&mut Transform> {
        None
    }
    
    /// Check if this entity is active (inactive entities are not updated/drawn)
    fn is_active(&self) -> bool {
        true
    }
}

/// A basic entity implementation with transform component
/// Use this as a base for simple entities, or implement Entity trait directly for more control
pub struct GameObject {
    pub transform: Transform,
    pub active: bool,
}

impl GameObject {
    /// Create a new GameObject at the given position
    pub fn new(position: Vec2) -> Self {
        Self {
            transform: Transform::new(position),
            active: true,
        }
    }
    
    /// Create a new GameObject with a custom transform
    pub fn with_transform(transform: Transform) -> Self {
        Self {
            transform,
            active: true,
        }
    }
    
    /// Deactivate this entity (will be cleaned up by scene)
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    
    /// Reactivate this entity
    pub fn activate(&mut self) {
        self.active = true;
    }
}

impl Entity for GameObject {
    fn update(&mut self, _dt: f32) {
        // Default implementation does nothing - override this method
    }
    
    fn draw(&self) {
        // Default implementation draws a simple red circle
        draw_circle(
            self.transform.position.x,
            self.transform.position.y,
            5.0,
            RED,
        );
    }
    
    fn get_transform(&self) -> Option<&Transform> {
        Some(&self.transform)
    }
    
    fn get_transform_mut(&mut self) -> Option<&mut Transform> {
        Some(&mut self.transform)
    }
    
    fn is_active(&self) -> bool {
        self.active
    }
}