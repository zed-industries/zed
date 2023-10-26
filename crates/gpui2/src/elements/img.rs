use crate::{
    div, AnyElement, BorrowWindow, Bounds, Div, DivState, Element, ElementFocus, ElementId,
    ElementInteraction, FocusDisabled, FocusEnabled, FocusListeners, Focusable, IntoAnyElement,
    LayoutId, Pixels, SharedString, StatefulInteraction, StatefulInteractive, StatelessInteraction,
    StatelessInteractive, StyleRefinement, Styled, ViewContext,
};
use futures::FutureExt;
use util::ResultExt;

pub struct Img<
    V: 'static,
    I: ElementInteraction<V> = StatelessInteraction<V>,
    F: ElementFocus<V> = FocusDisabled,
> {
    base: Div<V, I, F>,
    uri: Option<SharedString>,
    grayscale: bool,
}

pub fn img<V: 'static>() -> Img<V, StatelessInteraction<V>, FocusDisabled> {
    Img {
        base: div(),
        uri: None,
        grayscale: false,
    }
}

impl<V, I, F> Img<V, I, F>
where
    V: 'static,
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
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

impl<V, F> Img<V, StatelessInteraction<V>, F>
where
    F: ElementFocus<V>,
{
    pub fn id(self, id: impl Into<ElementId>) -> Img<V, StatefulInteraction<V>, F> {
        Img {
            base: self.base.id(id),
            uri: self.uri,
            grayscale: self.grayscale,
        }
    }
}

impl<V, I, F> IntoAnyElement<V> for Img<V, I, F>
where
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, I, F> Element<V> for Img<V, I, F>
where
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
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

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) {
        cx.stack(0, |cx| {
            self.base.paint(bounds, view, element_state, cx);
        });

        let style = self.base.compute_style(bounds, element_state, cx);
        let corner_radii = style.corner_radii;

        if let Some(uri) = self.uri.clone() {
            let image_future = cx.image_cache.get(uri);
            if let Some(data) = image_future
                .clone()
                .now_or_never()
                .and_then(ResultExt::log_err)
            {
                let corner_radii = corner_radii.to_pixels(bounds.size, cx.rem_size());
                cx.stack(1, |cx| {
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
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
{
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<V, I, F> StatelessInteractive<V> for Img<V, I, F>
where
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
{
    fn stateless_interaction(&mut self) -> &mut StatelessInteraction<V> {
        self.base.stateless_interaction()
    }
}

impl<V, F> StatefulInteractive<V> for Img<V, StatefulInteraction<V>, F>
where
    F: ElementFocus<V>,
{
    fn stateful_interaction(&mut self) -> &mut StatefulInteraction<V> {
        self.base.stateful_interaction()
    }
}

impl<V, I> Focusable<V> for Img<V, I, FocusEnabled<V>>
where
    V: 'static,
    I: ElementInteraction<V>,
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
