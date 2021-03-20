use anyhow::{anyhow, Result};

use crate::Scene;

use super::window::RenderContext;

const SHADERS_METALLIB: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));

pub struct Renderer {
    quad_pipeline_state: metal::RenderPipelineState,
}

impl Renderer {
    pub fn new(device: &metal::DeviceRef, pixel_format: metal::MTLPixelFormat) -> Result<Self> {
        let library = device
            .new_library_with_data(SHADERS_METALLIB)
            .map_err(|message| anyhow!("error building metal library: {}", message))?;

        Ok(Self {
            quad_pipeline_state: build_pipeline_state(
                device,
                &library,
                "quad",
                "quad_vertex",
                "quad_fragment",
                pixel_format,
            )?,
        })
    }

    pub fn render(&mut self, scene: &Scene, ctx: RenderContext) {}
}

fn build_pipeline_state(
    device: &metal::DeviceRef,
    library: &metal::LibraryRef,
    label: &str,
    vertex_fn_name: &str,
    fragment_fn_name: &str,
    pixel_format: metal::MTLPixelFormat,
) -> Result<metal::RenderPipelineState> {
    let vertex_fn = library
        .get_function(vertex_fn_name, None)
        .map_err(|message| anyhow!("error locating vertex function: {}", message))?;
    let fragment_fn = library
        .get_function(fragment_fn_name, None)
        .map_err(|message| anyhow!("error locating fragment function: {}", message))?;

    let mut descriptor = metal::RenderPipelineDescriptor::new();
    descriptor.set_label(label);
    descriptor.set_vertex_function(Some(vertex_fn.as_ref()));
    descriptor.set_fragment_function(Some(fragment_fn.as_ref()));
    descriptor
        .color_attachments()
        .object_at(0)
        .unwrap()
        .set_pixel_format(pixel_format);

    device
        .new_render_pipeline_state(&descriptor)
        .map_err(|message| anyhow!("could not create render pipeline state: {}", message))
}
