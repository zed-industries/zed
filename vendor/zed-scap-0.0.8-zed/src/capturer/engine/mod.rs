use std::sync::mpsc;

use anyhow::Result;

use super::Options;
use crate::{frame::Frame, Target};

#[cfg(target_os = "macos")]
pub mod mac;

#[cfg(target_os = "windows")]
mod win;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod linux;

#[cfg(target_os = "macos")]
pub type ChannelItem = (
    screencapturekit::cm_sample_buffer::CMSampleBuffer,
    screencapturekit::sc_output_handler::SCStreamOutputType,
);
#[cfg(not(target_os = "macos"))]
pub type ChannelItem = Frame;

pub fn get_output_frame_size(options: &Options) -> [u32; 2] {
    #[cfg(target_os = "macos")]
    {
        mac::get_output_frame_size(options)
    }

    #[cfg(target_os = "windows")]
    {
        win::get_output_frame_size(options)
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        // TODO: How to calculate this on Linux?
        return [0, 0];
    }
}

pub struct Engine {
    options: Options,
    target: Option<Target>,
    #[cfg(target_os = "macos")]
    mac: screencapturekit::sc_stream::SCStream,
    #[cfg(target_os = "macos")]
    error_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,

    #[cfg(target_os = "windows")]
    win: win::WCStream,

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    linux: linux::LinuxCapturer,
}

impl Engine {
    pub fn new(options: &Options, tx: mpsc::Sender<Result<ChannelItem>>) -> Result<Engine> {
        #[cfg(target_os = "macos")]
        {
            let error_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let (mac, target) = mac::create_capturer(options, tx, error_flag.clone());

            Ok(Engine {
                mac,
                error_flag,
                options: (*options).clone(),
                target: Some(target),
            })
        }

        #[cfg(target_os = "windows")]
        {
            let (win, target) = win::create_capturer(&options, tx);
            Ok(Engine {
                win,
                options: (*options).clone(),
                target: Some(target),
            })
        }

        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        {
            use linux::LinuxCapturerImpl;
            let linux = linux::create_capturer(&options, tx)?;
            let target = linux.imp.target().cloned();
            Ok(Engine {
                linux,
                options: (*options).clone(),
                target,
            })
        }
    }

    pub fn start(&mut self) {
        #[cfg(target_os = "macos")]
        {
            // self.mac.add_output(Capturer::new(tx));
            self.mac.start_capture().expect("Failed to start capture");
        }

        #[cfg(target_os = "windows")]
        {
            self.win.start_capture();
        }

        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        {
            self.linux.imp.start_capture();
        }
    }

    pub fn stop(&mut self) {
        #[cfg(target_os = "macos")]
        {
            self.mac.stop_capture().expect("Failed to stop capture");
        }

        #[cfg(target_os = "windows")]
        {
            self.win.stop_capture();
        }

        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        {
            self.linux.imp.stop_capture();
        }
    }

    pub fn get_output_frame_size(&mut self) -> [u32; 2] {
        get_output_frame_size(&self.options)
    }

    pub fn process_channel_item(&self, data: ChannelItem) -> Option<Frame> {
        #[cfg(target_os = "macos")]
        {
            mac::process_sample_buffer(data.0, data.1, self.options.output_type)
        }
        #[cfg(not(target_os = "macos"))]
        Some(data)
    }
    pub fn target(&self) -> Option<&Target> {
        self.target.as_ref()
    }
}
