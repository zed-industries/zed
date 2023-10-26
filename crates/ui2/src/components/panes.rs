use std::marker::PhantomData;

use gpui2::{hsla, red, AnyElement, ElementId, ExternalPaths, Hsla, Length, Size};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(Default, PartialEq)]
pub enum SplitDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(IntoAnyElement)]
pub struct Pane<V: 'static> {
    id: ElementId,
    size: Size<Length>,
    fill: Hsla,
    children: SmallVec<[AnyElement<V>; 2]>,
}

// impl<V: 'static> IntoAnyElement<V> for Pane<V> {
//     fn into_any(self) -> AnyElement<V> {
//         (move |view_state: &mut V, cx: &mut ViewContext<'_, '_, V>| self.render(view_state, cx))
//             .into_any()
//     }
// }

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

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> impl IntoAnyElement<V> {
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
                // TODO kb! Figure out why we can't we see the red background when we drag a file over this div.
                div()
                    .z_index(1)
                    .id("drag-target")
                    .drag_over::<ExternalPaths>(|d| d.bg(red()))
                    .on_drop(|_, files: ExternalPaths, _| {
                        dbg!("dropped files!", files);
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

#[derive(IntoAnyElement)]
pub struct PaneGroup<V: 'static + Send + Sync> {
    state_type: PhantomData<V>,
    groups: Vec<PaneGroup<V>>,
    panes: Vec<Pane<V>>,
    split_direction: SplitDirection,
}

impl<V: 'static + Send + Sync> PaneGroup<V> {
    pub fn new_groups(groups: Vec<PaneGroup<V>>, split_direction: SplitDirection) -> Self {
        Self {
            state_type: PhantomData,
            groups,
            panes: Vec::new(),
            split_direction,
        }
    }

    pub fn new_panes(panes: Vec<Pane<V>>, split_direction: SplitDirection) -> Self {
        Self {
            state_type: PhantomData,
            groups: Vec::new(),
            panes,
            split_direction,
        }
    }

    fn render(mut self, view: &mut V, cx: &mut ViewContext<V>) -> impl IntoAnyElement<V> {
        let theme = theme(cx);

        if !self.panes.is_empty() {
            let el = div()
                .flex()
                .flex_1()
                .gap_px()
                .w_full()
                .h_full()
                .children(self.panes.drain(..).map(|pane| pane.render(view, cx)));

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
                .bg(theme.editor)
                .children(self.groups.drain(..).map(|group| group.render(view, cx)));

            if self.split_direction == SplitDirection::Horizontal {
                return el;
            } else {
                return el.flex_col();
            }
        }

        unreachable!()
    }
}
