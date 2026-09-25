use crate::{
    AnyElement, AppContext as _, Context, ElementId, Entity, Hsla, InteractiveElement, IntoElement,
    ParentElement, Render, StatefulInteractiveElement, Styled, Window, div, px,
    util::FluentBuilder,
};
use collections::{HashMap, HashSet};
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

crate::actions!(
    randomized_element_tree,
    [
        /// Dispatched to no one: the action generated elements register handlers for.
        RandomizedElementTreeAction
    ]
);

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
        }
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

/// A persistent root view for randomized GPUI element-tree tests and benchmarks.
pub struct RandomizedElementTree {
    snapshot: Rc<RefCell<RandomizedElementTreeSnapshot>>,
    entities: Rc<RefCell<RandomizedElementEntities>>,
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
            paths_by_element_id: HashMap::default(),
        };
        snapshot.rebuild_element_paths();

        Self {
            snapshot: Rc::new(RefCell::new(snapshot)),
            entities: Rc::new(RefCell::new(HashMap::default())),
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

    fn mutate_random_child(
        &mut self,
        mutation: RandomizedChildMutation,
    ) -> AppliedRandomizedElementTreeMutation {
        let mut snapshot = self.snapshot.borrow_mut();
        let Some(path) = random_node_path(&snapshot.children, &mut self.rng) else {
            drop(snapshot);
            return self.insert_child_internal();
        };
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
                node.style.width = self.rng.random_range(36.0..220.0);
                node.style.height = self.rng.random_range(28.0..180.0);
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
            .children(
                snapshot
                    .children
                    .iter()
                    .map(|node| render_node(node, &entities, &mut work_counters)),
            )
    }
}

type RandomizedElementEntities = HashMap<u64, Entity<RandomizedElementEntity>>;

struct RandomizedElementEntity {
    element_id: u64,
    snapshot: Rc<RefCell<RandomizedElementTreeSnapshot>>,
    entities: Weak<RefCell<RandomizedElementEntities>>,
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
            .map(|node| render_node_body(node, entities.as_deref(), &mut work_counters))
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
    fn random(id: u64, entity_density: f64, handler_density: f64, rng: &mut StdRng) -> Self {
        Self {
            id,
            style: RandomizedElementStyle::random_child(rng),
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
    work_counters: &mut RandomizedElementTreeWorkCounters,
) -> AnyElement {
    if node.is_entity
        && let Some(entity) = entities.get(&node.id)
    {
        return entity.clone().into_any_element();
    }

    render_node_body(node, Some(entities), work_counters)
}

#[cfg_attr(feature = "stacker", stacksafe::stacksafe)]
fn render_node_body(
    node: &RandomizedElementNode,
    entities: Option<&RandomizedElementEntities>,
    work_counters: &mut RandomizedElementTreeWorkCounters,
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
        .when(style.vertical, |element| element.flex_col())
        .when(style.wrap, |element| element.flex_wrap())
        .gap(px(style.gap))
        .p(px(style.padding))
        .w(px(style.width))
        .h(px(style.height))
        .bg(style.background)
        .border(px(style.border_width))
        .border_color(style.border_color)
        .rounded(px(style.corner_radius))
        .opacity(style.opacity)
        .when(style.clipped, |element| element.overflow_hidden())
        .when(!style.visible, |element| element.invisible())
        .when(node.features.action_handler, |element| {
            element.on_action(|_: &RandomizedElementTreeAction, _window, _cx| {})
        })
        .when(node.features.click_handler, |element| {
            element.on_click(|_event, _window, _cx| {})
        })
        .when(node.features.key_context, |element| {
            element.key_context("RandomizedElementTree")
        })
        .children(node.children.iter().map(|child| {
            if let Some(entities) = entities {
                render_node(child, entities, work_counters)
            } else {
                render_node_body(child, None, work_counters)
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
                    .filter_map(|(global_id, _)| {
                        let ElementId::NamedInteger(name, element_id) = global_id.0.last()? else {
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
