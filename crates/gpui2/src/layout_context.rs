use crate::{element::LayoutId, style::Style};
use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use gpui::{geometry::Size, taffy::style::Overflow, MeasureParams};
pub use gpui::{taffy::tree::NodeId, LayoutContext as LegacyLayoutContext};

#[derive(Deref, DerefMut)]
pub struct LayoutContext<'a, 'b, 'c, 'd, V> {
    #[deref]
    #[deref_mut]
    pub(crate) legacy_cx: &'d mut LegacyLayoutContext<'a, 'b, 'c, V>,
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
        let rem_size = self.rem_size();
        let style = style.to_taffy(rem_size);
        let id = self
            .legacy_cx
            .layout_engine()
            .ok_or_else(|| anyhow!("no layout engine"))?
            .add_node(style, children)?;

        Ok(id)
    }

    pub fn add_measured_layout_node<F>(&mut self, style: Style, measure: F) -> Result<LayoutId>
    where
        F: Fn(MeasureParams) -> Size<f32> + Sync + Send + 'static,
    {
        let rem_size = self.rem_size();
        let layout_id = self
            .layout_engine()
            .ok_or_else(|| anyhow!("no layout engine"))?
            .add_measured_node(style.to_taffy(rem_size), measure)?;

        Ok(layout_id)
    }
}
