use wasmtime_environ::WasmValType;

/// Base register used to address the local slot.
///
/// Slots for stack arguments are addressed from the frame pointer.
/// Slots for function-defined locals and for registers are addressed
/// from the stack pointer.
#[derive(Clone, Eq, PartialEq, Copy, Debug)]
enum Base {
    FP,
    SP,
}

/// A local slot.
///
/// Represents the type, location and addressing mode of a local
/// in the stack's local and argument area.
/// LocalSlots are well known slots in the machine stack, and are generally
/// reference by the stack pointer register (SP) or the base pointer register (FP).
/// * Local slots that are referenced by the stack pointer register are the
///   function defined locals and the param locals.
/// * Local slots that represent arguments in the stack, are referenced through the
///   base pointer register.
///
/// A [crate::masm::StackSlot] is a generalized form of a [LocalSlot]: they
/// represent dynamic chunks of memory that get created throughout the function
/// compilation lifetime when spilling values (register and locals) into the
/// machine stack. A [LocalSlot] on the other hand gets created at the beginning
/// of a function compilation and gets cleaned up at the end.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalSlot {
    /// The offset of the local slot.
    pub offset: u32,
    /// The type contained by this local slot.
    pub ty: WasmValType,
    /// Base register associated to this local slot.
    base: Base,
}

impl LocalSlot {
    /// Creates a local slot for a function defined local or
    /// for a spilled argument register.
    pub fn new(ty: WasmValType, offset: u32) -> Self {
        Self {
            ty,
            offset,
            base: Base::SP,
        }
    }

    /// Int32 shortcut for `new`.
    pub fn i32(offset: u32) -> Self {
        Self {
            ty: WasmValType::I32,
            offset,
            base: Base::SP,
        }
    }

    /// Int64 shortcut for `new`.
    pub fn i64(offset: u32) -> Self {
        Self {
            ty: WasmValType::I64,
            offset,
            base: Base::SP,
        }
    }

    /// Creates a local slot for a stack function argument.
    pub fn stack_arg(ty: WasmValType, offset: u32) -> Self {
        Self {
            ty,
            offset,
            base: Base::FP,
        }
    }

    /// Check if the local is addressed from the stack pointer.
    pub fn addressed_from_sp(&self) -> bool {
        self.base == Base::SP
    }
}
