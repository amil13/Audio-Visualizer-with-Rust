# Audio Visualizer Project - Real-Time Music Visualization

**Author:** Amil Shrivastava

## Introduction

The **Audio Visualizer Project** creates a very simple and basic real-time visual representations of audio input using **cpal** and **minifb** crates. This project transforms audio into dynamic visual patterns using:

- **cpal**: Handles real-time audio input from the default audio device
- **minifb**: Manages window creation and rendering of the visualization

This visualizer creates a basic visual display synchronized with audio input, displaying a waveform in real-time. It demonstrates the integration of **Rust**, **Audio Processing**, and **Real-Time Visualization** to build an interactive audio visualization tool.
 *Additionally, it serves as a demonstration of my skills and exposure to rust and its applications.*

## Demo

 ![Demo](GIF/av_gif.gif)

## Prerequisites

Before running the project, ensure you have:

- **Rust** installed on your system
- **Cargo** for managing Rust packages and building the project
- **Audio input device** microphone

## How to Run

1. Clone the repository
2. Navigate to the project directory
3. Build and run the project using Cargo:
   ```bash
   cargo run
   ```
## To Stop

Close the application window or press the `Escape` key to exit.