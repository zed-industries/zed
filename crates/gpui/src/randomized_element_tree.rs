//! A seeded, mutable element tree for benchmarks and tests of GPUI rendering.
use crate::{
    AlignContent, AlignItems, AlignSelf, AnimationExt as _, AnyElement, App, AppContext as _,
    AvailableSpace, Bounds, Context, Element, ElementId, Entity, GlobalElementId, HighlightStyle,
    Hsla, InspectorElementId, InteractiveElement, InteractiveText, IntoElement, JustifyContent,
    LayoutId, ListAlignment, ListOffset, ListSizingBehavior, ListState, ParentElement, Pixels,
    Render, Size, StatefulInteractiveElement, StrikethroughStyle, Style, Styled, StyledText,
    TextAlign, TextOverflow, UnderlineStyle, UniformListScrollHandle, WhiteSpace, Window, anchored,
    deferred, div, fill, list, point, px, relative, rems, size, uniform_list, util::FluentBuilder,
};
use collections::{HashMap, HashSet};
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use std::{
    cell::RefCell,
    ops::RangeInclusive,
    rc::{Rc, Weak},
};

/// Dispatched to no one: the action generated elements register handlers for. Not
/// registered, so builds that enable `bench-support` through feature unification (such as
/// the workspace's tests) don't gain an action namespace.
#[derive(Clone, Default, PartialEq, crate::Action)]
#[action(namespace = randomized_element_tree, no_json, no_register)]
pub struct RandomizedElementTreeAction;

/// The initial shape generated for a randomized element tree.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RandomizedElementTreeTopology {
    /// Every generated element is a direct child of the root.
    Wide,
    /// Every generated element is the child of the previous element.
    Deep,
    /// Generated elements choose parents throughout the existing tree.
    Mixed,
    /// Generated elements usually extend one deep branch, with occasional root children.
    Narrow,
}

impl RandomizedElementTreeTopology {
    /// Every topology.
    pub const ALL: [Self; 4] = [Self::Wide, Self::Deep, Self::Mixed, Self::Narrow];

    /// Whether the tree's depth grows with its element count. `Deep` nests every element
    /// under the previous one and `Narrow` nests four in five, so both recurse once per
    /// element through layout and paint; without gpui's `stacker` feature that overflows
    /// a 2 MB thread stack somewhere between 384 and 512 elements in a release build.
    pub fn is_tall(self) -> bool {
        matches!(self, Self::Deep | Self::Narrow)
    }

    fn from_seed(seed: u64) -> Self {
        match seed % 4 {
            0 => Self::Wide,
            1 => Self::Deep,
            2 => Self::Mixed,
            _ => Self::Narrow,
        }
    }
}

/// Configuration for a reproducible randomized element-tree workload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RandomizedElementTreeConfig {
    seed: u64,
    element_count: usize,
    topology: RandomizedElementTreeTopology,
    entity_density: f64,
    handler_density: f64,
    animation_density: f64,
    cached_views: bool,
    hover_styles: bool,
    paths: bool,
}

impl RandomizedElementTreeConfig {
    /// Creates an element-only workload whose topology is selected by `seed`.
    pub fn new(seed: u64, element_count: usize) -> Self {
        Self {
            seed,
            element_count,
            topology: RandomizedElementTreeTopology::from_seed(seed),
            entity_density: 0.0,
            handler_density: 0.0,
            animation_density: 0.0,
            cached_views: false,
            hover_styles: false,
            paths: true,
        }
    }

    /// Use definite-size cached entity boundaries in this separate workload variant.
    pub fn with_cached_views(mut self, cached: bool) -> Self {
        self.cached_views = cached;
        self
    }

    /// Sets the probability that each entity-backed element shows a looping animation, like
    /// a spinner, so its entity re-renders on every frame while the window is open.
    pub fn with_animation_density(mut self, density: f64) -> Self {
        self.animation_density = normalized_density(density);
        self
    }

    /// Gives every element a hover style and a group-hover style keyed to the tree's root.
    /// Off by default, so workloads that never move the pointer don't pay for them.
    pub fn with_hover_styles(mut self, hover_styles: bool) -> Self {
        self.hover_styles = hover_styles;
        self
    }

    /// Whether elements paint vector paths. Path tessellation dominates the cost of
    /// frames that draw them, so turning them off isolates layout and the rest of the
    /// frame; elements keep the canvas that would have painted them, so layout is the same.
    pub fn with_paths(mut self, paths: bool) -> Self {
        self.paths = paths;
        self
    }

    /// Selects the initial tree topology independently of the RNG seed.
    pub fn with_topology(mut self, topology: RandomizedElementTreeTopology) -> Self {
        self.topology = topology;
        self
    }

    /// Sets the probability that each generated element is backed by a persistent entity.
    pub fn with_entity_density(mut self, density: f64) -> Self {
        self.entity_density = normalized_density(density);
        self
    }

    /// Sets the probability of each generated interaction feature on an element.
    pub fn with_handler_density(mut self, density: f64) -> Self {
        self.handler_density = normalized_density(density);
        self
    }

    /// Returns the seed used for generation and subsequent random mutations.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Returns the requested number of initial descendants.
    pub fn element_count(&self) -> usize {
        self.element_count
    }

    /// Returns the selected initial topology.
    pub fn topology(&self) -> RandomizedElementTreeTopology {
        self.topology
    }

    /// Returns the probability that a generated element is backed by an entity.
    pub fn entity_density(&self) -> f64 {
        self.entity_density
    }

    /// Returns the probability of each generated interaction feature.
    pub fn handler_density(&self) -> f64 {
        self.handler_density
    }
}

/// Bounds an RNG is drawn against to produce a [`RandomizedElementTreeConfig`]: a family
/// of trees, such as "tall trees of 32 to 256 elements with up to half of them entities",
/// from which one draw picks one member. Every field of the resulting config, and the
/// tree built from it, follows from the RNG's state, so an RNG seeded by `#[gpui::bench]`
/// or `#[gpui::test]` samples the family across seeds and any one seed reproduces exactly.
#[derive(Clone, Debug, PartialEq)]
pub struct RandomizedElementTreeBounds {
    topologies: Vec<RandomizedElementTreeTopology>,
    element_count: RangeInclusive<usize>,
    entity_density: RangeInclusive<f64>,
    handler_density: RangeInclusive<f64>,
}

impl RandomizedElementTreeBounds {
    /// Trees of any topology with `element_count` elements, no entities and no handlers.
    pub fn new(element_count: RangeInclusive<usize>) -> Self {
        Self {
            topologies: RandomizedElementTreeTopology::ALL.to_vec(),
            element_count,
            entity_density: 0.0..=0.0,
            handler_density: 0.0..=0.0,
        }
    }

    /// Restricts the topologies a seed may pick. An empty list keeps them all.
    pub fn with_topologies(
        mut self,
        topologies: impl IntoIterator<Item = RandomizedElementTreeTopology>,
    ) -> Self {
        let topologies: Vec<_> = topologies.into_iter().collect();
        if !topologies.is_empty() {
            self.topologies = topologies;
        }
        self
    }

    /// The range a seed draws the share of entity-backed elements from.
    pub fn with_entity_density(mut self, density: RangeInclusive<f64>) -> Self {
        self.entity_density =
            normalized_density(*density.start())..=normalized_density(*density.end());
        self
    }

    /// The range a seed draws the per-feature handler probability from.
    pub fn with_handler_density(mut self, density: RangeInclusive<f64>) -> Self {
        self.handler_density =
            normalized_density(*density.start())..=normalized_density(*density.end());
        self
    }

    /// Draws one config from these bounds. The config's own seed, which drives the tree's
    /// contents and mutation stream, is drawn from `rng` too, so all of it reproduces from
    /// the seed `rng` started from.
    pub fn sample(&self, rng: &mut StdRng) -> RandomizedElementTreeConfig {
        let topology = self.topologies[rng.random_range(0..self.topologies.len())];
        let element_count = rng.random_range(self.element_count.clone());
        RandomizedElementTreeConfig::new(rng.random(), element_count)
            .with_topology(topology)
            .with_entity_density(sample_range(&self.entity_density, rng))
            .with_handler_density(sample_range(&self.handler_density, rng))
    }
}

fn sample_range(range: &RangeInclusive<f64>, rng: &mut StdRng) -> f64 {
    if range.start() >= range.end() {
        *range.start()
    } else {
        rng.random_range(range.clone())
    }
}

/// Work performed while converting randomized entities and elements into an element tree.
///
/// Recorded only in tests: benchmarks must not pay for bookkeeping, and the rendered-ID
/// history would otherwise grow for as long as a benchmark draws without resetting it.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RandomizedElementTreeWorkCounters {
    #[cfg(test)]
    root_render_count: usize,
    #[cfg(test)]
    entity_render_count: usize,
    #[cfg(test)]
    element_render_count: usize,
    #[cfg(test)]
    handler_registration_count: usize,
    #[cfg(test)]
    rendered_entity_ids: Vec<u64>,
}

impl RandomizedElementTreeWorkCounters {
    fn record_root_render(&mut self) {
        #[cfg(test)]
        {
            self.root_render_count += 1;
        }
    }

    fn record_entity_render(&mut self, _element_id: u64) {
        #[cfg(test)]
        {
            self.entity_render_count += 1;
            self.rendered_entity_ids.push(_element_id);
        }
    }

    fn record_element_render(&mut self, _handler_count: usize) {
        #[cfg(test)]
        {
            self.element_render_count += 1;
            self.handler_registration_count += _handler_count;
        }
    }
}

#[cfg(test)]
impl RandomizedElementTreeWorkCounters {
    /// Returns how many times the persistent root's `Render` implementation ran.
    pub fn root_render_count(&self) -> usize {
        self.root_render_count
    }

    /// Returns how many randomized child entities rendered.
    pub fn entity_render_count(&self) -> usize {
        self.entity_render_count
    }

    /// Returns how many randomized descendant elements were reconstructed.
    pub fn element_render_count(&self) -> usize {
        self.element_render_count
    }

    /// Returns how many interaction handlers were registered while rendering.
    pub fn handler_registration_count(&self) -> usize {
        self.handler_registration_count
    }

    /// Returns stable IDs for the randomized child entities that rendered, in render order.
    pub fn rendered_entity_ids(&self) -> &[u64] {
        &self.rendered_entity_ids
    }

    fn reset(&mut self) {
        self.root_render_count = 0;
        self.entity_render_count = 0;
        self.element_render_count = 0;
        self.handler_registration_count = 0;
        self.rendered_entity_ids.clear();
    }
}

/// A cloneable description of a randomized element tree's renderable state.
#[derive(Clone, Debug, PartialEq)]
pub struct RandomizedElementTreeSnapshot {
    root: RandomizedElementStyle,
    children: Vec<RandomizedElementNode>,
    next_element_id: u64,
    render_options: RenderOptions,
    paths_by_element_id: HashMap<u64, Vec<usize>>,
}

impl RandomizedElementTreeSnapshot {
    /// Returns the number of randomized elements below the persistent root.
    pub fn descendant_count(&self) -> usize {
        let mut count = 0;
        let mut stack = self.children.iter().collect::<Vec<_>>();
        while let Some(node) = stack.pop() {
            count += 1;
            stack.extend(node.children.iter());
        }
        count
    }

    /// Returns the deepest randomized element below the persistent root.
    ///
    /// A snapshot with no descendants has depth zero.
    pub fn max_depth(&self) -> usize {
        let mut max_depth = 0;
        let mut stack = self
            .children
            .iter()
            .map(|node| (node, 1))
            .collect::<Vec<_>>();
        while let Some((node, depth)) = stack.pop() {
            max_depth = max_depth.max(depth);
            stack.extend(node.children.iter().map(|child| (child, depth + 1)));
        }
        max_depth
    }

    /// Returns the stable element IDs in depth-first order.
    pub fn element_ids(&self) -> Vec<u64> {
        node_paths(&self.children)
            .iter()
            .filter_map(|path| node_at_path(&self.children, path).map(|node| node.id))
            .collect()
    }

    /// Returns the number of descendants backed by persistent child entities.
    pub fn entity_count(&self) -> usize {
        let mut count = 0;
        let mut stack = self.children.iter().collect::<Vec<_>>();
        while let Some(node) = stack.pop() {
            count += usize::from(node.is_entity);
            stack.extend(node.children.iter());
        }
        count
    }

    /// Returns the number of interaction handlers in the generated tree.
    pub fn handler_count(&self) -> usize {
        let mut count = 0;
        let mut stack = self.children.iter().collect::<Vec<_>>();
        while let Some(node) = stack.pop() {
            count += node.features.handler_count();
            stack.extend(node.children.iter());
        }
        count
    }

    fn rebuild_element_paths(&mut self) {
        self.paths_by_element_id = node_paths(&self.children)
            .into_iter()
            .filter_map(|path| node_at_path(&self.children, &path).map(|node| (node.id, path)))
            .collect();
    }

    fn node(&self, element_id: u64) -> Option<&RandomizedElementNode> {
        node_at_path(&self.children, self.paths_by_element_id.get(&element_id)?)
    }

    /// Returns element IDs for keyed nodes, matching the `"randomized-element"` entries
    /// whose element state is recorded during `request_layout`.
    #[cfg(test)]
    fn keyed_element_ids(&self) -> Vec<u64> {
        let mut ids = Vec::new();
        let mut stack = self.children.iter().collect::<Vec<_>>();
        while let Some(node) = stack.pop() {
            if node.is_keyed() {
                ids.push(node.id);
            }
            stack.extend(node.children.iter());
        }
        ids
    }
}

