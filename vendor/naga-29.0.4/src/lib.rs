/*!
Naga can be used to translate source code written in one shading language to another.

# Example

The following example translates WGSL to GLSL.
It requires the features `"wgsl-in"` and `"glsl-out"` to be enabled.

*/
// If we don't have the required front- and backends, don't try to build this example.
#![cfg_attr(all(feature = "wgsl-in", feature = "glsl-out"), doc = "```")]
#![cfg_attr(not(all(feature = "wgsl-in", feature = "glsl-out")), doc = "```ignore")]
/*!
let wgsl_source = "
@fragment
fn main_fs() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
";

// Parse the source into a Module.
let module: naga::Module = naga::front::wgsl::parse_str(wgsl_source)?;

// Validate the module.
// Validation can be made less restrictive by changing the ValidationFlags.
let module_info: naga::valid::ModuleInfo =
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .subgroup_stages(naga::valid::ShaderStages::all())
    .subgroup_operations(naga::valid::SubgroupOperationSet::all())
    .validate(&module)?;

// Translate the module.
use naga::back::glsl;
let mut glsl_source = String::new();
glsl::Writer::new(
    &mut glsl_source,
    &module,
    &module_info,
    &glsl::Options::default(),
    &glsl::PipelineOptions {
        entry_point: "main_fs".into(),
        shader_stage: naga::ShaderStage::Fragment,
        multiview: None,
    },
    naga::proc::BoundsCheckPolicies::default(),
)?.write()?;

assert_eq!(glsl_source, "\
#version 310 es

precision highp float;
precision highp int;

layout(location = 0) out vec4 _fs2p_location0;

void main() {
    _fs2p_location0 = vec4(1.0, 1.0, 1.0, 1.0);
    return;
}

");

# Ok::<(), Box<dyn core::error::Error>>(())
```
*/

#![allow(
    clippy::new_without_default,
    clippy::unneeded_field_pattern,
    clippy::match_like_matches_macro,
    clippy::collapsible_if,
    clippy::derive_partial_eq_without_eq,
    clippy::needless_borrowed_reference,
    clippy::single_match,
    clippy::enum_variant_names
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_qualifications,
    clippy::pattern_type_mismatch,
    clippy::missing_const_for_fn,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::match_wildcard_for_single_variants
)]
#![deny(clippy::exit)]
#![cfg_attr(
    not(test),
    warn(
        clippy::dbg_macro,
        clippy::panic,
        clippy::print_stderr,
        clippy::print_stdout,
        clippy::todo
    )
)]
#![no_std]
#![forbid(unsafe_code)]

#[cfg(std)]
extern crate std;

extern crate alloc;

mod arena;
pub mod back;
pub mod common;
pub mod compact;
pub mod diagnostic_filter;
pub mod error;
pub mod front;
pub mod ir;
pub mod keywords;
mod non_max_u32;
pub mod proc;
mod racy_lock;
mod span;
pub mod valid;

use alloc::string::String;

pub use crate::arena::{Arena, Handle, Range, UniqueArena};
pub use crate::span::{SourceLocation, Span, SpanContext, WithSpan};

// TODO: Eliminate this re-export and migrate uses of `crate::Foo` to `use crate::ir; ir::Foo`.
pub use ir::*;

/// Width of a boolean type, in bytes.
pub const BOOL_WIDTH: Bytes = 1;

/// Width of abstract types, in bytes.
pub const ABSTRACT_WIDTH: Bytes = 8;

/// Hash map that is faster but not resilient to DoS attacks.
/// (Similar to rustc_hash::FxHashMap but using hashbrown::HashMap instead of alloc::collections::HashMap.)
/// To construct a new instance: `FastHashMap::default()`
pub type FastHashMap<K, T> =
    hashbrown::HashMap<K, T, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

/// Hash set that is faster but not resilient to DoS attacks.
/// (Similar to rustc_hash::FxHashSet but using hashbrown::HashSet instead of alloc::collections::HashMap.)
pub type FastHashSet<K> =
    hashbrown::HashSet<K, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

/// Insertion-order-preserving hash set (`IndexSet<K>`), but with the same
/// hasher as `FastHashSet<K>` (faster but not resilient to DoS attacks).
pub type FastIndexSet<K> =
    indexmap::IndexSet<K, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

/// Insertion-order-preserving hash map (`IndexMap<K, V>`), but with the same
/// hasher as `FastHashMap<K, V>` (faster but not resilient to DoS attacks).
pub type FastIndexMap<K, V> =
    indexmap::IndexMap<K, V, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

/// Map of expressions that have associated variable names
pub(crate) type NamedExpressions = FastIndexMap<Handle<Expression>, String>;
