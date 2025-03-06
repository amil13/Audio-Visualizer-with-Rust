//! This audio capture module handles the real-time audio input from the default device.
//!
//! It uses `cpal` for audio device management and capture, and `ringbuf` for thread-safe
//! buffering of audio samples between the capture and visualization threads.
//!
//! `AudioCapture` struct:
//! - Initializes the default audio input device
//! - Creates a ring buffer for sample storage
//! - Continuously captures audio in a separate thread
//! - Provides access to captured samples through `get_samples()`

use cpal::traits::{DeviceTrait, HostTrait};
use ringbuf::producer::Producer;
use ringbuf::storage::Heap;
use ringbuf::traits::Consumer;
use ringbuf::traits::Split;
use ringbuf::wrap::caching::Caching;
use ringbuf::HeapRb;
use ringbuf::SharedRb;
use std::sync::{Arc, Mutex};

// Audio data capture struct
pub struct AudioCapture {
    consumer: Arc<Mutex<Caching<Arc<SharedRb<Heap<f32>>>, false, true>>>, // Consumer end of ring buffer
    _stream: cpal::Stream, // Audio stream, unused but kept alive through ownership
}
use cpal::traits::StreamTrait;

impl AudioCapture {
    // Constructor
    pub fn new(buffer_size: usize) -> Self {
        let rb = HeapRb::new(buffer_size); // Ring buffer initialization
        let (producer, consumer) = rb.split(); // producer and consumer ends of the rb
        let consumer = Arc::new(Mutex::new(consumer)); // Arc and Mutex for thread safety
        let producer = Arc::new(Mutex::new(producer)); // Arc and Mutex for thread safety

        let host = cpal::default_host(); // Audio host initialization

        let device = host.default_input_device().expect("No input device"); // Get default input with error handling

        let config = device.default_input_config().unwrap().config(); // Get default config for device

        // Callback to feed producer end of ring buffer
        let feedProducerCallback = move |data: &[f32], _: &_| {
            let mut producer = producer.lock().unwrap();
            for &sample in data {
                let _ = producer.try_push(sample); // Push each sample to ring buffer
            }
        };

        // Initialize audio stream
        let stream = device
            .build_input_stream(
                &config,
                feedProducerCallback, // Callback
                |err| eprintln!("Audio stream error: {}", err), // Error callback
                None,
            )
            .unwrap();

        stream.play().unwrap(); // Start capturing audio

        Self {
            consumer,
            _stream: stream,
        }
    }
    
    // Deal with consumer end of ring buffe
    pub fn get_samples(&self) -> Vec<f32> {
        let mut consumer = self.consumer.lock().unwrap(); // Lock consumer for thread safety
        let mut samples = Vec::new(); // initialize dynamic array
        while let Some(sample) = consumer.try_pop() {
            samples.push(sample);
        }
        samples // returnin samples
    }
}
