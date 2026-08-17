use alloc::sync::Arc;
use alloc::vec;
use core::sync::atomic::Ordering;

use arrayvec::ArrayVec;
use glow::HasContext;

use super::{conv::is_layered_target, lock, Command as C, PrivateCapabilities};

const DEBUG_ID: u32 = 0;

fn extract_marker<'a>(data: &'a [u8], range: &core::ops::Range<u32>) -> &'a str {
    core::str::from_utf8(&data[range.start as usize..range.end as usize]).unwrap()
}

fn to_debug_str(s: &str) -> &str {
    // The spec mentions that if the length given to debug functions is negative,
    // the implementation will access the ptr and look for a null that terminates
    // the string but some implementations will try to access the ptr even if the
    // length is 0.
    if s.is_empty() {
        "<empty>"
    } else {
        s
    }
}

fn get_2d_target(target: u32, array_layer: u32) -> u32 {
    const CUBEMAP_FACES: [u32; 6] = [
        glow::TEXTURE_CUBE_MAP_POSITIVE_X,
        glow::TEXTURE_CUBE_MAP_NEGATIVE_X,
        glow::TEXTURE_CUBE_MAP_POSITIVE_Y,
        glow::TEXTURE_CUBE_MAP_NEGATIVE_Y,
        glow::TEXTURE_CUBE_MAP_POSITIVE_Z,
        glow::TEXTURE_CUBE_MAP_NEGATIVE_Z,
    ];

    match target {
        glow::TEXTURE_2D => target,
        glow::TEXTURE_CUBE_MAP => CUBEMAP_FACES[array_layer as usize],
        _ => unreachable!(),
    }
}

fn get_z_offset(target: u32, base: &crate::TextureCopyBase) -> u32 {
    match target {
        glow::TEXTURE_2D_ARRAY | glow::TEXTURE_CUBE_MAP_ARRAY => base.array_layer,
        glow::TEXTURE_3D => base.origin.z,
        _ => unreachable!(),
    }
}

impl super::Queue {
    /// Performs a manual shader clear, used as a workaround for a clearing bug on mesa
    unsafe fn perform_shader_clear(&self, gl: &glow::Context, draw_buffer: u32, color: [f32; 4]) {
        let shader_clear = self
            .shader_clear_program
            .as_ref()
            .expect("shader_clear_program should always be set if the workaround is enabled");
        unsafe { gl.use_program(Some(shader_clear.program)) };
        unsafe {
            gl.uniform_4_f32(
                Some(&shader_clear.color_uniform_location),
                color[0],
                color[1],
                color[2],
                color[3],
            )
        };
        unsafe { gl.disable(glow::DEPTH_TEST) };
        unsafe { gl.disable(glow::STENCIL_TEST) };
        unsafe { gl.disable(glow::SCISSOR_TEST) };
        unsafe { gl.disable(glow::BLEND) };
        unsafe { gl.disable(glow::CULL_FACE) };
        unsafe { gl.draw_buffers(&[glow::COLOR_ATTACHMENT0 + draw_buffer]) };
        unsafe { gl.draw_arrays(glow::TRIANGLES, 0, 3) };

        let draw_buffer_count = self.draw_buffer_count.load(Ordering::Relaxed);
        if draw_buffer_count != 0 {
            // Reset the draw buffers to what they were before the clear
            let indices = (0..draw_buffer_count as u32)
                .map(|i| glow::COLOR_ATTACHMENT0 + i)
                .collect::<ArrayVec<_, { crate::MAX_COLOR_ATTACHMENTS }>>();
            unsafe { gl.draw_buffers(&indices) };
        }
    }

