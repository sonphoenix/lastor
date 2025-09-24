use macroquad::prelude::*;

/// Represents a game action that can be triggered by various inputs
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Action {
    // Movement actions
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    
    // Common game actions
    Jump,
    Attack,
    Defend,
    Interact,
    Pause,
    
    // Custom actions (users can extend this)
    Custom(String),
}

impl Action {
    pub fn custom(name: &str) -> Self {
        Action::Custom(name.to_string())
    }
}

/// Different types of input bindings
#[derive(Debug, Clone)]
pub enum InputBinding {
    Key(KeyBinding),
    Mouse(MouseBinding),
}

#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub key: KeyCode,
    pub modifiers: Vec<KeyCode>, // For Ctrl+S, Alt+F4, etc.
}

impl KeyBinding {
    pub fn new(key: KeyCode) -> Self {
        Self {
            key,
            modifiers: vec![],
        }
    }
    
    pub fn with_modifier(mut self, modifier: KeyCode) -> Self {
        self.modifiers.push(modifier);
        self
    }
}

#[derive(Debug, Clone)]
pub struct MouseBinding {
    pub button: MouseButton,
}

impl MouseBinding {
    pub fn new(button: MouseButton) -> Self {
        Self { button }
    }
}

// Convenient constructors
impl InputBinding {
    pub fn key(key: KeyCode) -> Self {
        InputBinding::Key(KeyBinding::new(key))
    }
    
    pub fn key_with_modifier(key: KeyCode, modifier: KeyCode) -> Self {
        InputBinding::Key(KeyBinding::new(key).with_modifier(modifier))
    }
    
    pub fn mouse(button: MouseButton) -> Self {
        InputBinding::Mouse(MouseBinding::new(button))
    }
}