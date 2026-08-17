use alloc::{string::ToString as _, sync::Arc, vec::Vec};
use core::mem::{size_of, ManuallyDrop};

#[cfg(feature = "trace")]
use crate::device::trace::{Action, IntoTrace};
use crate::device::DeviceError;
use crate::{
    api_log,
    device::Device,
    global::Global,
    hal_label,
    id::{self, BlasId, TlasId},
    lock::RwLock,
    lock::{rank, Mutex},
    ray_tracing::BlasPrepareCompactError,
    ray_tracing::{CreateBlasError, CreateTlasError},
    resource,
    resource::{
        BlasCompactCallback, BlasCompactState, Fallible, InvalidResourceError, TrackingData,
    },
    snatch::Snatchable,
    LabelHelpers,
};
use hal::AccelerationStructureTriangleIndices;
use wgt::Features;

impl Device {
    pub fn create_blas(
        self: &Arc<Self>,
        blas_desc: &resource::BlasDescriptor,
        sizes: wgt::BlasGeometrySizeDescriptors,
    ) -> Result<Arc<resource::Blas>, CreateBlasError> {
        self.check_is_valid()?;
        self.require_features(Features::EXPERIMENTAL_RAY_QUERY)?;

        if blas_desc
            .flags
            .contains(wgt::AccelerationStructureFlags::ALLOW_RAY_HIT_VERTEX_RETURN)
        {
            self.require_features(Features::EXPERIMENTAL_RAY_HIT_VERTEX_RETURN)?;
        }

        let size_info = match &sizes {
            wgt::BlasGeometrySizeDescriptors::Triangles { descriptors } => {
                if descriptors.len() as u32 > self.limits.max_blas_geometry_count {
                    return Err(CreateBlasError::TooManyGeometries(
                        self.limits.max_blas_geometry_count,
                        descriptors.len() as u32,
                    ));
                }

                let mut entries =
                    Vec::<hal::AccelerationStructureTriangles<dyn hal::DynBuffer>>::with_capacity(
                        descriptors.len(),
                    );
                for desc in descriptors {
                    if desc.index_count.is_some() != desc.index_format.is_some() {
                        return Err(CreateBlasError::MissingIndexData);
                    }
                    let indices =
                        desc.index_count
                            .map(|count| AccelerationStructureTriangleIndices::<
                                dyn hal::DynBuffer,
                            > {
                                format: desc.index_format.unwrap(),
                                buffer: Some(self.zero_buffer.as_ref()),
                                offset: 0,
                                count,
                            });
                    if !self
                        .features
                        .allowed_vertex_formats_for_blas()
                        .contains(&desc.vertex_format)
                    {
                        return Err(CreateBlasError::InvalidVertexFormat(
                            desc.vertex_format,
                            self.features.allowed_vertex_formats_for_blas(),
                        ));
                    }

                    let mut transform = None;

                    if blas_desc
                        .flags
                        .contains(wgt::AccelerationStructureFlags::USE_TRANSFORM)
                    {
                        transform = Some(wgpu_hal::AccelerationStructureTriangleTransform {
                            buffer: self.zero_buffer.as_ref(),
                            offset: 0,
                        })
                    }

                    if desc.vertex_count > self.limits.max_blas_primitive_count {
                        return Err(CreateBlasError::TooManyPrimitives(
                            self.limits.max_blas_primitive_count,
                            desc.vertex_count,
                        ));
                    }

                    entries.push(hal::AccelerationStructureTriangles::<dyn hal::DynBuffer> {
                        vertex_buffer: Some(self.zero_buffer.as_ref()),
                        vertex_format: desc.vertex_format,
                        first_vertex: 0,
                        vertex_count: desc.vertex_count,
                        vertex_stride: 0,
                        indices,
                        transform,
                        flags: desc.flags,
                    });
                }
                unsafe {
                    self.raw().get_acceleration_structure_build_sizes(
                        &hal::GetAccelerationStructureBuildSizesDescriptor {
                            entries: &hal::AccelerationStructureEntries::Triangles(entries),
                            flags: blas_desc.flags,
                        },
                    )
                }
            }
        };

        let raw = unsafe {
            self.raw()
                .create_acceleration_structure(&hal::AccelerationStructureDescriptor {
                    label: blas_desc.label.as_deref(),
                    size: size_info.acceleration_structure_size,
                    format: hal::AccelerationStructureFormat::BottomLevel,
                    allow_compaction: blas_desc
                        .flags
                        .contains(wgpu_types::AccelerationStructureFlags::ALLOW_COMPACTION),
                })
        }
        .map_err(|e| self.handle_hal_error_with_nonfatal_oom(e))?;

        let compaction_buffer = if blas_desc
            .flags
            .contains(wgpu_types::AccelerationStructureFlags::ALLOW_COMPACTION)
        {
            Some(ManuallyDrop::new(unsafe {
                self.raw()
                    .create_buffer(&hal::BufferDescriptor {
                        label: Some("(wgpu internal) compaction read-back buffer"),
                        size: size_of::<wgpu_types::BufferAddress>() as wgpu_types::BufferAddress,
                        usage: wgpu_types::BufferUses::ACCELERATION_STRUCTURE_QUERY
                            | wgpu_types::BufferUses::MAP_READ,
                        memory_flags: hal::MemoryFlags::PREFER_COHERENT,
                    })
                    .map_err(DeviceError::from_hal)?
            }))
        } else {
            None
        };

        let handle = unsafe {
            self.raw()
                .get_acceleration_structure_device_address(raw.as_ref())
        };

        Ok(Arc::new(resource::Blas {
            raw: Snatchable::new(raw),
            device: self.clone(),
            size_info,
            sizes,
            flags: blas_desc.flags,
            update_mode: blas_desc.update_mode,
            handle,
            label: blas_desc.label.to_string(),
            built_index: RwLock::new(rank::BLAS_BUILT_INDEX, None),
            tracking_data: TrackingData::new(self.tracker_indices.blas_s.clone()),
            compaction_buffer,
            compacted_state: Mutex::new(rank::BLAS_COMPACTION_STATE, BlasCompactState::Idle),
        }))
    }

