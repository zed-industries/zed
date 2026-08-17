//! In-memory representation of compiled machine code, with labels and fixups to
//! refer to those labels. Handles constant-pool island insertion and also
//! veneer insertion for out-of-range jumps.
//!
//! This code exists to solve three problems:
//!
//! - Branch targets for forward branches are not known until later, when we
//!   emit code in a single pass through the instruction structs.
//!
//! - On many architectures, address references or offsets have limited range.
//!   For example, on AArch64, conditional branches can only target code +/- 1MB
//!   from the branch itself.
//!
//! - The lowering of control flow from the CFG-with-edges produced by
//!   [BlockLoweringOrder](super::BlockLoweringOrder), combined with many empty
//!   edge blocks when the register allocator does not need to insert any
//!   spills/reloads/moves in edge blocks, results in many suboptimal branch
//!   patterns. The lowering also pays no attention to block order, and so
//!   two-target conditional forms (cond-br followed by uncond-br) can often by
//!   avoided because one of the targets is the fallthrough. There are several
//!   cases here where we can simplify to use fewer branches.
//!
//! This "buffer" implements a single-pass code emission strategy (with a later
//! "fixup" pass, but only through recorded fixups, not all instructions). The
//! basic idea is:
//!
//! - Emit branches as they are, including two-target (cond/uncond) compound
//!   forms, but with zero offsets and optimistically assuming the target will be
//!   in range. Record the "fixup" for later. Targets are denoted instead by
//!   symbolic "labels" that are then bound to certain offsets in the buffer as
//!   we emit code. (Nominally, there is a label at the start of every basic
//!   block.)
//!
//! - As we do this, track the offset in the buffer at which the first label
//!   reference "goes out of range". We call this the "deadline". If we reach the
//!   deadline and we still have not bound the label to which an unresolved branch
//!   refers, we have a problem!
//!
//! - To solve this problem, we emit "islands" full of "veneers". An island is
//!   simply a chunk of code inserted in the middle of the code actually produced
//!   by the emitter (e.g., vcode iterating over instruction structs). The emitter
//!   has some awareness of this: it either asks for an island between blocks, so
//!   it is not accidentally executed, or else it emits a branch around the island
//!   when all other options fail (see `Inst::EmitIsland` meta-instruction).
//!
//! - A "veneer" is an instruction (or sequence of instructions) in an "island"
//!   that implements a longer-range reference to a label. The idea is that, for
//!   example, a branch with a limited range can branch to a "veneer" instead,
//!   which is simply a branch in a form that can use a longer-range reference. On
//!   AArch64, for example, conditionals have a +/- 1 MB range, but a conditional
//!   can branch to an unconditional branch which has a +/- 128 MB range. Hence, a
//!   conditional branch's label reference can be fixed up with a "veneer" to
//!   achieve a longer range.
//!
//! - To implement all of this, we require the backend to provide a `LabelUse`
//!   type that implements a trait. This is nominally an enum that records one of
//!   several kinds of references to an offset in code -- basically, a relocation
//!   type -- and will usually correspond to different instruction formats. The
//!   `LabelUse` implementation specifies the maximum range, how to patch in the
//!   actual label location when known, and how to generate a veneer to extend the
//!   range.
//!
//! That satisfies label references, but we still may have suboptimal branch
//! patterns. To clean up the branches, we do a simple "peephole"-style
//! optimization on the fly. To do so, the emitter (e.g., `Inst::emit()`)
//! informs the buffer of branches in the code and, in the case of conditionals,
//! the code that would have been emitted to invert this branch's condition. We
//! track the "latest branches": these are branches that are contiguous up to
//! the current offset. (If any code is emitted after a branch, that branch or
//! run of contiguous branches is no longer "latest".) The latest branches are
//! those that we can edit by simply truncating the buffer and doing something
//! else instead.
//!
//! To optimize branches, we implement several simple rules, and try to apply
//! them to the "latest branches" when possible:
//!
//! - A branch with a label target, when that label is bound to the ending
//!   offset of the branch (the fallthrough location), can be removed altogether,
//!   because the branch would have no effect).
//!
//! - An unconditional branch that starts at a label location, and branches to
//!   another label, results in a "label alias": all references to the label bound
//!   *to* this branch instruction are instead resolved to the *target* of the
//!   branch instruction. This effectively removes empty blocks that just
//!   unconditionally branch to the next block. We call this "branch threading".
//!
//! - A conditional followed by an unconditional, when the conditional branches
//!   to the unconditional's fallthrough, results in (i) the truncation of the
//!   unconditional, (ii) the inversion of the condition's condition, and (iii)
//!   replacement of the conditional's target (using the original target of the
//!   unconditional). This is a fancy way of saying "we can flip a two-target
//!   conditional branch's taken/not-taken targets if it works better with our
//!   fallthrough". To make this work, the emitter actually gives the buffer
//!   *both* forms of every conditional branch: the true form is emitted into the
//!   buffer, and the "inverted" machine-code bytes are provided as part of the
//!   branch-fixup metadata.
//!
//! - An unconditional B preceded by another unconditional P, when B's label(s) have
//!   been redirected to target(B), can be removed entirely. This is an extension
//!   of the branch-threading optimization, and is valid because if we know there
//!   will be no fallthrough into this branch instruction (the prior instruction
//!   is an unconditional jump), and if we know we have successfully redirected
//!   all labels, then this branch instruction is unreachable. Note that this
//!   works because the redirection happens before the label is ever resolved
//!   (fixups happen at island emission time, at which point latest-branches are
//!   cleared, or at the end of emission), so we are sure to catch and redirect
//!   all possible paths to this instruction.
//!
//! # Branch-optimization Correctness
//!
//! The branch-optimization mechanism depends on a few data structures with
//! invariants, which are always held outside the scope of top-level public
//! methods:
//!
//! - The latest-branches list. Each entry describes a span of the buffer
//!   (start/end offsets), the label target, the corresponding fixup-list entry
//!   index, and the bytes (must be the same length) for the inverted form, if
//!   conditional. The list of labels that are bound to the start-offset of this
//!   branch is *complete* (if any label has a resolved offset equal to `start`
//!   and is not an alias, it must appear in this list) and *precise* (no label
//!   in this list can be bound to another offset). No label in this list should
//!   be an alias.  No two branch ranges can overlap, and branches are in
//!   ascending-offset order.
//!
//! - The labels-at-tail list. This contains all MachLabels that have been bound
//!   to (whose resolved offsets are equal to) the tail offset of the buffer.
//!   No label in this list should be an alias.
//!
//! - The label_offsets array, containing the bound offset of a label or
//!   UNKNOWN. No label can be bound at an offset greater than the current
//!   buffer tail.
//!
//! - The label_aliases array, containing another label to which a label is
//!   bound or UNKNOWN. A label's resolved offset is the resolved offset
//!   of the label it is aliased to, if this is set.
//!
//! We argue below, at each method, how the invariants in these data structures
//! are maintained (grep for "Post-invariant").
//!
//! Given these invariants, we argue why each optimization preserves execution
//! semantics below (grep for "Preserves execution semantics").
//!
//! # Avoiding Quadratic Behavior
//!
//! There are two cases where we've had to take some care to avoid
//! quadratic worst-case behavior:
//!
//! - The "labels at this branch" list can grow unboundedly if the
//!   code generator binds many labels at one location. If the count
//!   gets too high (defined by the `LABEL_LIST_THRESHOLD` constant), we
//!   simply abort an optimization early in a way that is always correct
//!   but is conservative.
//!
//! - The fixup list can interact with island emission to create
//!   "quadratic island behavior". In a little more detail, one can hit
//!   this behavior by having some pending fixups (forward label
//!   references) with long-range label-use kinds, and some others
//!   with shorter-range references that nonetheless still are pending
//!   long enough to trigger island generation. In such a case, we
//!   process the fixup list, generate veneers to extend some forward
//!   references' ranges, but leave the other (longer-range) ones
//!   alone. The way this was implemented put them back on a list and
//!   resulted in quadratic behavior.
//!
//!   To avoid this fixups are split into two lists: one "pending" list and one
//!   final list. The pending list is kept around for handling fixups related to
//!   branches so it can be edited/truncated. When an island is reached, which
//!   starts processing fixups, all pending fixups are flushed into the final
//!   list. The final list is a `BinaryHeap` which enables fixup processing to
//!   only process those which are required during island emission, deferring
//!   all longer-range fixups to later.

use crate::binemit::{Addend, CodeOffset, Reloc};
use crate::ir::function::FunctionParameters;
use crate::ir::{ExceptionTag, ExternalName, RelSourceLoc, SourceLoc, TrapCode};
use crate::isa::unwind::UnwindInst;
use crate::machinst::{
    BlockIndex, MachInstLabelUse, TextSectionBuilder, VCodeConstant, VCodeConstants, VCodeInst,
};
use crate::trace;
use crate::{MachInstEmitState, ir};
use crate::{VCodeConstantData, timing};
use core::ops::Range;
use cranelift_control::ControlPlane;
use cranelift_entity::{PrimaryMap, entity_impl};
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem;
use std::string::String;
use std::vec::Vec;

#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "enable-serde")]
pub trait CompilePhase {
    type MachSrcLocType: for<'a> Deserialize<'a> + Serialize + core::fmt::Debug + PartialEq + Clone;
    type SourceLocType: for<'a> Deserialize<'a> + Serialize + core::fmt::Debug + PartialEq + Clone;
}

#[cfg(not(feature = "enable-serde"))]
pub trait CompilePhase {
    type MachSrcLocType: core::fmt::Debug + PartialEq + Clone;
    type SourceLocType: core::fmt::Debug + PartialEq + Clone;
}

/// Status of a compiled artifact that needs patching before being used.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Stencil;

/// Status of a compiled artifact ready to use.
#[derive(Clone, Debug, PartialEq)]
pub struct Final;

impl CompilePhase for Stencil {
    type MachSrcLocType = MachSrcLoc<Stencil>;
    type SourceLocType = RelSourceLoc;
}

