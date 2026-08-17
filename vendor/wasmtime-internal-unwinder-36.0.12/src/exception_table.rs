//! Compact representation of exception handlers associated with
//! callsites, for use when searching a Cranelift stack for a handler.
//!
//! This module implements (i) conversion from the metadata provided
//! alongside Cranelift's compilation result (as provided by
//! [`cranelift_codegen::MachBufferFinalized::call_sites`]) to its
//! format, and (ii) use of its format to find a handler efficiently.
//!
//! The format has been designed so that it can be mapped in from disk
//! and used without post-processing; this enables efficient
//! module-loading in runtimes such as Wasmtime.

use object::{Bytes, LittleEndian, U32Bytes};

#[cfg(feature = "cranelift")]
use alloc::{vec, vec::Vec};
#[cfg(feature = "cranelift")]
use cranelift_codegen::{
    ExceptionContextLoc, FinalizedMachCallSite, FinalizedMachExceptionHandler, binemit::CodeOffset,
};

/// Collector struct for exception handlers per call site.
///
/// # Format
///
/// We keep five different arrays (`Vec`s) that we build as we visit
/// callsites, in ascending offset (address relative to beginning of
/// code segment) order: callsite offsets, tag/destination ranges,
/// tags, tag context SP offset, destination offsets.
///
/// The callsite offsets and tag/destination ranges logically form a
/// sorted lookup array, allowing us to find information for any
/// single callsite. The range denotes a range of indices in the
/// tag/context and destination offset arrays. Ranges are stored with
/// the (exclusive) *end* index only; the start index is implicit as
/// the previous end, or zero if first element.
///
/// The slices of tag, context, and handlers arrays named by `ranges`
/// for each callsite specify a series of handler items for that
/// callsite. The tag and context together allow a
/// dynamic-tag-instance match in the unwinder: the context specifies
/// an offset from SP at the callsite that contains a machine word
/// (e.g. with vmctx) that, together with the static tag index, can be
/// used to perform a dynamic match. A context of `-1` indicates no
/// dynamic context, and a tag of `-1` indicates a catch-all
/// handler. If a handler item matches, control should be transferred
/// to the code offset given in the last array, `handlers`.
///
/// # Example
///
/// An example of this data format:
///
/// ```plain
/// callsites: [0x10, 0x50, 0xf0] // callsites (return addrs) at offsets 0x10, 0x50, 0xf0
/// ranges: [2, 4, 5]             // corresponding ranges for each callsite
/// tags: [1, 5, 1, -1, -1]       // tags for each handler at each callsite
/// contexts: [-1, -1, 0x10, 0x20, 0x30] // SP-offset for context for each tag
/// handlers: [0x40, 0x42, 0x6f, 0x71, 0xf5] // handler destinations at each callsite
/// ```
///
/// Expanding this out:
///
/// ```plain
/// callsites: [0x10, 0x50, 0xf0],  # PCs relative to some start of return-points.
/// ranges: [
///     2,  # callsite 0x10 has tags/handlers indices 0..2
///     4,  # callsite 0x50 has tags/handlers indices 2..4
///     5,  # callsite 0xf0 has tags/handlers indices 4..5
/// ],
/// tags: [
///     # tags for callsite 0x10:
///     1,
///     5,
///     # tags for callsite 0x50:
///     1,
///     -1,  # "catch-all"
///     # tags for callsite 0xf0:
///     -1,  # "catch-all"
/// ]
/// contexts: [
///     # SP-offsets for context for each tag at callsite 0x10:
///     -1,
///     -1,
///     # for callsite 0x50:
///     0x10,
///     0x20,
///     # for callsite 0xf0:
///     0x30,
/// ]
/// handlers: [
///     # handlers for callsite 0x10:
///     0x40,  # relative PC to handle tag 1 (above)
///     0x42,  # relative PC to handle tag 5
///     # handlers for callsite 0x50:
///     0x6f,  # relative PC to handle tag 1
///     0x71,  # relative PC to handle all other tags
///     # handlers for callsite 0xf0:
///     0xf5,  # relative PC to handle all other tags
/// ]
/// ```
#[cfg(feature = "cranelift")]
#[derive(Clone, Debug, Default)]
pub struct ExceptionTableBuilder {
    pub callsites: Vec<U32Bytes<LittleEndian>>,
    pub ranges: Vec<U32Bytes<LittleEndian>>,
    pub tags: Vec<U32Bytes<LittleEndian>>,
    pub contexts: Vec<U32Bytes<LittleEndian>>,
    pub handlers: Vec<U32Bytes<LittleEndian>>,
    last_start_offset: CodeOffset,
}

