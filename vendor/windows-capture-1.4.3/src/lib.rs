//! # Windows Capture Rust Library
//!
//! **Windows Capture** is a highly efficient Rust and Python library that
//! enables you to effortlessly capture the screen using the Graphics Capture
//! API. This library allows you to easily capture the screen of your
//! Windows-based computer and use it for various purposes, such as creating
//! instructional videos, taking screenshots, or recording your gameplay. With
//! its intuitive interface and robust functionality, Windows-Capture is an
//! excellent choice for anyone looking for a reliable and easy-to-use screen
//! capturing solution.
//!
//! ## Features
//!
//! - Only Updates The Frame When Required.
//! - High Performance.
//! - Easy To Use.
//! - Latest Screen Capturing API.
//!
//! ## Installation
//!
//! Add this library to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! windows-capture = "1.4.3"
//! ```
//! or run this command
//!
//! ```text
//! cargo add windows-capture
//! ```
//!
//! ## Usage
//!
//! ```no_run
//! use std::{
//!     io::{self, Write},
//!     time::Instant,
//! };
//!
//! use windows_capture::{
//!     capture::{Context, GraphicsCaptureApiHandler},
//!     encoder::{AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder},
//!     frame::Frame,
//!     graphics_capture_api::InternalCaptureControl,
//!     monitor::Monitor,
//!     settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
//! };
//!
//! // Handles capture events.
//! struct Capture {
//!     // The video encoder that will be used to encode the frames.
//!     encoder: Option<VideoEncoder>,
//!     // To measure the time the capture has been running
//!     start: Instant,
//! }
//!
//! impl GraphicsCaptureApiHandler for Capture {
//!     // The type of flags used to get the values from the settings.
//!     type Flags = String;
//!
//!     // The type of error that can be returned from `CaptureControl` and `start` functions.
//!     type Error = Box<dyn std::error::Error + Send + Sync>;
//!
//!     // Function that will be called to create a new instance. The flags can be passed from settings.
//!     fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
//!         println!("Created with Flags: {}", ctx.flags);
//!
//!         let encoder = VideoEncoder::new(
//!             VideoSettingsBuilder::new(1920, 1080),
//!             AudioSettingsBuilder::default().disabled(true),
//!             ContainerSettingsBuilder::default(),
//!             "video.mp4",
//!         )?;
//!
//!         Ok(Self {
//!             encoder: Some(encoder),
//!             start: Instant::now(),
//!         })
//!     }
//!
//!     // Called every time a new frame is available.
//!     fn on_frame_arrived(
//!         &mut self,
//!         frame: &mut Frame,
//!         capture_control: InternalCaptureControl,
//!     ) -> Result<(), Self::Error> {
//!         print!(
//!             "\rRecording for: {} seconds",
//!             self.start.elapsed().as_secs()
//!         );
//!         io::stdout().flush()?;
//!
//!         // Send the frame to the video encoder
//!         self.encoder.as_mut().unwrap().send_frame(frame)?;
//!
//!         // Note: The frame has other uses too, for example, you can save a single frame to a file, like this:
//!         // frame.save_as_image("frame.png", ImageFormat::Png)?;
//!         // Or get the raw data like this so you have full control:
//!         // let data = frame.buffer()?;
//!
//!         // Stop the capture after 6 seconds
//!         if self.start.elapsed().as_secs() >= 6 {
//!             // Finish the encoder and save the video.
//!             self.encoder.take().unwrap().finish()?;
//!
//!             capture_control.stop();
//!
//!             // Because there wasn't any new lines in previous prints
//!             println!();
//!         }
//!
//!         Ok(())
//!     }
//!
//!     // Optional handler called when the capture item (usually a window) closes.
//!     fn on_closed(&mut self) -> Result<(), Self::Error> {
//!         println!("Capture session ended");
//!
//!         Ok(())
//!     }
//! }
//!
//!  // Gets the foreground window, refer to the docs for other capture items
//!  let primary_monitor = Monitor::primary().expect("There is no primary monitor");
//!
//!  let settings = Settings::new(
//!      // Item to capture
//!      primary_monitor,
//!      // Capture cursor settings
//!      CursorCaptureSettings::Default,
//!      // Draw border settings
//!      DrawBorderSettings::Default,
//!      // The desired color format for the captured frame.
//!      ColorFormat::Rgba8,
//!      // Additional flags for the capture settings that will be passed to user defined `new` function.
//!      "Yea this works".to_string(),
//!  );
//!
//!  // Starts the capture and takes control of the current thread.
//!  // The errors from handler trait will end up here
//!  Capture::start(settings).expect("Screen capture failed");
//! ```
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)] // Should update as soon as possible

/// Contains the main capture functionality, including the `WindowsCaptureHandler` trait and related types.
pub mod capture;
/// Internal module for Direct3D 11 related functionality.
mod d3d11;
/// Contains the encoder functionality for encoding captured frames.
pub mod encoder;
/// Contains the `Frame` struct and related types for representing captured frames.
pub mod frame;
/// Contains the types and functions related to the Graphics Capture API.
pub mod graphics_capture_api;
/// Contains the functionality for working with monitors and screen information.
pub mod monitor;
/// Contains the `Settings` struct and related types for configuring the capture settings.
pub mod settings;
/// Contains the functionality for working with windows and capturing specific windows.
pub mod window;