impl CompilePhase for Final {
    type MachSrcLocType = MachSrcLoc<Final>;
    type SourceLocType = SourceLoc;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ForceVeneers {
    Yes,
    No,
}

/// A buffer of output to be produced, fixed up, and then emitted to a CodeSink
/// in bulk.
///
/// This struct uses `SmallVec`s to support small-ish function bodies without
/// any heap allocation. As such, it will be several kilobytes large. This is
/// likely fine as long as it is stack-allocated for function emission then
/// thrown away; but beware if many buffer objects are retained persistently.
pub struct MachBuffer<I: VCodeInst> {
    /// The buffer contents, as raw bytes.
    data: SmallVec<[u8; 1024]>,
    /// The required alignment of this buffer.
    min_alignment: u32,
    /// Any relocations referring to this code. Note that only *external*
    /// relocations are tracked here; references to labels within the buffer are
    /// resolved before emission.
    relocs: SmallVec<[MachReloc; 16]>,
    /// Any trap records referring to this code.
    traps: SmallVec<[MachTrap; 16]>,
    /// Any call site records referring to this code.
    call_sites: SmallVec<[MachCallSite; 16]>,
    /// Any exception-handler records referred to at call sites.
    exception_handlers: SmallVec<[MachExceptionHandler; 16]>,
    /// Any source location mappings referring to this code.
    srclocs: SmallVec<[MachSrcLoc<Stencil>; 64]>,
    /// Any user stack maps for this code.
    ///
    /// Each entry is an `(offset, span, stack_map)` triple. Entries are sorted
    /// by code offset, and each stack map covers `span` bytes on the stack.
    user_stack_maps: SmallVec<[(CodeOffset, u32, ir::UserStackMap); 8]>,
    /// Any unwind info at a given location.
    unwind_info: SmallVec<[(CodeOffset, UnwindInst); 8]>,
    /// The current source location in progress (after `start_srcloc()` and
    /// before `end_srcloc()`).  This is a (start_offset, src_loc) tuple.
    cur_srcloc: Option<(CodeOffset, RelSourceLoc)>,
    /// Known label offsets; `UNKNOWN_LABEL_OFFSET` if unknown.
    label_offsets: SmallVec<[CodeOffset; 16]>,
    /// Label aliases: when one label points to an unconditional jump, and that
    /// jump points to another label, we can redirect references to the first
    /// label immediately to the second.
    ///
    /// Invariant: we don't have label-alias cycles. We ensure this by,
    /// before setting label A to alias label B, resolving B's alias
    /// target (iteratively until a non-aliased label); if B is already
    /// aliased to A, then we cannot alias A back to B.
    label_aliases: SmallVec<[MachLabel; 16]>,
    /// Constants that must be emitted at some point.
    pending_constants: SmallVec<[VCodeConstant; 16]>,
    /// Byte size of all constants in `pending_constants`.
    pending_constants_size: CodeOffset,
    /// Traps that must be emitted at some point.
    pending_traps: SmallVec<[MachLabelTrap; 16]>,
    /// Fixups that haven't yet been flushed into `fixup_records` below and may
    /// be related to branches that are chomped. These all get added to
    /// `fixup_records` during island emission.
    pending_fixup_records: SmallVec<[MachLabelFixup<I>; 16]>,
    /// The nearest upcoming deadline for entries in `pending_fixup_records`.
    pending_fixup_deadline: CodeOffset,
    /// Fixups that must be performed after all code is emitted.
    fixup_records: BinaryHeap<MachLabelFixup<I>>,
    /// Latest branches, to facilitate in-place editing for better fallthrough
    /// behavior and empty-block removal.
    latest_branches: SmallVec<[MachBranch; 4]>,
    /// All labels at the current offset (emission tail). This is lazily
    /// cleared: it is actually accurate as long as the current offset is
    /// `labels_at_tail_off`, but if `cur_offset()` has grown larger, it should
    /// be considered as empty.
    ///
    /// For correctness, this *must* be complete (i.e., the vector must contain
    /// all labels whose offsets are resolved to the current tail), because we
    /// rely on it to update labels when we truncate branches.
    labels_at_tail: SmallVec<[MachLabel; 4]>,
    /// The last offset at which `labels_at_tail` is valid. It is conceptually
    /// always describing the tail of the buffer, but we do not clear
    /// `labels_at_tail` eagerly when the tail grows, rather we lazily clear it
    /// when the offset has grown past this (`labels_at_tail_off`) point.
    /// Always <= `cur_offset()`.
    labels_at_tail_off: CodeOffset,
    /// Metadata about all constants that this function has access to.
    ///
    /// This records the size/alignment of all constants (not the actual data)
    /// along with the last available label generated for the constant. This map
    /// is consulted when constants are referred to and the label assigned to a
    /// constant may change over time as well.
    constants: PrimaryMap<VCodeConstant, MachBufferConstant>,
    /// All recorded usages of constants as pairs of the constant and where the
    /// constant needs to be placed within `self.data`. Note that the same
    /// constant may appear in this array multiple times if it was emitted
    /// multiple times.
    used_constants: SmallVec<[(VCodeConstant, CodeOffset); 4]>,
    /// Indicates when a patchable region is currently open, to guard that it's
    /// not possible to nest patchable regions.
    open_patchable: bool,
}

impl MachBufferFinalized<Stencil> {
    /// Get a finalized machine buffer by applying the function's base source location.
    pub fn apply_base_srcloc(self, base_srcloc: SourceLoc) -> MachBufferFinalized<Final> {
        MachBufferFinalized {
            data: self.data,
            relocs: self.relocs,
            traps: self.traps,
            call_sites: self.call_sites,
            exception_handlers: self.exception_handlers,
            srclocs: self
                .srclocs
                .into_iter()
                .map(|srcloc| srcloc.apply_base_srcloc(base_srcloc))
                .collect(),
            user_stack_maps: self.user_stack_maps,
            unwind_info: self.unwind_info,
            alignment: self.alignment,
        }
    }
}

/// A `MachBuffer` once emission is completed: holds generated code and records,
/// without fixups. This allows the type to be independent of the backend.
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct MachBufferFinalized<T: CompilePhase> {
    /// The buffer contents, as raw bytes.
    pub(crate) data: SmallVec<[u8; 1024]>,
    /// Any relocations referring to this code. Note that only *external*
    /// relocations are tracked here; references to labels within the buffer are
    /// resolved before emission.
    pub(crate) relocs: SmallVec<[FinalizedMachReloc; 16]>,
    /// Any trap records referring to this code.
    pub(crate) traps: SmallVec<[MachTrap; 16]>,
    /// Any call site records referring to this code.
    pub(crate) call_sites: SmallVec<[MachCallSite; 16]>,
    /// Any exception-handler records referred to at call sites.
    pub(crate) exception_handlers: SmallVec<[FinalizedMachExceptionHandler; 16]>,
    /// Any source location mappings referring to this code.
    pub(crate) srclocs: SmallVec<[T::MachSrcLocType; 64]>,
    /// Any user stack maps for this code.
    ///
    /// Each entry is an `(offset, span, stack_map)` triple. Entries are sorted
    /// by code offset, and each stack map covers `span` bytes on the stack.
    pub(crate) user_stack_maps: SmallVec<[(CodeOffset, u32, ir::UserStackMap); 8]>,
    /// Any unwind info at a given location.
    pub unwind_info: SmallVec<[(CodeOffset, UnwindInst); 8]>,
    /// The required alignment of this buffer.
    pub alignment: u32,
}

const UNKNOWN_LABEL_OFFSET: CodeOffset = 0xffff_ffff;
const UNKNOWN_LABEL: MachLabel = MachLabel(0xffff_ffff);

/// Threshold on max length of `labels_at_this_branch` list to avoid
/// unbounded quadratic behavior (see comment below at use-site).
const LABEL_LIST_THRESHOLD: usize = 100;

/// A label refers to some offset in a `MachBuffer`. It may not be resolved at
/// the point at which it is used by emitted code; the buffer records "fixups"
/// for references to the label, and will come back and patch the code
/// appropriately when the label's location is eventually known.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MachLabel(u32);
entity_impl!(MachLabel);

impl MachLabel {
    /// Get a label for a block. (The first N MachLabels are always reserved for
    /// the N blocks in the vcode.)
    pub fn from_block(bindex: BlockIndex) -> MachLabel {
        MachLabel(bindex.index() as u32)
    }

    /// Creates a string representing this label, for convenience.
    pub fn to_string(&self) -> String {
        format!("label{}", self.0)
    }
}

impl Default for MachLabel {
    fn default() -> Self {
        UNKNOWN_LABEL
    }
}

/// Represents the beginning of an editable region in the [`MachBuffer`], while code emission is
/// still occurring. An [`OpenPatchRegion`] is closed by [`MachBuffer::end_patchable`], consuming
/// the [`OpenPatchRegion`] token in the process.
pub struct OpenPatchRegion(usize);

/// A region in the [`MachBuffer`] code buffer that can be edited prior to finalization. An example
/// of where you might want to use this is for patching instructions that mention constants that
/// won't be known until later: [`MachBuffer::start_patchable`] can be used to begin the patchable
/// region, instructions can be emitted with placeholder constants, and the [`PatchRegion`] token
/// can be produced by [`MachBuffer::end_patchable`]. Once the values of those constants are known,
/// the [`PatchRegion::patch`] function can be used to get a mutable buffer to the instruction
/// bytes, and the constants uses can be updated directly.
pub struct PatchRegion {
    range: Range<usize>,
}

impl PatchRegion {
    /// Consume the patch region to yield a mutable slice of the [`MachBuffer`] data buffer.
    pub fn patch<I: VCodeInst>(self, buffer: &mut MachBuffer<I>) -> &mut [u8] {
        &mut buffer.data[self.range]
    }
}

impl<I: VCodeInst> MachBuffer<I> {
    /// Create a new section, known to start at `start_offset` and with a size limited to
    /// `length_limit`.
    pub fn new() -> MachBuffer<I> {
        MachBuffer {
            data: SmallVec::new(),
            min_alignment: I::function_alignment().minimum,
            relocs: SmallVec::new(),
            traps: SmallVec::new(),
            call_sites: SmallVec::new(),
            exception_handlers: SmallVec::new(),
            srclocs: SmallVec::new(),
            user_stack_maps: SmallVec::new(),
            unwind_info: SmallVec::new(),
            cur_srcloc: None,
            label_offsets: SmallVec::new(),
            label_aliases: SmallVec::new(),
            pending_constants: SmallVec::new(),
            pending_constants_size: 0,
            pending_traps: SmallVec::new(),
            pending_fixup_records: SmallVec::new(),
            pending_fixup_deadline: u32::MAX,
            fixup_records: Default::default(),
            latest_branches: SmallVec::new(),
            labels_at_tail: SmallVec::new(),
            labels_at_tail_off: 0,
            constants: Default::default(),
            used_constants: Default::default(),
            open_patchable: false,
        }
    }

    /// Current offset from start of buffer.
    pub fn cur_offset(&self) -> CodeOffset {
        self.data.len() as CodeOffset
    }

    /// Add a byte.
    pub fn put1(&mut self, value: u8) {
        self.data.push(value);

        // Post-invariant: conceptual-labels_at_tail contains a complete and
        // precise list of labels bound at `cur_offset()`. We have advanced
        // `cur_offset()`, hence if it had been equal to `labels_at_tail_off`
        // before, it is not anymore (and it cannot become equal, because
        // `labels_at_tail_off` is always <= `cur_offset()`). Thus the list is
        // conceptually empty (even though it is only lazily cleared). No labels
        // can be bound at this new offset (by invariant on `label_offsets`).
        // Hence the invariant holds.
    }

    /// Add 2 bytes.
    pub fn put2(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.data.extend_from_slice(&bytes[..]);

        // Post-invariant: as for `put1()`.
    }

    /// Add 4 bytes.
    pub fn put4(&mut self, value: u32) {
        let bytes = value.to_le_bytes();
        self.data.extend_from_slice(&bytes[..]);

        // Post-invariant: as for `put1()`.
    }

    /// Add 8 bytes.
    pub fn put8(&mut self, value: u64) {
        let bytes = value.to_le_bytes();
        self.data.extend_from_slice(&bytes[..]);

        // Post-invariant: as for `put1()`.
    }

    /// Add a slice of bytes.
    pub fn put_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);

