use std::{ffi::c_void, mem};

use self::shaders::ToUchar4;

use super::window::RenderContext;
use crate::{color::ColorU, scene::Layer, Scene};
use anyhow::{anyhow, Result};
use metal::{MTLResourceOptions, NSRange};
use shaders::ToFloat2 as _;

const SHADERS_METALLIB: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));
const INSTANCE_BUFFER_SIZE: u64 = 1024 * 1024;

pub struct Renderer {
    quad_pipeline_state: metal::RenderPipelineState,
    shadow_pipeline_state: metal::RenderPipelineState,
    unit_vertices: metal::Buffer,
    instances: metal::Buffer,
}

impl Renderer {
    pub fn new(device: &metal::DeviceRef, pixel_format: metal::MTLPixelFormat) -> Result<Self> {
        let library = device
            .new_library_with_data(SHADERS_METALLIB)
            .map_err(|message| anyhow!("error building metal library: {}", message))?;

        let unit_vertices = [
            (0., 0.).to_float2(),
            (1., 0.).to_float2(),
            (0., 1.).to_float2(),
            (0., 1.).to_float2(),
            (1., 0.).to_float2(),
            (1., 1.).to_float2(),
        ];
        let unit_vertices = device.new_buffer_with_data(
            unit_vertices.as_ptr() as *const c_void,
            (unit_vertices.len() * mem::size_of::<shaders::vector_float2>()) as u64,
            MTLResourceOptions::StorageModeManaged,
        );
        let instances =
            device.new_buffer(INSTANCE_BUFFER_SIZE, MTLResourceOptions::StorageModeManaged);

        Ok(Self {
            quad_pipeline_state: build_pipeline_state(
                device,
                &library,
                "quad",
                "quad_vertex",
                "quad_fragment",
                pixel_format,
            )?,
            shadow_pipeline_state: build_pipeline_state(
                device,
                &library,
                "shadow",
                "shadow_vertex",
                "shadow_fragment",
                pixel_format,
            )?,
            unit_vertices,
            instances,
        })
    }

    pub fn render(&mut self, scene: &Scene, ctx: &RenderContext) {
        ctx.command_encoder.set_viewport(metal::MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: ctx.drawable_size.x() as f64,
            height: ctx.drawable_size.y() as f64,
            znear: 0.0,
            zfar: 1.0,
        });

