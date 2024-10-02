use gpui::AnyElement;
use smallvec::SmallVec;

use crate::{prelude::*, ListHeader};

/// A group of settings.
#[derive(IntoElement)]
pub struct SettingsGroup {
    header: SharedString,
    children: SmallVec<[AnyElement; 2]>,
}

impl SettingsGroup {
    pub fn new(header: impl Into<SharedString>) -> Self {
        Self {
            header: header.into(),
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for SettingsGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for SettingsGroup {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        v_flex()
            .p_1()
            .gap_2()
            .child(ListHeader::new(self.header))
            .children(self.children)
    }
}