        // Post-invariant: as for `put1()`.
    }

    /// Reserve appended space and return a mutable slice referring to it.
    pub fn get_appended_space(&mut self, len: usize) -> &mut [u8] {
        let off = self.data.len();
        let new_len = self.data.len() + len;
        self.data.resize(new_len, 0);
        &mut self.data[off..]

        // Post-invariant: as for `put1()`.
    }

    /// Align up to the given alignment.
    pub fn align_to(&mut self, align_to: CodeOffset) {
        trace!("MachBuffer: align to {}", align_to);
        assert!(
            align_to.is_power_of_two(),
            "{align_to} is not a power of two"
        );
        while self.cur_offset() & (align_to - 1) != 0 {
            self.put1(0);
        }

        // Post-invariant: as for `put1()`.
    }

    /// Begin a region of patchable code. There is one requirement for the
    /// code that is emitted: It must not introduce any instructions that
    /// could be chomped (branches are an example of this). In other words,
    /// you must not call [`MachBuffer::add_cond_branch`] or
    /// [`MachBuffer::add_uncond_branch`] between calls to this method and
    /// [`MachBuffer::end_patchable`].
    pub fn start_patchable(&mut self) -> OpenPatchRegion {
        assert!(!self.open_patchable, "Patchable regions may not be nested");
        self.open_patchable = true;
        OpenPatchRegion(usize::try_from(self.cur_offset()).unwrap())
    }

    /// End a region of patchable code, yielding a [`PatchRegion`] value that
    /// can be consumed later to produce a one-off mutable slice to the
    /// associated region of the data buffer.
    pub fn end_patchable(&mut self, open: OpenPatchRegion) -> PatchRegion {
        // No need to assert the state of `open_patchable` here, as we take
        // ownership of the only `OpenPatchable` value.
        self.open_patchable = false;
        let end = usize::try_from(self.cur_offset()).unwrap();
        PatchRegion { range: open.0..end }
    }

    /// Allocate a `Label` to refer to some offset. May not be bound to a fixed
    /// offset yet.
    pub fn get_label(&mut self) -> MachLabel {
        let l = self.label_offsets.len() as u32;
        self.label_offsets.push(UNKNOWN_LABEL_OFFSET);
        self.label_aliases.push(UNKNOWN_LABEL);
        trace!("MachBuffer: new label -> {:?}", MachLabel(l));
        MachLabel(l)

        // Post-invariant: the only mutation is to add a new label; it has no
        // bound offset yet, so it trivially satisfies all invariants.
    }

    /// Reserve the first N MachLabels for blocks.
    pub fn reserve_labels_for_blocks(&mut self, blocks: usize) {
        trace!("MachBuffer: first {} labels are for blocks", blocks);
        debug_assert!(self.label_offsets.is_empty());
        self.label_offsets.resize(blocks, UNKNOWN_LABEL_OFFSET);
        self.label_aliases.resize(blocks, UNKNOWN_LABEL);

        // Post-invariant: as for `get_label()`.
    }

    /// Registers metadata in this `MachBuffer` about the `constants` provided.
    ///
    /// This will record the size/alignment of all constants which will prepare
    /// them for emission later on.
    pub fn register_constants(&mut self, constants: &VCodeConstants) {
        for (c, val) in constants.iter() {
            self.register_constant(&c, val);
        }
    }

    /// Similar to [`MachBuffer::register_constants`] but registers a
    /// single constant metadata. This function is useful in
    /// situations where not all constants are known at the time of
    /// emission.
    pub fn register_constant(&mut self, constant: &VCodeConstant, data: &VCodeConstantData) {
        let c2 = self.constants.push(MachBufferConstant {
            upcoming_label: None,
            align: data.alignment(),
            size: data.as_slice().len(),
        });
        assert_eq!(*constant, c2);
    }

    /// Completes constant emission by iterating over `self.used_constants` and
    /// filling in the "holes" with the constant values provided by `constants`.
    ///
    /// Returns the alignment required for this entire buffer. Alignment starts
    /// at the ISA's minimum function alignment and can be increased due to
    /// constant requirements.
    fn finish_constants(&mut self, constants: &VCodeConstants) -> u32 {
        let mut alignment = self.min_alignment;
        for (constant, offset) in mem::take(&mut self.used_constants) {
            let constant = constants.get(constant);
            let data = constant.as_slice();
            self.data[offset as usize..][..data.len()].copy_from_slice(data);
            alignment = constant.alignment().max(alignment);
        }
        alignment
    }

    /// Returns a label that can be used to refer to the `constant` provided.
    ///
    /// This will automatically defer a new constant to be emitted for
    /// `constant` if it has not been previously emitted. Note that this
    /// function may return a different label for the same constant at
    /// different points in time. The label is valid to use only from the
    /// current location; the MachBuffer takes care to emit the same constant
    /// multiple times if needed so the constant is always in range.
    pub fn get_label_for_constant(&mut self, constant: VCodeConstant) -> MachLabel {
        let MachBufferConstant {
            align,
            size,
            upcoming_label,
        } = self.constants[constant];
        if let Some(label) = upcoming_label {
            return label;
        }

        let label = self.get_label();
        trace!(
            "defer constant: eventually emit {size} bytes aligned \
             to {align} at label {label:?}",
        );
        self.pending_constants.push(constant);
        self.pending_constants_size += size as u32;
        self.constants[constant].upcoming_label = Some(label);
        label
    }

    /// Bind a label to the current offset. A label can only be bound once.
    pub fn bind_label(&mut self, label: MachLabel, ctrl_plane: &mut ControlPlane) {
        trace!(
            "MachBuffer: bind label {:?} at offset {}",
            label,
            self.cur_offset()
        );
        debug_assert_eq!(self.label_offsets[label.0 as usize], UNKNOWN_LABEL_OFFSET);
        debug_assert_eq!(self.label_aliases[label.0 as usize], UNKNOWN_LABEL);
        let offset = self.cur_offset();
        self.label_offsets[label.0 as usize] = offset;
        self.lazily_clear_labels_at_tail();
        self.labels_at_tail.push(label);

        // Invariants hold: bound offset of label is <= cur_offset (in fact it
        // is equal). If the `labels_at_tail` list was complete and precise
        // before, it is still, because we have bound this label to the current
        // offset and added it to the list (which contains all labels at the
        // current offset).

        self.optimize_branches(ctrl_plane);

        // Post-invariant: by `optimize_branches()` (see argument there).
    }

    /// Lazily clear `labels_at_tail` if the tail offset has moved beyond the
    /// offset that it applies to.
    fn lazily_clear_labels_at_tail(&mut self) {
        let offset = self.cur_offset();
        if offset > self.labels_at_tail_off {
            self.labels_at_tail_off = offset;
            self.labels_at_tail.clear();
        }

        // Post-invariant: either labels_at_tail_off was at cur_offset, and
        // state is untouched, or was less than cur_offset, in which case the
        // labels_at_tail list was conceptually empty, and is now actually
        // empty.
    }

    /// Resolve a label to an offset, if known. May return `UNKNOWN_LABEL_OFFSET`.
    pub(crate) fn resolve_label_offset(&self, mut label: MachLabel) -> CodeOffset {
        let mut iters = 0;
        while self.label_aliases[label.0 as usize] != UNKNOWN_LABEL {
            label = self.label_aliases[label.0 as usize];
            // To protect against an infinite loop (despite our assurances to
            // ourselves that the invariants make this impossible), assert out
            // after 1M iterations. The number of basic blocks is limited
            // in most contexts anyway so this should be impossible to hit with
            // a legitimate input.
            iters += 1;
            assert!(iters < 1_000_000, "Unexpected cycle in label aliases");
        }
        self.label_offsets[label.0 as usize]

        // Post-invariant: no mutations.
    }

    /// Emit a reference to the given label with the given reference type (i.e.,
    /// branch-instruction format) at the current offset.  This is like a
    /// relocation, but handled internally.
    ///
    /// This can be called before the branch is actually emitted; fixups will
    /// not happen until an island is emitted or the buffer is finished.
    pub fn use_label_at_offset(&mut self, offset: CodeOffset, label: MachLabel, kind: I::LabelUse) {
        trace!(
            "MachBuffer: use_label_at_offset: offset {} label {:?} kind {:?}",
            offset, label, kind
        );

        // Add the fixup, and update the worst-case island size based on a
        // veneer for this label use.
        let fixup = MachLabelFixup {
            label,
            offset,
            kind,
        };
        self.pending_fixup_deadline = self.pending_fixup_deadline.min(fixup.deadline());
        self.pending_fixup_records.push(fixup);

        // Post-invariant: no mutations to branches/labels data structures.
    }

    /// Inform the buffer of an unconditional branch at the given offset,
    /// targeting the given label. May be used to optimize branches.
    /// The last added label-use must correspond to this branch.
    /// This must be called when the current offset is equal to `start`; i.e.,
    /// before actually emitting the branch. This implies that for a branch that
    /// uses a label and is eligible for optimizations by the MachBuffer, the
    /// proper sequence is:
    ///
    /// - Call `use_label_at_offset()` to emit the fixup record.
    /// - Call `add_uncond_branch()` to make note of the branch.
    /// - Emit the bytes for the branch's machine code.
    ///
    /// Additional requirement: no labels may be bound between `start` and `end`
    /// (exclusive on both ends).
    pub fn add_uncond_branch(&mut self, start: CodeOffset, end: CodeOffset, target: MachLabel) {
        debug_assert!(
            !self.open_patchable,
            "Branch instruction inserted within a patchable region"
        );
        assert!(self.cur_offset() == start);
        debug_assert!(end > start);
        assert!(!self.pending_fixup_records.is_empty());
        let fixup = self.pending_fixup_records.len() - 1;
        self.lazily_clear_labels_at_tail();
        self.latest_branches.push(MachBranch {
            start,
            end,
            target,
            fixup,
            inverted: None,
            labels_at_this_branch: self.labels_at_tail.clone(),
        });

        // Post-invariant: we asserted branch start is current tail; the list of
        // labels at branch is cloned from list of labels at current tail.
    }

    /// Inform the buffer of a conditional branch at the given offset,
    /// targeting the given label. May be used to optimize branches.
    /// The last added label-use must correspond to this branch.
    ///
    /// Additional requirement: no labels may be bound between `start` and `end`
    /// (exclusive on both ends).
    pub fn add_cond_branch(
        &mut self,
        start: CodeOffset,
        end: CodeOffset,
        target: MachLabel,
        inverted: &[u8],
    ) {
        debug_assert!(
            !self.open_patchable,
            "Branch instruction inserted within a patchable region"
        );
        assert!(self.cur_offset() == start);
        debug_assert!(end > start);
        assert!(!self.pending_fixup_records.is_empty());
        debug_assert!(
            inverted.len() == (end - start) as usize,
            "branch length = {}, but inverted length = {}",
            end - start,
            inverted.len()
        );
        let fixup = self.pending_fixup_records.len() - 1;
        let inverted = Some(SmallVec::from(inverted));
        self.lazily_clear_labels_at_tail();
        self.latest_branches.push(MachBranch {
            start,
            end,
            target,
            fixup,
            inverted,
            labels_at_this_branch: self.labels_at_tail.clone(),
        });

        // Post-invariant: we asserted branch start is current tail; labels at
        // branch list is cloned from list of labels at current tail.
    }

    fn truncate_last_branch(&mut self) {
        debug_assert!(
            !self.open_patchable,
            "Branch instruction truncated within a patchable region"
        );

        self.lazily_clear_labels_at_tail();
        // Invariants hold at this point.

        let b = self.latest_branches.pop().unwrap();
        assert!(b.end == self.cur_offset());

        // State:
        //    [PRE CODE]
        //  Offset b.start, b.labels_at_this_branch:
        //    [BRANCH CODE]
        //  cur_off, self.labels_at_tail -->
        //    (end of buffer)
        self.data.truncate(b.start as usize);
        self.pending_fixup_records.truncate(b.fixup);
        while let Some(last_srcloc) = self.srclocs.last_mut() {
            if last_srcloc.end <= b.start {
                break;
            }
            if last_srcloc.start < b.start {
                last_srcloc.end = b.start;
                break;
            }
            self.srclocs.pop();
        }
        // State:
        //    [PRE CODE]
        //  cur_off, Offset b.start, b.labels_at_this_branch:
        //    (end of buffer)
        //
        //  self.labels_at_tail -->  (past end of buffer)
        let cur_off = self.cur_offset();
        self.labels_at_tail_off = cur_off;
        // State:
        //    [PRE CODE]
        //  cur_off, Offset b.start, b.labels_at_this_branch,
        //  self.labels_at_tail:
        //    (end of buffer)
        //
        // resolve_label_offset(l) for l in labels_at_tail:
        //    (past end of buffer)

        trace!(
            "truncate_last_branch: truncated {:?}; off now {}",
            b, cur_off
        );

        // Fix up resolved label offsets for labels at tail.
        for &l in &self.labels_at_tail {
            self.label_offsets[l.0 as usize] = cur_off;
        }
        // Old labels_at_this_branch are now at cur_off.
        self.labels_at_tail.extend(b.labels_at_this_branch);

        // Post-invariant: this operation is defined to truncate the buffer,
        // which moves cur_off backward, and to move labels at the end of the
        // buffer back to the start-of-branch offset.
        //
        // latest_branches satisfies all invariants:
        // - it has no branches past the end of the buffer (branches are in
        //   order, we removed the last one, and we truncated the buffer to just
        //   before the start of that branch)
        // - no labels were moved to lower offsets than the (new) cur_off, so
        //   the labels_at_this_branch list for any other branch need not change.
        //
        // labels_at_tail satisfies all invariants:
        // - all labels that were at the tail after the truncated branch are
        //   moved backward to just before the branch, which becomes the new tail;
        //   thus every element in the list should remain (ensured by `.extend()`
        //   above).
        // - all labels that refer to the new tail, which is the start-offset of
        //   the truncated branch, must be present. The `labels_at_this_branch`
        //   list in the truncated branch's record is a complete and precise list
        //   of exactly these labels; we append these to labels_at_tail.
        // - labels_at_tail_off is at cur_off after truncation occurs, so the
        //   list is valid (not to be lazily cleared).
        //
        // The stated operation was performed:
        // - For each label at the end of the buffer prior to this method, it
        //   now resolves to the new (truncated) end of the buffer: it must have
        //   been in `labels_at_tail` (this list is precise and complete, and
        //   the tail was at the end of the truncated branch on entry), and we
        //   iterate over this list and set `label_offsets` to the new tail.
        //   None of these labels could have been an alias (by invariant), so
        //   `label_offsets` is authoritative for each.
        // - No other labels will be past the end of the buffer, because of the
        //   requirement that no labels be bound to the middle of branch ranges
        //   (see comments to `add_{cond,uncond}_branch()`).
        // - The buffer is truncated to just before the last branch, and the
        //   fixup record referring to that last branch is removed.
    }

    /// Performs various optimizations on branches pointing at the current label.
    pub fn optimize_branches(&mut self, ctrl_plane: &mut ControlPlane) {
        if ctrl_plane.get_decision() {
            return;
        }

        self.lazily_clear_labels_at_tail();
        // Invariants valid at this point.

        trace!(
            "enter optimize_branches:\n b = {:?}\n l = {:?}\n f = {:?}",
            self.latest_branches, self.labels_at_tail, self.pending_fixup_records
        );

        // We continue to munch on branches at the tail of the buffer until no
        // more rules apply. Note that the loop only continues if a branch is
        // actually truncated (or if labels are redirected away from a branch),
        // so this always makes progress.
        while let Some(b) = self.latest_branches.last() {
            let cur_off = self.cur_offset();
            trace!("optimize_branches: last branch {:?} at off {}", b, cur_off);
            // If there has been any code emission since the end of the last branch or
            // label definition, then there's nothing we can edit (because we
            // don't move code once placed, only back up and overwrite), so
            // clear the records and finish.
            if b.end < cur_off {
                break;
            }

            // If the "labels at this branch" list on this branch is
            // longer than a threshold, don't do any simplification,
            // and let the branch remain to separate those labels from
            // the current tail. This avoids quadratic behavior (see
            // #3468): otherwise, if a long string of "goto next;
            // next:" patterns are emitted, all of the labels will
            // coalesce into a long list of aliases for the current
            // buffer tail. We must track all aliases of the current
            // tail for correctness, but we are also allowed to skip
            // optimization (removal) of any branch, so we take the
            // escape hatch here and let it stand. In effect this
            // "spreads" the many thousands of labels in the
            // pathological case among an actual (harmless but
            // suboptimal) instruction once per N labels.
            if b.labels_at_this_branch.len() > LABEL_LIST_THRESHOLD {
                break;
            }

            // Invariant: we are looking at a branch that ends at the tail of
            // the buffer.

            // For any branch, conditional or unconditional:
            // - If the target is a label at the current offset, then remove
            //   the conditional branch, and reset all labels that targeted
            //   the current offset (end of branch) to the truncated
            //   end-of-code.
            //
            // Preserves execution semantics: a branch to its own fallthrough
            // address is equivalent to a no-op; in both cases, nextPC is the
            // fallthrough.
            if self.resolve_label_offset(b.target) == cur_off {
                trace!("branch with target == cur off; truncating");
                self.truncate_last_branch();
                continue;
            }

            // If latest is an unconditional branch:
            //
            // - If the branch's target is not its own start address, then for
            //   each label at the start of branch, make the label an alias of the
            //   branch target, and remove the label from the "labels at this
            //   branch" list.
            //
            //   - Preserves execution semantics: an unconditional branch's
            //     only effect is to set PC to a new PC; this change simply
            //     collapses one step in the step-semantics.
            //
            //   - Post-invariant: the labels that were bound to the start of
            //     this branch become aliases, so they must not be present in any
            //     labels-at-this-branch list or the labels-at-tail list. The
            //     labels are removed form the latest-branch record's
            //     labels-at-this-branch list, and are never placed in the
            //     labels-at-tail list. Furthermore, it is correct that they are
            //     not in either list, because they are now aliases, and labels
            //     that are aliases remain aliases forever.
            //
            // - If there is a prior unconditional branch that ends just before
            //   this one begins, and this branch has no labels bound to its
            //   start, then we can truncate this branch, because it is entirely
            //   unreachable (we have redirected all labels that make it
            //   reachable otherwise). Do so and continue around the loop.
            //
            //   - Preserves execution semantics: the branch is unreachable,
            //     because execution can only flow into an instruction from the
            //     prior instruction's fallthrough or from a branch bound to that
            //     instruction's start offset. Unconditional branches have no
            //     fallthrough, so if the prior instruction is an unconditional
            //     branch, no fallthrough entry can happen. The
            //     labels-at-this-branch list is complete (by invariant), so if it
            //     is empty, then the instruction is entirely unreachable. Thus,
            //     it can be removed.
            //
            //   - Post-invariant: ensured by truncate_last_branch().
            //
            // - If there is a prior conditional branch whose target label
            //   resolves to the current offset (branches around the
            //   unconditional branch), then remove the unconditional branch,
            //   and make the target of the unconditional the target of the
            //   conditional instead.
            //
            //   - Preserves execution semantics: previously we had:
            //
            //         L1:
            //            cond_br L2
            //            br L3
            //         L2:
            //            (end of buffer)
            //
            //     by removing the last branch, we have:
            //
            //         L1:
            //            cond_br L2
            //         L2:
            //            (end of buffer)
            //
            //     we then fix up the records for the conditional branch to
            //     have:
            //
            //         L1:
            //           cond_br.inverted L3
            //         L2:
            //
            //     In the original code, control flow reaches L2 when the
            //     conditional branch's predicate is true, and L3 otherwise. In
            //     the optimized code, the same is true.
            //
            //   - Post-invariant: all edits to latest_branches and
            //     labels_at_tail are performed by `truncate_last_branch()`,
            //     which maintains the invariants at each step.

            if b.is_uncond() {
                // Set any label equal to current branch's start as an alias of
                // the branch's target, if the target is not the branch itself
                // (i.e., an infinite loop).
                //
                // We cannot perform this aliasing if the target of this branch
                // ultimately aliases back here; if so, we need to keep this
                // branch, so break out of this loop entirely (and clear the
                // latest-branches list below).
                //
                // Note that this check is what prevents cycles from forming in
                // `self.label_aliases`. To see why, consider an arbitrary start
                // state:
                //
                // label_aliases[L1] = L2, label_aliases[L2] = L3, ..., up to
                // Ln, which is not aliased.
                //
                // We would create a cycle if we assigned label_aliases[Ln]
                // = L1.  Note that the below assignment is the only write
                // to label_aliases.
                //
                // By our other invariants, we have that Ln (`l` below)
                // resolves to the offset `b.start`, because it is in the
                // set `b.labels_at_this_branch`.
                //
                // If L1 were already aliased, through some arbitrarily deep
                // chain, to Ln, then it must also resolve to this offset
                // `b.start`.
                //
                // By checking the resolution of `L1` against this offset,
                // and aborting this branch-simplification if they are
                // equal, we prevent the below assignment from ever creating
                // a cycle.
                if self.resolve_label_offset(b.target) != b.start {
                    let redirected = b.labels_at_this_branch.len();
                    for &l in &b.labels_at_this_branch {
                        trace!(
                            " -> label at start of branch {:?} redirected to target {:?}",
                            l, b.target
                        );
                        self.label_aliases[l.0 as usize] = b.target;
                        // NOTE: we continue to ensure the invariant that labels
                        // pointing to tail of buffer are in `labels_at_tail`
                        // because we already ensured above that the last branch
                        // cannot have a target of `cur_off`; so we never have
                        // to put the label into `labels_at_tail` when moving it
                        // here.
                    }
                    // Maintain invariant: all branches have been redirected
                    // and are no longer pointing at the start of this branch.
                    let mut_b = self.latest_branches.last_mut().unwrap();
                    mut_b.labels_at_this_branch.clear();

                    if redirected > 0 {
                        trace!(" -> after label redirects, restarting loop");
                        continue;
                    }
                } else {
                    break;
                }

                let b = self.latest_branches.last().unwrap();

                // Examine any immediately preceding branch.
                if self.latest_branches.len() > 1 {
                    let prev_b = &self.latest_branches[self.latest_branches.len() - 2];
                    trace!(" -> more than one branch; prev_b = {:?}", prev_b);
                    // This uncond is immediately after another uncond; we
                    // should have already redirected labels to this uncond away
                    // (but check to be sure); so we can truncate this uncond.
                    if prev_b.is_uncond()
                        && prev_b.end == b.start
                        && b.labels_at_this_branch.is_empty()
                    {
                        trace!(" -> uncond follows another uncond; truncating");
                        self.truncate_last_branch();
                        continue;
                    }

                    // This uncond is immediately after a conditional, and the
                    // conditional's target is the end of this uncond, and we've
                    // already redirected labels to this uncond away; so we can
                    // truncate this uncond, flip the sense of the conditional, and
                    // set the conditional's target (in `latest_branches` and in
                    // `fixup_records`) to the uncond's target.
                    if prev_b.is_cond()
                        && prev_b.end == b.start
                        && self.resolve_label_offset(prev_b.target) == cur_off
                    {
                        trace!(
                            " -> uncond follows a conditional, and conditional's target resolves to current offset"
                        );
                        // Save the target of the uncond (this becomes the
                        // target of the cond), and truncate the uncond.
                        let target = b.target;
                        let data = prev_b.inverted.clone().unwrap();
                        self.truncate_last_branch();

                        // Mutate the code and cond branch.
                        let off_before_edit = self.cur_offset();
                        let prev_b = self.latest_branches.last_mut().unwrap();
                        let not_inverted = SmallVec::from(
                            &self.data[(prev_b.start as usize)..(prev_b.end as usize)],
                        );

                        // Low-level edit: replaces bytes of branch with
                        // inverted form. cur_off remains the same afterward, so
                        // we do not need to modify label data structures.
                        self.data.truncate(prev_b.start as usize);
                        self.data.extend_from_slice(&data[..]);

                        // Save the original code as the inversion of the
                        // inverted branch, in case we later edit this branch
                        // again.
                        prev_b.inverted = Some(not_inverted);
                        self.pending_fixup_records[prev_b.fixup].label = target;
                        trace!(" -> reassigning target of condbr to {:?}", target);
                        prev_b.target = target;
                        debug_assert_eq!(off_before_edit, self.cur_offset());
                        continue;
                    }
                }
            }

            // If we couldn't do anything with the last branch, then break.
            break;
        }

        self.purge_latest_branches();

        trace!(
            "leave optimize_branches:\n b = {:?}\n l = {:?}\n f = {:?}",
            self.latest_branches, self.labels_at_tail, self.pending_fixup_records
        );
    }

    fn purge_latest_branches(&mut self) {
        // All of our branch simplification rules work only if a branch ends at
        // the tail of the buffer, with no following code; and branches are in
        // order in latest_branches; so if the last entry ends prior to
        // cur_offset, then clear all entries.
        let cur_off = self.cur_offset();
        if let Some(l) = self.latest_branches.last() {
            if l.end < cur_off {
                trace!("purge_latest_branches: removing branch {:?}", l);
                self.latest_branches.clear();
            }
        }

        // Post-invariant: no invariant requires any branch to appear in
        // `latest_branches`; it is always optional. The list-clear above thus
        // preserves all semantics.
    }

    /// Emit a trap at some point in the future with the specified code and
    /// stack map.
    ///
    /// This function returns a [`MachLabel`] which will be the future address
    /// of the trap. Jumps should refer to this label, likely by using the
    /// [`MachBuffer::use_label_at_offset`] method, to get a relocation
    /// patched in once the address of the trap is known.
    ///
    /// This will batch all traps into the end of the function.
    pub fn defer_trap(&mut self, code: TrapCode) -> MachLabel {
        let label = self.get_label();
        self.pending_traps.push(MachLabelTrap {
            label,
            code,
            loc: self.cur_srcloc.map(|(_start, loc)| loc),
        });
        label
    }

    /// Is an island needed within the next N bytes?
    pub fn island_needed(&self, distance: CodeOffset) -> bool {
        let deadline = match self.fixup_records.peek() {
            Some(fixup) => fixup.deadline().min(self.pending_fixup_deadline),
            None => self.pending_fixup_deadline,
        };
        deadline < u32::MAX && self.worst_case_end_of_island(distance) > deadline
    }

    /// Returns the maximal offset that islands can reach if `distance` more
    /// bytes are appended.
    ///
    /// This is used to determine if veneers need insertions since jumps that
    /// can't reach past this point must get a veneer of some form.
    fn worst_case_end_of_island(&self, distance: CodeOffset) -> CodeOffset {
        // Assume that all fixups will require veneers and that the veneers are
        // the worst-case size for each platform. This is an over-generalization
        // to avoid iterating over the `fixup_records` list or maintaining
        // information about it as we go along.
        let island_worst_case_size = ((self.fixup_records.len() + self.pending_fixup_records.len())
            as u32)
            * (I::LabelUse::worst_case_veneer_size())
            + self.pending_constants_size
            + (self.pending_traps.len() * I::TRAP_OPCODE.len()) as u32;
        self.cur_offset()
            .saturating_add(distance)
            .saturating_add(island_worst_case_size)
    }

    /// Emit all pending constants and required pending veneers.
    ///
    /// Should only be called if `island_needed()` returns true, i.e., if we
    /// actually reach a deadline. It's not necessarily a problem to do so
    /// otherwise but it may result in unnecessary work during emission.
    pub fn emit_island(&mut self, distance: CodeOffset, ctrl_plane: &mut ControlPlane) {
        self.emit_island_maybe_forced(ForceVeneers::No, distance, ctrl_plane);
    }

    /// Same as `emit_island`, but an internal API with a `force_veneers`
    /// argument to force all veneers to always get emitted for debugging.
    fn emit_island_maybe_forced(
        &mut self,
        force_veneers: ForceVeneers,
        distance: CodeOffset,
        ctrl_plane: &mut ControlPlane,
    ) {
        // We're going to purge fixups, so no latest-branch editing can happen
        // anymore.
        self.latest_branches.clear();

        // End the current location tracking since anything emitted during this
        // function shouldn't be attributed to whatever the current source
        // location is.
        //
        // Note that the current source location, if it's set right now, will be
        // restored at the end of this island emission.
        let cur_loc = self.cur_srcloc.map(|(_, loc)| loc);
        if cur_loc.is_some() {
            self.end_srcloc();
        }

        let forced_threshold = self.worst_case_end_of_island(distance);

        // First flush out all traps/constants so we have more labels in case
        // fixups are applied against these labels.
        //
        // Note that traps are placed first since this typically happens at the
        // end of the function and for disassemblers we try to keep all the code
        // contiguously together.
        for MachLabelTrap { label, code, loc } in mem::take(&mut self.pending_traps) {
            // If this trap has source information associated with it then
            // emit this information for the trap instruction going out now too.
            if let Some(loc) = loc {
                self.start_srcloc(loc);
            }
            self.align_to(I::LabelUse::ALIGN);
            self.bind_label(label, ctrl_plane);
            self.add_trap(code);
            self.put_data(I::TRAP_OPCODE);
            if loc.is_some() {
                self.end_srcloc();
            }
        }

        for constant in mem::take(&mut self.pending_constants) {
            let MachBufferConstant { align, size, .. } = self.constants[constant];
            let label = self.constants[constant].upcoming_label.take().unwrap();
            self.align_to(align);
            self.bind_label(label, ctrl_plane);
            self.used_constants.push((constant, self.cur_offset()));
            self.get_appended_space(size);
        }

        // Either handle all pending fixups because they're ready or move them
        // onto the `BinaryHeap` tracking all pending fixups if they aren't
        // ready.
        assert!(self.latest_branches.is_empty());
        for fixup in mem::take(&mut self.pending_fixup_records) {
            if self.should_apply_fixup(&fixup, forced_threshold) {
                self.handle_fixup(fixup, force_veneers, forced_threshold);
            } else {
                self.fixup_records.push(fixup);
            }
        }
        self.pending_fixup_deadline = u32::MAX;
        while let Some(fixup) = self.fixup_records.peek() {
            trace!("emit_island: fixup {:?}", fixup);

            // If this fixup shouldn't be applied, that means its label isn't
            // defined yet and there'll be remaining space to apply a veneer if
            // necessary in the future after this island. In that situation
            // because `fixup_records` is sorted by deadline this loop can
            // exit.
            if !self.should_apply_fixup(fixup, forced_threshold) {
                break;
            }

            let fixup = self.fixup_records.pop().unwrap();
            self.handle_fixup(fixup, force_veneers, forced_threshold);
        }

        if let Some(loc) = cur_loc {
            self.start_srcloc(loc);
        }
    }

    fn should_apply_fixup(&self, fixup: &MachLabelFixup<I>, forced_threshold: CodeOffset) -> bool {
        let label_offset = self.resolve_label_offset(fixup.label);
        label_offset != UNKNOWN_LABEL_OFFSET || fixup.deadline() < forced_threshold
    }

    fn handle_fixup(
        &mut self,
        fixup: MachLabelFixup<I>,
        force_veneers: ForceVeneers,
        forced_threshold: CodeOffset,
    ) {
        let MachLabelFixup {
            label,
            offset,
            kind,
        } = fixup;
        let start = offset as usize;
        let end = (offset + kind.patch_size()) as usize;
        let label_offset = self.resolve_label_offset(label);

        if label_offset != UNKNOWN_LABEL_OFFSET {
            // If the offset of the label for this fixup is known then
            // we're going to do something here-and-now. We're either going
            // to patch the original offset because it's an in-bounds jump,
            // or we're going to generate a veneer, patch the fixup to jump
            // to the veneer, and then keep going.
            //
            // If the label comes after the original fixup, then we should
            // be guaranteed that the jump is in-bounds. Otherwise there's
            // a bug somewhere because this method wasn't called soon
            // enough. All forward-jumps are tracked and should get veneers
            // before their deadline comes and they're unable to jump
            // further.
            //
            // Otherwise if the label is before the fixup, then that's a
            // backwards jump. If it's past the maximum negative range
            // then we'll emit a veneer that to jump forward to which can
            // then jump backwards.
            let veneer_required = if label_offset >= offset {
                assert!((label_offset - offset) <= kind.max_pos_range());
                false
            } else {
                (offset - label_offset) > kind.max_neg_range()
            };
            trace!(
                " -> label_offset = {}, known, required = {} (pos {} neg {})",
                label_offset,
                veneer_required,
                kind.max_pos_range(),
                kind.max_neg_range()
            );

            if (force_veneers == ForceVeneers::Yes && kind.supports_veneer()) || veneer_required {
                self.emit_veneer(label, offset, kind);
            } else {
                let slice = &mut self.data[start..end];
                trace!(
                    "patching in-range! slice = {slice:?}; offset = {offset:#x}; label_offset = {label_offset:#x}"
                );
                kind.patch(slice, offset, label_offset);
            }
        } else {
            // If the offset of this label is not known at this time then
            // that means that a veneer is required because after this
            // island the target can't be in range of the original target.
            assert!(forced_threshold - offset > kind.max_pos_range());
            self.emit_veneer(label, offset, kind);
        }
    }

    /// Emits a "veneer" the `kind` code at `offset` to jump to `label`.
    ///
    /// This will generate extra machine code, using `kind`, to get a
    /// larger-jump-kind than `kind` allows. The code at `offset` is then
    /// patched to jump to our new code, and then the new code is enqueued for
    /// a fixup to get processed at some later time.
    fn emit_veneer(&mut self, label: MachLabel, offset: CodeOffset, kind: I::LabelUse) {
        // If this `kind` doesn't support a veneer then that's a bug in the
        // backend because we need to implement support for such a veneer.
        assert!(
            kind.supports_veneer(),
            "jump beyond the range of {kind:?} but a veneer isn't supported",
        );

        // Allocate space for a veneer in the island.
        self.align_to(I::LabelUse::ALIGN);
        let veneer_offset = self.cur_offset();
        trace!("making a veneer at {}", veneer_offset);
        let start = offset as usize;
        let end = (offset + kind.patch_size()) as usize;
        let slice = &mut self.data[start..end];
        // Patch the original label use to refer to the veneer.
        trace!(
            "patching original at offset {} to veneer offset {}",
            offset, veneer_offset
        );
        kind.patch(slice, offset, veneer_offset);
        // Generate the veneer.
        let veneer_slice = self.get_appended_space(kind.veneer_size() as usize);
        let (veneer_fixup_off, veneer_label_use) =
            kind.generate_veneer(veneer_slice, veneer_offset);
        trace!(
            "generated veneer; fixup offset {}, label_use {:?}",
            veneer_fixup_off, veneer_label_use
        );
        // Register a new use of `label` with our new veneer fixup and
        // offset. This'll recalculate deadlines accordingly and
        // enqueue this fixup to get processed at some later
        // time.
        self.use_label_at_offset(veneer_fixup_off, label, veneer_label_use);
    }

    fn finish_emission_maybe_forcing_veneers(
        &mut self,
        force_veneers: ForceVeneers,
        ctrl_plane: &mut ControlPlane,
    ) {
        while !self.pending_constants.is_empty()
            || !self.pending_traps.is_empty()
            || !self.fixup_records.is_empty()
            || !self.pending_fixup_records.is_empty()
        {
            // `emit_island()` will emit any pending veneers and constants, and
            // as a side-effect, will also take care of any fixups with resolved
            // labels eagerly.
            self.emit_island_maybe_forced(force_veneers, u32::MAX, ctrl_plane);
        }

        // Ensure that all labels have been fixed up after the last island is emitted. This is a
        // full (release-mode) assert because an unresolved label means the emitted code is
        // incorrect.
        assert!(self.fixup_records.is_empty());
        assert!(self.pending_fixup_records.is_empty());
    }

    /// Finish any deferred emissions and/or fixups.
    pub fn finish(
        mut self,
        constants: &VCodeConstants,
        ctrl_plane: &mut ControlPlane,
    ) -> MachBufferFinalized<Stencil> {
        let _tt = timing::vcode_emit_finish();

        self.finish_emission_maybe_forcing_veneers(ForceVeneers::No, ctrl_plane);

        let alignment = self.finish_constants(constants);

        // Resolve all labels to their offsets.
        let finalized_relocs = self
            .relocs
            .iter()
            .map(|reloc| FinalizedMachReloc {
                offset: reloc.offset,
                kind: reloc.kind,
                addend: reloc.addend,
                target: match &reloc.target {
                    RelocTarget::ExternalName(name) => {
                        FinalizedRelocTarget::ExternalName(name.clone())
                    }
                    RelocTarget::Label(label) => {
                        FinalizedRelocTarget::Func(self.resolve_label_offset(*label))
                    }
                },
            })
            .collect();

        let finalized_exception_handlers = self
            .exception_handlers
            .iter()
            .map(|handler| handler.finalize(|label| self.resolve_label_offset(label)))
            .collect();

        let mut srclocs = self.srclocs;
        srclocs.sort_by_key(|entry| entry.start);

        MachBufferFinalized {
            data: self.data,
            relocs: finalized_relocs,
            traps: self.traps,
            call_sites: self.call_sites,
            exception_handlers: finalized_exception_handlers,
            srclocs,
            user_stack_maps: self.user_stack_maps,
            unwind_info: self.unwind_info,
            alignment,
        }
    }

    /// Add an external relocation at the given offset.
    pub fn add_reloc_at_offset<T: Into<RelocTarget> + Clone>(
        &mut self,
        offset: CodeOffset,
        kind: Reloc,
        target: &T,
        addend: Addend,
    ) {
        let target: RelocTarget = target.clone().into();
        // FIXME(#3277): This should use `I::LabelUse::from_reloc` to optionally
        // generate a label-use statement to track whether an island is possibly
        // needed to escape this function to actually get to the external name.
        // This is most likely to come up on AArch64 where calls between
        // functions use a 26-bit signed offset which gives +/- 64MB. This means
        // that if a function is 128MB in size and there's a call in the middle
        // it's impossible to reach the actual target. Also, while it's
        // technically possible to jump to the start of a function and then jump
        // further, island insertion below always inserts islands after
        // previously appended code so for Cranelift's own implementation this
        // is also a problem for 64MB functions on AArch64 which start with a
        // call instruction, those won't be able to escape.
        //
        // Ideally what needs to happen here is that a `LabelUse` is
        // transparently generated (or call-sites of this function are audited
        // to generate a `LabelUse` instead) and tracked internally. The actual
        // relocation would then change over time if and when a veneer is
        // inserted, where the relocation here would be patched by this
        // `MachBuffer` to jump to the veneer. The problem, though, is that all
        // this still needs to end up, in the case of a singular function,
        // generating a final relocation pointing either to this particular
        // relocation or to the veneer inserted. Additionally
        // `MachBuffer` needs the concept of a label which will never be
        // resolved, so `emit_island` doesn't trip over not actually ever
        // knowning what some labels are. Currently the loop in
        // `finish_emission_maybe_forcing_veneers` would otherwise infinitely
        // loop.
        //
        // For now this means that because relocs aren't tracked at all that
        // AArch64 functions have a rough size limits of 64MB. For now that's
        // somewhat reasonable and the failure mode is a panic in `MachBuffer`
        // when a relocation can't otherwise be resolved later, so it shouldn't
        // actually result in any memory unsafety or anything like that.
        self.relocs.push(MachReloc {
            offset,
            kind,
            target,
            addend,
        });
    }

    /// Add an external relocation at the current offset.
    pub fn add_reloc<T: Into<RelocTarget> + Clone>(
        &mut self,
        kind: Reloc,
        target: &T,
        addend: Addend,
    ) {
        self.add_reloc_at_offset(self.data.len() as CodeOffset, kind, target, addend);
    }

    /// Add a trap record at the current offset.
    pub fn add_trap(&mut self, code: TrapCode) {
        self.traps.push(MachTrap {
            offset: self.data.len() as CodeOffset,
            code,
        });
    }

    /// Add a call-site record at the current offset.
    pub fn add_call_site(&mut self) {
        self.add_try_call_site(core::iter::empty());
    }

    /// Add a call-site record at the current offset with exception
    /// handlers.
    pub fn add_try_call_site(
        &mut self,
        exception_handlers: impl Iterator<Item = MachExceptionHandler>,
    ) {
        let start = u32::try_from(self.exception_handlers.len()).unwrap();
        self.exception_handlers.extend(exception_handlers);
        let end = u32::try_from(self.exception_handlers.len()).unwrap();
        let exception_handler_range = start..end;

        self.call_sites.push(MachCallSite {
            ret_addr: self.data.len() as CodeOffset,
            exception_handler_range,
        });
    }

    /// Add an unwind record at the current offset.
    pub fn add_unwind(&mut self, unwind: UnwindInst) {
        self.unwind_info.push((self.cur_offset(), unwind));
    }

    /// Set the `SourceLoc` for code from this offset until the offset at the
    /// next call to `end_srcloc()`.
    /// Returns the current [CodeOffset] and [RelSourceLoc].
    pub fn start_srcloc(&mut self, loc: RelSourceLoc) -> (CodeOffset, RelSourceLoc) {
        let cur = (self.cur_offset(), loc);
        self.cur_srcloc = Some(cur);
        cur
    }

    /// Mark the end of the `SourceLoc` segment started at the last
    /// `start_srcloc()` call.
    pub fn end_srcloc(&mut self) {
        let (start, loc) = self
            .cur_srcloc
            .take()
            .expect("end_srcloc() called without start_srcloc()");
        let end = self.cur_offset();
        // Skip zero-length extends.
        debug_assert!(end >= start);
        if end > start {
            self.srclocs.push(MachSrcLoc { start, end, loc });
        }
    }

    /// Push a user stack map onto this buffer.
    ///
    /// The stack map is associated with the given `return_addr` code
    /// offset. This must be the PC for the instruction just *after* this stack
    /// map's associated instruction. For example in the sequence `call $foo;
    /// add r8, rax`, the `return_addr` must be the offset of the start of the
    /// `add` instruction.
    ///
    /// Stack maps must be pushed in sorted `return_addr` order.
    pub fn push_user_stack_map(
        &mut self,
        emit_state: &I::State,
        return_addr: CodeOffset,
        mut stack_map: ir::UserStackMap,
    ) {
        let span = emit_state.frame_layout().active_size();
        trace!("Adding user stack map @ {return_addr:#x} spanning {span} bytes: {stack_map:?}");

        debug_assert!(
            self.user_stack_maps
                .last()
                .map_or(true, |(prev_addr, _, _)| *prev_addr < return_addr),
            "pushed stack maps out of order: {} is not less than {}",
            self.user_stack_maps.last().unwrap().0,
            return_addr,
        );

        stack_map.finalize(emit_state.frame_layout().sp_to_sized_stack_slots());
        self.user_stack_maps.push((return_addr, span, stack_map));
    }

    /// Increase the alignment of the buffer to the given alignment if bigger
    /// than the current alignment.
    pub fn set_log2_min_function_alignment(&mut self, align_to: u8) {
        self.min_alignment = self.min_alignment.max(
            1u32.checked_shl(u32::from(align_to))
                .expect("log2_min_function_alignment too large"),
        );
    }
}

