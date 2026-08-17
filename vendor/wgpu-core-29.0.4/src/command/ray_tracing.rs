use alloc::{sync::Arc, vec::Vec};
use core::{
    cmp::max,
    num::NonZeroU64,
    ops::{Deref, Range},
};

use wgt::{math::align_to, BufferUsages, BufferUses, Features};

use crate::{
    command::encoder::EncodingState,
    ray_tracing::{AsAction, AsBuild, TlasBuild, ValidateAsActionsError},
    resource::InvalidResourceError,
};
use crate::{command::EncoderStateError, device::resource::CommandIndices};
use crate::{
    command::{ArcCommand, ArcReferences, CommandBufferMutable},
    device::queue::TempResource,
    global::Global,
    id::CommandEncoderId,
    init_tracker::MemoryInitKind,
    ray_tracing::{
        ArcBlasBuildEntry, ArcBlasGeometries, ArcBlasTriangleGeometry, ArcTlasInstance,
        ArcTlasPackage, BlasBuildEntry, BlasGeometries, BuildAccelerationStructureError,
        OwnedBlasBuildEntry, OwnedTlasPackage, TlasPackage,
    },
    resource::{Blas, BlasCompactState, Labeled, StagingBuffer, Tlas},
    scratch::ScratchBuffer,
    snatch::SnatchGuard,
};
use crate::{lock::RwLockWriteGuard, resource::RawResourceAccess};

use crate::id::{BlasId, TlasId};

struct BlasStore<'a> {
    blas: Arc<Blas>,
    entries: hal::AccelerationStructureEntries<'a, dyn hal::DynBuffer>,
    scratch_buffer_offset: u64,
}

struct UnsafeTlasStore<'a> {
    tlas: Arc<Tlas>,
    entries: hal::AccelerationStructureEntries<'a, dyn hal::DynBuffer>,
    scratch_buffer_offset: u64,
}

struct TlasStore<'a> {
    internal: UnsafeTlasStore<'a>,
    range: Range<usize>,
}

impl Global {
    fn resolve_blas_id(&self, blas_id: BlasId) -> Result<Arc<Blas>, InvalidResourceError> {
        self.hub.blas_s.get(blas_id).get()
    }

    fn resolve_tlas_id(&self, tlas_id: TlasId) -> Result<Arc<Tlas>, InvalidResourceError> {
        self.hub.tlas_s.get(tlas_id).get()
    }

    pub fn command_encoder_mark_acceleration_structures_built(
        &self,
        command_encoder_id: CommandEncoderId,
        blas_ids: &[BlasId],
        tlas_ids: &[TlasId],
    ) -> Result<(), EncoderStateError> {
        profiling::scope!("CommandEncoder::mark_acceleration_structures_built");

        let hub = &self.hub;

        let cmd_enc = hub.command_encoders.get(command_encoder_id);

        let mut cmd_buf_data = cmd_enc.data.lock();
        cmd_buf_data.with_buffer(
            crate::command::EncodingApi::Raw,
            |cmd_buf_data| -> Result<(), BuildAccelerationStructureError> {
                let device = &cmd_enc.device;
                device.check_is_valid()?;
                device.require_features(Features::EXPERIMENTAL_RAY_QUERY)?;

                let mut build_command = AsBuild::with_capacity(blas_ids.len(), tlas_ids.len());

                for blas in blas_ids {
                    let blas = hub.blas_s.get(*blas).get()?;
                    build_command.blas_s_built.push(blas);
                }

                for tlas in tlas_ids {
                    let tlas = hub.tlas_s.get(*tlas).get()?;
                    build_command.tlas_s_built.push(TlasBuild {
                        tlas,
                        dependencies: Vec::new(),
                    });
                }

                cmd_buf_data.as_actions.push(AsAction::Build(build_command));
                Ok(())
            },
        )
    }