    pub fn create_tlas(
        self: &Arc<Self>,
        desc: &resource::TlasDescriptor,
    ) -> Result<Arc<resource::Tlas>, CreateTlasError> {
        self.check_is_valid()?;
        self.require_features(Features::EXPERIMENTAL_RAY_QUERY)?;

        if desc.max_instances > self.limits.max_tlas_instance_count {
            return Err(CreateTlasError::TooManyInstances(
                self.limits.max_tlas_instance_count,
                desc.max_instances,
            ));
        }

        if desc
            .flags
            .contains(wgt::AccelerationStructureFlags::USE_TRANSFORM)
        {
            return Err(CreateTlasError::DisallowedFlag(
                wgt::AccelerationStructureFlags::USE_TRANSFORM,
            ));
        }

        if desc
            .flags
            .contains(wgt::AccelerationStructureFlags::ALLOW_RAY_HIT_VERTEX_RETURN)
        {
            self.require_features(Features::EXPERIMENTAL_RAY_HIT_VERTEX_RETURN)?;
        }

        let size_info = unsafe {
            self.raw().get_acceleration_structure_build_sizes(
                &hal::GetAccelerationStructureBuildSizesDescriptor {
                    entries: &hal::AccelerationStructureEntries::Instances(
                        hal::AccelerationStructureInstances {
                            buffer: Some(self.zero_buffer.as_ref()),
                            offset: 0,
                            count: desc.max_instances,
                        },
                    ),
                    flags: desc.flags,
                },
            )
        };

        let raw = unsafe {
            self.raw()
                .create_acceleration_structure(&hal::AccelerationStructureDescriptor {
                    label: desc.label.as_deref(),
                    size: size_info.acceleration_structure_size,
                    format: hal::AccelerationStructureFormat::TopLevel,
                    allow_compaction: false,
                })
        }
        .map_err(|e| self.handle_hal_error_with_nonfatal_oom(e))?;

        let instance_buffer_size =
            self.alignments.raw_tlas_instance_size * desc.max_instances.max(1) as usize;
        let instance_buffer = unsafe {
            self.raw().create_buffer(&hal::BufferDescriptor {
                label: hal_label(Some("(wgpu-core) instances_buffer"), self.instance_flags),
                size: instance_buffer_size as u64,
                usage: wgt::BufferUses::COPY_DST
                    | wgt::BufferUses::TOP_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                memory_flags: hal::MemoryFlags::PREFER_COHERENT,
            })
        }
        .map_err(|e| self.handle_hal_error_with_nonfatal_oom(e))?;

        Ok(Arc::new(resource::Tlas {
            raw: Snatchable::new(raw),
            device: self.clone(),
            size_info,
            flags: desc.flags,
            update_mode: desc.update_mode,
            built_index: RwLock::new(rank::TLAS_BUILT_INDEX, None),
            dependencies: RwLock::new(rank::TLAS_DEPENDENCIES, Vec::new()),
            instance_buffer: ManuallyDrop::new(instance_buffer),
            label: desc.label.to_string(),
            max_instance_count: desc.max_instances,
            tracking_data: TrackingData::new(self.tracker_indices.tlas_s.clone()),
        }))
    }
}

