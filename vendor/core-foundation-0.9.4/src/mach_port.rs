use crate::base::TCFType;
use crate::runloop::CFRunLoopSource;
use core_foundation_sys::base::kCFAllocatorDefault;
pub use core_foundation_sys::mach_port::*;

declare_TCFType! {
    /// An immutable numeric value.
    CFMachPort, CFMachPortRef
}
impl_TCFType!(CFMachPort, CFMachPortRef, CFMachPortGetTypeID);
impl_CFTypeDescription!(CFMachPort);

impl CFMachPort {
    pub fn create_runloop_source(&self, order: CFIndex) -> Result<CFRunLoopSource, ()> {
        unsafe {
            let runloop_source_ref =
                CFMachPortCreateRunLoopSource(kCFAllocatorDefault, self.0, order);
            if runloop_source_ref.is_null() {
                Err(())
            } else {
                Ok(CFRunLoopSource::wrap_under_create_rule(runloop_source_ref))
            }
        }
    }
}
