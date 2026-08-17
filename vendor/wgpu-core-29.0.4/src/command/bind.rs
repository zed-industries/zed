use alloc::{boxed::Box, sync::Arc, vec::Vec};

use thiserror::Error;

use crate::{
    binding_model::{BindGroup, LateMinBufferBindingSizeMismatch, PipelineLayout},
    pipeline::LateSizedBufferGroup,
    resource::{Labeled, ParentDevice, ResourceErrorIdent},
};

mod compat {
    use alloc::{
        string::{String, ToString as _},
        sync::{Arc, Weak},
        vec::Vec,
    };
    use core::num::NonZeroU32;

    use thiserror::Error;
    use wgt::{BindingType, ShaderStages};

    use crate::{
        binding_model::BindGroupLayout,
        error::MultiError,
        resource::{Labeled, ParentDevice, ResourceErrorIdent},
    };

    pub(crate) enum Error {
        Incompatible {
            expected_bgl: ResourceErrorIdent,
            assigned_bgl: ResourceErrorIdent,
            inner: MultiError,
        },
        Missing,
    }

    #[derive(Debug, Clone)]
    struct Entry {
        assigned: Option<Arc<BindGroupLayout>>,
        expected: Option<Arc<BindGroupLayout>>,
    }

    impl Entry {
        const fn empty() -> Self {
            Self {
                assigned: None,
                expected: None,
            }
        }

        fn is_active(&self) -> bool {
            self.assigned.is_some() && self.expected.is_some()
        }

        fn is_valid(&self) -> bool {
            if let Some(expected_bgl) = self.expected.as_ref() {
                if let Some(assigned_bgl) = self.assigned.as_ref() {
                    expected_bgl.is_equal(assigned_bgl)
                } else {
                    false
                }
            } else {
                false
            }
        }