    unsafe fn reset_state(&self, gl: &glow::Context) {
        unsafe { gl.use_program(None) };
        unsafe { gl.bind_framebuffer(glow::FRAMEBUFFER, None) };
        unsafe { gl.disable(glow::DEPTH_TEST) };
        unsafe { gl.disable(glow::STENCIL_TEST) };
        unsafe { gl.disable(glow::SCISSOR_TEST) };
        unsafe { gl.disable(glow::BLEND) };
        unsafe { gl.disable(glow::CULL_FACE) };
        unsafe { gl.disable(glow::POLYGON_OFFSET_FILL) };
        unsafe { gl.disable(glow::SAMPLE_ALPHA_TO_COVERAGE) };
        if self.features.contains(wgt::Features::DEPTH_CLIP_CONTROL) {
            unsafe { gl.disable(glow::DEPTH_CLAMP) };
        }

        unsafe { gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None) };
        let mut current_index_buffer = self.current_index_buffer.lock();
        *current_index_buffer = None;
    }

    unsafe fn set_attachment(
        &self,
        gl: &glow::Context,
        fbo_target: u32,
        attachment: u32,
        view: &super::TextureView,
        depth_slice: Option<u32>,
        sample_count: u32,
    ) {
        match view.inner {
            super::TextureInner::Renderbuffer { raw } => {
                unsafe {
                    gl.framebuffer_renderbuffer(
                        fbo_target,
                        attachment,
                        glow::RENDERBUFFER,
                        Some(raw),
                    )
                };
            }
            super::TextureInner::DefaultRenderbuffer => panic!("Unexpected default RBO"),
            super::TextureInner::Texture { raw, target } => {
                let num_layers = view.array_layers.end - view.array_layers.start;
                if num_layers > 1 {
                    #[cfg(webgl)]
                    unsafe {
                        gl.framebuffer_texture_multiview_ovr(
                            fbo_target,
                            attachment,
                            Some(raw),
                            view.mip_levels.start as i32,
                            view.array_layers.start as i32,
                            num_layers as i32,
                        )
                    };
                } else if is_layered_target(target) {
                    let layer = if target == glow::TEXTURE_3D {
                        depth_slice.unwrap() as i32
                    } else {
                        view.array_layers.start as i32
                    };
                    unsafe {
                        gl.framebuffer_texture_layer(
                            fbo_target,
                            attachment,
                            Some(raw),
                            view.mip_levels.start as i32,
                            layer,
                        )
                    };
                } else {
                    unsafe {
                        assert_eq!(view.mip_levels.len(), 1);
                        if sample_count != 1 {
                            gl.framebuffer_texture_2d_multisample(
                                fbo_target,
                                attachment,
                                get_2d_target(target, view.array_layers.start),
                                Some(raw),
                                view.mip_levels.start as i32,
                                sample_count as i32,
                            )
                        } else {
                            gl.framebuffer_texture_2d(
                                fbo_target,
                                attachment,
                                get_2d_target(target, view.array_layers.start),
                                Some(raw),
                                view.mip_levels.start as i32,
                            )
                        }
                    };
                }
            }
            #[cfg(webgl)]
            super::TextureInner::ExternalFramebuffer { ref inner } => unsafe {
                gl.bind_external_framebuffer(glow::FRAMEBUFFER, inner);
            },
            #[cfg(native)]
            super::TextureInner::ExternalNativeFramebuffer { ref inner } => unsafe {
                gl.bind_framebuffer(glow::FRAMEBUFFER, Some(*inner));
            },
        }
    }

    unsafe fn process(
        &self,
        gl: &glow::Context,
        command: &C,
        #[cfg_attr(target_arch = "wasm32", allow(unused))] data_bytes: &[u8],
        queries: &[glow::Query],
    ) {
        match *command {
            C::Draw {
                topology,
                first_vertex,
                vertex_count,
                instance_count,
                first_instance,
                ref first_instance_location,
            } => {
                let supports_full_instancing = self
                    .shared
                    .private_caps
                    .contains(PrivateCapabilities::FULLY_FEATURED_INSTANCING);

                if supports_full_instancing {
                    unsafe {
                        gl.draw_arrays_instanced_base_instance(
                            topology,
                            first_vertex as i32,
                            vertex_count as i32,
                            instance_count as i32,
                            first_instance,
                        )
                    }
                } else {
                    unsafe {
                        gl.uniform_1_u32(first_instance_location.as_ref(), first_instance);
                    }

                    // Don't use `gl.draw_arrays` for `instance_count == 1`.
                    // Angle has a bug where it doesn't consider the instance divisor when `DYNAMIC_DRAW` is used in `draw_arrays`.
                    // See https://github.com/gfx-rs/wgpu/issues/3578
                    unsafe {
                        gl.draw_arrays_instanced(
                            topology,
                            first_vertex as i32,
                            vertex_count as i32,
                            instance_count as i32,
                        )
                    }
                };
            }
            C::DrawIndexed {
                topology,
                index_type,
                index_count,
                index_offset,
                base_vertex,
                first_instance,
                instance_count,
                ref first_instance_location,
            } => {
                let supports_full_instancing = self
                    .shared
                    .private_caps
                    .contains(PrivateCapabilities::FULLY_FEATURED_INSTANCING);

                if supports_full_instancing {
                    unsafe {
                        gl.draw_elements_instanced_base_vertex_base_instance(
                            topology,
                            index_count as i32,
                            index_type,
                            index_offset as i32,
                            instance_count as i32,
                            base_vertex,
                            first_instance,
                        )
                    }
                } else {
                    unsafe { gl.uniform_1_u32(first_instance_location.as_ref(), first_instance) };

                    if base_vertex == 0 {
                        unsafe {
                            // Don't use `gl.draw_elements`/`gl.draw_elements_base_vertex` for `instance_count == 1`.
                            // Angle has a bug where it doesn't consider the instance divisor when `DYNAMIC_DRAW` is used in `gl.draw_elements`/`gl.draw_elements_base_vertex`.
                            // See https://github.com/gfx-rs/wgpu/issues/3578
                            gl.draw_elements_instanced(
                                topology,
                                index_count as i32,
                                index_type,
                                index_offset as i32,
                                instance_count as i32,
                            )
                        }
                    } else {
                        // If we've gotten here, wgpu-core has already validated that this function exists via the DownlevelFlags::BASE_VERTEX feature.
                        unsafe {
                            gl.draw_elements_instanced_base_vertex(
                                topology,
                                index_count as _,
                                index_type,
                                index_offset as i32,
                                instance_count as i32,
                                base_vertex,
                            )
                        }
                    }
                }
            }
            C::DrawIndirect {
                topology,
                indirect_buf,
                indirect_offset,
                ref first_instance_location,
            } => {
                unsafe { gl.uniform_1_u32(first_instance_location.as_ref(), 0) };

                unsafe { gl.bind_buffer(glow::DRAW_INDIRECT_BUFFER, Some(indirect_buf)) };
                unsafe { gl.draw_arrays_indirect_offset(topology, indirect_offset as i32) };
            }
            C::DrawIndexedIndirect {
                topology,
                index_type,
                indirect_buf,
                indirect_offset,
                ref first_instance_location,
            } => {
                unsafe { gl.uniform_1_u32(first_instance_location.as_ref(), 0) };

                unsafe { gl.bind_buffer(glow::DRAW_INDIRECT_BUFFER, Some(indirect_buf)) };
                unsafe {
                    gl.draw_elements_indirect_offset(topology, index_type, indirect_offset as i32)
                };
            }
            C::Dispatch(group_counts) => {
                unsafe { gl.dispatch_compute(group_counts[0], group_counts[1], group_counts[2]) };
            }
            C::DispatchIndirect {
                indirect_buf,
                indirect_offset,
            } => {
                unsafe { gl.bind_buffer(glow::DISPATCH_INDIRECT_BUFFER, Some(indirect_buf)) };
                unsafe { gl.dispatch_compute_indirect(indirect_offset as i32) };
            }
            C::ClearBuffer {
                ref dst,
                dst_target,
                ref range,
            } => match dst.raw {
                Some(buffer) => {
                    // When `INDEX_BUFFER_ROLE_CHANGE` isn't available, we can't copy into the
                    // index buffer from the zero buffer. This would fail in Chrome with the
                    // following message:
                    //
                    // > Cannot copy into an element buffer destination from a non-element buffer
                    // > source
                    //
                    // Instead, we'll upload zeroes into the buffer.
                    let can_use_zero_buffer = self
                        .shared
                        .private_caps
                        .contains(PrivateCapabilities::INDEX_BUFFER_ROLE_CHANGE)
                        || dst_target != glow::ELEMENT_ARRAY_BUFFER;

                    if can_use_zero_buffer {
                        unsafe { gl.bind_buffer(glow::COPY_READ_BUFFER, Some(self.zero_buffer)) };
                        unsafe { gl.bind_buffer(dst_target, Some(buffer)) };
                        let mut dst_offset = range.start;
                        while dst_offset < range.end {
                            let size = (range.end - dst_offset).min(super::ZERO_BUFFER_SIZE as u64);
                            unsafe {
                                gl.copy_buffer_sub_data(
                                    glow::COPY_READ_BUFFER,
                                    dst_target,
                                    0,
                                    dst_offset as i32,
                                    size as i32,
                                )
                            };
                            dst_offset += size;
                        }
                    } else {
                        unsafe { gl.bind_buffer(dst_target, Some(buffer)) };
                        let zeroes = vec![0u8; (range.end - range.start) as usize];
                        unsafe {
                            gl.buffer_sub_data_u8_slice(dst_target, range.start as i32, &zeroes)
                        };
                    }
                }
                None => {
                    lock(dst.data.as_ref().unwrap()).as_mut_slice()
                        [range.start as usize..range.end as usize]
                        .fill(0);
                }
            },
            C::CopyBufferToBuffer {
                ref src,
                src_target,
                ref dst,
                dst_target,
                copy,
            } => {
                let copy_src_target = glow::COPY_READ_BUFFER;
                let is_index_buffer_only_element_dst = !self
                    .shared
                    .private_caps
                    .contains(PrivateCapabilities::INDEX_BUFFER_ROLE_CHANGE)
                    && dst_target == glow::ELEMENT_ARRAY_BUFFER
                    || src_target == glow::ELEMENT_ARRAY_BUFFER;

                // WebGL not allowed to copy data from other targets to element buffer and can't copy element data to other buffers
                let copy_dst_target = if is_index_buffer_only_element_dst {
                    glow::ELEMENT_ARRAY_BUFFER
                } else {
                    glow::COPY_WRITE_BUFFER
                };
                let size = copy.size.get() as usize;
                match (src.raw, dst.raw) {
                    (Some(ref src), Some(ref dst)) => {
                        unsafe { gl.bind_buffer(copy_src_target, Some(*src)) };
                        unsafe { gl.bind_buffer(copy_dst_target, Some(*dst)) };
                        unsafe {
                            gl.copy_buffer_sub_data(
                                copy_src_target,
                                copy_dst_target,
                                copy.src_offset as _,
                                copy.dst_offset as _,
                                copy.size.get() as _,
                            )
                        };
                    }
                    (Some(src), None) => {
                        let mut data = lock(dst.data.as_ref().unwrap());
                        let dst_data = &mut data.as_mut_slice()
                            [copy.dst_offset as usize..copy.dst_offset as usize + size];

                        unsafe { gl.bind_buffer(copy_src_target, Some(src)) };
                        unsafe {
                            self.shared.get_buffer_sub_data(
                                gl,
                                copy_src_target,
                                copy.src_offset as i32,
                                dst_data,
                            )
                        };
                    }
                    (None, Some(dst)) => {
                        let data = lock(src.data.as_ref().unwrap());
                        let src_data = &data.as_slice()
                            [copy.src_offset as usize..copy.src_offset as usize + size];
                        unsafe { gl.bind_buffer(copy_dst_target, Some(dst)) };
                        unsafe {
                            gl.buffer_sub_data_u8_slice(
                                copy_dst_target,
                                copy.dst_offset as i32,
                                src_data,
                            )
                        };
                    }
                    (None, None) => {
                        todo!()
                    }
                }
                unsafe { gl.bind_buffer(copy_src_target, None) };
                if is_index_buffer_only_element_dst {
                    unsafe {
                        gl.bind_buffer(
                            glow::ELEMENT_ARRAY_BUFFER,
                            *self.current_index_buffer.lock(),
                        )
                    };
                } else {
                    unsafe { gl.bind_buffer(copy_dst_target, None) };
                }
            }
            #[cfg(webgl)]
            C::CopyExternalImageToTexture {
                ref src,
                dst,
                dst_target,
                dst_format,
                dst_premultiplication,
                ref copy,
            } => {
                const UNPACK_FLIP_Y_WEBGL: u32 =
                    web_sys::WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL;
                const UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 =
                    web_sys::WebGl2RenderingContext::UNPACK_PREMULTIPLY_ALPHA_WEBGL;

                unsafe {
                    if src.flip_y {
                        gl.pixel_store_bool(UNPACK_FLIP_Y_WEBGL, true);
                    }
                    if dst_premultiplication {
                        gl.pixel_store_bool(UNPACK_PREMULTIPLY_ALPHA_WEBGL, true);
                    }
                }

                unsafe { gl.bind_texture(dst_target, Some(dst)) };
                let format_desc = self.shared.describe_texture_format(dst_format);
                if is_layered_target(dst_target) {
                    let z_offset = get_z_offset(dst_target, &copy.dst_base);

                    match src.source {
                        wgt::ExternalImageSource::ImageBitmap(ref b) => unsafe {
                            gl.tex_sub_image_3d_with_image_bitmap(
                                dst_target,
                                copy.dst_base.mip_level as i32,
                                copy.dst_base.origin.x as i32,
                                copy.dst_base.origin.y as i32,
                                z_offset as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                copy.size.depth as i32,
                                format_desc.external,
                                format_desc.data_type,
                                b,
                            );
                        },
                        wgt::ExternalImageSource::HTMLImageElement(ref i) => unsafe {
                            gl.tex_sub_image_3d_with_html_image_element(
                                dst_target,
                                copy.dst_base.mip_level as i32,
                                copy.dst_base.origin.x as i32,
                                copy.dst_base.origin.y as i32,
                                z_offset as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                copy.size.depth as i32,
                                format_desc.external,
                                format_desc.data_type,
                                i,
                            );
                        },
                        wgt::ExternalImageSource::HTMLVideoElement(ref v) => unsafe {
                            gl.tex_sub_image_3d_with_html_video_element(
                                dst_target,
                                copy.dst_base.mip_level as i32,
                                copy.dst_base.origin.x as i32,
                                copy.dst_base.origin.y as i32,
                                z_offset as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                copy.size.depth as i32,
                                format_desc.external,
                                format_desc.data_type,
                                v,
                            );
                        },
                        #[cfg(web_sys_unstable_apis)]
                        wgt::ExternalImageSource::VideoFrame(ref v) => unsafe {
                            gl.tex_sub_image_3d_with_video_frame(
                                dst_target,
                                copy.dst_base.mip_level as i32,
                                copy.dst_base.origin.x as i32,
                                copy.dst_base.origin.y as i32,
                                z_offset as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                copy.size.depth as i32,
                                format_desc.external,
                                format_desc.data_type,
                                v,
                            )
                        },
                        wgt::ExternalImageSource::ImageData(ref i) => unsafe {
                            gl.tex_sub_image_3d_with_image_data(
                                dst_target,
                                copy.dst_base.mip_level as i32,
                                copy.dst_base.origin.x as i32,
                                copy.dst_base.origin.y as i32,
                                z_offset as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                copy.size.depth as i32,
                                format_desc.external,
                                format_desc.data_type,
                                i,
                            );
                        },
                        wgt::ExternalImageSource::HTMLCanvasElement(ref c) => unsafe {
                            gl.tex_sub_image_3d_with_html_canvas_element(
                                dst_target,
                                copy.dst_base.mip_level as i32,
                                copy.dst_base.origin.x as i32,
                                copy.dst_base.origin.y as i32,
                                z_offset as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                copy.size.depth as i32,
                                format_desc.external,
                                format_desc.data_type,
                                c,
                            );
                        },
                        wgt::ExternalImageSource::OffscreenCanvas(_) => unreachable!(),
                    }
                } else {
                    let dst_target = get_2d_target(dst_target, copy.dst_base.array_layer);

                    match src.source {
                        wgt::ExternalImageSource::ImageBitmap(ref b) => unsafe {
                            gl.tex_sub_image_2d_with_image_bitmap_and_width_and_height(
                                dst_target,
                                copy.dst_base.mip_level as i32,
                                copy.dst_base.origin.x as i32,
                                copy.dst_base.origin.y as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                format_desc.external,
                                format_desc.data_type,
                                b,
                            );
                        },
                        wgt::ExternalImageSource::HTMLImageElement(ref i) => unsafe {
                            gl.tex_sub_image_2d_with_html_image_and_width_and_height(
                                dst_target,
                                copy.dst_base.mip_level as i32,
                                copy.dst_base.origin.x as i32,
                                copy.dst_base.origin.y as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                format_desc.external,
                                format_desc.data_type,
                                i,
                            )
                        },
                        wgt::ExternalImageSource::HTMLVideoElement(ref v) => unsafe {
                            gl.tex_sub_image_2d_with_html_video_and_width_and_height(
                                dst_target,
                                copy.dst_base.mip_level as i32,
                                copy.dst_base.origin.x as i32,
                                copy.dst_base.origin.y as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                format_desc.external,
                                format_desc.data_type,
                                v,
                            )
                        },
                        #[cfg(web_sys_unstable_apis)]
                        wgt::ExternalImageSource::VideoFrame(ref v) => unsafe {
                            gl.tex_sub_image_2d_with_video_frame_and_width_and_height(
                                dst_target,
                                copy.dst_base.mip_level as i32,
                                copy.dst_base.origin.x as i32,
                                copy.dst_base.origin.y as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                format_desc.external,
                                format_desc.data_type,
                                v,
                            )
                        },
                        wgt::ExternalImageSource::ImageData(ref i) => unsafe {
                            gl.tex_sub_image_2d_with_image_data_and_width_and_height(
                                dst_target,
                                copy.dst_base.mip_level as i32,
                                copy.dst_base.origin.x as i32,
                                copy.dst_base.origin.y as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                format_desc.external,
                                format_desc.data_type,
                                i,
                            );
                        },
                        wgt::ExternalImageSource::HTMLCanvasElement(ref c) => unsafe {
                            gl.tex_sub_image_2d_with_html_canvas_and_width_and_height(
                                dst_target,
                                copy.dst_base.mip_level as i32,
                                copy.dst_base.origin.x as i32,
                                copy.dst_base.origin.y as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                format_desc.external,
                                format_desc.data_type,
                                c,
                            )
                        },
                        wgt::ExternalImageSource::OffscreenCanvas(_) => unreachable!(),
                    }
                }

                unsafe {
                    if src.flip_y {
                        gl.pixel_store_bool(UNPACK_FLIP_Y_WEBGL, false);
                    }
                    if dst_premultiplication {
                        gl.pixel_store_bool(UNPACK_PREMULTIPLY_ALPHA_WEBGL, false);
                    }
                }
            }
            C::CopyTextureToTexture {
                src,
                src_target,
                dst,
                dst_target,
                ref copy,
            } => {
                //TODO: handle 3D copies
                unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(self.copy_fbo)) };
                if is_layered_target(src_target) {
                    //TODO: handle GLES without framebuffer_texture_3d
                    unsafe {
                        gl.framebuffer_texture_layer(
                            glow::READ_FRAMEBUFFER,
                            glow::COLOR_ATTACHMENT0,
                            Some(src),
                            copy.src_base.mip_level as i32,
                            copy.src_base.array_layer as i32,
                        )
                    };
                } else {
                    unsafe {
                        gl.framebuffer_texture_2d(
                            glow::READ_FRAMEBUFFER,
                            glow::COLOR_ATTACHMENT0,
                            src_target,
                            Some(src),
                            copy.src_base.mip_level as i32,
                        )
                    };
                }

                unsafe { gl.bind_texture(dst_target, Some(dst)) };
                if is_layered_target(dst_target) {
                    unsafe {
                        gl.copy_tex_sub_image_3d(
                            dst_target,
                            copy.dst_base.mip_level as i32,
                            copy.dst_base.origin.x as i32,
                            copy.dst_base.origin.y as i32,
                            get_z_offset(dst_target, &copy.dst_base) as i32,
                            copy.src_base.origin.x as i32,
                            copy.src_base.origin.y as i32,
                            copy.size.width as i32,
                            copy.size.height as i32,
                        )
                    };
                } else {
                    unsafe {
                        gl.copy_tex_sub_image_2d(
                            get_2d_target(dst_target, copy.dst_base.array_layer),
                            copy.dst_base.mip_level as i32,
                            copy.dst_base.origin.x as i32,
                            copy.dst_base.origin.y as i32,
                            copy.src_base.origin.x as i32,
                            copy.src_base.origin.y as i32,
                            copy.size.width as i32,
                            copy.size.height as i32,
                        )
                    };
                }
            }
            C::CopyBufferToTexture {
                ref src,
                src_target: _,
                dst,
                dst_target,
                dst_format,
                ref copy,
            } => {
                let (block_width, block_height) = dst_format.block_dimensions();
                let block_size = dst_format.block_copy_size(None).unwrap();
                let format_desc = self.shared.describe_texture_format(dst_format);
                let row_texels = copy
                    .buffer_layout
                    .bytes_per_row
                    .map_or(0, |bpr| block_width * bpr / block_size);
                let column_texels = copy
                    .buffer_layout
                    .rows_per_image
                    .map_or(0, |rpi| block_height * rpi);

                unsafe { gl.bind_texture(dst_target, Some(dst)) };
                unsafe { gl.pixel_store_i32(glow::UNPACK_ROW_LENGTH, row_texels as i32) };
                unsafe { gl.pixel_store_i32(glow::UNPACK_IMAGE_HEIGHT, column_texels as i32) };
                let mut unbind_unpack_buffer = false;
                if !dst_format.is_compressed() {
                    let buffer_data;
                    let unpack_data = match src.raw {
                        Some(buffer) => {
                            unsafe { gl.bind_buffer(glow::PIXEL_UNPACK_BUFFER, Some(buffer)) };
                            unbind_unpack_buffer = true;
                            glow::PixelUnpackData::BufferOffset(copy.buffer_layout.offset as u32)
                        }
                        None => {
                            buffer_data = lock(src.data.as_ref().unwrap());
                            let src_data =
                                &buffer_data.as_slice()[copy.buffer_layout.offset as usize..];
                            glow::PixelUnpackData::Slice(Some(src_data))
                        }
                    };
                    if is_layered_target(dst_target) {
                        unsafe {
                            gl.tex_sub_image_3d(
                                dst_target,
                                copy.texture_base.mip_level as i32,
                                copy.texture_base.origin.x as i32,
                                copy.texture_base.origin.y as i32,
                                get_z_offset(dst_target, &copy.texture_base) as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                copy.size.depth as i32,
                                format_desc.external,
                                format_desc.data_type,
                                unpack_data,
                            )
                        };
                    } else {
                        unsafe {
                            gl.tex_sub_image_2d(
                                get_2d_target(dst_target, copy.texture_base.array_layer),
                                copy.texture_base.mip_level as i32,
                                copy.texture_base.origin.x as i32,
                                copy.texture_base.origin.y as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                format_desc.external,
                                format_desc.data_type,
                                unpack_data,
                            )
                        };
                    }
                } else {
                    let bytes_per_row = copy
                        .buffer_layout
                        .bytes_per_row
                        .unwrap_or(copy.size.width * block_size);
                    let minimum_rows_per_image = copy.size.height.div_ceil(block_height);
                    let rows_per_image = copy
                        .buffer_layout
                        .rows_per_image
                        .unwrap_or(minimum_rows_per_image);

                    let bytes_per_image = bytes_per_row * rows_per_image;
                    let minimum_bytes_per_image = bytes_per_row * minimum_rows_per_image;
                    let bytes_in_upload =
                        (bytes_per_image * (copy.size.depth - 1)) + minimum_bytes_per_image;
                    let offset = copy.buffer_layout.offset as u32;

                    let buffer_data;
                    let unpack_data = match src.raw {
                        Some(buffer) => {
                            unsafe { gl.bind_buffer(glow::PIXEL_UNPACK_BUFFER, Some(buffer)) };
                            unbind_unpack_buffer = true;
                            glow::CompressedPixelUnpackData::BufferRange(
                                offset..offset + bytes_in_upload,
                            )
                        }
                        None => {
                            buffer_data = lock(src.data.as_ref().unwrap());
                            let src_data = &buffer_data.as_slice()
                                [(offset as usize)..(offset + bytes_in_upload) as usize];
                            glow::CompressedPixelUnpackData::Slice(src_data)
                        }
                    };

                    if is_layered_target(dst_target) {
                        unsafe {
                            gl.compressed_tex_sub_image_3d(
                                dst_target,
                                copy.texture_base.mip_level as i32,
                                copy.texture_base.origin.x as i32,
                                copy.texture_base.origin.y as i32,
                                get_z_offset(dst_target, &copy.texture_base) as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                copy.size.depth as i32,
                                format_desc.internal,
                                unpack_data,
                            )
                        };
                    } else {
                        unsafe {
                            gl.compressed_tex_sub_image_2d(
                                get_2d_target(dst_target, copy.texture_base.array_layer),
                                copy.texture_base.mip_level as i32,
                                copy.texture_base.origin.x as i32,
                                copy.texture_base.origin.y as i32,
                                copy.size.width as i32,
                                copy.size.height as i32,
                                format_desc.internal,
                                unpack_data,
                            )
                        };
                    }
                }
                if unbind_unpack_buffer {
                    unsafe { gl.bind_buffer(glow::PIXEL_UNPACK_BUFFER, None) };
                }
            }
            C::CopyTextureToBuffer {
                src,
                src_target,
                src_format,
                ref dst,
                dst_target: _,
                ref copy,
            } => {
                let block_size = src_format.block_copy_size(None).unwrap();
                if src_format.is_compressed() {
                    log::error!("Not implemented yet: compressed texture copy to buffer");
                    return;
                }
                if src_target == glow::TEXTURE_CUBE_MAP
                    || src_target == glow::TEXTURE_CUBE_MAP_ARRAY
                {
                    log::error!("Not implemented yet: cubemap texture copy to buffer");
                    return;
                }
                let format_desc = self.shared.describe_texture_format(src_format);
                let row_texels = copy
                    .buffer_layout
                    .bytes_per_row
                    .map_or(copy.size.width, |bpr| bpr / block_size);
                let column_texels = copy
                    .buffer_layout
                    .rows_per_image
                    .unwrap_or(copy.size.height);

                unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(self.copy_fbo)) };

                let read_pixels = |offset| {
                    let mut buffer_data;
                    let unpack_data = match dst.raw {
                        Some(buffer) => {
                            unsafe { gl.pixel_store_i32(glow::PACK_ROW_LENGTH, row_texels as i32) };
                            unsafe { gl.bind_buffer(glow::PIXEL_PACK_BUFFER, Some(buffer)) };
                            glow::PixelPackData::BufferOffset(offset as u32)
                        }
                        None => {
                            buffer_data = lock(dst.data.as_ref().unwrap());
                            let dst_data = &mut buffer_data.as_mut_slice()[offset as usize..];
                            glow::PixelPackData::Slice(Some(dst_data))
                        }
                    };
                    unsafe {
                        gl.read_pixels(
                            copy.texture_base.origin.x as i32,
                            copy.texture_base.origin.y as i32,
                            copy.size.width as i32,
                            copy.size.height as i32,
                            format_desc.external,
                            format_desc.data_type,
                            unpack_data,
                        )
                    };
                };

                match src_target {
                    glow::TEXTURE_2D => {
                        unsafe {
                            gl.framebuffer_texture_2d(
                                glow::READ_FRAMEBUFFER,
                                glow::COLOR_ATTACHMENT0,
                                src_target,
                                Some(src),
                                copy.texture_base.mip_level as i32,
                            )
                        };
                        read_pixels(copy.buffer_layout.offset);
                    }
                    glow::TEXTURE_2D_ARRAY => {
                        unsafe {
                            gl.framebuffer_texture_layer(
                                glow::READ_FRAMEBUFFER,
                                glow::COLOR_ATTACHMENT0,
                                Some(src),
                                copy.texture_base.mip_level as i32,
                                copy.texture_base.array_layer as i32,
                            )
                        };
                        read_pixels(copy.buffer_layout.offset);
                    }
                    glow::TEXTURE_3D => {
                        for z in copy.texture_base.origin.z..copy.size.depth {
                            unsafe {
                                gl.framebuffer_texture_layer(
                                    glow::READ_FRAMEBUFFER,
                                    glow::COLOR_ATTACHMENT0,
                                    Some(src),
                                    copy.texture_base.mip_level as i32,
                                    z as i32,
                                )
                            };
                            let offset = copy.buffer_layout.offset
                                + (z * block_size * row_texels * column_texels) as u64;
                            read_pixels(offset);
                        }
                    }
                    glow::TEXTURE_CUBE_MAP | glow::TEXTURE_CUBE_MAP_ARRAY => unimplemented!(),
                    _ => unreachable!(),
                }
            }
            C::SetIndexBuffer(buffer) => {
                unsafe { gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer)) };
                let mut current_index_buffer = self.current_index_buffer.lock();
                *current_index_buffer = Some(buffer);
            }
            C::BeginQuery(query, target) => {
                unsafe { gl.begin_query(target, query) };
            }
            C::EndQuery(target) => {
                unsafe { gl.end_query(target) };
            }
            C::TimestampQuery(query) => {
                unsafe { gl.query_counter(query, glow::TIMESTAMP) };
            }
            C::CopyQueryResults {
                ref query_range,
                ref dst,
                dst_target,
                dst_offset,
            } => {
                if self
                    .shared
                    .private_caps
                    .contains(PrivateCapabilities::QUERY_BUFFERS)
                    && dst.raw.is_some()
                {
                    unsafe {
                        // We're assuming that the only relevant queries are 8 byte timestamps or
                        // occlusion tests.
                        let query_size = 8;

                        let query_range_size = query_size * query_range.len();

                        let buffer = gl.create_buffer().ok();
                        gl.bind_buffer(glow::QUERY_BUFFER, buffer);
                        gl.buffer_data_size(
                            glow::QUERY_BUFFER,
                            query_range_size as _,
                            glow::STREAM_COPY,
                        );

                        for (i, &query) in queries
                            [query_range.start as usize..query_range.end as usize]
                            .iter()
                            .enumerate()
                        {
                            gl.get_query_parameter_u64_with_offset(
                                query,
                                glow::QUERY_RESULT,
                                query_size * i,
                            )
                        }
                        gl.bind_buffer(dst_target, dst.raw);
                        gl.copy_buffer_sub_data(
                            glow::QUERY_BUFFER,
                            dst_target,
                            0,
                            dst_offset as _,
                            query_range_size as _,
                        );
                        if let Some(buffer) = buffer {
                            gl.delete_buffer(buffer)
                        }
                    }
                } else {
                    let mut temp_query_results = self.temp_query_results.lock();
                    temp_query_results.clear();
                    for &query in
                        queries[query_range.start as usize..query_range.end as usize].iter()
                    {
                        let mut result: u64 = 0;
                        unsafe {
                            if self
                                .shared
                                .private_caps
                                .contains(PrivateCapabilities::QUERY_64BIT)
                            {
                                let result: *mut u64 = &mut result;
                                gl.get_query_parameter_u64_with_offset(
                                    query,
                                    glow::QUERY_RESULT,
                                    result as usize,
                                )
                            } else {
                                result =
                                    gl.get_query_parameter_u32(query, glow::QUERY_RESULT) as u64;
                            }
                        };
                        temp_query_results.push(result);
                    }
                    let query_data = bytemuck::cast_slice(&temp_query_results);
                    match dst.raw {
                        Some(buffer) => {
                            unsafe { gl.bind_buffer(dst_target, Some(buffer)) };
                            unsafe {
                                gl.buffer_sub_data_u8_slice(
                                    dst_target,
                                    dst_offset as i32,
                                    query_data,
                                )
                            };
                        }
                        None => {
                            let data = &mut lock(dst.data.as_ref().unwrap());
                            let len = query_data.len().min(data.len());
                            data[..len].copy_from_slice(&query_data[..len]);
                        }
                    }
                }
            }
            C::ResetFramebuffer { is_default } => {
                if is_default {
                    unsafe { gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, None) };
                } else {
                    unsafe { gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, Some(self.draw_fbo)) };
                    unsafe {
                        gl.framebuffer_texture_2d(
                            glow::DRAW_FRAMEBUFFER,
                            glow::DEPTH_STENCIL_ATTACHMENT,
                            glow::TEXTURE_2D,
                            None,
                            0,
                        )
                    };
                    for i in 0..self.shared.limits.max_color_attachments {
                        let target = glow::COLOR_ATTACHMENT0 + i;
                        unsafe {
                            gl.framebuffer_texture_2d(
                                glow::DRAW_FRAMEBUFFER,
                                target,
                                glow::TEXTURE_2D,
                                None,
                                0,
                            )
                        };
                    }
                }
                unsafe { gl.color_mask(true, true, true, true) };
                unsafe { gl.depth_mask(true) };
                unsafe { gl.stencil_mask(!0) };
                unsafe { gl.disable(glow::DEPTH_TEST) };
                unsafe { gl.disable(glow::STENCIL_TEST) };
                unsafe { gl.disable(glow::SCISSOR_TEST) };
            }
            C::BindAttachment {
                attachment,
                ref view,
                depth_slice,
                sample_count,
            } => {
                unsafe {
                    self.set_attachment(
                        gl,
                        glow::DRAW_FRAMEBUFFER,
                        attachment,
                        view,
                        depth_slice,
                        sample_count,
                    )
                };
            }
            C::ResolveAttachment {
                attachment,
                ref dst,
                ref size,
            } => {
                unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(self.draw_fbo)) };
                unsafe { gl.read_buffer(attachment) };
                unsafe { gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, Some(self.copy_fbo)) };
                unsafe {
                    self.set_attachment(
                        gl,
                        glow::DRAW_FRAMEBUFFER,
                        glow::COLOR_ATTACHMENT0,
                        dst,
                        None,
                        1,
                    )
                };
                unsafe {
                    gl.blit_framebuffer(
                        0,
                        0,
                        size.width as i32,
                        size.height as i32,
                        0,
                        0,
                        size.width as i32,
                        size.height as i32,
                        glow::COLOR_BUFFER_BIT,
                        glow::NEAREST,
                    )
                };
                unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, None) };
                unsafe { gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, Some(self.draw_fbo)) };
            }
            C::InvalidateAttachments(ref list) => {
                if self
                    .shared
                    .private_caps
                    .contains(PrivateCapabilities::INVALIDATE_FRAMEBUFFER)
                {
                    unsafe { gl.invalidate_framebuffer(glow::DRAW_FRAMEBUFFER, list) };
                }
            }
            C::SetDrawColorBuffers(count) => {
                self.draw_buffer_count.store(count, Ordering::Relaxed);
                let indices = (0..count as u32)
                    .map(|i| glow::COLOR_ATTACHMENT0 + i)
                    .collect::<ArrayVec<_, { crate::MAX_COLOR_ATTACHMENTS }>>();
                unsafe { gl.draw_buffers(&indices) };
            }
            C::ClearColorF {
                draw_buffer,
                ref color,
                is_srgb,
            } => {
                if self
                    .shared
                    .workarounds
                    .contains(super::Workarounds::MESA_I915_SRGB_SHADER_CLEAR)
                    && is_srgb
                {
                    unsafe { self.perform_shader_clear(gl, draw_buffer, *color) };
                } else {
                    unsafe { gl.clear_buffer_f32_slice(glow::COLOR, draw_buffer, color) };
                }
            }
            C::ClearColorU(draw_buffer, ref color) => {
                unsafe { gl.clear_buffer_u32_slice(glow::COLOR, draw_buffer, color) };
            }
            C::ClearColorI(draw_buffer, ref color) => {
                unsafe { gl.clear_buffer_i32_slice(glow::COLOR, draw_buffer, color) };
            }
            C::ClearDepth(depth) => {
                // Prefer `clear` as `clear_buffer` functions have issues on Sandy Bridge
                // on Windows.
                unsafe {
                    gl.clear_depth_f32(depth);
                    gl.clear(glow::DEPTH_BUFFER_BIT);
                }
            }
            C::ClearStencil(value) => {
                // Prefer `clear` as `clear_buffer` functions have issues on Sandy Bridge
                // on Windows.
                unsafe {
                    gl.clear_stencil(value as i32);
                    gl.clear(glow::STENCIL_BUFFER_BIT);
                }
            }
            C::ClearDepthAndStencil(depth, stencil_value) => {
                // Prefer `clear` as `clear_buffer` functions have issues on Sandy Bridge
                // on Windows.
                unsafe {
                    gl.clear_depth_f32(depth);
                    gl.clear_stencil(stencil_value as i32);
                    gl.clear(glow::DEPTH_BUFFER_BIT | glow::STENCIL_BUFFER_BIT);
                }
            }
            C::BufferBarrier(raw, usage) => {
                let mut flags = 0;
                if usage.contains(wgt::BufferUses::VERTEX) {
                    flags |= glow::VERTEX_ATTRIB_ARRAY_BARRIER_BIT;
                    unsafe { gl.bind_buffer(glow::ARRAY_BUFFER, Some(raw)) };
                    unsafe { gl.vertex_attrib_pointer_f32(0, 1, glow::BYTE, true, 0, 0) };
                }
                if usage.contains(wgt::BufferUses::INDEX) {
                    flags |= glow::ELEMENT_ARRAY_BARRIER_BIT;
                    unsafe { gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(raw)) };
                }
                if usage.contains(wgt::BufferUses::UNIFORM) {
                    flags |= glow::UNIFORM_BARRIER_BIT;
                }
                if usage.contains(wgt::BufferUses::INDIRECT) {
                    flags |= glow::COMMAND_BARRIER_BIT;
                    unsafe { gl.bind_buffer(glow::DRAW_INDIRECT_BUFFER, Some(raw)) };
                }
                if usage.contains(wgt::BufferUses::COPY_SRC) {
                    flags |= glow::PIXEL_BUFFER_BARRIER_BIT;
                    unsafe { gl.bind_buffer(glow::PIXEL_UNPACK_BUFFER, Some(raw)) };
                }
                if usage.contains(wgt::BufferUses::COPY_DST) {
                    flags |= glow::PIXEL_BUFFER_BARRIER_BIT;
                    unsafe { gl.bind_buffer(glow::PIXEL_PACK_BUFFER, Some(raw)) };
                }
                if usage.intersects(wgt::BufferUses::MAP_READ | wgt::BufferUses::MAP_WRITE) {
                    flags |= glow::BUFFER_UPDATE_BARRIER_BIT;
                }
                if usage.intersects(
                    wgt::BufferUses::STORAGE_READ_ONLY | wgt::BufferUses::STORAGE_READ_WRITE,
                ) {
                    flags |= glow::SHADER_STORAGE_BARRIER_BIT;
                }
                unsafe { gl.memory_barrier(flags) };
            }
            // because `STORAGE_WRITE_ONLY` and `STORAGE_READ_WRITE` are only states
            // we can transit from due OpenGL memory barriers are used to make _subsequent_
            // operations see changes from the _shader_ side. We filter out usage changes that are
            // does not comes from the shader side in `transition_textures`
            C::TextureBarrier(usage) => {
                let mut flags = 0;
                if usage.contains(wgt::TextureUses::RESOURCE) {
                    flags |= glow::TEXTURE_FETCH_BARRIER_BIT;
                }
                if usage.intersects(
                    wgt::TextureUses::STORAGE_READ_ONLY
                        | wgt::TextureUses::STORAGE_WRITE_ONLY
                        | wgt::TextureUses::STORAGE_READ_WRITE,
                ) {
                    flags |= glow::SHADER_IMAGE_ACCESS_BARRIER_BIT;
                }
                if usage.intersects(wgt::TextureUses::COPY_SRC) {
                    flags |= glow::PIXEL_BUFFER_BARRIER_BIT;
                }
                if usage.contains(wgt::TextureUses::COPY_DST) {
                    flags |= glow::TEXTURE_UPDATE_BARRIER_BIT;
                }
                if usage.intersects(
                    wgt::TextureUses::COLOR_TARGET
                        | wgt::TextureUses::DEPTH_STENCIL_READ
                        | wgt::TextureUses::DEPTH_STENCIL_WRITE,
                ) {
                    flags |= glow::FRAMEBUFFER_BARRIER_BIT;
                }
                unsafe { gl.memory_barrier(flags) };
            }
            C::SetViewport {
                ref rect,
                ref depth,
            } => {
                unsafe { gl.viewport(rect.x, rect.y, rect.w, rect.h) };
                unsafe { gl.depth_range_f32(depth.start, depth.end) };
            }
            C::SetScissor(ref rect) => {
                unsafe { gl.scissor(rect.x, rect.y, rect.w, rect.h) };
                unsafe { gl.enable(glow::SCISSOR_TEST) };
            }
            C::SetStencilFunc {
                face,
                function,
                reference,
                read_mask,
            } => {
                unsafe { gl.stencil_func_separate(face, function, reference as i32, read_mask) };
            }
            C::SetStencilOps {
                face,
                write_mask,
                ref ops,
            } => {
                unsafe { gl.stencil_mask_separate(face, write_mask) };
                unsafe { gl.stencil_op_separate(face, ops.fail, ops.depth_fail, ops.pass) };
            }
            C::SetVertexAttribute {
                buffer,
                ref buffer_desc,
                attribute_desc: ref vat,
            } => {
                unsafe { gl.bind_buffer(glow::ARRAY_BUFFER, buffer) };
                unsafe { gl.enable_vertex_attrib_array(vat.location) };

                if buffer.is_none() {
                    match vat.format_desc.attrib_kind {
                        super::VertexAttribKind::Float => unsafe {
                            gl.vertex_attrib_format_f32(
                                vat.location,
                                vat.format_desc.element_count,
                                vat.format_desc.element_format,
                                true, // always normalized
                                vat.offset,
                            )
                        },
                        super::VertexAttribKind::Integer => unsafe {
                            gl.vertex_attrib_format_i32(
                                vat.location,
                                vat.format_desc.element_count,
                                vat.format_desc.element_format,
                                vat.offset,
                            )
                        },
                    }

                    //Note: there is apparently a bug on AMD 3500U:
                    // this call is ignored if the current array is disabled.
                    unsafe { gl.vertex_attrib_binding(vat.location, vat.buffer_index) };
                } else {
                    match vat.format_desc.attrib_kind {
                        super::VertexAttribKind::Float => unsafe {
                            gl.vertex_attrib_pointer_f32(
                                vat.location,
                                vat.format_desc.element_count,
                                vat.format_desc.element_format,
                                true, // always normalized
                                buffer_desc.stride as i32,
                                vat.offset as i32,
                            )
                        },
                        super::VertexAttribKind::Integer => unsafe {
                            gl.vertex_attrib_pointer_i32(
                                vat.location,
                                vat.format_desc.element_count,
                                vat.format_desc.element_format,
                                buffer_desc.stride as i32,
                                vat.offset as i32,
                            )
                        },
                    }
                    unsafe { gl.vertex_attrib_divisor(vat.location, buffer_desc.step as u32) };
                }
            }
            C::UnsetVertexAttribute(location) => {
                unsafe { gl.disable_vertex_attrib_array(location) };
            }
            C::SetVertexBuffer {
                index,
                ref buffer,
                ref buffer_desc,
            } => {
                unsafe { gl.vertex_binding_divisor(index, buffer_desc.step as u32) };
                unsafe {
                    gl.bind_vertex_buffer(
                        index,
                        Some(buffer.raw),
                        buffer.offset as i32,
                        buffer_desc.stride as i32,
                    )
                };
            }
            C::SetDepth(ref depth) => {
                unsafe { gl.depth_func(depth.function) };
                unsafe { gl.depth_mask(depth.mask) };
            }
            C::SetDepthBias(bias) => {
                if bias.is_enabled() {
                    unsafe { gl.enable(glow::POLYGON_OFFSET_FILL) };
                    unsafe { gl.polygon_offset(bias.slope_scale, bias.constant as f32) };
                } else {
                    unsafe { gl.disable(glow::POLYGON_OFFSET_FILL) };
                }
            }
            C::ConfigureDepthStencil(aspects) => {
                if aspects.contains(crate::FormatAspects::DEPTH) {
                    unsafe { gl.enable(glow::DEPTH_TEST) };
                } else {
                    unsafe { gl.disable(glow::DEPTH_TEST) };
                }
                if aspects.contains(crate::FormatAspects::STENCIL) {
                    unsafe { gl.enable(glow::STENCIL_TEST) };
                } else {
                    unsafe { gl.disable(glow::STENCIL_TEST) };
                }
            }
            C::SetAlphaToCoverage(enabled) => {
                if enabled {
                    unsafe { gl.enable(glow::SAMPLE_ALPHA_TO_COVERAGE) };
                } else {
                    unsafe { gl.disable(glow::SAMPLE_ALPHA_TO_COVERAGE) };
                }
            }
            C::SetProgram(program) => {
                unsafe { gl.use_program(Some(program)) };
            }
            C::SetPrimitive(ref state) => {
                unsafe { gl.front_face(state.front_face) };
                if state.cull_face != 0 {
                    unsafe { gl.enable(glow::CULL_FACE) };
                    unsafe { gl.cull_face(state.cull_face) };
                } else {
                    unsafe { gl.disable(glow::CULL_FACE) };
                }
                if self.features.contains(wgt::Features::DEPTH_CLIP_CONTROL) {
                    //Note: this is a bit tricky, since we are controlling the clip, not the clamp.
                    if state.unclipped_depth {
                        unsafe { gl.enable(glow::DEPTH_CLAMP) };
                    } else {
                        unsafe { gl.disable(glow::DEPTH_CLAMP) };
                    }
                }
                // POLYGON_MODE_LINE also implies POLYGON_MODE_POINT
                if self.features.contains(wgt::Features::POLYGON_MODE_LINE) {
                    unsafe { gl.polygon_mode(glow::FRONT_AND_BACK, state.polygon_mode) };
                }
            }
            C::SetBlendConstant(c) => {
                unsafe { gl.blend_color(c[0], c[1], c[2], c[3]) };
            }
            C::SetColorTarget {
                draw_buffer_index,
                desc: super::ColorTargetDesc { mask, ref blend },
            } => {
                use wgt::ColorWrites as Cw;
                if let Some(index) = draw_buffer_index {
                    unsafe {
                        gl.color_mask_draw_buffer(
                            index,
                            mask.contains(Cw::RED),
                            mask.contains(Cw::GREEN),
                            mask.contains(Cw::BLUE),
                            mask.contains(Cw::ALPHA),
                        )
                    };
                    if let Some(ref blend) = *blend {
                        unsafe { gl.enable_draw_buffer(glow::BLEND, index) };
                        if blend.color != blend.alpha {
                            unsafe {
                                gl.blend_equation_separate_draw_buffer(
                                    index,
                                    blend.color.equation,
                                    blend.alpha.equation,
                                )
                            };
                            unsafe {
                                gl.blend_func_separate_draw_buffer(
                                    index,
                                    blend.color.src,
                                    blend.color.dst,
                                    blend.alpha.src,
                                    blend.alpha.dst,
                                )
                            };
                        } else {
                            unsafe { gl.blend_equation_draw_buffer(index, blend.color.equation) };
                            unsafe {
                                gl.blend_func_draw_buffer(index, blend.color.src, blend.color.dst)
                            };
                        }
                    } else {
                        unsafe { gl.disable_draw_buffer(glow::BLEND, index) };
                    }
                } else {
                    unsafe {
                        gl.color_mask(
                            mask.contains(Cw::RED),
                            mask.contains(Cw::GREEN),
                            mask.contains(Cw::BLUE),
                            mask.contains(Cw::ALPHA),
                        )
                    };
                    if let Some(ref blend) = *blend {
                        unsafe { gl.enable(glow::BLEND) };
                        if blend.color != blend.alpha {
                            unsafe {
                                gl.blend_equation_separate(
                                    blend.color.equation,
                                    blend.alpha.equation,
                                )
                            };
                            unsafe {
                                gl.blend_func_separate(
                                    blend.color.src,
                                    blend.color.dst,
                                    blend.alpha.src,
                                    blend.alpha.dst,
                                )
                            };
                        } else {
                            unsafe { gl.blend_equation(blend.color.equation) };
                            unsafe { gl.blend_func(blend.color.src, blend.color.dst) };
                        }
                    } else {
                        unsafe { gl.disable(glow::BLEND) };
                    }
                }
            }
            C::BindBuffer {
                target,
                slot,
                buffer,
                offset,
                size,
            } => {
                unsafe { gl.bind_buffer_range(target, slot, Some(buffer), offset, size) };
            }
            C::BindSampler(texture_index, sampler) => {
                unsafe { gl.bind_sampler(texture_index, sampler) };
            }
            C::BindTexture {
                slot,
                texture,
                target,
                aspects,
                ref mip_levels,
            } => {
                unsafe { gl.active_texture(glow::TEXTURE0 + slot) };
                unsafe { gl.bind_texture(target, Some(texture)) };

                unsafe {
                    gl.tex_parameter_i32(target, glow::TEXTURE_BASE_LEVEL, mip_levels.start as i32)
                };
                unsafe {
                    gl.tex_parameter_i32(
                        target,
                        glow::TEXTURE_MAX_LEVEL,
                        (mip_levels.end - 1) as i32,
                    )
                };

                let version = gl.version();
                let is_min_es_3_1 = version.is_embedded && (version.major, version.minor) >= (3, 1);
                let is_min_4_3 = !version.is_embedded && (version.major, version.minor) >= (4, 3);
                if is_min_es_3_1 || is_min_4_3 {
                    let mode = match aspects {
                        crate::FormatAspects::DEPTH => Some(glow::DEPTH_COMPONENT),
                        crate::FormatAspects::STENCIL => Some(glow::STENCIL_INDEX),
                        _ => None,
                    };
                    if let Some(mode) = mode {
                        unsafe {
                            gl.tex_parameter_i32(
                                target,
                                glow::DEPTH_STENCIL_TEXTURE_MODE,
                                mode as _,
                            )
                        };
                    }
                }
            }
            C::BindImage { slot, ref binding } => {
                unsafe {
                    gl.bind_image_texture(
                        slot,
                        Some(binding.raw),
                        binding.mip_level as i32,
                        binding.array_layer.is_none(),
                        binding.array_layer.unwrap_or_default() as i32,
                        binding.access,
                        binding.format,
                    )
                };
            }
            C::InsertDebugMarker(ref range) => {
                let marker = extract_marker(data_bytes, range);
                unsafe {
                    if self
                        .shared
                        .private_caps
                        .contains(PrivateCapabilities::DEBUG_FNS)
                    {
                        gl.debug_message_insert(
                            glow::DEBUG_SOURCE_APPLICATION,
                            glow::DEBUG_TYPE_MARKER,
                            DEBUG_ID,
                            glow::DEBUG_SEVERITY_NOTIFICATION,
                            to_debug_str(marker),
                        )
                    }
                };
            }
            C::PushDebugGroup(ref range) => {
                let marker = extract_marker(data_bytes, range);
                unsafe {
                    if self
                        .shared
                        .private_caps
                        .contains(PrivateCapabilities::DEBUG_FNS)
                    {
                        gl.push_debug_group(
                            glow::DEBUG_SOURCE_APPLICATION,
                            DEBUG_ID,
                            to_debug_str(marker),
                        )
                    }
                };
            }
            C::PopDebugGroup => {
                unsafe {
                    if self
                        .shared
                        .private_caps
                        .contains(PrivateCapabilities::DEBUG_FNS)
                    {
                        gl.pop_debug_group()
                    }
                };
            }
            C::SetImmediates {
                ref uniform,
                offset,
            } => {
                fn get_data<T, const COUNT: usize>(data: &[u8], offset: u32) -> [T; COUNT]
                where
                    [T; COUNT]: bytemuck::AnyBitPattern,
                {
                    let data_required = size_of::<T>() * COUNT;
                    let raw = &data[(offset as usize)..][..data_required];
                    bytemuck::pod_read_unaligned(raw)
                }

                let location = Some(&uniform.location);

                match uniform.ty {
                    //
                    // --- Float 1-4 Component ---
                    //
                    naga::TypeInner::Scalar(naga::Scalar::F32) => {
                        let data = get_data::<f32, 1>(data_bytes, offset)[0];
                        unsafe { gl.uniform_1_f32(location, data) };
                    }
                    naga::TypeInner::Vector {
                        size: naga::VectorSize::Bi,
                        scalar: naga::Scalar::F32,
                    } => {
                        let data = &get_data::<f32, 2>(data_bytes, offset);
                        unsafe { gl.uniform_2_f32_slice(location, data) };
                    }
                    naga::TypeInner::Vector {
                        size: naga::VectorSize::Tri,
                        scalar: naga::Scalar::F32,
                    } => {
                        let data = &get_data::<f32, 3>(data_bytes, offset);
                        unsafe { gl.uniform_3_f32_slice(location, data) };
                    }
                    naga::TypeInner::Vector {
                        size: naga::VectorSize::Quad,
                        scalar: naga::Scalar::F32,
                    } => {
                        let data = &get_data::<f32, 4>(data_bytes, offset);
                        unsafe { gl.uniform_4_f32_slice(location, data) };
                    }

                    //
                    // --- Int 1-4 Component ---
                    //
                    naga::TypeInner::Scalar(naga::Scalar::I32) => {
                        let data = get_data::<i32, 1>(data_bytes, offset)[0];
                        unsafe { gl.uniform_1_i32(location, data) };
                    }
                    naga::TypeInner::Vector {
                        size: naga::VectorSize::Bi,
                        scalar: naga::Scalar::I32,
                    } => {
                        let data = &get_data::<i32, 2>(data_bytes, offset);
                        unsafe { gl.uniform_2_i32_slice(location, data) };
                    }
                    naga::TypeInner::Vector {
                        size: naga::VectorSize::Tri,
                        scalar: naga::Scalar::I32,
                    } => {
                        let data = &get_data::<i32, 3>(data_bytes, offset);
                        unsafe { gl.uniform_3_i32_slice(location, data) };
                    }
                    naga::TypeInner::Vector {
                        size: naga::VectorSize::Quad,
                        scalar: naga::Scalar::I32,
                    } => {
                        let data = &get_data::<i32, 4>(data_bytes, offset);
                        unsafe { gl.uniform_4_i32_slice(location, data) };
                    }

                    //
                    // --- Uint 1-4 Component ---
                    //
                    naga::TypeInner::Scalar(naga::Scalar::U32) => {
                        let data = get_data::<u32, 1>(data_bytes, offset)[0];
                        unsafe { gl.uniform_1_u32(location, data) };
                    }
                    naga::TypeInner::Vector {
                        size: naga::VectorSize::Bi,
                        scalar: naga::Scalar::U32,
                    } => {
                        let data = &get_data::<u32, 2>(data_bytes, offset);
                        unsafe { gl.uniform_2_u32_slice(location, data) };
                    }
                    naga::TypeInner::Vector {
                        size: naga::VectorSize::Tri,
                        scalar: naga::Scalar::U32,
                    } => {
                        let data = &get_data::<u32, 3>(data_bytes, offset);
                        unsafe { gl.uniform_3_u32_slice(location, data) };
                    }
                    naga::TypeInner::Vector {
                        size: naga::VectorSize::Quad,
                        scalar: naga::Scalar::U32,
                    } => {
                        let data = &get_data::<u32, 4>(data_bytes, offset);
                        unsafe { gl.uniform_4_u32_slice(location, data) };
                    }

                    //
                    // --- Matrix 2xR ---
                    //
                    naga::TypeInner::Matrix {
                        columns: naga::VectorSize::Bi,
                        rows: naga::VectorSize::Bi,
                        scalar: naga::Scalar::F32,
                    } => {
                        let data = &get_data::<f32, 4>(data_bytes, offset);
                        unsafe { gl.uniform_matrix_2_f32_slice(location, false, data) };
                    }
                    naga::TypeInner::Matrix {
                        columns: naga::VectorSize::Bi,
                        rows: naga::VectorSize::Tri,
                        scalar: naga::Scalar::F32,
                    } => {
                        // repack 2 vec3s into 6 values.
                        let unpacked_data = &get_data::<f32, 8>(data_bytes, offset);
                        #[rustfmt::skip]
                        let packed_data = [
                            unpacked_data[0], unpacked_data[1], unpacked_data[2],
                            unpacked_data[4], unpacked_data[5], unpacked_data[6],
                        ];
                        unsafe { gl.uniform_matrix_2x3_f32_slice(location, false, &packed_data) };
                    }
                    naga::TypeInner::Matrix {
                        columns: naga::VectorSize::Bi,
                        rows: naga::VectorSize::Quad,
                        scalar: naga::Scalar::F32,
                    } => {
                        let data = &get_data::<f32, 8>(data_bytes, offset);
                        unsafe { gl.uniform_matrix_2x4_f32_slice(location, false, data) };
                    }

                    //
                    // --- Matrix 3xR ---
                    //
                    naga::TypeInner::Matrix {
                        columns: naga::VectorSize::Tri,
                        rows: naga::VectorSize::Bi,
                        scalar: naga::Scalar::F32,
                    } => {
                        let data = &get_data::<f32, 6>(data_bytes, offset);
                        unsafe { gl.uniform_matrix_3x2_f32_slice(location, false, data) };
                    }
                    naga::TypeInner::Matrix {
                        columns: naga::VectorSize::Tri,
                        rows: naga::VectorSize::Tri,
                        scalar: naga::Scalar::F32,
                    } => {
                        // repack 3 vec3s into 9 values.
                        let unpacked_data = &get_data::<f32, 12>(data_bytes, offset);
                        #[rustfmt::skip]
                        let packed_data = [
                            unpacked_data[0], unpacked_data[1], unpacked_data[2],
                            unpacked_data[4], unpacked_data[5], unpacked_data[6],
                            unpacked_data[8], unpacked_data[9], unpacked_data[10],
                        ];
                        unsafe { gl.uniform_matrix_3_f32_slice(location, false, &packed_data) };
                    }
                    naga::TypeInner::Matrix {
                        columns: naga::VectorSize::Tri,
                        rows: naga::VectorSize::Quad,
                        scalar: naga::Scalar::F32,
                    } => {
                        let data = &get_data::<f32, 12>(data_bytes, offset);
                        unsafe { gl.uniform_matrix_3x4_f32_slice(location, false, data) };
                    }

                    //
                    // --- Matrix 4xR ---
                    //
                    naga::TypeInner::Matrix {
                        columns: naga::VectorSize::Quad,
                        rows: naga::VectorSize::Bi,
                        scalar: naga::Scalar::F32,
                    } => {
                        let data = &get_data::<f32, 8>(data_bytes, offset);
                        unsafe { gl.uniform_matrix_4x2_f32_slice(location, false, data) };
                    }
                    naga::TypeInner::Matrix {
                        columns: naga::VectorSize::Quad,
                        rows: naga::VectorSize::Tri,
                        scalar: naga::Scalar::F32,
                    } => {
                        // repack 4 vec3s into 12 values.
                        let unpacked_data = &get_data::<f32, 16>(data_bytes, offset);
                        #[rustfmt::skip]
                        let packed_data = [
                            unpacked_data[0], unpacked_data[1], unpacked_data[2],
                            unpacked_data[4], unpacked_data[5], unpacked_data[6],
                            unpacked_data[8], unpacked_data[9], unpacked_data[10],
                            unpacked_data[12], unpacked_data[13], unpacked_data[14],
                        ];
                        unsafe { gl.uniform_matrix_4x3_f32_slice(location, false, &packed_data) };
                    }
                    naga::TypeInner::Matrix {
                        columns: naga::VectorSize::Quad,
                        rows: naga::VectorSize::Quad,
                        scalar: naga::Scalar::F32,
                    } => {
                        let data = &get_data::<f32, 16>(data_bytes, offset);
                        unsafe { gl.uniform_matrix_4_f32_slice(location, false, data) };
                    }
                    _ => panic!("Unsupported uniform datatype: {:?}!", uniform.ty),
                }
            }
            C::SetClipDistances {
                old_count,
                new_count,
            } => {
                // Disable clip planes that are no longer active
                for i in new_count..old_count {
                    unsafe { gl.disable(glow::CLIP_DISTANCE0 + i) };
                }

                // Enable clip planes that are now active
                for i in old_count..new_count {
                    unsafe { gl.enable(glow::CLIP_DISTANCE0 + i) };
                }
            }
        }
    }
}

