use core::fmt;
use object::{Bytes, LittleEndian, U32Bytes};

/// Information about trap.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TrapInformation {
    /// The offset of the trapping instruction in native code.
    ///
    /// This is relative to the beginning of the function.
    pub code_offset: u32,

    /// Code of the trap.
    pub trap_code: Trap,
}

// The code can be accessed from the c-api, where the possible values are
// translated into enum values defined there:
//
// * `wasm_trap_code` in c-api/src/trap.rs, and
// * `wasmtime_trap_code_enum` in c-api/include/wasmtime/trap.h.
//
// These need to be kept in sync.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[expect(missing_docs, reason = "self-describing variants")]
pub enum Trap {
    /// The current stack space was exhausted.
    StackOverflow,

    /// An out-of-bounds memory access.
    MemoryOutOfBounds,

    /// A wasm atomic operation was presented with a not-naturally-aligned linear-memory address.
    HeapMisaligned,

    /// An out-of-bounds access to a table.
    TableOutOfBounds,

    /// Indirect call to a null table entry.
    IndirectCallToNull,

    /// Signature mismatch on indirect call.
    BadSignature,

    /// An integer arithmetic operation caused an overflow.
    IntegerOverflow,

    /// An integer division by zero.
    IntegerDivisionByZero,

    /// Failed float-to-int conversion.
    BadConversionToInteger,

    /// Code that was supposed to have been unreachable was reached.
    UnreachableCodeReached,

    /// Execution has potentially run too long and may be interrupted.
    Interrupt,

    /// When the `component-model` feature is enabled this trap represents a
    /// function that was `canon lift`'d, then `canon lower`'d, then called.
    /// This combination of creation of a function in the component model
    /// generates a function that always traps and, when called, produces this
    /// flavor of trap.
    AlwaysTrapAdapter,

    /// When wasm code is configured to consume fuel and it runs out of fuel
    /// then this trap will be raised.
    OutOfFuel,

    /// Used to indicate that a trap was raised by atomic wait operations on non shared memory.
    AtomicWaitNonSharedMemory,

    /// Call to a null reference.
    NullReference,

    /// Attempt to access beyond the bounds of an array.
    ArrayOutOfBounds,

    /// Attempted an allocation that was too large to succeed.
    AllocationTooLarge,

    /// Attempted to cast a reference to a type that it is not an instance of.
    CastFailure,

    /// When the `component-model` feature is enabled this trap represents a
    /// scenario where one component tried to call another component but it
    /// would have violated the reentrance rules of the component model,
    /// triggering a trap instead.
    CannotEnterComponent,

    /// Async-lifted export failed to produce a result by calling `task.return`
    /// before returning `STATUS_DONE` and/or after all host tasks completed.
    NoAsyncResult,

    /// We are suspending to a tag for which there is no active handler.
    UnhandledTag,

    /// Attempt to resume a continuation twice.
    ContinuationAlreadyConsumed,

    /// A Pulley opcode was executed at runtime when the opcode was disabled at
    /// compile time.
    DisabledOpcode,

    /// Async event loop deadlocked; i.e. it cannot make further progress given
    /// that all host tasks have completed and any/all host-owned stream/future
    /// handles have been dropped.
    AsyncDeadlock,
    // if adding a variant here be sure to update the `check!` macro below
}

impl Trap {
    /// Converts a byte back into a `Trap` if its in-bounds
    pub fn from_u8(byte: u8) -> Option<Trap> {
        // FIXME: this could use some sort of derive-like thing to avoid having to
        // deduplicate the names here.
        //
        // This simply converts from the a `u8`, to the `Trap` enum.
        macro_rules! check {
            ($($name:ident)*) => ($(if byte == Trap::$name as u8 {
                return Some(Trap::$name);
            })*);
        }

        check! {
            StackOverflow
            MemoryOutOfBounds
            HeapMisaligned
            TableOutOfBounds
            IndirectCallToNull
            BadSignature
            IntegerOverflow
            IntegerDivisionByZero
            BadConversionToInteger
            UnreachableCodeReached
            Interrupt
            AlwaysTrapAdapter
            OutOfFuel
            AtomicWaitNonSharedMemory
            NullReference
            ArrayOutOfBounds
            AllocationTooLarge
            CastFailure
            CannotEnterComponent
            NoAsyncResult
            UnhandledTag
            ContinuationAlreadyConsumed
            DisabledOpcode
            AsyncDeadlock
        }

        None
    }
}

