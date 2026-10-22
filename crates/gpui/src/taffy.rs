use crate::{
    AbsoluteLength, App, Bounds, DefiniteLength, Edges, GridTemplate, Length, Pixels, Point, Size,
    Style, Window, size,
    util::{
        ceil_to_device_pixel, round_half_toward_zero, round_stroke_to_device_pixel,
        round_to_device_pixel,
    },
};
use collections::{FxHashMap, FxHashSet};
use slotmap::{DefaultKey, SecondaryMap};
use std::{fmt::Debug, ops::Range};
use taffy::{
    TaffyTree, TraversePartialTree as _,
    geometry::{Point as TaffyPoint, Rect as TaffyRect, Size as TaffySize},
    prelude::{max_content, min_content},
    style::AvailableSpace as TaffyAvailableSpace,
    tree::NodeId,
};

#[cfg(feature = "stacker")]
type StackSafe<T> = stacksafe::StackSafe<T>;
#[cfg(not(feature = "stacker"))]
type StackSafe<T> = T;

type MeasureFn =
    dyn FnMut(Size<Option<Pixels>>, Size<AvailableSpace>, &mut Window, &mut App) -> Size<Pixels>;
type NodeMeasureFn = StackSafe<Box<MeasureFn>>;

enum NodeContext {
    Custom(NodeMeasureFn),
    OwnedCustom(Box<OwnedMeasure>),
    Text(Box<crate::elements::TextMeasure>),
}

struct OwnedMeasure {
    measure: NodeMeasureFn,
    occurrence: crate::view::ViewOccurrenceId,
}

struct LayoutCache {
    scope_indices: FxHashMap<crate::element::InternedHandle, usize>,
    scopes: Vec<Option<LayoutScope>>,
    free_scopes: Vec<usize>,
    visited_scopes: Vec<usize>,
    request_scope: Option<usize>,
    epoch: u64,
    nodes: Vec<NodeEpochs>,
    // Retained epoch walks must not pull the large input keys into cache.
    layout_inputs: Vec<Option<LayoutInputKey>>,
    computed_roots: Vec<LayoutId>,
    font_generation: Option<usize>,
}

impl Default for LayoutCache {
    fn default() -> Self {
        Self {
            scope_indices: FxHashMap::default(),
            scopes: Vec::new(),
            free_scopes: Vec::new(),
            visited_scopes: Vec::new(),
            request_scope: None,
            epoch: 1,
            nodes: Vec::new(),
            layout_inputs: Vec::new(),
            computed_roots: Vec::new(),
            font_generation: None,
        }
    }
}

#[derive(Default)]
struct NodeEpochs {
    last_requested: u64,
    last_reachable: u64,
    last_computed: u64,
    last_retained: u64,
    owner: (usize, usize),
}

fn node_index(id: NodeId) -> usize {
    // Taffy's NodeId stores slotmap KeyData: the low 32 bits are the slot.
    // A newly claimed slot resets its stamps, even if its generation changed.
    u64::from(id) as u32 as usize
}

struct LayoutScope {
    handle: crate::element::InternedHandle,
    nodes: Vec<Option<LayoutId>>,
    cursor: usize,
    last_requested_epoch: u64,
}

#[derive(Default)]
struct RetainedLayouts {
    layouts: SecondaryMap<DefaultKey, taffy::Layout>,
    hoisted_children: SecondaryMap<DefaultKey, Vec<NodeId>>,
    detailed_layout_info: SecondaryMap<DefaultKey, taffy::DetailedLayoutInfo>,
    previous_layouts: FxHashMap<NodeId, taffy::Layout>,
    boundaries: FxHashMap<NodeId, BoundaryLayout>,
}

#[derive(Default)]
struct BoundaryLayout {
    input: Option<taffy::LayoutInput>,
    output: Option<taffy::LayoutOutput>,
}

impl RetainedLayouts {
    fn set_layout(&mut self, node: NodeId, layout: taffy::Layout) {
        if let Some(previous) = self.layouts.insert(node.into(), layout)
            && previous != layout
        {
            self.previous_layouts.entry(node).or_insert(previous);
        }
    }
}

// TaffyTree's high-level API hides container inputs. Retention needs the exact
// inputs, not a size-derived approximation (flex, grid and percentages differ).
struct RetainedLayoutTree<'a, F> {
    taffy: &'a mut TaffyTree<NodeContext>,
    retained: &'a mut RetainedLayouts,
    measure: F,
}

impl<F> taffy::TraversePartialTree for RetainedLayoutTree<'_, F> {
    type ChildIter<'a>
        = <TaffyTree<NodeContext> as taffy::TraversePartialTree>::ChildIter<'a>
    where
        Self: 'a;

    fn child_ids(&self, node: NodeId) -> Self::ChildIter<'_> {
        self.taffy.child_ids(node)
    }

    fn child_count(&self, node: NodeId) -> usize {
        self.taffy.child_count(node)
    }

    fn get_child_id(&self, node: NodeId, index: usize) -> NodeId {
        self.taffy.get_child_id(node, index)
    }
}

impl<F> taffy::CacheTree for RetainedLayoutTree<'_, F> {
    fn cache_get(
        &mut self,
        node: NodeId,
        input: &taffy::LayoutInput,
    ) -> Option<taffy::LayoutOutput> {
        taffy::CacheTree::cache_get(self.taffy, node, input)
    }

    fn cache_store(
        &mut self,
        node: NodeId,
        input: &taffy::LayoutInput,
        output: taffy::LayoutOutput,
    ) {
        taffy::CacheTree::cache_store(self.taffy, node, input, output);
    }

    fn cache_clear(&mut self, node: NodeId) {
        taffy::CacheTree::cache_clear(self.taffy, node);
    }
}

impl<F> RetainedLayoutTree<'_, F>
where
    F: FnMut(
        taffy::LayoutInput,
        NodeId,
        Option<&mut NodeContext>,
        &taffy::Style,
    ) -> taffy::LayoutOutput,
{
    fn compute(
        &mut self,
        node: NodeId,
        input: taffy::LayoutInput,
        block: Option<&mut taffy::BlockContext<'_>>,
    ) -> taffy::LayoutOutput {
        if input.run_mode == taffy::RunMode::PerformLayout
            && let Some(boundary) = self.retained.boundaries.get_mut(&node)
        {
            boundary.input = Some(input);
        }
        if input.run_mode == taffy::RunMode::PerformHiddenLayout {
            return taffy::compute_hidden_layout(self, node);
        }
        let output = taffy::compute_cached_layout(self, node, input, |tree, node, input| {
            let style = tree.taffy.style(node).expect(EXPECT_MESSAGE);
            let mut output = if style.display == taffy::Display::None {
                taffy::compute_hidden_layout(tree, node)
            } else if tree.taffy.child_count(node) == 0 {
                let style = style.clone();
                (tree.measure)(input, node, tree.taffy.get_node_context_mut(node), &style)
            } else {
                match style.display {
                    taffy::Display::Flex => taffy::compute_flexbox_layout(tree, node, input),
                    taffy::Display::Grid => taffy::compute_grid_layout(tree, node, input),
                    taffy::Display::Block => taffy::compute_block_layout(tree, node, input, block),
                    taffy::Display::FlowRoot => {
                        taffy::compute_block_layout(tree, node, input, None)
                    }
                    taffy::Display::None => unreachable!(),
                }
            };
            if input.run_mode == taffy::RunMode::PerformLayout {
                taffy::compute_oof_layout(tree, node, &mut output);
            }
            output
        });
        if input.run_mode == taffy::RunMode::PerformLayout
            && let Some(boundary) = self.retained.boundaries.get_mut(&node)
        {
            boundary.output = Some(output.clone());
        }
        output
    }
}

impl<F> taffy::LayoutPartialTree for RetainedLayoutTree<'_, F>
where
    F: FnMut(
        taffy::LayoutInput,
        NodeId,
        Option<&mut NodeContext>,
        &taffy::Style,
    ) -> taffy::LayoutOutput,
{
    type CoreContainerStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;
    type CustomIdent = String;

    fn get_core_container_style(&self, node: NodeId) -> Self::CoreContainerStyle<'_> {
        self.taffy.style(node).expect(EXPECT_MESSAGE)
    }

    fn set_unrounded_layout(&mut self, node: NodeId, layout: &taffy::Layout) {
        self.retained.set_layout(node, *layout);
    }

    fn compute_child_layout(
        &mut self,
        node: NodeId,
        input: taffy::LayoutInput,
    ) -> taffy::LayoutOutput {
        self.compute(node, input, None)
    }
}

impl<F> taffy::LayoutBlockContainer for RetainedLayoutTree<'_, F>
where
    F: FnMut(
        taffy::LayoutInput,
        NodeId,
        Option<&mut NodeContext>,
        &taffy::Style,
    ) -> taffy::LayoutOutput,
{
    type BlockContainerStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;
    type BlockItemStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;
    fn get_block_container_style(&self, node: NodeId) -> Self::BlockContainerStyle<'_> {
        self.taffy.style(node).expect(EXPECT_MESSAGE)
    }
    fn get_block_child_style(&self, node: NodeId) -> Self::BlockItemStyle<'_> {
        self.taffy.style(node).expect(EXPECT_MESSAGE)
    }
    fn compute_block_child_layout(
        &mut self,
        node: NodeId,
        input: taffy::LayoutInput,
        block: Option<&mut taffy::BlockContext<'_>>,
    ) -> taffy::LayoutOutput {
        self.compute(node, input, block)
    }
}

impl<F> taffy::LayoutContainingBlock for RetainedLayoutTree<'_, F>
where
    Self: taffy::LayoutPartialTree<CustomIdent = String>,
{
    type OofItemStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;

    fn get_oof_item_style(&self, node: NodeId) -> Self::OofItemStyle<'_> {
        self.taffy.style(node).expect(EXPECT_MESSAGE)
    }

    fn clear_hoisted_children(&mut self, node: NodeId) {
        self.retained
            .hoisted_children
            .insert(node.into(), Vec::new());
    }

    fn add_hoisted_children(&mut self, node: NodeId, hoisted: &[NodeId]) {
        self.retained
            .hoisted_children
            .entry(node.into())
            .expect(EXPECT_MESSAGE)
            .or_default()
            .extend_from_slice(hoisted);
    }

    fn get_detailed_layout_info(&self, node: NodeId) -> &taffy::DetailedLayoutInfo {
        self.retained
            .detailed_layout_info
            .get(node.into())
            .unwrap_or(&taffy::DetailedLayoutInfo::None)
    }
}

impl<F> taffy::LayoutFlexboxContainer for RetainedLayoutTree<'_, F>
where
    Self: taffy::LayoutPartialTree,
{
    type FlexboxContainerStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;
    type FlexboxItemStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;
    fn get_flexbox_container_style(&self, node: NodeId) -> Self::FlexboxContainerStyle<'_> {
        self.taffy.style(node).expect(EXPECT_MESSAGE)
    }
    fn get_flexbox_child_style(&self, node: NodeId) -> Self::FlexboxItemStyle<'_> {
        self.taffy.style(node).expect(EXPECT_MESSAGE)
    }
}

