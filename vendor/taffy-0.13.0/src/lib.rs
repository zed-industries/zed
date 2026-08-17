//! # Taffy
//!
//! Taffy is a flexible, high-performance library for **UI layout**.
//! It currently implements the Flexbox, Grid and Block layout algorithms from the CSS specification. Support for other paradigms is planned.
//! For more information on this and other future development plans see the [roadmap issue](https://github.com/DioxusLabs/taffy/issues/345).
//!
//! ## Architecture
//!
//! Taffy is based on a tree of "UI nodes" similar to the tree of DOM nodes that one finds in web-based UI. Each node has:
//!   - A [`Style`] struct which holds a set of CSS styles which function as the primary input to the layout computations.
//!   - A [`Layout`] struct containing a position (x/y) and a size (width/height) which function as the output of the layout computations.
//!   - Optionally:
//!       - A `Vec` set of child nodes
//!       - "Context": arbitrary user-defined data (which you can access when using a "measure function" to integrate Taffy with other kinds of layout such as text layout)
//!
//! Usage of Taffy consists of constructing a tree of UI nodes (with associated styles, children and context), then calling function(s)
//! from Taffy to translate those styles, parent-child relationships and measure functions into a size and position in 2d space for each node
//! in the tree.
//!
//! ## High-level API vs. Low-level API
//!
//! Taffy has two APIs: a high-level API that is simpler and easier to get started with, and a low-level API that is more flexible gives greater control.
//! We would generally recommend the high-level API for users using Taffy standalone and the low-level API for users wanting to embed Taffy as part of
//! a wider layout system or as part of a UI framework that already has it's own node/widget tree representation.
//!
//! ### High-level API
//!
//! The high-level API consists of the [`TaffyTree`] struct which contains a tree implementation and provides methods that allow you to construct
//! a tree of UI nodes. Once constructed, you can call the [`compute_layout_with_measure`](crate::TaffyTree::compute_layout_with_measure) method to compute the layout (passing in a "measure function" closure which is used to compute the size of leaf nodes), and then access
//! the layout of each node using the [`layout`](crate::TaffyTree::layout) method.
//!
//! When using the high-level API, Taffy will take care of node storage, caching and dispatching to the correct layout algorithm for a given node for you.
//! See the [`TaffyTree`] struct for more details on this API.
//!
//! Examples which show usage of the high-level API include:
//!
//!   - [basic](https://github.com/DioxusLabs/taffy/blob/main/examples/basic.rs)
//!   - [flexbox_gap](https://github.com/DioxusLabs/taffy/blob/main/examples/flexbox_gap.rs)
//!   - [grid_holy_grail](https://github.com/DioxusLabs/taffy/blob/main/examples/grid_holy_grail.rs)
//!   - [measure](https://github.com/DioxusLabs/taffy/blob/main/examples/measure.rs)
//!   - [cosmic_text](https://github.com/DioxusLabs/taffy/blob/main/examples/cosmic_text.rs)
//!
//! In particular, the "measure" example shows how to integrate Taffy layout with other layout modalities such as text or image layout when using the high level API.
//!
//! ### Low-level API
//!
//! The low-level API consists of a [set of traits](crate::tree::traits) (notably the [`LayoutPartialTree`] trait) which define an interface behind which you must implement your own
//! tree implementation, and a [set of functions](crate::compute) such as [`compute_flexbox_layout`] and [`compute_grid_layout`] which implement the layout algorithms (for a single node at a time), and are designed to be flexible
//! and easy to integrate into a wider layout or UI system.
//!
//! When using this API, you must handle node storage, caching, and dispatching to the correct layout algorithm for a given node yourself.
//! See the [`crate::tree::traits`] module for more details on this API.
//!
//! Examples which show usage of the low-level API are:
//!
//!   - [custom_tree_vec](https://github.com/DioxusLabs/taffy/blob/main/examples/custom_tree_vec.rs) which implements a custom Taffy tree using a `Vec` as an arena with NodeId's being index's into the Vec.
//!   - [custom_tree_owned_partial](https://github.com/DioxusLabs/taffy/blob/main/examples/custom_tree_owned_partial.rs) which implements a custom Taffy tree using directly owned children with NodeId's being index's into vec on parent node.
//!   - [custom_tree_owned_unsafe](https://github.com/DioxusLabs/taffy/blob/main/examples/custom_tree_owned_unsafe.rs) which implements a custom Taffy tree using directly owned children with NodeId's being pointers.

// document the feature flags for the crate by extracting the comments from Cargo.toml
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
// annotate items with their required features (gated by docsrs flag as this requires the nightly toolchain)
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
// Disable "unused_x" warnings when default features aren't enabled.
#![cfg_attr(not(feature = "default"), allow(dead_code))]
#![cfg_attr(not(feature = "default"), allow(unused_imports))]
#![cfg_attr(not(feature = "default"), allow(unused_variables))]
#![cfg_attr(not(feature = "default"), allow(unused_mut))]

// We always need std for the tests
// See <https://github.com/la10736/rstest/issues/149#issuecomment-1156402989>
#[cfg(all(test, not(feature = "std")))]
#[macro_use]
extern crate std;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[cfg_attr(feature = "serde", macro_use)]
#[cfg(feature = "serde")]
extern crate serde;

pub mod compute;
pub mod geometry;
pub mod prelude;
pub mod style;
pub mod style_helpers;
pub mod tree;
#[macro_use]
pub mod util;

mod readme_doctest {
    #![doc = include_str!("../README.md")]
}

#[cfg(feature = "block_layout")]
#[doc(inline)]
pub use crate::compute::compute_block_layout;
#[cfg(feature = "flexbox")]
#[doc(inline)]
pub use crate::compute::compute_flexbox_layout;
#[cfg(feature = "grid")]
#[doc(inline)]
pub use crate::compute::compute_grid_layout;
#[cfg(feature = "detailed_layout_info")]
pub use crate::compute::detailed_info::*;
#[doc(inline)]
pub use crate::compute::{
    compute_cached_layout, compute_hidden_layout, compute_leaf_layout, compute_root_layout, round_layout,
};
#[doc(inline)]
pub use crate::style::Style;
#[doc(inline)]
pub use crate::tree::traits::*;
#[cfg(feature = "taffy_tree")]
#[doc(inline)]
pub use crate::tree::TaffyTree;
#[cfg(feature = "std")]
#[doc(inline)]
pub use crate::util::print_tree;

#[cfg(feature = "parse")]
pub use parse::{ParseError, ParseResult};

pub use crate::compute::*;
pub use crate::geometry::*;
pub use crate::style::*;
pub use crate::tree::*;
pub use crate::util::*;
