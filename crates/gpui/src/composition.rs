use anyhow::{Result, anyhow, bail};
use collections::FxHashMap;
use slotmap::SlotMap;

slotmap::new_key_type! {
    /// Stable identity for a surface in a window composition tree.
    pub struct CompositionSurfaceId;
}

/// The renderer or platform facility responsible for a composition surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompositionSurfaceKind {
    /// A surface rendered from a range of GPUI scene operations.
    Gpui,
    /// A surface whose content is supplied by a platform-native view or visual.
    Native,
    /// A surface whose content is supplied by an external GPU producer.
    ExternalGpu,
}

#[derive(Clone)]
struct CompositionSurfaceNode {
    kind: CompositionSurfaceKind,
    parent: Option<CompositionSurfaceId>,
    children: Vec<CompositionSurfaceId>,
}

#[derive(Clone)]
pub(crate) struct CompositionTree {
    surfaces: SlotMap<CompositionSurfaceId, CompositionSurfaceNode>,
    roots: Vec<CompositionSurfaceId>,
}

impl CompositionTree {
    pub(crate) fn new() -> Self {
        Self {
            surfaces: SlotMap::with_key(),
            roots: Vec::new(),
        }
    }

    pub(crate) fn insert(
        &mut self,
        kind: CompositionSurfaceKind,
        parent: Option<CompositionSurfaceId>,
    ) -> Result<CompositionSurfaceId> {
        if let Some(parent) = parent {
            self.surface(parent)?;
        }

        let surface = self.surfaces.insert(CompositionSurfaceNode {
            kind,
            parent,
            children: Vec::new(),
        });
        self.siblings_mut(parent)?.push(surface);
        Ok(surface)
    }

    pub(crate) fn remove(&mut self, surface: CompositionSurfaceId) -> Result<()> {
        let parent = self.surface(surface)?.parent;
        let children = self.surface(surface)?.children.clone();
        let siblings = self.siblings_mut(parent)?;
        let index = siblings
            .iter()
            .position(|candidate| *candidate == surface)
            .ok_or_else(|| anyhow!("composition surface is missing from its parent"))?;
        siblings.splice(index..=index, children.iter().copied());
        for child in children {
            self.surface_mut(child)?.parent = parent;
        }
        self.surfaces.remove(surface);
        Ok(())
    }

    pub(crate) fn place_above(
        &mut self,
        surface: CompositionSurfaceId,
        sibling: CompositionSurfaceId,
    ) -> Result<()> {
        self.place_relative(surface, sibling, true)
    }

    pub(crate) fn place_below(
        &mut self,
        surface: CompositionSurfaceId,
        sibling: CompositionSurfaceId,
    ) -> Result<()> {
        self.place_relative(surface, sibling, false)
    }

    pub(crate) fn reparent(
        &mut self,
        surface: CompositionSurfaceId,
        parent: Option<CompositionSurfaceId>,
    ) -> Result<()> {
        self.surface(surface)?;
        if let Some(parent) = parent {
            self.surface(parent)?;
            if parent == surface || self.is_descendant(parent, surface)? {
                bail!("composition surfaces cannot contain themselves");
            }
        }

        let previous_parent = self.surface(surface)?.parent;
        self.siblings_mut(previous_parent)?
            .retain(|candidate| *candidate != surface);
        self.surface_mut(surface)?.parent = parent;
        self.siblings_mut(parent)?.push(surface);
        Ok(())
    }

    pub(crate) fn kind(&self, surface: CompositionSurfaceId) -> Result<CompositionSurfaceKind> {
        Ok(self.surface(surface)?.kind)
    }

    pub(crate) fn parent(
        &self,
        surface: CompositionSurfaceId,
    ) -> Result<Option<CompositionSurfaceId>> {
        Ok(self.surface(surface)?.parent)
    }

    pub(crate) fn children(
        &self,
        parent: Option<CompositionSurfaceId>,
    ) -> Result<&[CompositionSurfaceId]> {
        match parent {
            Some(parent) => Ok(&self.surface(parent)?.children),
            None => Ok(&self.roots),
        }
    }

