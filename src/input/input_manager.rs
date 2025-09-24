use super::{Action, InputBinding};
use macroquad::prelude::*;
use std::collections::{HashMap, HashSet};

/// Manages all input state and action bindings
pub struct InputManager {
    // Action bindings
    bindings: HashMap<Action, Vec<InputBinding>>,
    
    // Input state tracking
    keys_pressed: HashSet<KeyCode>,
    keys_just_pressed: HashSet<KeyCode>,
    keys_just_released: HashSet<KeyCode>,
    
    mouse_pressed: HashSet<MouseButton>,
    mouse_just_pressed: HashSet<MouseButton>,
    mouse_just_released: HashSet<MouseButton>,
    mouse_position: Vec2,
    mouse_delta: Vec2,
    scroll_delta: Vec2,
    
    // Action state
    actions_active: HashSet<Action>,
    actions_just_activated: HashSet<Action>,
    actions_just_deactivated: HashSet<Action>,
    
    // Input buffering (for fighting games, precise timing)
    buffer_time: f32,
    buffered_actions: HashMap<Action, f32>,
}

impl InputManager {
    pub fn new() -> Self {
        let mut input_manager = Self {
            bindings: HashMap::new(),
            keys_pressed: HashSet::new(),
            keys_just_pressed: HashSet::new(),
            keys_just_released: HashSet::new(),
            mouse_pressed: HashSet::new(),
            mouse_just_pressed: HashSet::new(),
            mouse_just_released: HashSet::new(),
            mouse_position: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            scroll_delta: Vec2::ZERO,
            actions_active: HashSet::new(),
            actions_just_activated: HashSet::new(),
            actions_just_deactivated: HashSet::new(),
            buffer_time: 0.1, // 100ms buffer by default
            buffered_actions: HashMap::new(),
        };
        
        // Set up default bindings
        input_manager.setup_default_bindings();
        input_manager
    }
    
    /// Set up common default bindings
    fn setup_default_bindings(&mut self) {
        // Movement (WASD + Arrow keys)
        self.bind_action(Action::MoveUp, vec![
            InputBinding::key(KeyCode::W),
            InputBinding::key(KeyCode::Up),
        ]);
        
        self.bind_action(Action::MoveDown, vec![
            InputBinding::key(KeyCode::S),
            InputBinding::key(KeyCode::Down),
        ]);
        
        self.bind_action(Action::MoveLeft, vec![
            InputBinding::key(KeyCode::A),
            InputBinding::key(KeyCode::Left),
        ]);
        
        self.bind_action(Action::MoveRight, vec![
            InputBinding::key(KeyCode::D),
            InputBinding::key(KeyCode::Right),
        ]);
        
        // Common actions
        self.bind_action(Action::Jump, vec![InputBinding::key(KeyCode::Space)]);
        self.bind_action(Action::Attack, vec![
            InputBinding::mouse(MouseButton::Left),
            InputBinding::key(KeyCode::X),
        ]);
        self.bind_action(Action::Defend, vec![
            InputBinding::mouse(MouseButton::Right),
            InputBinding::key(KeyCode::Z),
        ]);
        self.bind_action(Action::Interact, vec![InputBinding::key(KeyCode::E)]);
        self.bind_action(Action::Pause, vec![InputBinding::key(KeyCode::Escape)]);
    }
    
    /// Update input state - call this once per frame
    pub fn update(&mut self, dt: f32) {
        // Clear previous frame state
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();
        self.mouse_just_pressed.clear();
        self.mouse_just_released.clear();
        self.actions_just_activated.clear();
        self.actions_just_deactivated.clear();
        
        // Update key state
        self.update_key_state();
        
        // Update mouse state
        self.update_mouse_state();
        
        // Update action state
        self.update_action_state();
        
        // Update input buffer
        self.update_input_buffer(dt);
    }
    
    fn update_key_state(&mut self) {
        // Check all possible keys (this is a simplified approach)
        let all_keys = [
            KeyCode::A, KeyCode::B, KeyCode::C, KeyCode::D, KeyCode::E, KeyCode::F,
            KeyCode::G, KeyCode::H, KeyCode::I, KeyCode::J, KeyCode::K, KeyCode::L,
            KeyCode::M, KeyCode::N, KeyCode::O, KeyCode::P, KeyCode::Q, KeyCode::R,
            KeyCode::S, KeyCode::T, KeyCode::U, KeyCode::V, KeyCode::W, KeyCode::X,
            KeyCode::Y, KeyCode::Z, KeyCode::Key0, KeyCode::Key1, KeyCode::Key2,
            KeyCode::Key3, KeyCode::Key4, KeyCode::Key5, KeyCode::Key6, KeyCode::Key7,
            KeyCode::Key8, KeyCode::Key9, KeyCode::Space, KeyCode::Enter, KeyCode::Escape,
            KeyCode::Backspace, KeyCode::Tab, KeyCode::LeftShift, KeyCode::RightShift,
            KeyCode::LeftControl, KeyCode::RightControl, KeyCode::LeftAlt, KeyCode::RightAlt,
            KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right,
        ];
        
        for &key in &all_keys {
            let is_down = is_key_down(key);
            let was_pressed = self.keys_pressed.contains(&key);
            
            if is_down && !was_pressed {
                self.keys_just_pressed.insert(key);
                self.keys_pressed.insert(key);
            } else if !is_down && was_pressed {
                self.keys_just_released.insert(key);
                self.keys_pressed.remove(&key);
            }
        }
    }
    
