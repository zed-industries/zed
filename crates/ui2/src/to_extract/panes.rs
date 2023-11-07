use gpui::{hsla, red, AnyElement, ElementId, ExternalPaths, Hsla, Length, Size, View};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(Default, PartialEq)]
pub enum SplitDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Component)]
pub struct Pane<V: 'static> {
    id: ElementId,
    size: Size<Length>,
    fill: Hsla,
    children: SmallVec<[AnyElement<V>; 2]>,
}

impl<V: 'static> Pane<V> {
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

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
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

impl<V: 'static> ParentElement<V> for Pane<V> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        &mut self.children
    }
}

#[derive(Component)]
pub struct PaneGroup<V: 'static> {
    groups: Vec<PaneGroup<V>>,
    panes: Vec<Pane<V>>,
    split_direction: SplitDirection,
}

impl<V: 'static> PaneGroup<V> {
    pub fn new_groups(groups: Vec<PaneGroup<V>>, split_direction: SplitDirection) -> Self {
        Self {
            groups,
            panes: Vec::new(),
            split_direction,
        }
    }

    pub fn new_panes(panes: Vec<Pane<V>>, split_direction: SplitDirection) -> Self {
        Self {
            groups: Vec::new(),
            panes,
            split_direction,
        }
    }

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        if !self.panes.is_empty() {
            let el = div()
                .flex()
                .flex_1()
                .gap_px()
                .w_full()
                .h_full()
                .children(self.panes.into_iter().map(|pane| pane.render(view, cx)));

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
                .children(self.groups.into_iter().map(|group| group.render(view, cx)));

            if self.split_direction == SplitDirection::Horizontal {
                return el;
            } else {
                return el.flex_col();
            }
        }

        unreachable!()
    }
}
