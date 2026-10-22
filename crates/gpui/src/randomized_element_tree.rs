use crate::{
    AnyElement, AppContext as _, Context, ElementId, Entity, Hsla, InteractiveElement, IntoElement,
    ParentElement, Render, StatefulInteractiveElement, Styled, Window, div, px,
    util::FluentBuilder,
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
    cached_views: bool,
    hover_styles: bool,
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
            cached_views: false,
            hover_styles: false,
        }
    }

    /// Use definite-size cached entity boundaries in this separate workload variant.
    pub fn with_cached_views(mut self, cached: bool) -> Self {
        self.cached_views = cached;
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RandomizedElementTreeWorkCounters {
    root_render_count: usize,
    entity_render_count: usize,
    element_render_count: usize,
    handler_registration_count: usize,
    rendered_entity_ids: Vec<u64>,
}

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
    cached_views: bool,
    hover_styles: bool,
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
    pub const ALL: [Self; 16] = [
        Self::RootColor,
        Self::RootOpacity,
        Self::RootBorder,
        Self::RootBounds,
        Self::RootClipping,
        Self::RootVisibility,
        Self::ChildColor,
        Self::ChildOpacity,
        Self::ChildBorder,
        Self::ChildBounds,
        Self::ChildClipping,
        Self::ChildVisibility,
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
            let node = RandomizedElementNode::random(
                element_index as u64,
                config.entity_density,
                config.handler_density,
                &mut rng,
            );
            let (_, inserted_path) = insert_node(&mut children, &mut parent_path, node);
            deepest_path = inserted_path;
        }

        let mut snapshot = RandomizedElementTreeSnapshot {
            root: RandomizedElementStyle::random_root(&mut rng),
            children,
            next_element_id: config.element_count as u64,
            cached_views: config.cached_views,
            hover_styles: config.hover_styles,
            paths_by_element_id: HashMap::default(),
        };
        snapshot.rebuild_element_paths();

        Self {
            snapshot: Rc::new(RefCell::new(snapshot)),
            entities: Rc::new(RefCell::new(HashMap::default())),
            scroll_handles: Rc::default(),
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

    /// Changes tracked scroll offsets without notifying any entity.
    ///
    /// Callers must request a frame, as with a scroll event handled by the window.
    pub fn scroll_without_notify(&mut self) {
        let mut snapshot = self.snapshot.borrow_mut();
        let handles = self.scroll_handles.borrow();
        let mut element_ids = handles.keys().copied().collect::<Vec<_>>();
        element_ids.sort_unstable();
        for element_id in element_ids {
            let Some(path) = snapshot.paths_by_element_id.get(&element_id).cloned() else {
                continue;
            };
            let offset = crate::point(
                px(-self.rng.random_range(0.0..160.0)),
                px(-self.rng.random_range(0.0..240.0)),
            );
            if let Some(node) = node_at_path_mut(&mut snapshot.children, &path) {
                node.scroll_offset = offset;
                if let Some(handle) = handles.get(&element_id) {
                    handle.set_offset(offset);
                }
            }
        }
    }

    /// Resolve a view's logical element identity for independent-window verification.
    pub fn view_element_identity(&self, entity_id: crate::EntityId) -> Option<u64> {
        self.entities
            .borrow()
            .iter()
            .find_map(|(element_id, entity)| {
                (entity.entity_id() == entity_id).then_some(*element_id)
            })
    }

    /// Clears render work accumulated since construction or the previous reset.
    pub fn reset_work_counters(&mut self) {
        self.work_counters.borrow_mut().reset();
    }

    /// Returns work accumulated since construction or the previous reset.
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

    /// Alternates one fixed leaf between two colors without changing the tree's shape.
    ///
    /// Sampling a different descendant each frame changes which cached subtree renders,
    /// so short and long benchmark samples otherwise measure different workload mixes.
    pub fn recolor_first_leaf(&mut self, cx: &mut Context<Self>) -> Option<u64> {
        let (element_id, owner) = {
            let mut snapshot = self.snapshot.borrow_mut();
            let path = node_paths(&snapshot.children).into_iter().find(|path| {
                node_at_path(&snapshot.children, path).is_some_and(|node| node.children.is_empty())
            })?;
            let owner = nearest_entity_id(&snapshot.children, &path);
            let node = node_at_path_mut(&mut snapshot.children, &path)?;
            let first = crate::hsla(0., 1., 0.5, 1.);
            let second = crate::hsla(0.5, 1., 0.5, 1.);
            node.style.background = if node.style.background == first {
                second
            } else {
                first
            };
            (node.id, owner)
        };
        self.notify_owner(owner, cx);
        Some(element_id)
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
        let node = RandomizedElementNode::random(
            element_id,
            self.config.entity_density,
            self.config.handler_density,
            &mut self.rng,
        );
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
        let owners = {
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
            owners
        };
        let notified = owners.len();
        let recolored = count.min(self.snapshot.borrow().descendant_count());
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
        // Cached bounds belong to the composing view, not the child's rendered contents.
        let external_style_changed = snapshot.cached_views
            && matches!(mutation, RandomizedChildMutation::Bounds)
            && node_at_path(&snapshot.children, &path).is_some_and(|node| node.is_entity);
        let owner_path = if external_style_changed {
            path.split_last()
                .map(|(_, parent)| parent)
                .unwrap_or_default()
        } else {
            &path
        };
        let owner_entity_id = nearest_entity_id(&snapshot.children, owner_path);
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
                node.style.width = if self.rng.random_bool(0.125) {
                    0.0
                } else {
                    self.rng.random_range(36.0..220.0)
                };
                node.style.height = if self.rng.random_bool(0.125) {
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
                work_counters: self.work_counters.clone(),
            });
            self.entities.borrow_mut().insert(element_id, entity);
        }

        let entity_count = self.entities.borrow().len();
        let mut work_counters = self.work_counters.borrow_mut();
        let additional_capacity =
            entity_count.saturating_sub(work_counters.rendered_entity_ids.len());
        work_counters
            .rendered_entity_ids
            .reserve(additional_capacity);
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
        work_counters.root_render_count += 1;

        div()
            .flex()
            .when(snapshot.hover_styles, |element| {
                element.group("randomized-tree-root")
            })
            .when(root.vertical, |element| element.flex_col())
            .when(root.wrap, |element| element.flex_wrap())
            .gap(px(root.gap))
            .p(px(root.padding))
            .w(px(root.width))
            .h(px(root.height))
            .bg(root.background)
            .border(px(root.border_width))
            .border_color(root.border_color)
            .rounded(px(root.corner_radius))
            .opacity(root.opacity)
            .when(root.clipped, |element| element.overflow_hidden())
            .when(!root.visible, |element| element.invisible())
            .children(snapshot.children.iter().map(|node| {
                render_node(
                    node,
                    &entities,
                    &self.scroll_handles,
                    &mut work_counters,
                    snapshot.cached_views,
                    snapshot.hover_styles,
                )
            }))
    }
}