        for layer in scene.layers() {
            self.render_shadows(scene, layer, ctx);
            self.render_quads(scene, layer, ctx);
        }
    }

    fn render_shadows(&mut self, scene: &Scene, layer: &Layer, ctx: &RenderContext) {
        ctx.command_encoder
            .set_render_pipeline_state(&self.shadow_pipeline_state);
        ctx.command_encoder.set_vertex_buffer(
            shaders::GPUIShadowInputIndex_GPUIShadowInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        ctx.command_encoder.set_vertex_buffer(
            shaders::GPUIShadowInputIndex_GPUIShadowInputIndexShadows as u64,
            Some(&self.instances),
            0,
        );
        ctx.command_encoder.set_vertex_bytes(
            shaders::GPUIShadowInputIndex_GPUIShadowInputIndexUniforms as u64,
            mem::size_of::<shaders::GPUIUniforms>() as u64,
            [shaders::GPUIUniforms {
                viewport_size: ctx.drawable_size.to_float2(),
            }]
            .as_ptr() as *const c_void,
        );

        let batch_size = self.instances.length() as usize / mem::size_of::<shaders::GPUIShadow>();

        let buffer_contents = self.instances.contents() as *mut shaders::GPUIShadow;
        for shadow_batch in layer.shadows().chunks(batch_size) {
            for (ix, shadow) in shadow_batch.iter().enumerate() {
                let shape_bounds = shadow.bounds * scene.scale_factor();
                let shader_shadow = shaders::GPUIShadow {
                    origin: shape_bounds.origin().to_float2(),
                    size: shape_bounds.size().to_float2(),
                    corner_radius: shadow.corner_radius,
                    sigma: shadow.sigma,
                    color: shadow.color.to_uchar4(),
                };
                unsafe {
                    *(buffer_contents.offset(ix as isize)) = shader_shadow;
                }
            }
            self.instances.did_modify_range(NSRange {
                location: 0,
                length: (shadow_batch.len() * mem::size_of::<shaders::GPUIShadow>()) as u64,
            });

            ctx.command_encoder.draw_primitives_instanced(
                metal::MTLPrimitiveType::Triangle,
                0,
                6,
                shadow_batch.len() as u64,
            );
        }
    }

    fn render_quads(&mut self, scene: &Scene, layer: &Layer, ctx: &RenderContext) {
        ctx.command_encoder
            .set_render_pipeline_state(&self.quad_pipeline_state);
        ctx.command_encoder.set_vertex_buffer(
            shaders::GPUIQuadInputIndex_GPUIQuadInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        ctx.command_encoder.set_vertex_buffer(
            shaders::GPUIQuadInputIndex_GPUIQuadInputIndexQuads as u64,
            Some(&self.instances),
            0,
        );
        ctx.command_encoder.set_vertex_bytes(
            shaders::GPUIQuadInputIndex_GPUIQuadInputIndexUniforms as u64,
            mem::size_of::<shaders::GPUIUniforms>() as u64,
            [shaders::GPUIUniforms {
                viewport_size: ctx.drawable_size.to_float2(),
            }]
            .as_ptr() as *const c_void,
        );

        let batch_size = self.instances.length() as usize / mem::size_of::<shaders::GPUIQuad>();

        let buffer_contents = self.instances.contents() as *mut shaders::GPUIQuad;
        for quad_batch in layer.quads().chunks(batch_size) {
            for (ix, quad) in quad_batch.iter().enumerate() {
                let bounds = quad.bounds * scene.scale_factor();
                let border_width = quad.border.width * scene.scale_factor();
                let shader_quad = shaders::GPUIQuad {
                    origin: bounds.origin().to_float2(),
                    size: bounds.size().to_float2(),
                    background_color: quad
                        .background
                        .unwrap_or(ColorU::transparent_black())
                        .to_uchar4(),
                    border_top: border_width * (quad.border.top as usize as f32),
                    border_right: border_width * (quad.border.right as usize as f32),
                    border_bottom: border_width * (quad.border.bottom as usize as f32),
                    border_left: border_width * (quad.border.left as usize as f32),
                    border_color: quad
                        .border
                        .color
                        .unwrap_or(ColorU::transparent_black())
                        .to_uchar4(),
                    corner_radius: quad.corner_radius * scene.scale_factor(),
                };
                unsafe {
                    *(buffer_contents.offset(ix as isize)) = shader_quad;
                }
            }
            self.instances.did_modify_range(NSRange {
                location: 0,
                length: (quad_batch.len() * mem::size_of::<shaders::GPUIQuad>()) as u64,
            });

            ctx.command_encoder.draw_primitives_instanced(
                metal::MTLPrimitiveType::Triangle,
                0,
                6,
                quad_batch.len() as u64,
            );
        }
    }
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

    let descriptor = metal::RenderPipelineDescriptor::new();
    descriptor.set_label(label);
    descriptor.set_vertex_function(Some(vertex_fn.as_ref()));
    descriptor.set_fragment_function(Some(fragment_fn.as_ref()));
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();
    color_attachment.set_pixel_format(pixel_format);
    color_attachment.set_blending_enabled(true);
    color_attachment.set_rgb_blend_operation(metal::MTLBlendOperation::Add);
    color_attachment.set_alpha_blend_operation(metal::MTLBlendOperation::Add);
    color_attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::SourceAlpha);
    color_attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::SourceAlpha);
    color_attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
    color_attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);

    device
        .new_render_pipeline_state(&descriptor)
        .map_err(|message| anyhow!("could not create render pipeline state: {}", message))
}

mod shaders {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    use crate::{color::ColorU, geometry::vector::Vector2F};
    use std::mem;

    include!(concat!(env!("OUT_DIR"), "/shaders.rs"));

    pub trait ToFloat2 {
        fn to_float2(&self) -> vector_float2;
    }

    pub trait ToUchar4 {
        fn to_uchar4(&self) -> vector_uchar4;
    }

    impl ToFloat2 for (f32, f32) {
        fn to_float2(&self) -> vector_float2 {
            unsafe {
                let mut output = mem::transmute::<_, u32>(self.1.to_bits()) as vector_float2;
                output <<= 32;
                output |= mem::transmute::<_, u32>(self.0.to_bits()) as vector_float2;
                output
            }
        }
    }

    impl ToFloat2 for Vector2F {
        fn to_float2(&self) -> vector_float2 {
            unsafe {
                let mut output = mem::transmute::<_, u32>(self.y().to_bits()) as vector_float2;
                output <<= 32;
                output |= mem::transmute::<_, u32>(self.x().to_bits()) as vector_float2;
                output
            }
        }
    }

    impl ToUchar4 for ColorU {
        fn to_uchar4(&self) -> vector_uchar4 {
            let mut vec = self.a as vector_uchar4;
            vec <<= 8;
            vec |= self.b as vector_uchar4;
            vec <<= 8;
            vec |= self.g as vector_uchar4;
            vec <<= 8;
            vec |= self.r as vector_uchar4;
            vec
        }
    }
}