#[cfg(feature = "cranelift")]
impl ExceptionTableBuilder {
    /// Add a function at a given offset from the start of the
    /// compiled code section, recording information about its call
    /// sites.
    ///
    /// Functions must be added in ascending offset order.
    pub fn add_func<'a>(
        &mut self,
        start_offset: CodeOffset,
        call_sites: impl Iterator<Item = FinalizedMachCallSite<'a>>,
    ) -> anyhow::Result<()> {
        // Ensure that we see functions in offset order.
        assert!(start_offset >= self.last_start_offset);
        self.last_start_offset = start_offset;

        // Visit each callsite in turn, translating offsets from
        // function-local to section-local.
        let mut handlers = vec![];
        for call_site in call_sites {
            let ret_addr = call_site.ret_addr.checked_add(start_offset).unwrap();
            handlers.extend(call_site.exception_handlers.iter().cloned());

            let start_idx = u32::try_from(self.tags.len()).unwrap();
            let mut context = u32::MAX;
            for handler in call_site.exception_handlers {
                match handler {
                    FinalizedMachExceptionHandler::Tag(tag, offset) => {
                        self.tags.push(U32Bytes::new(LittleEndian, tag.as_u32()));
                        self.contexts.push(U32Bytes::new(LittleEndian, context));
                        self.handlers.push(U32Bytes::new(
                            LittleEndian,
                            offset.checked_add(start_offset).unwrap(),
                        ));
                    }
                    FinalizedMachExceptionHandler::Default(offset) => {
                        self.tags.push(U32Bytes::new(LittleEndian, u32::MAX));
                        self.contexts.push(U32Bytes::new(LittleEndian, context));
                        self.handlers.push(U32Bytes::new(
                            LittleEndian,
                            offset.checked_add(start_offset).unwrap(),
                        ));
                    }
                    FinalizedMachExceptionHandler::Context(ExceptionContextLoc::SPOffset(
                        offset,
                    )) => {
                        context = *offset;
                    }
                    FinalizedMachExceptionHandler::Context(ExceptionContextLoc::GPR(_)) => {
                        panic!(
                            "Wasmtime exception unwind info only supports dynamic contexts on the stack"
                        );
                    }
                }
            }
            let end_idx = u32::try_from(self.tags.len()).unwrap();

            // Omit empty callsites for compactness.
            if end_idx > start_idx {
                self.ranges.push(U32Bytes::new(LittleEndian, end_idx));
                self.callsites.push(U32Bytes::new(LittleEndian, ret_addr));
            }
        }

        Ok(())
    }

    /// Serialize the exception-handler data section, taking a closure
    /// to consume slices.
    pub fn serialize<F: FnMut(&[u8])>(&self, mut f: F) {
        // Serialize the length of `callsites` / `ranges`.
        let callsite_count = u32::try_from(self.callsites.len()).unwrap();
        f(&callsite_count.to_le_bytes());
        // Serialize the length of `tags` / `handlers`.
        let handler_count = u32::try_from(self.handlers.len()).unwrap();
        f(&handler_count.to_le_bytes());

        // Serialize `callsites`, `ranges`, `tags`, and `handlers` in
        // that order.
        f(object::bytes_of_slice(&self.callsites));
        f(object::bytes_of_slice(&self.ranges));
        f(object::bytes_of_slice(&self.tags));
        f(object::bytes_of_slice(&self.contexts));
        f(object::bytes_of_slice(&self.handlers));
    }

    /// Serialize the exception-handler data section to a vector of
    /// bytes.
    pub fn to_vec(&self) -> Vec<u8> {
        let mut bytes = vec![];
        self.serialize(|slice| bytes.extend(slice.iter().cloned()));
        bytes
    }
}

/// ExceptionTable deserialized from a serialized slice.
///
/// This struct retains borrows of the various serialized parts of the
/// exception table data as produced by
/// [`ExceptionTableBuilder::serialize`].
#[derive(Clone, Debug)]
pub struct ExceptionTable<'a> {
    callsites: &'a [U32Bytes<LittleEndian>],
    ranges: &'a [U32Bytes<LittleEndian>],
    tags: &'a [U32Bytes<LittleEndian>],
    #[expect(
        dead_code,
        reason = "Will be used in subsequent PR for Wasm exception handling"
    )]
    contexts: &'a [U32Bytes<LittleEndian>],
    handlers: &'a [U32Bytes<LittleEndian>],
}