    pub fn command_encoder_build_acceleration_structures<'a>(
        &self,
        command_encoder_id: CommandEncoderId,
        blas_iter: impl Iterator<Item = BlasBuildEntry<'a>>,
        tlas_iter: impl Iterator<Item = TlasPackage<'a>>,
    ) -> Result<(), EncoderStateError> {
        profiling::scope!("CommandEncoder::build_acceleration_structures");

        let hub = &self.hub;

        let cmd_enc = hub.command_encoders.get(command_encoder_id);
        let mut cmd_buf_data = cmd_enc.data.lock();

        cmd_buf_data.push_with(|| -> Result<_, BuildAccelerationStructureError> {
            let blas = blas_iter
                .map(|blas_entry| {
                    let geometries = match blas_entry.geometries {
                        BlasGeometries::TriangleGeometries(triangle_geometries) => {
                            let tri_geo = triangle_geometries
                                .map(|tg| {
                                    Ok(ArcBlasTriangleGeometry {
                                        size: tg.size.clone(),
                                        vertex_buffer: self.resolve_buffer_id(tg.vertex_buffer)?,
                                        index_buffer: tg
                                            .index_buffer
                                            .map(|id| self.resolve_buffer_id(id))
                                            .transpose()?,
                                        transform_buffer: tg
                                            .transform_buffer
                                            .map(|id| self.resolve_buffer_id(id))
                                            .transpose()?,
                                        first_vertex: tg.first_vertex,
                                        vertex_stride: tg.vertex_stride,
                                        first_index: tg.first_index,
                                        transform_buffer_offset: tg.transform_buffer_offset,
                                    })
                                })
                                .collect::<Result<_, BuildAccelerationStructureError>>()?;
                            ArcBlasGeometries::TriangleGeometries(tri_geo)
                        }
                    };
                    Ok(ArcBlasBuildEntry {
                        blas: self.resolve_blas_id(blas_entry.blas_id)?,
                        geometries,
                    })
                })
                .collect::<Result<_, BuildAccelerationStructureError>>()?;

            let tlas = tlas_iter
                .map(|tlas_package| {
                    let instances = tlas_package
                        .instances
                        .map(|instance| {
                            instance
                                .as_ref()
                                .map(|instance| {
                                    Ok(ArcTlasInstance {
                                        blas: self.resolve_blas_id(instance.blas_id)?,
                                        transform: *instance.transform,
                                        custom_data: instance.custom_data,
                                        mask: instance.mask,
                                    })
                                })
                                .transpose()
                        })
                        .collect::<Result<_, BuildAccelerationStructureError>>()?;
                    Ok(ArcTlasPackage {
                        tlas: self.resolve_tlas_id(tlas_package.tlas_id)?,
                        instances,
                        lowest_unmodified: tlas_package.lowest_unmodified,
                    })
                })
                .collect::<Result<_, BuildAccelerationStructureError>>()?;

            Ok(ArcCommand::BuildAccelerationStructures { blas, tlas })
        })
    }
}

