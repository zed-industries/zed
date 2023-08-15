use anyhow::{anyhow, Result};
use gpui::{EngineLayout, LayoutNodeId};

use crate::{
    element::{AnyElement, Element, LayoutContext, PaintContext},
    style::Style,
};

pub struct Frame<V> {
    style: Style,
    children: Vec<AnyElement<V>>,
}

pub fn frame<V>() -> Frame<V> {
    Frame {
        style: Style::default(),
        children: Vec::new(),
    }
}

impl<V: 'static> Element<V> for Frame<V> {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<taffy::tree::NodeId> {
        let child_layout_node_ids = self
            .children
            .iter_mut()
            .map(|child| child.layout(view, cx))
            .collect::<Result<Vec<LayoutNodeId>>>()?;

        let rem_size = cx.rem_pixels();
        cx.layout_engine()
            .ok_or_else(|| anyhow!("no layout engine"))?
            .add_node(self.style.to_taffy(rem_size), child_layout_node_ids)
    }

    fn paint(
        &mut self,
        layout: EngineLayout,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Result<()> {
        cx.scene.push_quad(gpui::scene::Quad {
            bounds: layout.bounds,
            background: self.style.fill.color().map(Into::into),
            border: Default::default(),
            corner_radii: Default::default(),
        });

        for child in &mut self.children {
            child.paint(view, cx)?;
        }
        Ok(())
    }
}

impl<V> Clone for Frame<V> {
    fn clone(&self) -> Self {
        Self {
            style: self.style.clone(),
            children: self.children.clone(),
        }
    }
}
