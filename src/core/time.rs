use macroquad::prelude::*;

/// Manages game time and provides utilities
pub struct TimeManager {
    delta_time: f32,
    time_scale: f32,
    total_time: f32,
    last_frame_time: f64,
    fps_counter: FPSCounter,
}

impl TimeManager {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            time_scale: 1.0,
            total_time: 0.0,
            last_frame_time: get_time(),
            fps_counter: FPSCounter::new(),
        }
    }
    
    pub fn update(&mut self) {
        let current_time = get_time();
        self.delta_time = ((current_time - self.last_frame_time) as f32) * self.time_scale;
        self.last_frame_time = current_time;
        self.total_time += self.delta_time;
        self.fps_counter.update();
    }
    
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }
    
    pub fn total_time(&self) -> f32 {
        self.total_time
    }
    
    pub fn time_scale(&self) -> f32 {
        self.time_scale
    }
    
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.0);
    }
    
    pub fn fps(&self) -> f32 {
        self.fps_counter.fps()
    }
}

struct FPSCounter {
    frame_count: u32,
    last_fps_time: f64,
    current_fps: f32,
}

impl FPSCounter {
    fn new() -> Self {
        Self {
            frame_count: 0,
            last_fps_time: get_time(),
            current_fps: 0.0,
        }
    }
    
    fn update(&mut self) {
        self.frame_count += 1;
        let current_time = get_time();
        
        if current_time - self.last_fps_time >= 1.0 {
            self.current_fps = self.frame_count as f32 / (current_time - self.last_fps_time) as f32;
            self.frame_count = 0;
            self.last_fps_time = current_time;
        }
    }
    
    fn fps(&self) -> f32 {
        self.current_fps
    }
}