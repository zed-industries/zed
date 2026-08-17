use taffy::geometry::Size;

pub struct ImageContext {
    pub width: f32,
    pub height: f32,
}

pub fn image_measure_function(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    image_context: &ImageContext,
) -> taffy::geometry::Size<f32> {
    match (known_dimensions.width, known_dimensions.height) {
        (Some(width), Some(height)) => Size { width, height },
        (Some(width), None) => Size { width, height: (width / image_context.width) * image_context.height },
        (None, Some(height)) => Size { width: (height / image_context.height) * image_context.width, height },
        (None, None) => Size { width: image_context.width, height: image_context.height },
    }
}
