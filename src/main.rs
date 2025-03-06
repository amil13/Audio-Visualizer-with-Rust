//! This is an audio visualization application that displays real-time audio input as a waveform.
//!
//! IT uses `cpal` for audio capture from the default input device and `minifb` for 
//! rendering the visualization. The audio processing and rendering are handled through
//! implicit threading.
//!
//! The visualization shows the audio waveform as a white line on a black background,
//! with the amplitude normalized to the window height.

mod audio;
mod renderer;
mod visualizer;

fn main() {
    let audio_capture = audio::AudioCapture::new(4096);  // Audio Setup
    let mut renderer = renderer::Renderer::new(800, 600); // Renderer Setup
    let mut visualizer = visualizer::Visualizer::new("Audio Visualizer", 800, 600); // Visualizer Setup

    loop {
        let samples = audio_capture.get_samples(); // Capturing audio samples
        // println!("Got {} samples", samples.len()); // Debug print
        
        renderer.update(&samples); // pixel buffer update

        visualizer.render(renderer.get_buffer()); // render the pixel buffer to the window
        
        if !visualizer.is_open() { // Loop exit condition
            break;
        }
    }
}
