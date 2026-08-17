//! Definitions for "memory types" in CLIF.
//!
//! A memory type is a struct-like definition -- fields with offsets,
//! each field having a type and possibly an attached fact -- that we
//! can use in proof-carrying code to validate accesses to structs and
//! propagate facts onto the loaded values as well.
//!
//! Memory types are meant to be rich enough to describe the *layout*
//! of values in memory, but do not necessarily need to embody
//! higher-level features such as subtyping directly. Rather, they
//! should encode an implementation of a type or object system.
//!
//! Note also that it is a non-goal for now for this type system to be
//! "complete" or fully orthogonal: we have some restrictions now
//! (e.g., struct fields are only primitives) because this is all we
//! need for existing PCC applications, and it keeps the
//! implementation simpler.
//!
//! There are a few basic kinds of types:
//!
//! - A struct is an aggregate of fields and an overall size. Each
//!   field has a *primitive Cranelift type*. This is for simplicity's
//!   sake: we do not allow nested memory types because to do so
//!   invites cycles, requires recursive computation of sizes, creates
//!   complicated questions when field types are dynamically-sized,
//!   and in general is more complexity than we need.
//!
//!   The expectation (validated by PCC) is that when a checked load
//!   or store accesses memory typed by a memory type, accesses will
//!   only be to fields at offsets named in the type, and will be via
//!   the given Cranelift type -- i.e., no type-punning occurs in
//!   memory.
//!
//!   The overall size of the struct may be larger than that implied
//!   by the fields because (i) we may not want or need to name all
//!   the actually-existing fields in the memory type, and (ii) there
//!   may be alignment padding that we also don't want or need to
//!   represent explicitly.
//!
//! - A static memory is an untyped blob of storage with a static
//!   size. This is memory that can be accessed with any type of load
//!   or store at any valid offset.
//!
//!   Note that this is *distinct* from an "array of u8" kind of
//!   representation of memory, if/when we can represent such a thing,
//!   because the expectation with memory types' fields (including
//!   array elements) is that they are strongly typed, only accessed
//!   via that type, and not type-punned. We don't want to imply any
//!   restriction on load/store size, or any actual structure, with
//!   untyped memory; it's just a blob.
//!
//! Eventually we plan to also have:
//!
//! - A dynamic array is a sequence of struct memory types, with a
//!   length given by a global value (GV). This is useful to model,
//!   e.g., tables.
//!
//! - A discriminated union is a union of several memory types
//!   together with a tag field. This will be useful to model and
//!   verify subtyping/downcasting for Wasm GC, among other uses.
//!
//! - Nullability on pointer fields: the fact will hold only if the
//!   field is not null (all zero bits).

use crate::ir::pcc::Fact;
use crate::ir::{GlobalValue, Type};
use alloc::vec::Vec;

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// Data defining a memory type.
///
/// A memory type corresponds to a layout of data in memory. It may
/// have a statically-known or dynamically-known size.
#[derive(Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum MemoryTypeData {
    /// An aggregate consisting of certain fields at certain offsets.
    ///
    /// Fields must be sorted by offset, must be within the struct's
    /// overall size, and must not overlap. These conditions are
    /// checked by the CLIF verifier.
    Struct {
        /// Size of this type.
        size: u64,

        /// Fields in this type. Sorted by offset.
        fields: Vec<MemoryTypeField>,
    },

    /// A statically-sized untyped blob of memory.
    Memory {
        /// Accessible size.
        size: u64,
    },

    /// A dynamically-sized untyped blob of memory, with bound given
    /// by a global value plus some static amount.
    DynamicMemory {
        /// Static part of size.
        size: u64,
        /// Dynamic part of size.
        gv: GlobalValue,
    },

    /// A type with no size.
    Empty,
}

impl std::default::Default for MemoryTypeData {
    fn default() -> Self {
        Self::Empty
    }
}

impl std::fmt::Display for MemoryTypeData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Struct { size, fields } => {
                write!(f, "struct {size} {{")?;
                let mut first = true;
                for field in fields {
                    if first {
                        first = false;
                    } else {
                        write!(f, ",")?;
                    }
                    write!(f, " {}: {}", field.offset, field.ty)?;
                    if field.readonly {
                        write!(f, " readonly")?;
                    }
                    if let Some(fact) = &field.fact {
                        write!(f, " ! {fact}")?;
                    }
                }
                write!(f, " }}")?;
                Ok(())
            }
            Self::Memory { size } => {
                write!(f, "memory {size:#x}")
            }
            Self::DynamicMemory { size, gv } => {
                write!(f, "dynamic_memory {gv}+{size:#x}")
            }
            Self::Empty => {
                write!(f, "empty")
            }
        }
    }
}

/// One field in a memory type.
#[derive(Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct MemoryTypeField {
    /// The offset of this field in the memory type.
    pub offset: u64,
    /// The primitive type of the value in this field. Accesses to the
    /// field must use this type (i.e., cannot bitcast/type-pun in
    /// memory).
    pub ty: Type,
    /// A proof-carrying-code fact about this value, if any.
    pub fact: Option<Fact>,
    /// Whether this field is read-only, i.e., stores should be
    /// disallowed.
    pub readonly: bool,
}

impl MemoryTypeField {
    /// Get the fact, if any, on a field.
    pub fn fact(&self) -> Option<&Fact> {
        self.fact.as_ref()
    }
}

impl MemoryTypeData {
    /// Provide the static size of this type, if known.
    ///
    /// (The size may not be known for dynamically-sized arrays or
    /// memories, when those memtype kinds are added.)
    pub fn static_size(&self) -> Option<u64> {
        match self {
            Self::Struct { size, .. } => Some(*size),
            Self::Memory { size } => Some(*size),
            Self::DynamicMemory { .. } => None,
            Self::Empty => Some(0),
        }
    }
}
