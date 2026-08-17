//! The abstractions that make up the core of Taffy's low-level API
//!
//! ## Examples
//!
//! The following examples demonstrate end-to-end implementation of Taffy's traits and usage of the low-level compute APIs:
//!
//!   - [custom_tree_vec](https://github.com/DioxusLabs/taffy/blob/main/examples/custom_tree_vec.rs) which implements a custom Taffy tree using a `Vec` as an arena with NodeId's being index's into the Vec.
//!   - [custom_tree_owned_partial](https://github.com/DioxusLabs/taffy/blob/main/examples/custom_tree_owned_partial.rs) which implements a custom Taffy tree using directly owned children with NodeId's being index's into vec on parent node.
//!   - [custom_tree_owned_unsafe](https://github.com/DioxusLabs/taffy/blob/main/examples/custom_tree_owned_unsafe.rs) which implements a custom Taffy tree using directly owned children with NodeId's being pointers.
//!
//! ## Overview
//!
//! ### Trait dependency tree
//!
//! The tree below illustrates which traits depend on which other traits.
//!
//! ```text
//! TraversePartialTree     - Access a node's children
//! ├──  LayoutPartialTree  - Run layout algorithms on a node and it's direct children
//! └──  TraverseTree       - Recursively access a node's descendants
//!     ├──  RoundTree      - Round a float-valued`  layout to integer pixels
//!     └──  PrintTree      - Print a debug representation of a node tree
//! ```
//!
//! ### A table of traits
//!
//! | Trait                 | Requires                | Enables                                                                                                                                                                                                                                                                                                                                                                                                                   |
//! | ---                   | ---                     | ---                                                                                                                                                                                                                                                                                                                                                                                                                       |
//! | [`LayoutPartialTree`] | [`TraversePartialTree`] | [`compute_flexbox_layout`](crate::compute_flexbox_layout)<br />[`compute_grid_layout`](crate::compute_grid_layout)<br />[`compute_block_layout`](crate::compute_block_layout)<br />[`compute_root_layout`](crate::compute_root_layout)<br />[`compute_leaf_layout`](crate::compute_leaf_layout)<br />[`compute_hidden_layout`](crate::compute_hidden_layout)<br />[`compute_cached_layout`](crate::compute_cached_layout) |
//! | [`RoundTree`]         | [`TraverseTree`]        | [`round_layout`](crate::round_layout)                                                                                                                                                                                                                                                                                                                                                                                     |
//! | [`PrintTree`]         | [`TraverseTree`]        | [`print_tree`](crate::print_tree)                                                                                                                                                                                                                                                                                                                                                                                         |
//!
//! ## All of the traits on one page
//!
//! ### TraversePartialTree and TraverseTree
//! These traits are Taffy's abstraction for downward tree traversal:
//!  - [`TraversePartialTree`] allows access to a single container node, and it's immediate children. This is the only "traverse" trait that is required
//!    for use of Taffy's core layout algorithms (flexbox, grid, etc).
//!  - [`TraverseTree`] is a marker trait which uses the same API signature as `TraversePartialTree`, but extends it with a guarantee that the child/children methods can be used to recurse
//!    infinitely down the tree. It is required by the `RoundTree` and
//!    the `PrintTree` traits.
//! ```rust
//! # use taffy::*;
//! pub trait TraversePartialTree {
//!     /// Type representing an iterator of the children of a node
//!     type ChildIter<'a>: Iterator<Item = NodeId>
//!     where
//!         Self: 'a;
//!
//!     /// Get the list of children IDs for the given node
//!     fn child_ids(&self, parent_node_id: NodeId) -> Self::ChildIter<'_>;
//!
//!     /// Get the number of children for the given node
//!     fn child_count(&self, parent_node_id: NodeId) -> usize;
//!
//!     /// Get a specific child of a node, where the index represents the nth child
//!     fn get_child_id(&self, parent_node_id: NodeId, child_index: usize) -> NodeId;
//! }
//!
//! pub trait TraverseTree: TraversePartialTree {}
//! ```
//!
//! You must implement [`TraversePartialTree`] to access any of Taffy's low-level API. If your tree implementation allows you to implement [`TraverseTree`] with
//! the correct semantics (full recursive traversal is available) then you should.
//!
//! ### LayoutPartialTree
//!
//! **Requires:** `TraversePartialTree`<br />
//! **Enables:** Flexbox, Grid, Block and Leaf layout algorithms from the [`crate::compute`] module
//!
//! Any type that implements [`LayoutPartialTree`] can be laid out using [Taffy's algorithms](crate::compute)
//!
//! Note that this trait extends [`TraversePartialTree`] (not [`TraverseTree`]). Taffy's algorithm implementations have been designed such that they can be used for a laying out a single
//! node that only has access to it's immediate children.
//!
//! ```rust
//! # use taffy::*;
//! pub trait LayoutPartialTree: TraversePartialTree {
//!     /// Get a reference to the [`Style`] for this node.
//!     fn get_style(&self, node_id: NodeId) -> &Style;
//!
//!     /// Set the node's unrounded layout
//!     fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout);
//!
//!     /// Get a mutable reference to the [`Cache`] for this node.
//!     fn get_cache_mut(&mut self, node_id: NodeId) -> &mut Cache;
//!
//!     /// Compute the specified node's size or full layout given the specified constraints
//!     fn compute_child_layout(&mut self, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput;
//! }
//! ```
//!
//! ### RoundTree
//!
//! **Requires:** `TraverseTree`
//!
//! Trait used by the `round_layout` method which takes a tree of unrounded float-valued layouts and performs
//! rounding to snap the values to the pixel grid.
//!
//! As indicated by it's dependence on `TraverseTree`, it required full recursive access to the tree.
//!
//! ```rust
//! # use taffy::*;
//! pub trait RoundTree: TraverseTree {
//!     /// Get the node's unrounded layout
//!     fn get_unrounded_layout(&self, node_id: NodeId) -> Layout;
//!     /// Get a reference to the node's final layout
//!     fn set_final_layout(&mut self, node_id: NodeId, layout: &Layout);
//! }
//! ```
//!
//! ### PrintTree
//!
//! **Requires:** `TraverseTree`
//!
//! ```rust
//! /// Trait used by the `print_tree` method which prints a debug representation
//! ///
//! /// As indicated by it's dependence on `TraverseTree`, it required full recursive access to the tree.
//! # use taffy::*;
//! pub trait PrintTree: TraverseTree {
//!     /// Get a debug label for the node (typically the type of node: flexbox, grid, text, image, etc)
//!     fn get_debug_label(&self, node_id: NodeId) -> &'static str;
//!     /// Get a reference to the node's final layout
//!     fn get_final_layout(&self, node_id: NodeId) -> Layout;
//! }
//! ```
//!
use super::{Layout, LayoutInput, LayoutOutput, NodeId, RequestedAxis, RunMode, SizingMode};
#[cfg(feature = "detailed_layout_info")]
use crate::debug::debug_log;
use crate::geometry::{AbsoluteAxis, Line, Size};
use crate::style::{AvailableSpace, CoreStyle};
#[cfg(feature = "flexbox")]
use crate::style::{FlexboxContainerStyle, FlexboxItemStyle};
#[cfg(feature = "grid")]
use crate::style::{GridContainerStyle, GridItemStyle};
use crate::CheapCloneStr;
#[cfg(feature = "block_layout")]
use crate::{BlockContainerStyle, BlockContext, BlockItemStyle};

