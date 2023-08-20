use crate::{
    element::{
        AnyElement, Element, EventHandler, IntoElement, Layout, LayoutContext, NodeId,
        PaintContext, ParentElement,
    },
    style::{Style, StyleRefinement},
};
use anyhow::{anyhow, Result};
use gpui::LayoutId;
use playground_macros::IntoElement;
use refineable::Refineable;

#[derive(IntoElement)]
#[element_crate = "crate"]
pub struct Frame<V: 'static> {
    style: StyleRefinement,
    handlers: Vec<EventHandler<V>>,
    children: Vec<AnyElement<V>>,
}

pub fn frame<V>() -> Frame<V> {
    Frame {
        style: StyleRefinement::default(),
        handlers: Vec::new(),
        children: Vec::new(),
    }
}

impl<V: 'static> Element<V> for Frame<V> {
    type Layout = ();

    fn declared_style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }

    fn handlers_mut(&mut self) -> &mut Vec<EventHandler<V>> {
        &mut self.handlers
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
            .collect::<Result<Vec<LayoutId>>>()?;

        let rem_size = cx.rem_pixels();
        let style = Style::default().refined(&self.style);
        let node_id = cx
            .layout_engine()
            .ok_or_else(|| anyhow!("no layout engine"))?
            .add_node(style.to_taffy(rem_size), child_layout_node_ids)?;

        Ok((node_id, ()))
    }

    fn paint(&mut self, layout: Layout<()>, view: &mut V, cx: &mut PaintContext<V>) -> Result<()> {
        for child in &mut self.children {
            child.paint(view, cx)?;
        }
        Ok(())
    }
}

impl<V: 'static> ParentElement<V> for Frame<V> {
    fn child(mut self, child: impl IntoElement<V>) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    fn children<I, E>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: IntoElement<V>,
    {
        self.children
            .extend(children.into_iter().map(|e| e.into_any_element()));
        self
    }
}