impl<I: VCodeInst> Extend<u8> for MachBuffer<I> {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        for b in iter {
            self.put1(b);
        }
    }
}

impl<T: CompilePhase> MachBufferFinalized<T> {
    /// Get a list of source location mapping tuples in sorted-by-start-offset order.
    pub fn get_srclocs_sorted(&self) -> &[T::MachSrcLocType] {
        &self.srclocs[..]
    }

    /// Get the total required size for the code.
    pub fn total_size(&self) -> CodeOffset {
        self.data.len() as CodeOffset
    }

    /// Return the code in this mach buffer as a hex string for testing purposes.
    pub fn stringify_code_bytes(&self) -> String {
        // This is pretty lame, but whatever ..
        use std::fmt::Write;
        let mut s = String::with_capacity(self.data.len() * 2);
        for b in &self.data {
            write!(&mut s, "{b:02X}").unwrap();
        }
        s
    }

    /// Get the code bytes.
    pub fn data(&self) -> &[u8] {
        // N.B.: we emit every section into the .text section as far as
        // the `CodeSink` is concerned; we do not bother to segregate
        // the contents into the actual program text, the jumptable and the
        // rodata (constant pool). This allows us to generate code assuming
        // that these will not be relocated relative to each other, and avoids
        // having to designate each section as belonging in one of the three
        // fixed categories defined by `CodeSink`. If this becomes a problem
        // later (e.g. because of memory permissions or similar), we can
        // add this designation and segregate the output; take care, however,
        // to add the appropriate relocations in this case.

        &self.data[..]
    }

