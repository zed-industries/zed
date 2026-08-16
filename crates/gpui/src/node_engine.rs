use crate::{
    AnyView, Bounds, EntityId, GlobalElementId, Pixels, ViewNode, ViewNodeCacheKey,
    ViewNodeRecording,
};
use collections::{FxHashMap, FxHashSet};
use slotmap::SlotMap;
#[cfg(test)]
use std::cell::Cell;
use std::rc::Rc;

#[cfg(test)]
thread_local! {
    static FORCE_NODE_ENGINE: Cell<bool> = const { Cell::new(false) };
}

slotmap::new_key_type! {
    pub(crate) struct ViewNodeId;
}

pub(crate) enum NodeRenderDecision {
    Graft {
        node_id: ViewNodeId,
        recording: Rc<ViewNodeRecording>,
        accessed_entities: FxHashSet<EntityId>,
    },
    Render {
        node_id: ViewNodeId,
    },
}

pub(crate) struct NodeEngine {
    nodes: SlotMap<ViewNodeId, ViewNode>,
    occurrences: FxHashMap<GlobalElementId, ViewNodeId>,
    occurrences_by_entity: FxHashMap<EntityId, FxHashSet<ViewNodeId>>,
    dirty_nodes: FxHashSet<ViewNodeId>,
    traversal_stack: Vec<ViewNodeId>,
    roots: Vec<ViewNodeId>,
    next_roots: Vec<ViewNodeId>,
    full_damage: bool,
    damage_bounds: Option<Bounds<Pixels>>,
}

pub(crate) enum DrawEngine {
    Legacy,
    Node(NodeEngine),
}

impl DrawEngine {
    pub(crate) fn from_environment() -> Self {
        #[cfg(test)]
        let forced_for_test = FORCE_NODE_ENGINE.get();
        #[cfg(not(test))]
        let forced_for_test = false;

        if forced_for_test
            || std::env::var("GPUI_EXPERIMENTAL_NODE_ENGINE").is_ok_and(|value| value == "1")
        {
            Self::Node(NodeEngine::new())
        } else {
            Self::Legacy
        }
    }

    #[cfg(test)]
    pub(crate) fn force_node_engine_for_test() -> NodeEngineTestGuard {
        let previous = FORCE_NODE_ENGINE.replace(true);
        NodeEngineTestGuard { previous }
    }
}

#[cfg(test)]
pub(crate) struct NodeEngineTestGuard {
    previous: bool,
}

#[cfg(test)]
impl Drop for NodeEngineTestGuard {
    fn drop(&mut self) {
        FORCE_NODE_ENGINE.set(self.previous);
    }
}