pub(crate) fn build_acceleration_structures(
    state: &mut EncodingState,
    blas: Vec<OwnedBlasBuildEntry<ArcReferences>>,
    tlas: Vec<OwnedTlasPackage<ArcReferences>>,
) -> Result<(), BuildAccelerationStructureError> {
    state
        .device
        .require_features(Features::EXPERIMENTAL_RAY_QUERY)?;

    let mut build_command = AsBuild::with_capacity(blas.len(), tlas.len());
    let mut input_barriers = Vec::<hal::BufferBarrier<dyn hal::DynBuffer>>::new();
    let mut scratch_buffer_blas_size = 0;
    let mut blas_storage = Vec::with_capacity(blas.len());
    iter_blas(
        blas.iter(),
        &mut build_command,
        &mut input_barriers,
        &mut scratch_buffer_blas_size,
        &mut blas_storage,
        state,
    )?;

    let mut scratch_buffer_tlas_size = 0;
    let mut tlas_storage = Vec::<TlasStore>::with_capacity(tlas.len());
    let mut instance_buffer_staging_source = Vec::<u8>::new();

    for package in tlas.iter() {
        let tlas = &package.tlas;
        state.tracker.tlas_s.insert_single(tlas.clone());

        let scratch_buffer_offset = scratch_buffer_tlas_size;
        scratch_buffer_tlas_size += align_to(
            tlas.size_info.build_scratch_size as u32,
            state.device.alignments.ray_tracing_scratch_buffer_alignment,
        ) as u64;

        let first_byte_index = instance_buffer_staging_source.len();

        let mut dependencies = Vec::new();

        let mut instance_count = 0;
        for instance in package.instances.iter().flatten() {
            if instance.custom_data >= (1u32 << 24u32) {
                return Err(BuildAccelerationStructureError::TlasInvalidCustomIndex(
                    tlas.error_ident(),
                ));
            }
            let blas = &instance.blas;
            state.tracker.blas_s.insert_single(blas.clone());

            instance_buffer_staging_source.extend(state.device.raw().tlas_instance_to_bytes(
                hal::TlasInstance {
                    transform: instance.transform,
                    custom_data: instance.custom_data,
                    mask: instance.mask,
                    blas_address: blas.handle,
                },
            ));

            if tlas
                .flags
                .contains(wgpu_types::AccelerationStructureFlags::ALLOW_RAY_HIT_VERTEX_RETURN)
                && !blas
                    .flags
                    .contains(wgpu_types::AccelerationStructureFlags::ALLOW_RAY_HIT_VERTEX_RETURN)
            {
                return Err(
                    BuildAccelerationStructureError::TlasDependentMissingVertexReturn(
                        tlas.error_ident(),
                        blas.error_ident(),
                    ),
                );
            }

            instance_count += 1;

            dependencies.push(blas.clone());
        }

        build_command.tlas_s_built.push(TlasBuild {
            tlas: tlas.clone(),
            dependencies,
        });

        if instance_count > tlas.max_instance_count {
            return Err(BuildAccelerationStructureError::TlasInstanceCountExceeded(
                tlas.error_ident(),
                instance_count,
                tlas.max_instance_count,
            ));
        }

        tlas_storage.push(TlasStore {
            internal: UnsafeTlasStore {
                tlas: tlas.clone(),
                entries: hal::AccelerationStructureEntries::Instances(
                    hal::AccelerationStructureInstances {
                        buffer: Some(tlas.instance_buffer.as_ref()),
                        offset: 0,
                        count: instance_count,
                    },
                ),
                scratch_buffer_offset,
            },
            range: first_byte_index..instance_buffer_staging_source.len(),
        });
    }

    let Some(scratch_size) =
        wgt::BufferSize::new(max(scratch_buffer_blas_size, scratch_buffer_tlas_size))
    else {
        // if the size is zero there is nothing to build
        return Ok(());
    };

    let scratch_buffer = ScratchBuffer::new(state.device, scratch_size)?;

    let scratch_buffer_barrier = hal::BufferBarrier::<dyn hal::DynBuffer> {
        buffer: scratch_buffer.raw(),
        usage: hal::StateTransition {
            from: BufferUses::ACCELERATION_STRUCTURE_SCRATCH,
            to: BufferUses::ACCELERATION_STRUCTURE_SCRATCH,
        },
    };

    let mut tlas_descriptors = Vec::with_capacity(tlas_storage.len());

    for &TlasStore {
        internal:
            UnsafeTlasStore {
                ref tlas,
                ref entries,
                ref scratch_buffer_offset,
            },
        ..
    } in &tlas_storage
    {
        if tlas.update_mode == wgt::AccelerationStructureUpdateMode::PreferUpdate {
            log::warn!("build_acceleration_structures called with PreferUpdate, but only rebuild is implemented");
        }
        tlas_descriptors.push(hal::BuildAccelerationStructureDescriptor {
            entries,
            mode: hal::AccelerationStructureBuildMode::Build,
            flags: tlas.flags,
            source_acceleration_structure: None,
            destination_acceleration_structure: tlas.try_raw(state.snatch_guard)?,
            scratch_buffer: scratch_buffer.raw(),
            scratch_buffer_offset: *scratch_buffer_offset,
        })
    }

    let blas_present = !blas_storage.is_empty();
    let tlas_present = !tlas_storage.is_empty();

    let raw_encoder = &mut state.raw_encoder;

    let mut blas_s_compactable = Vec::new();
    let mut descriptors = Vec::with_capacity(blas.len());

    for storage in &blas_storage {
        descriptors.push(map_blas(
            storage,
            scratch_buffer.raw(),
            state.snatch_guard,
            &mut blas_s_compactable,
        )?);
    }

    build_blas(
        *raw_encoder,
        blas_present,
        tlas_present,
        input_barriers,
        &descriptors,
        scratch_buffer_barrier,
        blas_s_compactable,
    );

    if tlas_present {
        let staging_buffer = if !instance_buffer_staging_source.is_empty() {
            let mut staging_buffer = StagingBuffer::new(
                state.device,
                wgt::BufferSize::new(instance_buffer_staging_source.len() as u64).unwrap(),
            )?;
            staging_buffer.write(&instance_buffer_staging_source);
            let flushed = staging_buffer.flush();
            Some(flushed)
        } else {
            None
        };

        unsafe {
            if let Some(ref staging_buffer) = staging_buffer {
                raw_encoder.transition_buffers(&[hal::BufferBarrier::<dyn hal::DynBuffer> {
                    buffer: staging_buffer.raw(),
                    usage: hal::StateTransition {
                        from: BufferUses::MAP_WRITE,
                        to: BufferUses::COPY_SRC,
                    },
                }]);
            }
        }

        let mut instance_buffer_barriers = Vec::new();
        for &TlasStore {
            internal: UnsafeTlasStore { ref tlas, .. },
            ref range,
        } in &tlas_storage
        {
            let size = match wgt::BufferSize::new((range.end - range.start) as u64) {
                None => continue,
                Some(size) => size,
            };
            instance_buffer_barriers.push(hal::BufferBarrier::<dyn hal::DynBuffer> {
                buffer: tlas.instance_buffer.as_ref(),
                usage: hal::StateTransition {
                    from: BufferUses::COPY_DST,
                    to: BufferUses::TOP_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                },
            });
            unsafe {
                raw_encoder.transition_buffers(&[hal::BufferBarrier::<dyn hal::DynBuffer> {
                    buffer: tlas.instance_buffer.as_ref(),
                    usage: hal::StateTransition {
                        from: BufferUses::TOP_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                        to: BufferUses::COPY_DST,
                    },
                }]);
                let temp = hal::BufferCopy {
                    src_offset: range.start as u64,
                    dst_offset: 0,
                    size,
                };
                raw_encoder.copy_buffer_to_buffer(
                    staging_buffer.as_ref().unwrap().raw(),
                    tlas.instance_buffer.as_ref(),
                    &[temp],
                );
            }
        }

        unsafe {
            raw_encoder.transition_buffers(&instance_buffer_barriers);

            raw_encoder.build_acceleration_structures(&tlas_descriptors);

            raw_encoder.place_acceleration_structure_barrier(hal::AccelerationStructureBarrier {
                usage: hal::StateTransition {
                    from: hal::AccelerationStructureUses::BUILD_OUTPUT,
                    to: hal::AccelerationStructureUses::SHADER_INPUT,
                },
            });
        }

        if let Some(staging_buffer) = staging_buffer {
            state
                .temp_resources
                .push(TempResource::StagingBuffer(staging_buffer));
        }
    }

    state
        .temp_resources
        .push(TempResource::ScratchBuffer(scratch_buffer));

    state.as_actions.push(AsAction::Build(build_command));

    Ok(())
}

