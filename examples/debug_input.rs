// examples/debug_input.rs - Simple test to verify input works
use lastor::prelude::*;

struct TestPlayer {
    transform: Transform,
    active: bool,
}

impl TestPlayer {
    fn new(position: Vec2) -> Self {
        Self {
            transform: Transform::new(position),
            active: true,
        }
    }
}

impl Entity for TestPlayer {
    fn update(&mut self, _dt: f32) {
        // Default update does nothing
    }
    
    fn update_with_input(&mut self, dt: f32, input: &InputManager) {
        println!("update_with_input called!"); // Debug print
        
        // Test raw input first
        if is_key_down(KeyCode::W) {
            println!("W key detected via macroquad!");
            self.transform.position.y -= 100.0 * dt;
        }
        if is_key_down(KeyCode::S) {
            println!("S key detected via macroquad!");
            self.transform.position.y += 100.0 * dt;
        }
        if is_key_down(KeyCode::A) {
            println!("A key detected via macroquad!");
            self.transform.position.x -= 100.0 * dt;
        }
        if is_key_down(KeyCode::D) {
            println!("D key detected via macroquad!");
            self.transform.position.x += 100.0 * dt;
        }
        
        // Test our input manager
        let movement = input.get_movement_input();
        if movement != Vec2::ZERO {
            println!("Movement detected: {:?}", movement);
        }
        
        // Test individual actions
        if input.is_action_active(&Action::MoveUp) {
            println!("MoveUp action active!");
        }
        if input.is_action_active(&Action::MoveDown) {
            println!("MoveDown action active!");
        }
        if input.is_action_active(&Action::MoveLeft) {
            println!("MoveLeft action active!");
        }
        if input.is_action_active(&Action::MoveRight) {
            println!("MoveRight action active!");
        }
        
        // Apply movement
        if movement != Vec2::ZERO {
            self.transform.translate(movement * 200.0 * dt);
            println!("Player moved to: {:?}", self.transform.position);
        }
        
        // Keep on screen
        let screen_width = screen_width();
        let screen_height = screen_height();
        self.transform.position.x = self.transform.position.x.clamp(20.0, screen_width - 20.0);
        self.transform.position.y = self.transform.position.y.clamp(20.0, screen_height - 20.0);
    }

    fn draw(&self) {
        draw_circle(
            self.transform.position.x,
            self.transform.position.y,
            20.0,
            RED,
        );
        
        // Draw position text
        draw_text(
            &format!("Pos: {:.0}, {:.0}", self.transform.position.x, self.transform.position.y),
            self.transform.position.x - 30.0,
            self.transform.position.y - 30.0,
            16.0,
            WHITE,
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

#[macroquad::main("Input Debug Test")]
async fn main() {
    let config = GameConfig {
        title: "Input Debug".to_string(),
        window_width: 800,
        window_height: 600,
        show_fps: true,
        show_input_debug: true,
        background_color: BLACK,
        ..Default::default()
    };
    
    let mut game = Game::with_config(config);
    
    // Add a simple test player
    game.add_entity(Box::new(TestPlayer::new(Vec2::new(400.0, 300.0))));
    
    println!("=== INPUT DEBUG TEST ===");
    println!("Use WASD to move the red circle");
    println!("Watch the console for debug output");
    
    game.run().await;
}