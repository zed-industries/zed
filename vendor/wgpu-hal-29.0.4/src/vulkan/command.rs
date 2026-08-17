use super::conv;
use arrayvec::ArrayVec;
use ash::vk;
use core::{mem, ops::Range};
use hashbrown::hash_map::Entry;

const ALLOCATION_GRANULARITY: u32 = 16;
const DST_IMAGE_LAYOUT: vk::ImageLayout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;

impl super::Texture {
    fn map_buffer_copies<T>(&self, regions: T) -> impl Iterator<Item = vk::BufferImageCopy>
    where
        T: Iterator<Item = crate::BufferTextureCopy>,
    {
        let (block_width, block_height) = self.format.block_dimensions();
        let format = self.format;
        let copy_size = self.copy_size;
        regions.map(move |r| {
            let extent = r.texture_base.max_copy_size(&copy_size).min(&r.size);
            let (image_subresource, image_offset) = conv::map_subresource_layers(&r.texture_base);
            vk::BufferImageCopy {
                buffer_offset: r.buffer_layout.offset,
                buffer_row_length: r.buffer_layout.bytes_per_row.map_or(0, |bpr| {
                    let block_size = format
                        .block_copy_size(Some(r.texture_base.aspect.map()))
                        .unwrap();
                    block_width * (bpr / block_size)
                }),
                buffer_image_height: r
                    .buffer_layout
                    .rows_per_image
                    .map_or(0, |rpi| rpi * block_height),
                image_subresource,
                image_offset,
                image_extent: conv::map_copy_extent(&extent),
            }
        })
    }
}

impl super::CommandEncoder {
    fn write_pass_end_timestamp_if_requested(&mut self) {
        if let Some((query_set, index)) = self.end_of_pass_timer_query.take() {
            unsafe {
                self.device.raw.cmd_write_timestamp(
                    self.active,
                    vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                    query_set,
                    index,
                );
            }
        }
    }

    fn make_framebuffer(
        &mut self,
        key: super::FramebufferKey,
    ) -> Result<vk::Framebuffer, crate::DeviceError> {
        Ok(match self.framebuffers.entry(key) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let super::FramebufferKey {
                    raw_pass,
                    ref attachment_views,
                    attachment_identities: _,
                    extent,
                } = *e.key();

                let vk_info = vk::FramebufferCreateInfo::default()
                    .render_pass(raw_pass)
                    .width(extent.width)
                    .height(extent.height)
                    .layers(extent.depth_or_array_layers)
                    .attachments(attachment_views);

                let raw = unsafe { self.device.raw.create_framebuffer(&vk_info, None).unwrap() };
                *e.insert(raw)
            }
        })
    }

    fn make_temp_texture_view(
        &mut self,
        key: super::TempTextureViewKey,
    ) -> Result<super::IdentifiedTextureView, crate::DeviceError> {
        Ok(match self.temp_texture_views.entry(key) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let super::TempTextureViewKey {
                    texture,
                    texture_identity: _,
                    format,
                    mip_level,
                    depth_slice,
                } = *e.key();

                let vk_info = vk::ImageViewCreateInfo::default()
                    .image(texture)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(format)
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: mip_level,
                        level_count: 1,
                        base_array_layer: depth_slice,
                        layer_count: 1,
                    });
                let raw = unsafe { self.device.raw.create_image_view(&vk_info, None) }
                    .map_err(super::map_host_device_oom_and_ioca_err)?;

                let identity = self.device.texture_view_identity_factory.next();

                *e.insert(super::IdentifiedTextureView { raw, identity })
            }
        })
    }
}

impl crate::CommandEncoder for super::CommandEncoder {
    type A = super::Api;

    unsafe fn begin_encoding(&mut self, label: crate::Label) -> Result<(), crate::DeviceError> {
        if self.free.is_empty() {
            let vk_info = vk::CommandBufferAllocateInfo::default()
                .command_pool(self.raw)
                .command_buffer_count(ALLOCATION_GRANULARITY);
            let cmd_buf_vec = unsafe {
                self.device
                    .raw
                    .allocate_command_buffers(&vk_info)
                    .map_err(super::map_host_device_oom_err)?
            };
            self.free.extend(cmd_buf_vec);
        }
        let raw = self.free.pop().unwrap();

        // Set the name unconditionally, since there might be a
        // previous name assigned to this.
        unsafe { self.device.set_object_name(raw, label.unwrap_or_default()) };

        // Reset this in case the last renderpass was never ended.
        self.rpass_debug_marker_active = false;

        let vk_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe { self.device.raw.begin_command_buffer(raw, &vk_info) }
            .map_err(super::map_host_device_oom_err)?;
        self.active = raw;

        Ok(())
    }

    unsafe fn end_encoding(&mut self) -> Result<super::CommandBuffer, crate::DeviceError> {
        let raw = self.active;
        self.active = vk::CommandBuffer::null();
        unsafe { self.device.raw.end_command_buffer(raw) }.map_err(map_err)?;
        fn map_err(err: vk::Result) -> crate::DeviceError {
            // We don't use VK_KHR_video_encode_queue
            // VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR
            super::map_host_device_oom_err(err)
        }
        Ok(super::CommandBuffer { raw })
    }

    unsafe fn discard_encoding(&mut self) {
        // Safe use requires this is not called in the "closed" state, so the buffer
        // shouldn't be null. Assert this to make sure we're not pushing null
        // buffers to the discard pile.
        assert_ne!(self.active, vk::CommandBuffer::null());

        self.discarded.push(self.active);
        self.active = vk::CommandBuffer::null();
    }

