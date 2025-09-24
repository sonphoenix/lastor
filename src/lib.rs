// lib.rs - Main library exports
pub mod core;
pub mod math;
pub mod input;
pub mod rendering;  


// Re-export commonly used types for convenience
pub use core::{Entity, Scene, Game, GameConfig, GameObject, TimeManager};
pub use math::{Transform, Vec2Utils};
pub use input::{InputManager, Action, InputBinding};
pub use rendering::{Camera, CameraBounds};

// Re-export macroquad types that users will commonly need
pub use macroquad::prelude::{Vec2, Color, KeyCode, MouseButton};

// Convenience prelude for users of the framework
pub mod prelude {
    pub use crate::core::{Entity, Scene, Game, GameConfig, GameObject, TimeManager};
    pub use crate::math::{Transform, Vec2Utils};
    pub use crate::input::{InputManager, Action, InputBinding};
    pub use crate::rendering::{Camera, CameraBounds}; 
    pub use macroquad::prelude::*;
}