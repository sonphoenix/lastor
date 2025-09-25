// examples/basic_game.rs - FIXED VERSION
use lastor::prelude::*;

struct Player {
    transform: Transform,
    speed: f32,
    active: bool,
}

impl Player {
    fn new(position: Vec2) -> Self {
        Self {
            transform: Transform::new(position),
            speed: 200.0,
            active: true,
        }
    }
}

impl Entity for Player {
    fn update(&mut self, dt: f32) {
        let mut movement = Vec2::ZERO;
        
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            movement.x += 1.0;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            movement.x -= 1.0;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            movement.y -= 1.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            movement.y += 1.0;
        }
        
        // Normalize diagonal movement
        if movement != Vec2::ZERO {
            movement = movement.normalize();
            self.transform.translate(movement * self.speed * dt);
        }

        
        
        // Keep player in world bounds (larger than screen)
        let world_width = 2000.0;
        let world_height = 2000.0;
        self.transform.position.x = self.transform.position.x.clamp(20.0, world_width - 20.0);
        self.transform.position.y = self.transform.position.y.clamp(20.0, world_height - 20.0);
    }

    fn draw(&self) {
        draw_circle(
            self.transform.position.x,
            self.transform.position.y,
            20.0,
            BLUE,
        );
        
        // Draw a direction indicator
        let forward = self.transform.forward() * 15.0;
        draw_line(
            self.transform.position.x,
            self.transform.position.y,
            self.transform.position.x + forward.x,
            self.transform.position.y + forward.y,
            2.0,
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

struct Enemy {
    transform: Transform,
    target_position: Vec2,
    speed: f32,
    active: bool,
}

impl Enemy {
    fn new(position: Vec2) -> Self {
        Self {
            transform: Transform::new(position),
            target_position: position,
            speed: 100.0,
            active: true,
        }
    }
}

impl Entity for Enemy {
    fn update(&mut self, dt: f32) {
        // Simple AI: move toward a random target
        let distance_to_target = self.transform.position.distance_to(self.target_position);
        
        if distance_to_target < 5.0 {
            // Pick a new random target in the world
            self.target_position = Vec2::new(
                rand::gen_range(50.0, 1950.0),
                rand::gen_range(50.0, 1950.0),
            );
        } else {
            // Move toward target
            self.transform.position = self.transform.position.move_toward(
                self.target_position,
                self.speed * dt,
            );
        }
        
        // Keep enemy in world bounds
        let world_width = 2000.0;
        let world_height = 2000.0;
        self.transform.position.x = self.transform.position.x.clamp(20.0, world_width - 20.0);
        self.transform.position.y = self.transform.position.y.clamp(20.0, world_height - 20.0);
    }

    fn draw(&self) {
        draw_circle(
            self.transform.position.x,
            self.transform.position.y,
            15.0,
            RED,
        );
        
        // Draw target position
        draw_circle_lines(
            self.target_position.x,
            self.target_position.y,
            5.0,
            1.0,
            YELLOW,
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

// Simple camera controller that doesn't require complex input handling
struct CameraController;

impl Entity for CameraController {
    fn update(&mut self, _dt: f32) {
        // Camera logic will be handled in main loop
    }
    
    fn draw(&self) {
        // No drawing
    }
    
    fn is_active(&self) -> bool {
        true
    }
}

#[macroquad::main("Lastor Framework Demo with Camera")]
async fn main() {
    let config = GameConfig {
        title: "Lastor Demo with Camera".to_string(),
        window_width: 1024,
        window_height: 768,
        show_fps: true,
        background_color: Color::from_hex(0x0f0f0f),
        ..Default::default()
    };

    let mut game = Game::with_config(config);
    

    // Set up camera for a larger world
    let world_size = Vec2::new(2000.0, 2000.0);
    game.get_scene_mut().camera.set_bounds(Some(CameraBounds::new(
        0.0, 0.0, world_size.x, world_size.y,
    )));

    // Add player in the world center
    let player_pos = Vec2::new(1000.0, 1000.0);
    let player = Box::new(Player::new(player_pos));
    let player_ref: *const Player = &*player; // raw pointer to access later
    game.add_entity(player);

    // Set camera to follow player dynamically
    game.get_scene_mut().camera.set_follow_target(move || unsafe {
        (*player_ref).transform.position
    });
    game.get_scene_mut().camera.set_follow_speed(6.0);
                game.get_scene_mut()
                .camera
                .add_screen_shake(5.0, 12.0); // duration, magnitude
    // Add some enemies
    let enemy_positions = [
        Vec2::new(500.0, 500.0),
        Vec2::new(1500.0, 500.0),
        Vec2::new(500.0, 1500.0),
        Vec2::new(1500.0, 1500.0),
    ];

    for pos in enemy_positions {
        game.add_entity(Box::new(Enemy::new(pos)));
    }

    // Add camera controller
    game.add_entity(Box::new(CameraController));

    println!("=== LASTOR BASIC GAME WITH CAMERA ===");
    println!("Use WASD or arrow keys to move the blue player!");
    println!("Red enemies move randomly around the large world.");
    println!("Camera automatically follows the player.");

    // Run the game
    game.run().await;
}