    unsafe fn reset_all<I>(&mut self, cmd_bufs: I)
    where
        I: Iterator<Item = super::CommandBuffer>,
    {
        self.temp.clear();
        self.free
            .extend(cmd_bufs.into_iter().map(|cmd_buf| cmd_buf.raw));
        self.free.append(&mut self.discarded);
        // Delete framebuffers from the framebuffer cache
        for (_, framebuffer) in self.framebuffers.drain() {
            unsafe { self.device.raw.destroy_framebuffer(framebuffer, None) };
        }
        let _ = unsafe {
            self.device
                .raw
                .reset_command_pool(self.raw, vk::CommandPoolResetFlags::default())
        };
    }

    unsafe fn transition_buffers<'a, T>(&mut self, barriers: T)
    where
        T: Iterator<Item = crate::BufferBarrier<'a, super::Buffer>>,
    {
        //Note: this is done so that we never end up with empty stage flags
        let mut src_stages = vk::PipelineStageFlags::TOP_OF_PIPE;
        let mut dst_stages = vk::PipelineStageFlags::BOTTOM_OF_PIPE;
        let vk_barriers = &mut self.temp.buffer_barriers;
        vk_barriers.clear();

        for bar in barriers {
            let (src_stage, src_access) = conv::map_buffer_usage_to_barrier(bar.usage.from);
            src_stages |= src_stage;
            let (dst_stage, dst_access) = conv::map_buffer_usage_to_barrier(bar.usage.to);
            dst_stages |= dst_stage;

            vk_barriers.push(
                vk::BufferMemoryBarrier::default()
                    .buffer(bar.buffer.raw)
                    .size(vk::WHOLE_SIZE)
                    .src_access_mask(src_access)
                    .dst_access_mask(dst_access),
            )
        }

        if !vk_barriers.is_empty() {
            unsafe {
                self.device.raw.cmd_pipeline_barrier(
                    self.active,
                    src_stages,
                    dst_stages,
                    vk::DependencyFlags::empty(),
                    &[],
                    vk_barriers,
                    &[],
                )
            };
        }
    }

    unsafe fn transition_textures<'a, T>(&mut self, barriers: T)
    where
        T: Iterator<Item = crate::TextureBarrier<'a, super::Texture>>,
    {
        let mut src_stages = vk::PipelineStageFlags::empty();
        let mut dst_stages = vk::PipelineStageFlags::empty();
        let vk_barriers = &mut self.temp.image_barriers;
        vk_barriers.clear();

        for bar in barriers {
            let range = conv::map_subresource_range_combined_aspect(
                &bar.range,
                bar.texture.format,
                &self.device.private_caps,
            );
            let (src_stage, src_access) = conv::map_texture_usage_to_barrier(bar.usage.from);
            let src_layout = conv::derive_image_layout(bar.usage.from, bar.texture.format);
            src_stages |= src_stage;
            let (dst_stage, dst_access) = conv::map_texture_usage_to_barrier(bar.usage.to);
            let dst_layout = conv::derive_image_layout(bar.usage.to, bar.texture.format);
            dst_stages |= dst_stage;

            vk_barriers.push(
                vk::ImageMemoryBarrier::default()
                    .image(bar.texture.raw)
                    .subresource_range(range)
                    .src_access_mask(src_access)
                    .dst_access_mask(dst_access)
                    .old_layout(src_layout)
                    .new_layout(dst_layout),
            );
        }

        if !vk_barriers.is_empty() {
            unsafe {
                self.device.raw.cmd_pipeline_barrier(
                    self.active,
                    src_stages,
                    dst_stages,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    vk_barriers,
                )
            };
        }
    }

    unsafe fn clear_buffer(&mut self, buffer: &super::Buffer, range: crate::MemoryRange) {
        let range_size = range.end - range.start;
        if self.device.workarounds.contains(
            super::Workarounds::FORCE_FILL_BUFFER_WITH_SIZE_GREATER_4096_ALIGNED_OFFSET_16,
        ) && range_size >= 4096
            && !range.start.is_multiple_of(16)
        {
            let rounded_start = wgt::math::align_to(range.start, 16);
            let prefix_size = rounded_start - range.start;

            unsafe {
                self.device.raw.cmd_fill_buffer(
                    self.active,
                    buffer.raw,
                    range.start,
                    prefix_size,
                    0,
                )
            };

            // This will never be zero, as rounding can only add up to 12 bytes, and the total size is 4096.
            let suffix_size = range.end - rounded_start;

            unsafe {
                self.device.raw.cmd_fill_buffer(
                    self.active,
                    buffer.raw,
                    rounded_start,
                    suffix_size,
                    0,
                )
            };
        } else {
            unsafe {
                self.device
                    .raw
                    .cmd_fill_buffer(self.active, buffer.raw, range.start, range_size, 0)
            };
        }
    }

    unsafe fn copy_buffer_to_buffer<T>(
        &mut self,
        src: &super::Buffer,
        dst: &super::Buffer,
        regions: T,
    ) where
        T: Iterator<Item = crate::BufferCopy>,
    {
        let vk_regions_iter = regions.map(|r| vk::BufferCopy {
            src_offset: r.src_offset,
            dst_offset: r.dst_offset,
            size: r.size.get(),
        });

        unsafe {
            self.device.raw.cmd_copy_buffer(
                self.active,
                src.raw,
                dst.raw,
                &smallvec::SmallVec::<[vk::BufferCopy; 32]>::from_iter(vk_regions_iter),
            )
        };
    }

