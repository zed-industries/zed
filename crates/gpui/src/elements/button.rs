#![allow(missing_docs)]
use super::{FocusableElement, InteractiveElement, Interactivity, StatefulInteractiveElement};
use crate::{
    App, ClickEvent, Element, ElementId, GlobalElementId, Hitbox, IntoElement, LayoutId,
    SharedString, StyleRefinement, Styled, Window,
};

pub fn button(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Button {
    Button {
        id: id.into(),
        label: label.into(),
        interactivity: Interactivity::default(),
        disabled: false,
        on_click: None,
    }
}

pub struct Button {
    id: ElementId,
    label: SharedString,
    interactivity: Interactivity,
    disabled: bool, // todo: this could move into Interactivity
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Element for Button {
    type RequestLayoutState = ();
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        // Get a LayoutId, an identifier Taffy uses to indicate a unique layout element
        let layout_id =
            self.interactivity
                .request_layout(global_id, window, cx, |style, window, cx| {
                    window.request_layout(style, vec![], cx)
                });

        // Initialize the layout state
        let layout_state = ();

        (layout_id, layout_state)
    }

    fn prepaint(
        &mut self,
        global_id: Option<&crate::GlobalElementId>,
        bounds: crate::Bounds<crate::Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        if let Some(handle) = self.interactivity.scroll_anchor.as_ref() {
            *handle.last_origin.borrow_mut() = bounds.origin - window.element_offset();
        }
        let content_size = bounds.size;

        self.interactivity.prepaint(
            global_id,
            bounds,
            content_size,
            window,
            cx,
            |_style, _scroll_offset, hitbox, _window, _cx| hitbox,
        )
    }

    fn paint(
        &mut self,
        global_id: Option<&crate::GlobalElementId>,
        bounds: crate::Bounds<crate::Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        hitbox: &mut Option<Hitbox>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.interactivity.paint(
            global_id,
            bounds,
            hitbox.as_ref(),
            window,
            cx,
            |_style, _window, _cx| {},
        )
    }
}

impl IntoElement for Button {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}
impl StatefulInteractiveElement for Button {}
impl FocusableElement for Button {}