impl<F> taffy::LayoutGridContainer for RetainedLayoutTree<'_, F>
where
    Self: taffy::LayoutPartialTree<CustomIdent = String>,
{
    type GridContainerStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;
    type GridItemStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;
    fn get_grid_container_style(&self, node: NodeId) -> Self::GridContainerStyle<'_> {
        self.taffy.style(node).expect(EXPECT_MESSAGE)
    }
    fn get_grid_child_style(&self, node: NodeId) -> Self::GridItemStyle<'_> {
        self.taffy.style(node).expect(EXPECT_MESSAGE)
    }

    fn set_detailed_grid_info(&mut self, node: NodeId, info: taffy::DetailedGridInfo) {
        self.retained
            .detailed_layout_info
            .insert(node.into(), taffy::DetailedLayoutInfo::Grid(Box::new(info)));
    }
}

pub struct TaffyLayoutEngine {
    taffy: TaffyTree<NodeContext>,
    cache: Option<Box<LayoutCache>>,
    absolute_layout: SecondaryMap<DefaultKey, AbsoluteLayout>,
    layout_epoch: u64,
    retained: Option<Box<RetainedLayouts>>,
    computed_layouts: FxHashSet<LayoutId>,
    layout_bounds_scratch_space: Vec<LayoutId>,
    layout_changes: u64,
    window_root: Option<(LayoutId, Size<AvailableSpace>, f32, u64)>,
}

#[derive(Clone, Copy)]
struct AbsoluteLayout {
    epoch: u64,
    bounds: Bounds<Pixels>,
    /// Descendants need the unrounded device-pixel origin, not the snapped bounds.
    outer_origin: Point<f32>,
}

const EXPECT_MESSAGE: &str = "we should avoid taffy layout errors by construction if possible";

fn text_layout_style() -> taffy::Style {
    taffy::Style {
        display: taffy::Display::Block,
        position: taffy::Position::Relative,
        ..Default::default()
    }
}

impl TaffyLayoutEngine {
    pub fn new() -> Self {
        let mut taffy = TaffyTree::new();
        taffy.disable_rounding();
        TaffyLayoutEngine {
            taffy,
            cache: None,
            absolute_layout: SecondaryMap::new(),
            layout_epoch: 0,
            retained: None,
            computed_layouts: FxHashSet::default(),
            layout_bounds_scratch_space: Vec::new(),
            layout_changes: 0,
            window_root: None,
        }
    }

    pub(crate) fn enable_cache(&mut self) {
        self.cache.get_or_insert_with(Box::default);
    }

    #[cfg(any(test, feature = "bench-support"))]
    pub(crate) fn cache_enabled(&self) -> bool {
        self.cache.is_some()
    }

    pub(crate) fn enable_retention(&mut self) {
        if self.retained.is_none() {
            self.retained = Some(Box::default());
        }
    }

    pub(crate) fn retained_style(&self, id: LayoutId) -> taffy::Style {
        self.taffy.style(id.0).expect(EXPECT_MESSAGE).clone()
    }

    pub(crate) fn retained_input(&self, id: LayoutId) -> Option<taffy::LayoutInput> {
        self.retained.as_ref()?.boundaries.get(&id.0)?.input
    }

    pub(crate) fn retained_output(&self, id: LayoutId) -> Option<taffy::LayoutOutput> {
        self.retained
            .as_ref()?
            .boundaries
            .get(&id.0)?
            .output
            .clone()
    }

    pub(crate) fn register_retained_boundary(&mut self, id: LayoutId) {
        if let Some(retained) = self.retained.as_mut() {
            retained.boundaries.entry(id.0).or_default();
        }
    }

    pub(crate) fn layout_changes(&self) -> u64 {
        self.layout_changes
    }

    pub(crate) fn accept_internal_layout_changes(&mut self, before: u64) {
        // Only a contiguous span of validated writes can advance the root's
        // frontier. An earlier parent-facing write must still force root layout.
        if let Some((_, _, _, accepted)) = self.window_root.as_mut()
            && *accepted >= before
        {
            *accepted = self.layout_changes;
        }
    }

    pub(crate) fn compute_window_root_layout(
        &mut self,
        id: LayoutId,
        available_space: Size<AvailableSpace>,
        allow_skip: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        let scale_factor = window.scale_factor();
        if allow_skip
            && self.retained.is_some()
            && self.window_root == Some((id, available_space, scale_factor, self.layout_changes))
            && self.cache.as_ref().is_some_and(|cache| {
                cache.font_generation == Some(window.text_system().font_generation())
            })
        {
            self.retain_subtree(id);
            #[cfg(any(test, feature = "bench-support"))]
            {
                window.view_reuse_stats.root_layout_skips += 1;
            }
            return;
        }
        #[cfg(any(test, feature = "bench-support"))]
        {
            window.view_reuse_stats.root_layout_computations += 1;
        }
        self.compute_layout(id, available_space, window, cx);
        self.window_root = Some((id, available_space, scale_factor, self.layout_changes));
    }

    pub(crate) fn retained_geometry_unchanged(
        &self,
        id: LayoutId,
        boundaries: &[LayoutId],
    ) -> bool {
        let Some(retained) = self.retained.as_ref() else {
            return false;
        };
        let mut stack = vec![id.0];
        while let Some(node) = stack.pop() {
            if let Some(previous) = retained.previous_layouts.get(&node)
                && retained.layouts.get(node.into()) != Some(previous)
            {
                return false;
            }
            if node == id.0 || !boundaries.contains(&LayoutId(node)) {
                stack.extend(self.taffy.child_ids(node));
            }
        }
        true
    }

    pub(crate) fn retain_subtree(&mut self, id: LayoutId) {
        let Some(cache) = self.cache.as_mut() else {
            return;
        };
        let epochs = &mut cache.nodes[node_index(id.0)];
        if epochs.last_computed != cache.epoch {
            epochs.last_computed = cache.epoch;
            cache.computed_roots.push(id);
        }
        let stack = &mut self.layout_bounds_scratch_space;
        stack.push(id);
        while let Some(node) = stack.pop() {
            let epochs = &mut cache.nodes[node_index(node.0)];
            if epochs.last_retained == cache.epoch {
                continue;
            }
            epochs.last_retained = cache.epoch;
            epochs.last_requested = cache.epoch;
            let (scope, ordinal) = epochs.owner;
            let scope = cache.scopes[scope].as_mut().expect("live layout scope");
            scope.last_requested_epoch = cache.epoch;
            scope.cursor = scope.cursor.max(ordinal + 1);
            stack.extend(self.taffy.child_ids(node.0).map(LayoutId));
        }
    }

    pub(crate) fn reset_retained_subtree(&mut self, id: LayoutId) {
        let Some(cache) = self.cache.as_mut() else {
            return;
        };
        let stack = &mut self.layout_bounds_scratch_space;
        stack.push(id);
        while let Some(node) = stack.pop() {
            let epochs = &mut cache.nodes[node_index(node.0)];
            epochs.last_retained = 0;
            cache.scopes[epochs.owner.0]
                .as_mut()
                .expect("live layout scope")
                .cursor = 0;
            stack.extend(self.taffy.child_ids(node.0).map(LayoutId));
        }
    }

    #[cfg(test)]
    pub(crate) fn cache_counts(&self) -> (usize, usize, usize) {
        let mut scope_count = 0;
        let mut text_count = 0;
        if let Some(cache) = &self.cache {
            for scope in cache.scopes.iter().flatten() {
                scope_count += 1;
                for id in scope.nodes.iter().flatten() {
                    if matches!(
                        self.taffy.get_node_context(id.0),
                        Some(NodeContext::Text(_))
                    ) {
                        text_count += 1;
                    }
                }
            }
        }
        (self.taffy.total_node_count(), text_count, scope_count)
    }

    #[cfg(test)]
    pub(crate) fn text_context_identity_and_dirty(
        &self,
        id: LayoutId,
    ) -> (Option<*const crate::elements::TextMeasure>, bool) {
        let identity = match self.taffy.get_node_context(id.0) {
            Some(NodeContext::Text(measure)) => Some(&**measure as *const _),
            _ => None,
        };
        (identity, self.taffy.dirty(id.0).expect(EXPECT_MESSAGE))
    }

    pub(crate) fn set_request_scope(&mut self, handle: &crate::element::InternedHandle) {
        let Some(cache) = self.cache.as_mut() else {
            return;
        };
        let index = *cache.scope_indices.entry(*handle).or_insert_with(|| {
            let scope = LayoutScope {
                handle: *handle,
                nodes: Vec::new(),
                cursor: 0,
                last_requested_epoch: 0,
            };
            let index = if let Some(index) = cache.free_scopes.pop() {
                cache.scopes[index] = Some(scope);
                index
            } else {
                cache.scopes.push(Some(scope));
                cache.scopes.len() - 1
            };
            cache.visited_scopes.push(index);
            index
        });
        cache.request_scope = Some(index);
    }

    pub fn clear(&mut self) {
        if let Some(cache) = self.cache.as_mut() {
            let stack = &mut self.layout_bounds_scratch_space;
            stack.append(&mut cache.computed_roots);
            while let Some(id) = stack.pop() {
                let epochs = &mut cache.nodes[node_index(id.0)];
                if epochs.last_reachable != cache.epoch {
                    epochs.last_reachable = cache.epoch;
                    stack.extend(self.taffy.child_ids(id.0).map(LayoutId));
                }
            }
            cache.visited_scopes.retain(|index| {
                let scope = cache.scopes[*index].as_mut().expect(EXPECT_MESSAGE);
                for node in &mut scope.nodes {
                    if let Some(id) = *node {
                        let epochs = &mut cache.nodes[node_index(id.0)];
                        if scope.last_requested_epoch != cache.epoch
                            || epochs.last_requested != cache.epoch
                            || epochs.last_reachable != cache.epoch
                        {
                            // Taffy 0.14's remove does not remove node context data.
                            self.taffy
                                .set_node_context(id.0, None)
                                .expect(EXPECT_MESSAGE);
                            self.taffy.remove(id.0).expect(EXPECT_MESSAGE);
                            self.absolute_layout.remove(id.0.into());
                            if let Some(retained) = self.retained.as_mut() {
                                retained.layouts.remove(id.0.into());
                                retained.hoisted_children.remove(id.0.into());
                                retained.detailed_layout_info.remove(id.0.into());
                                retained.boundaries.remove(&id.0);
                            }
                            *node = None;
                        }
                    }
                }
                scope.nodes.truncate(scope.cursor);
                scope.cursor = 0;
                if scope.nodes.iter().any(Option::is_some) {
                    true
                } else {
                    cache.scope_indices.remove(&scope.handle);
                    cache.scopes[*index] = None;
                    cache.free_scopes.push(*index);
                    false
                }
            });
            cache.request_scope = None;
            cache.epoch = cache.epoch.checked_add(1).expect("layout epoch overflow");
        } else {
            self.taffy.clear();
        }
        // Retained bounds survive frame teardown. Root computes invalidate the
        // whole epoch; in-place computes invalidate only the subtree they change.
        if self.retained.is_none() {
            self.advance_layout_epoch();
        }
        self.computed_layouts.clear();
        if let Some(retained) = self.retained.as_mut() {
            retained.previous_layouts.clear();
        }
    }

