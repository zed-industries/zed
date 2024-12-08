use gpui::{
    div, AnyElement, Bounds, Div, DivFrameState, Element, ElementId, GlobalElementId, Hitbox,
    IntoElement, LayoutId, ParentElement, Pixels, StyleRefinement, Styled,
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
        self.div.id()
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        window.with_rem_size(Some(self.rem_size), |window| {
            self.div.request_layout(id, window, cx)
        })
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> Self::PrepaintState {
        window.with_rem_size(Some(self.rem_size), |window| {
            self.div.prepaint(id, bounds, request_layout, window, cx)
        })
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) {
        window.with_rem_size(Some(self.rem_size), |window| {
            self.div
                .paint(id, bounds, request_layout, prepaint, window, cx)
        })
    }
}

impl IntoElement for WithRemSize {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