impl Global {
    pub fn device_create_blas(
        &self,
        device_id: id::DeviceId,
        desc: &resource::BlasDescriptor,
        sizes: wgt::BlasGeometrySizeDescriptors,
        id_in: Option<BlasId>,
    ) -> (BlasId, Option<u64>, Option<CreateBlasError>) {
        profiling::scope!("Device::create_blas");

        let fid = self.hub.blas_s.prepare(id_in);

        let error = 'error: {
            let device = self.hub.devices.get(device_id);

            #[cfg(feature = "trace")]
            let trace_sizes = sizes.clone();

            let blas = match device.create_blas(desc, sizes) {
                Ok(blas) => blas,
                Err(e) => break 'error e,
            };
            let handle = blas.handle;

            #[cfg(feature = "trace")]
            if let Some(trace) = device.trace.lock().as_mut() {
                trace.add(Action::CreateBlas {
                    id: blas.to_trace(),
                    desc: desc.clone(),
                    sizes: trace_sizes,
                });
            }

            let id = fid.assign(Fallible::Valid(blas));
            api_log!("Device::create_blas -> {id:?}");

            return (id, Some(handle), None);
        };

        let id = fid.assign(Fallible::Invalid(Arc::new(error.to_string())));
        (id, None, Some(error))
    }

    pub fn device_create_tlas(
        &self,
        device_id: id::DeviceId,
        desc: &resource::TlasDescriptor,
        id_in: Option<TlasId>,
    ) -> (TlasId, Option<CreateTlasError>) {
        profiling::scope!("Device::create_tlas");

        let fid = self.hub.tlas_s.prepare(id_in);

        let error = 'error: {
            let device = self.hub.devices.get(device_id);

            let tlas = match device.create_tlas(desc) {
                Ok(tlas) => tlas,
                Err(e) => break 'error e,
            };

            #[cfg(feature = "trace")]
            if let Some(trace) = device.trace.lock().as_mut() {
                trace.add(Action::CreateTlas {
                    id: tlas.to_trace(),
                    desc: desc.clone(),
                });
            }

            let id = fid.assign(Fallible::Valid(tlas));
            api_log!("Device::create_tlas -> {id:?}");

            return (id, None);
        };

        let id = fid.assign(Fallible::Invalid(Arc::new(error.to_string())));
        (id, Some(error))
    }

    pub fn blas_drop(&self, blas_id: BlasId) {
        profiling::scope!("Blas::drop");
        api_log!("Blas::drop {blas_id:?}");

        let _blas = self.hub.blas_s.remove(blas_id);

        #[cfg(feature = "trace")]
        if let Ok(blas) = _blas.get() {
            if let Some(t) = blas.device.trace.lock().as_mut() {
                t.add(Action::DestroyBlas(blas.to_trace()));
            }
        }
    }

    pub fn tlas_drop(&self, tlas_id: TlasId) {
        profiling::scope!("Tlas::drop");
        api_log!("Tlas::drop {tlas_id:?}");

        let _tlas = self.hub.tlas_s.remove(tlas_id);

        #[cfg(feature = "trace")]
        if let Ok(tlas) = _tlas.get() {
            if let Some(t) = tlas.device.trace.lock().as_mut() {
                t.add(Action::DestroyTlas(tlas.to_trace()));
            }
        }
    }

    pub fn blas_prepare_compact_async(
        &self,
        blas_id: BlasId,
        callback: Option<BlasCompactCallback>,
    ) -> Result<crate::SubmissionIndex, BlasPrepareCompactError> {
        profiling::scope!("Blas::prepare_compact_async");
        api_log!("Blas::prepare_compact_async {blas_id:?}");

        let hub = &self.hub;

        let compact_result = match hub.blas_s.get(blas_id).get() {
            Ok(blas) => blas.prepare_compact_async(callback),
            Err(e) => Err((callback, e.into())),
        };

        match compact_result {
            Ok(submission_index) => Ok(submission_index),
            Err((mut callback, err)) => {
                if let Some(callback) = callback.take() {
                    callback(Err(err.clone()));
                }
                Err(err)
            }
        }
    }

    pub fn ready_for_compaction(&self, blas_id: BlasId) -> Result<bool, InvalidResourceError> {
        profiling::scope!("Blas::prepare_compact_async");
        api_log!("Blas::prepare_compact_async {blas_id:?}");

        let hub = &self.hub;

        let blas = hub.blas_s.get(blas_id).get()?;

        let lock = blas.compacted_state.lock();

        Ok(matches!(*lock, BlasCompactState::Ready { .. }))
    }
}
