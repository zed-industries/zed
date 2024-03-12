use crate::{AnyElement, Bounds, Element, ElementContext, IntoElement, LayoutId, Pixels};

/// Builds a `Deferred` element, which delays the layout and paint of its child.
pub fn deferred(child: impl IntoElement) -> Deferred {
    Deferred {
        child: Some(child.into_any_element()),
        priority: 0,
    }
}

/// An element which delays the painting of its child until after all of
/// its ancestors, while keeping its layout as part of the current element tree.
pub struct Deferred {
    child: Option<AnyElement>,
    priority: usize,
}

impl Element for Deferred {
    type BeforeLayout = ();
    type AfterLayout = ();

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, ()) {
        let layout_id = self.child.as_mut().unwrap().before_layout(cx);
        (layout_id, ())
    }

    fn after_layout(
        &mut self,
        _bounds: Bounds<Pixels>,
        _before_layout: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) {
        let child = self.child.take().unwrap();
        let element_offset = cx.element_offset();
        cx.defer_draw(child, element_offset, self.priority)
    }

    fn paint(
        &mut self,
        _bounds: Bounds<Pixels>,
        _before_layout: &mut Self::BeforeLayout,
        _after_layout: &mut Self::AfterLayout,
        _cx: &mut ElementContext,
    ) {
    }
}

impl IntoElement for Deferred {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Deferred {
    /// Sets a priority for the element. A higher priority conceptually means painting the element
    /// on top of deferred draws with a lower priority (i.e. closer to the viewer).
    pub fn priority(mut self, priority: usize) -> Self {
        self.priority = priority;
        self
    }
}
