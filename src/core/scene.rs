use super::Entity;
use crate::input::InputManager;

/// A scene is a collection of entities with lifecycle management
pub struct Scene {
    entities: Vec<Box<dyn Entity>>,
    entities_to_add: Vec<Box<dyn Entity>>,
    should_clear_inactive: bool,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            entities_to_add: vec![],
            should_clear_inactive: false,
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entities_to_add.push(entity);
    }

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
    
    // New: update with input access
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

    pub fn draw(&self) {
        for entity in &self.entities {
            if entity.is_active() {
                entity.draw();
            }
        }
    }
    
    pub fn clear_inactive(&mut self) {
        self.should_clear_inactive = true;
    }
    
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
    
    pub fn active_entity_count(&self) -> usize {
        self.entities.iter().filter(|e| e.is_active()).count()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}