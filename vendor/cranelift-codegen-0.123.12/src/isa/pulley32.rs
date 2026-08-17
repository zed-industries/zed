pub use super::pulley_shared::isa_builder;

use super::pulley_shared::PulleyTargetKind;
use crate::isa::pulley_shared::PointerWidth;

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Pulley32;

impl PulleyTargetKind for Pulley32 {
    fn pointer_width() -> PointerWidth {
        PointerWidth::PointerWidth32
    }
}