    /// Get the list of external relocations for this code.
    pub fn relocs(&self) -> &[FinalizedMachReloc] {
        &self.relocs[..]
    }

    /// Get the list of trap records for this code.
    pub fn traps(&self) -> &[MachTrap] {
        &self.traps[..]
    }

    /// Get the user stack map metadata for this code.
    pub fn user_stack_maps(&self) -> &[(CodeOffset, u32, ir::UserStackMap)] {
        &self.user_stack_maps
    }

    /// Take this buffer's user strack map metadata.
    pub fn take_user_stack_maps(&mut self) -> SmallVec<[(CodeOffset, u32, ir::UserStackMap); 8]> {
        mem::take(&mut self.user_stack_maps)
    }

    /// Get the list of call sites for this code, along with
    /// associated exception handlers.
    ///
    /// Each item yielded by the returned iterator is a struct with:
    ///
    /// - The call site metadata record, with a `ret_addr` field
    ///   directly accessible and denoting the offset of the return
    ///   address into this buffer's code.
    /// - The slice of pairs of exception tags and code offsets
    ///   denoting exception-handler entry points associated with this
    ///   call site.
    pub fn call_sites(&self) -> impl Iterator<Item = FinalizedMachCallSite<'_>> + '_ {
        self.call_sites.iter().map(|call_site| {
            let handler_range = call_site.exception_handler_range.clone();
            let handler_range = usize::try_from(handler_range.start).unwrap()
                ..usize::try_from(handler_range.end).unwrap();
            FinalizedMachCallSite {
                ret_addr: call_site.ret_addr,
                exception_handlers: &self.exception_handlers[handler_range],
            }
        })
    }
}

