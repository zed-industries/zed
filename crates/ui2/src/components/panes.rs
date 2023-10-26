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
pub struct Pane<V: 'static> {
    id: ElementId,
    state_type: PhantomData<V>,
    size: Size<Length>,
    fill: Hsla,
    children: SmallVec<[AnyElement<V>; 2]>,
}

impl<V: 'static + Send + Sync> IntoAnyElement<V> for Pane<V> {
    fn into_any(self) -> AnyElement<V> {
        let render =
            move |view_state: &mut V, cx: &mut ViewContext<'_, '_, V>| self.render(view_state, cx);

        AnyElement::new(ElementRenderer {
            render: Some(render),
            view_type: PhantomData,
            element_type: PhantomData,
        })
    }
}

struct ElementRenderer<V, E, F>
where
    V: 'static,
    E: 'static + IntoAnyElement<V> + Send + Sync,
    F: FnOnce(&mut V, &mut ViewContext<'_, '_, V>) -> E + 'static + Send + Sync,
{
    render: Option<F>,
    view_type: PhantomData<V>,
    element_type: PhantomData<E>,
}

unsafe impl<V, E, F> Send for ElementRenderer<V, E, F>
where
    V: 'static,
    E: 'static + IntoAnyElement<V> + Send + Sync,
    F: FnOnce(&mut V, &mut ViewContext<'_, '_, V>) -> E + 'static + Send + Sync,
{
}

unsafe impl<V, E, F> Sync for ElementRenderer<V, E, F>
where
    V: 'static,
    E: 'static + IntoAnyElement<V> + Send + Sync,
    F: FnOnce(&mut V, &mut ViewContext<'_, '_, V>) -> E + 'static + Send + Sync,
{
}

impl<V, E, F> Element<V> for ElementRenderer<V, E, F>
where
    V: 'static,
    E: 'static + IntoAnyElement<V> + Send + Sync,
    F: FnOnce(&mut V, &mut ViewContext<'_, '_, V>) -> E + 'static + Send + Sync,
{
    type ElementState = AnyElement<V>;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn initialize(
        &mut self,
        view_state: &mut V,
        _element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<V>,
    ) -> Self::ElementState {
        let render = self.render.take().unwrap();
        (render)(view_state, cx).into_any()
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        rendered_element: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) -> gpui2::LayoutId {
        rendered_element.layout(view_state, cx)
    }

    fn paint(
        &mut self,
        bounds: gpui2::Bounds<gpui2::Pixels>,
        view_state: &mut V,
        rendered_element: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) {
        rendered_element.paint(view_state, cx)
    }
}

impl<V, E, F> IntoAnyElement<V> for ElementRenderer<V, E, F>
where
    V: 'static,
    E: 'static + IntoAnyElement<V> + Send + Sync,
    F: FnOnce(&mut V, &mut ViewContext<V>) -> E + 'static + Send + Sync,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V: 'static> Pane<V> {
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

#[derive(Element)]
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

    fn render(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
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