impl CommandBufferMutable {
    pub(crate) fn validate_acceleration_structure_actions(
        &self,
        snatch_guard: &SnatchGuard,
        command_index_guard: &mut RwLockWriteGuard<CommandIndices>,
    ) -> Result<(), ValidateAsActionsError> {
        profiling::scope!("CommandEncoder::[submission]::validate_as_actions");
        for action in &self.as_actions {
            match action {
                AsAction::Build(build) => {
                    let build_command_index = NonZeroU64::new(
                        command_index_guard.next_acceleration_structure_build_command_index,
                    )
                    .unwrap();

                    command_index_guard.next_acceleration_structure_build_command_index += 1;
                    for blas in build.blas_s_built.iter() {
                        let mut state_lock = blas.compacted_state.lock();
                        *state_lock = match *state_lock {
                            BlasCompactState::Compacted => {
                                unreachable!("Should be validated out in build.")
                            }
                            // Reset the compacted state to idle. This means any prepares, before mapping their
                            // internal buffer, will terminate.
                            _ => BlasCompactState::Idle,
                        };
                        *blas.built_index.write() = Some(build_command_index);
                    }

                    for tlas_build in build.tlas_s_built.iter() {
                        for blas in &tlas_build.dependencies {
                            if blas.built_index.read().is_none() {
                                return Err(ValidateAsActionsError::UsedUnbuiltBlas(
                                    blas.error_ident(),
                                    tlas_build.tlas.error_ident(),
                                ));
                            }
                        }
                        *tlas_build.tlas.built_index.write() = Some(build_command_index);
                        tlas_build
                            .tlas
                            .dependencies
                            .write()
                            .clone_from(&tlas_build.dependencies)
                    }
                }
                AsAction::UseTlas(tlas) => {
                    let tlas_build_index = tlas.built_index.read();
                    let dependencies = tlas.dependencies.read();

                    if (*tlas_build_index).is_none() {
                        return Err(ValidateAsActionsError::UsedUnbuiltTlas(tlas.error_ident()));
                    }
                    for blas in dependencies.deref() {
                        let blas_build_index = *blas.built_index.read();
                        if blas_build_index.is_none() {
                            return Err(ValidateAsActionsError::UsedUnbuiltBlas(
                                tlas.error_ident(),
                                blas.error_ident(),
                            ));
                        }
                        if blas_build_index.unwrap() > tlas_build_index.unwrap() {
                            return Err(ValidateAsActionsError::BlasNewerThenTlas(
                                blas.error_ident(),
                                tlas.error_ident(),
                            ));
                        }
                        blas.try_raw(snatch_guard)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn set_acceleration_structure_dependencies(&self, snatch_guard: &SnatchGuard) {
        profiling::scope!("CommandEncoder::[submission]::set_acceleration_structure_dependencies");
        let tlas_dependencies_locks: Vec<_> = self
            .as_actions
            .iter()
            .filter_map(|action| {
                if let AsAction::UseTlas(tlas) = action {
                    Some(tlas.dependencies.read())
                } else {
                    None
                }
            })
            .collect();
        let mut tlas_dependencies_lock_iter = tlas_dependencies_locks.iter();
        let mut dependencies = Vec::new();
        for action in &self.as_actions {
            match action {
                AsAction::Build(build) => {
                    for tlas_build in build.tlas_s_built.iter() {
                        for dependency in &tlas_build.dependencies {
                            if let Some(dependency) = dependency.raw(snatch_guard) {
                                dependencies.push(dependency);
                            }
                        }
                    }
                }
                AsAction::UseTlas(_tlas) => {
                    let tlas_dependencies = tlas_dependencies_lock_iter.next().unwrap(); // _tlas.dependencies.read();
                    for dependency in tlas_dependencies.iter() {
                        if let Some(dependency) = dependency.raw(snatch_guard) {
                            dependencies.push(dependency);
                        }
                    }
                }
            }
        }
        if !dependencies.is_empty() {
            unsafe {
                self.encoder
                    .raw
                    .set_acceleration_structure_dependencies(&self.encoder.list, &dependencies);
            }
        }
    }
}

///iterates over the blas iterator, and it's geometry, pushing the buffers into a storage vector (and also some validation).
fn iter_blas<'snatch_guard: 'buffers, 'buffers>(
    blas_iter: impl Iterator<Item = &'buffers OwnedBlasBuildEntry<ArcReferences>>,
    build_command: &mut AsBuild,
    input_barriers: &mut Vec<hal::BufferBarrier<'buffers, dyn hal::DynBuffer>>,
    scratch_buffer_blas_size: &mut u64,
    blas_storage: &mut Vec<BlasStore<'buffers>>,
    state: &mut EncodingState<'snatch_guard, '_>,
) -> Result<(), BuildAccelerationStructureError> {
    for entry in blas_iter {
        let blas = &entry.blas;
        state.tracker.blas_s.insert_single(blas.clone());

        build_command.blas_s_built.push(blas.clone());

        match &entry.geometries {
            ArcBlasGeometries::TriangleGeometries(triangle_geometries) => {
                let mut triangle_entries =
                    Vec::<hal::AccelerationStructureTriangles<dyn hal::DynBuffer>>::new();

                for (i, mesh) in triangle_geometries.iter().enumerate() {
                    let size_desc = match &blas.sizes {
                        wgt::BlasGeometrySizeDescriptors::Triangles { descriptors } => descriptors,
                    };
                    if i >= size_desc.len() {
                        return Err(BuildAccelerationStructureError::IncompatibleBlasBuildSizes(
                            blas.error_ident(),
                        ));
                    }
                    let size_desc = &size_desc[i];

                    if size_desc.flags != mesh.size.flags {
                        return Err(BuildAccelerationStructureError::IncompatibleBlasFlags(
                            blas.error_ident(),
                            size_desc.flags,
                            mesh.size.flags,
                        ));
                    }

                    if size_desc.vertex_count < mesh.size.vertex_count {
                        return Err(
                            BuildAccelerationStructureError::IncompatibleBlasVertexCount(
                                blas.error_ident(),
                                size_desc.vertex_count,
                                mesh.size.vertex_count,
                            ),
                        );
                    }

                    if size_desc.vertex_format != mesh.size.vertex_format {
                        return Err(BuildAccelerationStructureError::DifferentBlasVertexFormats(
                            blas.error_ident(),
                            size_desc.vertex_format,
                            mesh.size.vertex_format,
                        ));
                    }

                    if size_desc
                        .vertex_format
                        .min_acceleration_structure_vertex_stride()
                        > mesh.vertex_stride
                    {
                        return Err(BuildAccelerationStructureError::VertexStrideTooSmall(
                            blas.error_ident(),
                            size_desc
                                .vertex_format
                                .min_acceleration_structure_vertex_stride(),
                            mesh.vertex_stride,
                        ));
                    }

                    if mesh.vertex_stride
                        % size_desc
                            .vertex_format
                            .acceleration_structure_stride_alignment()
                        != 0
                    {
                        return Err(BuildAccelerationStructureError::VertexStrideUnaligned(
                            blas.error_ident(),
                            size_desc
                                .vertex_format
                                .acceleration_structure_stride_alignment(),
                            mesh.vertex_stride,
                        ));
                    }

                    match (size_desc.index_count, mesh.size.index_count) {
                        (Some(_), None) | (None, Some(_)) => {
                            return Err(
                                BuildAccelerationStructureError::BlasIndexCountProvidedMismatch(
                                    blas.error_ident(),
                                ),
                            )
                        }
                        (Some(create), Some(build)) if create < build => {
                            return Err(
                                BuildAccelerationStructureError::IncompatibleBlasIndexCount(
                                    blas.error_ident(),
                                    create,
                                    build,
                                ),
                            )
                        }
                        _ => {}
                    }

                    if size_desc.index_format != mesh.size.index_format {
                        return Err(BuildAccelerationStructureError::DifferentBlasIndexFormats(
                            blas.error_ident(),
                            size_desc.index_format,
                            mesh.size.index_format,
                        ));
                    }

                    if size_desc.index_count.is_some() && mesh.index_buffer.is_none() {
                        return Err(BuildAccelerationStructureError::MissingIndexBuffer(
                            blas.error_ident(),
                        ));
                    }
                    let vertex_buffer = mesh.vertex_buffer.clone();
                    let vertex_pending = state.tracker.buffers.set_single(
                        &vertex_buffer,
                        BufferUses::BOTTOM_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                    );
                    let vertex_buffer = {
                        let vertex_raw = mesh.vertex_buffer.as_ref().try_raw(state.snatch_guard)?;
                        let vertex_buffer = &mesh.vertex_buffer;
                        vertex_buffer.check_usage(BufferUsages::BLAS_INPUT)?;

                        if let Some(barrier) = vertex_pending.map(|pending| {
                            pending.into_hal(vertex_buffer.as_ref(), state.snatch_guard)
                        }) {
                            input_barriers.push(barrier);
                        }
                        if vertex_buffer.size
                            < (mesh.size.vertex_count + mesh.first_vertex) as u64
                                * mesh.vertex_stride
                        {
                            return Err(BuildAccelerationStructureError::InsufficientBufferSize(
                                vertex_buffer.error_ident(),
                                vertex_buffer.size,
                                (mesh.size.vertex_count + mesh.first_vertex) as u64
                                    * mesh.vertex_stride,
                            ));
                        }
                        let vertex_buffer_offset = mesh.first_vertex as u64 * mesh.vertex_stride;
                        state.buffer_memory_init_actions.extend(
                            vertex_buffer.initialization_status.read().create_action(
                                vertex_buffer,
                                vertex_buffer_offset
                                    ..(vertex_buffer_offset
                                        + mesh.size.vertex_count as u64 * mesh.vertex_stride),
                                MemoryInitKind::NeedsInitializedMemory,
                            ),
                        );
                        vertex_raw
                    };
                    let index_buffer = if let Some(ref index_buffer) = mesh.index_buffer {
                        if mesh.first_index.is_none()
                            || mesh.size.index_count.is_none()
                            || mesh.size.index_count.is_none()
                        {
                            return Err(BuildAccelerationStructureError::MissingAssociatedData(
                                index_buffer.error_ident(),
                            ));
                        }
                        let index_pending = state.tracker.buffers.set_single(
                            index_buffer,
                            BufferUses::BOTTOM_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                        );
                        let index_raw = index_buffer.try_raw(state.snatch_guard)?;
                        index_buffer.check_usage(BufferUsages::BLAS_INPUT)?;

                        if let Some(barrier) = index_pending.map(|pending| {
                            pending.into_hal(index_buffer.as_ref(), state.snatch_guard)
                        }) {
                            input_barriers.push(barrier);
                        }
                        let index_stride = mesh.size.index_format.unwrap().byte_size() as u64;
                        let offset = mesh.first_index.unwrap() as u64 * index_stride;
                        let index_buffer_size =
                            mesh.size.index_count.unwrap() as u64 * index_stride;

                        if mesh.size.index_count.unwrap() % 3 != 0 {
                            return Err(BuildAccelerationStructureError::InvalidIndexCount(
                                index_buffer.error_ident(),
                                mesh.size.index_count.unwrap(),
                            ));
                        }
                        if index_buffer.size
                            < mesh.size.index_count.unwrap() as u64 * index_stride + offset
                        {
                            return Err(BuildAccelerationStructureError::InsufficientBufferSize(
                                index_buffer.error_ident(),
                                index_buffer.size,
                                mesh.size.index_count.unwrap() as u64 * index_stride + offset,
                            ));
                        }

                        state.buffer_memory_init_actions.extend(
                            index_buffer.initialization_status.read().create_action(
                                index_buffer,
                                offset..(offset + index_buffer_size),
                                MemoryInitKind::NeedsInitializedMemory,
                            ),
                        );
                        Some(index_raw)
                    } else {
                        None
                    };
                    let transform_buffer = if let Some(ref transform_buffer) = mesh.transform_buffer
                    {
                        if !blas
                            .flags
                            .contains(wgt::AccelerationStructureFlags::USE_TRANSFORM)
                        {
                            return Err(BuildAccelerationStructureError::UseTransformMissing(
                                blas.error_ident(),
                            ));
                        }
                        if mesh.transform_buffer_offset.is_none() {
                            return Err(BuildAccelerationStructureError::MissingAssociatedData(
                                transform_buffer.error_ident(),
                            ));
                        }
                        let transform_pending = state.tracker.buffers.set_single(
                            transform_buffer,
                            BufferUses::BOTTOM_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                        );
                        if mesh.transform_buffer_offset.is_none() {
                            return Err(BuildAccelerationStructureError::MissingAssociatedData(
                                transform_buffer.error_ident(),
                            ));
                        }
                        let transform_raw = transform_buffer.try_raw(state.snatch_guard)?;
                        transform_buffer.check_usage(BufferUsages::BLAS_INPUT)?;

                        if let Some(barrier) = transform_pending.map(|pending| {
                            pending.into_hal(transform_buffer.as_ref(), state.snatch_guard)
                        }) {
                            input_barriers.push(barrier);
                        }

                        let offset = mesh.transform_buffer_offset.unwrap();

                        if offset % wgt::TRANSFORM_BUFFER_ALIGNMENT != 0 {
                            return Err(
                                BuildAccelerationStructureError::UnalignedTransformBufferOffset(
                                    transform_buffer.error_ident(),
                                ),
                            );
                        }
                        if transform_buffer.size < 48 + offset {
                            return Err(BuildAccelerationStructureError::InsufficientBufferSize(
                                transform_buffer.error_ident(),
                                transform_buffer.size,
                                48 + offset,
                            ));
                        }
                        state.buffer_memory_init_actions.extend(
                            transform_buffer.initialization_status.read().create_action(
                                transform_buffer,
                                offset..(offset + 48),
                                MemoryInitKind::NeedsInitializedMemory,
                            ),
                        );
                        Some(transform_raw)
                    } else {
                        if blas
                            .flags
                            .contains(wgt::AccelerationStructureFlags::USE_TRANSFORM)
                        {
                            return Err(BuildAccelerationStructureError::TransformMissing(
                                blas.error_ident(),
                            ));
                        }
                        None
                    };

                    let triangles = hal::AccelerationStructureTriangles {
                        vertex_buffer: Some(vertex_buffer),
                        vertex_format: mesh.size.vertex_format,
                        first_vertex: mesh.first_vertex,
                        vertex_count: mesh.size.vertex_count,
                        vertex_stride: mesh.vertex_stride,
                        indices: index_buffer.map(|index_buffer| {
                            let index_stride = mesh.size.index_format.unwrap().byte_size() as u32;
                            hal::AccelerationStructureTriangleIndices::<dyn hal::DynBuffer> {
                                format: mesh.size.index_format.unwrap(),
                                buffer: Some(index_buffer),
                                offset: mesh.first_index.unwrap() * index_stride,
                                count: mesh.size.index_count.unwrap(),
                            }
                        }),
                        transform: transform_buffer.map(|transform_buffer| {
                            hal::AccelerationStructureTriangleTransform {
                                buffer: transform_buffer,
                                offset: mesh.transform_buffer_offset.unwrap() as u32,
                            }
                        }),
                        flags: mesh.size.flags,
                    };
                    triangle_entries.push(triangles);
                }

                {
                    let scratch_buffer_offset = *scratch_buffer_blas_size;
                    *scratch_buffer_blas_size += align_to(
                        blas.size_info.build_scratch_size as u32,
                        state.device.alignments.ray_tracing_scratch_buffer_alignment,
                    ) as u64;

                    blas_storage.push(BlasStore {
                        blas: blas.clone(),
                        entries: hal::AccelerationStructureEntries::Triangles(triangle_entries),
                        scratch_buffer_offset,
                    });
                }
            }
        }
    }
    Ok(())
}

fn map_blas<'a>(
    storage: &'a BlasStore<'_>,
    scratch_buffer: &'a dyn hal::DynBuffer,
    snatch_guard: &'a SnatchGuard,
    blases_compactable: &mut Vec<(
        &'a dyn hal::DynBuffer,
        &'a dyn hal::DynAccelerationStructure,
    )>,
) -> Result<
    hal::BuildAccelerationStructureDescriptor<
        'a,
        dyn hal::DynBuffer,
        dyn hal::DynAccelerationStructure,
    >,
    BuildAccelerationStructureError,
> {
    let BlasStore {
        blas,
        entries,
        scratch_buffer_offset,
    } = storage;
    if blas.update_mode == wgt::AccelerationStructureUpdateMode::PreferUpdate {
        log::debug!("only rebuild implemented")
    }
    let raw = blas.try_raw(snatch_guard)?;

    let state_lock = blas.compacted_state.lock();
    if let BlasCompactState::Compacted = *state_lock {
        return Err(BuildAccelerationStructureError::CompactedBlas(
            blas.error_ident(),
        ));
    }

    if blas
        .flags
        .contains(wgpu_types::AccelerationStructureFlags::ALLOW_COMPACTION)
    {
        blases_compactable.push((blas.compaction_buffer.as_ref().unwrap().as_ref(), raw));
    }
    Ok(hal::BuildAccelerationStructureDescriptor {
        entries,
        mode: hal::AccelerationStructureBuildMode::Build,
        flags: blas.flags,
        source_acceleration_structure: None,
        destination_acceleration_structure: raw,
        scratch_buffer,
        scratch_buffer_offset: *scratch_buffer_offset,
    })
}

fn build_blas<'a>(
    cmd_buf_raw: &mut dyn hal::DynCommandEncoder,
    blas_present: bool,
    tlas_present: bool,
    input_barriers: Vec<hal::BufferBarrier<dyn hal::DynBuffer>>,
    blas_descriptors: &[hal::BuildAccelerationStructureDescriptor<
        'a,
        dyn hal::DynBuffer,
        dyn hal::DynAccelerationStructure,
    >],
    scratch_buffer_barrier: hal::BufferBarrier<dyn hal::DynBuffer>,
    blas_s_for_compaction: Vec<(
        &'a dyn hal::DynBuffer,
        &'a dyn hal::DynAccelerationStructure,
    )>,
) {
    unsafe {
        cmd_buf_raw.transition_buffers(&input_barriers);
    }

    if blas_present {
        unsafe {
            cmd_buf_raw.place_acceleration_structure_barrier(hal::AccelerationStructureBarrier {
                usage: hal::StateTransition {
                    from: hal::AccelerationStructureUses::BUILD_INPUT,
                    to: hal::AccelerationStructureUses::BUILD_OUTPUT,
                },
            });

            cmd_buf_raw.build_acceleration_structures(blas_descriptors);
        }
    }

    if blas_present && tlas_present {
        unsafe {
            cmd_buf_raw.transition_buffers(&[scratch_buffer_barrier]);
        }
    }

    let mut source_usage = hal::AccelerationStructureUses::empty();
    let mut destination_usage = hal::AccelerationStructureUses::empty();
    for &(buf, blas) in blas_s_for_compaction.iter() {
        unsafe {
            cmd_buf_raw.transition_buffers(&[hal::BufferBarrier {
                buffer: buf,
                usage: hal::StateTransition {
                    from: BufferUses::ACCELERATION_STRUCTURE_QUERY,
                    to: BufferUses::ACCELERATION_STRUCTURE_QUERY,
                },
            }])
        }
        unsafe { cmd_buf_raw.read_acceleration_structure_compact_size(blas, buf) }
        destination_usage |= hal::AccelerationStructureUses::COPY_SRC;
    }

    if blas_present {
        source_usage |= hal::AccelerationStructureUses::BUILD_OUTPUT;
        destination_usage |= hal::AccelerationStructureUses::BUILD_INPUT
    }
    if tlas_present {
        source_usage |= hal::AccelerationStructureUses::SHADER_INPUT;
        destination_usage |= hal::AccelerationStructureUses::BUILD_OUTPUT;
    }
    unsafe {
        cmd_buf_raw.place_acceleration_structure_barrier(hal::AccelerationStructureBarrier {
            usage: hal::StateTransition {
                from: source_usage,
                to: destination_usage,
            },
        });
    }
}
