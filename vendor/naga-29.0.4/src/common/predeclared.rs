//! Generating names for predeclared types.

use crate::ir;

use alloc::format;
use alloc::string::String;

impl ir::PredeclaredType {
    pub fn struct_name(&self) -> String {
        use crate::PredeclaredType as Pt;
        match *self {
            Pt::AtomicCompareExchangeWeakResult(scalar) => {
                format!(
                    "__atomic_compare_exchange_result<{:?},{}>",
                    scalar.kind, scalar.width,
                )
            }
            Pt::ModfResult { size, scalar } => frexp_mod_name("modf", size, scalar),
            Pt::FrexpResult { size, scalar } => frexp_mod_name("frexp", size, scalar),
        }
    }
}

fn frexp_mod_name(function: &str, size: Option<ir::VectorSize>, scalar: ir::Scalar) -> String {
    let bits = 8 * scalar.width;
    match size {
        Some(size) => {
            let size = size as u8;
            format!("__{function}_result_vec{size}_f{bits}")
        }
        None => format!("__{function}_result_f{bits}"),
    }
}