type RandomizedElementEntities = HashMap<u64, Entity<RandomizedElementEntity>>;

struct RandomizedElementEntity {
    element_id: u64,
    snapshot: Rc<RefCell<RandomizedElementTreeSnapshot>>,
    entities: Weak<RefCell<RandomizedElementEntities>>,
    scroll_handles: Rc<RefCell<HashMap<u64, crate::ScrollHandle>>>,
    work_counters: Rc<RefCell<RandomizedElementTreeWorkCounters>>,
}

impl Render for RandomizedElementEntity {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let snapshot = self.snapshot.borrow();
        let entities = self.entities.upgrade();
        let entities = entities.as_ref().map(|entities| entities.borrow());
        let mut work_counters = self.work_counters.borrow_mut();
        work_counters.entity_render_count += 1;
        work_counters.rendered_entity_ids.push(self.element_id);

        snapshot
            .node(self.element_id)
            .map(|node| {
                render_node_body(
                    node,
                    entities.as_deref(),
                    &self.scroll_handles,
                    &mut work_counters,
                    snapshot.cached_views,
                    snapshot.hover_styles,
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
    scroll_offset: crate::Point<crate::Pixels>,
    is_entity: bool,
    features: RandomizedElementFeatures,
    children: Vec<RandomizedElementNode>,
}

impl RandomizedElementNode {
    fn random(id: u64, entity_density: f64, handler_density: f64, rng: &mut StdRng) -> Self {
        Self {
            id,
            style: RandomizedElementStyle::random_child(rng),
            scroll_offset: crate::Point::default(),
            is_entity: rng.random_bool(entity_density),
            features: RandomizedElementFeatures::random(handler_density, rng),
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
}

#[derive(Clone, Debug, PartialEq)]
struct RandomizedElementFeatures {
    action_handler: bool,
    click_handler: bool,
    key_context: bool,
}

impl RandomizedElementFeatures {
    fn random(handler_density: f64, rng: &mut StdRng) -> Self {
        Self {
            action_handler: rng.random_bool(handler_density),
            click_handler: rng.random_bool(handler_density),
            key_context: rng.random_bool(handler_density),
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
}

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
        }
    }

    fn randomize_border(&mut self, rng: &mut StdRng) {
        self.border_color = random_color(rng);
        self.border_width = rng.random_range(0.0..5.0);
        self.corner_radius = rng.random_range(0.0..18.0);
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
}

#[cfg_attr(feature = "stacker", stacksafe::stacksafe)]
fn render_node(
    node: &RandomizedElementNode,
    entities: &RandomizedElementEntities,
    scroll_handles: &RefCell<HashMap<u64, crate::ScrollHandle>>,
    work_counters: &mut RandomizedElementTreeWorkCounters,
    cached_views: bool,
    hover_styles: bool,
) -> AnyElement {
    if node.is_entity
        && let Some(entity) = entities.get(&node.id)
    {
        if cached_views {
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
        work_counters,
        cached_views,
        hover_styles,
    )
}

#[cfg_attr(feature = "stacker", stacksafe::stacksafe)]
fn render_node_body(
    node: &RandomizedElementNode,
    entities: Option<&RandomizedElementEntities>,
    scroll_handles: &RefCell<HashMap<u64, crate::ScrollHandle>>,
    work_counters: &mut RandomizedElementTreeWorkCounters,
    cached_views: bool,
    hover_styles: bool,
) -> AnyElement {
    work_counters.element_render_count += 1;
    work_counters.handler_registration_count += node.features.handler_count();
    let style = &node.style;
    div()
        .id(ElementId::NamedInteger(
            "randomized-element".into(),
            node.id,
        ))
        .flex()
        .flex_shrink_0()
        .when(style.vertical, |element| element.flex_col())
        .when(style.wrap, |element| element.flex_wrap())
        .gap(px(style.gap))
        .p(px(style.padding))
        .w(px(style.width))
        .h(px(style.height))
        .bg(style.background)
        .when(hover_styles, |element| {
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
        .when(node.id.is_multiple_of(3), |element| {
            let mut handles = scroll_handles.borrow_mut();
            let handle = handles.entry(node.id).or_insert_with(|| {
                let handle = crate::ScrollHandle::new();
                handle.set_offset(node.scroll_offset);
                handle
            });
            element.overflow_scroll().track_scroll(handle)
        })
        .when(!style.visible, |element| element.invisible())
        .when(node.features.action_handler, |element| {
            element.on_action(|_: &RandomizedElementTreeAction, _window, _cx| {})
        })
        .when(node.features.click_handler, |element| {
            element.cursor_pointer().on_click(|_event, _window, _cx| {})
        })
        .when(node.features.key_context, |element| {
            element
                .key_context("RandomizedElementTree")
                .tab_index((node.id % 5) as isize)
                .tab_stop(!node.id.is_multiple_of(7))
        })
        .when(node.id.is_multiple_of(5), |element| element.tab_group())
        .when(node.id % 4 == 1, |element| {
            element.underline().child("Retained glyphs: abc Ω")
        })
        .child({
            let color = style.border_color;
            let width = style.border_width;
            let bend = style.corner_radius;
            crate::canvas(
                |_, _, _| (),
                move |bounds, (), window, _| {
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
                    work_counters,
                    cached_views,
                    hover_styles,
                )
            } else {
                render_node_body(
                    child,
                    None,
                    scroll_handles,
                    work_counters,
                    cached_views,
                    hover_styles,
                )
            }
        }))
        .into_any_element()
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

    #[derive(Clone, Copy)]
    struct DependencyValue {
        color: u32,
        extent: f32,
    }
    impl crate::Global for DependencyValue {}

    #[derive(Clone, Copy)]
    enum DependencyPhase {
        Render,
        Prepaint,
        Paint,
    }

    #[derive(Clone)]
    struct DependencyLeaf {
        models: [Entity<DependencyValue>; 2],
        secondary: bool,
        phase: DependencyPhase,
        decoration: u8,
        priority: usize,
        anchor: crate::Point<Pixels>,
    }

    impl DependencyLeaf {
        fn value(&self, cx: &crate::App) -> DependencyValue {
            let model = self.models[usize::from(self.secondary)].read(cx);
            let global = cx.global::<DependencyValue>();
            DependencyValue {
                color: model.color ^ global.color,
                extent: (model.extent + global.extent) * 0.5,
            }
        }
    }

    impl Render for DependencyLeaf {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let initial = if matches!(self.phase, DependencyPhase::Render) {
                self.value(cx)
            } else {
                DependencyValue {
                    color: 0,
                    extent: 0.0,
                }
            };
            let prepaint = self.clone();
            let paint = self.clone();
            let body = div()
                .w(px(120.0))
                .h(px(90.0))
                .child(
                    crate::canvas(
                        move |bounds, window, cx| {
                            let value = if matches!(prepaint.phase, DependencyPhase::Prepaint) {
                                prepaint.value(cx)
                            } else {
                                initial
                            };
                            if matches!(prepaint.phase, DependencyPhase::Prepaint) {
                                window.insert_hitbox(
                                    Bounds::new(bounds.origin, size(px(value.extent), px(30.0))),
                                    crate::HitboxBehavior::Normal,
                                );
                            }
                            value
                        },
                        move |bounds, value, window, cx| {
                            let value = if matches!(paint.phase, DependencyPhase::Paint) {
                                paint.value(cx)
                            } else {
                                value
                            };
                            window.paint_quad(crate::fill(
                                Bounds::new(bounds.origin, size(px(value.extent), px(30.0))),
                                crate::rgb(value.color),
                            ));
                        },
                    )
                    .size_full(),
                )
                .into_any_element();
            match self.decoration {
                1 => crate::deferred(body)
                    .with_priority(self.priority)
                    .into_any_element(),
                2 => crate::anchored()
                    .position(self.anchor)
                    .child(body)
                    .into_any_element(),
                3 => crate::deferred(crate::anchored().position(self.anchor).child(body))
                    .with_priority(self.priority)
                    .into_any_element(),
                4 => crate::deferred(
                    div().child(crate::deferred(body).with_priority(5 - self.priority)),
                )
                .with_priority(self.priority)
                .into_any_element(),
                _ => body,
            }
        }
    }

    struct DependencyRoot {
        leaves: Vec<Entity<DependencyLeaf>>,
        hidden: Option<crate::EntityId>,
    }

    impl Render for DependencyRoot {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().flex().size_full().children((0..2).map(|parent| {
                div()
                    .id(("dependency-parent", parent))
                    .flex()
                    .flex_col()
                    .w(px(400.0))
                    .h(px(500.0))
                    .children(self.leaves.iter().enumerate().filter_map(|(index, leaf)| {
                        (index % 2 == parent && self.hidden != Some(leaf.entity_id()))
                            .then(|| leaf.clone())
                    }))
            }))
        }
    }

    struct DependencyFixture {
        window: crate::WindowHandle<DependencyRoot>,
        models: [Entity<DependencyValue>; 2],
        leaves: Vec<Entity<DependencyLeaf>>,
        identities: HashMap<crate::EntityId, String>,
    }

    impl DependencyFixture {
        fn new(cx: &mut TestAppContext, retained: bool, fresh: bool) -> Self {
            let models = [0, 1].map(|index| {
                cx.new(|_| DependencyValue {
                    color: 0xff00 + index,
                    extent: 40.0,
                })
            });
            let leaves = (0..6)
                .map(|index| {
                    cx.new(|_| DependencyLeaf {
                        models: models.clone(),
                        secondary: index % 2 == 1,
                        phase: [
                            DependencyPhase::Render,
                            DependencyPhase::Prepaint,
                            DependencyPhase::Paint,
                        ][index % 3],
                        decoration: 0,
                        priority: index % 6,
                        anchor: crate::point(px(index as f32 * 30.0), px(index as f32 * 20.0)),
                    })
                })
                .collect::<Vec<_>>();
            let mut identities = leaves
                .iter()
                .enumerate()
                .map(|(index, leaf)| (leaf.entity_id(), format!("leaf-{index}")))
                .collect::<HashMap<_, _>>();
            let root_leaves = leaves.clone();
            let window = cx.open_window(WINDOW_SIZE, move |window, _| {
                window.set_view_retention(retained);
                window.set_force_view_reconstruction(fresh);
                if fresh {
                    window.layout_engine = Some(crate::TaffyLayoutEngine::new());
                }
                DependencyRoot {
                    leaves: root_leaves,
                    hidden: None,
                }
            });
            identities.insert(
                window.root(cx).expect("dependency root").entity_id(),
                "root".into(),
            );
            Self {
                window,
                models,
                leaves,
                identities,
            }
        }

        fn capture(&self, cx: &mut TestAppContext) -> crate::ViewReuseSnapshot {
            cx.update_window(self.window.into(), |_, window, cx| {
                window.draw(cx).clear(cx);
                window.view_reuse_snapshot(|id| {
                    self.identities
                        .get(&id)
                        .cloned()
                        .expect("dependency fixture view")
                })
            })
            .expect("dependency window")
        }
    }

    struct VirtualListRoot {
        rows: Entity<Vec<DependencyValue>>,
        uniform: crate::UniformListScrollHandle,
        variable: crate::ListState,
    }

    fn virtual_row(index: usize, row: DependencyValue, uniform: bool) -> AnyElement {
        div()
            .id(index)
            .w_full()
            .h(px(if uniform { 24.0 } else { row.extent }))
            .bg(crate::rgb(row.color))
            .cursor_pointer()
            .focusable()
            .on_click(|_, _, _| {})
            .child(format!("Row {index}"))
            .into_any_element()
    }

    impl Render for VirtualListRoot {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let count = self.rows.read(cx).len();
            let uniform_rows = self.rows.clone();
            let variable_rows = self.rows.clone();
            div()
                .flex()
                .size_full()
                .child(
                    crate::uniform_list("uniform-rows", count, move |range, _, cx| {
                        range
                            .map(|index| virtual_row(index, uniform_rows.read(cx)[index], true))
                            .collect()
                    })
                    .track_scroll(&self.uniform)
                    .w(px(300.0))
                    .h(px(200.0)),
                )
                .child(
                    crate::list(self.variable.clone(), move |index, _, cx| {
                        virtual_row(index, variable_rows.read(cx)[index], false)
                    })
                    .w(px(300.0))
                    .h(px(200.0)),
                )
        }
    }

    fn virtual_list_windows(cx: &mut TestAppContext) -> [crate::WindowHandle<VirtualListRoot>; 2] {
        [false, true].map(|fresh| {
            cx.open_window(WINDOW_SIZE, move |window, cx| {
                window.set_view_retention(!fresh);
                window.set_force_view_reconstruction(fresh);
                VirtualListRoot {
                    rows: cx.new(|_| {
                        (0..64)
                            .map(|index| DependencyValue {
                                color: 0x103050 + index as u32 * 1024,
                                extent: 18.0 + (index % 4) as f32 * 8.0,
                            })
                            .collect()
                    }),
                    uniform: crate::UniformListScrollHandle::new(),
                    variable: crate::ListState::new(64, crate::ListAlignment::Top, px(20.0)),
                }
            })
        })
    }

    #[gpui::test]
    fn uniform_list_scroll_to_item_without_notify(cx: &mut TestAppContext) {
        virtual_list_scroll_regression(cx, true);
    }

    #[gpui::test]
    fn variable_list_scroll_to_item_without_notify(cx: &mut TestAppContext) {
        virtual_list_scroll_regression(cx, false);
    }

    fn virtual_list_scroll_regression(cx: &mut TestAppContext, uniform: bool) {
        let windows = virtual_list_windows(cx);
        for frame in 0..4 {
            let snapshots = windows.map(|window| {
                let test_window = cx.test_window(window.into());
                let wakes = test_window.frame_wake_count();
                if frame == 1 {
                    let root = window.root(cx).expect("virtual list root");
                    root.read_with(cx, |root, _| {
                        if uniform {
                            root.uniform.scroll_to_item(32, crate::ScrollStrategy::Top);
                        } else {
                            root.variable.scroll_to(crate::ListOffset {
                                item_ix: 32,
                                offset_in_item: px(0.0),
                            });
                        }
                    });
                }
                let snapshot = cx
                    .update_window(window.into(), |_, window, cx| {
                        window.draw(cx).clear(cx);
                        window.view_reuse_snapshot(|_| "root".into())
                    })
                    .expect("virtual list window");
                if frame > 0 {
                    assert_eq!(
                        test_window.frame_wake_count(),
                        wakes,
                        "scroll versions and probe reconciliation must not request frames"
                    );
                }
                snapshot
            });
            snapshots[0]
                .compare(&snapshots[1], frame)
                .expect("scroll to item matches");
        }
        for window in windows {
            cx.update_window(window.into(), |_, window, _| window.remove_window())
                .expect("virtual list window");
        }
    }

    #[gpui::test(iterations = 2)]
    fn randomized_virtual_lists_match_forced_reconstruction(
        mut rng: StdRng,
        cx: &mut TestAppContext,
    ) {
        let windows = virtual_list_windows(cx);
        let roots = windows.map(|window| window.root(cx).expect("virtual list root"));
        for frame in 0..24 {
            let index = rng.random_range(0..48);
            let color = rng.random_range(0..=0xffffff);
            let height = rng.random_range(1.0..80.0);
            for root in &roots {
                root.update(cx, |root, cx| match frame % 8 {
                    1 | 5 => root.uniform.scroll_to_item(
                        index,
                        [
                            crate::ScrollStrategy::Top,
                            crate::ScrollStrategy::Center,
                            crate::ScrollStrategy::Bottom,
                            crate::ScrollStrategy::Nearest,
                        ][frame % 4],
                    ),
                    2 | 6 => root.variable.scroll_to(crate::ListOffset {
                        item_ix: index,
                        offset_in_item: px(3.0),
                    }),
                    3 => {
                        root.rows.update(cx, |rows, cx| {
                            rows[index] = DependencyValue {
                                color,
                                extent: height,
                            };
                            cx.notify();
                        });
                        root.variable.reset(root.rows.read(cx).len());
                    }
                    4 => {
                        root.rows.update(cx, |rows, cx| {
                            rows.push(DependencyValue {
                                color,
                                extent: height,
                            });
                            cx.notify();
                        });
                        root.variable.splice(64..64, 1);
                    }
                    _ => {}
                });
            }
            let snapshots = windows.map(|window| {
                cx.update_window(window.into(), |_, window, cx| {
                    window.draw(cx).clear(cx);
                    window.view_reuse_snapshot(|_| "root".into())
                })
                .expect("virtual list window")
            });
            snapshots[0]
                .compare(&snapshots[1], frame)
                .expect("virtual lists match reconstruction");
        }
        for window in windows {
            cx.update_window(window.into(), |_, window, _| window.remove_window())
                .expect("virtual list window");
        }
    }

    #[gpui::test]
    fn unnotified_model_updates_under_notified_root(cx: &mut TestAppContext) {
        cx.set_global(DependencyValue {
            color: 0x00ff00,
            extent: 40.0,
        });
        let reused = DependencyFixture::new(cx, true, false);
        let fresh = DependencyFixture::new(cx, false, true);
        for frame in 0..3 {
            reused
                .capture(cx)
                .compare(&fresh.capture(cx), frame)
                .expect("initial dependency frame");
        }
        cx.update_window(reused.window.into(), |_, window, _| {
            assert!(
                window.view_reuse_stats().cached_hits > 0,
                "fixture must reuse views"
            );
        })
        .expect("dependency stats");
        let unchanged_scene = reused.capture(cx).scene;
        for fixture in [&reused, &fresh] {
            fixture.models[0].update(cx, |model, _| {
                model.color = 0x123456;
                model.extent = 100.0;
            });
        }
        // Raw entity writes do not invalidate readers without a notified ancestor.
        // The parity guarantee starts when the owning subtree requests a rebuild.
        assert_eq!(reused.capture(cx).scene, unchanged_scene);
        assert_ne!(fresh.capture(cx).scene, unchanged_scene);
        for fixture in [&reused, &fresh] {
            fixture
                .window
                .update(cx, |_, _, cx| cx.notify())
                .expect("notify model owner");
        }
        reused
            .capture(cx)
            .compare(&fresh.capture(cx), 3)
            .expect("notified ancestor reconstructs model readers");
        for fixture in [reused, fresh] {
            cx.update_window(fixture.window.into(), |_, window, _| window.remove_window())
                .expect("dependency window");
        }
    }

    #[gpui::test(iterations = 2)]
    fn randomized_phase_dependencies_match_forced_reconstruction(
        mut rng: StdRng,
        cx: &mut TestAppContext,
    ) {
        for (retained, decorated) in [false, true]
            .into_iter()
            .flat_map(|retained| [false, true].map(|decorated| (retained, decorated)))
        {
            cx.set_global(DependencyValue {
                color: 0x00ff00,
                extent: 40.0,
            });
            let reused = DependencyFixture::new(cx, retained, false);
            let fresh = DependencyFixture::new(cx, false, true);
            if decorated {
                for fixture in [&reused, &fresh] {
                    for (index, leaf) in fixture.leaves.iter().enumerate() {
                        leaf.update(cx, |leaf, cx| {
                            leaf.decoration = (index % 5) as u8;
                            cx.notify();
                        });
                    }
                }
            }
            for frame in 0..48 {
                let value = DependencyValue {
                    color: rng.random_range(0..=0xffffff),
                    extent: rng.random_range(0.0..120.0),
                };
                match frame % 12 {
                    1 | 2 => {
                        let model_index = frame % 2;
                        for fixture in [&reused, &fresh] {
                            fixture.models[model_index].update(cx, |model, cx| {
                                *model = value;
                                cx.notify();
                            });
                        }
                    }
                    3 => cx.set_global(value),
                    4 => cx.update_global::<DependencyValue, _>(|global, _| *global = value),
                    5 => {
                        let leaf_index = rng.random_range(0..reused.leaves.len());
                        for fixture in [&reused, &fresh] {
                            fixture.leaves[leaf_index].update(cx, |leaf, cx| {
                                leaf.secondary = !leaf.secondary;
                                cx.notify();
                            });
                        }
                    }
                    6 => {
                        for fixture in [&reused, &fresh] {
                            fixture.models[0].update(cx, |model, _| *model = value);
                            fixture
                                .window
                                .update(cx, |_, _, cx| cx.notify())
                                .expect("notify model owner");
                        }
                    }
                    7 => {
                        for fixture in [&reused, &fresh] {
                            fixture
                                .window
                                .update(cx, |root, _, cx| {
                                    root.leaves.rotate_left(1);
                                    cx.notify();
                                })
                                .expect("reparent dependency views");
                        }
                    }
                    8 | 9 => {
                        for fixture in [&reused, &fresh] {
                            fixture
                                .window
                                .update(cx, |root, _, cx| {
                                    root.hidden =
                                        (frame % 12 == 8).then(|| fixture.leaves[0].entity_id());
                                    cx.notify();
                                })
                                .expect("toggle dependency view");
                        }
                    }
                    10 if decorated => {
                        let index = rng.random_range(0..reused.leaves.len());
                        let priority = rng.random_range(0..6);
                        let anchor = crate::point(
                            px(rng.random_range(-40.0..1250.0)),
                            px(rng.random_range(-40.0..950.0)),
                        );
                        for fixture in [&reused, &fresh] {
                            fixture.leaves[index].update(cx, |leaf, cx| {
                                leaf.decoration = (leaf.decoration + 1) % 5;
                                leaf.priority = priority;
                                leaf.anchor = anchor;
                                cx.notify();
                            });
                        }
                    }
                    _ => {}
                }
                let reused_snapshot = reused.capture(cx);
                let fresh_snapshot = fresh.capture(cx);
                reused_snapshot
                    .compare(&fresh_snapshot, frame)
                    .unwrap_or_else(|error| panic!(
                        "phase dependencies retained={retained} decorated={decorated} frame={frame}: {error}"
                    ));
            }
            for fixture in [reused, fresh] {
                cx.update_window(fixture.window.into(), |_, window, _| window.remove_window())
                    .expect("dependency window");
            }
        }
    }

    #[gpui::test]
    fn fixed_leaf_recolor_is_stationary(cx: &mut TestAppContext) {
        for cached in [false, true] {
            let config = RandomizedElementTreeConfig::new(0, ELEMENT_COUNT)
                .with_cached_views(cached)
                .with_topology(RandomizedElementTreeTopology::Mixed)
                .with_entity_density(0.5);
            let tree = cx.new(|cx| RandomizedElementTree::new_with_config(config, cx));
            let target = tree.update(cx, |tree, cx| tree.recolor_first_leaf(cx));
            assert!(target.is_some());
            let first = tree.read_with(cx, |tree, _| tree.snapshot());
            assert_eq!(first.descendant_count(), ELEMENT_COUNT);
            assert_eq!(
                target,
                tree.update(cx, |tree, cx| tree.recolor_first_leaf(cx))
            );
            let second = tree.read_with(cx, |tree, _| tree.snapshot());
            assert_ne!(first, second);
            for frame in 0..1000 {
                assert_eq!(
                    target,
                    tree.update(cx, |tree, cx| tree.recolor_first_leaf(cx))
                );
                assert_eq!(
                    tree.read_with(cx, |tree, _| tree.snapshot()),
                    if frame % 2 == 0 {
                        first.clone()
                    } else {
                        second.clone()
                    }
                );
            }
        }
    }

    #[gpui::test(iterations = 2)]
    fn cached_views_match_forced_reconstruction(rng: StdRng, cx: &mut TestAppContext) {
        views_match_forced_reconstruction(rng, cx, true, false, false, false);
    }

    #[gpui::test(iterations = 2)]
    fn implicit_views_match_forced_reconstruction(rng: StdRng, cx: &mut TestAppContext) {
        views_match_forced_reconstruction(rng, cx, false, false, false, false);
    }

    #[gpui::test(iterations = 2)]
    fn all_mutations_match_forced_reconstruction(rng: StdRng, cx: &mut TestAppContext) {
        views_match_forced_reconstruction(rng, cx, true, true, false, false);
    }

    #[gpui::test(iterations = 2)]
    fn scroll_offsets_match_forced_reconstruction(rng: StdRng, cx: &mut TestAppContext) {
        views_match_forced_reconstruction(rng, cx, true, false, true, false);
    }

    // Cached group-hover listeners retain an ancestor hitbox from an older frame.
    #[gpui::test(iterations = 2)]
    fn hover_styles_match_forced_reconstruction(rng: StdRng, cx: &mut TestAppContext) {
        views_match_forced_reconstruction(rng, cx, true, false, false, true);
    }

    // Redrawing a group replaces the ancestor hitbox captured by a cached child.
    #[gpui::test]
    fn cached_group_hover_survives_parent_redraw(cx: &mut TestAppContext) {
        struct Leaf;
        impl Render for Leaf {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .id("hover-leaf")
                    .size_full()
                    .bg(crate::rgb(0xff0000))
                    .group_hover("hover-root", |style| style.bg(crate::rgb(0x0000ff)))
                    .cursor_pointer()
            }
        }
        struct Root(Entity<Leaf>);
        impl Render for Root {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .id("hover-root")
                    .group("hover-root")
                    .w(px(200.0))
                    .h(px(200.0))
                    .child(
                        self.0
                            .clone()
                            .cached(crate::StyleRefinement::default().size(px(100.0))),
                    )
            }
        }
        for retained in [false, true] {
            let windows = [false, true].map(|fresh| {
                cx.open_window(WINDOW_SIZE, |window, cx| {
                    window.set_view_retention(retained && !fresh);
                    window.set_force_view_reconstruction(fresh);
                    Root(cx.new(|_| Leaf))
                })
            });
            for (frame, inside) in [false, false, true, true, false, false, true]
                .into_iter()
                .enumerate()
            {
                let position = crate::point(px(if inside { 10.0 } else { 500.0 }), px(10.0));
                let snapshots = windows.map(|window| {
                    cx.update_window(window.into(), |_, window, cx| {
                        window.simulate_mouse_move(position, cx);
                    })
                    .expect("hover input");
                    cx.update_window(window.into(), |_, window, cx| {
                        window.draw(cx).clear(cx);
                        window.view_reuse_snapshot(|_| "hover-view".into())
                    })
                    .expect("hover frame")
                });
                snapshots[0]
                    .compare(&snapshots[1], frame)
                    .expect("group hover matches");
            }
            for window in windows {
                cx.update_window(window.into(), |_, window, _| window.remove_window())
                    .expect("hover window");
            }
        }
    }

    #[gpui::test]
    fn cached_descendants_inherit_changed_visibility(cx: &mut TestAppContext) {
        for retention in [false, true] {
            inherited_scope_regression(cx, false, retention);
        }
    }

    #[gpui::test]
    fn cached_scroll_offset_without_notify(cx: &mut TestAppContext) {
        for retention in [false, true] {
            inherited_scope_regression(cx, true, retention);
        }
    }

    #[gpui::test]
    fn cached_group_scope_tracks_anonymous_siblings(cx: &mut TestAppContext) {
        struct Leaf;
        impl Render for Leaf {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .id("leaf")
                    .size_full()
                    .bg(crate::rgb(0xff0000))
                    .group_hover("shared", |style| style.bg(crate::rgb(0x0000ff)))
                    .group_active("shared", |style| style.bg(crate::rgb(0x00ff00)))
            }
        }
        struct Root {
            leaves: [Entity<Leaf>; 2],
            insert_empty: bool,
            nested: bool,
            hidden: bool,
        }
        impl Render for Root {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .id("groups")
                    .flex()
                    .flex_col()
                    .w(px(200.0))
                    .h(px(400.0))
                    .when(self.insert_empty, |element| {
                        element.child(div().group("shared").w(px(200.0)).h(px(0.0)))
                    })
                    .children(self.leaves.iter().enumerate().filter_map(|(index, leaf)| {
                        if self.hidden && index == 0 {
                            return None;
                        }
                        let child = leaf
                            .clone()
                            .cached(crate::StyleRefinement::default().size(px(100.0)))
                            .into_any_element();
                        let child = if self.nested && index == 0 {
                            div()
                                .group("shared")
                                .w(px(100.0))
                                .h(px(100.0))
                                .child(child)
                                .into_any_element()
                        } else {
                            child
                        };
                        Some(div().group("shared").w(px(200.0)).h(px(100.0)).child(child))
                    }))
            }
        }
        for retained in [false, true] {
            let windows = [false, true].map(|fresh| {
                cx.open_window(WINDOW_SIZE, |window, cx| {
                    window.set_view_retention(retained && !fresh);
                    window.set_force_view_reconstruction(fresh);
                    Root {
                        leaves: [cx.new(|_| Leaf), cx.new(|_| Leaf)],
                        insert_empty: false,
                        nested: false,
                        hidden: false,
                    }
                })
            });
            for frame in 0..14 {
                let position = match frame {
                    2 | 5 | 7 | 9 | 11 | 13 => crate::point(px(10.0), px(10.0)),
                    3 | 4 => crate::point(px(10.0), px(110.0)),
                    8 => crate::point(px(150.0), px(10.0)),
                    _ => crate::point(px(500.0), px(10.0)),
                };
                let snapshots = windows.map(|window| {
                    if matches!(frame, 4 | 6 | 8 | 10) {
                        window
                            .update(cx, |root, _, cx| {
                                root.insert_empty = frame == 4;
                                root.nested = frame >= 8;
                                root.hidden = frame >= 10;
                                cx.notify();
                            })
                            .expect("anonymous group mutation");
                    }
                    cx.update_window(window.into(), |_, window, cx| {
                        window.simulate_mouse_move(position, cx);
                        if frame == 9 {
                            window.dispatch_event(
                                crate::PlatformInput::MouseDown(crate::MouseDownEvent {
                                    position,
                                    button: crate::MouseButton::Left,
                                    modifiers: Default::default(),
                                    click_count: 1,
                                    first_mouse: false,
                                }),
                                cx,
                            );
                        } else if frame == 10 {
                            window.dispatch_event(
                                crate::PlatformInput::MouseUp(crate::MouseUpEvent {
                                    position,
                                    button: crate::MouseButton::Left,
                                    modifiers: Default::default(),
                                    click_count: 1,
                                }),
                                cx,
                            );
                        }
                    })
                    .expect("anonymous group input");
                    let root = window.root(cx).expect("anonymous group root");
                    let mut identities = root.read_with(cx, |root, _| {
                        root.leaves
                            .iter()
                            .enumerate()
                            .map(|(index, leaf)| (leaf.entity_id(), format!("leaf-{index}")))
                            .collect::<HashMap<_, _>>()
                    });
                    identities.insert(root.entity_id(), "root".into());
                    cx.update_window(window.into(), |_, window, cx| {
                        window.draw(cx).clear(cx);
                        window.view_reuse_snapshot(|id| identities[&id].clone())
                    })
                    .expect("anonymous group frame")
                });
                snapshots[0]
                    .compare(&snapshots[1], frame)
                    .expect("anonymous group scope matches");
            }
            for window in windows {
                cx.update_window(window.into(), |_, window, _| window.remove_window())
                    .expect("anonymous group window");
            }
        }
    }