    fn update_mouse_state(&mut self) {
        let current_mouse_pos = mouse_position().into();
        self.mouse_delta = current_mouse_pos - self.mouse_position;
        self.mouse_position = current_mouse_pos;
        
        let mouse_wheel = mouse_wheel();
        self.scroll_delta = Vec2::new(mouse_wheel.0, mouse_wheel.1);
        
        let buttons = [MouseButton::Left, MouseButton::Right, MouseButton::Middle];
        
        for &button in &buttons {
            let is_down = is_mouse_button_down(button);
            let was_pressed = self.mouse_pressed.contains(&button);
            
            if is_down && !was_pressed {
                self.mouse_just_pressed.insert(button);
                self.mouse_pressed.insert(button);
            } else if !is_down && was_pressed {
                self.mouse_just_released.insert(button);
                self.mouse_pressed.remove(&button);
            }
        }
    }
    
    fn update_action_state(&mut self) {
        let mut new_active_actions = HashSet::new();
        
        for (action, bindings) in &self.bindings {
            let is_active = bindings.iter().any(|binding| self.is_binding_active(binding));
            
            if is_active {
                new_active_actions.insert(action.clone());
                
                if !self.actions_active.contains(action) {
                    self.actions_just_activated.insert(action.clone());
                    // Add to buffer
                    self.buffered_actions.insert(action.clone(), self.buffer_time);
                }
            } else if self.actions_active.contains(action) {
                self.actions_just_deactivated.insert(action.clone());
            }
        }
        
        self.actions_active = new_active_actions;
    }
    
    fn is_binding_active(&self, binding: &InputBinding) -> bool {
        match binding {
            InputBinding::Key(key_binding) => {
                // Check if main key is pressed
                if !self.keys_pressed.contains(&key_binding.key) {
                    return false;
                }
                
                // Check if all modifiers are pressed
                for modifier in &key_binding.modifiers {
                    if !self.keys_pressed.contains(modifier) {
                        return false;
                    }
                }
                
                true
            }
            InputBinding::Mouse(mouse_binding) => {
                self.mouse_pressed.contains(&mouse_binding.button)
            }
        }
    }
    
    fn update_input_buffer(&mut self, dt: f32) {
        // Decay buffered actions
        self.buffered_actions.retain(|_, time_left| {
            *time_left -= dt;
            *time_left > 0.0
        });
    }
    
    // Public API for querying input state
    
    /// Check if an action is currently active
    pub fn is_action_active(&self, action: &Action) -> bool {
        self.actions_active.contains(action)
    }
    
    /// Check if an action was just activated this frame
    pub fn is_action_just_activated(&self, action: &Action) -> bool {
        self.actions_just_activated.contains(action)
    }
    
    /// Check if an action was just deactivated this frame
    pub fn is_action_just_deactivated(&self, action: &Action) -> bool {
        self.actions_just_deactivated.contains(action)
    }
    
    /// Check if an action is in the input buffer (for timing-sensitive games)
    pub fn is_action_buffered(&self, action: &Action) -> bool {
        self.buffered_actions.contains_key(action)
    }
    
    /// Consume a buffered action (removes it from buffer)
    pub fn consume_buffered_action(&mut self, action: &Action) -> bool {
        self.buffered_actions.remove(action).is_some()
    }
    
    /// Get movement input as a Vec2 (normalized)
    pub fn get_movement_input(&self) -> Vec2 {
        let mut movement = Vec2::ZERO;
        
        if self.is_action_active(&Action::MoveUp) {
            movement.y -= 1.0;
        }
        if self.is_action_active(&Action::MoveDown) {
            movement.y += 1.0;
        }
        if self.is_action_active(&Action::MoveLeft) {
            movement.x -= 1.0;
        }
        if self.is_action_active(&Action::MoveRight) {
            movement.x += 1.0;
        }
        
        if movement != Vec2::ZERO {
            movement.normalize()
        } else {
            movement
        }
    }
    
    // Raw input queries (for when you need direct access)
    
    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }
    
    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.keys_just_pressed.contains(&key)
    }
    
    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.keys_just_released.contains(&key)
    }
    
    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }
    
    pub fn is_mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.mouse_just_pressed.contains(&button)
    }
    
    pub fn is_mouse_button_just_released(&self, button: MouseButton) -> bool {
        self.mouse_just_released.contains(&button)
    }
    
    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }
    
    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }
    
    pub fn scroll_delta(&self) -> Vec2 {
        self.scroll_delta
    }
    
    // Binding management
    
    /// Bind an action to multiple input bindings
    pub fn bind_action(&mut self, action: Action, bindings: Vec<InputBinding>) {
        self.bindings.insert(action, bindings);
    }
    
    /// Add a binding to an existing action
    pub fn add_binding(&mut self, action: Action, binding: InputBinding) {
        self.bindings.entry(action).or_insert_with(Vec::new).push(binding);
    }
    
    /// Remove all bindings for an action
    pub fn unbind_action(&mut self, action: &Action) {
        self.bindings.remove(action);
    }
    
    /// Clear all bindings
    pub fn clear_bindings(&mut self) {
        self.bindings.clear();
    }
    
    /// Set the input buffer time (in seconds)
    pub fn set_buffer_time(&mut self, time: f32) {
        self.buffer_time = time;
    }
    
    /// Get current bindings for an action
    pub fn get_bindings(&self, action: &Action) -> Option<&Vec<InputBinding>> {
        self.bindings.get(action)
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}