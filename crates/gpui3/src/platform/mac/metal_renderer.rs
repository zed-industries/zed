// use cocoa::{
//     base::{NO, YES},
//     foundation::NSUInteger,
//     quartzcore::AutoresizingMask,
// };
// use core_foundation::base::TCFType;
// use foreign_types::ForeignTypeRef;
// use log::warn;
// use media::core_video::{self, CVMetalTextureCache};
// use metal::{CommandQueue, MTLPixelFormat, MTLResourceOptions, NSRange};
// use objc::{self, msg_send, sel, sel_impl};
// use shaders::ToFloat2 as _;
// use std::{collections::HashMap, ffi::c_void, iter::Peekable, mem, ptr, sync::Arc, vec};

// use crate::{Quad, Scene};

// const SHADERS_METALLIB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));
// const BUFFER_SIZE: usize = 8192 * 1024; // This is an arbitrary decision. There's probably a more optimal value.

// pub struct Renderer {
//     layer: metal::MetalLayer,
//     command_queue: CommandQueue,
//     quad_pipeline_state: metal::RenderPipelineState,
//     buffer: metal::Buffer,
// }

// impl Renderer {
//     pub fn new(is_opaque: bool, fonts: Arc<dyn platform::FontSystem>) -> Self {
//         const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::BGRA8Unorm;

//         let device: metal::Device = if let Some(device) = metal::Device::system_default() {
//             device
//         } else {
//             log::error!("unable to access a compatible graphics device");
//             std::process::exit(1);
//         };

//         let layer = metal::MetalLayer::new();
//         layer.set_device(&device);
//         layer.set_pixel_format(PIXEL_FORMAT);
//         layer.set_presents_with_transaction(true);
//         layer.set_opaque(is_opaque);
//         unsafe {
//             let _: () = msg_send![&*layer, setAllowsNextDrawableTimeout: NO];
//             let _: () = msg_send![&*layer, setNeedsDisplayOnBoundsChange: YES];
//             let _: () = msg_send![
//                 &*layer,
//                 setAutoresizingMask: AutoresizingMask::WIDTH_SIZABLE
//                     | AutoresizingMask::HEIGHT_SIZABLE
//             ];
//         }

//         let library = device
//             .new_library_with_data(SHADERS_METALLIB)
//             .expect("error building metal library");

//         let buffer = device.new_buffer(BUFFER_SIZE as u64, MTLResourceOptions::StorageModeManaged);

//         let quad_pipeline_state = build_pipeline_state(
//             &device,
//             &library,
//             "quad",
//             "quad_vertex",
//             "quad_fragment",
//             PIXEL_FORMAT,
//         );

//         Self {
//             layer,
//             command_queue: device.new_command_queue(),
//             quad_pipeline_state,
//             buffer,
//         }
//     }

//     pub fn draw(&mut self, scene: &Scene) {
//         draw_quads(scene);
//     }

//     fn draw_quads(
//         &mut self,
//         quads: &[Quad],
//         scale_factor: f32,
//         offset: &mut usize,
//         drawable_size: Vector2F,
//         command_encoder: &metal::RenderCommandEncoderRef,
//     ) {
//         if quads.is_empty() {
//             return;
//         }
//         align_offset(offset);
//         let next_offset = *offset + quads.len() * mem::size_of::<shaders::GPUIQuad>();
//         assert!(
//             next_offset <= INSTANCE_BUFFER_SIZE,
//             "instance buffer exhausted"
//         );

//         command_encoder.set_render_pipeline_state(&self.quad_pipeline_state);
//         command_encoder.set_vertex_buffer(
//             shaders::GPUIQuadInputIndex_GPUIQuadInputIndexVertices as u64,
//             Some(&self.unit_vertices),
//             0,
//         );
//         command_encoder.set_vertex_buffer(
//             shaders::GPUIQuadInputIndex_GPUIQuadInputIndexQuads as u64,
//             Some(&self.instances),
//             *offset as u64,
//         );
//         command_encoder.set_vertex_bytes(
//             shaders::GPUIQuadInputIndex_GPUIQuadInputIndexUniforms as u64,
//             mem::size_of::<shaders::GPUIUniforms>() as u64,
//             [shaders::GPUIUniforms {
//                 viewport_size: drawable_size.to_float2(),
//             }]
//             .as_ptr() as *const c_void,
//         );

//         let buffer_contents = unsafe {
//             (self.instances.contents() as *mut u8).add(*offset) as *mut shaders::GPUIQuad
//         };
//         for (ix, quad) in quads.iter().enumerate() {
//             let bounds = quad.bounds * scale_factor;
//             let shader_quad = shaders::GPUIQuad {
//                 origin: bounds.origin().round().to_float2(),
//                 size: bounds.size().round().to_float2(),
//                 background_color: quad
//                     .background
//                     .unwrap_or_else(Color::transparent_black)
//                     .to_uchar4(),
//                 border_top: quad.border.top * scale_factor,
//                 border_right: quad.border.right * scale_factor,
//                 border_bottom: quad.border.bottom * scale_factor,
//                 border_left: quad.border.left * scale_factor,
//                 border_color: quad.border.color.to_uchar4(),
//                 corner_radius_top_left: quad.corner_radii.top_left * scale_factor,
//                 corner_radius_top_right: quad.corner_radii.top_right * scale_factor,
//                 corner_radius_bottom_right: quad.corner_radii.bottom_right * scale_factor,
//                 corner_radius_bottom_left: quad.corner_radii.bottom_left * scale_factor,
//             };
//             unsafe {
//                 *(buffer_contents.add(ix)) = shader_quad;
//             }
//         }

//         command_encoder.draw_primitives_instanced(
//             metal::MTLPrimitiveType::Triangle,
//             0,
//             6,
//             quads.len() as u64,
//         );
//         *offset = next_offset;
//     }
// }
