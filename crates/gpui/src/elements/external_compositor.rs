use crate::{
    App, Background, Bounds, Element, ElementId, ExternalSlotHandle, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, Length, Pixels, Style, StyleRefinement, Styled,
    Window, fill, px,
};
use refineable::Refineable;

/// An element that composites the content of an [`ExternalSlotHandle`] (see
/// [`crate::ExternalCompositorRegistry`]) into the scene.
///
/// The render backend resolves the handle and draws its content at a controlled
/// point in the frame. There is no `object_fit` support yet.
///
/// Defaults to the slot's native resolution (converted from device texels to
/// logical pixels via the window's scale factor) unless an explicit `w`/`h` is set
/// via [`Styled`]; if there is no registry (e.g. macOS/Metal in this phase) or the
/// handle isn't registered, defaults to zero size instead of guessing.
pub struct ExternalCompositorElement {
    handle: ExternalSlotHandle,
    background: Option<Background>,
    style: StyleRefinement,
}

/// Create a new external compositor element for the given slot handle.
pub fn external_compositor(handle: ExternalSlotHandle) -> ExternalCompositorElement {
    ExternalCompositorElement {
        handle,
        background: None,
        style: Default::default(),
    }
}

impl ExternalCompositorElement {
    /// Sets a color painted behind the composited texture, at the same bounds,
    /// before the external compositor primitive: visible while the compositor is
    /// not ready, on backends without composition support, or showing through
    /// under translucent content.
    pub fn background(mut self, color: impl Into<Background>) -> Self {
        self.background = Some(color.into());
        self
    }
}

impl Element for ExternalCompositorElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.refine(&self.style);

        // Default to the slot's native resolution, the same way `Img` defaults to
        // an image's natural size (see `elements/img.rs`): only when the caller
        // hasn't set an explicit `w`/`h` (`Length::Auto`), so an explicit style
        // always wins. With no registry (e.g. macOS/Metal in this phase) or an
        // unregistered handle, there's no size to infer, so default to zero rather
        // than guess.
        let slot_size = window
            .external_compositor_registry()
            .and_then(|registry| registry.borrow().slot_size(self.handle))
            .map(|size| size.to_pixels(window.scale_factor()));

        if let Length::Auto = style.size.width {
            style.size.width =
                Length::Definite(slot_size.map(|size| size.width).unwrap_or(px(0.)).into());
        }
        if let Length::Auto = style.size.height {
            style.size.height =
                Length::Definite(slot_size.map(|size| size.height).unwrap_or(px(0.)).into());
        }

        let layout_id = window.request_layout(style, [], cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        _: &mut App,
    ) {
        // Paint the background first, at the same bounds, before the external
        // compositor primitive: this gives backends without composition support
        // (e.g. macOS/Metal in this phase), or a slot with no frame ready yet,
        // something to show instead of a transparent hole. Backends that do
        // composite this frame draw over it (the render pass uses `Load`, not
        // `Clear`, for this primitive), so translucent content painted above this
        // element still shows the background through, by design (see
        // `Self::background`).
        if let Some(color) = self.background {
            window.paint_quad(fill(bounds, color));
        }

        window.paint_external_compositor(bounds, self.handle);
    }
}

impl IntoElement for ExternalCompositorElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for ExternalCompositorElement {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
