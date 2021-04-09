use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json::{self, json, ToJson},
    AfterLayoutContext, DebugContext, Element, ElementBox, Event, EventContext, LayoutContext,
    PaintContext, SizeConstraint,
};

pub struct Stack {
    children: Vec<ElementBox>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            children: Vec::new(),
        }
    }
}

impl Element for Stack {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let mut size = constraint.min;
        for child in &mut self.children {
            size = size.max(child.layout(constraint, ctx));
        }
        (size, ())
    }

    fn after_layout(
        &mut self,
        _: Vector2F,
        _: &mut Self::LayoutState,
        ctx: &mut AfterLayoutContext,
    ) {
        for child in &mut self.children {
            child.after_layout(ctx);
        }
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: &mut Self::LayoutState,
        ctx: &mut PaintContext,
    ) -> Self::PaintState {
        for child in &mut self.children {
            ctx.scene.push_layer(None);
            child.paint(bounds.origin(), ctx);
            ctx.scene.pop_layer();
        }
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        ctx: &mut EventContext,
    ) -> bool {
        for child in self.children.iter_mut().rev() {
            if child.dispatch_event(event, ctx) {
                return true;
            }
        }
        false
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        ctx: &DebugContext,
    ) -> json::Value {
        json!({
            "type": "Stack",
            "bounds": bounds.to_json(),
            "children": self.children.iter().map(|child| child.debug(ctx)).collect::<Vec<json::Value>>()
        })
    }
}

impl Extend<ElementBox> for Stack {
    fn extend<T: IntoIterator<Item = ElementBox>>(&mut self, children: T) {
        self.children.extend(children)
    }
}
