use windows_capture::graphics_capture_api::GraphicsCaptureApi;

pub fn is_supported() -> bool {
    GraphicsCaptureApi::is_supported().expect("Failed to check support")
}
