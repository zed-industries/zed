use std::marker::PhantomData;

use gpui3::{hsla, Hsla, Length, Size};

use crate::prelude::*;
use crate::theme;

#[derive(Default, PartialEq)]
pub enum SplitDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Element)]
pub struct Pane<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    scroll_state: ScrollState,
    size: Size<Length>,
    fill: Hsla,
    children: HackyChildren<S>,
    payload: HackyChildrenPayload,
}

impl<S: 'static + Send + Sync> Pane<S> {
    pub fn new(
        scroll_state: ScrollState,
        size: Size<Length>,
        children: HackyChildren<S>,
        payload: HackyChildrenPayload,
    ) -> Self {
        // Fill is only here for debugging purposes, remove before release
        let system_color = SystemColor::new();

        Self {
            state_type: PhantomData,
            scroll_state,
            size,
            fill: hsla(0.3, 0.3, 0.3, 1.),
            // fill: system_color.transparent,
            children,
            payload,
        }
    }

    pub fn fill(mut self, fill: Hsla) -> Self {
        self.fill = fill;
        self
    }

    fn render(&mut self, view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let theme = theme(cx);

        div()
            .flex()
            .flex_initial()
            .fill(self.fill)
            .w(self.size.width)
            .h(self.size.height)
            .overflow_y_scroll(self.scroll_state.clone())
            .children_any((self.children)(cx, self.payload.as_ref()))
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
        let theme = theme(cx);

        if !self.panes.is_empty() {
            let el = div()
                .flex()
                .flex_1()
                .gap_px()
                .w_full()
                .h_full()
                .fill(theme.lowest.base.default.background)
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
                .fill(theme.lowest.base.default.background)
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