    unsafe fn copy_texture_to_texture<T>(
        &mut self,
        src: &super::Texture,
        src_usage: wgt::TextureUses,
        dst: &super::Texture,
        regions: T,
    ) where
        T: Iterator<Item = crate::TextureCopy>,
    {
        let src_layout = conv::derive_image_layout(src_usage, src.format);

        let vk_regions_iter = regions.map(|r| {
            let (src_subresource, src_offset) = conv::map_subresource_layers(&r.src_base);
            let (dst_subresource, dst_offset) = conv::map_subresource_layers(&r.dst_base);
            let extent = r
                .size
                .min(&r.src_base.max_copy_size(&src.copy_size))
                .min(&r.dst_base.max_copy_size(&dst.copy_size));
            vk::ImageCopy {
                src_subresource,
                src_offset,
                dst_subresource,
                dst_offset,
                extent: conv::map_copy_extent(&extent),
            }
        });

        unsafe {
            self.device.raw.cmd_copy_image(
                self.active,
                src.raw,
                src_layout,
                dst.raw,
                DST_IMAGE_LAYOUT,
                &smallvec::SmallVec::<[vk::ImageCopy; 32]>::from_iter(vk_regions_iter),
            )
        };
    }

    unsafe fn copy_buffer_to_texture<T>(
        &mut self,
        src: &super::Buffer,
        dst: &super::Texture,
        regions: T,
    ) where
        T: Iterator<Item = crate::BufferTextureCopy>,
    {
        let vk_regions_iter = dst.map_buffer_copies(regions);

        unsafe {
            self.device.raw.cmd_copy_buffer_to_image(
                self.active,
                src.raw,
                dst.raw,
                DST_IMAGE_LAYOUT,
                &smallvec::SmallVec::<[vk::BufferImageCopy; 32]>::from_iter(vk_regions_iter),
            )
        };
    }

    unsafe fn copy_texture_to_buffer<T>(
        &mut self,
        src: &super::Texture,
        src_usage: wgt::TextureUses,
        dst: &super::Buffer,
        regions: T,
    ) where
        T: Iterator<Item = crate::BufferTextureCopy>,
    {
        let src_layout = conv::derive_image_layout(src_usage, src.format);
        let vk_regions_iter = src.map_buffer_copies(regions);

        unsafe {
            self.device.raw.cmd_copy_image_to_buffer(
                self.active,
                src.raw,
                src_layout,
                dst.raw,
                &smallvec::SmallVec::<[vk::BufferImageCopy; 32]>::from_iter(vk_regions_iter),
            )
        };
    }

    unsafe fn begin_query(&mut self, set: &super::QuerySet, index: u32) {
        unsafe {
            self.device.raw.cmd_begin_query(
                self.active,
                set.raw,
                index,
                vk::QueryControlFlags::empty(),
            )
        };
    }
    unsafe fn end_query(&mut self, set: &super::QuerySet, index: u32) {
        unsafe { self.device.raw.cmd_end_query(self.active, set.raw, index) };
    }
    unsafe fn write_timestamp(&mut self, set: &super::QuerySet, index: u32) {
        unsafe {
            self.device.raw.cmd_write_timestamp(
                self.active,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                set.raw,
                index,
            )
        };
    }
    unsafe fn read_acceleration_structure_compact_size(
        &mut self,
        acceleration_structure: &super::AccelerationStructure,
        buffer: &super::Buffer,
    ) {
        let ray_tracing_functions = self
            .device
            .extension_fns
            .ray_tracing
            .as_ref()
            .expect("Feature `RAY_TRACING` not enabled");
        let query_pool = acceleration_structure
            .compacted_size_query
            .as_ref()
            .unwrap();
        unsafe {
            self.device
                .raw
                .cmd_reset_query_pool(self.active, *query_pool, 0, 1);
            ray_tracing_functions
                .acceleration_structure
                .cmd_write_acceleration_structures_properties(
                    self.active,
                    &[acceleration_structure.raw],
                    vk::QueryType::ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR,
                    *query_pool,
                    0,
                );
            self.device.raw.cmd_copy_query_pool_results(
                self.active,
                *query_pool,
                0,
                1,
                buffer.raw,
                0,
                wgt::QUERY_SIZE as vk::DeviceSize,
                vk::QueryResultFlags::TYPE_64 | vk::QueryResultFlags::WAIT,
            )
        };
    }
    unsafe fn reset_queries(&mut self, set: &super::QuerySet, range: Range<u32>) {
        unsafe {
            self.device.raw.cmd_reset_query_pool(
                self.active,
                set.raw,
                range.start,
                range.end - range.start,
            )
        };
    }
    unsafe fn copy_query_results(
        &mut self,
        set: &super::QuerySet,
        range: Range<u32>,
        buffer: &super::Buffer,
        offset: wgt::BufferAddress,
        stride: wgt::BufferSize,
    ) {
        unsafe {
            self.device.raw.cmd_copy_query_pool_results(
                self.active,
                set.raw,
                range.start,
                range.end - range.start,
                buffer.raw,
                offset,
                stride.get(),
                vk::QueryResultFlags::TYPE_64 | vk::QueryResultFlags::WAIT,
            )
        };
    }

