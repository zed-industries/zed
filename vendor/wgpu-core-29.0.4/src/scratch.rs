use alloc::{boxed::Box, sync::Arc};
use core::mem::ManuallyDrop;

use wgt::BufferUses;

use crate::device::{Device, DeviceError};
use crate::{hal_label, resource_log};

#[derive(Debug)]
pub struct ScratchBuffer {
    raw: ManuallyDrop<Box<dyn hal::DynBuffer>>,
    device: Arc<Device>,
}

impl ScratchBuffer {
    pub(crate) fn new(device: &Arc<Device>, size: wgt::BufferSize) -> Result<Self, DeviceError> {
        let raw = unsafe {
            device
                .raw()
                .create_buffer(&hal::BufferDescriptor {
                    label: hal_label(Some("(wgpu) scratch buffer"), device.instance_flags),
                    size: size.get(),
                    usage: BufferUses::ACCELERATION_STRUCTURE_SCRATCH,
                    memory_flags: hal::MemoryFlags::empty(),
                })
                .map_err(DeviceError::from_hal)?
        };
        Ok(Self {
            raw: ManuallyDrop::new(raw),
            device: device.clone(),
        })
    }
    pub(crate) fn raw(&self) -> &dyn hal::DynBuffer {
        self.raw.as_ref()
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        resource_log!("Destroy raw ScratchBuffer");
        // SAFETY: We are in the Drop impl and we don't use self.raw anymore after this point.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };
        unsafe { self.device.raw().destroy_buffer(raw) };
    }
}