/// A requested class of deterministic randomized-tree mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RandomizedElementTreeMutationKind {
    /// Change the root background color.
    RootColor,
    /// Change the root opacity.
    RootOpacity,
    /// Change the root border.
    RootBorder,
    /// Change the root width and height.
    RootBounds,
    /// Toggle clipping on the root.
    RootClipping,
    /// Toggle root visibility.
    RootVisibility,
    /// Change one field of the root's inherited default text style.
    RootTextStyle,
    /// Cycle the root's optional scope wrapper element ID.
    RootScope,
    /// Change a seeded random descendant's colors.
    ChildColor,
    /// Change a seeded random descendant's opacity.
    ChildOpacity,
    /// Change a seeded random descendant's border.
    ChildBorder,
    /// Change a seeded random descendant's width and height.
    ChildBounds,
    /// Toggle clipping on a seeded random descendant.
    ChildClipping,
    /// Toggle visibility on a seeded random descendant.
    ChildVisibility,
    /// Change a seeded random descendant's text, adding text to one that had none or
    /// removing it from one that didn't always have it.
    ChildText,
    /// Toggle `Display::None` on a seeded random descendant.
    ChildDisplay,
    /// Mutate layout properties (`auto` sizing, `rems`, margin, flex grow/shrink,
    /// direction, wrap, alignment, aspect ratio, unkeyed ID) on a seeded random descendant.
    ChildLayout,
    /// Mutate text styling (`color`, `font_size`, `line_height`, `white_space`,
    /// `text_overflow`, `line_clamp`, `text_align`, runs/interactive text) on a descendant.
    ChildTextStyle,
    /// Mutate a descendant's measured leaf or inferred-size list content.
    ChildMeasuredContent,
    /// Mutate a descendant's deferred anchored popup.
    ChildPopup,
    /// Reorder a seeded random descendant among its siblings.
    Reorder,
    /// Insert a generated descendant below a seeded random parent.
    Insert,
    /// Remove a seeded random descendant and its subtree.
    Remove,
    /// Remove a seeded random childless descendant. Unlike [`Self::Remove`] this changes
    /// the element count by exactly one, so it can alternate with [`Self::Insert`] in a
    /// long benchmark loop without the tree collapsing or growing.
    RemoveLeaf,
}

impl RandomizedElementTreeMutationKind {
    /// Every supported mutation kind, suitable for parameterized tests and benchmarks.
    pub const ALL: [Self; 24] = [
        Self::RootColor,
        Self::RootOpacity,
        Self::RootBorder,
        Self::RootBounds,
        Self::RootClipping,
        Self::RootVisibility,
        Self::RootTextStyle,
        Self::RootScope,
        Self::ChildColor,
        Self::ChildOpacity,
        Self::ChildBorder,
        Self::ChildBounds,
        Self::ChildClipping,
        Self::ChildVisibility,
        Self::ChildText,
        Self::ChildDisplay,
        Self::ChildLayout,
        Self::ChildTextStyle,
        Self::ChildMeasuredContent,
        Self::ChildPopup,
        Self::Reorder,
        Self::Insert,
        Self::Remove,
        Self::RemoveLeaf,
    ];
}

/// A deterministic mutation applied to a [`RandomizedElementTree`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RandomizedElementTreeMutation {
    /// Changed the root background color.
    RootColor,
    /// Changed the root opacity.
    RootOpacity,
    /// Changed the root border.
    RootBorder,
    /// Changed the root width and height.
    RootBounds,
    /// Toggled clipping on the root.
    RootClipping,
    /// Toggled root visibility.
    RootVisibility,
    /// Changed the root's inherited default text style.
    RootTextStyle,
    /// Changed the root's scope wrapper element ID.
    RootScope,
    /// Changed a descendant's colors.
    ChildColor {
        /// The stable ID of the mutated element.
        element_id: u64,
    },
    /// Changed a descendant's opacity.
    ChildOpacity {
        /// The stable ID of the mutated element.
        element_id: u64,
    },
    /// Changed a descendant's border.
    ChildBorder {
        /// The stable ID of the mutated element.
        element_id: u64,
    },
    /// Changed a descendant's width and height.
    ChildBounds {
        /// The stable ID of the mutated element.
        element_id: u64,
    },
    /// Toggled clipping on a descendant.
    ChildClipping {
        /// The stable ID of the mutated element.
        element_id: u64,
    },
    /// Toggled a descendant's visibility.
    ChildVisibility {
        /// The stable ID of the mutated element.
        element_id: u64,
    },
    /// Changed a descendant's text.
    ChildText {
        /// The stable ID of the mutated element.
        element_id: u64,
    },
    /// Toggled `Display::None` on a descendant.
    ChildDisplay {
        /// The stable ID of the mutated element.
        element_id: u64,
    },
    /// Changed a descendant's layout properties.
    ChildLayout {
        /// The stable ID of the mutated element.
        element_id: u64,
    },
    /// Changed a descendant's text styling.
    ChildTextStyle {
        /// The stable ID of the mutated element.
        element_id: u64,
    },
    /// Changed a descendant's measured leaf or list content.
    ChildMeasuredContent {
        /// The stable ID of the mutated element.
        element_id: u64,
    },
    /// Changed a descendant's deferred anchored popup.
    ChildPopup {
        /// The stable ID of the mutated element.
        element_id: u64,
    },
    /// Moved an element to a different position among its siblings.
    Reordered {
        /// The stable ID of the moved element.
        element_id: u64,
        /// The stable ID of its parent, or `None` for the persistent root.
        parent_id: Option<u64>,
        /// The element's previous position.
        old_index: usize,
        /// The element's new position.
        new_index: usize,
    },
    /// Inserted a newly generated element.
    Inserted {
        /// The stable ID of the inserted element.
        element_id: u64,
        /// The stable ID of its parent, or `None` for the persistent root.
        parent_id: Option<u64>,
        /// The position of the inserted element.
        index: usize,
    },
    /// Removed an element and all of its descendants.
    Removed {
        /// The stable ID of the removed element.
        element_id: u64,
        /// The stable ID of its parent, or `None` for the persistent root.
        parent_id: Option<u64>,
        /// The former position of the removed element.
        index: usize,
        /// The total number of elements removed with the subtree.
        removed_element_count: usize,
    },
}

/// Where the elements changed by one frame sit relative to each other.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeLocality {
    /// Inside one random entity's subtree, as a user action or one component's state
    /// change would be; the change reaches the rest of the tree only if the subtree has
    /// fewer elements than are changing (a common ancestor changing).
    Localized,
    /// Anywhere in the tree, uniformly: the worst case for a renderer that reuses clean
    /// subtrees, since every changed element may sit in a different one.
    Spread,
}

/// What [`RandomizedElementTree::recolor_children`] changed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecoloredChildren {
    /// Descendants whose background changed.
    pub recolored: usize,
    /// Entity boundaries (or the root) notified, each once.
    pub notified: usize,
}

/// A persistent root view for randomized GPUI element-tree tests and benchmarks.
pub struct RandomizedElementTree {
    snapshot: Rc<RefCell<RandomizedElementTreeSnapshot>>,
    entities: Rc<RefCell<RandomizedElementEntities>>,
    scroll_handles: Rc<RefCell<HashMap<u64, crate::ScrollHandle>>>,
    list_states: Rc<RefCell<HashMap<u64, (ListState, u64)>>>,
    uniform_list_scroll_handles: Rc<RefCell<HashMap<u64, (UniformListScrollHandle, u64)>>>,
    work_counters: Rc<RefCell<RandomizedElementTreeWorkCounters>>,
    config: RandomizedElementTreeConfig,
    rng: StdRng,
}

impl RandomizedElementTree {
    /// Generates a tree with `element_count` descendants and a mutation stream from `seed`.
    ///
    /// Different seeds deliberately produce wide, deep, mixed, and narrow initial topologies.
    pub fn new(seed: u64, element_count: usize) -> Self {
        Self::from_config(RandomizedElementTreeConfig::new(seed, element_count))
    }

    /// Generates a configured tree and creates its persistent child entities.
    pub fn new_with_config(config: RandomizedElementTreeConfig, cx: &mut Context<Self>) -> Self {
        let mut tree = Self::from_config(config);
        tree.synchronize_entities(cx);
        tree
    }

    fn from_config(config: RandomizedElementTreeConfig) -> Self {
        let mut rng = StdRng::seed_from_u64(config.seed);
        let mut children = Vec::new();
        let mut deepest_path = Vec::new();

        for element_index in 0..config.element_count {
            let mut parent_path = match config.topology {
                RandomizedElementTreeTopology::Wide => Vec::new(),
                RandomizedElementTreeTopology::Deep => deepest_path.clone(),
                RandomizedElementTreeTopology::Mixed => random_parent_path(&children, &mut rng),
                RandomizedElementTreeTopology::Narrow => {
                    if rng.random_bool(0.8) {
                        deepest_path.clone()
                    } else {
                        Vec::new()
                    }
                }
            };
            let node = RandomizedElementNode::random(element_index as u64, &config, &mut rng);
            let (_, inserted_path) = insert_node(&mut children, &mut parent_path, node);
            deepest_path = inserted_path;
        }

        let mut snapshot = RandomizedElementTreeSnapshot {
            root: RandomizedElementStyle::random_root(&mut rng),
            children,
            next_element_id: config.element_count as u64,
            render_options: RenderOptions {
                cached_views: config.cached_views,
                hover_styles: config.hover_styles,
                paths: config.paths,
                id_generation: 0,
                root_scope: 0,
            },
            paths_by_element_id: HashMap::default(),
        };
        snapshot.rebuild_element_paths();

        Self {
            snapshot: Rc::new(RefCell::new(snapshot)),
            entities: Rc::new(RefCell::new(HashMap::default())),
            scroll_handles: Rc::default(),
            list_states: Rc::default(),
            uniform_list_scroll_handles: Rc::default(),
            work_counters: Rc::new(RefCell::new(RandomizedElementTreeWorkCounters::default())),
            config,
            rng,
        }
    }

    /// Reconstructs a tree from renderable state captured by [`Self::snapshot`].
    pub fn from_snapshot(snapshot: RandomizedElementTreeSnapshot) -> Self {
        Self {
            config: RandomizedElementTreeConfig::new(0, snapshot.descendant_count()),
            snapshot: Rc::new(RefCell::new(snapshot)),
            entities: Rc::new(RefCell::new(HashMap::default())),
            scroll_handles: Rc::default(),
            list_states: Rc::default(),
            uniform_list_scroll_handles: Rc::default(),
            work_counters: Rc::new(RefCell::new(RandomizedElementTreeWorkCounters::default())),
            rng: StdRng::seed_from_u64(0),
        }
    }

    /// Reconstructs a snapshot while preserving its randomized entity boundaries.
    pub fn from_snapshot_with_entities(
        snapshot: RandomizedElementTreeSnapshot,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut tree = Self::from_snapshot(snapshot);
        tree.synchronize_entities(cx);
        tree
    }

    /// Captures all state that affects rendering and future element IDs.
    pub fn snapshot(&self) -> RandomizedElementTreeSnapshot {
        self.snapshot.borrow().clone()
    }

    /// Clears render work accumulated since construction or the previous reset.
    #[cfg(test)]
    pub fn reset_work_counters(&mut self) {
        self.work_counters.borrow_mut().reset();
    }

    /// Returns work accumulated since construction or the previous reset.
    #[cfg(test)]
    pub fn work_counters(&self) -> RandomizedElementTreeWorkCounters {
        self.work_counters.borrow().clone()
    }

    /// Applies one seeded random style or structural mutation.
    ///
    /// This does not notify persistent child entities. Entity-aware callers should use
    /// [`Self::apply_random_mutation`] from an entity update instead.
    pub fn mutate(&mut self) -> RandomizedElementTreeMutation {
        let kind = RandomizedElementTreeMutationKind::ALL[self
            .rng
            .random_range(0..RandomizedElementTreeMutationKind::ALL.len())];
        self.apply_mutation_without_notification(kind).mutation
    }

    /// Applies one seeded random mutation and notifies its nearest persistent entity boundary.
    pub fn apply_random_mutation(
        &mut self,
        cx: &mut Context<Self>,
    ) -> RandomizedElementTreeMutation {
        let kind = RandomizedElementTreeMutationKind::ALL[self
            .rng
            .random_range(0..RandomizedElementTreeMutationKind::ALL.len())];
        self.apply_mutation(kind, cx)
    }

    /// Applies a selected mutation kind and notifies its nearest persistent entity boundary.
    pub fn apply_mutation(
        &mut self,
        kind: RandomizedElementTreeMutationKind,
        cx: &mut Context<Self>,
    ) -> RandomizedElementTreeMutation {
        let applied = self.apply_mutation_without_notification(kind);
        if applied.synchronize_entities {
            self.synchronize_entities(cx);
        }
        self.notify_owner(applied.owner_entity_id, cx);
        applied.mutation
    }

    /// Picks a seeded random element, at any depth, or `None` if the tree is empty.
    pub fn random_element_id(&mut self) -> Option<u64> {
        let snapshot = self.snapshot.borrow();
        let path = random_node_path(&snapshot.children, &mut self.rng)?;
        node_at_path(&snapshot.children, &path).map(|node| node.id)
    }

    /// Alternates an element's background between two colors without changing the tree's
    /// shape, notifying its nearest persistent entity boundary. Returns `false` if the
    /// element is no longer in the tree.
    pub fn alternate_element_color(&mut self, element_id: u64, cx: &mut Context<Self>) -> bool {
        let owner = {
            let mut snapshot = self.snapshot.borrow_mut();
            let Some(path) = snapshot.paths_by_element_id.get(&element_id).cloned() else {
                return false;
            };
            let owner = nearest_entity_id(&snapshot.children, &path);
            let Some(node) = node_at_path_mut(&mut snapshot.children, &path) else {
                return false;
            };
            let first = crate::hsla(0., 1., 0.5, 1.);
            let second = crate::hsla(0.5, 1., 0.5, 1.);
            node.style.background = if node.style.background == first {
                second
            } else {
                first
            };
            owner
        };
        self.notify_owner(owner, cx);
        true
    }

    /// Gives every element a new ID, as when a list's items are keyed by data that
    /// changes every frame, and re-renders the whole tree. Nothing else changes, so each
    /// element's state is lost but its layout and paint are as before.
    pub fn churn_element_ids(&mut self, cx: &mut Context<Self>) {
        self.snapshot.borrow_mut().render_options.id_generation += 1;
        cx.notify();
    }

    /// Generates and inserts an element below a seeded random parent.
    ///
    /// This does not synchronize or notify persistent child entities. Entity-aware callers should
    /// use [`Self::apply_mutation`] with [`RandomizedElementTreeMutationKind::Insert`].
    pub fn insert_child(&mut self) -> RandomizedElementTreeMutation {
        self.insert_child_internal().mutation
    }

