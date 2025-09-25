// src/scene.rs
use super::Entity;
use crate::input::InputManager;
use crate::rendering::Camera;
use macroquad::prelude::Vec2;
/// A scene is a collection of entities with lifecycle management
pub struct Scene {
    entities: Vec<Box<dyn Entity>>,
    entities_to_add: Vec<Box<dyn Entity>>,
    should_clear_inactive: bool,
    pub camera: Camera,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            entities_to_add: vec![],
            should_clear_inactive: false,
            camera: Camera::new(),
        }
    }

    /// Add an entity to the scene (will be added on next update)
    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entities_to_add.push(entity);
    }

    /// Update all active entities
    pub fn update(&mut self, dt: f32) {
        // Add new entities
        self.entities.extend(self.entities_to_add.drain(..));
        
        // Update active entities
        for entity in self.entities.iter_mut() {
            if entity.is_active() {
                entity.update(dt);
            }
        }
        
        // Remove inactive entities if needed
        if self.should_clear_inactive {
            self.entities.retain(|entity| entity.is_active());
            self.should_clear_inactive = false;
        }
    }
    
    /// Update all active entities with input access
    pub fn update_with_input(&mut self, dt: f32, input: &InputManager) {
        // Add new entities
        self.entities.extend(self.entities_to_add.drain(..));
        
        // Update active entities with input
        for entity in self.entities.iter_mut() {
            if entity.is_active() {
                entity.update_with_input(dt, input);
            }
        }
        
        // Remove inactive entities if needed
        if self.should_clear_inactive {
            self.entities.retain(|entity| entity.is_active());
            self.should_clear_inactive = false;
        }
    }

    /// Update only the camera (called by Game before drawing)
    pub fn update_camera(&mut self, dt: f32) {
        self.camera.update(dt);
    }

    /// Draw all active entities (without camera operations - Game handles camera.apply/reset)
    pub fn draw_entities(&self) {
        for entity in &self.entities {
            if entity.is_active() {
                entity.draw();
            }
        }
    }

    /// Draw entities with frustum culling optimization
    pub fn draw_entities_optimized(&self) {
        for entity in &self.entities {
            if !entity.is_active() {
                continue;
            }
            
            // Frustum culling - only draw if visible
            if let Some((pos, size)) = entity.get_bounds() {
                if !self.camera.is_rect_visible(pos, size) {
                    continue;
                }
            }
            
            entity.draw();
        }
    }

    /// Get immutable reference to camera
    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }
    
    /// Get mutable reference to camera
    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Mark inactive entities for removal (will be cleared on next update)
    pub fn clear_inactive(&mut self) {
        self.should_clear_inactive = true;
    }

    /// Remove all entities immediately
    pub fn clear_all_entities(&mut self) {
        self.entities.clear();
        self.entities_to_add.clear();
        self.should_clear_inactive = false;
    }

    /// Get total number of entities (including inactive)
    pub fn entity_count(&self) -> usize {
        self.entities.len() + self.entities_to_add.len()
    }
    
    /// Get number of active entities
    pub fn active_entity_count(&self) -> usize {
        self.entities.iter().filter(|e| e.is_active()).count() + 
        self.entities_to_add.iter().filter(|e| e.is_active()).count()
    }

    /// Get reference to all entities (for iteration)
    pub fn get_entities(&self) -> &Vec<Box<dyn Entity>> {
        &self.entities
    }

    /// Get mutable reference to all entities
    pub fn get_entities_mut(&mut self) -> &mut Vec<Box<dyn Entity>> {
        &mut self.entities
    }

    /// Find entities by type (simple filtering)
    pub fn find_entities<F>(&self, predicate: F) -> Vec<&Box<dyn Entity>> 
    where 
        F: Fn(&Box<dyn Entity>) -> bool,
    {
        self.entities.iter()
            .filter(|e| e.is_active() && predicate(e))
            .collect()
    }

    /// Find first entity that matches predicate
    pub fn find_first_entity<F>(&self, predicate: F) -> Option<&Box<dyn Entity>> 
    where 
        F: Fn(&Box<dyn Entity>) -> bool,
    {
        self.entities.iter()
            .find(|e| e.is_active() && predicate(e))
    }

    /// Set up camera for a platformer game
    pub fn setup_platformer_camera(&mut self, player_position: Vec2, level_size: Vec2) {
        self.camera.set_position(player_position);
        self.camera.set_bounds_from_level_size(level_size.x, level_size.y);
        //self.camera.follow_target(player_position);
        self.camera.set_follow_speed(8.0);
        self.camera.set_dead_zone(Some(50.0));
    }

    /// Set up camera for a strategy/top-down game
    pub fn setup_strategy_camera(&mut self, center: Vec2, map_size: Vec2) {
        self.camera.set_position(center);
        self.camera.set_bounds_from_level_size(map_size.x, map_size.y);
        self.camera.set_zoom(0.5);
        self.camera.set_follow_speed(5.0);
    }

    /// Set up camera for a fixed view (no following)
    pub fn setup_fixed_camera(&mut self, position: Vec2, zoom: f32) {
        self.camera.set_position(position);
        self.camera.set_zoom(zoom);
        self.camera.stop_following();
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}