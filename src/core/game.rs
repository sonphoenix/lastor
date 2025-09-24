use macroquad::prelude::*;
use super::{Entity, Scene, TimeManager};
use crate::input::InputManager;
use crate::rendering::Camera;

/// Configuration for the game
pub struct GameConfig {
    pub title: String,
    pub window_width: i32,
    pub window_height: i32,
    pub target_fps: u32,
    pub background_color: Color,
    pub show_fps: bool,
    pub show_input_debug: bool,  // New: show input debug info
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            title: "Lastor Game".to_string(),
            window_width: 800,
            window_height: 600,
            target_fps: 60,
            background_color: Color::from_hex(0x1e1e1e),
            show_fps: false,
            show_input_debug: false,
        }
    }
}

/// The main game runner with enhanced features
pub struct Game {
    scene: Scene,
    time_manager: TimeManager,
    input_manager: InputManager,  // New: integrated input manager
    config: GameConfig,
    camera: Camera,
}

impl Game {
    pub fn new() -> Self {
        Self::with_config(GameConfig::default())
    }
    
    pub fn with_config(config: GameConfig) -> Self {
        Self {
            scene: Scene::new(),
            time_manager: TimeManager::new(),
            input_manager: InputManager::new(),  // Initialize input manager
            config,
            camera: Camera::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.scene.add_entity(entity);
    }
    
    pub fn get_scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
    
    pub fn get_time(&self) -> &TimeManager {
        &self.time_manager
    }
    
    pub fn get_input(&self) -> &InputManager {  // New: access to input manager
        &self.input_manager
    }
    
    pub fn get_input_mut(&mut self) -> &mut InputManager {  // New: mutable access for binding changes
        &mut self.input_manager
    }
    
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_manager.set_time_scale(scale);
    }

    pub fn get_camera(&self) -> &Camera {
    &self.camera
    }

pub fn get_camera_mut(&mut self) -> &mut Camera {
    &mut self.camera
}


    pub async fn run(&mut self) {
        loop {
            // Update time
            self.time_manager.update();
            let dt = self.time_manager.delta_time();
            
            // Update input 
            self.input_manager.update(dt);
             self.scene.update_with_input(dt, &self.input_manager);

            // Clear screen
            clear_background(self.config.background_color);

            // Update camera
            self.camera.update(dt);
            // Apply camera transform
            self.camera.apply();    
            
            // Update and draw scene
            self.scene.update(dt);
            self.scene.draw();
            
            // Show debug info if enabled
            if self.config.show_fps {
                let fps = get_fps();
                draw_text(&format!("FPS: {}", fps), 10.0, 30.0, 20.0, WHITE);
                draw_text(
                    &format!("Entities: {}", self.scene.active_entity_count()),
                    10.0,
                    50.0,
                    20.0,
                    WHITE,
                );
            }
            
            // Show input debug info
            if self.config.show_input_debug {
                self.draw_input_debug();
            }

            next_frame().await;
        }
    }
    
    fn draw_input_debug(&self) {
        let y_start = if self.config.show_fps { 70.0 } else { 30.0 };
        let mut y_offset = 0.0;
        
        draw_text("=== INPUT DEBUG ===", 10.0, y_start + y_offset, 16.0, YELLOW);
        y_offset += 20.0;
        
        // Show movement input
        let movement = self.input_manager.get_movement_input();
        if movement != Vec2::ZERO {
            draw_text(
                &format!("Movement: ({:.2}, {:.2})", movement.x, movement.y),
                10.0,
                y_start + y_offset,
                16.0,
                GREEN,
            );
            y_offset += 20.0;
        }
        
        // Show active actions
        use crate::input::Action;
        let test_actions = [
            Action::MoveUp, Action::MoveDown, Action::MoveLeft, Action::MoveRight,
            Action::Jump, Action::Attack, Action::Defend, Action::Interact, Action::Pause,
        ];
        
        for action in &test_actions {
            if self.input_manager.is_action_active(action) {
                draw_text(
                    &format!("Active: {:?}", action),
                    10.0,
                    y_start + y_offset,
                    16.0,
                    GREEN,
                );
                y_offset += 20.0;
            }
        }
        
        // Show mouse position
        let mouse_pos = self.input_manager.mouse_position();
        draw_text(
            &format!("Mouse: ({:.0}, {:.0})", mouse_pos.x, mouse_pos.y),
            10.0,
            y_start + y_offset,
            16.0,
            GRAY,
        );
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}