    unsafe fn build_acceleration_structures<'a, T>(&mut self, descriptor_count: u32, descriptors: T)
    where
        super::Api: 'a,
        T: IntoIterator<
            Item = crate::BuildAccelerationStructureDescriptor<
                'a,
                super::Buffer,
                super::AccelerationStructure,
            >,
        >,
    {
        const CAPACITY_OUTER: usize = 8;
        const CAPACITY_INNER: usize = 1;
        let descriptor_count = descriptor_count as usize;

        let ray_tracing_functions = self
            .device
            .extension_fns
            .ray_tracing
            .as_ref()
            .expect("Feature `RAY_TRACING` not enabled");

        let get_device_address = |buffer: Option<&super::Buffer>| unsafe {
            match buffer {
                Some(buffer) => ray_tracing_functions
                    .buffer_device_address
                    .get_buffer_device_address(
                        &vk::BufferDeviceAddressInfo::default().buffer(buffer.raw),
                    ),
                None => panic!("Buffers are required to build acceleration structures"),
            }
        };

        // storage to all the data required for cmd_build_acceleration_structures
        let mut ranges_storage = smallvec::SmallVec::<
            [smallvec::SmallVec<[vk::AccelerationStructureBuildRangeInfoKHR; CAPACITY_INNER]>;
                CAPACITY_OUTER],
        >::with_capacity(descriptor_count);
        let mut geometries_storage = smallvec::SmallVec::<
            [smallvec::SmallVec<[vk::AccelerationStructureGeometryKHR; CAPACITY_INNER]>;
                CAPACITY_OUTER],
        >::with_capacity(descriptor_count);

        // pointers to all the data required for cmd_build_acceleration_structures
        let mut geometry_infos = smallvec::SmallVec::<
            [vk::AccelerationStructureBuildGeometryInfoKHR; CAPACITY_OUTER],
        >::with_capacity(descriptor_count);
        let mut ranges_ptrs = smallvec::SmallVec::<
            [&[vk::AccelerationStructureBuildRangeInfoKHR]; CAPACITY_OUTER],
        >::with_capacity(descriptor_count);

        for desc in descriptors {
            let (geometries, ranges) = match *desc.entries {
                crate::AccelerationStructureEntries::Instances(ref instances) => {
                    let instance_data = vk::AccelerationStructureGeometryInstancesDataKHR::default(
                    // TODO: Code is so large that rustfmt refuses to treat this... :(
                    )
                    .data(vk::DeviceOrHostAddressConstKHR {
                        device_address: get_device_address(instances.buffer),
                    });

                    let geometry = vk::AccelerationStructureGeometryKHR::default()
                        .geometry_type(vk::GeometryTypeKHR::INSTANCES)
                        .geometry(vk::AccelerationStructureGeometryDataKHR {
                            instances: instance_data,
                        });

                    let range = vk::AccelerationStructureBuildRangeInfoKHR::default()
                        .primitive_count(instances.count)
                        .primitive_offset(instances.offset);

                    (smallvec::smallvec![geometry], smallvec::smallvec![range])
                }
                crate::AccelerationStructureEntries::Triangles(ref in_geometries) => {
                    let mut ranges = smallvec::SmallVec::<
                        [vk::AccelerationStructureBuildRangeInfoKHR; CAPACITY_INNER],
                    >::with_capacity(in_geometries.len());
                    let mut geometries = smallvec::SmallVec::<
                        [vk::AccelerationStructureGeometryKHR; CAPACITY_INNER],
                    >::with_capacity(in_geometries.len());
                    for triangles in in_geometries {
                        let mut triangle_data =
                            vk::AccelerationStructureGeometryTrianglesDataKHR::default()
                                // IndexType::NONE_KHR is not set by default (due to being provided by VK_KHR_acceleration_structure) but unless there is an
                                // index buffer we need to have IndexType::NONE_KHR as our index type.
                                .index_type(vk::IndexType::NONE_KHR)
                                .vertex_data(vk::DeviceOrHostAddressConstKHR {
                                    device_address: get_device_address(triangles.vertex_buffer)
                                        + (triangles.first_vertex as u64 * triangles.vertex_stride),
                                })
                                .vertex_format(conv::map_vertex_format(triangles.vertex_format))
                                .max_vertex(triangles.vertex_count)
                                .vertex_stride(triangles.vertex_stride);

                        let mut range = vk::AccelerationStructureBuildRangeInfoKHR::default();

                        if let Some(ref indices) = triangles.indices {
                            triangle_data = triangle_data
                                .index_data(vk::DeviceOrHostAddressConstKHR {
                                    device_address: get_device_address(indices.buffer),
                                })
                                .index_type(conv::map_index_format(indices.format));

                            range = range
                                .primitive_count(indices.count / 3)
                                .primitive_offset(indices.offset);
                        } else {
                            range = range.primitive_count(triangles.vertex_count / 3);
                        }

                        if let Some(ref transform) = triangles.transform {
                            let transform_device_address = unsafe {
                                ray_tracing_functions
                                    .buffer_device_address
                                    .get_buffer_device_address(
                                        &vk::BufferDeviceAddressInfo::default()
                                            .buffer(transform.buffer.raw),
                                    )
                            };
                            triangle_data =
                                triangle_data.transform_data(vk::DeviceOrHostAddressConstKHR {
                                    device_address: transform_device_address,
                                });

                            range = range.transform_offset(transform.offset);
                        }

                        let geometry = vk::AccelerationStructureGeometryKHR::default()
                            .geometry_type(vk::GeometryTypeKHR::TRIANGLES)
                            .geometry(vk::AccelerationStructureGeometryDataKHR {
                                triangles: triangle_data,
                            })
                            .flags(conv::map_acceleration_structure_geometry_flags(
                                triangles.flags,
                            ));

                        geometries.push(geometry);
                        ranges.push(range);
                    }
                    (geometries, ranges)
                }
                crate::AccelerationStructureEntries::AABBs(ref in_geometries) => {
                    let mut ranges = smallvec::SmallVec::<
                        [vk::AccelerationStructureBuildRangeInfoKHR; CAPACITY_INNER],
                    >::with_capacity(in_geometries.len());
                    let mut geometries = smallvec::SmallVec::<
                        [vk::AccelerationStructureGeometryKHR; CAPACITY_INNER],
                    >::with_capacity(in_geometries.len());
                    for aabb in in_geometries {
                        let aabbs_data = vk::AccelerationStructureGeometryAabbsDataKHR::default()
                            .data(vk::DeviceOrHostAddressConstKHR {
                                device_address: get_device_address(aabb.buffer),
                            })
                            .stride(aabb.stride);

                        let range = vk::AccelerationStructureBuildRangeInfoKHR::default()
                            .primitive_count(aabb.count)
                            .primitive_offset(aabb.offset);

                        let geometry = vk::AccelerationStructureGeometryKHR::default()
                            .geometry_type(vk::GeometryTypeKHR::AABBS)
                            .geometry(vk::AccelerationStructureGeometryDataKHR {
                                aabbs: aabbs_data,
                            })
                            .flags(conv::map_acceleration_structure_geometry_flags(aabb.flags));

                        geometries.push(geometry);
                        ranges.push(range);
                    }
                    (geometries, ranges)
                }
            };

            ranges_storage.push(ranges);
            geometries_storage.push(geometries);

            let scratch_device_address = unsafe {
                ray_tracing_functions
                    .buffer_device_address
                    .get_buffer_device_address(
                        &vk::BufferDeviceAddressInfo::default().buffer(desc.scratch_buffer.raw),
                    )
            };
            let ty = match *desc.entries {
                crate::AccelerationStructureEntries::Instances(_) => {
                    vk::AccelerationStructureTypeKHR::TOP_LEVEL
                }
                _ => vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL,
            };
            let mut geometry_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
                .ty(ty)
                .mode(conv::map_acceleration_structure_build_mode(desc.mode))
                .flags(conv::map_acceleration_structure_flags(desc.flags))
                .dst_acceleration_structure(desc.destination_acceleration_structure.raw)
                .scratch_data(vk::DeviceOrHostAddressKHR {
                    device_address: scratch_device_address + desc.scratch_buffer_offset,
                });

            if desc.mode == crate::AccelerationStructureBuildMode::Update {
                geometry_info.src_acceleration_structure = desc
                    .source_acceleration_structure
                    .unwrap_or(desc.destination_acceleration_structure)
                    .raw;
            }

            geometry_infos.push(geometry_info);
        }

        for (i, geometry_info) in geometry_infos.iter_mut().enumerate() {
            geometry_info.geometry_count = geometries_storage[i].len() as u32;
            geometry_info.p_geometries = geometries_storage[i].as_ptr();
            ranges_ptrs.push(&ranges_storage[i]);
        }

        unsafe {
            ray_tracing_functions
                .acceleration_structure
                .cmd_build_acceleration_structures(self.active, &geometry_infos, &ranges_ptrs);
        }
    }

