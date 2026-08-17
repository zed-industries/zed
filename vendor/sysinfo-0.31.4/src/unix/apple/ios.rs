// Take a look at the license at the top of the repository in the LICENSE file.

pub mod ffi {}
#[cfg(feature = "component")]
pub use crate::sys::app_store::component;
#[cfg(feature = "system")]
pub use crate::sys::app_store::process;
