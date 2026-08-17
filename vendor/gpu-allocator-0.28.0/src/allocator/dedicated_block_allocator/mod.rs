#![deny(unsafe_code, clippy::unwrap_used)]
#[cfg(feature = "std")]
use alloc::sync::Arc;
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec,
};
#[cfg(feature = "std")]
use std::backtrace::Backtrace;

use log::{log, Level};

#[cfg(feature = "visualizer")]
pub(crate) mod visualizer;

use super::{AllocationReport, AllocationType, SubAllocator, SubAllocatorBase};
use crate::{AllocationError, Result};

#[derive(Debug)]
pub(crate) struct DedicatedBlockAllocator {
    size: u64,
    allocated: u64,
    /// Only used if [`crate::AllocatorDebugSettings::store_stack_traces`] is [`true`]
    name: Option<String>,
    #[cfg(feature = "std")]
    backtrace: Arc<Backtrace>,
}

impl DedicatedBlockAllocator {
    pub(crate) fn new(size: u64) -> Self {
        Self {
            size,
            allocated: 0,
            name: None,
            #[cfg(feature = "std")]
            backtrace: Arc::new(Backtrace::disabled()),
        }
    }
}

impl SubAllocatorBase for DedicatedBlockAllocator {}
impl SubAllocator for DedicatedBlockAllocator {
    fn allocate(
        &mut self,
        size: u64,
        _alignment: u64,
        _allocation_type: AllocationType,
        _granularity: u64,
        name: &str,
        #[cfg(feature = "std")] backtrace: Arc<Backtrace>,
    ) -> Result<(u64, core::num::NonZeroU64)> {
        if self.allocated != 0 {
            return Err(AllocationError::OutOfMemory);
        }

        if self.size != size {
            return Err(AllocationError::Internal(
                "DedicatedBlockAllocator size must match allocation size.".into(),
            ));
        }

        self.allocated = size;
        self.name = Some(name.to_string());
        #[cfg(feature = "std")]
        {
            self.backtrace = backtrace;
        }

        #[allow(clippy::unwrap_used)]
        let dummy_id = core::num::NonZeroU64::new(1).unwrap();
        Ok((0, dummy_id))
    }

    fn free(&mut self, chunk_id: Option<core::num::NonZeroU64>) -> Result<()> {
        if chunk_id != core::num::NonZeroU64::new(1) {
            Err(AllocationError::Internal("Chunk ID must be 1.".into()))
        } else {
            self.allocated = 0;
            Ok(())
        }
    }

    fn rename_allocation(
        &mut self,
        chunk_id: Option<core::num::NonZeroU64>,
        name: &str,
    ) -> Result<()> {
        if chunk_id != core::num::NonZeroU64::new(1) {
            Err(AllocationError::Internal("Chunk ID must be 1.".into()))
        } else {
            self.name = Some(name.into());
            Ok(())
        }
    }

    fn report_memory_leaks(
        &self,
        log_level: Level,
        memory_type_index: usize,
        memory_block_index: usize,
    ) {
        let empty = "".to_string();
        let name = self.name.as_ref().unwrap_or(&empty);
        let backtrace_info;
        #[cfg(feature = "std")]
        {
            // TODO: Allocation could be avoided here if https://github.com/rust-lang/rust/pull/139135 is merged and stabilized.
            backtrace_info = format!(
                ",
        backtrace: {}",
                self.backtrace
            )
        }
        #[cfg(not(feature = "std"))]
        {
            backtrace_info = ""
        }

        log!(
            log_level,
            r#"leak detected: {{
    memory type: {}
    memory block: {}
    dedicated allocation: {{
        size: 0x{:x},
        name: {}{backtrace_info}
    }}
}}"#,
            memory_type_index,
            memory_block_index,
            self.size,
            name,
        );
    }

    fn report_allocations(&self) -> Vec<AllocationReport> {
        vec![AllocationReport {
            name: self
                .name
                .clone()
                .unwrap_or_else(|| "<Unnamed Dedicated allocation>".to_owned()),
            offset: 0,
            size: self.size,
            #[cfg(feature = "visualizer")]
            backtrace: self.backtrace.clone(),
        }]
    }

    fn allocated(&self) -> u64 {
        self.allocated
    }

    fn supports_general_allocations(&self) -> bool {
        false
    }
}