    fn advance_layout_epoch(&mut self) {
        self.layout_epoch = self.layout_epoch.wrapping_add(1);
        if self.layout_epoch == 0 {
            self.absolute_layout.clear();
        }
    }

    fn claim_node(&mut self, style: RequestedStyle, children: &[LayoutId]) -> (LayoutId, bool) {
        let scope = self.cache.as_mut().map(|cache| {
            let index = cache
                .request_scope
                .take()
                .expect("cached layout requests require an execution scope");
            let scope = cache.scopes[index].as_mut().expect(EXPECT_MESSAGE);
            scope.last_requested_epoch = cache.epoch;
            let ordinal = scope.cursor;
            scope.cursor += 1;
            if ordinal == scope.nodes.len() {
                scope.nodes.push(None);
            }
            (index, ordinal, scope.nodes[ordinal])
        });
        let existing = scope.and_then(|(_, _, id)| id);
        let layout_inputs = match &style {
            RequestedStyle::Inputs(inputs) => Some(inputs.clone()),
            RequestedStyle::Converted(_) => None,
        };
        let id = if let Some(id) = existing {
            let inputs_match = layout_inputs.as_ref().is_some_and(|inputs| {
                self.cache.as_ref().expect(EXPECT_MESSAGE).layout_inputs[node_index(id.0)].as_ref()
                    == Some(inputs)
            });
            if !inputs_match {
                let style = style.convert();
                if self.taffy.style(id.0).expect(EXPECT_MESSAGE) != &style {
                    self.taffy.set_style(id.0, style).expect(EXPECT_MESSAGE);
                    self.layout_changes += 1;
                }
            }
            if !self
                .taffy
                .child_ids(id.0)
                .eq(children.iter().map(|id| id.0))
            {
                self.taffy
                    .set_children(id.0, LayoutId::to_taffy_slice(children))
                    .expect(EXPECT_MESSAGE);
                self.layout_changes += 1;
            }
            id
        } else {
            self.layout_changes += 1;
            let id = self.taffy.new_leaf(style.convert()).expect(EXPECT_MESSAGE);
            if !children.is_empty() {
                // Unlike new_with_children, set_children detaches previous parents.
                self.taffy
                    .set_children(id, LayoutId::to_taffy_slice(children))
                    .expect(EXPECT_MESSAGE);
            }
            LayoutId(id)
        };
        if let Some((index, ordinal, _)) = scope {
            let cache = self.cache.as_mut().expect(EXPECT_MESSAGE);
            cache.scopes[index].as_mut().expect(EXPECT_MESSAGE).nodes[ordinal] = Some(id);
            let slot = node_index(id.0);
            if slot >= cache.nodes.len() {
                cache.nodes.resize_with(slot + 1, NodeEpochs::default);
                cache.layout_inputs.resize_with(slot + 1, || None);
            }
            if existing.is_none() {
                cache.nodes[slot] = NodeEpochs::default();
                cache.nodes[slot].owner = (index, ordinal);
            }
            cache.nodes[slot].last_requested = cache.epoch;
            cache.nodes[slot].last_retained = 0;
            cache.layout_inputs[slot] = layout_inputs;
        }
        (id, existing.is_some())
    }

    fn request_node(
        &mut self,
        style: RequestedStyle,
        children: &[LayoutId],
        context: Option<NodeContext>,
    ) -> LayoutId {
        let (id, reused) = self.claim_node(style, children);
        if context.is_some() || (reused && self.taffy.get_node_context(id.0).is_some()) {
            // Opaque measurements can observe arbitrary external state.
            // Replacing the callback must invalidate every cached constraint.
            self.taffy
                .set_node_context(id.0, context)
                .expect(EXPECT_MESSAGE);
            self.layout_changes += 1;
        }
        id
    }

    pub fn request_layout(
        &mut self,
        style: Style,
        rem_size: Pixels,
        scale_factor: f32,
        children: &[LayoutId],
    ) -> LayoutId {
        if self.cache.is_some() {
            return self.request_node(
                RequestedStyle::Inputs(LayoutInputKey::new(&style, rem_size, scale_factor)),
                children,
                None,
            );
        }
        let taffy_style = style.to_taffy(rem_size, scale_factor);
        if children.is_empty() {
            self.taffy
                .new_leaf(taffy_style)
                .expect(EXPECT_MESSAGE)
                .into()
        } else {
            self.taffy
                // This is safe because LayoutId is repr(transparent) to taffy::tree::NodeId.
                .new_with_children(taffy_style, LayoutId::to_taffy_slice(children))
                .expect(EXPECT_MESSAGE)
                .into()
        }
    }

    pub fn request_measured_layout(
        &mut self,
        style: Style,
        rem_size: Pixels,
        scale_factor: f32,
        measure: impl FnMut(
            Size<Option<Pixels>>,
            Size<AvailableSpace>,
            &mut Window,
            &mut App,
        ) -> Size<Pixels>
        + 'static,
    ) -> LayoutId {
        self.request_owned_measured_layout(style, rem_size, scale_factor, None, measure)
    }

    pub(crate) fn request_owned_measured_layout(
        &mut self,
        style: Style,
        rem_size: Pixels,
        scale_factor: f32,
        occurrence: Option<crate::view::ViewOccurrenceId>,
        measure: impl FnMut(
            Size<Option<Pixels>>,
            Size<AvailableSpace>,
            &mut Window,
            &mut App,
        ) -> Size<Pixels>
        + 'static,
    ) -> LayoutId {
        let measure = Box::new(measure) as Box<MeasureFn>;
        #[cfg(feature = "stacker")]
        let measure = StackSafe::new(measure);

        let context = if let Some(occurrence) = occurrence {
            NodeContext::OwnedCustom(Box::new(OwnedMeasure {
                measure,
                occurrence,
            }))
        } else {
            NodeContext::Custom(measure)
        };
        if self.cache.is_some() {
            return self.request_node(
                RequestedStyle::Inputs(LayoutInputKey::new(&style, rem_size, scale_factor)),
                &[],
                Some(context),
            );
        }
        let taffy_style = style.to_taffy(rem_size, scale_factor);
        self.taffy
            .new_leaf_with_context(taffy_style, context)
            .expect(EXPECT_MESSAGE)
            .into()
    }

    pub(crate) fn request_text_layout(
        &mut self,
        measure: crate::elements::TextMeasure,
    ) -> LayoutId {
        let style = text_layout_style();
        if self.cache.is_some() {
            let (id, reused) = self.claim_node(RequestedStyle::Converted(style), &[]);
            if reused
                && let Some(NodeContext::Text(previous)) = self.taffy.get_node_context_mut(id.0)
            {
                let carried = measure.carry_from(previous);
                if !carried {
                    measure.reset();
                }
                // Keep the authoritative allocation, not the old descriptor.
                // In-place replacement bypasses the setter's invalidation.
                **previous = measure;
                if !carried {
                    self.taffy.mark_dirty(id.0).expect(EXPECT_MESSAGE);
                    self.layout_changes += 1;
                }
            } else {
                measure.reset();
                self.taffy
                    .set_node_context(id.0, Some(NodeContext::Text(Box::new(measure))))
                    .expect(EXPECT_MESSAGE);
                self.layout_changes += 1;
            }
            id
        } else {
            self.taffy
                .new_leaf_with_context(style, NodeContext::Text(Box::new(measure)))
                .expect(EXPECT_MESSAGE)
                .into()
        }
    }

    /// Treats any `auto` dimension of the given node's style as filling `size`.
    ///
    /// This is applied to window roots before layout so they behave like the
    /// root element on the web, which stretches to fill the initial containing
    /// block (the viewport) unless given an explicit size. Explicitly styled
    /// dimensions are preserved.
    pub fn stretch_auto_size_to_fill(
        &mut self,
        id: LayoutId,
        size: Size<Pixels>,
        scale_factor: f32,
    ) {
        let style = self.taffy.style(id.0).expect(EXPECT_MESSAGE);
        let stretch_width = style.size.width.is_auto();
        let stretch_height = style.size.height.is_auto();
        if !stretch_width && !stretch_height {
            return;
        }
        let mut style = style.clone();
        if stretch_width {
            style.size.width =
                taffy::style::Dimension::length(round_to_device_pixel(size.width.0, scale_factor));
        }
        if stretch_height {
            style.size.height =
                taffy::style::Dimension::length(round_to_device_pixel(size.height.0, scale_factor));
        }
        if self.taffy.style(id.0).expect(EXPECT_MESSAGE) != &style {
            self.taffy.set_style(id.0, style).expect(EXPECT_MESSAGE);
            self.layout_changes += 1;
            if let Some(cache) = self.cache.as_mut() {
                cache.layout_inputs[node_index(id.0)] = None;
            }
        }
    }

    // Used to understand performance
    #[allow(dead_code)]
    fn count_all_children(&self, parent: LayoutId) -> anyhow::Result<u32> {
        let mut count = 0;

        for child in self.taffy.children(parent.0)? {
            // Count this child.
            count += 1;

            // Count all of this child's children.
            count += self.count_all_children(LayoutId(child))?
        }

        Ok(count)
    }

    // Used to understand performance
    #[allow(dead_code)]
    fn max_depth(&self, depth: u32, parent: LayoutId) -> anyhow::Result<u32> {
        println!(
            "{parent:?} at depth {depth} has {} children",
            self.taffy.child_count(parent.0)
        );

        let mut max_child_depth = 0;

        for child in self.taffy.children(parent.0)? {
            max_child_depth = std::cmp::max(max_child_depth, self.max_depth(0, LayoutId(child))?);
        }

        Ok(depth + 1 + max_child_depth)
    }

    // Used to understand performance
    #[allow(dead_code)]
    fn get_edges(&self, parent: LayoutId) -> anyhow::Result<Vec<(LayoutId, LayoutId)>> {
        let mut edges = Vec::new();

        for child in self.taffy.children(parent.0)? {
            edges.push((parent, LayoutId(child)));

            edges.extend(self.get_edges(LayoutId(child))?);
        }

        Ok(edges)
    }

    #[cfg_attr(feature = "stacker", stacksafe::stacksafe)]
    pub fn compute_layout(
        &mut self,
        id: LayoutId,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.compute_layout_with_input(id, available_space, None, window, cx);
    }

    pub(crate) fn compute_retained_layout(
        &mut self,
        id: LayoutId,
        input: taffy::LayoutInput,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.compute_layout_with_input(id, AvailableSpace::min_size(), Some(input), window, cx);
    }

