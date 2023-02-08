use crate::{
    elements::*,
    fonts::TextStyle,
    geometry::{rect::RectF, vector::Vector2F},
    Action, ElementBox, LayoutContext, PaintContext, SizeConstraint,
};
use serde_json::json;

use super::ContainerStyle;

pub struct KeystrokeLabel {
    action: Box<dyn Action>,
    container_style: ContainerStyle,
    text_style: TextStyle,
    window_id: usize,
    view_id: usize,
}

impl KeystrokeLabel {
    pub fn new(
        window_id: usize,
        view_id: usize,
        action: Box<dyn Action>,
        container_style: ContainerStyle,
        text_style: TextStyle,
    ) -> Self {
        Self {
            window_id,
            view_id,
            action,
            container_style,
            text_style,
        }
    }
}

impl Element for KeystrokeLabel {
    type LayoutState = ElementBox;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, ElementBox) {
        let mut element = if let Some(keystrokes) =
            cx.app
                .keystrokes_for_action(self.window_id, self.view_id, self.action.as_ref())
        {
            Flex::row()
                .with_children(keystrokes.iter().map(|keystroke| {
                    Label::new(keystroke.to_string(), self.text_style.clone())
                        .contained()
                        .with_style(self.container_style)
                        .boxed()
                }))
                .boxed()
        } else {
            Empty::new().collapsed().boxed()
        };

        let size = element.layout(constraint, cx);
        (size, element)
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        element: &mut ElementBox,
        cx: &mut PaintContext,
    ) {
        element.paint(bounds.origin(), visible_bounds, cx);
    }

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &MeasurementContext,
    ) -> Option<RectF> {
        None
    }

    fn debug(
        &self,
        _: RectF,
        element: &ElementBox,
        _: &(),
        cx: &crate::DebugContext,
    ) -> serde_json::Value {
        json!({
            "type": "KeystrokeLabel",
            "action": self.action.name(),
            "child": element.debug(cx)
        })
    }
}