    fn inherited_scope_regression(cx: &mut TestAppContext, scroll: bool, retention: bool) {
        let config = RandomizedElementTreeConfig::new(0, 2)
            .with_topology(RandomizedElementTreeTopology::Deep)
            .with_entity_density(1.0)
            .with_handler_density(1.0)
            .with_cached_views(true);
        let windows = [false, true].map(|fresh| {
            cx.open_window(WINDOW_SIZE, move |window, cx| {
                window.set_view_retention(retention);
                window.set_force_view_reconstruction(fresh);
                let tree = RandomizedElementTree::new_with_config(config, cx);
                {
                    let mut snapshot = tree.snapshot.borrow_mut();
                    snapshot.root.visible = true;
                    for path in [vec![0], vec![0, 0]] {
                        let node = node_at_path_mut(&mut snapshot.children, &path)
                            .expect("two-node deep tree");
                        node.style.visible = true;
                        node.style.width = if path.len() == 1 { 100.0 } else { 300.0 };
                        node.style.height = if path.len() == 1 { 100.0 } else { 300.0 };
                        node.style.padding = 0.0;
                        node.style.border_width = 0.0;
                        node.style.wrap = false;
                    }
                }
                tree
            })
        });
        let roots = windows.map(|window| window.root(cx).expect("scope root"));
        for frame in 0..4 {
            let mut snapshots = Vec::new();
            for (window, root) in windows.iter().zip(&roots) {
                if frame > 0 {
                    root.update(cx, |tree, cx| {
                        if scroll {
                            tree.scroll_handles
                                .borrow()
                                .get(&0)
                                .expect("scroll handle mounted")
                                .set_offset(crate::point(px(-10.0), px(-10.0)));
                        } else {
                            tree.snapshot.borrow_mut().root.visible = frame % 2 == 0;
                            cx.notify();
                        }
                    });
                }
                snapshots.push(
                    cx.update_window((*window).into(), |_, window, cx| {
                        window.draw(cx).clear(cx);
                        window.view_reuse_snapshot(|id| {
                            root.read(cx)
                                .view_element_identity(id)
                                .map(|id| format!("element-{id}"))
                                .unwrap_or_else(|| "root".into())
                        })
                    })
                    .expect("scope window"),
                );
            }
            snapshots[0]
                .compare(&snapshots[1], frame)
                .expect("inherited scope matches reconstruction");
        }
        for window in windows {
            cx.update_window(window.into(), |_, window, _| window.remove_window())
                .expect("scope window");
        }
    }

