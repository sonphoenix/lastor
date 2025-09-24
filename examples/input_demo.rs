// Fixed examples/input_demo.rs
use lastor::prelude::*;

struct Player {
    transform: Transform,
    speed: f32,
    active: bool,
    last_shot_time: f32,
    shoot_cooldown: f32,
}

impl Player {
    fn new(position: Vec2) -> Self {
        Self {
            transform: Transform::new(position),
            speed: 300.0,
            active: true,
            last_shot_time: 0.0,
            shoot_cooldown: 0.3, // Shoot every 300ms
        }
    }
}

impl Entity for Player {
    fn update(&mut self, dt: f32) {
        self.last_shot_time += dt;
    }
    
    fn update_with_input(&mut self, dt: f32, input: &InputManager) {
        self.update(dt);
        
        // Movement using the input manager
        let movement = input.get_movement_input();
        if movement != Vec2::ZERO {
            // Check for sprint action to modify speed
            let current_speed = if input.is_action_active(&Action::custom("sprint")) {
                self.speed * 2.0 // Double speed when sprinting
            } else {
                self.speed
            };
            
            self.transform.translate(movement * current_speed * dt);
        }
        
        // Rotation based on mouse position
        let mouse_pos = input.mouse_position();
        let direction = mouse_pos - self.transform.position;
        if direction.length() > 0.0 {
            self.transform.rotation = direction.to_angle();
        }
        
        // Shooting with cooldown
        if input.is_action_active(&Action::Attack) && 
           self.last_shot_time >= self.shoot_cooldown {
            println!("BANG! Shooting at angle: {:.2} radians", self.transform.rotation);
            self.last_shot_time = 0.0;
        }
        
        // Jump with buffered input (great for platformers)
        if input.is_action_just_activated(&Action::Jump) {
            println!("JUMP!");
        } else if input.is_action_buffered(&Action::Jump) {
            println!("BUFFERED JUMP! (pressed jump recently)");
        }
        
        // Interaction
        if input.is_action_just_activated(&Action::Interact) {
            println!("Interacting with object!");
        }
        
        // Defense
        if input.is_action_just_activated(&Action::Defend) {
            println!("Defending!");
        }
        
        // Pause
        if input.is_action_just_activated(&Action::Pause) {
            println!("Pause toggled!");
        }
        
        // Custom sprint action feedback
        if input.is_action_just_activated(&Action::custom("sprint")) {
            println!("Started sprinting!");
        }
        if input.is_action_just_deactivated(&Action::custom("sprint")) {
            println!("Stopped sprinting!");
        }
        
        // Keep player on screen
        let screen_width = screen_width();
        let screen_height = screen_height();
        self.transform.position.x = self.transform.position.x.clamp(20.0, screen_width - 20.0);
        self.transform.position.y = self.transform.position.y.clamp(20.0, screen_height - 20.0);
    }

