//! Transform support

use core_foundation::base::{CFType, TCFType};
use core_foundation::error::CFError;
use core_foundation::string::CFString;
use security_framework_sys::transform::*;
use std::ptr;

declare_TCFType! {
    /// A type representing a transform.
    SecTransform, SecTransformRef
}
impl_TCFType!(SecTransform, SecTransformRef, SecTransformGetTypeID);

unsafe impl Sync for SecTransform {}
unsafe impl Send for SecTransform {}

impl SecTransform {
    /// Sets an attribute of the transform.
    pub fn set_attribute<T>(&mut self, key: &CFString, value: &T) -> Result<(), CFError>
    where
        T: TCFType,
    {
        unsafe {
            let mut error = ptr::null_mut();
            SecTransformSetAttribute(
                self.0,
                key.as_concrete_TypeRef(),
                value.as_CFTypeRef(),
                &mut error,
            );
            if !error.is_null() {
                return Err(CFError::wrap_under_create_rule(error));
            }

            Ok(())
        }
    }

    /// Executes the transform.
    ///
    /// The return type depends on the type of transform.
    pub fn execute(&mut self) -> Result<CFType, CFError> {
        unsafe {
            let mut error = ptr::null_mut();
            let result = SecTransformExecute(self.0, &mut error);
            if result.is_null() {
                return Err(CFError::wrap_under_create_rule(error));
            }

            Ok(CFType::wrap_under_create_rule(result))
        }
    }
}
