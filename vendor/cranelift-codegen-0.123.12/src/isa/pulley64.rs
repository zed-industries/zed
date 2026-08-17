pub use super::pulley_shared::isa_builder;

use super::pulley_shared::PulleyTargetKind;
use crate::isa::pulley_shared::PointerWidth;

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Pulley64;

impl PulleyTargetKind for Pulley64 {
    fn pointer_width() -> PointerWidth {
        PointerWidth::PointerWidth64
    }
}