#[cfg(all(feature = "grid", feature = "detailed_layout_info"))]
use crate::compute::grid::DetailedGridInfo;

/// Taffy's abstraction for downward tree traversal.
///
/// However, this trait does *not* require access to any node's other than a single container node's immediate children unless you also intend to implement `TraverseTree`.
pub trait TraversePartialTree {
    /// Type representing an iterator of the children of a node
    type ChildIter<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    /// Get the list of children IDs for the given node
    fn child_ids(&self, parent_node_id: NodeId) -> Self::ChildIter<'_>;

    /// Get the number of children for the given node
    fn child_count(&self, parent_node_id: NodeId) -> usize;

    /// Get a specific child of a node, where the index represents the nth child
    fn get_child_id(&self, parent_node_id: NodeId, child_index: usize) -> NodeId;
}

/// A marker trait which extends `TraversePartialTree`
///
/// Implementing this trait implies the additional guarantee that the child/children methods can be used to recurse
/// infinitely down the tree. Is required by the `RoundTree` and the `PrintTree` traits.
pub trait TraverseTree: TraversePartialTree {}

/// Any type that implements [`LayoutPartialTree`] can be laid out using [Taffy's algorithms](crate::compute)
///
/// Note that this trait extends [`TraversePartialTree`] (not [`TraverseTree`]). Taffy's algorithm implementations have been designed such that they can be used for a laying out a single
/// node that only has access to it's immediate children.
pub trait LayoutPartialTree: TraversePartialTree {
    /// The style type representing the core container styles that all containers should have
    /// Used when laying out the root node of a tree
    type CoreContainerStyle<'a>: CoreStyle<CustomIdent = Self::CustomIdent>
    where
        Self: 'a;

