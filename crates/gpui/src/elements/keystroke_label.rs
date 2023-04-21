use crate::{
    elements::*,
    fonts::TextStyle,
    geometry::{rect::RectF, vector::Vector2F},
    Action, Element, SizeConstraint,
};
use serde_json::json;

use super::ContainerStyle;

pub struct KeystrokeLabel {
    action: Box<dyn Action>,
    container_style: ContainerStyle,
    text_style: TextStyle,
    view_id: usize,
}

impl KeystrokeLabel {
    pub fn new(
        view_id: usize,
        action: Box<dyn Action>,
        container_style: ContainerStyle,
        text_style: TextStyle,
    ) -> Self {
        Self {
            view_id,
            action,
            container_style,
            text_style,
        }
    }
}

impl<V: View> Drawable<V> for KeystrokeLabel {
    type LayoutState = Element<V>;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, Element<V>) {
        let mut element = if let Some(keystrokes) =
            cx.keystrokes_for_action(self.view_id, self.action.as_ref())
        {
            Flex::row()
                .with_children(keystrokes.iter().map(|keystroke| {
                    Label::new(keystroke.to_string(), self.text_style.clone())
                        .contained()
                        .with_style(self.container_style)
                }))
                .into_element()
        } else {
            Empty::new().collapsed().into_element()
        };

        let size = element.layout(constraint, view, cx);
        (size, element)
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        element: &mut Element<V>,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) {
        element.paint(scene, bounds.origin(), visible_bounds, view, cx);
    }

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &ViewContext<V>,
    ) -> Option<RectF> {
        None
    }

    fn debug(
        &self,
        _: RectF,
        element: &Element<V>,
        _: &(),
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        json!({
            "type": "KeystrokeLabel",
            "action": self.action.name(),
            "child": element.debug(view, cx)
        })
    }
}
