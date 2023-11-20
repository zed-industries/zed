use gpui::{
    hsla, red, AnyElement, Div, ElementId, ExternalPaths, Hsla, Length, RenderOnce, Size, Stateful,
    View,
};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(Default, PartialEq)]
pub enum SplitDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(RenderOnce)]
pub struct Pane {
    id: ElementId,
    size: Size<Length>,
    fill: Hsla,
    children: SmallVec<[AnyElement; 2]>,
}

impl Component for Pane {
    type Rendered = gpui::Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div()
            .id(self.id.clone())
            .flex()
            .flex_initial()
            .bg(self.fill)
            .w(self.size.width)
            .h(self.size.height)
            .relative()
            .child(div().z_index(0).size_full().children(self.children))
            .child(
                div()
                    .z_index(1)
                    .id("drag-target")
                    .drag_over::<ExternalPaths>(|d| d.bg(red()))
                    .on_drop(|_, files: View<ExternalPaths>, cx| {
                        eprintln!("dropped files! {:?}", files.read(cx));
                    })
                    .absolute()
                    .inset_0(),
            )
    }
}

impl Pane {
    pub fn new(id: impl Into<ElementId>, size: Size<Length>) -> Self {
        // Fill is only here for debugging purposes, remove before release

        Self {
            id: id.into(),
            size,
            fill: hsla(0.3, 0.3, 0.3, 1.),
            children: SmallVec::new(),
        }
    }

    pub fn fill(mut self, fill: Hsla) -> Self {
        self.fill = fill;
        self
    }
}

impl ParentElement for Pane {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

#[derive(RenderOnce)]
pub struct PaneGroup {
    groups: Vec<PaneGroup>,
    panes: Vec<Pane>,
    split_direction: SplitDirection,
}

impl Component for PaneGroup {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        if !self.panes.is_empty() {
            let el = div()
                .flex()
                .flex_1()
                .gap_px()
                .w_full()
                .h_full()
                .children(self.panes.into_iter().map(|pane| pane.render(cx)));

            if self.split_direction == SplitDirection::Horizontal {
                return el;
            } else {
                return el.flex_col();
            }
        }

        if !self.groups.is_empty() {
            let el = div()
                .flex()
                .flex_1()
                .gap_px()
                .w_full()
                .h_full()
                .bg(cx.theme().colors().editor_background)
                .children(self.groups.into_iter().map(|group| group.render(cx)));

            if self.split_direction == SplitDirection::Horizontal {
                return el;
            } else {
                return el.flex_col();
            }
        }

        unreachable!()
    }
}

impl PaneGroup {
    pub fn new_groups(groups: Vec<PaneGroup>, split_direction: SplitDirection) -> Self {
        Self {
            groups,
            panes: Vec::new(),
            split_direction,
        }
    }

    pub fn new_panes(panes: Vec<Pane>, split_direction: SplitDirection) -> Self {
        Self {
            groups: Vec::new(),
            panes,
            split_direction,
        }
    }
}