    fn compute_layout_with_input(
        &mut self,
        id: LayoutId,
        available_space: Size<AvailableSpace>,
        input: Option<taffy::LayoutInput>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(cache) = self.cache.as_mut() {
            let generation = window.text_system().font_generation();
            if cache.font_generation != Some(generation) {
                // Fonts may be installed after request_layout, before this root
                // is computed. Invalidating only at descriptor creation is too early.
                for scope in cache.scopes.iter().flatten() {
                    for id in scope.nodes.iter().flatten() {
                        if let Some(NodeContext::Text(measure)) =
                            self.taffy.get_node_context_mut(id.0)
                            && measure.refresh_fonts(generation)
                        {
                            self.taffy.mark_dirty(id.0).expect(EXPECT_MESSAGE);
                            self.layout_changes += 1;
                        }
                    }
                }
                cache.font_generation = Some(generation);
            }
        }
        if input.is_none()
            && let Some(cache) = self.cache.as_ref()
            && let Some(parent) = self.taffy.parent(id.0)
            && cache.nodes[node_index(parent)].last_requested != cache.epoch
        {
            // A root may still have a parent from the previous frame. Preserve
            // current-frame parentage when a custom element computes a subtree.
            self.taffy.remove_child(parent, id.0).expect(EXPECT_MESSAGE);
            self.layout_changes += 1;
        }
        // Leaving this here until we have a better instrumentation approach.
        // println!("Laying out {} children", self.count_all_children(id)?);
        // println!("Max layout depth: {}", self.max_depth(0, id)?);

        // Output the edges (branches) of the tree in Mermaid format for visualization.
        // println!("Edges:");
        // for (a, b) in self.get_edges(id)? {
        //     println!("N{} --> N{}", u64::from(a), u64::from(b));
        // }
        //

        if let Some(cache) = self.cache.as_mut() {
            let epochs = &mut cache.nodes[node_index(id.0)];
            if epochs.last_computed != cache.epoch {
                epochs.last_computed = cache.epoch;
                cache.computed_roots.push(id);
            }
        } else {
            self.computed_layouts.insert(id);
        }
        // A speculative child may have populated absolute bounds before its
        // containing root redistributes flex space, even on that root's first pass.
        if input.is_some() {
            let stack = &mut self.layout_bounds_scratch_space;
            stack.push(id);
            while let Some(node) = stack.pop() {
                self.absolute_layout.remove(node.0.into());
                stack.extend(self.taffy.child_ids(node.0).map(LayoutId));
            }
        } else {
            self.advance_layout_epoch();
        }

        let scale_factor = window.scale_factor();

        let transform = |v: AvailableSpace| match v {
            AvailableSpace::Definite(pixels) => {
                AvailableSpace::Definite(Pixels(pixels.0 * scale_factor))
            }
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        };
        let available_space = size(
            transform(available_space.width),
            transform(available_space.height),
        );

        let cache_enabled = self.cache.is_some();
        let measure = |inputs: taffy::LayoutInput,
                       _id,
                       node_context: Option<&mut NodeContext>,
                       style: &taffy::Style| {
            taffy::compute_leaf_layout(
                inputs,
                style,
                |_, _| 0.0,
                |known_dimensions, available_space| {
                    let Some(node_context) = node_context else {
                        return taffy::geometry::Size::default();
                    };

                    let known_dimensions = Size {
                        width: known_dimensions.width.map(|e| Pixels(e / scale_factor)),
                        height: known_dimensions.height.map(|e| Pixels(e / scale_factor)),
                    };

                    let available_space: Size<AvailableSpace> = available_space.into();
                    let untransform = |ev: AvailableSpace| match ev {
                        AvailableSpace::Definite(pixels) => {
                            AvailableSpace::Definite(Pixels(pixels.0 / scale_factor))
                        }
                        AvailableSpace::MinContent => AvailableSpace::MinContent,
                        AvailableSpace::MaxContent => AvailableSpace::MaxContent,
                    };
                    let available_space = size(
                        untransform(available_space.width),
                        untransform(available_space.height),
                    );

                    let measured_size = match node_context {
                        NodeContext::Custom(measure) => {
                            measure(known_dimensions, available_space, window, cx)
                        }
                        NodeContext::OwnedCustom(owned) => {
                            window.with_view_measure_access(owned.occurrence, cx, |window, cx| {
                                (owned.measure)(known_dimensions, available_space, window, cx)
                            })
                        }
                        NodeContext::Text(measure) => measure.measure(
                            known_dimensions,
                            available_space,
                            cache_enabled.then_some(inputs.run_mode),
                            window,
                            cx,
                        ),
                    };
                    snap_measured_size_to_device_pixels(measured_size, scale_factor).into()
                },
            )
        };
        if let Some(retained) = self.retained.as_mut() {
            let mut tree = RetainedLayoutTree {
                taffy: &mut self.taffy,
                retained,
                measure,
            };
            if let Some(input) = input {
                let output = tree.compute(id.0, input, None);
                if let Some(mut layout) = tree.retained.layouts.get(id.0.into()).copied() {
                    layout.size = output.size;
                    layout.scrollable_overflow_rect = output.scrollable_overflow_rect;
                    tree.retained.set_layout(id.0, layout);
                }
            } else {
                taffy::compute_root_layout(&mut tree, id.0, available_space.into());
            }
        } else {
            self.taffy
                .compute_layout_with_measure(id.into(), available_space.into(), measure)
                .expect(EXPECT_MESSAGE);
        }
    }

    // Pixel snapping
    //
    // Painting primitives at non-integer pixel coordinates produces blurry
    // output. Pixel snapping converts layout coordinates into integer
    // device-pixel coordinates so painted edges land exactly on physical
    // pixel boundaries.
    //
    // Non-integer coordinates can arise for several reasons, including:
    //   - flex distribution, percentages, centering, and text measurement
    //     can produce fractional element sizes and positions;
    //   - at fractional scale factors (for example 125% or 150%), integer
    //     logical-pixel values can map to non-integer device-pixel values.
    //
    // We pixel-snap by rounding in device-pixel space, after multiplying
    // by `scale_factor`, so that snapping targets physical pixels. Bounds
    // are divided by `scale_factor` before being returned to GPUI.
    //
    // Midpoints are rounded toward zero. This is a stylistic choice: a
    // 1-logical-pixel line at 150% scale should render as 1 dp rather than
    // 2 dp.
    //
    // Pixel snapping is done in two phases:
    //
    //  1. Pre-layout metric snapping. Before Taffy computes layout, all
    //     authored absolute lengths are rounded in `to_taffy`. This
    //     includes borders, padding, gaps, and explicit sizes.
    //     Custom-measured leaf nodes have their measured sizes rounded up
    //     to integer device-pixel lengths.
    //
    //  2. Post-layout edge snapping. After Taffy resolves the tree, layout
    //     relationships such as flex shares, grid tracks, percentages, and
    //     centering can produce new fractional edge positions. Boxes now
    //     have edges in absolute coordinates, and snapping must decide
    //     where those edges land on the device-pixel grid.
    //
    // Ideally, post-layout snapping would satisfy:
    //
    //  - Edge closure. Two raw layout edges at the same absolute position
    //    should snap to the same pixel column.
    //  - Translation stability. A component's internal geometry should not
    //    change when it moves to a new absolute position.
    //
    // These goals are in tension because rounding is not associative.
    // The simple local schemes make different tradeoffs:
    //
    //  - Absolute edge rounding gives each window coordinate one answer,
    //    so coincident edges always close globally. But a span's snapped
    //    length is `round(far) - round(near)`, which may change by 1 dp
    //    as its absolute origin moves.
    //
    //  - Parent-relative edge rounding rounds each child inside its
    //    parent's coordinate space. This guarantees translation stability,
    //    but a shared edge reached through different parents can
    //    accumulate different rounding, causing non-closure between
    //    cousins.
    //
    //  - Length rounding rounds each width, height, and thickness
    //    independently and then places boxes from those rounded lengths.
    //    Sizes stay stable under translation, but neighboring boxes derive
    //    their shared boundary from different sources, so closure is not
    //    guaranteed.
    //
    // We apply absolute edge rounding for each element's outer box in
    // post-layout rounding to preserve closure. Border and padding widths
    // are not touched by post-layout rounding; they keep their pre-layout
    // rounded value so that they remain stable under translation.
    //
    // This gives both closure and translation stability in the case that
    // all local metrics are integer device-pixel lengths. Pre-layout
    // rounding covers that in most cases. The exception is metrics
    // resolved by layout relationships, such as percentages. Outer box
    // edges will still close globally, and painted border widths are still
    // snapped independently, but the raw content-box origin can carry a
    // 1dp residual into descendants.

    pub fn layout_bounds(&mut self, id: LayoutId, scale_factor: f32) -> Bounds<Pixels> {
        self.absolute_layout(id, scale_factor).bounds
    }

    fn absolute_layout(&mut self, id: LayoutId, scale_factor: f32) -> AbsoluteLayout {
        if let Some(layout) = self.absolute_layout.get(id.0.into()).copied()
            && layout.epoch == self.layout_epoch
        {
            return layout;
        }

        let layout = self
            .retained
            .as_ref()
            .and_then(|retained| retained.layouts.get(id.0.into()))
            .unwrap_or_else(|| self.taffy.layout(id.into()).expect(EXPECT_MESSAGE));
        let layout_location = layout.location;
        let layout_size = layout.size;
        let parent = self.taffy.parent(id.0);

        let absolute_outer_origin = match parent {
            Some(parent_id) => {
                let parent_id = LayoutId::from(parent_id);
                let parent_origin = self.absolute_layout(parent_id, scale_factor).outer_origin;
                parent_origin + Point::from(layout_location)
            }
            None => Point::from(layout_location),
        };
        let absolute_far = absolute_outer_origin + Point::from(Size::from(layout_size));
        let snapped_bounds = Bounds::from_corners(
            absolute_outer_origin.map(round_half_toward_zero),
            absolute_far.map(round_half_toward_zero),
        );

        let bounds = (snapped_bounds / scale_factor).map(Pixels);
        let layout = AbsoluteLayout {
            epoch: self.layout_epoch,
            bounds,
            outer_origin: absolute_outer_origin,
        };
        self.absolute_layout.insert(id.0.into(), layout);
        layout
    }
}

/// A unique identifier for a layout node, generated when requesting a layout from Taffy
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct LayoutId(NodeId);

impl LayoutId {
    fn to_taffy_slice(node_ids: &[Self]) -> &[taffy::NodeId] {
        // SAFETY: LayoutId is repr(transparent) to taffy::tree::NodeId.
        unsafe { std::mem::transmute::<&[LayoutId], &[taffy::NodeId]>(node_ids) }
    }
}

impl std::hash::Hash for LayoutId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u64::from(self.0).hash(state);
    }
}

impl From<NodeId> for LayoutId {
    fn from(node_id: NodeId) -> Self {
        Self(node_id)
    }
}

impl From<LayoutId> for NodeId {
    fn from(layout_id: LayoutId) -> NodeId {
        layout_id.0
    }
}

fn snap_measured_size_to_device_pixels(size: Size<Pixels>, scale_factor: f32) -> Size<f32> {
    size.map(|d| ceil_to_device_pixel(d.0.max(0.0), scale_factor))
}

fn border_widths_to_taffy(
    widths: &Edges<AbsoluteLength>,
    rem_size: Pixels,
    scale_factor: f32,
) -> TaffyRect<taffy::style::LengthPercentage> {
    let snap = |w: &AbsoluteLength| {
        taffy::style::LengthPercentage::length(round_stroke_to_device_pixel(
            w.to_pixels(rem_size).0,
            scale_factor,
        ))
    };
    TaffyRect {
        top: snap(&widths.top),
        right: snap(&widths.right),
        bottom: snap(&widths.bottom),
        left: snap(&widths.left),
    }
}

