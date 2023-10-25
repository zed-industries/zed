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

// #[derive(Element)]
pub struct Pane<S: 'static + Send + Sync> {
    id: ElementId,
    state_type: PhantomData<S>,
    size: Size<Length>,
    fill: Hsla,
    children: SmallVec<[AnyElement<S>; 2]>,
}

impl<V: 'static + Send + Sync> IntoAnyElement<V> for Pane<V> {
    fn into_any(self) -> AnyElement<V> {
        ElementRenderer {
            id: Some(self.id),
            render: Some(move |view_state, cx| self.render(view_state, cx)),
            view_type: PhantomData,
            element_type: PhantomData,
        }
    }
}

struct ElementRenderer<V, E, F>
where
    E: IntoAnyElement<V>,
    F: FnOnce(&mut V, &mut ViewContext<V>) -> E,
{
    id: Option<ElementId>,
    render: Option<F>,
    view_type: PhantomData<V>,
    element_type: PhantomData<E>,
}

impl<V, E, F> Element for ElementRenderer<V, E, F>
where
    V: 'static,
    E: IntoAnyElement<V>,
    F: FnOnce(&mut V, &mut ViewContext<V>) -> E,
{
    type ViewState = V;
    type ElementState = AnyElement<V>;

    fn id(&self) -> Option<ElementId> {
        self.id
    }

    fn initialize(
        &mut self,
        view_state: &mut Self::ViewState,
        rendered_element: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> Self::ElementState {
        rendered_element.unwrap_or_else(|| {
            let render = self.render.take().unwrap();
            (render)(view_state, cx)
        })
    }

    fn layout(
        &mut self,
        view_state: &mut Self::ViewState,
        rendered_element: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> gpui2::LayoutId {
        rendered_element.layout(view_state, cx)
    }

    fn paint(
        &mut self,
        bounds: gpui2::Bounds<gpui2::Pixels>,
        view_state: &mut Self::ViewState,
        rendered_element: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        rendered_element.paint(view_state, cx)
    }
}

impl<V, E, F> IntoAnyElement<V> for ElementRenderer<V, E, F>
where
    V: 'static,
    E: IntoAnyElement<V>,
    F: FnOnce(&mut V, &mut ViewContext<V>) -> E,
{
    fn into_any(self) -> AnyElement<V> {
        self
    }
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
        div()
            .id(self.id.clone())
            .flex()
            .flex_initial()
            .bg(self.fill)
            .w(self.size.width)
            .h(self.size.height)
            .relative()
            .child(
                div()
                    .z_index(0)
                    .size_full()
                    .children(self.children.drain(..)),
            )
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
        let theme = theme(cx);

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
                .bg(theme.editor)
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
