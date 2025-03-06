//! This visualization module handles the window display and rendering of the audio waveform.
//! It uses `minifb` for window management and buffer rendering. 
//! 
//!  `Visualizer` struct:
//! - Creates and manages the application window
//! - Handles window events like closing and escape key
//! - Renders the audio visualization buffer at ~60fps
//! - Provides window state information through `is_open()` 

use minifb::{Key, Window, WindowOptions};
use std::thread;
use std::time::Duration;

// Visualizer struct
pub struct Visualizer {
    window: Window, // window
}

impl Visualizer {
    // Constructor
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let window = Window::new(  // Window initialization
            title,
            width,
            height,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        Visualizer { window } // return instance of Visualizer
    }

    // To close with escape key
    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    // Render pixel buffer to the window
    pub fn render(&mut self, buffer: &[u32]) {
        self.window.update_with_buffer(buffer, self.window.get_size().0, self.window.get_size().1).unwrap();
        thread::sleep(Duration::from_millis(16)); // ~60fps
    }
}
