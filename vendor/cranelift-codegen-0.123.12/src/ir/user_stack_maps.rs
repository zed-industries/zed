//! User-defined stack maps.
//!
//! This module provides types allowing users to define stack maps and associate
//! them with safepoints.
//!
//! A **safepoint** is a program point (i.e. CLIF instruction) where it must be
//! safe to run GC. Currently all non-tail call instructions are considered
//! safepoints. (This does *not* allow, for example, skipping safepoints for
//! calls that are statically known not to trigger collections, or to have a
//! safepoint on a volatile load to a page that gets protected when it is time
//! to GC, triggering a fault that pauses the mutator and lets the collector do
//! its work before resuming the mutator. We can lift this restriction in the
//! future, if necessary.)
//!
//! A **stack map** is a description of where to find all the GC-managed values
//! that are live at a particular safepoint. Stack maps let the collector find
//! on-stack roots. Each stack map is logically a set of offsets into the stack
//! frame and the type of value at that associated offset. However, because the
//! stack layout isn't defined until much later in the compiler's pipeline, each
//! stack map entry instead includes both an `ir::StackSlot` and an offset
//! within that slot.
//!
//! These stack maps are **user-defined** in that it is the CLIF producer's
//! responsibility to identify and spill the live GC-managed values and attach
//! the associated stack map entries to each safepoint themselves (see
//! `cranelift_frontend::Function::declare_needs_stack_map` and
//! `cranelift_codegen::ir::DataFlowGraph::append_user_stack_map_entry`). Cranelift
//! will not insert spills and record these stack map entries automatically.
//!
//! Logically, a set of stack maps for a function record a table of the form:
//!
//! ```text
//! +---------------------+-------------------------------------------+
//! | Instruction Pointer | SP-Relative Offsets of Live GC References |
//! +---------------------+-------------------------------------------+
//! | 0x12345678          | 2, 6, 12                                  |
//! | 0x1234abcd          | 2, 6                                      |
//! | ...                 | ...                                       |
//! +---------------------+-------------------------------------------+
//! ```
//!
//! Where "instruction pointer" is an instruction pointer within the function,
//! and "offsets of live GC references" contains the offsets (in units of words)
//! from the frame's stack pointer where live GC references are stored on the
//! stack. Instruction pointers within the function that do not have an entry in
//! this table are not GC safepoints.
//!
//! Because
//!
//! * offsets of live GC references are relative from the stack pointer, and
//! * stack frames grow down from higher addresses to lower addresses,
//!
//! to get a pointer to a live reference at offset `x` within a stack frame, you
//! add `x` to the frame's stack pointer.
//!
//! For example, to calculate the pointer to the live GC reference inside "frame
//! 1" below, you would do `frame_1_sp + x`:
//!
//! ```text
//!           Stack
//!         +-------------------+
//!         | Frame 0           |
//!         |                   |
//!    |    |                   |
//!    |    +-------------------+ <--- Frame 0's SP
//!    |    | Frame 1           |
//!  Grows  |                   |
//!  down   |                   |
//!    |    | Live GC reference | --+--
//!    |    |                   |   |
//!    |    |                   |   |
//!    V    |                   |   x = offset of live GC reference
//!         |                   |   |
//!         |                   |   |
//!         +-------------------+ --+--  <--- Frame 1's SP
//!         | Frame 2           |
//!         | ...               |
//! ```
//!
//! An individual `UserStackMap` is associated with just one instruction pointer
//! within the function, contains the size of the stack frame, and represents
//! the stack frame as a bitmap. There is one bit per word in the stack frame,
//! and if the bit is set, then the word contains a live GC reference.
//!
//! Note that a caller's outgoing argument stack slots (if any) and callee's
//! incoming argument stack slots (if any) overlap, so we must choose which
//! function's stack maps record live GC references in these slots. We record
//! the incoming arguments in the callee's stack map. This choice plays nice
//! with tail calls, where by the time we transfer control to the callee, the
//! caller no longer exists.