impl<'a> ExceptionTable<'a> {
    /// Parse exception tables from a byte-slice as produced by
    /// [`ExceptionTableBuilder::serialize`].
    pub fn parse(data: &'a [u8]) -> anyhow::Result<ExceptionTable<'a>> {
        let mut data = Bytes(data);
        let callsite_count = data
            .read::<U32Bytes<LittleEndian>>()
            .map_err(|_| anyhow::anyhow!("Unable to read callsite count prefix"))?;
        let callsite_count = usize::try_from(callsite_count.get(LittleEndian))?;
        let handler_count = data
            .read::<U32Bytes<LittleEndian>>()
            .map_err(|_| anyhow::anyhow!("Unable to read handler count prefix"))?;
        let handler_count = usize::try_from(handler_count.get(LittleEndian))?;
        let (callsites, data) =
            object::slice_from_bytes::<U32Bytes<LittleEndian>>(data.0, callsite_count)
                .map_err(|_| anyhow::anyhow!("Unable to read callsites slice"))?;
        let (ranges, data) =
            object::slice_from_bytes::<U32Bytes<LittleEndian>>(data, callsite_count)
                .map_err(|_| anyhow::anyhow!("Unable to read ranges slice"))?;
        let (tags, data) = object::slice_from_bytes::<U32Bytes<LittleEndian>>(data, handler_count)
            .map_err(|_| anyhow::anyhow!("Unable to read tags slice"))?;
        let (contexts, data) =
            object::slice_from_bytes::<U32Bytes<LittleEndian>>(data, handler_count)
                .map_err(|_| anyhow::anyhow!("Unable to read contexts slice"))?;
        let (handlers, data) =
            object::slice_from_bytes::<U32Bytes<LittleEndian>>(data, handler_count)
                .map_err(|_| anyhow::anyhow!("Unable to read handlers slice"))?;

        if !data.is_empty() {
            anyhow::bail!("Unexpected data at end of serialized exception table");
        }

        Ok(ExceptionTable {
            callsites,
            ranges,
            tags,
            contexts,
            handlers,
        })
    }

    /// Look up the handler destination, if any, for a given return
    /// address (as an offset into the code section) and exception
    /// tag.
    ///
    /// Note: we use raw `u32` types for code offsets and tags here to
    /// avoid dependencies on `cranelift-codegen` when this crate is
    /// built without compiler backend support (runtime-only config).
    pub fn lookup(&self, pc: u32, tag: u32) -> Option<u32> {
        // First, look up the callsite in the sorted callsites list.
        let callsite_idx = self
            .callsites
            .binary_search_by_key(&pc, |callsite| callsite.get(LittleEndian))
            .ok()?;
        // Now get the range.
        let end_idx = self.ranges[callsite_idx].get(LittleEndian);
        let start_idx = if callsite_idx > 0 {
            self.ranges[callsite_idx - 1].get(LittleEndian)
        } else {
            0
        };

        // Take the subslices of `tags` and `handlers` corresponding
        // to this callsite.
        let start_idx = usize::try_from(start_idx).unwrap();
        let end_idx = usize::try_from(end_idx).unwrap();
        let tags = &self.tags[start_idx..end_idx];
        let handlers = &self.handlers[start_idx..end_idx];

        // Is there any handler with an exact tag match?
        if let Ok(handler_idx) = tags.binary_search_by_key(&tag, |tag| tag.get(LittleEndian)) {
            return Some(handlers[handler_idx].get(LittleEndian));
        }

        // If not, is there a fallback handler? Note that we serialize
        // it with the tag `u32::MAX`, so it is always last in sorted
        // order.
        if tags.last().map(|v| v.get(LittleEndian)) == Some(u32::MAX) {
            return Some(handlers.last().unwrap().get(LittleEndian));
        }

        None
    }
}

#[cfg(all(test, feature = "cranelift"))]
mod test {
    use super::*;
    use cranelift_codegen::entity::EntityRef;
    use cranelift_codegen::ir::ExceptionTag;

    #[test]
    fn serialize_exception_table() {
        let callsites = [
            FinalizedMachCallSite {
                ret_addr: 0x10,
                exception_handlers: &[
                    FinalizedMachExceptionHandler::Tag(ExceptionTag::new(1), 0x20),
                    FinalizedMachExceptionHandler::Tag(ExceptionTag::new(2), 0x30),
                    FinalizedMachExceptionHandler::Default(0x40),
                ],
            },
            FinalizedMachCallSite {
                ret_addr: 0x48,
                exception_handlers: &[],
            },
            FinalizedMachCallSite {
                ret_addr: 0x50,
                exception_handlers: &[FinalizedMachExceptionHandler::Default(0x60)],
            },
        ];

        let mut builder = ExceptionTableBuilder::default();
        builder.add_func(0x100, callsites.into_iter()).unwrap();
        let mut bytes = vec![];
        builder.serialize(|slice| bytes.extend(slice.iter().cloned()));

        let deserialized = ExceptionTable::parse(&bytes).unwrap();

        assert_eq!(deserialized.lookup(0x148, 1), None);
        assert_eq!(deserialized.lookup(0x110, 1), Some(0x120));
        assert_eq!(deserialized.lookup(0x110, 2), Some(0x130));
        assert_eq!(deserialized.lookup(0x110, 42), Some(0x140));
        assert_eq!(deserialized.lookup(0x150, 100), Some(0x160));
    }
}
