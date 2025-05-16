use gpui::{
    AnyElement, App, Bounds, Div, DivFrameState, Element, ElementId, GlobalElementId, Hitbox,
    InteractiveElement as _, IntoElement, LayoutId, ParentElement, Pixels, StyleRefinement, Styled,
    Window, div,
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
    ///
    /// [`Interactivity::occlude_mouse`]: gpui::Interactivity::occlude_mouse
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
    type DebugState = ();

    fn id(&self) -> Option<ElementId> {
        Element::id(&self.div)
    }

    fn source(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        _debug_state: &mut Option<Self::DebugState>,
        window: &mut Window,
        cx: &mut App,
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
        _debug_state: &mut Option<Self::DebugState>,
        window: &mut Window,
        cx: &mut App,
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
        _debug_state: &mut Option<Self::DebugState>,
        window: &mut Window,
        cx: &mut App,
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