    fn draw(&self) {
        // Draw player body
        draw_circle(
            self.transform.position.x,
            self.transform.position.y,
            20.0,
            BLUE,
        );
        
        // Draw direction indicator (gun barrel)
        let forward = self.transform.forward() * 25.0;
        draw_line(
            self.transform.position.x,
            self.transform.position.y,
            self.transform.position.x + forward.x,
            self.transform.position.y + forward.y,
            3.0,
            WHITE,
        );
        
        // Draw a small circle at the end of the barrel
        draw_circle(
            self.transform.position.x + forward.x,
            self.transform.position.y + forward.y,
            3.0,
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

struct MovingTarget {
    transform: Transform,
    speed: f32,
    direction: Vec2,
    active: bool,
    color: Color,
    bounce_count: u32,
}

impl MovingTarget {
    fn new(position: Vec2, color: Color) -> Self {
        // Use macroquad's rand module instead of external rand crate
        Self {
            transform: Transform::new(position),
            speed: rand::gen_range(50.0, 150.0),
            direction: Vec2::new(
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(-1.0, 1.0),
            ).normalize(),
            active: true,
            color,
            bounce_count: 0,
        }
    }
}

impl Entity for MovingTarget {
    fn update(&mut self, dt: f32) {
        // Move in current direction
        self.transform.translate(self.direction * self.speed * dt);
        
        // Bounce off screen edges
        let screen_width = screen_width();
        let screen_height = screen_height();
        let radius = 10.0;
        
        let mut bounced = false;
        
        if self.transform.position.x <= radius || self.transform.position.x >= screen_width - radius {
            self.direction.x *= -1.0;
            bounced = true;
        }
        if self.transform.position.y <= radius || self.transform.position.y >= screen_height - radius {
            self.direction.y *= -1.0;
            bounced = true;
        }
        
        if bounced {
            self.bounce_count += 1;
        }
        
        // Keep in bounds
        self.transform.position.x = self.transform.position.x.clamp(radius, screen_width - radius);
        self.transform.position.y = self.transform.position.y.clamp(radius, screen_height - radius);
    }

    fn draw(&self) {
        // Draw the target
        draw_circle(
            self.transform.position.x,
            self.transform.position.y,
            10.0,
            self.color,
        );
        
        // Draw direction indicator
        let forward = self.direction * 15.0;
        draw_line(
            self.transform.position.x,
            self.transform.position.y,
            self.transform.position.x + forward.x,
            self.transform.position.y + forward.y,
            2.0,
            WHITE,
        );
        
        // Draw bounce count
        draw_text(
            &format!("{}", self.bounce_count),
            self.transform.position.x - 5.0,
            self.transform.position.y - 15.0,
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

// Custom UI entity to show instructions
struct InstructionsUI {
    active: bool,
}

impl InstructionsUI {
    fn new() -> Self {
        Self { active: true }
    }
}

impl Entity for InstructionsUI {
    fn update(&mut self, _dt: f32) {
        // Nothing to update for UI
    }
    
    fn draw(&self) {
        // Draw instructions in the top-right corner
        let instructions = [
            "=== CONTROLS ===",
            "WASD/Arrows: Move",
            "Mouse: Aim",
            "Left Click/X: Shoot",
            "Right Click/Z: Defend", 
            "Space/Up: Jump",
            "E: Interact",
            "Escape: Pause",
            "Shift: Sprint (2x speed)",
            "",
            "Blue circle is you!",
            "Aim with mouse cursor.",
            "Watch the input debug!",
        ];
        
        let start_x = screen_width() - 220.0;
        let mut y = 30.0;
        
        for instruction in &instructions {
            if instruction.is_empty() {
                y += 10.0; // Extra space for empty lines
            } else {
                draw_text(instruction, start_x, y, 16.0, WHITE);
            }
            y += 18.0;
        }
        
        // Draw mouse crosshair
        let (mouse_x, mouse_y) = mouse_position();
        draw_circle_lines(mouse_x, mouse_y, 12.0, 2.0, RED);
        draw_line(mouse_x - 6.0, mouse_y, mouse_x + 6.0, mouse_y, 2.0, RED);
        draw_line(mouse_x, mouse_y - 6.0, mouse_x, mouse_y + 6.0, 2.0, RED);
    }
    
    fn is_active(&self) -> bool {
        self.active
    }
}

#[macroquad::main("Lastor Input System Demo")]
async fn main() {
    let config = GameConfig {
        title: "Input Management Demo".to_string(),
        window_width: 1200,
        window_height: 800,
        show_fps: true,
        show_input_debug: true,  // Enable input debugging
        background_color:BLACK,
        ..Default::default()
        
    };
    
    let mut game = Game::with_config(config);
    
    // Customize input bindings
    {
        let input = game.get_input_mut();
        
        // Add custom sprint action
        input.bind_action(Action::custom("sprint"), vec![
            InputBinding::key(KeyCode::LeftShift),
            InputBinding::key(KeyCode::RightShift),
        ]);
        
        // Add alternative jump binding (Up arrow in addition to Space)
        input.add_binding(Action::Jump, InputBinding::key(KeyCode::Up));
        
        // Set a shorter buffer time for more responsive controls
        input.set_buffer_time(0.15);
    }
    
    // Add player at center-left
    game.add_entity(Box::new(Player::new(Vec2::new(200.0, 400.0))));
    
    // Add some moving targets to make it interesting
    let colors = [RED, GREEN, YELLOW, PURPLE, ORANGE];
    for i in 0..5 {
        game.add_entity(Box::new(MovingTarget::new(
            Vec2::new(
                500.0 + (i as f32 * 100.0), 
                150.0 + (i as f32 * 80.0)
            ),
            colors[i % colors.len()],
        )));
    }
    
    // Add instructions UI
    game.add_entity(Box::new(InstructionsUI::new()));
    
    println!("=== LASTOR INPUT DEMO ===");
    println!("Try all the controls and watch the debug info!");
    println!("Notice how input buffering makes jump feel responsive.");
    println!("The sprint action is a custom action - you can create any actions you need!");
    
    game.run().await;
}
