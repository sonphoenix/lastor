use macroquad::prelude::*;
use crate::math::Vec2Utils;

/// Camera bounds for constraining camera movement
#[derive(Debug, Clone)]
pub struct CameraBounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl CameraBounds {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min: Vec2::new(min_x, min_y),
            max: Vec2::new(max_x, max_y),
        }
    }
    
    pub fn from_size(width: f32, height: f32) -> Self {
        Self {
            min: Vec2::ZERO,
            max: Vec2::new(width, height),
        }
    }
    
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y
    }
    
    pub fn clamp(&self, point: Vec2) -> Vec2 {
        Vec2::new(
            point.x.clamp(self.min.x, self.max.x),
            point.y.clamp(self.min.y, self.max.y),
        )
    }
}

/// Camera with following, screen shake, zoom, and bounds support
pub struct Camera {
    // Basic transform
    pub position: Vec2,
    pub zoom: f32,
    pub rotation: f32,
    
    // Screen shake
    shake_intensity: f32,
    shake_duration: f32,
    shake_timer: f32,
    shake_offset: Vec2,
    
    // Target following (changed: now closure instead of static Vec2)
    pub follow_target: Option<Box<dyn Fn() -> Vec2>>,
    follow_speed: f32,
    follow_offset: Vec2,
    
    // Camera bounds
    bounds: Option<CameraBounds>,
    
    // Screen properties
    screen_center: Vec2,
    
    // Smoothing
    target_position: Vec2,
    target_zoom: f32,
    zoom_speed: f32,
    
    // Dead zone (area where camera doesn't follow)
    dead_zone: Option<f32>,
}

impl Camera {
    pub fn new() -> Self {
        let screen_center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
        
        Self {
            position: screen_center,
            zoom: 1.0,
            rotation: 0.0,
            
            shake_intensity: 0.0,
            shake_duration: 0.0,
            shake_timer: 0.0,
            shake_offset: Vec2::ZERO,
            
            follow_target: None,
            follow_speed: 5.0,
            follow_offset: Vec2::ZERO,
            
            bounds: None,
            screen_center,
            
            target_position: screen_center,
            target_zoom: 1.0,
            zoom_speed: 5.0,
            
            dead_zone: None,
        }
    }

    /// Set a dynamic follow target (closure that returns Vec2)
    pub fn set_follow_target<F>(&mut self, f: F)
    where
        F: Fn() -> Vec2 + 'static,
    {
        self.follow_target = Some(Box::new(f));
    }

    pub fn clear_follow_target(&mut self) {
        self.follow_target = None;
    }
    
    pub fn update(&mut self, dt: f32) {
        self.screen_center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
        self.update_following(dt);
        self.update_smooth_movement(dt);
        self.update_screen_shake(dt);
        self.update_smooth_zoom(dt);
        self.apply_bounds();
    }
    
    fn update_following(&mut self, dt: f32) {
        if let Some(get_target) = &self.follow_target {
            let target = get_target(); 
            let target_with_offset = target + self.follow_offset;
            
            // Dead zone
            if let Some(dead_zone_radius) = self.dead_zone {
                let distance = self.target_position.distance_to(target_with_offset);
                if distance <= dead_zone_radius {
                    return;
                }
            }
            
            // Smooth following
            if self.follow_speed > 0.0 {
                let distance_factor_val = distance_factor(self.target_position, target_with_offset);
                let move_amount = self.follow_speed * distance_factor_val * dt * 60.0;
                self.target_position = self.target_position.move_toward(
                    target_with_offset,
                    move_amount
                );
            } else {
                self.target_position = target_with_offset;
            }
        }
    }

    fn update_smooth_movement(&mut self, dt: f32) {
        // Smooth position interpolation
        let move_speed = 10.0; // Adjust for responsiveness
        self.position = self.position.move_toward(self.target_position, move_speed * dt * 60.0);
    }
    
    fn update_screen_shake(&mut self, dt: f32) {
        if self.shake_timer > 0.0 {
            self.shake_timer -= dt;
            
            // Calculate shake intensity (decreases over time)
            let shake_factor = self.shake_timer / self.shake_duration;
            let current_intensity = self.shake_intensity * shake_factor;
            
            // Generate random shake offset
            self.shake_offset = Vec2::new(
                rand::gen_range(-current_intensity, current_intensity),
                rand::gen_range(-current_intensity, current_intensity),
            );
        } else {
            self.shake_offset = Vec2::ZERO;
        }
    }
    
    fn update_smooth_zoom(&mut self, dt: f32) {
        if (self.zoom - self.target_zoom).abs() > 0.01 {
            let zoom_direction = if self.target_zoom > self.zoom { 1.0 } else { -1.0 };
            let zoom_delta = self.zoom_speed * zoom_direction * dt;
            
            self.zoom += zoom_delta;
            
            // Clamp to target if we overshot
            if zoom_direction > 0.0 && self.zoom > self.target_zoom {
                self.zoom = self.target_zoom;
            } else if zoom_direction < 0.0 && self.zoom < self.target_zoom {
                self.zoom = self.target_zoom;
            }
        }
    }
    