/// An item in the exception-handler list for a callsite, with label
/// references.  Items are interpreted in left-to-right order and the
/// first match wins.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MachExceptionHandler {
    /// A specific tag (in the current dynamic context) should be
    /// handled by the code at the given offset.
    Tag(ExceptionTag, MachLabel),
    /// All exceptions should be handled by the code at the given
    /// offset.
    Default(MachLabel),
    /// The dynamic context for interpreting tags is updated to the
    /// value stored in the given machine location (in this frame's
    /// context).
    Context(ExceptionContextLoc),
}

impl MachExceptionHandler {
    fn finalize<F: Fn(MachLabel) -> CodeOffset>(self, f: F) -> FinalizedMachExceptionHandler {
        match self {
            Self::Tag(tag, label) => FinalizedMachExceptionHandler::Tag(tag, f(label)),
            Self::Default(label) => FinalizedMachExceptionHandler::Default(f(label)),
            Self::Context(loc) => FinalizedMachExceptionHandler::Context(loc),
        }
    }
}

/// An item in the exception-handler list for a callsite, with final
/// (lowered) code offsets. Items are interpreted in left-to-right
/// order and the first match wins.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum FinalizedMachExceptionHandler {
    /// A specific tag (in the current dynamic context) should be
    /// handled by the code at the given offset.
    Tag(ExceptionTag, CodeOffset),
    /// All exceptions should be handled by the code at the given
    /// offset.
    Default(CodeOffset),
    /// The dynamic context for interpreting tags is updated to the
    /// value stored in the given machine location (in this frame's
    /// context).
    Context(ExceptionContextLoc),
}

/// A location for a dynamic exception context value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum ExceptionContextLoc {
    /// An offset from SP at the callsite.
    SPOffset(u32),
    /// A GPR at the callsite. The physical register number for the
    /// GPR register file on the target architecture is used.
    GPR(u8),
}

/// Metadata about a constant.
struct MachBufferConstant {
    /// A label which has not yet been bound which can be used for this
    /// constant.
    ///
    /// This is lazily created when a label is requested for a constant and is
    /// cleared when a constant is emitted.
    upcoming_label: Option<MachLabel>,
    /// Required alignment.
    align: CodeOffset,
    /// The byte size of this constant.
    size: usize,
}

