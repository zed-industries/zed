use crate::{
    div, AnyElement, BorrowWindow, Bounds, Component, Div, DivState, Element, ElementId,
    ElementInteractivity, FocusListeners, Focusable, FocusableKeyDispatch, KeyDispatch, LayoutId,
    NonFocusableKeyDispatch, Pixels, SharedString, StatefulInteractive, StatefulInteractivity,
    StatelessInteractive, StatelessInteractivity, StyleRefinement, Styled, ViewContext,
};
use futures::FutureExt;
use util::ResultExt;

pub struct Img<
    V: 'static,
    I: ElementInteractivity<V> = StatelessInteractivity<V>,
    F: KeyDispatch<V> = NonFocusableKeyDispatch,
> {
    base: Div<V, I, F>,
    uri: Option<SharedString>,
    grayscale: bool,
}

pub fn img<V: 'static>() -> Img<V, StatelessInteractivity<V>, NonFocusableKeyDispatch> {
    Img {
        base: div(),
        uri: None,
        grayscale: false,
    }
}

impl<V, I, F> Img<V, I, F>
where
    V: 'static,
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    pub fn uri(mut self, uri: impl Into<SharedString>) -> Self {
        self.uri = Some(uri.into());
        self
    }

    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }
}

impl<V, F> Img<V, StatelessInteractivity<V>, F>
where
    F: KeyDispatch<V>,
{
    pub fn id(self, id: impl Into<ElementId>) -> Img<V, StatefulInteractivity<V>, F> {
        Img {
            base: self.base.id(id),
            uri: self.uri,
            grayscale: self.grayscale,
        }
    }
}

impl<V, I, F> Component<V> for Img<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    fn render(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, I, F> Element<V> for Img<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    type ElementState = DivState;

    fn id(&self) -> Option<crate::ElementId> {
        self.base.id()
    }

    fn initialize(
        &mut self,
        view_state: &mut V,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<V>,
    ) -> Self::ElementState {
        self.base.initialize(view_state, element_state, cx)
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) -> LayoutId {
        self.base.layout(view_state, element_state, cx)
    }

    fn prepaint(
        &mut self,
        bounds: Bounds<Pixels>,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) {
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) {
        cx.with_z_index(0, |cx| {
            self.base.paint(bounds, view, element_state, cx);
        });

        let style = self.base.compute_style(bounds, element_state, cx);
        let corner_radii = style.corner_radii;

        if let Some(uri) = self.uri.clone() {
            // eprintln!(">>> image_cache.get({uri}");
            let image_future = cx.image_cache.get(uri.clone());
            // eprintln!("<<< image_cache.get({uri}");
            if let Some(data) = image_future
                .clone()
                .now_or_never()
                .and_then(ResultExt::log_err)
            {
                let corner_radii = corner_radii.to_pixels(bounds.size, cx.rem_size());
                cx.with_z_index(1, |cx| {
                    cx.paint_image(bounds, corner_radii, data, self.grayscale)
                        .log_err()
                });
            } else {
                cx.spawn(|_, mut cx| async move {
                    if image_future.await.log_err().is_some() {
                        cx.on_next_frame(|cx| cx.notify());
                    }
                })
                .detach()
            }
        }
    }
}

impl<V, I, F> Styled for Img<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<V, I, F> StatelessInteractive<V> for Img<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    fn stateless_interactivity(&mut self) -> &mut StatelessInteractivity<V> {
        self.base.stateless_interactivity()
    }
}

impl<V, F> StatefulInteractive<V> for Img<V, StatefulInteractivity<V>, F>
where
    F: KeyDispatch<V>,
{
    fn stateful_interactivity(&mut self) -> &mut StatefulInteractivity<V> {
        self.base.stateful_interactivity()
    }
}

impl<V, I> Focusable<V> for Img<V, I, FocusableKeyDispatch<V>>
where
    V: 'static,
    I: ElementInteractivity<V>,
{
    fn focus_listeners(&mut self) -> &mut FocusListeners<V> {
        self.base.focus_listeners()
    }

    fn set_focus_style(&mut self, style: StyleRefinement) {
        self.base.set_focus_style(style)
    }

    fn set_focus_in_style(&mut self, style: StyleRefinement) {
        self.base.set_focus_in_style(style)
    }

    fn set_in_focus_style(&mut self, style: StyleRefinement) {
        self.base.set_in_focus_style(style)
    }
}