    fn apply_bounds(&mut self) {
        if let Some(bounds) = &self.bounds {
            // Calculate camera viewport in world space
            let half_view_width = (screen_width() * 0.5) / self.zoom;
            let half_view_height = (screen_height() * 0.5) / self.zoom;
            
            // Clamp camera position to keep viewport within bounds
            let min_camera_pos = Vec2::new(
                bounds.min.x + half_view_width,
                bounds.min.y + half_view_height,
            );
            let max_camera_pos = Vec2::new(
                bounds.max.x - half_view_width,
                bounds.max.y - half_view_height,
            );
            
            self.position.x = self.position.x.clamp(min_camera_pos.x, max_camera_pos.x);
            self.position.y = self.position.y.clamp(min_camera_pos.y, max_camera_pos.y);
            
            // Also clamp target position for smooth movement
            self.target_position.x = self.target_position.x.clamp(min_camera_pos.x, max_camera_pos.x);
            self.target_position.y = self.target_position.y.clamp(min_camera_pos.y, max_camera_pos.y);
        }
    }

        /// Helper method for entities to convert coordinates using the active camera
    pub fn world_to_screen_current(world_pos: Vec2) -> Vec2 {
        // This would need a different approach - maybe a global camera instance
        // or we pass camera reference to entities
        world_pos // placeholder
    }
    
    /// Check if an entity is visible for culling
    pub fn is_rect_visible(&self, position: Vec2, size: Vec2) -> bool {
        let (min, max) = self.get_view_rect();
        position.x + size.x >= min.x && position.x <= max.x &&
        position.y + size.y >= min.y && position.y <= max.y
    }


      pub fn with_bounds(mut self, bounds: CameraBounds) -> Self {
        self.set_bounds(Some(bounds));
        self
    }
    
    pub fn with_follow_speed(mut self, speed: f32) -> Self {
        self.set_follow_speed(speed);
        self
    }
    
    pub fn strategic_camera(position: Vec2, level_size: Vec2) -> Self {
        let mut camera = Camera::new();
        camera.set_position(position);
        camera.set_bounds_from_level_size(level_size.x, level_size.y);
        camera.set_zoom(0.5); // Zoomed out for strategy games
        camera
    }
    
    pub fn platformer_camera(position: Vec2, level_size: Vec2) -> Self {
        let mut camera = Camera::new();
        camera.set_position(position);
        camera.set_bounds_from_level_size(level_size.x, level_size.y);
        camera.set_follow_speed(8.0); // Faster following for platformers
        camera.set_dead_zone(Some(50.0)); // Dead zone for less jittery movement
        camera
    }
    
    // === Basic Controls ===
    