    /// Removes a seeded random element and its subtree.
    ///
    /// This does not synchronize or notify persistent child entities. Entity-aware callers should
    /// use [`Self::apply_mutation`] with [`RandomizedElementTreeMutationKind::Remove`].
    pub fn remove_child(&mut self) -> Option<RandomizedElementTreeMutation> {
        self.remove_child_internal(false)
            .map(|applied| applied.mutation)
    }

    fn apply_mutation_without_notification(
        &mut self,
        kind: RandomizedElementTreeMutationKind,
    ) -> AppliedRandomizedElementTreeMutation {
        match kind {
            RandomizedElementTreeMutationKind::RootColor => {
                self.snapshot.borrow_mut().root.background = random_color(&mut self.rng);
                AppliedRandomizedElementTreeMutation::root(RandomizedElementTreeMutation::RootColor)
            }
            RandomizedElementTreeMutationKind::RootOpacity => {
                self.snapshot.borrow_mut().root.opacity = random_opacity(&mut self.rng);
                AppliedRandomizedElementTreeMutation::root(
                    RandomizedElementTreeMutation::RootOpacity,
                )
            }
            RandomizedElementTreeMutationKind::RootBorder => {
                self.snapshot
                    .borrow_mut()
                    .root
                    .randomize_border(&mut self.rng);
                AppliedRandomizedElementTreeMutation::root(
                    RandomizedElementTreeMutation::RootBorder,
                )
            }
            RandomizedElementTreeMutationKind::RootBounds => {
                let mut snapshot = self.snapshot.borrow_mut();
                snapshot.root.width = self.rng.random_range(520.0..980.0);
                snapshot.root.height = self.rng.random_range(360.0..760.0);
                AppliedRandomizedElementTreeMutation::root(
                    RandomizedElementTreeMutation::RootBounds,
                )
            }
            RandomizedElementTreeMutationKind::RootClipping => {
                let mut snapshot = self.snapshot.borrow_mut();
                snapshot.root.clipped = !snapshot.root.clipped;
                AppliedRandomizedElementTreeMutation::root(
                    RandomizedElementTreeMutation::RootClipping,
                )
            }
            RandomizedElementTreeMutationKind::RootVisibility => {
                let mut snapshot = self.snapshot.borrow_mut();
                snapshot.root.visible = !snapshot.root.visible;
                AppliedRandomizedElementTreeMutation::root(
                    RandomizedElementTreeMutation::RootVisibility,
                )
            }
            RandomizedElementTreeMutationKind::RootTextStyle => {
                self.snapshot
                    .borrow_mut()
                    .root
                    .rich
                    .text
                    .mutate(&mut self.rng);
                AppliedRandomizedElementTreeMutation::root(
                    RandomizedElementTreeMutation::RootTextStyle,
                )
            }
            RandomizedElementTreeMutationKind::RootScope => {
                let mut snapshot = self.snapshot.borrow_mut();
                snapshot.render_options.root_scope = (snapshot.render_options.root_scope + 1) % 3;
                AppliedRandomizedElementTreeMutation::root(RandomizedElementTreeMutation::RootScope)
            }
            RandomizedElementTreeMutationKind::ChildColor => {
                self.mutate_random_child(RandomizedChildMutation::Color)
            }
            RandomizedElementTreeMutationKind::ChildOpacity => {
                self.mutate_random_child(RandomizedChildMutation::Opacity)
            }
            RandomizedElementTreeMutationKind::ChildBorder => {
                self.mutate_random_child(RandomizedChildMutation::Border)
            }
            RandomizedElementTreeMutationKind::ChildBounds => {
                self.mutate_random_child(RandomizedChildMutation::Bounds)
            }
            RandomizedElementTreeMutationKind::ChildClipping => {
                self.mutate_random_child(RandomizedChildMutation::Clipping)
            }
            RandomizedElementTreeMutationKind::ChildVisibility => {
                self.mutate_random_child(RandomizedChildMutation::Visibility)
            }
            RandomizedElementTreeMutationKind::ChildText => {
                self.mutate_random_child(RandomizedChildMutation::Text)
            }
            RandomizedElementTreeMutationKind::ChildDisplay => {
                self.mutate_random_child(RandomizedChildMutation::Display)
            }
            RandomizedElementTreeMutationKind::ChildLayout => {
                self.mutate_random_child(RandomizedChildMutation::Layout)
            }
            RandomizedElementTreeMutationKind::ChildTextStyle => {
                self.mutate_random_child(RandomizedChildMutation::TextStyle)
            }
            RandomizedElementTreeMutationKind::ChildMeasuredContent => {
                self.mutate_random_child(RandomizedChildMutation::MeasuredContent)
            }
            RandomizedElementTreeMutationKind::ChildPopup => {
                self.mutate_random_child(RandomizedChildMutation::Popup)
            }
            RandomizedElementTreeMutationKind::Reorder => self.reorder_child(),
            RandomizedElementTreeMutationKind::Insert => self.insert_child_internal(),
            RandomizedElementTreeMutationKind::Remove => self
                .remove_child_internal(false)
                .unwrap_or_else(|| self.insert_child_internal()),
            RandomizedElementTreeMutationKind::RemoveLeaf => self
                .remove_child_internal(true)
                .unwrap_or_else(|| self.insert_child_internal()),
        }
    }

    fn insert_child_internal(&mut self) -> AppliedRandomizedElementTreeMutation {
        let mut snapshot = self.snapshot.borrow_mut();
        let element_id = snapshot.next_element_id;
        snapshot.next_element_id += 1;
        let mut parent_path = random_parent_path(&snapshot.children, &mut self.rng);
        let parent_id = node_at_path(&snapshot.children, &parent_path).map(|node| node.id);
        let owner_entity_id = nearest_entity_id(&snapshot.children, &parent_path);
        let node = RandomizedElementNode::random(element_id, &self.config, &mut self.rng);
        let (index, _) = insert_node(&mut snapshot.children, &mut parent_path, node);
        snapshot.rebuild_element_paths();

        AppliedRandomizedElementTreeMutation {
            mutation: RandomizedElementTreeMutation::Inserted {
                element_id,
                parent_id,
                index,
            },
            owner_entity_id,
            synchronize_entities: true,
        }
    }

    fn remove_child_internal(
        &mut self,
        leaves_only: bool,
    ) -> Option<AppliedRandomizedElementTreeMutation> {
        let mut snapshot = self.snapshot.borrow_mut();
        let path = if leaves_only {
            random_leaf_path(&snapshot.children, &mut self.rng)?
        } else {
            random_node_path(&snapshot.children, &mut self.rng)?
        };
        let (index, parent_path) = path.split_last()?;
        let parent_id = node_at_path(&snapshot.children, parent_path).map(|node| node.id);
        let owner_entity_id = nearest_entity_id(&snapshot.children, parent_path);
        let siblings = children_at_path_mut(&mut snapshot.children, parent_path)?;
        if *index >= siblings.len() {
            return None;
        }

        let removed = siblings.remove(*index);
        let removed_element_count = removed.element_count();
        let element_id = removed.id;
        snapshot.rebuild_element_paths();
        let mut scroll_handles = self.scroll_handles.borrow_mut();
        let mut removed_nodes = vec![&removed];
        while let Some(node) = removed_nodes.pop() {
            scroll_handles.remove(&node.id);
            removed_nodes.extend(node.children.iter());
        }
        drop(scroll_handles);
        Some(AppliedRandomizedElementTreeMutation {
            mutation: RandomizedElementTreeMutation::Removed {
                element_id,
                parent_id,
                index: *index,
                removed_element_count,
            },
            owner_entity_id,
            synchronize_entities: true,
        })
    }

    /// Recolors `count` distinct seeded random descendants in one pass and notifies each
    /// affected entity boundary once (the root, for descendants owned by no entity).
    /// Returns how many descendants changed and how many boundaries were notified.
    ///
    /// This is the "share of the scene changing per frame" knob: `count` calls to
    /// [`Self::apply_mutation`] would each walk the tree and notify separately, costing
    /// more than the frame being measured once `count` is a fair share of the tree.
    pub fn recolor_children(
        &mut self,
        count: usize,
        locality: ChangeLocality,
        cx: &mut Context<Self>,
    ) -> RecoloredChildren {
        let (owners, recolored) = {
            let mut snapshot = self.snapshot.borrow_mut();
            let mut paths = node_paths(&snapshot.children);
            let count = count.min(paths.len());
            if locality == ChangeLocality::Localized {
                // Order the paths so one random entity's subtree comes first: the change
                // stays inside that entity unless there is more of it than the subtree.
                let entity_paths: Vec<_> = paths
                    .iter()
                    .filter(|path| {
                        node_at_path(&snapshot.children, path).is_some_and(|node| node.is_entity)
                    })
                    .cloned()
                    .collect();
                if let Some(entity_path) = entity_paths
                    .get(self.rng.random_range(0..entity_paths.len().max(1)))
                    .cloned()
                {
                    let (inside, outside): (Vec<_>, Vec<_>) = paths
                        .into_iter()
                        .partition(|path| path.starts_with(&entity_path));
                    paths = inside;
                    paths.extend(outside);
                }
            }
            // A partial Fisher–Yates shuffle over the first `count` slots: for `Spread`, a
            // uniform sample of the tree; for `Localized`, a uniform sample of the chosen
            // subtree, spilling into the rest only once the subtree is exhausted.
            let sample_from = match locality {
                ChangeLocality::Spread => paths.len(),
                ChangeLocality::Localized => count,
            };
            for index in 0..count {
                let other = self.rng.random_range(index..sample_from.max(index + 1));
                paths.swap(index, other);
            }
            let mut owners = HashSet::default();
            for path in &paths[..count] {
                owners.insert(nearest_entity_id(&snapshot.children, path));
                if let Some(node) = node_at_path_mut(&mut snapshot.children, path) {
                    node.style.background = random_color(&mut self.rng);
                }
            }
            (owners, count)
        };
        let notified = owners.len();
        for owner in owners {
            self.notify_owner(owner, cx);
        }
        RecoloredChildren {
            recolored,
            notified,
        }
    }

    fn mutate_random_child(
        &mut self,
        mutation: RandomizedChildMutation,
    ) -> AppliedRandomizedElementTreeMutation {
        let mut snapshot = self.snapshot.borrow_mut();
        let Some(path) = random_node_path(&snapshot.children, &mut self.rng) else {
            drop(snapshot);
            return self.insert_child_internal();
        };
        // Notifying a view also re-renders its ancestors, so this reaches the view that
        // passes a cached entity's size to `.cached(style)` too.
        let owner_entity_id = nearest_entity_id(&snapshot.children, &path);
        let Some(node) = node_at_path_mut(&mut snapshot.children, &path) else {
            drop(snapshot);
            return self.insert_child_internal();
        };
        let element_id = node.id;

        let mutation = match mutation {
            RandomizedChildMutation::Color => {
                node.style.background = random_color(&mut self.rng);
                RandomizedElementTreeMutation::ChildColor { element_id }
            }
            RandomizedChildMutation::Opacity => {
                node.style.opacity = random_opacity(&mut self.rng);
                RandomizedElementTreeMutation::ChildOpacity { element_id }
            }
            RandomizedChildMutation::Border => {
                node.style.randomize_border(&mut self.rng);
                RandomizedElementTreeMutation::ChildBorder { element_id }
            }
            RandomizedChildMutation::Bounds => {
                // Occasionally collapse an axis to exercise empty bounds.
                node.style.width = if self.rng.random_bool(0.02) {
                    0.0
                } else {
                    self.rng.random_range(36.0..220.0)
                };
                node.style.height = if self.rng.random_bool(0.02) {
                    0.0
                } else {
                    self.rng.random_range(28.0..180.0)
                };
                RandomizedElementTreeMutation::ChildBounds { element_id }
            }
            RandomizedChildMutation::Clipping => {
                node.style.clipped = !node.style.clipped;
                RandomizedElementTreeMutation::ChildClipping { element_id }
            }
            RandomizedChildMutation::Visibility => {
                node.style.visible = !node.style.visible;
                RandomizedElementTreeMutation::ChildVisibility { element_id }
            }
            RandomizedChildMutation::Text => {
                node.style.label =
                    (node.style.label + self.rng.random_range(1..LABELS.len())) % LABELS.len();
                RandomizedElementTreeMutation::ChildText { element_id }
            }
            RandomizedChildMutation::Display => {
                node.style.rich.display_none = !node.style.rich.display_none;
                RandomizedElementTreeMutation::ChildDisplay { element_id }
            }
            RandomizedChildMutation::Layout => {
                node.style.mutate_layout(&mut self.rng);
                RandomizedElementTreeMutation::ChildLayout { element_id }
            }
            RandomizedChildMutation::TextStyle => {
                node.style.rich.text_override = true;
                node.style.rich.text.mutate(&mut self.rng);
                RandomizedElementTreeMutation::ChildTextStyle { element_id }
            }
            RandomizedChildMutation::MeasuredContent => {
                node.style.rich.mutate_measured(&mut self.rng);
                RandomizedElementTreeMutation::ChildMeasuredContent { element_id }
            }
            RandomizedChildMutation::Popup => {
                node.style.rich.mutate_popup(&mut self.rng);
                RandomizedElementTreeMutation::ChildPopup { element_id }
            }
        };
        AppliedRandomizedElementTreeMutation {
            mutation,
            owner_entity_id,
            synchronize_entities: false,
        }
    }

    fn reorder_child(&mut self) -> AppliedRandomizedElementTreeMutation {
        let mut snapshot = self.snapshot.borrow_mut();
        let parent_paths = parent_paths_with_multiple_children(&snapshot.children);
        if parent_paths.is_empty() {
            drop(snapshot);
            return self.insert_child_internal();
        }

        let parent_path_index = self.rng.random_range(0..parent_paths.len());
        let Some(parent_path) = parent_paths.get(parent_path_index) else {
            drop(snapshot);
            return self.insert_child_internal();
        };
        let parent_id = node_at_path(&snapshot.children, parent_path).map(|node| node.id);
        let owner_entity_id = nearest_entity_id(&snapshot.children, parent_path);
        let Some(child_count) =
            children_at_path(&snapshot.children, parent_path).map(<[RandomizedElementNode]>::len)
        else {
            drop(snapshot);
            return self.insert_child_internal();
        };
        if child_count < 2 {
            drop(snapshot);
            return self.insert_child_internal();
        }

        let old_index = self.rng.random_range(0..child_count);
        let mut new_index = self.rng.random_range(0..child_count - 1);
        if new_index >= old_index {
            new_index += 1;
        }

        let Some(siblings) = children_at_path_mut(&mut snapshot.children, parent_path) else {
            drop(snapshot);
            return self.insert_child_internal();
        };
        let child = siblings.remove(old_index);
        let element_id = child.id;
        siblings.insert(new_index, child);
        snapshot.rebuild_element_paths();
        AppliedRandomizedElementTreeMutation {
            mutation: RandomizedElementTreeMutation::Reordered {
                element_id,
                parent_id,
                old_index,
                new_index,
            },
            owner_entity_id,
            synchronize_entities: false,
        }
    }

