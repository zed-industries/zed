use gpui::{
    div, AnyElement, Bounds, Div, DivFrameState, Element, ElementId, GlobalElementId, Hitbox,
    InteractiveElement as _, IntoElement, LayoutId, ParentElement, Pixels, StyleRefinement, Styled,
    WindowContext,
};

/// An element that sets a particular rem size for its children.
pub struct WithRemSize {
    div: Div,
    rem_size: Pixels,
}

impl WithRemSize {
    /// Create a new [WithRemSize] element, which sets a
    /// particular rem size for its children.
    pub fn new(rem_size: impl Into<Pixels>) -> Self {
        Self {
            div: div(),
            rem_size: rem_size.into(),
        }
    }

    /// Block the mouse from interacting with this element or any of its children
    /// The fluent API equivalent to [`Interactivity::occlude_mouse`]
    pub fn occlude(mut self) -> Self {
        self.div = self.div.occlude();
        self
    }
}

impl Styled for WithRemSize {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}

impl ParentElement for WithRemSize {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.div.extend(elements)
    }
}

impl Element for WithRemSize {
    type RequestLayoutState = DivFrameState;
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> {
        Element::id(&self.div)
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        cx.with_rem_size(Some(self.rem_size), |cx| self.div.request_layout(id, cx))
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        cx.with_rem_size(Some(self.rem_size), |cx| {
            self.div.prepaint(id, bounds, request_layout, cx)
        })
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        cx.with_rem_size(Some(self.rem_size), |cx| {
            self.div.paint(id, bounds, request_layout, prepaint, cx)
        })
    }
}

impl IntoElement for WithRemSize {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