    /// Set camera position immediately
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.target_position = position;
    }
    
    /// Move camera by offset
    pub fn translate(&mut self, offset: Vec2) {
        self.set_position(self.position + offset);
    }
    
    /// Set camera zoom immediately
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.1); // Prevent negative/zero zoom
        self.target_zoom = self.zoom;
    }
    
    /// Set target zoom (smooth transition)
    pub fn set_target_zoom(&mut self, zoom: f32) {
        self.target_zoom = zoom.max(0.1);
    }
    
    /// Set zoom transition speed
    pub fn set_zoom_speed(&mut self, speed: f32) {
        self.zoom_speed = speed;
    }
    
    /// Set camera rotation in radians
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
    
    /// Rotate camera by angle
    pub fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }
    
        // === Following System ===
    
    /// Update the follow target (replace with a new closure)
    pub fn update_follow_target<F>(&mut self, f: F)
    where
        F: Fn() -> Vec2 + 'static,
    {
        self.follow_target = Some(Box::new(f));
    }

    /// Stop following target
    pub fn stop_following(&mut self) {
        self.follow_target = None;
    }

    
    /// Set follow speed (0 = instant, higher = slower/smoother)
    pub fn set_follow_speed(&mut self, speed: f32) {
        self.follow_speed = speed;
    }
    
    /// Set offset from follow target
    pub fn set_follow_offset(&mut self, offset: Vec2) {
        self.follow_offset = offset;
    }
    
    /// Set dead zone radius (camera won't move if target is within this distance)
    pub fn set_dead_zone(&mut self, radius: Option<f32>) {
        self.dead_zone = radius;
    }
    
    // === Screen Shake ===
    
    /// Add screen shake effect
    pub fn add_screen_shake(&mut self, intensity: f32, duration: f32) {
        println!("camera is shaking");
        self.shake_intensity = intensity;
        self.shake_duration = duration;
        self.shake_timer = duration;
    }
    
    /// Stop screen shake immediately
    pub fn stop_screen_shake(&mut self) {
        self.shake_timer = 0.0;
        self.shake_offset = Vec2::ZERO;
    }
    
    /// Check if camera is currently shaking
    pub fn is_shaking(&self) -> bool {
        self.shake_timer > 0.0
    }
    
    // === Bounds System ===
    
    /// Set camera bounds (camera will not move outside these bounds)
    pub fn set_bounds(&mut self, bounds: Option<CameraBounds>) {
        self.bounds = bounds;
    }
    
    /// Set bounds from level size
    pub fn set_bounds_from_level_size(&mut self, width: f32, height: f32) {
        self.bounds = Some(CameraBounds::from_size(width, height));
    }
    
    /// Remove camera bounds
    pub fn clear_bounds(&mut self) {
        self.bounds = None;
    }
    
    // === Coordinate Conversion ===
    
    /// Convert world position to screen position
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let cam_pos = self.position + self.shake_offset;
        
        // Translate relative to camera
        let mut relative_pos = world_pos - cam_pos;
        
        // Apply rotation
        if self.rotation != 0.0 {
            let cos_rot = self.rotation.cos();
            let sin_rot = self.rotation.sin();
            relative_pos = Vec2::new(
                relative_pos.x * cos_rot - relative_pos.y * sin_rot,
                relative_pos.x * sin_rot + relative_pos.y * cos_rot,
            );
        }
        
        // Apply zoom and translate to screen center
        relative_pos * self.zoom + self.screen_center
    }
    
    /// Convert screen position to world position
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let cam_pos = self.position + self.shake_offset;
        
        // Translate relative to screen center and apply inverse zoom
        let mut relative_pos = (screen_pos - self.screen_center) / self.zoom;
        
        // Apply inverse rotation
        if self.rotation != 0.0 {
            let cos_rot = (-self.rotation).cos();
            let sin_rot = (-self.rotation).sin();
            relative_pos = Vec2::new(
                relative_pos.x * cos_rot - relative_pos.y * sin_rot,
                relative_pos.x * sin_rot + relative_pos.y * cos_rot,
            );
        }
        
        // Translate to world position
        relative_pos + cam_pos
    }
    
    /// Get the camera's view rectangle in world space
    pub fn get_view_rect(&self) -> (Vec2, Vec2) {
        let half_width = (screen_width() * 0.5) / self.zoom;
        let half_height = (screen_height() * 0.5) / self.zoom;
        let center = self.position + self.shake_offset;
        
        let min = Vec2::new(center.x - half_width, center.y - half_height);
        let max = Vec2::new(center.x + half_width, center.y + half_height);
        
        (min, max)
    }
    
    /// Check if a point is visible by the camera
    pub fn is_point_visible(&self, world_pos: Vec2) -> bool {
        let (min, max) = self.get_view_rect();
        world_pos.x >= min.x && world_pos.x <= max.x &&
        world_pos.y >= min.y && world_pos.y <= max.y
    }
    
    /// Check if a circle is visible by the camera (with radius)
    pub fn is_circle_visible(&self, world_pos: Vec2, radius: f32) -> bool {
        let (min, max) = self.get_view_rect();
        world_pos.x + radius >= min.x && world_pos.x - radius <= max.x &&
        world_pos.y + radius >= min.y && world_pos.y - radius <= max.y
    }
    
    // === Camera Application ===
    
    /// Apply camera transform for drawing world objects
    pub fn apply(&self) {
        let final_pos = self.position + self.shake_offset;
        
        // Push matrix
        push_camera_state();
        
        // Set camera
        set_camera(&Camera2D {
            target: final_pos,
            zoom: Vec2::new(self.zoom / screen_width(), self.zoom / screen_height()),
            rotation: self.rotation,
            ..Default::default()
        });
    }
    
    /// Reset camera transform (for UI drawing)
    pub fn reset(&mut self ) {
        pop_camera_state();
    }
    
    // === Utility Methods ===
    
    /// Get current camera position (including shake)
    pub fn get_final_position(&self) -> Vec2 {
        self.position + self.shake_offset
    }
    
    /// Get camera forward direction (based on rotation)
    pub fn get_forward(&self) -> Vec2 {
        Vec2::new(self.rotation.cos(), self.rotation.sin())
    }
    
    /// Get camera right direction
    pub fn get_right(&self) -> Vec2 {
        Vec2::new(-self.rotation.sin(), self.rotation.cos())
    }
    
    /// Smoothly move camera to a position
    pub fn move_to(&mut self, target: Vec2) {
        self.target_position = target;
    }
    
    /// Check if camera has reached its target position
    pub fn is_at_target(&self) -> bool {
        self.position.distance_to(self.target_position) < 1.0
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

fn distance_factor(from: Vec2, to: Vec2) -> f32 {
    let distance = from.distance_to(to);
    (distance / 100.0).min(2.0).max(0.1)
}
