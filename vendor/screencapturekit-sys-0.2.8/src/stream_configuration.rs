use objc::{msg_send, runtime::Class, *};

use objc_foundation::INSObject;
use objc_id::Id;

use crate::os_types::{
    base::{CMTime, OSType, UInt32, BOOL},
    four_char_code::FourCharCode,
    geometry::CGRect,
    graphics::CGColor,
};

#[derive(Debug)]
pub struct UnsafeStreamConfigurationRef;
unsafe impl Message for UnsafeStreamConfigurationRef {}
impl From<UnsafeStreamConfiguration> for Id<UnsafeStreamConfigurationRef> {
    fn from(value: UnsafeStreamConfiguration) -> Self {
        let sys_ref = UnsafeStreamConfigurationRef::new();
        unsafe {
            let _: () = msg_send![sys_ref, setWidth: value.width];
            let _: () = msg_send![sys_ref, setHeight: value.height];
            let _: () = msg_send![sys_ref, setCapturesAudio: value.captures_audio];
            let _: () = msg_send![sys_ref, setSourceRect: value.source_rect];
            let _: () = msg_send![sys_ref, setDestinationRect: value.destination_rect];
            let _: () = msg_send![sys_ref, setPixelFormat: value.pixel_format];
            let _: () = msg_send![sys_ref, setMinimumFrameInterval: value.minimum_frame_interval];
            let _: () = msg_send![sys_ref, setScalesToFit: value.scales_to_fit];
            let _: () = msg_send![sys_ref, setShowsCursor: value.shows_cursor];
            // let _: () =
            // msg_send![sys_ref, setSetPreservesAspectRatio: value.preserves_aspect_ratio];
        }

        sys_ref
    }
}

impl INSObject for UnsafeStreamConfigurationRef {
    fn class() -> &'static Class {
        Class::get("SCStreamConfiguration")
                .expect("Missing SCStreamConfiguration class, check that the binary is linked with ScreenCaptureKit")
    }
}

#[derive(Debug)]
pub struct UnsafeStreamConfiguration {
    // The width of the output.
    pub width: UInt32,
    //   The height of the output.
    pub height: UInt32,
    // A boolean value that indicates whether to scale the output to fit the configured width and height.
    pub scales_to_fit: BOOL,
    // A rectangle that specifies the source area to capture.
    pub source_rect: CGRect,
    // A rectangle that specifies a destination into which to write the output.
    pub destination_rect: CGRect,
    // A Boolean value that determines if the stream preserves aspect ratio.
    pub preserves_aspect_ratio: BOOL,
    // Configuring Colors

    // A pixel format for sample buffers that a stream outputs.
    pub pixel_format: OSType,
    // A color matrix to apply to the output surface.
    pub color_matrix: String,
    // A color space to use for the output buffer.
    pub color_space_name: String,
    // A background color for the output.
    // Controlling Visibility
    pub background_color: CGColor,

    // A boolean value that determines whether the cursor is visible in the stream.
    pub shows_cursor: BOOL,
    // Optimizing Performance
    // The maximum number of frames for the queue to store.
    pub queue_depth: UInt32,
    // The desired minimum time between frame updates, in seconds.
    pub minimum_frame_interval: CMTime,
    // Configuring Audio
    // A boolean value that indicates whether to capture audio.
    pub captures_audio: BOOL,
    // The sample rate for audio capture.
    pub sample_rate: UInt32,
    // The number of audio channels to capture.
    pub channel_count: UInt32,
    // A boolean value that indicates whether to exclude a
    pub excludes_current_process_audio: BOOL,
}

impl Default for UnsafeStreamConfiguration {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            scales_to_fit: 0,
            preserves_aspect_ratio: 1,
            source_rect: Default::default(),
            destination_rect: Default::default(),
            pixel_format: FourCharCode::from_chars(*b"BGRA"),
            color_matrix: Default::default(),
            color_space_name: Default::default(),
            background_color: Default::default(),
            shows_cursor: Default::default(),
            queue_depth: Default::default(),
            minimum_frame_interval: Default::default(),
            captures_audio: Default::default(),
            sample_rate: Default::default(),
            channel_count: Default::default(),
            excludes_current_process_audio: Default::default(),
        }
    }
}

#[cfg(test)]
mod get_shareable_content {

    use super::*;
    #[test]
    fn test_from() {
        let _: Id<UnsafeStreamConfigurationRef> = UnsafeStreamConfiguration::default().into();
    }
}