use crate::ir;
use cranelift_bitset::CompoundBitSet;
use cranelift_entity::PrimaryMap;
use smallvec::SmallVec;

pub(crate) type UserStackMapEntryVec = SmallVec<[UserStackMapEntry; 4]>;

/// A stack map entry describes a single GC-managed value and its location on
/// the stack.
///
/// A stack map entry is associated with a particular instruction, and that
/// instruction must be a safepoint. The GC-managed value must be stored in the
/// described location across this entry's instruction.
#[derive(Clone, Debug, PartialEq, Hash)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct UserStackMapEntry {
    /// The type of the value stored in this stack map entry.
    pub ty: ir::Type,

    /// The stack slot that this stack map entry is within.
    pub slot: ir::StackSlot,

    /// The offset within the stack slot where this entry's value can be found.
    pub offset: u32,
}

/// A compiled stack map, describing the location of many GC-managed values.
///
/// A stack map is associated with a particular instruction, and that
/// instruction is a safepoint.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
pub struct UserStackMap {
    // Offsets into the frame's sized stack slots that are GC references, by type.
    by_type: SmallVec<[(ir::Type, CompoundBitSet); 1]>,

    // The offset of the sized stack slots, from SP, for this stack map's
    // associated PC.
    //
    // This is initially `None` upon construction during lowering, but filled in
    // after regalloc during emission when we have the precise frame layout.
    sp_to_sized_stack_slots: Option<u32>,
}

impl UserStackMap {
    /// Coalesce the given entries into a new `UserStackMap`.
    pub(crate) fn new(
        entries: &[UserStackMapEntry],
        stack_slot_offsets: &PrimaryMap<ir::StackSlot, u32>,
    ) -> Self {
        let mut by_type = SmallVec::<[(ir::Type, CompoundBitSet); 1]>::default();

        for entry in entries {
            let offset = stack_slot_offsets[entry.slot] + entry.offset;
            let offset = usize::try_from(offset).unwrap();

            // Don't bother trying to avoid an `O(n)` search here: `n` is
            // basically always one in practice; even if it isn't, there aren't
            // that many different CLIF types.
            let index = by_type
                .iter()
                .position(|(ty, _)| *ty == entry.ty)
                .unwrap_or_else(|| {
                    by_type.push((entry.ty, CompoundBitSet::with_capacity(offset + 1)));
                    by_type.len() - 1
                });

            by_type[index].1.insert(offset);
        }

        UserStackMap {
            by_type,
            sp_to_sized_stack_slots: None,
        }
    }

    /// Finalize this stack map by filling in the SP-to-stack-slots offset.
    pub(crate) fn finalize(&mut self, sp_to_sized_stack_slots: u32) {
        debug_assert!(self.sp_to_sized_stack_slots.is_none());
        self.sp_to_sized_stack_slots = Some(sp_to_sized_stack_slots);
    }

    /// Iterate over the entries in this stack map.
    ///
    /// Yields pairs of the type of GC reference that is at the offset, and the
    /// offset from SP. If a pair `(i64, 0x42)` is yielded, for example, then
    /// when execution is at this stack map's associated PC, `SP + 0x42` is a
    /// pointer to an `i64`, and that `i64` is a live GC reference.
    pub fn entries(&self) -> impl Iterator<Item = (ir::Type, u32)> + '_ {
        let sp_to_sized_stack_slots = self.sp_to_sized_stack_slots.expect(
            "`sp_to_sized_stack_slots` should have been filled in before this stack map was used",
        );
        self.by_type.iter().flat_map(move |(ty, bitset)| {
            bitset.iter().map(move |slot_offset| {
                (
                    *ty,
                    sp_to_sized_stack_slots + u32::try_from(slot_offset).unwrap(),
                )
            })
        })
    }
}