    fn synchronize_entities(&mut self, cx: &mut Context<Self>) {
        let entity_ids = {
            let snapshot = self.snapshot.borrow();
            snapshot
                .element_ids()
                .into_iter()
                .filter(|element_id| {
                    snapshot
                        .node(*element_id)
                        .is_some_and(|node| node.is_entity)
                })
                .collect::<Vec<_>>()
        };
        let entity_id_set = entity_ids.iter().copied().collect::<HashSet<_>>();

        self.entities
            .borrow_mut()
            .retain(|element_id, _| entity_id_set.contains(element_id));

        let missing_entity_ids = {
            let entities = self.entities.borrow();
            entity_ids
                .into_iter()
                .filter(|element_id| !entities.contains_key(element_id))
                .collect::<Vec<_>>()
        };

        let weak_entities = Rc::downgrade(&self.entities);
        for element_id in missing_entity_ids {
            let entity = cx.new(|_| RandomizedElementEntity {
                element_id,
                snapshot: self.snapshot.clone(),
                entities: weak_entities.clone(),
                scroll_handles: self.scroll_handles.clone(),
                list_states: self.list_states.clone(),
                uniform_list_scroll_handles: self.uniform_list_scroll_handles.clone(),
                work_counters: self.work_counters.clone(),
            });
            self.entities.borrow_mut().insert(element_id, entity);
        }
    }

    fn notify_owner(&self, owner_entity_id: Option<u64>, cx: &mut Context<Self>) {
        let owner =
            owner_entity_id.and_then(|element_id| self.entities.borrow().get(&element_id).cloned());
        if let Some(owner) = owner {
            owner.update(cx, |_, cx| cx.notify());
        } else {
            cx.notify();
        }
    }
}

impl Render for RandomizedElementTree {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let snapshot = self.snapshot.borrow();
        let root = &snapshot.root;
        let entities = self.entities.borrow();
        let mut work_counters = self.work_counters.borrow_mut();
        work_counters.record_root_render();

        let children = snapshot.children.iter().map(|node| {
            render_node(
                node,
                &entities,
                &self.scroll_handles,
                &self.list_states,
                &self.uniform_list_scroll_handles,
                &mut work_counters,
                snapshot.render_options,
            )
        });
        let content: AnyElement = match snapshot.render_options.root_scope {
            1 => div()
                .id("randomized-root-scope-a")
                .flex()
                .when(root.vertical, |element| element.flex_col())
                .when(root.wrap, |element| element.flex_wrap())
                .gap(px(root.gap))
                .children(children)
                .into_any_element(),
            2 => div()
                .id("randomized-root-scope-b")
                .flex()
                .when(root.vertical, |element| element.flex_col())
                .when(root.wrap, |element| element.flex_wrap())
                .gap(px(root.gap))
                .children(children)
                .into_any_element(),
            _ => div()
                .flex()
                .when(root.vertical, |element| element.flex_col())
                .when(root.wrap, |element| element.flex_wrap())
                .gap(px(root.gap))
                .children(children)
                .into_any_element(),
        };

        let mut root_div = div()
            .flex()
            .when(snapshot.render_options.hover_styles, |element| {
                element.group("randomized-tree-root")
            })
            .p(px(root.padding))
            .w(px(root.width))
            .h(px(root.height))
            // Shrinks with a window smaller than the root, so resizing it changes layout.
            .max_w_full()
            .max_h_full()
            .bg(root.background)
            .border(px(root.border_width))
            .border_color(root.border_color)
            .rounded(px(root.corner_radius))
            .opacity(root.opacity)
            .when(root.clipped, |element| element.overflow_hidden())
            .when(!root.visible, |element| element.invisible());
        apply_text_style(&mut root_div, &root.rich.text);
        root_div.child(content)
    }
}

type RandomizedElementEntities = HashMap<u64, Entity<RandomizedElementEntity>>;

/// Element IDs carry the tree's ID generation above this bit, so churning IDs never
/// changes the element number the low bits identify.
const ID_GENERATION_SHIFT: u32 = 32;

#[derive(Clone, Copy, Debug, PartialEq)]
struct RenderOptions {
    cached_views: bool,
    hover_styles: bool,
    paths: bool,
    id_generation: u64,
    root_scope: u8,
}

struct RandomizedElementEntity {
    element_id: u64,
    snapshot: Rc<RefCell<RandomizedElementTreeSnapshot>>,
    entities: Weak<RefCell<RandomizedElementEntities>>,
    scroll_handles: Rc<RefCell<HashMap<u64, crate::ScrollHandle>>>,
    list_states: Rc<RefCell<HashMap<u64, (ListState, u64)>>>,
    uniform_list_scroll_handles: Rc<RefCell<HashMap<u64, (UniformListScrollHandle, u64)>>>,
    work_counters: Rc<RefCell<RandomizedElementTreeWorkCounters>>,
}

impl Render for RandomizedElementEntity {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let snapshot = self.snapshot.borrow();
        let entities = self.entities.upgrade();
        let entities = entities.as_ref().map(|entities| entities.borrow());
        let mut work_counters = self.work_counters.borrow_mut();
        work_counters.record_entity_render(self.element_id);

        snapshot
            .node(self.element_id)
            .map(|node| {
                render_node_body(
                    node,
                    entities.as_deref(),
                    &self.scroll_handles,
                    &self.list_states,
                    &self.uniform_list_scroll_handles,
                    &mut work_counters,
                    snapshot.render_options,
                )
            })
            .unwrap_or_else(|| div().into_any_element())
    }
}

struct AppliedRandomizedElementTreeMutation {
    mutation: RandomizedElementTreeMutation,
    owner_entity_id: Option<u64>,
    synchronize_entities: bool,
}