    #[gpui::test]
    fn cached_descendants_inherit_changed_opacity(cx: &mut TestAppContext) {
        for splice in [false, true] {
            let config = RandomizedElementTreeConfig::new(0, 1)
                .with_topology(RandomizedElementTreeTopology::Wide)
                .with_entity_density(1.0)
                .with_cached_views(true);
            let window = cx.open_window(WINDOW_SIZE, move |window, cx| {
                window.set_view_retention(splice);
                let tree = RandomizedElementTree::new_with_config(config, cx);
                {
                    let mut snapshot = tree.snapshot.borrow_mut();
                    snapshot.root.visible = true;
                    snapshot.root.opacity = 1.0;
                    for node in &mut snapshot.children {
                        node.style.visible = true;
                        node.style.opacity = 1.0;
                    }
                }
                tree
            });
            for opacity in [1.0, 0.5, 0.0, 1.0] {
                window
                    .update(cx, |tree, _, cx| {
                        tree.snapshot.borrow_mut().root.opacity = opacity;
                        cx.notify();
                    })
                    .expect("opacity window");
                cx.update_window(window.into(), |_, window, cx| {
                    window.draw(cx).clear(cx);
                    for quad in window
                        .rendered_frame
                        .scene
                        .quads
                        .iter()
                        .filter(|quad| quad.background.solid.a > 0.0)
                    {
                        assert_eq!(quad.background.solid.a, opacity);
                    }
                })
                .expect("opacity window");
            }
            window
                .update(cx, |_, window, _| window.remove_window())
                .expect("opacity window");
        }
    }