/// A trap that is deferred to the next time an island is emitted for either
/// traps, constants, or fixups.
struct MachLabelTrap {
    /// This label will refer to the trap's offset.
    label: MachLabel,
    /// The code associated with this trap.
    code: TrapCode,
    /// An optional source location to assign for this trap.
    loc: Option<RelSourceLoc>,
}

/// A fixup to perform on the buffer once code is emitted. Fixups always refer
/// to labels and patch the code based on label offsets. Hence, they are like
/// relocations, but internal to one buffer.
#[derive(Debug)]
struct MachLabelFixup<I: VCodeInst> {
    /// The label whose offset controls this fixup.
    label: MachLabel,
    /// The offset to fix up / patch to refer to this label.
    offset: CodeOffset,
    /// The kind of fixup. This is architecture-specific; each architecture may have,
    /// e.g., several types of branch instructions, each with differently-sized
    /// offset fields and different places within the instruction to place the
    /// bits.
    kind: I::LabelUse,
}

impl<I: VCodeInst> MachLabelFixup<I> {
    fn deadline(&self) -> CodeOffset {
        self.offset.saturating_add(self.kind.max_pos_range())
    }
}

impl<I: VCodeInst> PartialEq for MachLabelFixup<I> {
    fn eq(&self, other: &Self) -> bool {
        self.deadline() == other.deadline()
    }
}

impl<I: VCodeInst> Eq for MachLabelFixup<I> {}

impl<I: VCodeInst> PartialOrd for MachLabelFixup<I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I: VCodeInst> Ord for MachLabelFixup<I> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.deadline().cmp(&self.deadline())
    }
}

/// A relocation resulting from a compilation.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct MachRelocBase<T> {
    /// The offset at which the relocation applies, *relative to the
    /// containing section*.
    pub offset: CodeOffset,
    /// The kind of relocation.
    pub kind: Reloc,
    /// The external symbol / name to which this relocation refers.
    pub target: T,
    /// The addend to add to the symbol value.
    pub addend: i64,
}

type MachReloc = MachRelocBase<RelocTarget>;

/// A relocation resulting from a compilation.
pub type FinalizedMachReloc = MachRelocBase<FinalizedRelocTarget>;

/// A Relocation target
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelocTarget {
    /// Points to an [ExternalName] outside the current function.
    ExternalName(ExternalName),
    /// Points to a [MachLabel] inside this function.
    /// This is different from [MachLabelFixup] in that both the relocation and the
    /// label will be emitted and are only resolved at link time.
    ///
    /// There is no reason to prefer this over [MachLabelFixup] unless the ABI requires it.
    Label(MachLabel),
}

impl From<ExternalName> for RelocTarget {
    fn from(name: ExternalName) -> Self {
        Self::ExternalName(name)
    }
}

impl From<MachLabel> for RelocTarget {
    fn from(label: MachLabel) -> Self {
        Self::Label(label)
    }
}

/// A Relocation target
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum FinalizedRelocTarget {
    /// Points to an [ExternalName] outside the current function.
    ExternalName(ExternalName),
    /// Points to a [CodeOffset] from the start of the current function.
    Func(CodeOffset),
}

impl FinalizedRelocTarget {
    /// Returns a display for the current [FinalizedRelocTarget], with extra context to prettify the
    /// output.
    pub fn display<'a>(&'a self, params: Option<&'a FunctionParameters>) -> String {
        match self {
            FinalizedRelocTarget::ExternalName(name) => format!("{}", name.display(params)),
            FinalizedRelocTarget::Func(offset) => format!("func+{offset}"),
        }
    }
}

/// A trap record resulting from a compilation.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct MachTrap {
    /// The offset at which the trap instruction occurs, *relative to the
    /// containing section*.
    pub offset: CodeOffset,
    /// The trap code.
    pub code: TrapCode,
}

/// A call site record resulting from a compilation.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct MachCallSite {
    /// The offset of the call's return address, *relative to the
    /// start of the buffer*.
    pub ret_addr: CodeOffset,

    /// Range in `exception_handlers` corresponding to the exception
    /// handlers for this callsite.
    exception_handler_range: Range<u32>,
}

/// A call site record resulting from a compilation.
#[derive(Clone, Debug, PartialEq)]
pub struct FinalizedMachCallSite<'a> {
    /// The offset of the call's return address, *relative to the
    /// start of the buffer*.
    pub ret_addr: CodeOffset,

    /// Exception handlers at this callsite, with target offsets
    /// *relative to the start of the buffer*.
    pub exception_handlers: &'a [FinalizedMachExceptionHandler],
}

/// A source-location mapping resulting from a compilation.
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct MachSrcLoc<T: CompilePhase> {
    /// The start of the region of code corresponding to a source location.
    /// This is relative to the start of the function, not to the start of the
    /// section.
    pub start: CodeOffset,
    /// The end of the region of code corresponding to a source location.
    /// This is relative to the start of the function, not to the start of the
    /// section.
    pub end: CodeOffset,
    /// The source location.
    pub loc: T::SourceLocType,
}

impl MachSrcLoc<Stencil> {
    fn apply_base_srcloc(self, base_srcloc: SourceLoc) -> MachSrcLoc<Final> {
        MachSrcLoc {
            start: self.start,
            end: self.end,
            loc: self.loc.expand(base_srcloc),
        }
    }
}

/// Record of branch instruction in the buffer, to facilitate editing.
#[derive(Clone, Debug)]
struct MachBranch {
    start: CodeOffset,
    end: CodeOffset,
    target: MachLabel,
    fixup: usize,
    inverted: Option<SmallVec<[u8; 8]>>,
    /// All labels pointing to the start of this branch. For correctness, this
    /// *must* be complete (i.e., must contain all labels whose resolved offsets
    /// are at the start of this branch): we rely on being able to redirect all
    /// labels that could jump to this branch before removing it, if it is
    /// otherwise unreachable.
    labels_at_this_branch: SmallVec<[MachLabel; 4]>,
}

impl MachBranch {
    fn is_cond(&self) -> bool {
        self.inverted.is_some()
    }
    fn is_uncond(&self) -> bool {
        self.inverted.is_none()
    }
}

/// Implementation of the `TextSectionBuilder` trait backed by `MachBuffer`.
///
/// Note that `MachBuffer` was primarily written for intra-function references
/// of jumps between basic blocks, but it's also quite usable for entire text
/// sections and resolving references between functions themselves. This
/// builder interprets "blocks" as labeled functions for the purposes of
/// resolving labels internally in the buffer.
pub struct MachTextSectionBuilder<I: VCodeInst> {
    buf: MachBuffer<I>,
    next_func: usize,
    force_veneers: ForceVeneers,
}

impl<I: VCodeInst> MachTextSectionBuilder<I> {
    /// Creates a new text section builder which will have `num_funcs` functions
    /// pushed into it.
    pub fn new(num_funcs: usize) -> MachTextSectionBuilder<I> {
        let mut buf = MachBuffer::new();
        buf.reserve_labels_for_blocks(num_funcs);
        MachTextSectionBuilder {
            buf,
            next_func: 0,
            force_veneers: ForceVeneers::No,
        }
    }
}

impl<I: VCodeInst> TextSectionBuilder for MachTextSectionBuilder<I> {
    fn append(
        &mut self,
        labeled: bool,
        func: &[u8],
        align: u32,
        ctrl_plane: &mut ControlPlane,
    ) -> u64 {
        // Conditionally emit an island if it's necessary to resolve jumps
        // between functions which are too far away.
        let size = func.len() as u32;
        if self.force_veneers == ForceVeneers::Yes || self.buf.island_needed(size) {
            self.buf
                .emit_island_maybe_forced(self.force_veneers, size, ctrl_plane);
        }

        self.buf.align_to(align);
        let pos = self.buf.cur_offset();
        if labeled {
            self.buf.bind_label(
                MachLabel::from_block(BlockIndex::new(self.next_func)),
                ctrl_plane,
            );
            self.next_func += 1;
        }
        self.buf.put_data(func);
        u64::from(pos)
    }

    fn resolve_reloc(&mut self, offset: u64, reloc: Reloc, addend: Addend, target: usize) -> bool {
        crate::trace!(
            "Resolving relocation @ {offset:#x} + {addend:#x} to target {target} of kind {reloc:?}"
        );
        let label = MachLabel::from_block(BlockIndex::new(target));
        let offset = u32::try_from(offset).unwrap();
        match I::LabelUse::from_reloc(reloc, addend) {
            Some(label_use) => {
                self.buf.use_label_at_offset(offset, label, label_use);
                true
            }
            None => false,
        }
    }

    fn force_veneers(&mut self) {
        self.force_veneers = ForceVeneers::Yes;
    }

    fn write(&mut self, offset: u64, data: &[u8]) {
        self.buf.data[offset.try_into().unwrap()..][..data.len()].copy_from_slice(data);
    }

    fn finish(&mut self, ctrl_plane: &mut ControlPlane) -> Vec<u8> {
        // Double-check all functions were pushed.
        assert_eq!(self.next_func, self.buf.label_offsets.len());

        // Finish up any veneers, if necessary.
        self.buf
            .finish_emission_maybe_forcing_veneers(self.force_veneers, ctrl_plane);

        // We don't need the data any more, so return it to the caller.
        mem::take(&mut self.buf.data).into_vec()
    }
}

// We use an actual instruction definition to do tests, so we depend on the `arm64` feature here.
#[cfg(all(test, feature = "arm64"))]
mod test {
    use cranelift_entity::EntityRef as _;

    use super::*;
    use crate::ir::UserExternalNameRef;
    use crate::isa::aarch64::inst::{BranchTarget, CondBrKind, EmitInfo, Inst};
    use crate::isa::aarch64::inst::{OperandSize, xreg};
    use crate::machinst::{MachInstEmit, MachInstEmitState};
    use crate::settings;

    fn label(n: u32) -> MachLabel {
        MachLabel::from_block(BlockIndex::new(n as usize))
    }
    fn target(n: u32) -> BranchTarget {
        BranchTarget::Label(label(n))
    }

    #[test]
    fn test_elide_jump_to_next() {
        let info = EmitInfo::new(settings::Flags::new(settings::builder()));
        let mut buf = MachBuffer::new();
        let mut state = <Inst as MachInstEmit>::State::default();
        let constants = Default::default();

        buf.reserve_labels_for_blocks(2);
        buf.bind_label(label(0), state.ctrl_plane_mut());
        let inst = Inst::Jump { dest: target(1) };
        inst.emit(&mut buf, &info, &mut state);
        buf.bind_label(label(1), state.ctrl_plane_mut());
        let buf = buf.finish(&constants, state.ctrl_plane_mut());
        assert_eq!(0, buf.total_size());
    }