impl AppliedRandomizedElementTreeMutation {
    fn root(mutation: RandomizedElementTreeMutation) -> Self {
        Self {
            mutation,
            owner_entity_id: None,
            synchronize_entities: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct RandomizedElementNode {
    id: u64,
    style: RandomizedElementStyle,
    is_entity: bool,
    features: RandomizedElementFeatures,
    children: Vec<RandomizedElementNode>,
}

impl RandomizedElementNode {
    fn random(id: u64, config: &RandomizedElementTreeConfig, rng: &mut StdRng) -> Self {
        let mut style = RandomizedElementStyle::random_child(rng);
        let is_entity = rng.random_bool(config.entity_density);
        let mut features = RandomizedElementFeatures::random(config.handler_density, rng);
        // Drawn only when enabled, so trees without animations keep the same RNG stream.
        features.animated = is_entity
            && config.animation_density > 0.0
            && rng.random_bool(config.animation_density);
        if rng.random_bool(0.35) {
            style.rich = RandomizedRichLayout::random(rng);
        }
        Self {
            id,
            style,
            is_entity,
            features,
            children: Vec::new(),
        }
    }

    fn element_count(&self) -> usize {
        let mut count = 0;
        let mut stack = vec![self];
        while let Some(node) = stack.pop() {
            count += 1;
            stack.extend(node.children.iter());
        }
        count
    }

    fn is_keyed(&self) -> bool {
        !self.style.rich.unkeyed || self.id.is_multiple_of(3) || self.is_entity
    }
}

#[derive(Clone, Debug, PartialEq)]
struct RandomizedElementFeatures {
    action_handler: bool,
    click_handler: bool,
    key_context: bool,
    animated: bool,
}

impl RandomizedElementFeatures {
    fn random(handler_density: f64, rng: &mut StdRng) -> Self {
        Self {
            action_handler: rng.random_bool(handler_density),
            click_handler: rng.random_bool(handler_density),
            key_context: rng.random_bool(handler_density),
            animated: false,
        }
    }

    fn handler_count(&self) -> usize {
        usize::from(self.action_handler) + usize::from(self.click_handler)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct RandomizedElementStyle {
    background: Hsla,
    border_color: Hsla,
    opacity: f32,
    border_width: f32,
    corner_radius: f32,
    width: f32,
    height: f32,
    padding: f32,
    gap: f32,
    clipped: bool,
    visible: bool,
    vertical: bool,
    wrap: bool,
    /// Index into [`LABELS`] of the element's text. Not drawn when generating a tree, so
    /// adding it left every seed's tree as it was.
    label: usize,
    /// Additional layout and measured-content state used when `rich_layout` is enabled.
    rich: RandomizedRichLayout,
}

#[derive(Clone, Debug, PartialEq)]
struct RandomizedRichLayout {
    display_none: bool,
    grid: Option<(u16, u16)>,
    grid_span: Option<(u16, u16)>,
    absolute_inset: Option<(f32, f32)>,
    auto_width: bool,
    auto_height: bool,
    min_width: Option<f32>,
    max_width: Option<f32>,
    min_height: Option<f32>,
    max_height: Option<f32>,
    rem_sized: bool,
    rem_size_override: Option<f32>,
    margin: f32,
    flex_basis: Option<f32>,
    flex_grow: f32,
    flex_shrink: f32,
    scrollbar_width: Option<f32>,
    align_items: Option<AlignItems>,
    align_self: Option<AlignSelf>,
    align_content: Option<AlignContent>,
    justify_content: Option<JustifyContent>,
    aspect_ratio: Option<f32>,
    unkeyed: bool,
    text_override: bool,
    text: RandomizedTextStyle,
    measured: RandomizedMeasuredContent,
    popup: Option<RandomizedPopup>,
    list_version: u64,
}

impl Default for RandomizedRichLayout {
    fn default() -> Self {
        Self {
            display_none: false,
            grid: None,
            grid_span: None,
            absolute_inset: None,
            auto_width: false,
            auto_height: false,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            rem_sized: false,
            rem_size_override: None,
            margin: 0.0,
            flex_basis: None,
            flex_grow: 0.0,
            flex_shrink: 0.0,
            scrollbar_width: None,
            align_items: None,
            align_self: None,
            align_content: None,
            justify_content: None,
            aspect_ratio: None,
            unkeyed: false,
            text_override: false,
            text: RandomizedTextStyle::default(),
            measured: RandomizedMeasuredContent::None,
            popup: None,
            list_version: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct RandomizedPopup {
    priority: usize,
    offset_x: f32,
    offset_y: f32,
    width: f32,
    height: f32,
    label: usize,
    color: Hsla,
}

impl RandomizedPopup {
    fn random(rng: &mut StdRng) -> Self {
        Self {
            priority: rng.random_range(0..3),
            offset_x: rng.random_range(0.0..400.0),
            offset_y: rng.random_range(0.0..300.0),
            width: rng.random_range(40.0..180.0),
            height: rng.random_range(20.0..90.0),
            label: rng.random_range(0..LABELS.len()),
            color: random_color(rng),
        }
    }

    fn mutate(&mut self, rng: &mut StdRng) {
        match rng.random_range(0..6) {
            0 => self.priority = rng.random_range(0..3),
            1 => {
                self.offset_x = rng.random_range(0.0..400.0);
                self.offset_y = rng.random_range(0.0..300.0);
            }
            2 => self.width = rng.random_range(40.0..180.0),
            3 => self.height = rng.random_range(20.0..90.0),
            4 => self.label = (self.label + rng.random_range(1..LABELS.len())) % LABELS.len(),
            _ => self.color = random_color(rng),
        }
    }
}

impl RandomizedRichLayout {
    fn random(rng: &mut StdRng) -> Self {
        let mut rich = Self {
            display_none: rng.random_bool(0.06),
            ..Self::default()
        };
        rich.randomize_layout(rng);
        if rng.random_bool(0.5) {
            rich.text_override = true;
            rich.text.randomize(rng);
        }
        if rng.random_bool(0.6) {
            rich.randomize_measured(rng);
        }
        if rng.random_bool(0.2) {
            rich.popup = Some(RandomizedPopup::random(rng));
        }
        rich
    }

    fn randomize_layout(&mut self, rng: &mut StdRng) {
        self.grid = rng
            .random_bool(0.18)
            .then(|| (rng.random_range(1..=3), rng.random_range(1..=3)));
        self.grid_span = rng
            .random_bool(0.15)
            .then(|| (rng.random_range(1..=2), rng.random_range(1..=2)));
        self.absolute_inset = rng
            .random_bool(0.12)
            .then(|| (rng.random_range(0.0..24.0), rng.random_range(0.0..24.0)));
        self.auto_width = rng.random_bool(0.45);
        self.auto_height = rng.random_bool(0.45);
        self.min_width = rng.random_bool(0.2).then(|| rng.random_range(0.0..60.0));
        self.max_width = rng.random_bool(0.2).then(|| rng.random_range(60.0..240.0));
        self.min_height = rng.random_bool(0.2).then(|| rng.random_range(0.0..50.0));
        self.max_height = rng.random_bool(0.2).then(|| rng.random_range(50.0..200.0));
        self.rem_sized = rng.random_bool(0.35);
        self.rem_size_override = rng.random_bool(0.15).then(|| rng.random_range(10.0..22.0));
        self.margin = if rng.random_bool(0.4) {
            rng.random_range(0.0..8.0)
        } else {
            0.0
        };
        self.flex_basis = rng.random_bool(0.2).then(|| rng.random_range(16.0..120.0));
        self.flex_grow = if rng.random_bool(0.25) { 1.0 } else { 0.0 };
        self.flex_shrink = if rng.random_bool(0.5) { 1.0 } else { 0.0 };
        self.scrollbar_width = rng.random_bool(0.15).then(|| rng.random_range(0.0..12.0));
        self.align_items = random_align_items(rng);
        self.align_self = random_align_self(rng);
        self.align_content = random_align_content(rng);
        self.justify_content = random_justify_content(rng);
        self.aspect_ratio = rng.random_bool(0.15).then(|| rng.random_range(0.5..2.5));
        self.unkeyed = rng.random_bool(0.3);
    }

    fn mutate_one_layout_field(&mut self, rng: &mut StdRng) {
        match rng.random_range(0..22) {
            0 => self.auto_width = !self.auto_width,
            1 => self.auto_height = !self.auto_height,
            2 => self.rem_sized = !self.rem_sized,
            3 => {
                self.margin = if self.margin == 0.0 || rng.random_bool(0.7) {
                    rng.random_range(0.0..8.0)
                } else {
                    0.0
                };
            }
            4 => self.flex_grow = if self.flex_grow == 0.0 { 1.0 } else { 0.0 },
            5 => self.flex_shrink = if self.flex_shrink == 0.0 { 1.0 } else { 0.0 },
            6 => self.align_items = random_align_items(rng),
            7 => self.justify_content = random_justify_content(rng),
            8 => {
                self.aspect_ratio = if self.aspect_ratio.is_some() && rng.random_bool(0.5) {
                    None
                } else {
                    Some(rng.random_range(0.5..2.5))
                };
            }
            9 => self.unkeyed = !self.unkeyed,
            10 => {
                self.grid = if self.grid.is_some() && rng.random_bool(0.5) {
                    None
                } else {
                    Some((rng.random_range(1..=3), rng.random_range(1..=3)))
                };
            }
            11 => {
                self.grid_span = if self.grid_span.is_some() && rng.random_bool(0.5) {
                    None
                } else {
                    Some((rng.random_range(1..=2), rng.random_range(1..=2)))
                };
            }
            12 => {
                self.absolute_inset = if self.absolute_inset.is_some() && rng.random_bool(0.5) {
                    None
                } else {
                    Some((rng.random_range(0.0..24.0), rng.random_range(0.0..24.0)))
                };
            }
            13 => {
                self.min_width = if self.min_width.is_some() && rng.random_bool(0.5) {
                    None
                } else {
                    Some(rng.random_range(0.0..60.0))
                };
            }
            14 => {
                self.max_width = if self.max_width.is_some() && rng.random_bool(0.5) {
                    None
                } else {
                    Some(rng.random_range(60.0..240.0))
                };
            }
            15 => {
                self.min_height = if self.min_height.is_some() && rng.random_bool(0.5) {
                    None
                } else {
                    Some(rng.random_range(0.0..50.0))
                };
            }
            16 => {
                self.max_height = if self.max_height.is_some() && rng.random_bool(0.5) {
                    None
                } else {
                    Some(rng.random_range(50.0..200.0))
                };
            }
            17 => {
                self.rem_size_override = if self.rem_size_override.is_some() && rng.random_bool(0.5)
                {
                    None
                } else {
                    Some(rng.random_range(10.0..22.0))
                };
            }
            18 => {
                self.flex_basis = if self.flex_basis.is_some() && rng.random_bool(0.5) {
                    None
                } else {
                    Some(rng.random_range(16.0..120.0))
                };
            }
            19 => {
                self.scrollbar_width = if self.scrollbar_width.is_some() && rng.random_bool(0.5) {
                    None
                } else {
                    Some(rng.random_range(0.0..12.0))
                };
            }
            20 => self.align_self = random_align_self(rng),
            _ => self.align_content = random_align_content(rng),
        }
    }

    fn randomize_measured(&mut self, rng: &mut StdRng) {
        self.list_version += 1;
        let color = random_color(rng);
        self.measured = match rng.random_range(0..6) {
            0 => RandomizedMeasuredContent::KeyedLeaf {
                width: rng.random_range(12.0..96.0),
                height: rng.random_range(10.0..48.0),
                color,
            },
            1 => RandomizedMeasuredContent::UnkeyedLeaf {
                width: rng.random_range(12.0..96.0),
                height: rng.random_range(10.0..48.0),
                color,
            },
            2 => RandomizedMeasuredContent::UniformList {
                item_count: rng.random_range(0..=6),
                item_width: rng.random_range(24.0..80.0),
                item_height: rng.random_range(10.0..24.0),
                scroll_offset: if rng.random_bool(0.5) {
                    rng.random_range(0.0..48.0)
                } else {
                    0.0
                },
                color,
            },
            3 => {
                let item_count = rng.random_range(0..=6);
                let scroll_index = if item_count > 0 && rng.random_bool(0.5) {
                    rng.random_range(0..item_count)
                } else {
                    0
                };
                RandomizedMeasuredContent::List {
                    item_count,
                    item_width: rng.random_range(24.0..80.0),
                    item_height: rng.random_range(10.0..24.0),
                    scroll_index,
                    color,
                }
            }
            _ => RandomizedMeasuredContent::None,
        };
    }

    fn mutate_measured(&mut self, rng: &mut StdRng) {
        // Usually tweak a single field so minimal-change invalidation edge cases get tested.
        if rng.random_bool(0.25) || matches!(self.measured, RandomizedMeasuredContent::None) {
            self.randomize_measured(rng);
            return;
        }
        match &mut self.measured {
            RandomizedMeasuredContent::None => {}
            RandomizedMeasuredContent::KeyedLeaf {
                width,
                height,
                color,
            }
            | RandomizedMeasuredContent::UnkeyedLeaf {
                width,
                height,
                color,
            } => match rng.random_range(0..3) {
                0 => *width = rng.random_range(12.0..96.0),
                1 => *height = rng.random_range(10.0..48.0),
                _ => *color = random_color(rng),
            },
            RandomizedMeasuredContent::UniformList {
                item_count,
                item_width,
                item_height,
                scroll_offset,
                color,
            } => match rng.random_range(0..5) {
                0 => *item_count = rng.random_range(0..=6),
                1 => *item_width = rng.random_range(24.0..80.0),
                2 => *item_height = rng.random_range(10.0..24.0),
                3 => {
                    *scroll_offset = rng.random_range(0.0..48.0);
                    self.list_version += 1;
                }
                _ => *color = random_color(rng),
            },
            RandomizedMeasuredContent::List {
                item_count,
                item_width,
                item_height,
                scroll_index,
                color,
            } => match rng.random_range(0..5) {
                0 => {
                    *item_count = rng.random_range(0..=6);
                    *scroll_index = (*scroll_index).min(item_count.saturating_sub(1));
                    self.list_version += 1;
                }
                1 => {
                    *item_width = rng.random_range(24.0..80.0);
                    self.list_version += 1;
                }
                2 => {
                    *item_height = rng.random_range(10.0..24.0);
                    self.list_version += 1;
                }
                3 => {
                    *scroll_index = if *item_count > 0 {
                        rng.random_range(0..*item_count)
                    } else {
                        0
                    };
                    self.list_version += 1;
                }
                _ => *color = random_color(rng),
            },
        }
    }

    fn mutate_popup(&mut self, rng: &mut StdRng) {
        match &mut self.popup {
            Some(popup) if rng.random_bool(0.75) => popup.mutate(rng),
            Some(_) => self.popup = None,
            None => self.popup = Some(RandomizedPopup::random(rng)),
        }
    }
}

fn random_align_items(rng: &mut StdRng) -> Option<AlignItems> {
    match rng.random_range(0..5) {
        0 => Some(AlignItems::FlexStart),
        1 => Some(AlignItems::Center),
        2 => Some(AlignItems::FlexEnd),
        3 => Some(AlignItems::Stretch),
        _ => None,
    }
}

fn random_justify_content(rng: &mut StdRng) -> Option<JustifyContent> {
    match rng.random_range(0..5) {
        0 => Some(JustifyContent::FlexStart),
        1 => Some(JustifyContent::Center),
        2 => Some(JustifyContent::SpaceBetween),
        3 => Some(JustifyContent::FlexEnd),
        _ => None,
    }
}

fn random_align_self(rng: &mut StdRng) -> Option<AlignSelf> {
    match rng.random_range(0..5) {
        0 => Some(AlignSelf::FlexStart),
        1 => Some(AlignSelf::Center),
        2 => Some(AlignSelf::FlexEnd),
        3 => Some(AlignSelf::Stretch),
        _ => None,
    }
}

fn random_align_content(rng: &mut StdRng) -> Option<AlignContent> {
    match rng.random_range(0..5) {
        0 => Some(AlignContent::FlexStart),
        1 => Some(AlignContent::Center),
        2 => Some(AlignContent::SpaceBetween),
        3 => Some(AlignContent::FlexEnd),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq)]
struct RandomizedTextStyle {
    color: Hsla,
    highlight_color: Hsla,
    highlight_bg: Hsla,
    size_in_rems: bool,
    font_size: f32,
    line_height_relative: bool,
    line_height: f32,
    white_space: WhiteSpace,
    text_overflow: Option<TextOverflow>,
    line_clamp: Option<usize>,
    text_align: TextAlign,
    kind: RandomizedTextKind,
}

impl Default for RandomizedTextStyle {
    fn default() -> Self {
        Self {
            color: crate::black(),
            highlight_color: crate::black(),
            highlight_bg: crate::transparent_black(),
            size_in_rems: false,
            font_size: 14.0,
            line_height_relative: true,
            line_height: 1.3,
            white_space: WhiteSpace::Normal,
            text_overflow: None,
            line_clamp: None,
            text_align: TextAlign::Left,
            kind: RandomizedTextKind::Plain,
        }
    }
}

impl RandomizedTextStyle {
    fn randomize(&mut self, rng: &mut StdRng) {
        self.color = random_color(rng);
        self.highlight_color = random_color(rng);
        self.highlight_bg = random_color(rng);
        self.size_in_rems = rng.random_bool(0.5);
        self.font_size = if self.size_in_rems {
            rng.random_range(0.75..1.5)
        } else {
            rng.random_range(10.0..22.0)
        };
        self.line_height_relative = rng.random_bool(0.5);
        self.line_height = if self.line_height_relative {
            rng.random_range(1.1..1.8)
        } else {
            rng.random_range(0.9..2.0)
        };
        self.white_space = if rng.random_bool(0.35) {
            WhiteSpace::Nowrap
        } else {
            WhiteSpace::Normal
        };
        self.text_overflow = random_text_overflow(rng);
        self.line_clamp = random_line_clamp(rng);
        self.text_align = random_text_align(rng);
        self.kind = random_text_kind(rng);
    }

    fn mutate(&mut self, rng: &mut StdRng) {
        if rng.random_bool(0.2) {
            self.randomize(rng);
            return;
        }
        match rng.random_range(0..9) {
            0 => self.color = random_color(rng),
            1 => {
                self.highlight_color = random_color(rng);
                self.highlight_bg = random_color(rng);
            }
            2 => {
                self.size_in_rems = rng.random_bool(0.5);
                self.font_size = if self.size_in_rems {
                    rng.random_range(0.75..1.5)
                } else {
                    rng.random_range(10.0..22.0)
                };
            }
            3 => {
                self.line_height_relative = rng.random_bool(0.5);
                self.line_height = if self.line_height_relative {
                    rng.random_range(1.1..1.8)
                } else {
                    rng.random_range(0.9..2.0)
                };
            }
            4 => {
                self.white_space = match self.white_space {
                    WhiteSpace::Normal => WhiteSpace::Nowrap,
                    WhiteSpace::Nowrap => WhiteSpace::Normal,
                };
            }
            5 => self.text_overflow = random_text_overflow(rng),
            6 => self.line_clamp = random_line_clamp(rng),
            7 => self.text_align = random_text_align(rng),
            _ => self.kind = random_text_kind(rng),
        }
    }
}

fn random_text_overflow(rng: &mut StdRng) -> Option<TextOverflow> {
    match rng.random_range(0..5) {
        0 => Some(TextOverflow::Truncate("...".into())),
        1 => Some(TextOverflow::TruncateStart("...".into())),
        2 => Some(TextOverflow::TruncateMiddle("...".into())),
        _ => None,
    }
}

fn random_line_clamp(rng: &mut StdRng) -> Option<usize> {
    match rng.random_range(0..4) {
        0 => Some(1),
        1 => Some(2),
        _ => None,
    }
}

fn random_text_align(rng: &mut StdRng) -> TextAlign {
    match rng.random_range(0..3) {
        0 => TextAlign::Left,
        1 => TextAlign::Center,
        _ => TextAlign::Right,
    }
}

fn random_text_kind(rng: &mut StdRng) -> RandomizedTextKind {
    match rng.random_range(0..3) {
        0 => RandomizedTextKind::Plain,
        1 => RandomizedTextKind::StyledRuns,
        _ => RandomizedTextKind::Interactive,
    }
}

fn apply_text_style(element: &mut crate::Div, text: &RandomizedTextStyle) {
    let text_refinement = element.text_style();
    text_refinement.color = Some(text.color);
    text_refinement.font_size = Some(if text.size_in_rems {
        rems(text.font_size).into()
    } else {
        px(text.font_size).into()
    });
    text_refinement.line_height = Some(if text.line_height_relative {
        relative(text.line_height)
    } else {
        rems(text.line_height).into()
    });
    text_refinement.white_space = Some(text.white_space);
    text_refinement.text_overflow = text.text_overflow.clone();
    text_refinement.line_clamp = text.line_clamp;
    text_refinement.text_align = Some(text.text_align);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RandomizedTextKind {
    Plain,
    StyledRuns,
    Interactive,
}

#[derive(Clone, Debug, PartialEq)]
enum RandomizedMeasuredContent {
    None,
    KeyedLeaf {
        width: f32,
        height: f32,
        color: Hsla,
    },
    UnkeyedLeaf {
        width: f32,
        height: f32,
        color: Hsla,
    },
    UniformList {
        item_count: usize,
        item_width: f32,
        item_height: f32,
        scroll_offset: f32,
        color: Hsla,
    },
    List {
        item_count: usize,
        item_width: f32,
        item_height: f32,
        scroll_index: usize,
        color: Hsla,
    },
}

struct RemSizeScope {
    rem_size: Pixels,
    child: AnyElement,
}

impl Element for RemSizeScope {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let layout_id = window.with_rem_size(Some(self.rem_size), |window| {
            self.child.request_layout(window, cx)
        });
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        window.with_rem_size(Some(self.rem_size), |window| {
            self.child.prepaint(window, cx);
        });
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        window.with_rem_size(Some(self.rem_size), |window| {
            self.child.paint(window, cx);
        });
    }
}

struct RandomizedMeasuredLeaf {
    width: f32,
    height: f32,
    color: Hsla,
    keyed: bool,
}

impl Element for RandomizedMeasuredLeaf {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        _: &mut App,
    ) -> (LayoutId, ()) {
        let intrinsic = size(px(self.width), px(self.height));
        let measure = move |known_dimensions: Size<Option<Pixels>>,
                            available_space: Size<AvailableSpace>,
                            _: &mut Window,
                            _: &mut App| {
            let width = known_dimensions
                .width
                .unwrap_or(match available_space.width {
                    AvailableSpace::Definite(available) => intrinsic.width.min(available),
                    AvailableSpace::MinContent => intrinsic.width * 0.5,
                    AvailableSpace::MaxContent => intrinsic.width,
                });
            let height = known_dimensions
                .height
                .unwrap_or(match available_space.height {
                    AvailableSpace::Definite(available) => intrinsic.height.min(available),
                    AvailableSpace::MinContent | AvailableSpace::MaxContent => intrinsic.height,
                });
            size(width, height)
        };
        let layout_id = if self.keyed {
            window.request_keyed_measured_layout(Style::default(), intrinsic, measure)
        } else {
            window.request_measured_layout(Style::default(), measure)
        };
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut Window,
        _: &mut App,
    ) {
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        _: &mut App,
    ) {
        window.paint_quad(fill(bounds, self.color));
    }
}

/// The texts an element can show. Elements whose number is 1 more than a multiple of 4
/// show the first; others show text only once a mutation changes their label.
const LABELS: [&str; 5] = [
    "Retained glyphs: abc Ω",
    "Retained glyphs: abc Ω, now with enough words that narrow parents wrap them",
    "Line one: abc Ω\nLine two wraps when narrow enough",
    "Ω",
    "",
];

impl RandomizedElementStyle {
    fn random_root(rng: &mut StdRng) -> Self {
        Self {
            background: random_color(rng),
            border_color: random_color(rng),
            opacity: random_opacity(rng),
            border_width: rng.random_range(0.0..4.0),
            corner_radius: rng.random_range(0.0..18.0),
            width: rng.random_range(520.0..980.0),
            height: rng.random_range(360.0..760.0),
            padding: rng.random_range(4.0..18.0),
            gap: rng.random_range(2.0..14.0),
            clipped: rng.random_bool(0.5),
            visible: true,
            vertical: rng.random_bool(0.5),
            wrap: rng.random_bool(0.7),
            label: 0,
            rich: RandomizedRichLayout::default(),
        }
    }

    fn random_child(rng: &mut StdRng) -> Self {
        Self {
            background: random_color(rng),
            border_color: random_color(rng),
            opacity: random_opacity(rng),
            border_width: rng.random_range(0.0..4.0),
            corner_radius: rng.random_range(0.0..14.0),
            width: rng.random_range(36.0..220.0),
            height: rng.random_range(28.0..180.0),
            padding: rng.random_range(1.0..10.0),
            gap: rng.random_range(1.0..8.0),
            clipped: rng.random_bool(0.3),
            visible: rng.random_bool(0.9),
            vertical: rng.random_bool(0.5),
            wrap: rng.random_bool(0.4),
            label: 0,
            rich: RandomizedRichLayout::default(),
        }
    }

    fn randomize_border(&mut self, rng: &mut StdRng) {
        self.border_color = random_color(rng);
        self.border_width = rng.random_range(0.0..5.0);
        self.corner_radius = rng.random_range(0.0..18.0);
    }

    fn mutate_layout(&mut self, rng: &mut StdRng) {
        if rng.random_bool(0.2) {
            self.vertical = rng.random_bool(0.5);
            self.wrap = rng.random_bool(0.4);
            self.padding = rng.random_range(0.0..12.0);
            self.gap = rng.random_range(0.0..10.0);
            self.rich.randomize_layout(rng);
            return;
        }
        match rng.random_range(0..14) {
            0 => self.vertical = !self.vertical,
            1 => self.wrap = !self.wrap,
            2 => self.padding = rng.random_range(0.0..12.0),
            3 => self.gap = rng.random_range(0.0..10.0),
            _ => self.rich.mutate_one_layout_field(rng),
        }
    }
}

#[derive(Clone, Copy)]
enum RandomizedChildMutation {
    Color,
    Opacity,
    Border,
    Bounds,
    Clipping,
    Visibility,
    Text,
    Display,
    Layout,
    TextStyle,
    MeasuredContent,
    Popup,
}

#[cfg_attr(feature = "stacker", stacksafe::stacksafe)]
fn render_node(
    node: &RandomizedElementNode,
    entities: &RandomizedElementEntities,
    scroll_handles: &RefCell<HashMap<u64, crate::ScrollHandle>>,
    list_states: &RefCell<HashMap<u64, (ListState, u64)>>,
    uniform_list_scroll_handles: &RefCell<HashMap<u64, (UniformListScrollHandle, u64)>>,
    work_counters: &mut RandomizedElementTreeWorkCounters,
    options: RenderOptions,
) -> AnyElement {
    if node.is_entity
        && let Some(entity) = entities.get(&node.id)
    {
        if options.cached_views {
            let mut style = crate::StyleRefinement::default();
            style.size.width = Some(px(node.style.width).into());
            style.size.height = Some(px(node.style.height).into());
            style.flex_shrink = Some(0.0);
            return entity.clone().cached(style).into_any_element();
        }
        return entity.clone().into_any_element();
    }

    render_node_body(
        node,
        Some(entities),
        scroll_handles,
        list_states,
        uniform_list_scroll_handles,
        work_counters,
        options,
    )
}

fn render_rich_text(
    node_id: u64,
    label: &'static str,
    text_style: &RandomizedTextStyle,
    id_generation: u64,
) -> AnyElement {
    let styled_with_highlights = || {
        let mut highlights = Vec::new();
        if let Some(first_end) = label.chars().next().map(char::len_utf8) {
            highlights.push((
                0..first_end,
                HighlightStyle {
                    color: Some(text_style.highlight_color),
                    background_color: Some(text_style.highlight_bg),
                    underline: Some(UnderlineStyle {
                        thickness: px(1.0),
                        color: Some(text_style.highlight_color),
                        wavy: false,
                    }),
                    strikethrough: Some(StrikethroughStyle {
                        thickness: px(1.0),
                        color: Some(text_style.color),
                    }),
                    ..Default::default()
                },
            ));
        }
        StyledText::new(label).with_highlights(highlights)
    };

    match text_style.kind {
        RandomizedTextKind::Plain => label.into_any_element(),
        RandomizedTextKind::StyledRuns => styled_with_highlights().into_any_element(),
        RandomizedTextKind::Interactive => InteractiveText::new(
            ElementId::NamedInteger(
                "randomized-interactive-text".into(),
                node_id | id_generation << ID_GENERATION_SHIFT,
            ),
            styled_with_highlights(),
        )
        .on_hover(|_, _, _, _| {})
        .into_any_element(),
    }
}

#[cfg_attr(feature = "stacker", stacksafe::stacksafe)]
fn render_node_body(
    node: &RandomizedElementNode,
    entities: Option<&RandomizedElementEntities>,
    scroll_handles: &RefCell<HashMap<u64, crate::ScrollHandle>>,
    list_states: &RefCell<HashMap<u64, (ListState, u64)>>,
    uniform_list_scroll_handles: &RefCell<HashMap<u64, (UniformListScrollHandle, u64)>>,
    work_counters: &mut RandomizedElementTreeWorkCounters,
    options: RenderOptions,
) -> AnyElement {
    work_counters.record_element_render(node.features.handler_count());
    let style = &node.style;
    let rich = &style.rich;
    let is_scroll = node.id.is_multiple_of(3);
    let keyed = node.is_keyed();

    let mut base = div();
    base = if rich.display_none {
        base.hidden()
    } else if let Some((cols, rows)) = rich.grid {
        base.grid().grid_cols(cols).grid_rows(rows)
    } else {
        base.flex()
    };
    if let Some((col_span, row_span)) = rich.grid_span {
        base = base.col_span(col_span).row_span(row_span);
    }
    if let Some((top, left)) = rich.absolute_inset {
        base = base.absolute();
        if rich.rem_sized {
            base = base.top(rems(top / 16.0)).left(rems(left / 16.0));
        } else {
            base = base.top(px(top)).left(px(left));
        }
    }
    base.style().flex_grow = Some(rich.flex_grow);
    base.style().flex_shrink = Some(rich.flex_shrink);
    if let Some(flex_basis) = rich.flex_basis {
        if rich.rem_sized {
            base = base.flex_basis(rems(flex_basis / 16.0));
        } else {
            base = base.flex_basis(px(flex_basis));
        }
    }
    if let Some(scrollbar_width) = rich.scrollbar_width {
        base = base.scrollbar_width(px(scrollbar_width));
    }
    if let Some(align_items) = rich.align_items {
        base.style().align_items = Some(align_items);
    }
    if let Some(align_self) = rich.align_self {
        base.style().align_self = Some(align_self);
    }
    if let Some(align_content) = rich.align_content {
        base.style().align_content = Some(align_content);
    }
    if let Some(justify_content) = rich.justify_content {
        base.style().justify_content = Some(justify_content);
    }
    if let Some(aspect_ratio) = rich.aspect_ratio {
        base.style().aspect_ratio = Some(aspect_ratio);
    }
    if let Some(min_width) = rich.min_width {
        if rich.rem_sized {
            base = base.min_w(rems(min_width / 16.0));
        } else {
            base = base.min_w(px(min_width));
        }
    }
    if let Some(max_width) = rich.max_width {
        if rich.rem_sized {
            base = base.max_w(rems(max_width / 16.0));
        } else {
            base = base.max_w(px(max_width));
        }
    }
    if let Some(min_height) = rich.min_height {
        if rich.rem_sized {
            base = base.min_h(rems(min_height / 16.0));
        } else {
            base = base.min_h(px(min_height));
        }
    }
    if let Some(max_height) = rich.max_height {
        if rich.rem_sized {
            base = base.max_h(rems(max_height / 16.0));
        } else {
            base = base.max_h(px(max_height));
        }
    }
    if rich.margin > 0.0 {
        if rich.rem_sized {
            base = base.m(rems(rich.margin / 16.0));
        } else {
            base = base.m(px(rich.margin));
        }
    }
    if rich.rem_sized {
        base = base
            .gap(rems(style.gap / 16.0))
            .p(rems(style.padding / 16.0));
    } else {
        base = base.gap(px(style.gap)).p(px(style.padding));
    }
    if !rich.auto_width {
        if rich.rem_sized {
            base = base.w(rems(style.width / 16.0));
        } else {
            base = base.w(px(style.width));
        }
    }
    if !rich.auto_height {
        if rich.rem_sized {
            base = base.h(rems(style.height / 16.0));
        } else {
            base = base.h(px(style.height));
        }
    }
    if rich.text_override {
        apply_text_style(&mut base, &rich.text);
    }

    let base = base
        .when(style.vertical, |element| element.flex_col())
        .when(style.wrap, |element| element.flex_wrap())
        .bg(style.background)
        .when(options.hover_styles, |element| {
            element
                .hover(|element| {
                    element
                        .bg(style.border_color)
                        .border(px(style.border_width + 1.0))
                })
                .group_hover("randomized-tree-root", |element| element.opacity(0.65))
        })
        .border(px(style.border_width))
        .border_color(style.border_color)
        .rounded(px(style.corner_radius))
        .when(node.id.is_multiple_of(2), |element| {
            element.shadow(vec![
                crate::BoxShadow::new(px(-style.gap), px(style.gap), style.border_color)
                    .blur_radius(px(style.corner_radius)),
            ])
        })
        .opacity(style.opacity)
        .when(style.clipped, |element| element.overflow_hidden())
        .when(!style.visible, |element| element.invisible())
        .when(node.features.action_handler, |element| {
            element.on_action(|_: &RandomizedElementTreeAction, _window, _cx| {})
        })
        .when(node.features.key_context, |element| {
            element
                .key_context("RandomizedElementTree")
                .tab_index((node.id % 5) as isize)
                .tab_stop(!node.id.is_multiple_of(7))
        })
        .when(node.id.is_multiple_of(5), |element| element.tab_group())
        .when(node.features.animated, |element| {
            element.child(
                div()
                    .size(px(10.))
                    .flex_shrink_0()
                    .rounded_full()
                    .bg(style.border_color)
                    .with_animation(
                        ElementId::NamedInteger("randomized-spinner".into(), node.id),
                        crate::Animation::new(std::time::Duration::from_millis(800)).repeat(),
                        |element, delta| element.opacity(0.25 + 0.75 * delta),
                    ),
            )
        })
        .when(node.id % 4 == 1 || style.label != 0, |element| {
            let label = LABELS[style.label % LABELS.len()];
            element.underline().child(render_rich_text(
                node.id,
                label,
                &rich.text,
                options.id_generation,
            ))
        })
        .map(|element| match rich.measured {
            RandomizedMeasuredContent::None => element,
            RandomizedMeasuredContent::KeyedLeaf {
                width,
                height,
                color,
            } => element.child(RandomizedMeasuredLeaf {
                width,
                height,
                color,
                keyed: true,
            }),
            RandomizedMeasuredContent::UnkeyedLeaf {
                width,
                height,
                color,
            } => element.child(RandomizedMeasuredLeaf {
                width,
                height,
                color,
                keyed: false,
            }),
            RandomizedMeasuredContent::UniformList {
                item_count,
                item_width,
                item_height,
                scroll_offset,
                color,
            } => {
                let scroll_handle = {
                    let mut handles = uniform_list_scroll_handles.borrow_mut();
                    let entry = handles.entry(node.id).or_insert_with(|| {
                        let handle = UniformListScrollHandle::new();
                        handle
                            .0
                            .borrow()
                            .base_handle
                            .set_offset(point(px(0.0), px(-scroll_offset)));
                        (handle, rich.list_version)
                    });
                    if entry.1 != rich.list_version {
                        entry
                            .0
                            .0
                            .borrow()
                            .base_handle
                            .set_offset(point(px(0.0), px(-scroll_offset)));
                        entry.1 = rich.list_version;
                    }
                    entry.0.clone()
                };
                element.child(
                    uniform_list(
                        ElementId::NamedInteger(
                            "randomized-uniform-list".into(),
                            node.id | options.id_generation << ID_GENERATION_SHIFT,
                        ),
                        item_count,
                        move |range, _, _| {
                            range
                                .map(|_| div().w(px(item_width)).h(px(item_height)).bg(color))
                                .collect()
                        },
                    )
                    .with_sizing_behavior(ListSizingBehavior::Infer)
                    .track_scroll(&scroll_handle)
                    .when(scroll_offset > 0.0, |list| list.max_h(px(32.0))),
                )
            }
            RandomizedMeasuredContent::List {
                item_count,
                item_width,
                item_height,
                scroll_index,
                color,
            } => {
                let list_state = {
                    let mut states = list_states.borrow_mut();
                    let entry = states.entry(node.id).or_insert_with(|| {
                        let state = ListState::new(item_count, ListAlignment::Top, px(500.0));
                        if scroll_index > 0 && item_count > 0 {
                            state.scroll_to(ListOffset {
                                item_ix: scroll_index.min(item_count - 1),
                                offset_in_item: px(0.0),
                            });
                        }
                        (state, rich.list_version)
                    });
                    if entry.1 != rich.list_version {
                        entry.0.reset(item_count);
                        if scroll_index > 0 && item_count > 0 {
                            entry.0.scroll_to(ListOffset {
                                item_ix: scroll_index.min(item_count - 1),
                                offset_in_item: px(0.0),
                            });
                        }
                        entry.1 = rich.list_version;
                    }
                    entry.0.clone()
                };
                element.child(
                    list(list_state, move |_, _, _| {
                        div()
                            .w(px(item_width))
                            .h(px(item_height))
                            .bg(color)
                            .into_any_element()
                    })
                    .with_sizing_behavior(ListSizingBehavior::Infer)
                    .when(scroll_index > 0, |list| list.max_h(px(32.0))),
                )
            }
        })
        .when_some(rich.popup.as_ref(), |element, popup| {
            element.child(
                deferred(
                    anchored()
                        .position(point(px(popup.offset_x), px(popup.offset_y)))
                        .snap_to_window()
                        .child(
                            div()
                                .w(px(popup.width))
                                .h(px(popup.height))
                                .bg(popup.color)
                                .child(LABELS[popup.label % LABELS.len()]),
                        ),
                )
                .with_priority(popup.priority),
            )
        })
        .child({
            let color = style.border_color;
            let width = style.border_width;
            let bend = style.corner_radius;
            crate::canvas(
                |_, _, _| (),
                move |bounds, (), window, _| {
                    if !options.paths {
                        return;
                    }
                    for index in 0..6 {
                        let mut path = if index % 2 == 0 {
                            crate::PathBuilder::fill()
                        } else {
                            crate::PathBuilder::stroke(px(width + 0.5))
                        };
                        let origin =
                            bounds.origin + crate::point(px(index as f32 * 3.0 - 8.0), px(-4.0));
                        path.move_to(origin);
                        path.curve_to(
                            origin + crate::point(bounds.size.width, bounds.size.height),
                            origin + crate::point(px(bend), px(-bend)),
                        );
                        path.line_to(origin + crate::point(px(-bend), bounds.size.height));
                        path.close();
                        window.paint_path(
                            path.build().expect("generated path must tessellate"),
                            color,
                        );
                    }
                },
            )
            .absolute()
            .size_full()
        })
        .children(node.children.iter().map(|child| {
            if let Some(entities) = entities {
                render_node(
                    child,
                    entities,
                    scroll_handles,
                    list_states,
                    uniform_list_scroll_handles,
                    work_counters,
                    options,
                )
            } else {
                render_node_body(
                    child,
                    None,
                    scroll_handles,
                    list_states,
                    uniform_list_scroll_handles,
                    work_counters,
                    options,
                )
            }
        }));

    let element = if keyed {
        base.id(ElementId::NamedInteger(
            "randomized-element".into(),
            node.id | options.id_generation << ID_GENERATION_SHIFT,
        ))
        .when(is_scroll, |element| {
            let mut handles = scroll_handles.borrow_mut();
            let handle = handles
                .entry(node.id)
                .or_insert_with(crate::ScrollHandle::new);
            element.overflow_scroll().track_scroll(handle)
        })
        .when(node.features.click_handler, |element| {
            element.cursor_pointer().on_click(|_event, _window, _cx| {})
        })
        .into_any_element()
    } else {
        base.into_any_element()
    };

    if let Some(rem_size) = rich.rem_size_override {
        RemSizeScope {
            rem_size: px(rem_size),
            child: element,
        }
        .into_any_element()
    } else {
        element
    }
}

fn insert_node(
    root_children: &mut Vec<RandomizedElementNode>,
    parent_path: &mut Vec<usize>,
    node: RandomizedElementNode,
) -> (usize, Vec<usize>) {
    let Some(children) = children_at_path_mut(root_children, parent_path) else {
        parent_path.clear();
        let index = root_children.len();
        root_children.push(node);
        return (index, vec![index]);
    };

    let index = children.len();
    children.push(node);
    let mut inserted_path = parent_path.clone();
    inserted_path.push(index);
    (index, inserted_path)
}

fn random_parent_path(root_children: &[RandomizedElementNode], rng: &mut StdRng) -> Vec<usize> {
    let paths = node_paths(root_children);
    let index = rng.random_range(0..=paths.len());
    paths.get(index).cloned().unwrap_or_default()
}

fn random_node_path(
    root_children: &[RandomizedElementNode],
    rng: &mut StdRng,
) -> Option<Vec<usize>> {
    let paths = node_paths(root_children);
    if paths.is_empty() {
        None
    } else {
        paths.get(rng.random_range(0..paths.len())).cloned()
    }
}

fn random_leaf_path(
    root_children: &[RandomizedElementNode],
    rng: &mut StdRng,
) -> Option<Vec<usize>> {
    let leaves = node_paths(root_children)
        .into_iter()
        .filter(|path| {
            node_at_path(root_children, path).is_some_and(|node| node.children.is_empty())
        })
        .collect::<Vec<_>>();
    if leaves.is_empty() {
        None
    } else {
        leaves.get(rng.random_range(0..leaves.len())).cloned()
    }
}

fn parent_paths_with_multiple_children(root_children: &[RandomizedElementNode]) -> Vec<Vec<usize>> {
    let mut parent_paths = Vec::new();
    if root_children.len() >= 2 {
        parent_paths.push(Vec::new());
    }
    parent_paths.extend(node_paths(root_children).into_iter().filter(|path| {
        node_at_path(root_children, path).is_some_and(|node| node.children.len() >= 2)
    }));
    parent_paths
}

fn node_paths(root_children: &[RandomizedElementNode]) -> Vec<Vec<usize>> {
    let mut paths = Vec::new();
    let mut stack = root_children
        .iter()
        .enumerate()
        .rev()
        .map(|(index, node)| (vec![index], node))
        .collect::<Vec<_>>();

    while let Some((path, node)) = stack.pop() {
        for (index, child) in node.children.iter().enumerate().rev() {
            let mut child_path = path.clone();
            child_path.push(index);
            stack.push((child_path, child));
        }
        paths.push(path);
    }
    paths
}

fn node_at_path<'a>(
    root_children: &'a [RandomizedElementNode],
    path: &[usize],
) -> Option<&'a RandomizedElementNode> {
    let (index, parent_path) = path.split_last()?;
    children_at_path(root_children, parent_path)?.get(*index)
}

fn node_at_path_mut<'a>(
    root_children: &'a mut Vec<RandomizedElementNode>,
    path: &[usize],
) -> Option<&'a mut RandomizedElementNode> {
    let (index, parent_path) = path.split_last()?;
    children_at_path_mut(root_children, parent_path)?.get_mut(*index)
}

fn children_at_path<'a>(
    mut children: &'a [RandomizedElementNode],
    path: &[usize],
) -> Option<&'a [RandomizedElementNode]> {
    for index in path {
        children = &children.get(*index)?.children;
    }
    Some(children)
}

fn children_at_path_mut<'a>(
    mut children: &'a mut Vec<RandomizedElementNode>,
    path: &[usize],
) -> Option<&'a mut Vec<RandomizedElementNode>> {
    for index in path {
        children = &mut children.get_mut(*index)?.children;
    }
    Some(children)
}