trait ToTaffy<Output> {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> Output;
}

enum RequestedStyle {
    Inputs(LayoutInputKey),
    Converted(taffy::Style),
}

impl RequestedStyle {
    fn convert(self) -> taffy::Style {
        match self {
            Self::Inputs(inputs) => inputs.convert(),
            Self::Converted(style) => style,
        }
    }
}

// Conversion is defined on the key, not Style, so it cannot consume a field
// without including it in the exact comparison.
macro_rules! layout_input_key {
    ($($field:ident: $field_type:ty),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq)]
        struct LayoutInputKey {
            rem_size: Pixels,
            scale_factor: f32,
            $($field: $field_type,)*
        }

        impl LayoutInputKey {
            fn new(style: &Style, rem_size: Pixels, scale_factor: f32) -> Self {
                Self {
                    rem_size,
                    scale_factor,
                    $($field: Clone::clone(&style.$field),)*
                }
            }

            #[cfg(test)]
            fn style(&self) -> Style {
                Style {
                    $($field: Clone::clone(&self.$field),)*
                    ..Style::default()
                }
            }
        }
    };
}

layout_input_key! {
    display: crate::Display,
    overflow: Point<crate::Overflow>,
    scrollbar_width: AbsoluteLength,
    position: crate::Position,
    inset: Edges<Length>,
    size: Size<Length>,
    min_size: Size<Length>,
    max_size: Size<Length>,
    aspect_ratio: Option<f32>,
    margin: Edges<Length>,
    padding: Edges<DefiniteLength>,
    border_widths: Edges<AbsoluteLength>,
    align_items: Option<crate::AlignItems>,
    align_self: Option<crate::AlignSelf>,
    align_content: Option<crate::AlignContent>,
    justify_content: Option<crate::JustifyContent>,
    gap: Size<DefiniteLength>,
    flex_direction: crate::FlexDirection,
    flex_wrap: crate::FlexWrap,
    flex_basis: Length,
    flex_grow: f32,
    flex_shrink: f32,
    grid_rows: Option<GridTemplate>,
    grid_cols: Option<GridTemplate>,
    grid_location: Option<crate::GridLocation>,
}

impl ToTaffy<taffy::style::Style> for Style {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::Style {
        LayoutInputKey::new(self, rem_size, scale_factor).convert()
    }
}

impl LayoutInputKey {
    fn convert(&self) -> taffy::style::Style {
        let rem_size = self.rem_size;
        let scale_factor = self.scale_factor;
        use taffy::style_helpers::{fr, length, minmax, repeat};

        fn to_grid_line(
            placement: &Range<crate::GridPlacement>,
        ) -> taffy::Line<taffy::GridPlacement> {
            taffy::Line {
                start: placement.start.into(),
                end: placement.end.into(),
            }
        }

        fn to_grid_repeat<T: taffy::style::CheapCloneStr>(
            unit: &Option<GridTemplate>,
        ) -> Vec<taffy::GridTemplateComponent<T>> {
            unit.map(|template| {
                match template.min_size {
                    // grid-template-*: repeat(<number>, minmax(0, 1fr));
                    crate::GridTemplateMinSize::Zero => {
                        vec![repeat(
                            template.repeat,
                            vec![minmax(length(0.0_f32), fr(1.0_f32))],
                        )]
                    }
                    // grid-template-*: repeat(<number>, minmax(min-content, 1fr));
                    crate::GridTemplateMinSize::MinContent => {
                        vec![repeat(
                            template.repeat,
                            vec![minmax(min_content(), fr(1.0_f32))],
                        )]
                    }
                    // grid-template-*: repeat(<number>, minmax(0, max-content))
                    crate::GridTemplateMinSize::MaxContent => {
                        vec![repeat(
                            template.repeat,
                            vec![minmax(length(0.0_f32), max_content())],
                        )]
                    }
                }
            })
            .unwrap_or_default()
        }

        taffy::style::Style {
            display: self.display.into(),
            overflow: self.overflow.into(),
            scrollbar_width: self.scrollbar_width.to_taffy(rem_size, scale_factor),
            position: self.position.into(),
            inset: self.inset.to_taffy(rem_size, scale_factor),
            size: self.size.to_taffy(rem_size, scale_factor),
            min_size: self.min_size.to_taffy(rem_size, scale_factor),
            max_size: self.max_size.to_taffy(rem_size, scale_factor),
            aspect_ratio: self.aspect_ratio,
            margin: self.margin.to_taffy(rem_size, scale_factor),
            padding: self.padding.to_taffy(rem_size, scale_factor),
            border: border_widths_to_taffy(&self.border_widths, rem_size, scale_factor),
            align_items: self.align_items.map(Into::into).unwrap_or_default(),
            align_self: self.align_self.map(|x| x.into()),
            align_content: self.align_content.map(Into::into).unwrap_or_default(),
            justify_content: self.justify_content.map(Into::into).unwrap_or_default(),
            gap: self.gap.to_taffy(rem_size, scale_factor),
            flex_direction: self.flex_direction.into(),
            flex_wrap: self.flex_wrap.into(),
            flex_basis: self.flex_basis.to_taffy(rem_size, scale_factor),
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            grid_template_rows: to_grid_repeat(&self.grid_rows),
            grid_template_columns: to_grid_repeat(&self.grid_cols),
            grid_row: self
                .grid_location
                .as_ref()
                .map(|location| to_grid_line(&location.row))
                .unwrap_or_default(),
            grid_column: self
                .grid_location
                .as_ref()
                .map(|location| to_grid_line(&location.column))
                .unwrap_or_default(),
            ..Default::default()
        }
    }
}

impl ToTaffy<f32> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> f32 {
        round_to_device_pixel(self.to_pixels(rem_size).0, scale_factor)
    }
}

impl ToTaffy<taffy::style::LengthPercentageAuto> for Length {
    fn to_taffy(
        &self,
        rem_size: Pixels,
        scale_factor: f32,
    ) -> taffy::prelude::LengthPercentageAuto {
        match self {
            Length::Definite(length) => length.to_taffy(rem_size, scale_factor),
            Length::Auto => taffy::prelude::LengthPercentageAuto::auto(),
        }
    }
}