impl crate::Queue for super::Queue {
    type A = super::Api;

    unsafe fn submit(
        &self,
        command_buffers: &[&super::CommandBuffer],
        _surface_textures: &[&super::Texture],
        (signal_fence, signal_value): (&mut super::Fence, crate::FenceValue),
    ) -> Result<(), crate::DeviceError> {
        let shared = Arc::clone(&self.shared);
        let gl = &shared.context.lock();
        for cmd_buf in command_buffers.iter() {
            // The command encoder assumes a default state when encoding the command buffer.
            // Always reset the state between command_buffers to reflect this assumption. Do
            // this at the beginning of the loop in case something outside of wgpu modified
            // this state prior to commit.
            unsafe { self.reset_state(gl) };
            if let Some(ref label) = cmd_buf.label {
                if self
                    .shared
                    .private_caps
                    .contains(PrivateCapabilities::DEBUG_FNS)
                {
                    unsafe {
                        gl.push_debug_group(
                            glow::DEBUG_SOURCE_APPLICATION,
                            DEBUG_ID,
                            to_debug_str(label),
                        )
                    };
                }
            }

            for command in cmd_buf.commands.iter() {
                unsafe { self.process(gl, command, &cmd_buf.data_bytes, &cmd_buf.queries) };
            }

            if cmd_buf.label.is_some()
                && self
                    .shared
                    .private_caps
                    .contains(PrivateCapabilities::DEBUG_FNS)
            {
                unsafe { gl.pop_debug_group() };
            }
        }

        signal_fence.maintain(gl);
        signal_fence.signal(gl, signal_value)?;

        // This is extremely important. If we don't flush, the above fences may never
        // be signaled, particularly in headless contexts. Headed contexts will
        // often flush every so often, but headless contexts may not.
        unsafe { gl.flush() };

        Ok(())
    }

    unsafe fn present(
        &self,
        surface: &super::Surface,
        texture: super::Texture,
    ) -> Result<(), crate::SurfaceError> {
        unsafe { surface.present(texture, &self.shared.context) }
    }

    unsafe fn get_timestamp_period(&self) -> f32 {
        1.0
    }
}

#[cfg(send_sync)]
unsafe impl Sync for super::Queue {}
#[cfg(send_sync)]
unsafe impl Send for super::Queue {}