impl NodeEngine {
    pub(crate) fn new() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            occurrences: FxHashMap::default(),
            occurrences_by_entity: FxHashMap::default(),
            dirty_nodes: FxHashSet::default(),
            traversal_stack: Vec::new(),
            roots: Vec::new(),
            next_roots: Vec::new(),
            full_damage: true,
            damage_bounds: None,
        }
    }

    pub(crate) fn begin_frame(&mut self, full_damage: bool) {
        debug_assert!(self.traversal_stack.is_empty());
        self.full_damage = full_damage;
        self.damage_bounds = None;
        self.next_roots.clear();
        if full_damage {
            self.dirty_nodes.extend(self.nodes.keys());
        }
    }

    pub(crate) fn invalidate_entities(&mut self, entities: &FxHashSet<EntityId>) {
        for entity_id in entities {
            if let Some(occurrences) = self.occurrences_by_entity.get(entity_id) {
                self.dirty_nodes.extend(occurrences.iter().copied());
            }
        }
    }

    pub(crate) fn begin_occurrence(
        &mut self,
        occurrence: GlobalElementId,
        view: AnyView,
        cache_key: ViewNodeCacheKey,
    ) -> NodeRenderDecision {
        let parent = self.traversal_stack.last().copied();
        let node_id = if let Some(node_id) = self.occurrences.get(&occurrence).copied() {
            node_id
        } else {
            let previous_bounds = cache_key.bounds;
            let node = ViewNode {
                occurrence: occurrence.clone(),
                parent,
                children: Vec::new(),
                next_children: Vec::new(),
                view,
                cache_key: cache_key.clone(),
                previous_bounds,
                accessed_entities: FxHashSet::default(),
                recording: None,
            };
            let node_id = self.nodes.insert(node);
            self.occurrences.insert(occurrence, node_id);
            self.dirty_nodes.insert(node_id);
            node_id
        };

        if let Some(parent_id) = parent
            && let Some(parent_node) = self.nodes.get_mut(parent_id)
            && !parent_node.next_children.contains(&node_id)
        {
            parent_node.next_children.push(node_id);
        } else if parent.is_none() && !self.next_roots.contains(&node_id) {
            self.next_roots.push(node_id);
        }

        let graft = self.nodes.get(node_id).and_then(|node| {
            (node.cache_key == cache_key
                && !self.dirty_nodes.contains(&node_id)
                && !self.full_damage)
                .then(|| {
                    node.recording
                        .as_ref()
                        .map(|recording| (recording.clone(), node.accessed_entities.clone()))
                })
                .flatten()
        });

        self.traversal_stack.push(node_id);
        if let Some((recording, accessed_entities)) = graft {
            NodeRenderDecision::Graft {
                node_id,
                recording,
                accessed_entities,
            }
        } else {
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.next_children.clear();
            }
            NodeRenderDecision::Render { node_id }
        }
    }

    pub(crate) fn finish_prepaint(&mut self, node_id: ViewNodeId, rendered: bool) {
        if rendered {
            self.reconcile_children(node_id);
        }
        self.pop_traversal(node_id);
    }

    pub(crate) fn store_render(
        &mut self,
        node_id: ViewNodeId,
        cache_key: ViewNodeCacheKey,
        recording: ViewNodeRecording,
        mut accessed_entities: FxHashSet<EntityId>,
    ) {
        let Some(node) = self.nodes.get(node_id) else {
            return;
        };
        let old_bounds = node.previous_bounds;
        let old_dependencies = node.accessed_entities.clone();
        let own_entity_id = node.view.entity_id();
        accessed_entities.insert(own_entity_id);

        for entity_id in old_dependencies {
            if let Some(occurrences) = self.occurrences_by_entity.get_mut(&entity_id) {
                occurrences.remove(&node_id);
                if occurrences.is_empty() {
                    self.occurrences_by_entity.remove(&entity_id);
                }
            }
        }
        for entity_id in &accessed_entities {
            self.occurrences_by_entity
                .entry(*entity_id)
                .or_default()
                .insert(node_id);
        }

        if let Some(node) = self.nodes.get_mut(node_id) {
            node.cache_key = cache_key.clone();
            node.previous_bounds = cache_key.bounds;
            node.accessed_entities = accessed_entities;
            node.recording = Some(Rc::new(recording));
        }

        self.dirty_nodes.remove(&node_id);
        self.extend_damage(old_bounds);
        self.extend_damage(cache_key.bounds);
    }

    pub(crate) fn store_graft(&mut self, node_id: ViewNodeId, recording: ViewNodeRecording) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.recording = Some(Rc::new(recording));
        }
    }

    pub(crate) fn finish_frame(&mut self) -> Option<Bounds<Pixels>> {
        debug_assert!(self.traversal_stack.is_empty());
        let stale_roots = std::mem::replace(&mut self.roots, std::mem::take(&mut self.next_roots));
        let current_roots = self.roots.clone();
        for root_id in stale_roots {
            if !current_roots.contains(&root_id) {
                self.remove_subtree(root_id);
            }
        }
        self.full_damage = false;
        self.damage_bounds.take()
    }

    #[cfg(test)]
    pub(crate) fn clear(&mut self) {
        self.nodes.clear();
        self.occurrences.clear();
        self.occurrences_by_entity.clear();
        self.dirty_nodes.clear();
        self.traversal_stack.clear();
        self.roots.clear();
        self.next_roots.clear();
        self.full_damage = true;
        self.damage_bounds = None;
    }

    fn extend_damage(&mut self, bounds: Bounds<Pixels>) {
        self.damage_bounds = Some(
            self.damage_bounds
                .map(|damage| damage.union(&bounds))
                .unwrap_or(bounds),
        );
    }

    fn pop_traversal(&mut self, node_id: ViewNodeId) {
        let popped = self.traversal_stack.pop();
        debug_assert_eq!(popped, Some(node_id));
    }

    fn reconcile_children(&mut self, node_id: ViewNodeId) {
        let stale_children = if let Some(node) = self.nodes.get_mut(node_id) {
            std::mem::replace(&mut node.children, std::mem::take(&mut node.next_children))
        } else {
            return;
        };
        let current_children = self
            .nodes
            .get(node_id)
            .map(|node| node.children.clone())
            .unwrap_or_default();
        for child_id in &current_children {
            if let Some(child) = self.nodes.get_mut(*child_id)
                && child.parent != Some(node_id)
            {
                child.parent = Some(node_id);
            }
        }
        for child_id in stale_children {
            if !current_children.contains(&child_id) {
                self.remove_subtree(child_id);
            }
        }
    }

    fn remove_subtree(&mut self, node_id: ViewNodeId) {
        let Some(node) = self.nodes.remove(node_id) else {
            return;
        };
        // A removed occurrence's pixels must be repainted even though no node
        // re-records them, so its last bounds join the damage union.
        self.extend_damage(node.previous_bounds);
        for child_id in node.children {
            self.remove_subtree(child_id);
        }
        self.occurrences.remove(&node.occurrence);
        self.dirty_nodes.remove(&node_id);
        for entity_id in node.accessed_entities {
            if let Some(occurrences) = self.occurrences_by_entity.get_mut(&entity_id) {
                occurrences.remove(&node_id);
                if occurrences.is_empty() {
                    self.occurrences_by_entity.remove(&entity_id);
                }
            }
        }
    }
}
