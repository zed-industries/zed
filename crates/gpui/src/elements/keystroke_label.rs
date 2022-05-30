use crate::{
    elements::*,
    fonts::TextStyle,
    geometry::{rect::RectF, vector::Vector2F},
    Action, ElementBox, Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};
use serde_json::json;

use super::ContainerStyle;

pub struct KeystrokeLabel {
    action: Box<dyn Action>,
    container_style: ContainerStyle,
    text_style: TextStyle,
}

impl KeystrokeLabel {
    pub fn new(
        action: Box<dyn Action>,
        container_style: ContainerStyle,
        text_style: TextStyle,
    ) -> Self {
        Self {
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
        let mut element = if let Some(keystrokes) = cx.keystrokes_for_action(self.action.as_ref()) {
            Flex::row()
                .with_children(keystrokes.iter().map(|keystroke| {
                    Label::new(
                        keystroke.to_string().to_uppercase(),
                        self.text_style.clone(),
                    )
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

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: RectF,
        element: &mut ElementBox,
        _: &mut (),
        cx: &mut EventContext,
    ) -> bool {
        element.dispatch_event(event, cx)
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
