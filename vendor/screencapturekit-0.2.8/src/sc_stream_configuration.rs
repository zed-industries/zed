use crate::sc_types::base::CMTime;
use crate::sc_types::four_char_code::FourCharCode;
use crate::sc_types::geometry::CGRect;
use crate::sc_types::graphics::CGColor;
use screencapturekit_sys::{
    os_types::rc::Id,
    stream_configuration::{UnsafeStreamConfiguration, UnsafeStreamConfigurationRef},
};

pub static PIXEL_FORMATS: [PixelFormat; 4] = [
    PixelFormat::ARGB8888,
    PixelFormat::ARGB2101010,
    PixelFormat::YCbCr420f,
    PixelFormat::YCbCr420v,
];

#[derive(Copy, Clone, Debug, Default)]
pub enum PixelFormat {
    ARGB8888,
    ARGB2101010,
    #[default]
    YCbCr420v,
    YCbCr420f,
}

impl From<FourCharCode> for PixelFormat {
    fn from(val: FourCharCode) -> Self {
        let code_str = val.to_string();
        match code_str.as_str() {
            "BGRA" => PixelFormat::ARGB8888,
            "l10r" => PixelFormat::ARGB2101010,
            "420v" => PixelFormat::YCbCr420v,
            "420f" => PixelFormat::YCbCr420f,
            _ => unreachable!(),
        }
    }
}
impl From<PixelFormat> for FourCharCode {
    fn from(val: PixelFormat) -> Self {
        match val {
            PixelFormat::ARGB8888 => FourCharCode::from_chars(*b"BGRA"),
            PixelFormat::ARGB2101010 => FourCharCode::from_chars(*b"l10r"),
            PixelFormat::YCbCr420v => FourCharCode::from_chars(*b"420v"),
            PixelFormat::YCbCr420f => FourCharCode::from_chars(*b"420f"),
        }
    }
}

pub struct Size {
    //   The width of the output.
    pub width: u32,
    //   The height of the output.
    pub height: u32,
    // A boolean value that indicates whether to scale the output to fit the configured width and height.
    pub scales_to_fit: bool,
}

#[derive(Debug)]
pub struct SCStreamConfiguration {
    //   The width of the output.
    pub width: u32,
    //   The height of the output.
    pub height: u32,
    // A boolean value that indicates whether to scale the output to fit the configured width and height.
    pub scales_to_fit: bool,
    // A rectangle that specifies the source area to capture.
    pub source_rect: CGRect,
    // A rectangle that specifies a destination into which to write the output.
    pub destination_rect: CGRect,
    // A boolean value that determines whether the cursor is visible in the stream.
    pub shows_cursor: bool,

    // A Boolean value that determines if the stream preserves aspect ratio.
    pub preserves_aspect_ratio: bool,
    // Optimizing Performance
    // The maximum number of frames for the queue to store.
    pub queue_depth: u32,
    // The desired minimum time between frame updates, in seconds.
    pub minimum_frame_interval: CMTime,
    // Configuring Audi
    // A boolean value that indicates whether to capture audio.
    pub captures_audio: bool,
    // The sample rate for audio capture.
    pub sample_rate: u32,
    // The number of audio channels to capture.
    pub channel_count: u32,
    // A boolean value that indicates whether to exclude a
    pub excludes_current_process_audio: bool,
    // Configuring Colors
    // A pixel format for sample buffers that a stream outputs.
    pub pixel_format: PixelFormat,
    // A color matrix to apply to the output surface.
    pub color_matrix: &'static str,
    // A color space to use for the output buffer.
    pub color_space_name: &'static str,
    // A background color for the output.
    // Controlling Visibility
    pub background_color: CGColor,
}

impl Default for SCStreamConfiguration {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            scales_to_fit: Default::default(),
            source_rect: Default::default(),
            destination_rect: Default::default(),
            // Apple docs set this to true by default, but we are consistent with UnsafeStreamConfiguration.
            shows_cursor: Default::default(),
            preserves_aspect_ratio: true,
            queue_depth: Default::default(),
            minimum_frame_interval: Default::default(),
            captures_audio: Default::default(),
            sample_rate: Default::default(),
            channel_count: Default::default(),
            excludes_current_process_audio: Default::default(),
            pixel_format: PixelFormat::ARGB8888,
            color_matrix: Default::default(),
            color_space_name: Default::default(),
            background_color: Default::default(),
        }
    }
}

impl SCStreamConfiguration {
    pub fn from_size(width: u32, height: u32, scales_to_fit: bool) -> Self {
        Self {
            width,
            height,
            scales_to_fit,
            ..Default::default()
        }
    }
}

impl From<SCStreamConfiguration> for UnsafeStreamConfiguration {
    fn from(value: SCStreamConfiguration) -> Self {
        UnsafeStreamConfiguration {
            width: value.width,
            height: value.height,
            scales_to_fit: value.scales_to_fit as i8,
            source_rect: value.source_rect,
            destination_rect: value.destination_rect,
            preserves_aspect_ratio: value.preserves_aspect_ratio as i8,
            pixel_format: value.pixel_format.into(),
            color_matrix: value.color_matrix.into(),
            color_space_name: value.color_space_name.into(),
            background_color: value.background_color,
            shows_cursor: value.shows_cursor as i8,
            queue_depth: value.queue_depth,
            minimum_frame_interval: value.minimum_frame_interval,
            captures_audio: value.captures_audio as i8,
            sample_rate: value.sample_rate,
            channel_count: value.channel_count,
            excludes_current_process_audio: value.excludes_current_process_audio as i8,
        }
    }
}

impl From<SCStreamConfiguration> for Id<UnsafeStreamConfigurationRef> {
    fn from(value: SCStreamConfiguration) -> Self {
        let unsafe_config: UnsafeStreamConfiguration = value.into();
        unsafe_config.into()
    }
}

#[cfg(test)]
mod get_configuration {

    use super::*;
    #[test]
    fn test_configuration() {
        SCStreamConfiguration::from_size(100, 100, false);
    }
}