    /// String type for representing "custom identifiers" (for example, named grid lines or areas)
    /// If you are unsure what to use here then consider `Arc<str>`.
    type CustomIdent: CheapCloneStr;

    /// Get core style
    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_>;

    /// Resolve calc value
    #[inline(always)]
    fn resolve_calc_value(&self, val: *const (), basis: f32) -> f32 {
        let _ = val;
        let _ = basis;
        0.0
    }

    /// Set the node's unrounded layout
    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout);

    /// Compute the specified node's size or full layout given the specified constraints
    fn compute_child_layout(&mut self, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput;
}

/// Trait used by the `compute_cached_layout` method which allows cached layout results to be stored and retrieved.
///
/// The `Cache` struct implements a per-node cache that is compatible with this trait.
pub trait CacheTree {
    /// Try to retrieve a cached result from the cache
    fn cache_get(&self, node_id: NodeId, input: &LayoutInput) -> Option<LayoutOutput>;

    /// Store a computed size in the cache
    fn cache_store(&mut self, node_id: NodeId, input: &LayoutInput, layout_output: LayoutOutput);

    /// Clear all cache entries for the node
    fn cache_clear(&mut self, node_id: NodeId);
}

/// Trait used by the `round_layout` method which takes a tree of unrounded float-valued layouts and performs
/// rounding to snap the values to the pixel grid.
///
/// As indicated by it's dependence on `TraverseTree`, it required full recursive access to the tree.
pub trait RoundTree: TraverseTree {
    /// Get the node's unrounded layout
    fn get_unrounded_layout(&self, node_id: NodeId) -> Layout;
    /// Get a reference to the node's final layout
    fn set_final_layout(&mut self, node_id: NodeId, layout: &Layout);
}

/// Trait used by the `print_tree` method which prints a debug representation
///
/// As indicated by it's dependence on `TraverseTree`, it required full recursive access to the tree.
pub trait PrintTree: TraverseTree {
    /// Get a debug label for the node (typically the type of node: flexbox, grid, text, image, etc)
    fn get_debug_label(&self, node_id: NodeId) -> &'static str;
    /// Get a reference to the node's final layout
    fn get_final_layout(&self, node_id: NodeId) -> Layout;
}

#[cfg(feature = "flexbox")]
/// Extends [`LayoutPartialTree`] with getters for the styles required for Flexbox layout
pub trait LayoutFlexboxContainer: LayoutPartialTree {
    /// The style type representing the Flexbox container's styles
    type FlexboxContainerStyle<'a>: FlexboxContainerStyle
    where
        Self: 'a;
    /// The style type representing each Flexbox item's styles
    type FlexboxItemStyle<'a>: FlexboxItemStyle
    where
        Self: 'a;

    /// Get the container's styles
    fn get_flexbox_container_style(&self, node_id: NodeId) -> Self::FlexboxContainerStyle<'_>;

    /// Get the child's styles
    fn get_flexbox_child_style(&self, child_node_id: NodeId) -> Self::FlexboxItemStyle<'_>;
}

#[cfg(feature = "grid")]
/// Extends [`LayoutPartialTree`] with getters for the styles required for CSS Grid layout
pub trait LayoutGridContainer: LayoutPartialTree {
    /// The style type representing the CSS Grid container's styles
    type GridContainerStyle<'a>: GridContainerStyle<CustomIdent = Self::CustomIdent>
    where
        Self: 'a;