    unsafe fn place_acceleration_structure_barrier(
        &mut self,
        barrier: crate::AccelerationStructureBarrier,
    ) {
        let (src_stage, src_access) = conv::map_acceleration_structure_usage_to_barrier(
            barrier.usage.from,
            self.device.features,
        );
        let (dst_stage, dst_access) = conv::map_acceleration_structure_usage_to_barrier(
            barrier.usage.to,
            self.device.features,
        );

        unsafe {
            self.device.raw.cmd_pipeline_barrier(
                self.active,
                src_stage | vk::PipelineStageFlags::TOP_OF_PIPE,
                dst_stage | vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[vk::MemoryBarrier::default()
                    .src_access_mask(src_access)
                    .dst_access_mask(dst_access)],
                &[],
                &[],
            )
        };
    }

    unsafe fn set_acceleration_structure_dependencies(
        _command_buffers: &[&super::CommandBuffer],
        _dependencies: &[&super::AccelerationStructure],
    ) {
    }
    // render

    unsafe fn begin_render_pass(
        &mut self,
        desc: &crate::RenderPassDescriptor<super::QuerySet, super::TextureView>,
    ) -> Result<(), crate::DeviceError> {
        let mut vk_clear_values =
            ArrayVec::<vk::ClearValue, { super::MAX_TOTAL_ATTACHMENTS }>::new();
        let mut rp_key = super::RenderPassKey {
            colors: ArrayVec::default(),
            depth_stencil: None,
            sample_count: desc.sample_count,
            multiview_mask: desc.multiview_mask,
        };
        let mut fb_key = super::FramebufferKey {
            raw_pass: vk::RenderPass::null(),
            attachment_views: ArrayVec::default(),
            attachment_identities: ArrayVec::default(),
            extent: desc.extent,
        };

        for cat in desc.color_attachments {
            if let Some(cat) = cat.as_ref() {
                let color_view = if cat.target.view.dimension == wgt::TextureViewDimension::D3 {
                    let key = super::TempTextureViewKey {
                        texture: cat.target.view.raw_texture,
                        texture_identity: cat.target.view.texture_identity,
                        format: cat.target.view.raw_format,
                        mip_level: cat.target.view.base_mip_level,
                        depth_slice: cat.depth_slice.unwrap(),
                    };
                    self.make_temp_texture_view(key)?
                } else {
                    cat.target.view.identified_raw_view()
                };

                vk_clear_values.push(vk::ClearValue {
                    color: unsafe { cat.make_vk_clear_color() },
                });
                let color = super::ColorAttachmentKey {
                    base: cat.target.make_attachment_key(cat.ops),
                    resolve: cat.resolve_target.as_ref().map(|target| {
                        target.make_attachment_key(
                            crate::AttachmentOps::LOAD_CLEAR | crate::AttachmentOps::STORE,
                        )
                    }),
                };

                rp_key.colors.push(Some(color));
                fb_key.push_view(color_view);
                if let Some(ref at) = cat.resolve_target {
                    vk_clear_values.push(unsafe { mem::zeroed() });
                    fb_key.push_view(at.view.identified_raw_view());
                }
            } else {
                rp_key.colors.push(None);
            }
        }
        if let Some(ref ds) = desc.depth_stencil_attachment {
            vk_clear_values.push(vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: ds.clear_value.0,
                    stencil: ds.clear_value.1,
                },
            });
            rp_key.depth_stencil = Some(super::DepthStencilAttachmentKey {
                base: ds.target.make_attachment_key(ds.depth_ops),
                stencil_ops: ds.stencil_ops,
            });
            fb_key.push_view(ds.target.view.identified_raw_view());
        }

        let render_area = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D {
                width: desc.extent.width,
                height: desc.extent.height,
            },
        };
        let vk_viewports = [vk::Viewport {
            x: 0.0,
            y: desc.extent.height as f32,
            width: desc.extent.width as f32,
            height: -(desc.extent.height as f32),
            min_depth: 0.0,
            max_depth: 1.0,
        }];

        let raw_pass = self.device.make_render_pass(rp_key).unwrap();
        fb_key.raw_pass = raw_pass;
        let raw_framebuffer = self.make_framebuffer(fb_key).unwrap();

        let vk_info = vk::RenderPassBeginInfo::default()
            .render_pass(raw_pass)
            .render_area(render_area)
            .clear_values(&vk_clear_values)
            .framebuffer(raw_framebuffer);

        if let Some(label) = desc.label {
            unsafe { self.begin_debug_marker(label) };
            self.rpass_debug_marker_active = true;
        }

        // Start timestamp if any (before all other commands but after debug marker)
        if let Some(timestamp_writes) = desc.timestamp_writes.as_ref() {
            if let Some(index) = timestamp_writes.beginning_of_pass_write_index {
                unsafe {
                    self.write_timestamp(timestamp_writes.query_set, index);
                }
            }
            self.end_of_pass_timer_query = timestamp_writes
                .end_of_pass_write_index
                .map(|index| (timestamp_writes.query_set.raw, index));
        }

        unsafe {
            self.device
                .raw
                .cmd_set_viewport(self.active, 0, &vk_viewports);
            self.device
                .raw
                .cmd_set_scissor(self.active, 0, &[render_area]);
            self.device.raw.cmd_begin_render_pass(
                self.active,
                &vk_info,
                vk::SubpassContents::INLINE,
            );
        };

        self.bind_point = vk::PipelineBindPoint::GRAPHICS;

        Ok(())
    }
    unsafe fn end_render_pass(&mut self) {
        unsafe {
            self.device.raw.cmd_end_render_pass(self.active);
        }

        // After all other commands but before debug marker, so this is still seen as part of this pass.
        self.write_pass_end_timestamp_if_requested();

        if self.rpass_debug_marker_active {
            unsafe {
                self.end_debug_marker();
            }
            self.rpass_debug_marker_active = false;
        }
    }

    unsafe fn set_bind_group(
        &mut self,
        layout: &super::PipelineLayout,
        index: u32,
        group: &super::BindGroup,
        dynamic_offsets: &[wgt::DynamicOffset],
    ) {
        let sets = [*group.set.raw()];
        unsafe {
            self.device.raw.cmd_bind_descriptor_sets(
                self.active,
                self.bind_point,
                layout.raw,
                index,
                &sets,
                dynamic_offsets,
            )
        };
    }
    unsafe fn set_immediates(
        &mut self,
        layout: &super::PipelineLayout,
        offset_bytes: u32,
        data: &[u32],
    ) {
        unsafe {
            self.device.raw.cmd_push_constants(
                self.active,
                layout.raw,
                vk::ShaderStageFlags::ALL,
                offset_bytes,
                bytemuck::cast_slice(data),
            )
        };
    }

    unsafe fn insert_debug_marker(&mut self, label: &str) {
        if let Some(ext) = self.device.extension_fns.debug_utils.as_ref() {
            let cstr = self.temp.make_c_str(label);
            let vk_label = vk::DebugUtilsLabelEXT::default().label_name(cstr);
            unsafe { ext.cmd_insert_debug_utils_label(self.active, &vk_label) };
        }
    }
    unsafe fn begin_debug_marker(&mut self, group_label: &str) {
        if let Some(ext) = self.device.extension_fns.debug_utils.as_ref() {
            let cstr = self.temp.make_c_str(group_label);
            let vk_label = vk::DebugUtilsLabelEXT::default().label_name(cstr);
            unsafe { ext.cmd_begin_debug_utils_label(self.active, &vk_label) };
        }
    }
    unsafe fn end_debug_marker(&mut self) {
        if let Some(ext) = self.device.extension_fns.debug_utils.as_ref() {
            unsafe { ext.cmd_end_debug_utils_label(self.active) };
        }
    }

    unsafe fn set_render_pipeline(&mut self, pipeline: &super::RenderPipeline) {
        unsafe {
            self.current_pipeline_is_multiview = pipeline.is_multiview;
            self.device.raw.cmd_bind_pipeline(
                self.active,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline.raw,
            )
        };
    }

    unsafe fn set_index_buffer<'a>(
        &mut self,
        binding: crate::BufferBinding<'a, super::Buffer>,
        format: wgt::IndexFormat,
    ) {
        unsafe {
            self.device.raw.cmd_bind_index_buffer(
                self.active,
                binding.buffer.raw,
                binding.offset,
                conv::map_index_format(format),
            )
        };
    }
    unsafe fn set_vertex_buffer<'a>(
        &mut self,
        index: u32,
        binding: crate::BufferBinding<'a, super::Buffer>,
    ) {
        let vk_buffers = [binding.buffer.raw];
        let vk_offsets = [binding.offset];
        unsafe {
            self.device
                .raw
                .cmd_bind_vertex_buffers(self.active, index, &vk_buffers, &vk_offsets)
        };
    }
    unsafe fn set_viewport(&mut self, rect: &crate::Rect<f32>, depth_range: Range<f32>) {
        let vk_viewports = [vk::Viewport {
            x: rect.x,
            y: rect.y + rect.h,
            width: rect.w,
            height: -rect.h, // flip Y
            min_depth: depth_range.start,
            max_depth: depth_range.end,
        }];
        unsafe {
            self.device
                .raw
                .cmd_set_viewport(self.active, 0, &vk_viewports)
        };
    }
    unsafe fn set_scissor_rect(&mut self, rect: &crate::Rect<u32>) {
        let vk_scissors = [vk::Rect2D {
            offset: vk::Offset2D {
                x: rect.x as i32,
                y: rect.y as i32,
            },
            extent: vk::Extent2D {
                width: rect.w,
                height: rect.h,
            },
        }];
        unsafe {
            self.device
                .raw
                .cmd_set_scissor(self.active, 0, &vk_scissors)
        };
    }
    unsafe fn set_stencil_reference(&mut self, value: u32) {
        unsafe {
            self.device.raw.cmd_set_stencil_reference(
                self.active,
                vk::StencilFaceFlags::FRONT_AND_BACK,
                value,
            )
        };
    }
    unsafe fn set_blend_constants(&mut self, color: &[f32; 4]) {
        unsafe { self.device.raw.cmd_set_blend_constants(self.active, color) };
    }

    unsafe fn draw(
        &mut self,
        first_vertex: u32,
        vertex_count: u32,
        first_instance: u32,
        instance_count: u32,
    ) {
        if self.current_pipeline_is_multiview
            && (first_instance as u64 + instance_count as u64 - 1)
                > self.device.private_caps.multiview_instance_index_limit as u64
        {
            panic!("This vulkan device is affected by [#8333](https://github.com/gfx-rs/wgpu/issues/8333)");
        }
        unsafe {
            self.device.raw.cmd_draw(
                self.active,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        };
    }
    unsafe fn draw_indexed(
        &mut self,
        first_index: u32,
        index_count: u32,
        base_vertex: i32,
        first_instance: u32,
        instance_count: u32,
    ) {
        if self.current_pipeline_is_multiview
            && (first_instance as u64 + instance_count as u64 - 1)
                > self.device.private_caps.multiview_instance_index_limit as u64
        {
            panic!("This vulkan device is affected by [#8333](https://github.com/gfx-rs/wgpu/issues/8333)");
        }
        unsafe {
            self.device.raw.cmd_draw_indexed(
                self.active,
                index_count,
                instance_count,
                first_index,
                base_vertex,
                first_instance,
            )
        };
    }
    unsafe fn draw_mesh_tasks(
        &mut self,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        if let Some(ref t) = self.device.extension_fns.mesh_shading {
            unsafe {
                t.cmd_draw_mesh_tasks(self.active, group_count_x, group_count_y, group_count_z);
            };
        } else {
            panic!("Feature `MESH_SHADING` not enabled");
        }
    }
    unsafe fn draw_indirect(
        &mut self,
        buffer: &super::Buffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        if draw_count >= 1
            && self.device.private_caps.multi_draw_indirect
            && draw_count <= self.device.private_caps.max_draw_indirect_count
        {
            unsafe {
                self.device.raw.cmd_draw_indirect(
                    self.active,
                    buffer.raw,
                    offset,
                    draw_count,
                    size_of::<wgt::DrawIndirectArgs>() as u32,
                )
            };
        } else {
            for i in 0..draw_count {
                let indirect_offset = offset
                    + i as wgt::BufferAddress
                        * size_of::<wgt::DrawIndirectArgs>() as wgt::BufferAddress;
                unsafe {
                    self.device.raw.cmd_draw_indirect(
                        self.active,
                        buffer.raw,
                        indirect_offset,
                        1,
                        size_of::<wgt::DrawIndirectArgs>() as u32,
                    )
                };
            }
        }
    }
    unsafe fn draw_indexed_indirect(
        &mut self,
        buffer: &super::Buffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        if draw_count >= 1
            && self.device.private_caps.multi_draw_indirect
            && draw_count <= self.device.private_caps.max_draw_indirect_count
        {
            unsafe {
                self.device.raw.cmd_draw_indexed_indirect(
                    self.active,
                    buffer.raw,
                    offset,
                    draw_count,
                    size_of::<wgt::DrawIndexedIndirectArgs>() as u32,
                )
            };
        } else {
            for i in 0..draw_count {
                let indirect_offset = offset
                    + i as wgt::BufferAddress
                        * size_of::<wgt::DrawIndexedIndirectArgs>() as wgt::BufferAddress;
                unsafe {
                    self.device.raw.cmd_draw_indexed_indirect(
                        self.active,
                        buffer.raw,
                        indirect_offset,
                        1,
                        size_of::<wgt::DrawIndexedIndirectArgs>() as u32,
                    )
                };
            }
        }
    }
    unsafe fn draw_mesh_tasks_indirect(
        &mut self,
        buffer: &<Self::A as crate::Api>::Buffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        if let Some(ref t) = self.device.extension_fns.mesh_shading {
            unsafe {
                t.cmd_draw_mesh_tasks_indirect(
                    self.active,
                    buffer.raw,
                    offset,
                    draw_count,
                    size_of::<wgt::DispatchIndirectArgs>() as u32,
                );
            };
        } else {
            panic!("Feature `MESH_SHADING` not enabled");
        }
    }
    unsafe fn draw_indirect_count(
        &mut self,
        buffer: &super::Buffer,
        offset: wgt::BufferAddress,
        count_buffer: &super::Buffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        let stride = size_of::<wgt::DrawIndirectArgs>() as u32;
        match self.device.extension_fns.draw_indirect_count {
            Some(ref t) => {
                unsafe {
                    t.cmd_draw_indirect_count(
                        self.active,
                        buffer.raw,
                        offset,
                        count_buffer.raw,
                        count_offset,
                        max_count,
                        stride,
                    )
                };
            }
            None => panic!("Feature `DRAW_INDIRECT_COUNT` not enabled"),
        }
    }
    unsafe fn draw_indexed_indirect_count(
        &mut self,
        buffer: &super::Buffer,
        offset: wgt::BufferAddress,
        count_buffer: &super::Buffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        let stride = size_of::<wgt::DrawIndexedIndirectArgs>() as u32;
        match self.device.extension_fns.draw_indirect_count {
            Some(ref t) => {
                unsafe {
                    t.cmd_draw_indexed_indirect_count(
                        self.active,
                        buffer.raw,
                        offset,
                        count_buffer.raw,
                        count_offset,
                        max_count,
                        stride,
                    )
                };
            }
            None => panic!("Feature `DRAW_INDIRECT_COUNT` not enabled"),
        }
    }
    unsafe fn draw_mesh_tasks_indirect_count(
        &mut self,
        buffer: &<Self::A as crate::Api>::Buffer,
        offset: wgt::BufferAddress,
        count_buffer: &super::Buffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        if self.device.extension_fns.draw_indirect_count.is_none() {
            panic!("Feature `DRAW_INDIRECT_COUNT` not enabled");
        }
        if let Some(ref t) = self.device.extension_fns.mesh_shading {
            unsafe {
                t.cmd_draw_mesh_tasks_indirect_count(
                    self.active,
                    buffer.raw,
                    offset,
                    count_buffer.raw,
                    count_offset,
                    max_count,
                    size_of::<wgt::DispatchIndirectArgs>() as u32,
                );
            };
        } else {
            panic!("Feature `MESH_SHADING` not enabled");
        }
    }

    // compute

    unsafe fn begin_compute_pass(
        &mut self,
        desc: &crate::ComputePassDescriptor<'_, super::QuerySet>,
    ) {
        self.bind_point = vk::PipelineBindPoint::COMPUTE;
        if let Some(label) = desc.label {
            unsafe { self.begin_debug_marker(label) };
            self.rpass_debug_marker_active = true;
        }

        if let Some(timestamp_writes) = desc.timestamp_writes.as_ref() {
            if let Some(index) = timestamp_writes.beginning_of_pass_write_index {
                unsafe {
                    self.write_timestamp(timestamp_writes.query_set, index);
                }
            }
            self.end_of_pass_timer_query = timestamp_writes
                .end_of_pass_write_index
                .map(|index| (timestamp_writes.query_set.raw, index));
        }
    }
    unsafe fn end_compute_pass(&mut self) {
        self.write_pass_end_timestamp_if_requested();

        if self.rpass_debug_marker_active {
            unsafe { self.end_debug_marker() };
            self.rpass_debug_marker_active = false
        }
    }

    unsafe fn set_compute_pipeline(&mut self, pipeline: &super::ComputePipeline) {
        unsafe {
            self.device.raw.cmd_bind_pipeline(
                self.active,
                vk::PipelineBindPoint::COMPUTE,
                pipeline.raw,
            )
        };
    }

    unsafe fn dispatch(&mut self, count: [u32; 3]) {
        unsafe {
            self.device
                .raw
                .cmd_dispatch(self.active, count[0], count[1], count[2])
        };
    }
    unsafe fn dispatch_indirect(&mut self, buffer: &super::Buffer, offset: wgt::BufferAddress) {
        unsafe {
            self.device
                .raw
                .cmd_dispatch_indirect(self.active, buffer.raw, offset)
        }
    }

    unsafe fn copy_acceleration_structure_to_acceleration_structure(
        &mut self,
        src: &super::AccelerationStructure,
        dst: &super::AccelerationStructure,
        copy: wgt::AccelerationStructureCopy,
    ) {
        let ray_tracing_functions = self
            .device
            .extension_fns
            .ray_tracing
            .as_ref()
            .expect("Feature `RAY_TRACING` not enabled");

        let mode = match copy {
            wgt::AccelerationStructureCopy::Clone => vk::CopyAccelerationStructureModeKHR::CLONE,
            wgt::AccelerationStructureCopy::Compact => {
                vk::CopyAccelerationStructureModeKHR::COMPACT
            }
        };

        unsafe {
            ray_tracing_functions
                .acceleration_structure
                .cmd_copy_acceleration_structure(
                    self.active,
                    &vk::CopyAccelerationStructureInfoKHR {
                        s_type: vk::StructureType::COPY_ACCELERATION_STRUCTURE_INFO_KHR,
                        p_next: core::ptr::null(),
                        src: src.raw,
                        dst: dst.raw,
                        mode,
                        _marker: Default::default(),
                    },
                );
        }
    }
}

#[test]
fn check_dst_image_layout() {
    assert_eq!(
        conv::derive_image_layout(wgt::TextureUses::COPY_DST, wgt::TextureFormat::Rgba8Unorm),
        DST_IMAGE_LAYOUT
    );
}
