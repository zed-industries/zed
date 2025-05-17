#![allow(missing_docs)]
use super::{FocusableElement, InteractiveElement, Interactivity, StatefulInteractiveElement};
use crate::{
    AbsoluteLength, AnyElement, App, BorderStyle, ClickEvent, CornersRefinement, Edges,
    EdgesRefinement, Element, ElementId, GlobalElementId, Hitbox, IntoElement, LayoutId,
    ParentElement, SharedString, StyleRefinement, Styled, TextStyleRefinement, Window,
    colors::Colors, px,
};
use refineable::Refineable;
use smallvec::SmallVec;
use taffy::FlexDirection;
use util::default;

pub fn button(id: impl Into<ElementId>) -> Button {
    Button {
        id: id.into(),
        interactivity: Interactivity::default(),
        children: SmallVec::new(),
    }
}

pub struct Button {
    id: ElementId,
    interactivity: Interactivity,
    children: SmallVec<[AnyElement; 2]>,
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
                    let mut child_layout_ids = Vec::new();
                    for child in &mut self.children {
                        let child_layout_id = child.request_layout(window, cx);
                        child_layout_ids.push(child_layout_id);
                    }
                    window.request_layout(style, child_layout_ids, cx)
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

        // Prepaint children
        for child in &mut self.children {
            child.prepaint(window, cx);
        }

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
        let colors = Colors::for_appearance(window);
        let text_style = self.text_style().clone();
        let abs_px = |number: f32| AbsoluteLength::Pixels(px(number));

        let mut text_style = if let Some(style) = text_style {
            style.clone()
        } else {
            TextStyleRefinement::default()
        };

        let new_style = StyleRefinement {
            background: Some(colors.container.into()),
            text: Some(TextStyleRefinement {
                color: Some(colors.text.into()),
                font_size: Some(px(13.).into()),
                ..text_style
            }),
            border_color: Some(colors.border.into()),
            border_style: Some(BorderStyle::Solid),
            border_widths: EdgesRefinement {
                top: Some(abs_px(1.)),
                right: Some(abs_px(1.)),
                bottom: Some(abs_px(1.)),
                left: Some(abs_px(1.)),
            },
            corner_radii: CornersRefinement {
                top_left: Some(abs_px(4.)),
                top_right: Some(abs_px(4.)),
                bottom_left: Some(abs_px(4.)),
                bottom_right: Some(abs_px(4.)),
            },
            flex_direction: Some(FlexDirection::Row),
            flex_grow: Some(0.),
            ..StyleRefinement::default()
        };

        let refined = self.style().refine(&new_style);

        self.interactivity.paint(
            global_id,
            bounds,
            hitbox.as_ref(),
            window,
            cx,
            |style, window, cx| {
                for child in &mut self.children {
                    child.paint(window, cx);
                }
            },
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

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Button {
    pub fn on_click(
        mut self,
        callback: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    ) -> Self {
        self.interactivity
            .on_click(move |event, window, cx| callback(event, window, cx));
        self
    }
}