    pub(crate) fn flattened(&self) -> Vec<CompositionSurfaceId> {
        let mut surfaces = Vec::with_capacity(self.surfaces.len());
        for root in &self.roots {
            self.flatten_into(*root, &mut surfaces);
        }
        surfaces
    }

    fn place_relative(
        &mut self,
        surface: CompositionSurfaceId,
        sibling: CompositionSurfaceId,
        above: bool,
    ) -> Result<()> {
        if surface == sibling {
            bail!("a composition surface cannot be ordered relative to itself");
        }
        let parent = self.surface(surface)?.parent;
        if self.surface(sibling)?.parent != parent {
            bail!("composition surfaces must share a parent to be reordered");
        }

        let siblings = self.siblings_mut(parent)?;
        siblings.retain(|candidate| *candidate != surface);
        let sibling_index = siblings
            .iter()
            .position(|candidate| *candidate == sibling)
            .ok_or_else(|| anyhow!("composition sibling is missing from its parent"))?;
        let index = sibling_index + usize::from(above);
        siblings.insert(index, surface);
        Ok(())
    }

    fn is_descendant(
        &self,
        candidate: CompositionSurfaceId,
        ancestor: CompositionSurfaceId,
    ) -> Result<bool> {
        let mut parent = Some(candidate);
        while let Some(surface) = parent {
            if surface == ancestor {
                return Ok(true);
            }
            parent = self.surface(surface)?.parent;
        }
        Ok(false)
    }

    fn surface(&self, surface: CompositionSurfaceId) -> Result<&CompositionSurfaceNode> {
        self.surfaces
            .get(surface)
            .ok_or_else(|| anyhow!("composition surface does not exist"))
    }

    fn surface_mut(
        &mut self,
        surface: CompositionSurfaceId,
    ) -> Result<&mut CompositionSurfaceNode> {
        self.surfaces
            .get_mut(surface)
            .ok_or_else(|| anyhow!("composition surface does not exist"))
    }

    fn siblings_mut(
        &mut self,
        parent: Option<CompositionSurfaceId>,
    ) -> Result<&mut Vec<CompositionSurfaceId>> {
        match parent {
            Some(parent) => Ok(&mut self.surface_mut(parent)?.children),
            None => Ok(&mut self.roots),
        }
    }

    fn flatten_into(
        &self,
        surface: CompositionSurfaceId,
        flattened: &mut Vec<CompositionSurfaceId>,
    ) {
        flattened.push(surface);
        if let Some(surface) = self.surfaces.get(surface) {
            for child in &surface.children {
                self.flatten_into(*child, flattened);
            }
        }
    }
}

