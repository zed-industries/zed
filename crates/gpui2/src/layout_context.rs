use crate::{element::LayoutId, style::Style};
use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use gpui::{geometry::Size, MeasureParams, RenderContext, ViewContext};
pub use gpui::{taffy::tree::NodeId, LayoutContext as LegacyLayoutContext};

#[derive(Deref, DerefMut)]
pub struct LayoutContext<'a, 'b, 'c, 'd, V> {
    #[deref]
    #[deref_mut]
    pub(crate) legacy_cx: &'d mut LegacyLayoutContext<'a, 'b, 'c, V>,
}

impl<'a, 'b, V> RenderContext<'a, 'b, V> for LayoutContext<'a, 'b, '_, '_, V> {
    fn text_style(&self) -> gpui::fonts::TextStyle {
        self.legacy_cx.text_style()
    }

    fn push_text_style(&mut self, style: gpui::fonts::TextStyle) {
        self.legacy_cx.push_text_style(style)
    }

    fn pop_text_style(&mut self) {
        self.legacy_cx.pop_text_style()
    }

    fn as_view_context(&mut self) -> &mut ViewContext<'a, 'b, V> {
        &mut self.view_context
    }
}

impl<'a, 'b, 'c, 'd, V: 'static> LayoutContext<'a, 'b, 'c, 'd, V> {
    pub fn new(legacy_cx: &'d mut LegacyLayoutContext<'a, 'b, 'c, V>) -> Self {
        Self { legacy_cx }
    }

    pub fn add_layout_node(
        &mut self,
        style: Style,
        children: impl IntoIterator<Item = NodeId>,
    ) -> Result<LayoutId> {
        let rem_size = self.rem_pixels();
        let id = self
            .legacy_cx
            .layout_engine()
            .ok_or_else(|| anyhow!("no layout engine"))?
            .add_node(style.to_taffy(rem_size), children)?;

        Ok(id)
    }

    pub fn add_measured_layout_node<F>(&mut self, style: Style, measure: F) -> Result<LayoutId>
    where
        F: Fn(MeasureParams) -> Size<f32> + Sync + Send + 'static,
    {
        let rem_size = self.rem_pixels();
        let layout_id = self
            .layout_engine()
            .ok_or_else(|| anyhow!("no layout engine"))?
            .add_measured_node(style.to_taffy(rem_size), measure)?;

        Ok(layout_id)
    }
}
