use core_foundation::{
    base::{CFType, TCFType},
    boolean::CFBoolean,
    dictionary::CFMutableDictionary,
    number::CFNumber,
    string::CFString,
};
use core_graphics2::{
    color_space::{CGColorSpace, CGColorSpaceNames},
    display::CGDisplay,
    display_stream::*,
};
use core_video::pixel_buffer::{self, CVPixelBuffer};
use dispatch2::{Queue, QueueAttribute};
use io_surface::IOSurface;

fn main() {
    let display = CGDisplay::main();
    let output_width = display.pixels_wide();
    let output_height = display.pixels_high();
    let pixel_format = pixel_buffer::kCVPixelFormatType_420YpCbCr8BiPlanarFullRange;
    let mut properties = CFMutableDictionary::<CFString, CFType>::new();
    properties.add(&CGDisplayStreamProperties::ShowCursor.into(), &CFBoolean::true_value().as_CFType());
    if let Some(color_space) = CGColorSpace::from_name(&CGColorSpaceNames::SRGB.into()) {
        properties.add(&CGDisplayStreamProperties::ColorSpace.into(), &color_space.as_CFType());
    }
    properties.add(&CGDisplayStreamProperties::MinimumFrameTime.into(), &CFNumber::from(1.0 / 60.0).as_CFType());
    let ycbcr_matrix: CFString = CGDisplayStreamYCbCrMatrices::ITU_R_709_2.into();
    properties.add(&CGDisplayStreamProperties::YCbCrMatrix.into(), &ycbcr_matrix.as_CFType());
    let closure =
        move |status: CGDisplayStreamFrameStatus, timestamp: u64, surface: Option<IOSurface>, update: Option<CGDisplayStreamUpdate>| match status {
            CGDisplayStreamFrameStatus::Stopped => {
                println!("status: Stopped");
            }
            CGDisplayStreamFrameStatus::FrameComplete => {
                println!("status: {:?}, timestamp: {}", status, timestamp);
                if let Some(update) = update {
                    println!("refreshed rects: {:?}", update.rects(CGDisplayStreamUpdateRectType::RefreshedRects));
                    println!("moved rects: {:?}", update.rects(CGDisplayStreamUpdateRectType::MovedRects));
                    println!("dirty rects: {:?}", update.rects(CGDisplayStreamUpdateRectType::DirtyRects));
                    println!("reduced dirty rects: {:?}", update.rects(CGDisplayStreamUpdateRectType::ReducedDirtyRects));
                }
                if let Some(surface) = surface {
                    if let Ok(pixel_buffer) = CVPixelBuffer::from_io_surface(&surface, None) {
                        println!("pixel_buffer: {:?}", pixel_buffer);
                    }
                }
            }
            _ => {}
        };
    let queue = Queue::new("com.screen_capture.queue", QueueAttribute::Serial);
    if let Ok(display_stream) = CGDisplayStream::new_with_dispatch_queue(
        display.id,
        output_width,
        output_height,
        pixel_format as i32,
        &properties.to_immutable(),
        &queue,
        closure,
    ) {
        display_stream.start();
        std::thread::sleep(std::time::Duration::from_secs(10));
        display_stream.stop();
    }
}
