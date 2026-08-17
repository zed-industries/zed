//! Stack slots.
//!
//! The `StackSlotData` struct keeps track of a single stack slot in a function.
//!

use crate::entity::PrimaryMap;
use crate::ir::StackSlot;
use crate::ir::entities::{DynamicStackSlot, DynamicType};
use core::fmt;
use core::str::FromStr;

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// The size of an object on the stack, or the size of a stack frame.
///
/// We don't use `usize` to represent object sizes on the target platform because Cranelift supports
/// cross-compilation, and `usize` is a type that depends on the host platform, not the target
/// platform.
pub type StackSize = u32;

/// The kind of a stack slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum StackSlotKind {
    /// An explicit stack slot. This is a chunk of stack memory for use by the `stack_load`
    /// and `stack_store` instructions.
    ExplicitSlot,
    /// An explicit stack slot for dynamic vector types. This is a chunk of stack memory
    /// for use by the `dynamic_stack_load` and `dynamic_stack_store` instructions.
    ExplicitDynamicSlot,
}

impl FromStr for StackSlotKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use self::StackSlotKind::*;
        match s {
            "explicit_slot" => Ok(ExplicitSlot),
            "explicit_dynamic_slot" => Ok(ExplicitDynamicSlot),
            _ => Err(()),
        }
    }
}

impl fmt::Display for StackSlotKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::StackSlotKind::*;
        f.write_str(match *self {
            ExplicitSlot => "explicit_slot",
            ExplicitDynamicSlot => "explicit_dynamic_slot",
        })
    }
}

/// Contents of a stack slot.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct StackSlotData {
    /// The kind of stack slot.
    pub kind: StackSlotKind,

    /// Size of stack slot in bytes.
    pub size: StackSize,

    /// Alignment of stack slot as a power-of-two exponent (log2
    /// value). The stack slot will be at least this aligned; it may
    /// be aligned according to other considerations, such as minimum
    /// stack slot size or machine word size, as well.
    pub align_shift: u8,
}

impl StackSlotData {
    /// Create a stack slot with the specified byte size and alignment.
    pub fn new(kind: StackSlotKind, size: StackSize, align_shift: u8) -> Self {
        Self {
            kind,
            size,
            align_shift,
        }
    }
}

impl fmt::Display for StackSlotData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.align_shift != 0 {
            write!(
                f,
                "{} {}, align = {}",
                self.kind,
                self.size,
                1u32 << self.align_shift
            )
        } else {
            write!(f, "{} {}", self.kind, self.size)
        }
    }
}

/// Contents of a dynamic stack slot.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct DynamicStackSlotData {
    /// The kind of stack slot.
    pub kind: StackSlotKind,

    /// The type of this slot.
    pub dyn_ty: DynamicType,
}

impl DynamicStackSlotData {
    /// Create a stack slot with the specified byte size.
    pub fn new(kind: StackSlotKind, dyn_ty: DynamicType) -> Self {
        assert!(kind == StackSlotKind::ExplicitDynamicSlot);
        Self { kind, dyn_ty }
    }

    /// Get the alignment in bytes of this stack slot given the stack pointer alignment.
    pub fn alignment(&self, max_align: StackSize) -> StackSize {
        debug_assert!(max_align.is_power_of_two());
        max_align
    }
}

impl fmt::Display for DynamicStackSlotData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.dyn_ty)
    }
}

/// All allocated stack slots.
pub type StackSlots = PrimaryMap<StackSlot, StackSlotData>;

/// All allocated dynamic stack slots.
pub type DynamicStackSlots = PrimaryMap<DynamicStackSlot, DynamicStackSlotData>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Function;
    use crate::ir::types::*;
    use crate::ir::{DynamicTypeData, GlobalValueData};
    use alloc::string::ToString;

    #[test]
    fn stack_slot() {
        let mut func = Function::new();

        let ss0 =
            func.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 4, 0));
        let ss1 =
            func.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8, 0));
        assert_eq!(ss0.to_string(), "ss0");
        assert_eq!(ss1.to_string(), "ss1");

        assert_eq!(func.sized_stack_slots[ss0].size, 4);
        assert_eq!(func.sized_stack_slots[ss1].size, 8);

        assert_eq!(func.sized_stack_slots[ss0].to_string(), "explicit_slot 4");
        assert_eq!(func.sized_stack_slots[ss1].to_string(), "explicit_slot 8");
    }

    #[test]
    fn dynamic_stack_slot() {
        let mut func = Function::new();

        let int_vector_ty = I32X4;
        let fp_vector_ty = F64X2;
        let scale0 = GlobalValueData::DynScaleTargetConst {
            vector_type: int_vector_ty,
        };
        let scale1 = GlobalValueData::DynScaleTargetConst {
            vector_type: fp_vector_ty,
        };
        let gv0 = func.create_global_value(scale0);
        let gv1 = func.create_global_value(scale1);
        let dtd0 = DynamicTypeData::new(int_vector_ty, gv0);
        let dtd1 = DynamicTypeData::new(fp_vector_ty, gv1);
        let dt0 = func.dfg.make_dynamic_ty(dtd0);
        let dt1 = func.dfg.make_dynamic_ty(dtd1);

        let dss0 = func.create_dynamic_stack_slot(DynamicStackSlotData::new(
            StackSlotKind::ExplicitDynamicSlot,
            dt0,
        ));
        let dss1 = func.create_dynamic_stack_slot(DynamicStackSlotData::new(
            StackSlotKind::ExplicitDynamicSlot,
            dt1,
        ));
        assert_eq!(dss0.to_string(), "dss0");
        assert_eq!(dss1.to_string(), "dss1");

        assert_eq!(
            func.dynamic_stack_slots[dss0].to_string(),
            "explicit_dynamic_slot dt0"
        );
        assert_eq!(
            func.dynamic_stack_slots[dss1].to_string(),
            "explicit_dynamic_slot dt1"
        );
    }
}
