#[cfg(not(any(feature = "wayland", feature = "x11")))]
compile_error!("'wayland' or 'x11' feature must be enabled.");

use std::{env, sync::mpsc};

use anyhow::{anyhow, Result};

#[cfg(feature = "x11")]
use x11::X11Capturer;

use crate::{capturer::Options, frame::Frame, Target};

mod error;

#[cfg(feature = "wayland")]
mod wayland;

#[cfg(feature = "x11")]
mod x11;

#[cfg(feature = "wayland")]
use wayland::WaylandCapturer;

pub trait LinuxCapturerImpl {
    fn start_capture(&mut self);
    fn stop_capture(&mut self);
    fn target(&self) -> Option<&Target> {
        None
    }
}

pub struct LinuxCapturer {
    pub imp: Box<dyn LinuxCapturerImpl>,
}

type Type = mpsc::Sender<Result<Frame>>;

impl LinuxCapturer {
    pub fn new(options: &Options, tx: Type) -> Result<Self> {
        #[cfg(feature = "wayland")]
        if env::var("WAYLAND_DISPLAY").is_ok() {
            log::debug!("Creating new Wayland screen capturer.");
            return Ok(Self {
                imp: Box::new(WaylandCapturer::new(options, tx)?),
            });
        }

        #[cfg(feature = "x11")]
        if env::var("DISPLAY").is_ok() {
            log::debug!("Creating new X11 screen capturer.");
            return Ok(Self {
                imp: Box::new(X11Capturer::new(options, tx)?),
            });
        }

        #[cfg(all(feature = "wayland", feature = "x11"))]
        let error_msg = "Unsupported platform. Could not detect Wayland or X11 displays";
        #[cfg(all(not(feature = "wayland"), feature = "x11"))]
        let error_msg = "Unsupported platform. Could not detect X11 display. Enable the 'wayland' feature for Wayland support.";
        #[cfg(all(feature = "wayland", not(feature = "x11")))]
        let error_msg = "Unsupported platform. Could not detect wayland display. Enable the 'x11' feature for X11 support.";

        Err(anyhow!(error_msg))
    }
}

pub fn create_capturer(
    options: &Options,
    tx: mpsc::Sender<Result<Frame>>,
) -> Result<LinuxCapturer> {
    LinuxCapturer::new(options, tx)
}