    /// The style type representing each CSS Grid item's styles
    type GridItemStyle<'a>: GridItemStyle<CustomIdent = Self::CustomIdent>
    where
        Self: 'a;

    /// Get the container's styles
    fn get_grid_container_style(&self, node_id: NodeId) -> Self::GridContainerStyle<'_>;

    /// Get the child's styles
    fn get_grid_child_style(&self, child_node_id: NodeId) -> Self::GridItemStyle<'_>;

    /// Set the node's detailed grid information
    ///
    /// Implementing this method is optional. Doing so allows you to access details about the the grid such as
    /// the computed size of each grid track and the computed placement of each grid item.
    #[cfg(feature = "detailed_layout_info")]
    fn set_detailed_grid_info(&mut self, _node_id: NodeId, _detailed_grid_info: DetailedGridInfo) {
        debug_log!("LayoutGridContainer::set_detailed_grid_info called");
    }
}

#[cfg(feature = "block_layout")]
/// Extends [`LayoutPartialTree`] with getters for the styles required for CSS Block layout
pub trait LayoutBlockContainer: LayoutPartialTree {
    /// The style type representing the CSS Block container's styles
    type BlockContainerStyle<'a>: BlockContainerStyle
    where
        Self: 'a;
    /// The style type representing each CSS Block item's styles
    type BlockItemStyle<'a>: BlockItemStyle
    where
        Self: 'a;

    /// Get the container's styles
    fn get_block_container_style(&self, node_id: NodeId) -> Self::BlockContainerStyle<'_>;

    /// Get the child's styles
    fn get_block_child_style(&self, child_node_id: NodeId) -> Self::BlockItemStyle<'_>;

    /// Compute the specified node's size or full layout given the specified constraints
    #[cfg(feature = "block_layout")]
    fn compute_block_child_layout(
        &mut self,
        node_id: NodeId,
        inputs: LayoutInput,
        block_ctx: Option<&mut BlockContext<'_>>,
    ) -> LayoutOutput {
        let _ = block_ctx;
        self.compute_child_layout(node_id, inputs)
    }
}

// --- PRIVATE TRAITS

/// A private trait which allows us to add extra convenience methods to types which implement
/// LayoutTree without making those methods public.
pub(crate) trait LayoutPartialTreeExt: LayoutPartialTree {
    /// Compute the size of the node given the specified constraints
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn measure_child_size(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
        axis: AbsoluteAxis,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> f32 {
        self.compute_child_layout(
            node_id,
            LayoutInput {
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                axis: axis.into(),
                run_mode: RunMode::ComputeSize,
                vertical_margins_are_collapsible,
            },
        )
        .size
        .get_abs(axis)
    }

    /// Compute the size of the node given the specified constraints
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn measure_child_size_both(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> Size<f32> {
        self.compute_child_layout(
            node_id,
            LayoutInput {
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                axis: RequestedAxis::Both,
                run_mode: RunMode::ComputeSize,
                vertical_margins_are_collapsible,
            },
        )
        .size
    }

    /// Perform a full layout on the node given the specified constraints
    #[inline(always)]
    fn perform_child_layout(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> LayoutOutput {
        self.compute_child_layout(
            node_id,
            LayoutInput {
                known_dimensions,
                parent_size,
                available_space,
                sizing_mode,
                axis: RequestedAxis::Both,
                run_mode: RunMode::PerformLayout,
                vertical_margins_are_collapsible,
            },
        )
    }

    /// Alias to `resolve_calc_value` with a shorter function name
    #[inline(always)]
    #[cfg(feature = "calc")]
    fn calc(&self, val: *const (), basis: f32) -> f32 {
        self.resolve_calc_value(val, basis)
    }

    /// Alias to `resolve_calc_value` with a shorter function name
    #[inline(always)]
    #[cfg(not(feature = "calc"))]
    fn calc(&self, _val: *const (), _basis: f32) -> f32 {
        0.0
    }
}

impl<T: LayoutPartialTree> LayoutPartialTreeExt for T {}