    fn views_match_forced_reconstruction(
        mut rng: StdRng,
        cx: &mut TestAppContext,
        cached: bool,
        all_mutations: bool,
        scroll: bool,
        mouse: bool,
    ) {
        for (topology, splice) in
            RandomizedElementTreeTopology::ALL
                .into_iter()
                .flat_map(|topology| {
                    [false, true]
                        .into_iter()
                        .filter(move |splice| cached || *splice)
                        .map(move |splice| (topology, splice))
                })
        {
            let mut config = RandomizedElementTreeConfig::new(rng.random(), ELEMENT_COUNT)
                .with_topology(topology)
                .with_entity_density(0.5)
                .with_handler_density(0.5)
                .with_cached_views(cached);
            config.hover_styles = mouse;
            let reused = cx.open_window(WINDOW_SIZE, move |window, cx| {
                if cached {
                    window.set_view_retention(splice);
                } else {
                    window.set_view_retention(true);
                }
                RandomizedElementTree::new_with_config(config, cx)
            });
            let reconstructed = cx.open_window(WINDOW_SIZE, move |window, cx| {
                window.set_view_retention(false);
                window.layout_engine = Some(crate::TaffyLayoutEngine::new());
                window.set_force_view_reconstruction(true);
                RandomizedElementTree::new_with_config(config, cx)
            });
            cx.update_window(reused.into(), |_, window, _| {
                window.set_force_view_reconstruction(false);
            })
            .expect("reused window exists");
            let reused_root = reused.root(cx).expect("reused root exists");
            let reconstructed_root = reconstructed.root(cx).expect("reconstructed root exists");
            let frame_count = if all_mutations {
                RandomizedElementTreeMutationKind::ALL.len() * 2 + 1
            } else {
                16
            };
            let mut pointer = crate::point(px(20.0), px(20.0));
            for frame in 0..frame_count {
                if mouse && frame % 4 == 1 {
                    pointer = crate::point(
                        px(rng.random_range(0.0..250.0)),
                        px(rng.random_range(0.0..250.0)),
                    );
                } else if mouse && frame % 4 == 3 {
                    pointer = crate::point(px(1300.0), px(950.0));
                }
                if frame > 0 {
                    for root in [&reused_root, &reconstructed_root] {
                        root.update(cx, |tree, cx| {
                            if scroll {
                                tree.scroll_without_notify();
                                return;
                            }
                            if mouse {
                                return;
                            }
                            let mutation = if all_mutations {
                                Some(
                                    RandomizedElementTreeMutationKind::ALL[(frame - 1)
                                        % RandomizedElementTreeMutationKind::ALL.len()],
                                )
                            } else {
                                match frame % 7 {
                                    1 => Some(RandomizedElementTreeMutationKind::ChildColor),
                                    2 => Some(RandomizedElementTreeMutationKind::ChildBounds),
                                    3 => Some(RandomizedElementTreeMutationKind::Insert),
                                    4 => Some(RandomizedElementTreeMutationKind::RemoveLeaf),
                                    5 => Some(RandomizedElementTreeMutationKind::Reorder),
                                    _ => None,
                                }
                            };
                            if let Some(mutation) = mutation {
                                tree.apply_mutation(mutation, cx);
                            } else {
                                cx.notify();
                            }
                        });
                    }
                }
                let capture =
                    |root: &Entity<RandomizedElementTree>, window, cx: &mut TestAppContext| {
                        if mouse {
                            cx.update_window(window, |_, window, cx| {
                                window.simulate_mouse_move(pointer, cx);
                            })
                            .expect("mouse input window");
                        }
                        let mut identities = root.read_with(cx, |tree, _| {
                            tree.entities
                                .borrow()
                                .iter()
                                .map(|(id, entity)| (entity.entity_id(), format!("element-{id}")))
                                .collect::<HashMap<_, _>>()
                        });
                        identities.insert(root.entity_id(), "root".into());
                        cx.update_window(window, |_, window, cx| {
                            if !mouse && frame % 4 == 1 {
                                window.focus_next(cx);
                            } else if !mouse && frame % 4 == 3 {
                                window.focus_prev(cx);
                            }
                            let arena_clear_needed = window.draw(cx);
                            let snapshot = window.view_reuse_snapshot(|entity_id| {
                                identities
                                    .get(&entity_id)
                                    .cloned()
                                    .expect("fixture owns every view")
                            });
                            arena_clear_needed.clear(cx);
                            snapshot
                        })
                        .expect("fixture window exists")
                    };
                let reused_snapshot = capture(&reused_root, reused.into(), cx);
                let reconstructed_snapshot = capture(&reconstructed_root, reconstructed.into(), cx);
                reused_snapshot
                    .compare(&reconstructed_snapshot, frame)
                    .unwrap_or_else(|error| {
                        panic!("cached={cached} retention={splice} topology={topology:?}: {error}")
                    });
            }
            let stats = cx
                .update_window(reused.into(), |_, window, _| window.take_view_reuse_stats())
                .expect("reused window exists");
            if cached
                || cx
                    .update_window(reused.into(), |_, window, _| window.layout_cache_enabled())
                    .expect("window")
            {
                assert!(stats.cached_hits > 0, "fixture exercises reuse");
                assert!(
                    stats.prepaint_reuse_calls > 0,
                    "fixture replays prepaint output"
                );
            }
            cx.update_window(reused.into(), |_, window, _| window.remove_window())
                .expect("reused window exists");
            cx.update_window(reconstructed.into(), |_, window, _| window.remove_window())
                .expect("reconstructed window exists");
        }
    }

    #[derive(Debug, PartialEq)]
    struct SceneSnapshot {
        quads: Vec<QuadSnapshot>,
        element_ids: BTreeSet<u64>,
    }

    impl SceneSnapshot {
        fn from_window(window: &Window) -> Self {
            Self {
                quads: window
                    .rendered_frame
                    .scene
                    .quads
                    .iter()
                    .map(QuadSnapshot::from)
                    .collect(),
                element_ids: window
                    .rendered_frame
                    .element_states
                    .keys()
                    .filter_map(|(handle, _)| {
                        let global_id = window.element_id_interner.global_id(*handle);
                        let ElementId::NamedInteger(name, element_id) = global_id.last()? else {
                            return None;
                        };
                        (name.as_ref() == "randomized-element").then_some(*element_id)
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
        let retained = cx
            .update_window(window, |_, window, _| window.view_splicing)
            .expect("window");
        if retained {
            assert_eq!(incremental_counters.root_render_count(), 0);
            assert_eq!(incremental_counters.entity_render_count(), 1);
        } else {
            assert!(incremental_counters.root_render_count() >= 1);
        }
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
        let expected_element_ids = snapshot.element_ids().into_iter().collect::<BTreeSet<_>>();
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