pub(crate) fn composed_scene_layers(
    tree: &CompositionTree,
    starts: &[(CompositionSurfaceId, usize)],
    scene_len: usize,
) -> Result<Vec<crate::ComposedSceneLayer>> {
    let mut ranges_by_surface = FxHashMap::<_, Vec<std::ops::Range<usize>>>::default();
    for (index, (surface, start)) in starts.iter().enumerate() {
        if tree.kind(*surface)? != CompositionSurfaceKind::Gpui {
            bail!("scene operations target a non-GPUI composition surface");
        }
        let end = starts
            .get(index + 1)
            .map_or(scene_len, |(_, next_start)| *next_start);
        if *start > end || end > scene_len {
            bail!("composition scene ranges are not monotonic");
        }
        ranges_by_surface
            .entry(*surface)
            .or_default()
            .push(*start..end);
    }

    let mut layers = Vec::new();
    for surface in tree.flattened() {
        if tree.kind(surface)? == CompositionSurfaceKind::Gpui {
            layers.push(crate::ComposedSceneLayer {
                surface,
                ranges: ranges_by_surface.remove(&surface).unwrap_or_default(),
            });
        }
    }
    Ok(layers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_surfaces_have_stable_identity_and_explicit_order() {
        let mut tree = CompositionTree::new();
        let base = tree.insert(CompositionSurfaceKind::Gpui, None).unwrap();
        let webview = tree.insert(CompositionSurfaceKind::Native, None).unwrap();
        let overlay = tree.insert(CompositionSurfaceKind::Gpui, None).unwrap();

        tree.place_below(overlay, webview).unwrap();

        assert_eq!(tree.flattened(), [base, overlay, webview]);
        assert_eq!(tree.kind(webview).unwrap(), CompositionSurfaceKind::Native);
    }

    #[test]
    fn composition_surfaces_can_be_nested_and_reparented() {
        let mut tree = CompositionTree::new();
        let base = tree.insert(CompositionSurfaceKind::Gpui, None).unwrap();
        let pane = tree
            .insert(CompositionSurfaceKind::Native, Some(base))
            .unwrap();
        let video = tree
            .insert(CompositionSurfaceKind::ExternalGpu, None)
            .unwrap();

        tree.reparent(video, Some(pane)).unwrap();

        assert_eq!(tree.parent(video).unwrap(), Some(pane));
        assert_eq!(tree.children(Some(pane)).unwrap(), [video]);
        assert_eq!(tree.flattened(), [base, pane, video]);
    }

    #[test]
    fn composition_tree_rejects_cycles_and_cross_parent_ordering() {
        let mut tree = CompositionTree::new();
        let parent = tree.insert(CompositionSurfaceKind::Gpui, None).unwrap();
        let child = tree
            .insert(CompositionSurfaceKind::Native, Some(parent))
            .unwrap();
        let sibling = tree.insert(CompositionSurfaceKind::Gpui, None).unwrap();

        assert!(tree.reparent(parent, Some(child)).is_err());
        assert!(tree.place_above(child, sibling).is_err());
        assert_eq!(tree.parent(parent).unwrap(), None);
        assert_eq!(tree.parent(child).unwrap(), Some(parent));
    }

    #[test]
    fn removing_a_surface_preserves_and_reparents_its_children() {
        let mut tree = CompositionTree::new();
        let parent = tree.insert(CompositionSurfaceKind::Gpui, None).unwrap();
        let child = tree
            .insert(CompositionSurfaceKind::Native, Some(parent))
            .unwrap();

        tree.remove(parent).unwrap();

        assert_eq!(tree.parent(child).unwrap(), None);
        assert_eq!(tree.flattened(), [child]);
    }

    #[test]
    fn scene_ranges_follow_tree_order_and_preserve_repeated_segments() {
        let mut tree = CompositionTree::new();
        let base = tree.insert(CompositionSurfaceKind::Gpui, None).unwrap();
        let native = tree.insert(CompositionSurfaceKind::Native, None).unwrap();
        let overlay = tree.insert(CompositionSurfaceKind::Gpui, None).unwrap();
        let starts = [(base, 0), (overlay, 4), (base, 7)];

        let layers = composed_scene_layers(&tree, &starts, 10).unwrap();

        let [base_layer, overlay_layer] = layers.as_slice() else {
            panic!("expected base and overlay scene layers");
        };
        assert_eq!(base_layer.surface, base);
        assert_eq!(base_layer.ranges, [0..4, 7..10]);
        assert_eq!(overlay_layer.surface, overlay);
        assert_eq!(overlay_layer.ranges, [4..7]);
        assert_eq!(tree.kind(native).unwrap(), CompositionSurfaceKind::Native);
    }

    #[test]
    fn scene_ranges_reject_non_gpui_targets_and_non_monotonic_markers() {
        let mut tree = CompositionTree::new();
        let base = tree.insert(CompositionSurfaceKind::Gpui, None).unwrap();
        let native = tree.insert(CompositionSurfaceKind::Native, None).unwrap();

        assert!(composed_scene_layers(&tree, &[(native, 0)], 1).is_err());
        assert!(composed_scene_layers(&tree, &[(base, 2), (base, 1)], 2).is_err());
    }
}
