use super::{
    AfterLayoutContext, Element, Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{self, json},
    AppContext, ElementBox,
};
use json::ToJson;
use parking_lot::Mutex;
use std::{cmp, ops::Range, sync::Arc};

#[derive(Clone, Default)]
pub struct UniformListState(Arc<Mutex<StateInner>>);

impl UniformListState {
    pub fn scroll_to(&self, item_ix: usize) {
        self.0.lock().scroll_to = Some(item_ix);
    }

    pub fn scroll_top(&self) -> f32 {
        self.0.lock().scroll_top
    }
}

#[derive(Default)]
struct StateInner {
    scroll_top: f32,
    scroll_to: Option<usize>,
}

pub struct LayoutState {
    scroll_max: f32,
    item_height: f32,
    items: Vec<ElementBox>,
}

pub struct UniformList<F>
where
    F: Fn(Range<usize>, &mut Vec<ElementBox>, &AppContext),
{
    state: UniformListState,
    item_count: usize,
    append_items: F,
}

impl<F> UniformList<F>
where
    F: Fn(Range<usize>, &mut Vec<ElementBox>, &AppContext),
{
    pub fn new(state: UniformListState, item_count: usize, append_items: F) -> Self {
        Self {
            state,
            item_count,
            append_items,
        }
    }

    fn scroll(
        &self,
        _: Vector2F,
        delta: Vector2F,
        precise: bool,
        scroll_max: f32,
        cx: &mut EventContext,
    ) -> bool {
        if !precise {
            todo!("still need to handle non-precise scroll events from a mouse wheel");
        }

        let mut state = self.state.0.lock();
        state.scroll_top = (state.scroll_top - delta.y()).max(0.0).min(scroll_max);
        cx.notify();

        true
    }

    fn autoscroll(&mut self, scroll_max: f32, list_height: f32, item_height: f32) {
        let mut state = self.state.0.lock();

        if state.scroll_top > scroll_max {
            state.scroll_top = scroll_max;
        }

        if let Some(item_ix) = state.scroll_to.take() {
            let item_top = item_ix as f32 * item_height;
            let item_bottom = item_top + item_height;

            if item_top < state.scroll_top {
                state.scroll_top = item_top;
            } else if item_bottom > (state.scroll_top + list_height) {
                state.scroll_top = item_bottom - list_height;
            }
        }
    }

    fn scroll_top(&self) -> f32 {
        self.state.0.lock().scroll_top
    }
}

impl<F> Element for UniformList<F>
where
    F: Fn(Range<usize>, &mut Vec<ElementBox>, &AppContext),
{
    type LayoutState = LayoutState;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        if constraint.max.y().is_infinite() {
            unimplemented!(
                "UniformList does not support being rendered with an unconstrained height"
            );
        }
        let mut size = constraint.max;
        let mut item_constraint =
            SizeConstraint::new(vec2f(size.x(), 0.0), vec2f(size.x(), f32::INFINITY));
        let mut item_height = 0.;
        let mut scroll_max = 0.;

        let mut items = Vec::new();
        (self.append_items)(0..1, &mut items, cx.app);
        if let Some(first_item) = items.first_mut() {
            let mut item_size = first_item.layout(item_constraint, cx);
            item_size.set_x(size.x());
            item_constraint.min = item_size;
            item_constraint.max = item_size;
            item_height = item_size.y();

            let scroll_height = self.item_count as f32 * item_height;
            if scroll_height < size.y() {
                size.set_y(size.y().min(scroll_height).max(constraint.min.y()));
            }

            scroll_max = item_height * self.item_count as f32 - size.y();
            self.autoscroll(scroll_max, size.y(), item_height);

            items.clear();
            let start = cmp::min((self.scroll_top() / item_height) as usize, self.item_count);
            let end = cmp::min(
                self.item_count,
                start + (size.y() / item_height).ceil() as usize + 1,
            );
            (self.append_items)(start..end, &mut items, cx.app);
            for item in &mut items {
                item.layout(item_constraint, cx);
            }
        }

        (
            size,
            LayoutState {
                item_height,
                scroll_max,
                items,
            },
        )
    }

    fn after_layout(
        &mut self,
        _: Vector2F,
        layout: &mut Self::LayoutState,
        cx: &mut AfterLayoutContext,
    ) {
        for item in &mut layout.items {
            item.after_layout(cx);
        }
    }

    fn paint(
        &mut self,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        cx.scene.push_layer(Some(bounds));

        let mut item_origin =
            bounds.origin() - vec2f(0.0, self.state.scroll_top() % layout.item_height);

        for item in &mut layout.items {
            item.paint(item_origin, cx);
            item_origin += vec2f(0.0, layout.item_height);
        }

        cx.scene.pop_layer();
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        let mut handled = false;
        for item in &mut layout.items {
            handled = item.dispatch_event(event, cx) || handled;
        }

        match event {
            Event::ScrollWheel {
                position,
                delta,
                precise,
            } => {
                if bounds.contains_point(*position) {
                    if self.scroll(*position, *delta, *precise, layout.scroll_max, cx) {
                        handled = true;
                    }
                }
            }
            _ => {}
        }

        handled
    }

    fn debug(
        &self,
        bounds: RectF,
        layout: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &crate::DebugContext,
    ) -> json::Value {
        json!({
            "type": "UniformList",
            "bounds": bounds.to_json(),
            "scroll_max": layout.scroll_max,
            "item_height": layout.item_height,
            "items": layout.items.iter().map(|item| item.debug(cx)).collect::<Vec<json::Value>>()

        })
    }
}
