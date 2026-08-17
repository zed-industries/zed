use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::{cmp, sync::Arc};

use pixelformat::get_pts_in_nanoseconds;
use screencapturekit::{
    cm_sample_buffer::CMSampleBuffer,
    sc_content_filter::{InitParams, SCContentFilter},
    sc_error_handler::StreamErrorHandler,
    sc_output_handler::{SCStreamOutputType, StreamOutput},
    sc_shareable_content::SCShareableContent,
    sc_stream::SCStream,
    sc_stream_configuration::{PixelFormat, SCStreamConfiguration},
    sc_types::SCFrameStatus,
};
use screencapturekit_sys::os_types::base::{CMTime, CMTimeScale};
use screencapturekit_sys::os_types::geometry::{CGPoint, CGRect, CGSize};

use crate::frame::{Frame, FrameType};
use crate::targets::Target;
use crate::{
    capturer::{Area, Options, Point, Resolution, Size},
    frame::BGRAFrame,
    targets,
};

use super::ChannelItem;

mod apple_sys;
mod pixel_buffer;
mod pixelformat;

pub use pixel_buffer::PixelBuffer;

struct ErrorHandler {
    error_flag: Arc<AtomicBool>,
}

impl StreamErrorHandler for ErrorHandler {
    fn on_error(&self) {
        log::error!("Screen capture error occurred.");
        self.error_flag
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

pub struct Capturer {
    pub tx: mpsc::Sender<anyhow::Result<ChannelItem>>,
}

impl Capturer {
    pub fn new(tx: mpsc::Sender<anyhow::Result<ChannelItem>>) -> Self {
        Capturer { tx }
    }
}

impl StreamOutput for Capturer {
    fn did_output_sample_buffer(&self, sample: CMSampleBuffer, of_type: SCStreamOutputType) {
        self.tx.send(Ok((sample, of_type))).unwrap_or(());
    }
}

pub fn create_capturer(
    options: &Options,
    tx: mpsc::Sender<anyhow::Result<ChannelItem>>,
    error_flag: Arc<AtomicBool>,
) -> (SCStream, Target) {
    // If no target is specified, capture the main display
    let target = options
        .target
        .clone()
        .unwrap_or_else(|| Target::Display(targets::get_main_display().unwrap()));

    let sc_shareable_content = SCShareableContent::current();

    let params = match &target {
        Target::Window(window) => {
            // Get SCWindow from window id
            let sc_window = sc_shareable_content
                .windows
                .into_iter()
                .find(|sc_win| sc_win.window_id == window.id)
                .unwrap();

            // Return a DesktopIndependentWindow
            // https://developer.apple.com/documentation/screencapturekit/sccontentfilter/3919804-init
            InitParams::DesktopIndependentWindow(sc_window)
        }
        Target::Display(display) => {
            // Get SCDisplay from display id
            let sc_display = sc_shareable_content
                .displays
                .into_iter()
                .find(|sc_dis| sc_dis.display_id == display.id)
                .unwrap();

            match &options.excluded_targets {
                None => InitParams::Display(sc_display),
                Some(excluded_targets) => {
                    let excluded_windows = sc_shareable_content
                        .windows
                        .into_iter()
                        .filter(|window| {
                            excluded_targets
                                .iter()
                                .any(|excluded_target| match excluded_target {
                                    Target::Window(excluded_window) => {
                                        excluded_window.id == window.window_id
                                    }
                                    _ => false,
                                })
                        })
                        .collect();

                    InitParams::DisplayExcludingWindows(sc_display, excluded_windows)
                }
            }
        }
    };

    let filter = SCContentFilter::new(params);

    let crop_area = get_crop_area(options);

    let source_rect = CGRect {
        origin: CGPoint {
            x: crop_area.origin.x,
            y: crop_area.origin.y,
        },
        size: CGSize {
            width: crop_area.size.width,
            height: crop_area.size.height,
        },
    };

    let pixel_format = match options.output_type {
        FrameType::YUVFrame => PixelFormat::YCbCr420v,
        FrameType::BGR0 => PixelFormat::ARGB8888,
        FrameType::RGB => PixelFormat::ARGB8888,
        FrameType::BGRAFrame => PixelFormat::ARGB8888,
    };

    let [width, height] = get_output_frame_size(options);

    let stream_config = SCStreamConfiguration {
        width,
        height,
        source_rect,
        pixel_format,
        shows_cursor: options.show_cursor,
        minimum_frame_interval: CMTime {
            value: 1,
            timescale: options.fps as CMTimeScale,
            epoch: 0,
            flags: 1,
        },
        ..Default::default()
    };

    let mut stream = SCStream::new(filter, stream_config, ErrorHandler { error_flag });
    stream.add_output(Capturer::new(tx), SCStreamOutputType::Screen);

    (stream, target)
}

pub fn get_output_frame_size(options: &Options) -> [u32; 2] {
    let target = options
        .target
        .clone()
        .unwrap_or_else(|| Target::Display(targets::get_main_display().unwrap()));

    let scale_factor = targets::get_scale_factor(&target);
    let source_rect = get_crop_area(options);

    // Calculate the output height & width based on the required resolution
    // Output width and height need to be multiplied by scale (or dpi)
    let mut output_width = (source_rect.size.width as u32) * (scale_factor as u32);
    let mut output_height = (source_rect.size.height as u32) * (scale_factor as u32);
    // 1200x800
    match options.output_resolution {
        Resolution::Captured => {}
        _ => {
            let [resolved_width, resolved_height] = options
                .output_resolution
                .value((source_rect.size.width as f32) / (source_rect.size.height as f32));
            // 1280 x 853
            output_width = cmp::min(output_width, resolved_width);
            output_height = cmp::min(output_height, resolved_height);
        }
    }

    output_width -= output_width % 2;
    output_height -= output_height % 2;

    [output_width, output_height]
}

pub fn get_crop_area(options: &Options) -> Area {
    let target = options
        .target
        .clone()
        .unwrap_or_else(|| Target::Display(targets::get_main_display().unwrap()));

    let (width, height) = targets::get_target_dimensions(&target);

    options
        .crop_area
        .as_ref()
        .map(|val| {
            let input_width = val.size.width + (val.size.width % 2.0);
            let input_height = val.size.height + (val.size.height % 2.0);

            Area {
                origin: Point {
                    x: val.origin.x,
                    y: val.origin.y,
                },
                size: Size {
                    width: input_width,
                    height: input_height,
                },
            }
        })
        .unwrap_or_else(|| Area {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: width as f64,
                height: height as f64,
            },
        })
}

pub fn process_sample_buffer(
    sample: CMSampleBuffer,
    of_type: SCStreamOutputType,
    output_type: FrameType,
) -> Option<Frame> {
    if let SCStreamOutputType::Screen = of_type {
        let frame_status = &sample.frame_status;

        match frame_status {
            SCFrameStatus::Complete | SCFrameStatus::Started => unsafe {
                return Some(match output_type {
                    FrameType::YUVFrame => {
                        let yuvframe = pixelformat::create_yuv_frame(sample).unwrap();
                        Frame::YUVFrame(yuvframe)
                    }
                    FrameType::RGB => {
                        let rgbframe = pixelformat::create_rgb_frame(sample).unwrap();
                        Frame::RGB(rgbframe)
                    }
                    FrameType::BGR0 => {
                        let bgrframe = pixelformat::create_bgr_frame(sample).unwrap();
                        Frame::BGR0(bgrframe)
                    }
                    FrameType::BGRAFrame => {
                        let bgraframe = pixelformat::create_bgra_frame(sample).unwrap();
                        Frame::BGRA(bgraframe)
                    }
                });
            },
            SCFrameStatus::Idle => {
                // Quick hack - just send an empty frame, and the caller can figure out how to handle it
                if let FrameType::BGRAFrame = output_type {
                    return Some(Frame::BGRA(BGRAFrame {
                        display_time: get_pts_in_nanoseconds(&sample),
                        width: 0,
                        height: 0,
                        data: vec![],
                    }));
                }
            }
            _ => {}
        }
    }

    None
}
