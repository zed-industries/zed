use slotmap::{DefaultKey, SecondaryMap, SlotMap};
use taffy::{
    CacheTree, RoundTree, TraversePartialTree, TraverseTree,
    style::AvailableSpace as TaffyAvailableSpace,
    tree::{Cache, Layout, LayoutOutput, NodeId, RunMode},
};

/// Layout information for a given node.
#[derive(Debug, PartialEq)]
pub(crate) struct NodeData {
    pub(crate) style: taffy::Style,
    pub(crate) unrounded_layout: Layout,
    pub(crate) final_layout: Layout,
    pub(crate) cache: Cache,
    pub(crate) has_context: bool,
    pub(crate) layout_generation: u64,
}

impl NodeData {
    pub fn new(style: taffy::Style) -> Self {
        Self {
            style,
            cache: Cache::new(),
            unrounded_layout: Layout::new(),
            final_layout: Layout::new(),
            has_context: false,
            layout_generation: 0,
        }
    }
}

pub struct TaffyTree<NodeContext = ()> {
    pub(crate) nodes: SlotMap<DefaultKey, NodeData>,
    pub(crate) node_context_data: SecondaryMap<DefaultKey, NodeContext>,
    pub(crate) children: SecondaryMap<DefaultKey, Vec<NodeId>>,
    pub(crate) parents: SecondaryMap<DefaultKey, Option<NodeId>>,
}

impl<NodeContext> TaffyTree<NodeContext> {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::with_capacity(16),
            node_context_data: SecondaryMap::with_capacity(16),
            children: SecondaryMap::with_capacity(16),
            parents: SecondaryMap::with_capacity(16),
        }
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.node_context_data.clear();
        self.children.clear();
        self.parents.clear();
    }

    pub fn new_leaf_with_context(&mut self, style: taffy::Style, context: NodeContext) -> NodeId {
        let mut data = NodeData::new(style);
        data.has_context = true;
        let id_key = self.nodes.insert(data);
        self.node_context_data.insert(id_key, context);
        self.children.insert(id_key, Vec::new());
        self.parents.insert(id_key, None);
        default_key_to_node_id(id_key)
    }

    pub fn new_with_children(&mut self, style: taffy::Style, children: &[NodeId]) -> NodeId {
        let id_key = self.nodes.insert(NodeData::new(style));
        let id = default_key_to_node_id(id_key);
        for child in children {
            self.parents
                .insert(node_id_to_default_key(*child), Some(id));
        }
        self.children.insert(id_key, children.to_vec());
        self.parents.insert(id_key, None);
        id
    }

    pub fn parent(&self, id: NodeId) -> Option<NodeId> {
        self.parents
            .get(node_id_to_default_key(id))
            .copied()
            .flatten()
    }

    pub fn layout(&self, id: NodeId) -> &Layout {
        &self.nodes[node_id_to_default_key(id)].final_layout
    }
}

pub(crate) fn node_id_to_default_key(node_id: NodeId) -> DefaultKey {
    node_id.into()
}

pub(crate) fn default_key_to_node_id(key: DefaultKey) -> NodeId {
    key.into()
}

impl<NodeContext> TraversePartialTree for TaffyTree<NodeContext> {
    type ChildIter<'a>
        = std::iter::Cloned<std::slice::Iter<'a, NodeId>>
    where
        Self: 'a;

    fn child_ids(&self, id: NodeId) -> Self::ChildIter<'_> {
        self.children[node_id_to_default_key(id)].iter().cloned()
    }

    fn child_count(&self, id: NodeId) -> usize {
        self.children[node_id_to_default_key(id)].len()
    }

    fn get_child_id(&self, id: NodeId, index: usize) -> NodeId {
        self.children[node_id_to_default_key(id)][index]
    }
}

impl<NodeContext> TraverseTree for TaffyTree<NodeContext> {}

impl<NodeContext> RoundTree for TaffyTree<NodeContext> {
    fn get_unrounded_layout(&self, id: NodeId) -> Layout {
        self.nodes[node_id_to_default_key(id)].unrounded_layout
    }

    fn set_final_layout(&mut self, id: NodeId, layout: &Layout) {
        self.nodes[node_id_to_default_key(id)].final_layout = *layout;
    }
}

impl<NodeContext> CacheTree for TaffyTree<NodeContext> {
    fn cache_get(
        &self,
        id: NodeId,
        known_dimensions: taffy::geometry::Size<Option<f32>>,
        available_space: taffy::geometry::Size<TaffyAvailableSpace>,
        run_mode: RunMode,
    ) -> Option<LayoutOutput> {
        self.nodes[node_id_to_default_key(id)].cache.get(
            known_dimensions,
            available_space,
            run_mode,
        )
    }

    fn cache_store(
        &mut self,
        id: NodeId,
        known_dimensions: taffy::geometry::Size<Option<f32>>,
        available_space: taffy::geometry::Size<TaffyAvailableSpace>,
        run_mode: RunMode,
        layout_output: LayoutOutput,
    ) {
        self.nodes[node_id_to_default_key(id)].cache.store(
            known_dimensions,
            available_space,
            run_mode,
            layout_output,
        );
    }

    fn cache_clear(&mut self, id: NodeId) {
        self.nodes[node_id_to_default_key(id)].cache.clear();
    }
}