impl ToTaffy<taffy::style::Dimension> for Length {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::prelude::Dimension {
        match self {
            Length::Definite(length) => length.to_taffy(rem_size, scale_factor),
            Length::Auto => taffy::prelude::Dimension::auto(),
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentage> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::LengthPercentage {
        match self {
            DefiniteLength::Absolute(length) => length.to_taffy(rem_size, scale_factor),
            DefiniteLength::Fraction(fraction) => {
                taffy::style::LengthPercentage::percent(*fraction)
            }
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentageAuto> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::LengthPercentageAuto {
        match self {
            DefiniteLength::Absolute(length) => length.to_taffy(rem_size, scale_factor),
            DefiniteLength::Fraction(fraction) => {
                taffy::style::LengthPercentageAuto::percent(*fraction)
            }
        }
    }
}

impl ToTaffy<taffy::style::Dimension> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::Dimension {
        match self {
            DefiniteLength::Absolute(length) => length.to_taffy(rem_size, scale_factor),
            DefiniteLength::Fraction(fraction) => taffy::style::Dimension::percent(*fraction),
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentage> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::LengthPercentage {
        taffy::style::LengthPercentage::length(self.to_taffy(rem_size, scale_factor))
    }
}

impl ToTaffy<taffy::style::LengthPercentageAuto> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::LengthPercentageAuto {
        taffy::style::LengthPercentageAuto::length(self.to_taffy(rem_size, scale_factor))
    }
}

impl ToTaffy<taffy::style::Dimension> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::Dimension {
        taffy::style::Dimension::length(self.to_taffy(rem_size, scale_factor))
    }
}

impl<T, T2> From<TaffyPoint<T>> for Point<T2>
where
    T: Into<T2>,
    T2: Clone + Debug + Default + PartialEq,
{
    fn from(point: TaffyPoint<T>) -> Point<T2> {
        Point {
            x: point.x.into(),
            y: point.y.into(),
        }
    }
}

impl<T, T2> From<Point<T>> for TaffyPoint<T2>
where
    T: Into<T2> + Clone + Debug + Default + PartialEq,
{
    fn from(val: Point<T>) -> Self {
        TaffyPoint {
            x: val.x.into(),
            y: val.y.into(),
        }
    }
}

impl<T, U> ToTaffy<TaffySize<U>> for Size<T>
where
    T: ToTaffy<U> + Clone + Debug + Default + PartialEq,
{
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> TaffySize<U> {
        TaffySize {
            width: self.width.to_taffy(rem_size, scale_factor),
            height: self.height.to_taffy(rem_size, scale_factor),
        }
    }
}

impl<T, U> ToTaffy<TaffyRect<U>> for Edges<T>
where
    T: ToTaffy<U> + Clone + Debug + Default + PartialEq,
{
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> TaffyRect<U> {
        TaffyRect {
            top: self.top.to_taffy(rem_size, scale_factor),
            right: self.right.to_taffy(rem_size, scale_factor),
            bottom: self.bottom.to_taffy(rem_size, scale_factor),
            left: self.left.to_taffy(rem_size, scale_factor),
        }
    }
}

impl<T, U> From<TaffySize<T>> for Size<U>
where
    T: Into<U>,
    U: Clone + Debug + Default + PartialEq,
{
    fn from(taffy_size: TaffySize<T>) -> Self {
        Size {
            width: taffy_size.width.into(),
            height: taffy_size.height.into(),
        }
    }
}

impl<T, U> From<Size<T>> for TaffySize<U>
where
    T: Into<U> + Clone + Debug + Default + PartialEq,
{
    fn from(size: Size<T>) -> Self {
        TaffySize {
            width: size.width.into(),
            height: size.height.into(),
        }
    }
}

/// The space available for an element to be laid out in
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub enum AvailableSpace {
    /// The amount of space available is the specified number of pixels
    Definite(Pixels),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    #[default]
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}

impl AvailableSpace {
    /// Returns a `Size` with both width and height set to `AvailableSpace::MinContent`.
    ///
    /// This function is useful when you want to create a `Size` with the minimum content constraints
    /// for both dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use gpui::AvailableSpace;
    /// let min_content_size = AvailableSpace::min_size();
    /// assert_eq!(min_content_size.width, AvailableSpace::MinContent);
    /// assert_eq!(min_content_size.height, AvailableSpace::MinContent);
    /// ```
    pub const fn min_size() -> Size<Self> {
        Size {
            width: Self::MinContent,
            height: Self::MinContent,
        }
    }
}

impl From<AvailableSpace> for TaffyAvailableSpace {
    fn from(space: AvailableSpace) -> TaffyAvailableSpace {
        match space {
            AvailableSpace::Definite(Pixels(value)) => TaffyAvailableSpace::Definite(value),
            AvailableSpace::MinContent => TaffyAvailableSpace::MinContent,
            AvailableSpace::MaxContent => TaffyAvailableSpace::MaxContent,
        }
    }
}

impl From<TaffyAvailableSpace> for AvailableSpace {
    fn from(space: TaffyAvailableSpace) -> AvailableSpace {
        match space {
            TaffyAvailableSpace::Definite(value) => AvailableSpace::Definite(Pixels(value)),
            TaffyAvailableSpace::MinContent => AvailableSpace::MinContent,
            TaffyAvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
}

impl From<Pixels> for AvailableSpace {
    fn from(pixels: Pixels) -> Self {
        AvailableSpace::Definite(pixels)
    }
}

impl From<Size<Pixels>> for Size<AvailableSpace> {
    fn from(size: Size<Pixels>) -> Self {
        Size {
            width: AvailableSpace::Definite(size.width),
            height: AvailableSpace::Definite(size.height),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_layout_inputs_have_equal_conversions() {
        use crate::{
            AlignContent, AlignItems, Display, FlexDirection, FlexWrap, GridLocation,
            GridPlacement, GridTemplateMinSize, Overflow, Position, Rems,
        };
        use rand::{Rng as _, SeedableRng as _, rngs::StdRng};

        fn choose<T: Copy>(random: &mut StdRng, choices: &[T]) -> T {
            choices[random.random_range(0..choices.len())]
        }
        fn absolute(random: &mut StdRng) -> AbsoluteLength {
            let value = random.random_range(-32.0..128.0);
            choose(random, &[Pixels(value).into(), Rems(value).into()])
        }
        fn definite(random: &mut StdRng) -> DefiniteLength {
            let length = absolute(random);
            let fraction = random.random_range(-1.0..2.0);
            choose(random, &[length.into(), DefiniteLength::Fraction(fraction)])
        }
        fn length(random: &mut StdRng) -> Length {
            let length = definite(random);
            choose(random, &[length.into(), Length::Auto])
        }
        fn edges<T: Clone + Debug + Default + PartialEq>(
            random: &mut StdRng,
            generate: fn(&mut StdRng) -> T,
        ) -> Edges<T> {
            Edges {
                top: generate(random),
                right: generate(random),
                bottom: generate(random),
                left: generate(random),
            }
        }
        fn dimensions<T: Clone + Debug + Default + PartialEq>(
            random: &mut StdRng,
            generate: fn(&mut StdRng) -> T,
        ) -> Size<T> {
            size(generate(random), generate(random))
        }
        fn alignment(random: &mut StdRng) -> Option<AlignItems> {
            choose(
                random,
                &[
                    None,
                    Some(AlignItems::Start),
                    Some(AlignItems::End),
                    Some(AlignItems::FlexStart),
                    Some(AlignItems::FlexEnd),
                    Some(AlignItems::Center),
                    Some(AlignItems::Baseline),
                    Some(AlignItems::Stretch),
                ],
            )
        }
        fn content(random: &mut StdRng) -> Option<AlignContent> {
            choose(
                random,
                &[
                    None,
                    Some(AlignContent::Start),
                    Some(AlignContent::End),
                    Some(AlignContent::FlexStart),
                    Some(AlignContent::FlexEnd),
                    Some(AlignContent::Center),
                    Some(AlignContent::Stretch),
                    Some(AlignContent::SpaceBetween),
                    Some(AlignContent::SpaceEvenly),
                    Some(AlignContent::SpaceAround),
                ],
            )
        }
        fn template(random: &mut StdRng) -> Option<GridTemplate> {
            let template = GridTemplate {
                repeat: random.random_range(1..32),
                min_size: choose(
                    random,
                    &[
                        GridTemplateMinSize::Zero,
                        GridTemplateMinSize::MinContent,
                        GridTemplateMinSize::MaxContent,
                    ],
                ),
            };
            choose(random, &[None, Some(template)])
        }
        fn placement(random: &mut StdRng) -> GridPlacement {
            let line = random.random_range(-32..32);
            let span = random.random_range(1..32);
            choose(
                random,
                &[
                    GridPlacement::Auto,
                    GridPlacement::Line(line),
                    GridPlacement::Span(span),
                ],
            )
        }
        let mut random = StdRng::seed_from_u64(0x1a90_17);
        for iteration in 0..10_000 {
            let location = GridLocation {
                row: placement(&mut random)..placement(&mut random),
                column: placement(&mut random)..placement(&mut random),
            };
            // Exhaustive initialization also forces newly added key fields to
            // get a generator rather than silently staying at their defaults.
            let inputs = LayoutInputKey {
                rem_size: Pixels(random.random_range(8.0..32.0)),
                scale_factor: random.random_range(0.5..4.0),
                display: choose(
                    &mut random,
                    &[Display::None, Display::Block, Display::Flex, Display::Grid],
                ),
                overflow: Point {
                    x: choose(
                        &mut random,
                        &[
                            Overflow::Visible,
                            Overflow::Clip,
                            Overflow::Hidden,
                            Overflow::Scroll,
                        ],
                    ),
                    y: choose(
                        &mut random,
                        &[
                            Overflow::Visible,
                            Overflow::Clip,
                            Overflow::Hidden,
                            Overflow::Scroll,
                        ],
                    ),
                },
                scrollbar_width: absolute(&mut random),
                position: choose(&mut random, &[Position::Relative, Position::Absolute]),
                inset: edges(&mut random, length),
                size: dimensions(&mut random, length),
                min_size: dimensions(&mut random, length),
                max_size: dimensions(&mut random, length),
                aspect_ratio: if random.random_bool(0.5) {
                    Some(random.random_range(0.1..10.0))
                } else {
                    None
                },
                margin: edges(&mut random, length),
                padding: edges(&mut random, definite),
                border_widths: edges(&mut random, absolute),
                align_items: alignment(&mut random),
                align_self: alignment(&mut random),
                align_content: content(&mut random),
                justify_content: content(&mut random),
                gap: dimensions(&mut random, definite),
                flex_direction: choose(
                    &mut random,
                    &[
                        FlexDirection::Row,
                        FlexDirection::RowReverse,
                        FlexDirection::Column,
                        FlexDirection::ColumnReverse,
                    ],
                ),
                flex_wrap: choose(
                    &mut random,
                    &[FlexWrap::NoWrap, FlexWrap::Wrap, FlexWrap::WrapReverse],
                ),
                flex_basis: length(&mut random),
                flex_grow: random.random_range(0.0..10.0),
                flex_shrink: random.random_range(0.0..10.0),
                grid_rows: template(&mut random),
                grid_cols: template(&mut random),
                grid_location: if random.random_bool(0.5) {
                    Some(location)
                } else {
                    None
                },
            };
            let first = inputs.style();
            let mut second = first.clone();
            second.opacity = Some(random.random_range(0.0..1.0));
            second.visibility = crate::Visibility::Hidden;
            second.border_color = Some(crate::red());
            second.corner_radii.top_left = absolute(&mut random);
            second.text.font_size = Some(absolute(&mut random));
            let first_key = LayoutInputKey::new(&first, inputs.rem_size, inputs.scale_factor);
            let second_key = LayoutInputKey::new(&second, inputs.rem_size, inputs.scale_factor);
            assert_eq!(first_key, second_key, "iteration {iteration}");
            assert_eq!(
                first.to_taffy(inputs.rem_size, inputs.scale_factor),
                second.to_taffy(inputs.rem_size, inputs.scale_factor),
                "iteration {iteration}",
            );
        }
    }

    #[test]
    fn layout_inputs_track_rem_scale_and_mutations() {
        let mut engine = TaffyLayoutEngine::new();
        engine.enable_cache();
        let mut style = Style::default();
        style.size.width = crate::Rems(2.0).into();
        style.border_widths = Edges::all(Pixels(0.6).into());
        let mut previous = None;
        for (rem_size, scale_factor, width) in [
            (16.0, 1.0, 2.0),
            (16.0, 1.0, 2.0),
            (20.0, 1.0, 2.0),
            (20.0, 2.0, 2.0),
            (20.0, 2.0, 3.0),
        ] {
            style.size.width = crate::Rems(width).into();
            engine.set_request_scope(&crate::element::InternedHandle::for_test(1, 0));
            let id = engine.request_layout(style.clone(), Pixels(rem_size), scale_factor, &[]);
            if let Some(previous) = previous {
                assert_eq!(id, previous);
            }
            assert_eq!(
                engine.taffy.style(id.0).expect(EXPECT_MESSAGE),
                &style.to_taffy(Pixels(rem_size), scale_factor),
            );
            // clear retains only nodes marked reachable by a compute.
            engine.cache.as_mut().unwrap().nodes[node_index(id.0)].last_reachable =
                engine.cache.as_ref().unwrap().epoch;
            previous = Some(id);
            engine.clear();
        }
    }

    fn old_layout_bounds(
        engine: &TaffyLayoutEngine,
        id: LayoutId,
        scale: f32,
        bounds: &mut FxHashMap<LayoutId, Bounds<Pixels>>,
        origins: &mut FxHashMap<LayoutId, Point<f32>>,
    ) -> Bounds<Pixels> {
        if let Some(bounds) = bounds.get(&id) {
            return *bounds;
        }
        let layout = *engine.taffy.layout(id.0).expect(EXPECT_MESSAGE);
        let origin = match engine.taffy.parent(id.0) {
            Some(parent) => {
                let parent = LayoutId(parent);
                old_layout_bounds(engine, parent, scale, bounds, origins);
                origins[&parent] + Point::from(layout.location)
            }
            None => Point::from(layout.location),
        };
        origins.insert(id, origin);
        let result = (Bounds::from_corners(
            origin.map(round_half_toward_zero),
            (origin + Point::from(Size::from(layout.size))).map(round_half_toward_zero),
        ) / scale)
            .map(Pixels);
        bounds.insert(id, result);
        result
    }

    #[gpui::test]
    fn dense_bounds_match_old_memo_across_reuse(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            let mut engine = TaffyLayoutEngine::new();
            for scale in [1.25, 1.5, 1.75] {
                window.set_scale_factor(scale);
                for frame in 0..3 {
                    let mut leaf_style = Style::default();
                    leaf_style.size =
                        size(DefiniteLength::Fraction(0.37).into(), Pixels(9.0).into());
                    let leaf = engine.request_layout(leaf_style, Pixels(16.0), scale, &[]);
                    let mut parent_style = Style::default();
                    parent_style.size = size(Pixels(31.0).into(), Pixels(29.0).into());
                    parent_style.padding = Edges::all(DefiniteLength::Fraction(0.075));
                    parent_style.margin.left = DefiniteLength::Fraction(0.13).into();
                    let parent = engine.request_layout(parent_style, Pixels(16.0), scale, &[leaf]);
                    let mut root_style = Style::default();
                    root_style.size =
                        size(Pixels(101.0 + frame as f32).into(), Pixels(83.0).into());
                    root_style.padding = Edges::all(DefiniteLength::Fraction(0.125));
                    let root = engine.request_layout(root_style, Pixels(16.0), scale, &[parent]);
                    for _ in 0..2 {
                        engine.compute_layout(
                            root,
                            size(Pixels(201.0), Pixels(151.0)).into(),
                            window,
                            cx,
                        );
                        let mut bounds = FxHashMap::default();
                        let mut origins = FxHashMap::default();
                        for id in [leaf, parent, root, leaf] {
                            let expected =
                                old_layout_bounds(&engine, id, scale, &mut bounds, &mut origins);
                            let actual = engine.layout_bounds(id, scale);
                            assert_eq!(
                                [
                                    actual.origin.x.0.to_bits(),
                                    actual.origin.y.0.to_bits(),
                                    actual.size.width.0.to_bits(),
                                    actual.size.height.0.to_bits(),
                                ],
                                [
                                    expected.origin.x.0.to_bits(),
                                    expected.origin.y.0.to_bits(),
                                    expected.size.width.0.to_bits(),
                                    expected.size.height.0.to_bits(),
                                ]
                            );
                        }
                    }
                    engine.clear();
                }
            }
        });
    }

    #[test]
    fn text_layout_style_matches_default_conversion() {
        for rem_size in [Pixels(12.0), Pixels(16.0), Pixels(24.0)] {
            for scale_factor in [1.0, 1.25, 1.5, 2.0] {
                assert_eq!(
                    text_layout_style(),
                    Style::default().to_taffy(rem_size, scale_factor),
                    "rem size {rem_size:?}, scale {scale_factor}"
                );
            }
        }
    }

    fn scoped_request(
        engine: &mut TaffyLayoutEngine,
        scope: u64,
        style: Style,
        scale: f32,
        children: &[LayoutId],
    ) -> LayoutId {
        engine.set_request_scope(&crate::element::InternedHandle::for_test(scope as u32, 0));
        engine.request_layout(style, Pixels(16.0), scale, children)
    }

    #[gpui::test]
    fn layout_cache_restores_root_style_when_reparented(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            window.set_scale_factor(1.0);
            let mut engine = TaffyLayoutEngine::new();
            engine.enable_cache();
            let root = scoped_request(&mut engine, 1, Style::default(), 1.0, &[]);
            let viewport = size(Pixels(200.0), Pixels(100.0));
            engine.stretch_auto_size_to_fill(root, viewport, 1.0);
            engine.compute_layout(root, viewport.into(), window, cx);
            assert_eq!(engine.layout_bounds(root, 1.0).size, viewport);
            engine.clear();

            let child = scoped_request(&mut engine, 1, Style::default(), 1.0, &[]);
            assert_eq!(child, root);
            let root = scoped_request(&mut engine, 2, Style::default(), 1.0, &[child]);
            engine.compute_layout(root, AvailableSpace::min_size(), window, cx);
            assert_eq!(engine.layout_bounds(child, 1.0).size, Size::default());
            engine.clear();
        });
    }

    #[test]
    fn layout_cache_distinguishes_handle_generations() {
        let mut engine = TaffyLayoutEngine::new();
        engine.enable_cache();
        let handle = crate::element::InternedHandle::for_test(1, 0);
        let recycled = crate::element::InternedHandle::for_test(1, 1);
        engine.set_request_scope(&handle);
        engine.set_request_scope(&recycled);
        let cache = engine.cache.as_ref().expect("cache enabled");
        assert_eq!(cache.request_scope, Some(1));
        assert_eq!(cache.scope_indices.len(), 2);
        assert_eq!(cache.scopes.len(), 2);
    }

    #[gpui::test(iterations = 10)]
    fn layout_cache_matches_fresh_geometry(
        mut rng: rand::rngs::StdRng,
        cx: &mut crate::TestAppContext,
    ) {
        use rand::{Rng as _, seq::SliceRandom as _};

        cx.add_empty_window().update(|window, cx| {
            let mut fresh = TaffyLayoutEngine::new();
            let mut cached = TaffyLayoutEngine::new();
            cached.enable_cache();
            for frame in 0..40 {
                let scale = [1.0, 1.25, 2.0][frame % 3];
                window.set_scale_factor(scale);
                let mut scopes: Vec<u64> = (10..26).collect();
                scopes.shuffle(&mut rng);
                scopes.truncate(rng.random_range(0..=16));
                let styles: Vec<_> = scopes
                    .iter()
                    .map(|_| {
                        let mut style = Style::default();
                        style.size = size(
                            Pixels(rng.random_range(5.0..80.0)).into(),
                            Pixels(rng.random_range(5.0..30.0)).into(),
                        );
                        style.padding = Edges::all(Pixels(1.3).into());
                        style
                    })
                    .collect();
                let split = rng.random_range(0..=scopes.len());
                let viewport = size(Pixels(400.0 + frame as f32), Pixels(300.0));
                let mut results = Vec::new();
                for engine in [&mut fresh, &mut cached] {
                    let leaves: Vec<_> = scopes
                        .iter()
                        .zip(&styles)
                        .map(|(scope, style)| {
                            scoped_request(engine, *scope, style.clone(), scale, &[])
                        })
                        .collect();
                    let left = scoped_request(engine, 1, Style::default(), scale, &leaves[..split]);
                    let right =
                        scoped_request(engine, 2, Style::default(), scale, &leaves[split..]);
                    let root = scoped_request(engine, 0, Style::default(), scale, &[left, right]);
                    engine.stretch_auto_size_to_fill(root, viewport, scale);
                    engine.compute_layout(root, viewport.into(), window, cx);
                    results.push(
                        leaves
                            .into_iter()
                            .chain([left, right, root])
                            .map(|id| engine.layout_bounds(id, scale))
                            .collect::<Vec<_>>(),
                    );
                    engine.clear();
                }
                assert_eq!(results[0], results[1], "frame {frame}");
                assert_eq!(cached.taffy.total_node_count(), scopes.len() + 3);
            }
            cached.clear();
            assert_eq!(cached.taffy.total_node_count(), 0);
        });
    }

    #[gpui::test]
    fn layout_cache_reuses_ordinals_and_preserves_reachable_measurements(
        cx: &mut crate::TestAppContext,
    ) {
        use std::{cell::Cell, rc::Rc};

        cx.add_empty_window().update(|window, cx| {
            window.set_scale_factor(1.0);
            let mut engine = TaffyLayoutEngine::new();
            engine.enable_cache();
            let scope = crate::element::InternedHandle::for_test(0, 0);
            let calls = Rc::new(Cell::new(0));
            let mut previous = None;
            let mut previous_capture: Option<std::rc::Weak<()>> = None;
            for frame in 0..4 {
                let capture = Rc::new(());
                let weak_capture = Rc::downgrade(&capture);
                engine.set_request_scope(&scope);
                let measured =
                    engine.request_measured_layout(Style::default(), Pixels(16.0), 1.0, {
                        let calls = calls.clone();
                        move |_, _, _, _| {
                            let _capture = &capture;
                            calls.set(calls.get() + 1);
                            size(Pixels(20.0 + frame as f32), Pixels(10.0))
                        }
                    });
                if let Some(previous_capture) = previous_capture.take() {
                    assert!(previous_capture.upgrade().is_none());
                }
                engine.set_request_scope(&scope);
                let root = engine.request_layout(Style::default(), Pixels(16.0), 1.0, &[measured]);
                if let Some(previous) = previous {
                    assert_eq!((measured, root), previous);
                }
                previous = Some((measured, root));
                let before = calls.get();
                engine.compute_layout(root, AvailableSpace::min_size(), window, cx);
                assert!(calls.get() > before);
                assert_eq!(
                    engine.layout_bounds(measured, 1.0).size,
                    size(Pixels(20.0 + frame as f32), Pixels(10.0))
                );
                engine.clear();
                assert!(weak_capture.upgrade().is_some());
                previous_capture = Some(weak_capture);
            }
            engine.set_request_scope(&scope);
            let unmeasured = engine.request_layout(Style::default(), Pixels(16.0), 1.0, &[]);
            assert!(
                previous_capture
                    .expect("previous capture")
                    .upgrade()
                    .is_none()
            );
            assert_eq!(unmeasured, previous.expect("previous frame").0);
            engine.compute_layout(unmeasured, AvailableSpace::min_size(), window, cx);
            assert_eq!(engine.layout_bounds(unmeasured, 1.0).size, Size::default());
            engine.clear();
            assert_eq!(engine.taffy.total_node_count(), 1);

            let abandoned_capture = Rc::new(());
            let weak_capture = Rc::downgrade(&abandoned_capture);
            engine.set_request_scope(&scope);
            engine.request_measured_layout(
                Style::default(),
                Pixels(16.0),
                1.0,
                move |_, _, _, _| {
                    let _capture = &abandoned_capture;
                    Size::default()
                },
            );
            engine.clear();
            assert!(weak_capture.upgrade().is_none());
            assert_eq!(engine.taffy.total_node_count(), 0);
        });
    }

    #[gpui::test]
    fn retained_nodes_mark_only_their_owners_and_survive_repeated_marking(
        cx: &mut crate::TestAppContext,
    ) {
        cx.add_empty_window().update(|window, cx| {
            let mut engine = TaffyLayoutEngine::new();
            engine.enable_cache();
            let child_scope = crate::element::InternedHandle::for_test(0, 0);
            let root_scope = crate::element::InternedHandle::for_test(1, 0);
            let unrelated_scope = crate::element::InternedHandle::for_test(2, 0);
            engine.set_request_scope(&child_scope);
            let child = engine.request_layout(Style::default(), Pixels(16.), 1., &[]);
            engine.set_request_scope(&root_scope);
            let root = engine.request_layout(Style::default(), Pixels(16.), 1., &[child]);
            engine.set_request_scope(&unrelated_scope);
            let unrelated = engine.request_layout(Style::default(), Pixels(16.), 1., &[]);
            engine.compute_layout(root, AvailableSpace::min_size(), window, cx);
            engine.compute_layout(unrelated, AvailableSpace::min_size(), window, cx);
            engine.clear();
            for _ in 0..3 {
                engine.retain_subtree(root);
                engine.retain_subtree(child);
                engine.retain_subtree(root);
                engine.clear();
                assert_eq!(engine.cache_counts(), (2, 0, 2));
            }
            engine.reset_retained_subtree(root);
            engine.set_request_scope(&child_scope);
            assert_eq!(
                engine.request_layout(Style::default(), Pixels(16.), 1., &[]),
                child
            );
            engine.set_request_scope(&root_scope);
            assert_eq!(
                engine.request_layout(Style::default(), Pixels(16.), 1., &[child]),
                root
            );
            engine.retain_subtree(root);
            engine.clear();
            assert_eq!(engine.cache_counts(), (2, 0, 2));
        });
    }

    #[gpui::test]
    fn unmeasured_leaf_preserves_box_styles(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            window.set_scale_factor(2.0);
            let mut engine = TaffyLayoutEngine::new();
            for (min_width, max_width, aspect_ratio, expected_size) in [
                (
                    Length::Auto,
                    Length::Auto,
                    None,
                    size(Pixels(10.0), Pixels(10.0)),
                ),
                (
                    Pixels(40.0).into(),
                    Length::Auto,
                    Some(2.0),
                    size(Pixels(40.0), Pixels(20.0)),
                ),
                (
                    Length::Auto,
                    Pixels(6.0).into(),
                    None,
                    size(Pixels(10.0), Pixels(10.0)),
                ),
            ] {
                let mut style = Style::default();
                style.padding = Edges::all(Pixels(4.0).into());
                style.border_widths = Edges::all(Pixels(1.0).into());
                style.min_size.width = min_width;
                style.max_size.width = max_width;
                style.aspect_ratio = aspect_ratio;
                let id = engine.request_layout(style, Pixels(16.0), 2.0, &[]);
                engine.compute_layout(
                    id,
                    size(AvailableSpace::MaxContent, AvailableSpace::MaxContent),
                    window,
                    cx,
                );
                assert_eq!(engine.layout_bounds(id, 2.0).size, expected_size);
            }
        });
    }

