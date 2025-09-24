use macroquad::prelude::*;

/// Utility trait extending Vec2 functionality
pub trait Vec2Utils {
    fn distance_to(&self, other: Vec2) -> f32;
    fn distance_squared_to(&self, other: Vec2) -> f32;
    fn angle_to(&self, other: Vec2) -> f32;
    fn move_toward(&self, target: Vec2, max_distance: f32) -> Vec2;
}

impl Vec2Utils for Vec2 {
    fn distance_to(&self, other: Vec2) -> f32 {
        (*self - other).length()
    }
    
    fn distance_squared_to(&self, other: Vec2) -> f32 {
        (*self - other).length_squared()
    }
    
    fn angle_to(&self, other: Vec2) -> f32 {
        (other - *self).to_angle()
    }
    
    fn move_toward(&self, target: Vec2, max_distance: f32) -> Vec2 {
        let diff = target - *self;
        let distance = diff.length();
        
        if distance <= max_distance {
            target
        } else {
            *self + (diff / distance) * max_distance
        }
    }
}