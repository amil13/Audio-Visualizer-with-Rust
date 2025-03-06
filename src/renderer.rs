//! This Rendeerer module renders the audio waveform in a window.
//! It manages a pixel buffer that represents the visual output 
//! and updates it based on the audio samples received.
//!
//! The `Renderer` struct:
//! - Maintains a pixel buffer for visualization output
//! - Handles normalization of audio samples to fit display
//! - Maps audio waveform data to visual representation
//! - Provides access to the rendered buffer for display

// Renderer struct
pub struct Renderer {
    buffer: Vec<u32>, // Buffer for pixel data
    width: usize,     // Width of rendering area
    height: usize,    // Height of rendering area
}

impl Renderer {
    // Constructor
    pub fn new(width: usize, height: usize) -> Self {
        Renderer {
            buffer: vec![0; width * height], // [all zeros] black buffer initialization
            width,
            height,
        }
    }

    // Update buffer with new audio samples
    pub fn update(&mut self, samples: &[f32]) {
        // Clear to black
        self.buffer.fill(0);
        
        // Draw center line if no audio
        if samples.is_empty() {
            let middle_y = self.height / 2;
            for x in 0..self.width {
                self.buffer[middle_y * self.width + x] = 0xFFFFFF; // White line
            }
            return;
        }
        
        // Calculate normalization scale
        let max_amplitude = samples.iter()
            .fold(0.0f32, |acc, &x| acc.max(x.abs())) // Get max amplitude
            .max(1e-3); // Prevent divide by zero
            
        let scale = (self.height as f32 / 2.0) / max_amplitude; 

        // Draw waveform
        for (x, &sample) in samples.iter().enumerate().take(self.width) {
            let y = ((sample * scale) + (self.height as f32 / 2.0)) as usize; // Y coordinate
            let y = y.clamp(0, self.height - 1); // Bound check
            self.buffer[y * self.width + x] = 0xFFFFFF; // White pixel
        }
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer // Return buffer reference
    }
}