    #[test]
    fn test_elide_trivial_jump_blocks() {
        let info = EmitInfo::new(settings::Flags::new(settings::builder()));
        let mut buf = MachBuffer::new();
        let mut state = <Inst as MachInstEmit>::State::default();
        let constants = Default::default();

        buf.reserve_labels_for_blocks(4);

        buf.bind_label(label(0), state.ctrl_plane_mut());
        let inst = Inst::CondBr {
            kind: CondBrKind::NotZero(xreg(0), OperandSize::Size64),
            taken: target(1),
            not_taken: target(2),
        };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(1), state.ctrl_plane_mut());
        let inst = Inst::Jump { dest: target(3) };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(2), state.ctrl_plane_mut());
        let inst = Inst::Jump { dest: target(3) };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(3), state.ctrl_plane_mut());

        let buf = buf.finish(&constants, state.ctrl_plane_mut());
        assert_eq!(0, buf.total_size());
    }

    #[test]
    fn test_flip_cond() {
        let info = EmitInfo::new(settings::Flags::new(settings::builder()));
        let mut buf = MachBuffer::new();
        let mut state = <Inst as MachInstEmit>::State::default();
        let constants = Default::default();

        buf.reserve_labels_for_blocks(4);

        buf.bind_label(label(0), state.ctrl_plane_mut());
        let inst = Inst::CondBr {
            kind: CondBrKind::Zero(xreg(0), OperandSize::Size64),
            taken: target(1),
            not_taken: target(2),
        };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(1), state.ctrl_plane_mut());
        let inst = Inst::Nop4;
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(2), state.ctrl_plane_mut());
        let inst = Inst::Udf {
            trap_code: TrapCode::STACK_OVERFLOW,
        };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(3), state.ctrl_plane_mut());

        let buf = buf.finish(&constants, state.ctrl_plane_mut());

        let mut buf2 = MachBuffer::new();
        let mut state = Default::default();
        let inst = Inst::TrapIf {
            kind: CondBrKind::NotZero(xreg(0), OperandSize::Size64),
            trap_code: TrapCode::STACK_OVERFLOW,
        };
        inst.emit(&mut buf2, &info, &mut state);
        let inst = Inst::Nop4;
        inst.emit(&mut buf2, &info, &mut state);

        let buf2 = buf2.finish(&constants, state.ctrl_plane_mut());

        assert_eq!(buf.data, buf2.data);
    }

    #[test]
    fn test_island() {
        let info = EmitInfo::new(settings::Flags::new(settings::builder()));
        let mut buf = MachBuffer::new();
        let mut state = <Inst as MachInstEmit>::State::default();
        let constants = Default::default();

        buf.reserve_labels_for_blocks(4);

        buf.bind_label(label(0), state.ctrl_plane_mut());
        let inst = Inst::CondBr {
            kind: CondBrKind::NotZero(xreg(0), OperandSize::Size64),
            taken: target(2),
            not_taken: target(3),
        };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(1), state.ctrl_plane_mut());
        while buf.cur_offset() < 2000000 {
            if buf.island_needed(0) {
                buf.emit_island(0, state.ctrl_plane_mut());
            }
            let inst = Inst::Nop4;
            inst.emit(&mut buf, &info, &mut state);
        }

        buf.bind_label(label(2), state.ctrl_plane_mut());
        let inst = Inst::Nop4;
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(3), state.ctrl_plane_mut());
        let inst = Inst::Nop4;
        inst.emit(&mut buf, &info, &mut state);

        let buf = buf.finish(&constants, state.ctrl_plane_mut());

        assert_eq!(2000000 + 8, buf.total_size());

        let mut buf2 = MachBuffer::new();
        let mut state = Default::default();
        let inst = Inst::CondBr {
            kind: CondBrKind::NotZero(xreg(0), OperandSize::Size64),

            // This conditionally taken branch has a 19-bit constant, shifted
            // to the left by two, giving us a 21-bit range in total. Half of
            // this range positive so the we should be around 1 << 20 bytes
            // away for our jump target.
            //
            // There are two pending fixups by the time we reach this point,
            // one for this 19-bit jump and one for the unconditional 26-bit
            // jump below. A 19-bit veneer is 4 bytes large and the 26-bit
            // veneer is 20 bytes large, which means that pessimistically
            // assuming we'll need two veneers. Currently each veneer is
            // pessimistically assumed to be the maximal size which means we
            // need 40 bytes of extra space, meaning that the actual island
            // should come 40-bytes before the deadline.
            taken: BranchTarget::ResolvedOffset((1 << 20) - 20 - 20),

            // This branch is in-range so no veneers should be needed, it should
            // go directly to the target.
            not_taken: BranchTarget::ResolvedOffset(2000000 + 4 - 4),
        };
        inst.emit(&mut buf2, &info, &mut state);

        let buf2 = buf2.finish(&constants, state.ctrl_plane_mut());

        assert_eq!(&buf.data[0..8], &buf2.data[..]);
    }

    #[test]
    fn test_island_backward() {
        let info = EmitInfo::new(settings::Flags::new(settings::builder()));
        let mut buf = MachBuffer::new();
        let mut state = <Inst as MachInstEmit>::State::default();
        let constants = Default::default();

        buf.reserve_labels_for_blocks(4);

        buf.bind_label(label(0), state.ctrl_plane_mut());
        let inst = Inst::Nop4;
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(1), state.ctrl_plane_mut());
        let inst = Inst::Nop4;
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(2), state.ctrl_plane_mut());
        while buf.cur_offset() < 2000000 {
            let inst = Inst::Nop4;
            inst.emit(&mut buf, &info, &mut state);
        }

        buf.bind_label(label(3), state.ctrl_plane_mut());
        let inst = Inst::CondBr {
            kind: CondBrKind::NotZero(xreg(0), OperandSize::Size64),
            taken: target(0),
            not_taken: target(1),
        };
        inst.emit(&mut buf, &info, &mut state);

        let buf = buf.finish(&constants, state.ctrl_plane_mut());

        assert_eq!(2000000 + 12, buf.total_size());

        let mut buf2 = MachBuffer::new();
        let mut state = Default::default();
        let inst = Inst::CondBr {
            kind: CondBrKind::NotZero(xreg(0), OperandSize::Size64),
            taken: BranchTarget::ResolvedOffset(8),
            not_taken: BranchTarget::ResolvedOffset(4 - (2000000 + 4)),
        };
        inst.emit(&mut buf2, &info, &mut state);
        let inst = Inst::Jump {
            dest: BranchTarget::ResolvedOffset(-(2000000 + 8)),
        };
        inst.emit(&mut buf2, &info, &mut state);

        let buf2 = buf2.finish(&constants, state.ctrl_plane_mut());

        assert_eq!(&buf.data[2000000..], &buf2.data[..]);
    }

    #[test]
    fn test_multiple_redirect() {
        // label0:
        //   cbz x0, label1
        //   b label2
        // label1:
        //   b label3
        // label2:
        //   nop
        //   nop
        //   b label0
        // label3:
        //   b label4
        // label4:
        //   b label5
        // label5:
        //   b label7
        // label6:
        //   nop
        // label7:
        //   ret
        //
        // -- should become:
        //
        // label0:
        //   cbz x0, label7
        // label2:
        //   nop
        //   nop
        //   b label0
        // label6:
        //   nop
        // label7:
        //   ret

        let info = EmitInfo::new(settings::Flags::new(settings::builder()));
        let mut buf = MachBuffer::new();
        let mut state = <Inst as MachInstEmit>::State::default();
        let constants = Default::default();

        buf.reserve_labels_for_blocks(8);

        buf.bind_label(label(0), state.ctrl_plane_mut());
        let inst = Inst::CondBr {
            kind: CondBrKind::Zero(xreg(0), OperandSize::Size64),
            taken: target(1),
            not_taken: target(2),
        };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(1), state.ctrl_plane_mut());
        let inst = Inst::Jump { dest: target(3) };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(2), state.ctrl_plane_mut());
        let inst = Inst::Nop4;
        inst.emit(&mut buf, &info, &mut state);
        inst.emit(&mut buf, &info, &mut state);
        let inst = Inst::Jump { dest: target(0) };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(3), state.ctrl_plane_mut());
        let inst = Inst::Jump { dest: target(4) };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(4), state.ctrl_plane_mut());
        let inst = Inst::Jump { dest: target(5) };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(5), state.ctrl_plane_mut());
        let inst = Inst::Jump { dest: target(7) };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(6), state.ctrl_plane_mut());
        let inst = Inst::Nop4;
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(7), state.ctrl_plane_mut());
        let inst = Inst::Ret {};
        inst.emit(&mut buf, &info, &mut state);

        let buf = buf.finish(&constants, state.ctrl_plane_mut());

        let golden_data = vec![
            0xa0, 0x00, 0x00, 0xb4, // cbz x0, 0x14
            0x1f, 0x20, 0x03, 0xd5, // nop
            0x1f, 0x20, 0x03, 0xd5, // nop
            0xfd, 0xff, 0xff, 0x17, // b 0
            0x1f, 0x20, 0x03, 0xd5, // nop
            0xc0, 0x03, 0x5f, 0xd6, // ret
        ];

        assert_eq!(&golden_data[..], &buf.data[..]);
    }

    #[test]
    fn test_handle_branch_cycle() {
        // label0:
        //   b label1
        // label1:
        //   b label2
        // label2:
        //   b label3
        // label3:
        //   b label4
        // label4:
        //   b label1  // note: not label0 (to make it interesting).
        //
        // -- should become:
        //
        // label0, label1, ..., label4:
        //   b label0
        let info = EmitInfo::new(settings::Flags::new(settings::builder()));
        let mut buf = MachBuffer::new();
        let mut state = <Inst as MachInstEmit>::State::default();
        let constants = Default::default();

        buf.reserve_labels_for_blocks(5);

        buf.bind_label(label(0), state.ctrl_plane_mut());
        let inst = Inst::Jump { dest: target(1) };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(1), state.ctrl_plane_mut());
        let inst = Inst::Jump { dest: target(2) };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(2), state.ctrl_plane_mut());
        let inst = Inst::Jump { dest: target(3) };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(3), state.ctrl_plane_mut());
        let inst = Inst::Jump { dest: target(4) };
        inst.emit(&mut buf, &info, &mut state);

        buf.bind_label(label(4), state.ctrl_plane_mut());
        let inst = Inst::Jump { dest: target(1) };
        inst.emit(&mut buf, &info, &mut state);

        let buf = buf.finish(&constants, state.ctrl_plane_mut());

        let golden_data = vec![
            0x00, 0x00, 0x00, 0x14, // b 0
        ];

        assert_eq!(&golden_data[..], &buf.data[..]);
    }

    #[test]
    fn metadata_records() {
        let mut buf = MachBuffer::<Inst>::new();
        let ctrl_plane = &mut Default::default();
        let constants = Default::default();

        buf.reserve_labels_for_blocks(3);

        buf.bind_label(label(0), ctrl_plane);
        buf.put1(1);
        buf.add_trap(TrapCode::HEAP_OUT_OF_BOUNDS);
        buf.put1(2);
        buf.add_trap(TrapCode::INTEGER_OVERFLOW);
        buf.add_trap(TrapCode::INTEGER_DIVISION_BY_ZERO);
        buf.add_try_call_site(
            [
                MachExceptionHandler::Tag(ExceptionTag::new(42), label(2)),
                MachExceptionHandler::Default(label(1)),
            ]
            .into_iter(),
        );
        buf.add_reloc(
            Reloc::Abs4,
            &ExternalName::User(UserExternalNameRef::new(0)),
            0,
        );
        buf.put1(3);
        buf.add_reloc(
            Reloc::Abs8,
            &ExternalName::User(UserExternalNameRef::new(1)),
            1,
        );
        buf.put1(4);
        buf.bind_label(label(1), ctrl_plane);
        buf.put1(0xff);
        buf.bind_label(label(2), ctrl_plane);
        buf.put1(0xff);

        let buf = buf.finish(&constants, ctrl_plane);

        assert_eq!(buf.data(), &[1, 2, 3, 4, 0xff, 0xff]);
        assert_eq!(
            buf.traps()
                .iter()
                .map(|trap| (trap.offset, trap.code))
                .collect::<Vec<_>>(),
            vec![
                (1, TrapCode::HEAP_OUT_OF_BOUNDS),
                (2, TrapCode::INTEGER_OVERFLOW),
                (2, TrapCode::INTEGER_DIVISION_BY_ZERO)
            ]
        );
        let call_sites: Vec<_> = buf.call_sites().collect();
        assert_eq!(call_sites[0].ret_addr, 2);
        assert_eq!(
            call_sites[0].exception_handlers,
            &[
                FinalizedMachExceptionHandler::Tag(ExceptionTag::new(42), 5),
                FinalizedMachExceptionHandler::Default(4)
            ],
        );
        assert_eq!(
            buf.relocs()
                .iter()
                .map(|reloc| (reloc.offset, reloc.kind))
                .collect::<Vec<_>>(),
            vec![(2, Reloc::Abs4), (3, Reloc::Abs8)]
        );
    }
}