impl fmt::Display for Trap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Trap::*;

        let desc = match self {
            StackOverflow => "call stack exhausted",
            MemoryOutOfBounds => "out of bounds memory access",
            HeapMisaligned => "unaligned atomic",
            TableOutOfBounds => "undefined element: out of bounds table access",
            IndirectCallToNull => "uninitialized element",
            BadSignature => "indirect call type mismatch",
            IntegerOverflow => "integer overflow",
            IntegerDivisionByZero => "integer divide by zero",
            BadConversionToInteger => "invalid conversion to integer",
            UnreachableCodeReached => "wasm `unreachable` instruction executed",
            Interrupt => "interrupt",
            AlwaysTrapAdapter => "degenerate component adapter called",
            OutOfFuel => "all fuel consumed by WebAssembly",
            AtomicWaitNonSharedMemory => "atomic wait on non-shared memory",
            NullReference => "null reference",
            ArrayOutOfBounds => "out of bounds array access",
            AllocationTooLarge => "allocation size too large",
            CastFailure => "cast failure",
            CannotEnterComponent => "cannot enter component instance",
            NoAsyncResult => "async-lifted export failed to produce a result",
            UnhandledTag => "unhandled tag",
            ContinuationAlreadyConsumed => "continuation already consumed",
            DisabledOpcode => "pulley opcode disabled at compile time was executed",
            AsyncDeadlock => "deadlock detected: event loop cannot make further progress",
        };
        write!(f, "wasm trap: {desc}")
    }
}

impl core::error::Error for Trap {}

/// Decodes the provided trap information section and attempts to find the trap
/// code corresponding to the `offset` specified.
///
/// The `section` provided is expected to have been built by
/// `TrapEncodingBuilder` above. Additionally the `offset` should be a relative
/// offset within the text section of the compilation image.
pub fn lookup_trap_code(section: &[u8], offset: usize) -> Option<Trap> {
    let (offsets, traps) = parse(section)?;

    // The `offsets` table is sorted in the trap section so perform a binary
    // search of the contents of this section to find whether `offset` is an
    // entry in the section. Note that this is a precise search because trap pcs
    // should always be precise as well as our metadata about them, which means
    // we expect an exact match to correspond to a trap opcode.
    //
    // Once an index is found within the `offsets` array then that same index is
    // used to lookup from the `traps` list of bytes to get the trap code byte
    // corresponding to this offset.
    let offset = u32::try_from(offset).ok()?;
    let index = offsets
        .binary_search_by_key(&offset, |val| val.get(LittleEndian))
        .ok()?;
    debug_assert!(index < traps.len());
    let byte = *traps.get(index)?;

    let trap = Trap::from_u8(byte);
    debug_assert!(trap.is_some(), "missing mapping for {byte}");
    trap
}

fn parse(section: &[u8]) -> Option<(&[U32Bytes<LittleEndian>], &[u8])> {
    let mut section = Bytes(section);
    // NB: this matches the encoding written by `append_to` above.
    let count = section.read::<U32Bytes<LittleEndian>>().ok()?;
    let count = usize::try_from(count.get(LittleEndian)).ok()?;
    let (offsets, traps) =
        object::slice_from_bytes::<U32Bytes<LittleEndian>>(section.0, count).ok()?;
    debug_assert_eq!(traps.len(), count);
    Some((offsets, traps))
}

/// Returns an iterator over all of the traps encoded in `section`, which should
/// have been produced by `TrapEncodingBuilder`.
pub fn iterate_traps(section: &[u8]) -> Option<impl Iterator<Item = (u32, Trap)> + '_> {
    let (offsets, traps) = parse(section)?;
    Some(
        offsets
            .iter()
            .zip(traps)
            .map(|(offset, trap)| (offset.get(LittleEndian), Trap::from_u8(*trap).unwrap())),
    )
}
