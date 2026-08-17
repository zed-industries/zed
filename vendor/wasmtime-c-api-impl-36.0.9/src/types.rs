#[repr(C)]
#[derive(Clone)]
pub struct wasm_limits_t {
    pub min: u32,
    pub max: u32,
}

impl wasm_limits_t {
    pub(crate) fn max(&self) -> Option<u32> {
        if self.max == u32::max_value() {
            None
        } else {
            Some(self.max)
        }
    }
}

mod export;
mod r#extern;
mod func;
mod global;
mod import;
mod memory;
mod table;
mod val;
pub use self::export::*;
pub use self::r#extern::*;
pub use self::func::*;
pub use self::global::*;
pub use self::import::*;
pub use self::memory::*;
pub use self::table::*;
pub use self::val::*;
