use crate::element::{
    AnyElement, Element, ElementMetadata, IntoElement, Layout, LayoutContext, NodeId, PaintContext,
};
use anyhow::{anyhow, Result};
use gpui::LayoutNodeId;
use playground_macros::IntoElement;

#[derive(IntoElement)]
#[element_crate = "crate"]
pub struct Div<V: 'static> {
    metadata: ElementMetadata<V>,
    children: Vec<AnyElement<V>>,
}

pub fn div<V>() -> Div<V> {
    Div {
        metadata: ElementMetadata::default(),
        children: Vec::new(),
    }
}

impl<V: 'static> Element<V> for Div<V> {
    type Layout = ();

    fn metadata(&mut self) -> &mut ElementMetadata<V> {
        &mut self.metadata
    }

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> Result<(NodeId, Self::Layout)> {
        let child_layout_node_ids = self
            .children
            .iter_mut()
            .map(|child| child.layout(view, cx))
            .collect::<Result<Vec<LayoutNodeId>>>()?;

        let rem_size = cx.rem_pixels();
        let node_id = cx
            .layout_engine()
            .ok_or_else(|| anyhow!("no layout engine"))?
            .add_node(
                self.metadata.style.to_taffy(rem_size),
                child_layout_node_ids,
            )?;

        Ok((node_id, ()))
    }

    fn paint(&mut self, layout: Layout<()>, view: &mut V, cx: &mut PaintContext<V>) -> Result<()> {
        cx.scene.push_quad(gpui::scene::Quad {
            bounds: layout.from_engine.bounds,
            background: self.metadata.style.fill.color().map(Into::into),
            border: Default::default(),
            corner_radii: Default::default(),
        });

        for child in &mut self.children {
            child.paint(view, cx)?;
        }
        Ok(())
    }
}

impl<V: 'static> Div<V> {
    pub fn child(mut self, child: impl IntoElement<V>) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children<I, E>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: IntoElement<V>,
    {
        self.children
            .extend(children.into_iter().map(|e| e.into_any_element()));
        self
    }
}