        fn check(&self) -> Result<(), Error> {
            if let Some(expected_bgl) = self.expected.as_ref() {
                if let Some(assigned_bgl) = self.assigned.as_ref() {
                    if expected_bgl.is_equal(assigned_bgl) {
                        Ok(())
                    } else {
                        #[derive(Clone, Debug, Error)]
                        #[error(
                            "Exclusive pipelines don't match: expected {expected}, got {assigned}"
                        )]
                        struct IncompatibleExclusivePipelines {
                            expected: String,
                            assigned: String,
                        }

                        use crate::binding_model::ExclusivePipeline;
                        match (
                            expected_bgl.exclusive_pipeline.get().unwrap(),
                            assigned_bgl.exclusive_pipeline.get().unwrap(),
                        ) {
                            (ExclusivePipeline::None, ExclusivePipeline::None) => {}
                            (
                                ExclusivePipeline::Render(e_pipeline),
                                ExclusivePipeline::Render(a_pipeline),
                            ) if Weak::ptr_eq(e_pipeline, a_pipeline) => {}
                            (
                                ExclusivePipeline::Compute(e_pipeline),
                                ExclusivePipeline::Compute(a_pipeline),
                            ) if Weak::ptr_eq(e_pipeline, a_pipeline) => {}
                            (expected, assigned) => {
                                return Err(Error::Incompatible {
                                    expected_bgl: expected_bgl.error_ident(),
                                    assigned_bgl: assigned_bgl.error_ident(),
                                    inner: MultiError::new(core::iter::once(
                                        IncompatibleExclusivePipelines {
                                            expected: expected.to_string(),
                                            assigned: assigned.to_string(),
                                        },
                                    ))
                                    .unwrap(),
                                });
                            }
                        }

                        #[derive(Clone, Debug, Error)]
                        enum EntryError {
                            #[error("Entries with binding {binding} differ in visibility: expected {expected:?}, got {assigned:?}")]
                            Visibility {
                                binding: u32,
                                expected: ShaderStages,
                                assigned: ShaderStages,
                            },
                            #[error("Entries with binding {binding} differ in type: expected {expected:?}, got {assigned:?}")]
                            Type {
                                binding: u32,
                                expected: BindingType,
                                assigned: BindingType,
                            },
                            #[error("Entries with binding {binding} differ in count: expected {expected:?}, got {assigned:?}")]
                            Count {
                                binding: u32,
                                expected: Option<NonZeroU32>,
                                assigned: Option<NonZeroU32>,
                            },
                            #[error("Expected entry with binding {binding} not found in assigned bind group layout")]
                            ExtraExpected { binding: u32 },
                            #[error("Assigned entry with binding {binding} not found in expected bind group layout")]
                            ExtraAssigned { binding: u32 },
                        }

                        let mut errors = Vec::new();

                        for (&binding, expected_entry) in expected_bgl.entries.iter() {
                            if let Some(assigned_entry) = assigned_bgl.entries.get(binding) {
                                if assigned_entry.visibility != expected_entry.visibility {
                                    errors.push(EntryError::Visibility {
                                        binding,
                                        expected: expected_entry.visibility,
                                        assigned: assigned_entry.visibility,
                                    });
                                }
                                if assigned_entry.ty != expected_entry.ty {
                                    errors.push(EntryError::Type {
                                        binding,
                                        expected: expected_entry.ty,
                                        assigned: assigned_entry.ty,
                                    });
                                }
                                if assigned_entry.count != expected_entry.count {
                                    errors.push(EntryError::Count {
                                        binding,
                                        expected: expected_entry.count,
                                        assigned: assigned_entry.count,
                                    });
                                }
                            } else {
                                errors.push(EntryError::ExtraExpected { binding });
                            }
                        }

                        for (&binding, _) in assigned_bgl.entries.iter() {
                            if !expected_bgl.entries.contains_key(binding) {
                                errors.push(EntryError::ExtraAssigned { binding });
                            }
                        }

                        Err(Error::Incompatible {
                            expected_bgl: expected_bgl.error_ident(),
                            assigned_bgl: assigned_bgl.error_ident(),
                            inner: MultiError::new(errors.drain(..)).unwrap(),
                        })
                    }
                } else {
                    Err(Error::Missing)
                }
            } else {
                Ok(())
            }
        }
    }

    #[derive(Debug)]
    pub(super) struct BoundBindGroupLayouts {
        entries: [Entry; hal::MAX_BIND_GROUPS],
        rebind_start: usize,
    }

    impl BoundBindGroupLayouts {
        pub fn new() -> Self {
            Self {
                entries: [const { Entry::empty() }; hal::MAX_BIND_GROUPS],
                rebind_start: 0,
            }
        }

        /// Takes the start index of the bind group range to be rebound, and clears it.
        pub fn take_rebind_start_index(&mut self) -> usize {
            let start = self.rebind_start;
            self.rebind_start = self.entries.len();
            start
        }

        pub fn update_rebind_start_index(&mut self, start_index: usize) {
            self.rebind_start = self.rebind_start.min(start_index);
        }

        pub fn update_expectations(&mut self, expectations: &[Option<Arc<BindGroupLayout>>]) {
            let mut rebind_start_index = None;

            for (i, (e, new_expected_bgl)) in self
                .entries
                .iter_mut()
                .zip(expectations.iter().chain(core::iter::repeat(&None)))
                .enumerate()
            {
                let (must_set, must_rebind) = match (&mut e.expected, new_expected_bgl) {
                    (None, None) => (false, false),
                    (None, Some(_)) => (true, true),
                    (Some(_), None) => (true, false),
                    (Some(old_expected_bgl), Some(new_expected_bgl)) => {
                        let is_different = !old_expected_bgl.is_equal(new_expected_bgl);
                        (is_different, is_different)
                    }
                };
                if must_set {
                    e.expected = new_expected_bgl.clone();
                }
                if must_rebind && rebind_start_index.is_none() {
                    rebind_start_index = Some(i);
                }
            }

            if let Some(rebind_start_index) = rebind_start_index {
                self.update_rebind_start_index(rebind_start_index);
            }
        }

        pub fn assign(&mut self, index: usize, value: Arc<BindGroupLayout>) {
            self.entries[index].assigned = Some(value);
            self.update_rebind_start_index(index);
        }

        pub fn clear(&mut self, index: usize) {
            self.entries[index].assigned = None;
        }

        pub fn list_active(&self) -> impl Iterator<Item = usize> + '_ {
            self.entries
                .iter()
                .enumerate()
                .filter_map(|(i, e)| if e.is_active() { Some(i) } else { None })
        }

        pub fn list_valid(&self) -> impl Iterator<Item = usize> + '_ {
            self.entries
                .iter()
                .enumerate()
                .filter_map(|(i, e)| if e.is_valid() { Some(i) } else { None })
        }

        #[allow(clippy::result_large_err)]
        pub fn get_invalid(&self) -> Result<(), (usize, Error)> {
            for (index, entry) in self.entries.iter().enumerate() {
                entry.check().map_err(|e| (index, e))?;
            }
            Ok(())
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum BinderError {
    #[error("The current set {pipeline} expects a BindGroup to be set at index {index}")]
    MissingBindGroup {
        index: usize,
        pipeline: ResourceErrorIdent,
    },
    #[error("The {assigned_bgl} of current set {assigned_bg} at index {index} is not compatible with the corresponding {expected_bgl} of {pipeline}")]
    IncompatibleBindGroup {
        expected_bgl: ResourceErrorIdent,
        assigned_bgl: ResourceErrorIdent,
        assigned_bg: ResourceErrorIdent,
        index: usize,
        pipeline: ResourceErrorIdent,
        #[source]
        inner: crate::error::MultiError,
    },
}

#[derive(Debug)]
struct LateBufferBinding {
    binding_index: u32,
    shader_expect_size: wgt::BufferAddress,
    bound_size: wgt::BufferAddress,
}

#[derive(Debug, Default)]
struct EntryPayload {
    group: Option<Arc<BindGroup>>,
    dynamic_offsets: Vec<wgt::DynamicOffset>,
    late_buffer_bindings: Vec<LateBufferBinding>,
    /// Since `LateBufferBinding` may contain information about the bindings
    /// not used by the pipeline, we need to know when to stop validating.
    late_bindings_effective_count: usize,
}

impl EntryPayload {
    fn reset(&mut self) {
        self.group = None;
        self.dynamic_offsets.clear();
        self.late_buffer_bindings.clear();
        self.late_bindings_effective_count = 0;
    }
}

#[derive(Debug)]
pub(super) struct Binder {
    pub(super) pipeline_layout: Option<Arc<PipelineLayout>>,
    manager: compat::BoundBindGroupLayouts,
    payloads: [EntryPayload; hal::MAX_BIND_GROUPS],
}

impl Binder {
    pub(super) fn new() -> Self {
        Self {
            pipeline_layout: None,
            manager: compat::BoundBindGroupLayouts::new(),
            payloads: Default::default(),
        }
    }
    pub(super) fn reset(&mut self) {
        self.pipeline_layout = None;
        self.manager = compat::BoundBindGroupLayouts::new();
        for payload in self.payloads.iter_mut() {
            payload.reset();
        }
    }

    /// Returns `true` if the pipeline layout has been changed, i.e. if the
    /// new PL was not the same as the old PL.
    pub(super) fn change_pipeline_layout<'a>(
        &'a mut self,
        new: &Arc<PipelineLayout>,
        late_sized_buffer_groups: &[LateSizedBufferGroup],
    ) -> bool {
        self.update_late_buffer_bindings(late_sized_buffer_groups);

        if let Some(old) = self.pipeline_layout.as_ref() {
            if old.is_equal(new) {
                return false;
            }
        }

        let old = self.pipeline_layout.replace(new.clone());

        self.manager.update_expectations(&new.bind_group_layouts);

        if let Some(old) = old {
            // root constants are the base compatibility property
            if old.immediate_size != new.immediate_size {
                self.manager.update_rebind_start_index(0);
            }
        }

        true
    }

    pub(super) fn assign_group<'a>(
        &'a mut self,
        index: usize,
        bind_group: &Arc<BindGroup>,
        offsets: &[wgt::DynamicOffset],
    ) {
        let payload = &mut self.payloads[index];
        payload.group = Some(bind_group.clone());
        payload.dynamic_offsets.clear();
        payload.dynamic_offsets.extend_from_slice(offsets);

        // Fill out the actual binding sizes for buffers,
        // whose layout doesn't specify `min_binding_size`.

        // Update entries that already exist as the pipeline was bound before the group
        // was bound.
        for (late_binding, late_info) in payload
            .late_buffer_bindings
            .iter_mut()
            .zip(bind_group.late_buffer_binding_infos.iter())
        {
            late_binding.binding_index = late_info.binding_index;
            late_binding.bound_size = late_info.size.get();
        }

        // Add new entries for the bindings that were not known when the pipeline was bound.
        if bind_group.late_buffer_binding_infos.len() > payload.late_buffer_bindings.len() {
            for late_info in
                bind_group.late_buffer_binding_infos[payload.late_buffer_bindings.len()..].iter()
            {
                payload.late_buffer_bindings.push(LateBufferBinding {
                    binding_index: late_info.binding_index,
                    shader_expect_size: 0,
                    bound_size: late_info.size.get(),
                });
            }
        }

        self.manager.assign(index, bind_group.layout.clone());
    }

    pub(super) fn clear_group(&mut self, index: usize) {
        self.payloads[index].reset();
        self.manager.clear(index);
    }

    /// Takes the start index of the bind group range to be rebound, and clears it.
    pub(super) fn take_rebind_start_index(&mut self) -> usize {
        self.manager.take_rebind_start_index()
    }

    pub(super) fn list_valid_with_start(
        &self,
        start: usize,
    ) -> impl Iterator<Item = (usize, &Arc<BindGroup>, &[wgt::DynamicOffset])> + '_ {
        let payloads = &self.payloads;
        self.manager
            .list_valid()
            .filter(move |i| *i >= start)
            .map(move |index| {
                (
                    index,
                    payloads[index].group.as_ref().unwrap(),
                    payloads[index].dynamic_offsets.as_slice(),
                )
            })
    }

    pub(super) fn list_active(&self) -> impl Iterator<Item = &Arc<BindGroup>> + '_ {
        let payloads = &self.payloads;
        self.manager
            .list_active()
            .map(move |index| payloads[index].group.as_ref().unwrap())
    }

    pub(super) fn list_valid(
        &self,
    ) -> impl Iterator<Item = (usize, &Arc<BindGroup>, &[wgt::DynamicOffset])> + '_ {
        self.list_valid_with_start(0)
    }

    pub(super) fn check_compatibility<T: Labeled>(
        &self,
        pipeline: &T,
    ) -> Result<(), Box<BinderError>> {
        self.manager.get_invalid().map_err(|(index, error)| {
            Box::new(match error {
                compat::Error::Incompatible {
                    expected_bgl,
                    assigned_bgl,
                    inner,
                } => BinderError::IncompatibleBindGroup {
                    expected_bgl,
                    assigned_bgl,
                    assigned_bg: self.payloads[index].group.as_ref().unwrap().error_ident(),
                    index,
                    pipeline: pipeline.error_ident(),
                    inner,
                },
                compat::Error::Missing => BinderError::MissingBindGroup {
                    index,
                    pipeline: pipeline.error_ident(),
                },
            })
        })
    }

    /// Scan active buffer bindings corresponding to layouts without `min_binding_size` specified.
    pub(super) fn check_late_buffer_bindings(
        &self,
    ) -> Result<(), LateMinBufferBindingSizeMismatch> {
        for group_index in self.manager.list_active() {
            let payload = &self.payloads[group_index];
            for late_binding in
                &payload.late_buffer_bindings[..payload.late_bindings_effective_count]
            {
                if late_binding.bound_size < late_binding.shader_expect_size {
                    return Err(LateMinBufferBindingSizeMismatch {
                        group_index: group_index as u32,
                        binding_index: late_binding.binding_index,
                        shader_size: late_binding.shader_expect_size,
                        bound_size: late_binding.bound_size,
                    });
                }
            }
        }
        Ok(())
    }

    /// This must be called even when a new pipeline has the same layout
    /// as the previous one, because different pipelines can have different
    /// shader-expected buffer sizes even with identical layouts.
    fn update_late_buffer_bindings(&mut self, late_sized_buffer_groups: &[LateSizedBufferGroup]) {
        for (payload, late_group) in self.payloads.iter_mut().zip(late_sized_buffer_groups) {
            payload.late_bindings_effective_count = late_group.shader_sizes.len();

            // Update entries that already exist as the bind group was bound before the pipeline
            // was bound.
            for (late_binding, &shader_expect_size) in payload
                .late_buffer_bindings
                .iter_mut()
                .zip(late_group.shader_sizes.iter())
            {
                late_binding.shader_expect_size = shader_expect_size;
            }

            // Add new entries for the bindings that were not known when the bind group was bound.
            if late_group.shader_sizes.len() > payload.late_buffer_bindings.len() {
                for &shader_expect_size in
                    late_group.shader_sizes[payload.late_buffer_bindings.len()..].iter()
                {
                    payload.late_buffer_bindings.push(LateBufferBinding {
                        binding_index: 0,
                        shader_expect_size,
                        bound_size: 0,
                    });
                }
            }
        }
    }
}
