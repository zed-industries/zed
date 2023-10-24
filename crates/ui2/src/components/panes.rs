use std::marker::PhantomData;

use gpui2::{hsla, red, AnyElement, DroppedFiles, ElementId, Hsla, Length, Size};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(Default, PartialEq)]
pub enum SplitDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Element)]
pub struct Pane<S: 'static + Send + Sync> {
    id: ElementId,
    state_type: PhantomData<S>,
    size: Size<Length>,
    fill: Hsla,
    children: SmallVec<[AnyElement<S>; 2]>,
}

impl<S: 'static + Send + Sync> Pane<S> {
    pub fn new(id: impl Into<ElementId>, size: Size<Length>) -> Self {
        // Fill is only here for debugging purposes, remove before release

        Self {
            id: id.into(),
            state_type: PhantomData,
            size,
            fill: hsla(0.3, 0.3, 0.3, 1.),
            // fill: system_color.transparent,
            children: SmallVec::new(),
        }
    }

    pub fn fill(mut self, fill: Hsla) -> Self {
        self.fill = fill;
        self
    }

    fn render(&mut self, view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new(cx);

        div()
            .id(self.id.clone())
            .flex()
            .flex_initial()
            .bg(self.fill)
            .w(self.size.width)
            .h(self.size.height)
            .relative()
            .children(cx.stack(0, |_| self.children.drain(..)))
            .child(cx.stack(1, |_| {
                // TODO kb! Figure out why we can't we see the red background when we drag a file over this div.
                div()
                    .id("drag-target")
                    .drag_over::<DroppedFiles>(|d| d.bg(red()))
                    .on_drop(|_, files: DroppedFiles, _| {
                        dbg!("dropped files!", files);
                    })
                    .absolute()
                    .inset_0()
            }))
    }
}

impl<S: 'static + Send + Sync> ParentElement for Pane<S> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::ViewState>; 2]> {
        &mut self.children
    }
}

#[derive(Element)]
pub struct PaneGroup<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    groups: Vec<PaneGroup<S>>,
    panes: Vec<Pane<S>>,
    split_direction: SplitDirection,
}

impl<S: 'static + Send + Sync> PaneGroup<S> {
    pub fn new_groups(groups: Vec<PaneGroup<S>>, split_direction: SplitDirection) -> Self {
        Self {
            state_type: PhantomData,
            groups,
            panes: Vec::new(),
            split_direction,
        }
    }

    pub fn new_panes(panes: Vec<Pane<S>>, split_direction: SplitDirection) -> Self {
        Self {
            state_type: PhantomData,
            groups: Vec::new(),
            panes,
            split_direction,
        }
    }

    fn render(&mut self, view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new(cx);

        if !self.panes.is_empty() {
            let el = div()
                .flex()
                .flex_1()
                .gap_px()
                .w_full()
                .h_full()
                .children(self.panes.iter_mut().map(|pane| pane.render(view, cx)));

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
                .bg(color.editor)
                .children(self.groups.iter_mut().map(|group| group.render(view, cx)));

            if self.split_direction == SplitDirection::Horizontal {
                return el;
            } else {
                return el.flex_col();
            }
        }

        unreachable!()
    }
}