fn nearest_entity_id(root_children: &[RandomizedElementNode], path: &[usize]) -> Option<u64> {
    let mut children = root_children;
    let mut nearest_entity_id = None;
    for index in path {
        let node = children.get(*index)?;
        if node.is_entity {
            nearest_entity_id = Some(node.id);
        }
        children = &node.children;
    }
    nearest_entity_id
}

fn normalized_density(density: f64) -> f64 {
    if density.is_nan() {
        0.0
    } else {
        density.clamp(0.0, 1.0)
    }
}

fn random_color(rng: &mut StdRng) -> Hsla {
    Hsla {
        h: rng.random_range(0.0..1.0),
        s: rng.random_range(0.25..0.9),
        l: rng.random_range(0.18..0.82),
        a: 1.0,
    }
}

fn random_opacity(rng: &mut StdRng) -> f32 {
    rng.random_range(0.25..1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AnyWindowHandle, Background, BorderStyle, Bounds, ContentMask, Corners, Edges, Pixels,
        ScaledPixels, Size, TestAppContext, size,
    };
    use proptest::prelude::*;
    use std::collections::BTreeSet;

    const ELEMENT_COUNT: usize = 64;
    const MUTATION_COUNT: usize = 32;
    const WINDOW_SIZE: Size<Pixels> = size(px(1200.0), px(900.0));

    #[derive(Debug, PartialEq)]
    struct SceneSnapshot {
        quads: Vec<QuadSnapshot>,
        /// Every other primitive kind, formatted without the per-window atlas tile
        /// IDs, which differ between two windows drawing the same scene.
        other_primitives: Vec<String>,
        element_ids: BTreeSet<u64>,
    }

    impl SceneSnapshot {
        fn from_window(window: &Window) -> Self {
            let scene = &window.rendered_frame.scene;
            let other_primitives = scene
                .shadows
                .iter()
                .map(|shadow| format!("{shadow:?}"))
                .chain(
                    scene
                        .underlines
                        .iter()
                        .map(|underline| format!("{underline:?}")),
                )
                .chain(scene.paths.iter().map(|path| format!("{path:?}")))
                .chain(scene.monochrome_sprites.iter().map(|sprite| {
                    format!(
                        "monochrome {} {:?} {:?} {:?} {:?}",
                        sprite.order,
                        sprite.bounds,
                        sprite.content_mask,
                        sprite.color,
                        sprite.transformation
                    )
                }))
                .chain(scene.subpixel_sprites.iter().map(|sprite| {
                    format!(
                        "subpixel {} {:?} {:?} {:?} {:?}",
                        sprite.order,
                        sprite.bounds,
                        sprite.content_mask,
                        sprite.color,
                        sprite.transformation
                    )
                }))
                .chain(scene.polychrome_sprites.iter().map(|sprite| {
                    format!(
                        "polychrome {} {:?} {:?} {:?} {}",
                        sprite.order,
                        sprite.bounds,
                        sprite.content_mask,
                        sprite.grayscale,
                        sprite.opacity
                    )
                }))
                .collect();
            Self {
                quads: scene.quads.iter().map(QuadSnapshot::from).collect(),
                other_primitives,
                element_ids: window
                    .rendered_frame
                    .element_states
                    .keys()
                    .filter_map(|(global_id, _)| {
                        let ElementId::NamedInteger(name, element_id) = global_id.last()? else {
                            return None;
                        };
                        (name.as_ref() == "randomized-element")
                            .then_some(element_id & ((1 << ID_GENERATION_SHIFT) - 1))
                    })
                    .collect(),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct QuadSnapshot {
        order: u32,
        border_style: BorderStyle,
        bounds: Bounds<ScaledPixels>,
        content_mask: ContentMask<ScaledPixels>,
        background: Background,
        border_color: Hsla,
        corner_radii: Corners<ScaledPixels>,
        border_widths: Edges<ScaledPixels>,
    }

    impl From<&crate::Quad> for QuadSnapshot {
        fn from(quad: &crate::Quad) -> Self {
            Self {
                order: quad.order,
                border_style: quad.border_style,
                bounds: quad.bounds,
                content_mask: quad.content_mask,
                background: quad.background,
                border_color: quad.border_color,
                corner_radii: quad.corner_radii,
                border_widths: quad.border_widths,
            }
        }
    }

    #[gpui::test]
    fn localized_recoloring_notifies_one_entity_until_its_subtree_is_exhausted(
        cx: &mut TestAppContext,
    ) {
        // Wide: every element is a root child, so every entity's subtree is exactly one
        // element and localized change of k elements must touch k boundaries only when it
        // spills; a mixed tree has larger subtrees to stay inside.
        let config = RandomizedElementTreeConfig::new(11, 200)
            .with_topology(RandomizedElementTreeTopology::Mixed)
            .with_entity_density(0.1);
        let window = cx.open_window(WINDOW_SIZE, move |_, cx| {
            RandomizedElementTree::new_with_config(config, cx)
        });
        let root = window.root(cx).expect("window has a root");
        let (localized, spread) = root.update(cx, |tree, cx| {
            let localized = tree.recolor_children(20, ChangeLocality::Localized, cx);
            let spread = tree.recolor_children(20, ChangeLocality::Spread, cx);
            (localized, spread)
        });
        assert_eq!(localized.recolored, 20);
        assert_eq!(spread.recolored, 20);
        assert!(
            localized.notified <= spread.notified,
            "localized change touches no more boundaries than spread change: {localized:?} vs {spread:?}"
        );
        assert!(localized.notified >= 1);
    }

    #[test]
    fn bounds_sample_within_range_and_reproduce_from_the_seed() {
        let bounds = RandomizedElementTreeBounds::new(32..=256)
            .with_topologies([
                RandomizedElementTreeTopology::Deep,
                RandomizedElementTreeTopology::Narrow,
            ])
            .with_entity_density(0.1..=0.5)
            .with_handler_density(0.25..=0.25);
        let mut topologies = BTreeSet::new();
        let sample = |seed| bounds.sample(&mut StdRng::seed_from_u64(seed));
        for seed in 0..64 {
            let config = sample(seed);
            assert!((32..=256).contains(&config.element_count()), "{config:?}");
            assert!(config.topology().is_tall(), "{config:?}");
            assert!((0.1..=0.5).contains(&config.entity_density()), "{config:?}");
            assert_eq!(config.handler_density(), 0.25);
            assert_eq!(sample(seed), config, "a seed draws the same config twice");
            topologies.insert(format!("{:?}", config.topology()));
        }
        assert_eq!(topologies.len(), 2, "64 seeds pick both allowed topologies");
        assert_ne!(sample(0), sample(1));
    }

    #[test]
    fn generated_topologies_include_wide_deep_narrow_and_empty_trees() {
        let wide = RandomizedElementTree::new(0, ELEMENT_COUNT).snapshot();
        assert_eq!(wide.children.len(), ELEMENT_COUNT);
        assert_eq!(wide.max_depth(), 1);

        let deep = RandomizedElementTree::new(1, ELEMENT_COUNT).snapshot();
        assert_eq!(deep.children.len(), 1);
        assert_eq!(deep.max_depth(), ELEMENT_COUNT);

        let narrow = RandomizedElementTree::new(3, ELEMENT_COUNT).snapshot();
        assert_eq!(narrow.descendant_count(), ELEMENT_COUNT);
        assert!(narrow.max_depth() > 1);

        let empty = RandomizedElementTree::new(2, 0).snapshot();
        assert_eq!(empty.descendant_count(), 0);
        assert_eq!(empty.max_depth(), 0);
    }

    #[test]
    fn randomized_element_ids_are_unique_and_stable() {
        let mut tree = RandomizedElementTree::new(0, ELEMENT_COUNT);
        let initial_ids = tree.snapshot().element_ids();
        assert_eq!(initial_ids.len(), ELEMENT_COUNT);
        assert_eq!(
            initial_ids.iter().copied().collect::<BTreeSet<_>>().len(),
            ELEMENT_COUNT
        );

        for mutation in [
            RandomizedChildMutation::Color,
            RandomizedChildMutation::Opacity,
            RandomizedChildMutation::Border,
            RandomizedChildMutation::Bounds,
            RandomizedChildMutation::Clipping,
            RandomizedChildMutation::Visibility,
            RandomizedChildMutation::Text,
        ] {
            tree.mutate_random_child(mutation);
            assert_eq!(tree.snapshot().element_ids(), initial_ids);
        }

        tree.reorder_child();
        let reordered_ids = tree.snapshot().element_ids();
        assert_eq!(
            reordered_ids.iter().copied().collect::<BTreeSet<_>>(),
            initial_ids.iter().copied().collect()
        );

        tree.insert_child();
        let inserted_ids = tree.snapshot().element_ids();
        let inserted_id_set = inserted_ids.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(inserted_ids.len(), ELEMENT_COUNT + 1);
        assert!(initial_ids.iter().all(|id| inserted_id_set.contains(id)));

        tree.remove_child();
        let remaining_ids = tree.snapshot().element_ids();
        assert_eq!(
            remaining_ids.iter().copied().collect::<BTreeSet<_>>().len(),
            remaining_ids.len()
        );
        assert!(remaining_ids.iter().all(|id| inserted_id_set.contains(id)));
    }

    #[gpui::test]
    fn randomized_entities_and_work_counters_support_incremental_benchmarks(
        cx: &mut TestAppContext,
    ) {
        let config = RandomizedElementTreeConfig::new(0, 16)
            .with_topology(RandomizedElementTreeTopology::Wide)
            .with_entity_density(1.0)
            .with_handler_density(1.0);
        let window = cx.open_window(WINDOW_SIZE, move |_, cx| {
            RandomizedElementTree::new_with_config(config, cx)
        });
        let root = window
            .root(cx)
            .expect("randomized tree window should have a root");
        let window = AnyWindowHandle::from(window);

        draw_window(window, cx);
        let initial_counters = root.read_with(cx, |tree, _| tree.work_counters());
        assert!(initial_counters.root_render_count() >= 1);
        assert!(initial_counters.entity_render_count() >= 16);
        assert!(initial_counters.element_render_count() >= 16);
        assert!(initial_counters.handler_registration_count() >= 32);

        let mutation = root.update(cx, |tree, cx| {
            tree.reset_work_counters();
            tree.apply_mutation(RandomizedElementTreeMutationKind::ChildColor, cx)
        });
        let RandomizedElementTreeMutation::ChildColor { element_id } = mutation else {
            panic!("a nonempty tree should apply the requested child-color mutation");
        };
        draw_window(window, cx);

        let incremental_counters = root.read_with(cx, |tree, _| tree.work_counters());
        assert!(incremental_counters.root_render_count() >= 1);
        assert!(incremental_counters.entity_render_count() >= 1);
        assert!(incremental_counters.element_render_count() >= 1);
        assert!(incremental_counters.handler_registration_count() >= 2);
        assert!(
            incremental_counters
                .rendered_entity_ids()
                .contains(&element_id)
        );

        cx.update_window(window, |_, window, _| window.remove_window())
            .expect("randomized tree window should remain open until the case completes");
    }

    #[gpui::property_test(config = ProptestConfig {
        cases: 4,
        ..Default::default()
    })]
    fn randomized_element_tree_matches_full_render(cx: &mut TestAppContext, seed: u64) {
        for topology in 0..4 {
            let topology_seed = (seed & !3) | topology;
            let config = RandomizedElementTreeConfig::new(topology_seed, ELEMENT_COUNT)
                .with_entity_density(0.2)
                .with_handler_density(0.2);
            run_render_equivalence_case(cx, config);
        }
        run_render_equivalence_case(
            cx,
            RandomizedElementTreeConfig::new(seed, 0)
                .with_entity_density(0.2)
                .with_handler_density(0.2),
        );
    }

    fn run_render_equivalence_case(cx: &mut TestAppContext, config: RandomizedElementTreeConfig) {
        let persistent_window = cx.open_window(WINDOW_SIZE, move |_, cx| {
            RandomizedElementTree::new_with_config(config, cx)
        });
        let persistent_root = persistent_window
            .root(cx)
            .expect("randomized tree window should have a root");
        let persistent_window = AnyWindowHandle::from(persistent_window);

        assert_matches_reference(&persistent_root, persistent_window, cx, config, None);

        for mutation_index in 0..MUTATION_COUNT {
            let mutation = persistent_root.update(cx, |tree, cx| match mutation_index {
                0 => tree.apply_mutation(RandomizedElementTreeMutationKind::Insert, cx),
                1 => tree.apply_mutation(RandomizedElementTreeMutationKind::Remove, cx),
                _ => tree.apply_random_mutation(cx),
            });

            assert_matches_reference(
                &persistent_root,
                persistent_window,
                cx,
                config,
                Some((mutation_index, mutation)),
            );
        }

        cx.update_window(persistent_window, |_, window, _| window.remove_window())
            .expect("persistent window should remain open until the case completes");
    }

    fn assert_matches_reference(
        persistent_root: &crate::Entity<RandomizedElementTree>,
        persistent_window: AnyWindowHandle,
        cx: &mut TestAppContext,
        config: RandomizedElementTreeConfig,
        mutation: Option<(usize, RandomizedElementTreeMutation)>,
    ) {
        let snapshot = persistent_root.read_with(cx, |tree, _| tree.snapshot());
        let expected_element_ids = snapshot
            .keyed_element_ids()
            .into_iter()
            .collect::<BTreeSet<_>>();
        let persistent_scene = draw_window(persistent_window, cx);
        assert_eq!(
            persistent_scene.element_ids, expected_element_ids,
            "persistent frame element IDs diverged from the model for config {config:?}, after \
             mutation {mutation:?}"
        );
        let reference_scene = draw_fresh_reference(snapshot, cx);

        assert_eq!(
            persistent_scene, reference_scene,
            "rendered scene diverged for config {config:?}, after mutation {mutation:?}"
        );
    }

    fn draw_fresh_reference(
        snapshot: RandomizedElementTreeSnapshot,
        cx: &mut TestAppContext,
    ) -> SceneSnapshot {
        let reference_window = cx.open_window(WINDOW_SIZE, move |_, cx| {
            RandomizedElementTree::from_snapshot_with_entities(snapshot, cx)
        });
        let reference_window = AnyWindowHandle::from(reference_window);
        let scene = draw_window(reference_window, cx);
        cx.update_window(reference_window, |_, window, _| window.remove_window())
            .expect("reference window should remain open until its frame is captured");
        scene
    }

    /// Applies the same random changes to two windows, one keeping its layout nodes
    /// between frames and one rebuilding them every frame, and checks they draw the same
    /// scene and hitboxes after every change.
    #[gpui::test(iterations = 16)]
    fn layout_cache_matches_rebuilding_every_frame(mut rng: StdRng, cx: &mut TestAppContext) {
        let seed = rng.random();
        let element_count = rng.random_range(0..ELEMENT_COUNT);
        let config = RandomizedElementTreeConfig::new(seed, element_count)
            .with_entity_density(rng.random_range(0.0..0.5))
            .with_handler_density(0.2)
            .with_cached_views(rng.random_bool(0.3));
        let [cached, uncached] = [(); 2].map(|()| {
            let window = cx.open_window(WINDOW_SIZE, move |_, cx| {
                RandomizedElementTree::new_with_config(config, cx)
            });
            let root = window
                .root(cx)
                .expect("randomized tree window should have a root");
            (AnyWindowHandle::from(window), root)
        });

        // Hiding an ancestor of `.cached()` views can panic on `main` today, with or
        // without the layout cache, so cached-view trees leave it out.
        let mutations = RandomizedElementTreeMutationKind::ALL
            .into_iter()
            .filter(|kind| {
                !config.cached_views
                    || !matches!(
                        kind,
                        RandomizedElementTreeMutationKind::RootVisibility
                            | RandomizedElementTreeMutationKind::ChildVisibility
                    )
            })
            .collect::<Vec<_>>();
        for step in 0..48 {
            let change = rng.random_range(0..100);
            let mutation = mutations[rng.random_range(0..mutations.len())];
            let window_size = size(
                px(rng.random_range(200.0..1400.0)),
                px(rng.random_range(150.0..1000.0)),
            );
            let rem_size = px(rng.random_range(8.0..24.0));
            let scale_factor = [1.0, 1.25, 1.5, 2.0, 2.5][rng.random_range(0..5)];
            let mouse_position = point(
                px(rng.random_range(0.0..WINDOW_SIZE.width.0)),
                px(rng.random_range(0.0..WINDOW_SIZE.height.0)),
            );
            for (window, root) in [&cached, &uncached] {
                match change {
                    0..60 => {
                        root.update(cx, |tree, cx| tree.apply_mutation(mutation, cx));
                    }
                    60..68 => root.update(cx, |tree, cx| tree.churn_element_ids(cx)),
                    68..76 => cx.simulate_window_resize(*window, window_size),
                    76..82 => cx
                        .update_window(*window, |_, window, _| window.set_rem_size(rem_size))
                        .expect("window should remain open"),
                    82..88 => cx
                        .update_window(*window, |_, window, _| {
                            window.set_scale_factor(scale_factor)
                        })
                        .expect("window should remain open"),
                    88..94 => {
                        cx.update_window(*window, |_, window, cx| {
                            window.dispatch_event(
                                crate::PlatformInput::MouseMove(crate::MouseMoveEvent {
                                    position: mouse_position,
                                    pressed_button: None,
                                    modifiers: crate::Modifiers::default(),
                                }),
                                cx,
                            );
                        })
                        .expect("window should remain open");
                    }
                    _ => root.update(cx, |_, cx| cx.notify()),
                }
            }
            cx.update_window(uncached.0, |_, window, _| window.clear_layout_cache())
                .expect("window should remain open");
            let [cached_frame, uncached_frame] =
                [&cached, &uncached].map(|(window, _)| draw_window_with_hitboxes(*window, cx));
            if cached_frame != uncached_frame {
                let first_quad_diff = cached_frame
                    .0
                    .quads
                    .iter()
                    .zip(&uncached_frame.0.quads)
                    .enumerate()
                    .find(|(_, (a, b))| a != b);
                let first_hitbox_diff = cached_frame
                    .1
                    .iter()
                    .zip(&uncached_frame.1)
                    .enumerate()
                    .find(|(_, (a, b))| a != b);
                panic!(
                    "cached layout diverged at step {step} (change {change}, mutation {mutation:?}, \
                     scale_factor {scale_factor}, rem_size {rem_size:?}) for {config:?}\n\
                     quad counts: cached={} uncached={}\n\
                     other_primitives counts: cached={} uncached={}\n\
                     hitbox counts: cached={} uncached={}\n\
                     first_quad_diff: {first_quad_diff:?}\n\
                     first_hitbox_diff: {first_hitbox_diff:?}",
                    cached_frame.0.quads.len(),
                    uncached_frame.0.quads.len(),
                    cached_frame.0.other_primitives.len(),
                    uncached_frame.0.other_primitives.len(),
                    cached_frame.1.len(),
                    uncached_frame.1.len(),
                );
            }
        }

        for (window, _) in [cached, uncached] {
            cx.update_window(window, |_, window, _| window.remove_window())
                .expect("window should remain open until the case completes");
        }
    }

    fn draw_window_with_hitboxes(
        window: AnyWindowHandle,
        cx: &mut TestAppContext,
    ) -> (SceneSnapshot, Vec<Bounds<Pixels>>) {
        let scene = draw_window(window, cx);
        let hitboxes = cx
            .update_window(window, |_, window, _| {
                window
                    .rendered_frame
                    .hitboxes
                    .iter()
                    .map(|hitbox| hitbox.bounds)
                    .collect()
            })
            .expect("window should remain open");
        (scene, hitboxes)
    }

    fn draw_window(window: AnyWindowHandle, cx: &mut TestAppContext) -> SceneSnapshot {
        cx.update_window(window, |_, window, cx| {
            let arena_clear_needed = window.draw(cx);
            let scene = SceneSnapshot::from_window(window);
            arena_clear_needed.clear(cx);
            scene
        })
        .expect("randomized element tree window should remain open while drawing")
    }
}