    #[gpui::test]
    fn measured_leaf_preserves_logical_units_and_ceil_snapping(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            window.set_scale_factor(2.0);
            let mut engine = TaffyLayoutEngine::new();
            for (width, expected_available_width, expected_width) in [
                (Length::Auto, AvailableSpace::MaxContent, Pixels(20.5)),
                (
                    Pixels(30.0).into(),
                    AvailableSpace::Definite(Pixels(20.0)),
                    Pixels(30.0),
                ),
            ] {
                let mut style = Style::default();
                style.size.width = width;
                style.padding = Edges::all(Pixels(4.0).into());
                style.border_widths = Edges::all(Pixels(1.0).into());
                let measured = std::rc::Rc::new(std::cell::Cell::new(false));
                let id = engine.request_measured_layout(style, Pixels(16.0), 2.0, {
                    let measured = measured.clone();
                    move |_, available_space, _, _| {
                        measured.set(true);
                        assert_eq!(available_space.width, expected_available_width);
                        assert_eq!(available_space.height, AvailableSpace::MaxContent);
                        size(Pixels(10.2), Pixels(20.2))
                    }
                });
                engine.compute_layout(
                    id,
                    size(AvailableSpace::MaxContent, AvailableSpace::MaxContent),
                    window,
                    cx,
                );
                assert!(measured.get());
                assert_eq!(
                    engine.layout_bounds(id, 2.0).size,
                    size(expected_width, Pixels(30.5))
                );
            }
        });
    }

    #[test]
    fn border_widths_to_taffy_use_stroke_snapping() {
        let border_widths = Edges {
            top: Pixels(0.0).into(),
            right: Pixels(0.4).into(),
            bottom: Pixels(0.5).into(),
            left: Pixels(1.6).into(),
        };
        let taffy_border = border_widths_to_taffy(&border_widths, Pixels(16.0), 1.0);

        assert_eq!(
            taffy_border.top,
            taffy::style::LengthPercentage::length(0.0)
        );
        assert_eq!(
            taffy_border.right,
            taffy::style::LengthPercentage::length(1.0)
        );
        assert_eq!(
            taffy_border.bottom,
            taffy::style::LengthPercentage::length(1.0)
        );
        assert_eq!(
            taffy_border.left,
            taffy::style::LengthPercentage::length(2.0)
        );
    }

    #[gpui::test]
    fn fractional_scale_root_dimensions(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            for (scale_factor, explicit_size, percentage_size) in [
                (1.25, size(26.0, 16.0), size(63.0, 42.0)),
                (1.5, size(31.0, 19.0), size(76.0, 50.0)),
            ] {
                window.set_scale_factor(scale_factor);
                for percentage in [false, true] {
                    let mut engine = TaffyLayoutEngine::new();
                    let mut style = Style::default();
                    style.size = if percentage {
                        size(
                            DefiniteLength::Fraction(0.5).into(),
                            DefiniteLength::Fraction(0.5).into(),
                        )
                    } else {
                        size(Pixels(21.0).into(), Pixels(13.0).into())
                    };
                    let root = engine.request_layout(style, Pixels(16.0), scale_factor, &[]);
                    let available_size = size(Pixels(101.0), Pixels(67.0));
                    engine.stretch_auto_size_to_fill(root, available_size, scale_factor);
                    engine.compute_layout(root, available_size.into(), window, cx);

                    // Author absolute sizes in already-snapped device pixels, independently
                    // of Style::to_taffy; percentages resolve against unsnapped available space.
                    let mut direct = TaffyTree::<()>::new();
                    direct.disable_rounding();
                    let direct_root = direct
                        .new_leaf(taffy::Style {
                            display: taffy::Display::Block,
                            size: if percentage {
                                TaffySize {
                                    width: taffy::Dimension::percent(0.5),
                                    height: taffy::Dimension::percent(0.5),
                                }
                            } else {
                                TaffySize {
                                    width: taffy::Dimension::length(explicit_size.width),
                                    height: taffy::Dimension::length(explicit_size.height),
                                }
                            },
                            ..Default::default()
                        })
                        .expect(EXPECT_MESSAGE);
                    direct
                        .compute_layout(
                            direct_root,
                            TaffySize {
                                width: TaffyAvailableSpace::Definite(101.0 * scale_factor),
                                height: TaffyAvailableSpace::Definite(67.0 * scale_factor),
                            },
                        )
                        .expect(EXPECT_MESSAGE);
                    let direct_layout = direct.layout(direct_root).expect(EXPECT_MESSAGE);
                    let adapter_layout = engine.taffy.layout(root.into()).expect(EXPECT_MESSAGE);
                    assert_eq!(adapter_layout.location, direct_layout.location);
                    assert_eq!(adapter_layout.size, direct_layout.size);

                    let expected_size = if percentage {
                        percentage_size
                    } else {
                        explicit_size
                    };
                    assert_eq!(
                        engine.layout_bounds(root, scale_factor),
                        Bounds::new(
                            Point::default(),
                            expected_size.map(|value| Pixels(value / scale_factor)),
                        ),
                        "scale={scale_factor}, percentage={percentage}",
                    );
                }
            }
        });
    }

    #[gpui::test]
    fn fractional_scale_nested_flex_edges_close(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            // 101 logical pixels snap to 126 or 151 device pixels before flex
            // distribution. Snap the resulting absolute quarter edges, not local widths.
            for (scale_factor, width, height, edges) in [
                (1.25, 126.0, 11.0, [0.0, 31.0, 63.0, 94.0, 126.0]),
                (1.5, 151.0, 13.0, [0.0, 38.0, 75.0, 113.0, 151.0]),
            ] {
                window.set_scale_factor(scale_factor);
                let mut engine = TaffyLayoutEngine::new();
                let flex_item = Style {
                    display: crate::Display::Flex,
                    flex_grow: 1.0,
                    flex_basis: Pixels(0.0).into(),
                    min_size: size(Pixels(0.0).into(), Pixels(0.0).into()),
                    ..Default::default()
                };
                let leaves: Vec<_> = (0..4)
                    .map(|_| {
                        engine.request_layout(flex_item.clone(), Pixels(16.0), scale_factor, &[])
                    })
                    .collect();
                let parents: Vec<_> = leaves
                    .chunks_exact(2)
                    .map(|children| {
                        engine.request_layout(
                            flex_item.clone(),
                            Pixels(16.0),
                            scale_factor,
                            children,
                        )
                    })
                    .collect();
                let root = engine.request_layout(
                    Style {
                        display: crate::Display::Flex,
                        size: size(Pixels(101.0).into(), Pixels(9.0).into()),
                        ..Default::default()
                    },
                    Pixels(16.0),
                    scale_factor,
                    &parents,
                );
                engine.compute_layout(
                    root,
                    size(AvailableSpace::MaxContent, AvailableSpace::MaxContent),
                    window,
                    cx,
                );

                for (&leaf, edge_pair) in leaves.iter().zip(edges.windows(2)) {
                    let raw = engine.taffy.layout(leaf.into()).expect(EXPECT_MESSAGE);
                    assert_eq!(raw.size.width, width / 4.0);
                    assert_eq!(
                        engine.layout_bounds(leaf, scale_factor),
                        Bounds::new(
                            Point::new(Pixels(edge_pair[0] / scale_factor), Pixels(0.0)),
                            size(
                                Pixels((edge_pair[1] - edge_pair[0]) / scale_factor),
                                Pixels(height / scale_factor),
                            ),
                        ),
                        "scale={scale_factor}, edges={edge_pair:?}",
                    );
                }
                for (&parent, edge_pair) in parents.iter().zip(edges.windows(3).step_by(2)) {
                    assert_eq!(
                        engine.layout_bounds(parent, scale_factor),
                        Bounds::new(
                            Point::new(Pixels(edge_pair[0] / scale_factor), Pixels(0.0)),
                            size(
                                Pixels((edge_pair[2] - edge_pair[0]) / scale_factor),
                                Pixels(height / scale_factor),
                            ),
                        ),
                    );
                }
                assert_eq!(
                    engine.layout_bounds(root, scale_factor).size,
                    size(Pixels(width / scale_factor), Pixels(height / scale_factor)),
                );
            }
        });
    }

    #[gpui::test]
    fn fractional_scale_padding_and_overflow_origins(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            for (scale_factor, root_size, child_size, leaf_size) in [
                (1.25, size(26.0, 16.0), size(39.0, 21.0), 2.0),
                (1.5, size(31.0, 19.0), size(46.0, 25.0), 3.0),
            ] {
                window.set_scale_factor(scale_factor);
                for overflow in [crate::Overflow::Visible, crate::Overflow::Hidden] {
                    let mut engine = TaffyLayoutEngine::new();
                    let leaf = engine.request_layout(
                        Style {
                            size: size(Pixels(2.0).into(), Pixels(2.0).into()),
                            ..Default::default()
                        },
                        Pixels(16.0),
                        scale_factor,
                        &[],
                    );
                    let child = engine.request_layout(
                        Style {
                            size: size(Pixels(31.0).into(), Pixels(17.0).into()),
                            flex_shrink: 0.0,
                            padding: Edges::all(Pixels(1.0).into()),
                            ..Default::default()
                        },
                        Pixels(16.0),
                        scale_factor,
                        &[leaf],
                    );
                    let root = engine.request_layout(
                        Style {
                            display: crate::Display::Flex,
                            size: size(Pixels(21.0).into(), Pixels(13.0).into()),
                            padding: Edges {
                                top: Pixels(3.0).into(),
                                ..Edges::all(Pixels(1.0).into())
                            },
                            border_widths: Edges::all(Pixels(1.0).into()),
                            overflow: Point::new(overflow, overflow),
                            ..Default::default()
                        },
                        Pixels(16.0),
                        scale_factor,
                        &[child],
                    );
                    engine.compute_layout(
                        root,
                        size(AvailableSpace::MaxContent, AvailableSpace::MaxContent),
                        window,
                        cx,
                    );

                    // At both scales, 1 logical pixel snaps to 1 device pixel,
                    // and the top padding of 3 snaps to 4. Overflow must not
                    // clip layout bounds or add padding again at each ancestor.
                    for (id, origin, expected_size) in [
                        (root, Point::new(0.0, 0.0), root_size),
                        (child, Point::new(2.0, 5.0), child_size),
                        (leaf, Point::new(3.0, 6.0), size(leaf_size, leaf_size)),
                    ] {
                        assert_eq!(
                            engine.layout_bounds(id, scale_factor),
                            Bounds::new(
                                origin.map(|value| Pixels(value / scale_factor)),
                                expected_size.map(|value| Pixels(value / scale_factor)),
                            ),
                            "scale={scale_factor}, overflow={overflow:?}, node={id:?}",
                        );
                    }
                }
            }
        });
    }
}
