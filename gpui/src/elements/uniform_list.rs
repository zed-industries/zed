use super::{
    try_rect, AfterLayoutContext, AppContext, Element, Event, EventContext, LayoutContext,
    MutableAppContext, PaintContext, SizeConstraint,
};
use crate::geometry::{
    rect::RectF,
    vector::{vec2f, Vector2F},
};
use parking_lot::Mutex;
use std::{cmp, ops::Range, sync::Arc};

#[derive(Clone)]
pub struct UniformListState(Arc<Mutex<StateInner>>);

struct StateInner {
    scroll_top: f32,
    scroll_to: Option<usize>,
}

impl UniformListState {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(StateInner {
            scroll_top: 0.0,
            scroll_to: None,
        })))
    }

    pub fn scroll_to(&self, item_ix: usize) {
        self.0.lock().scroll_to = Some(item_ix);
    }
}

pub struct UniformList<F, G>
where
    F: Fn(Range<usize>, &AppContext) -> G,
    G: Iterator<Item = Box<dyn Element>>,
{
    state: UniformListState,
    item_count: usize,
    build_items: F,
    scroll_max: Option<f32>,
    items: Vec<Box<dyn Element>>,
    origin: Option<Vector2F>,
    size: Option<Vector2F>,
}

impl<F, G> UniformList<F, G>
where
    F: Fn(Range<usize>, &AppContext) -> G,
    G: Iterator<Item = Box<dyn Element>>,
{
    pub fn new(state: UniformListState, item_count: usize, build_items: F) -> Self {
        Self {
            state,
            item_count,
            build_items,
            scroll_max: None,
            items: Default::default(),
            origin: None,
            size: None,
        }
    }

    fn scroll(
        &self,
        position: Vector2F,
        delta: Vector2F,
        precise: bool,
        ctx: &mut EventContext,
        _: &AppContext,
    ) -> bool {
        if !self.rect().unwrap().contains_point(position) {
            return false;
        }

        if !precise {
            todo!("still need to handle non-precise scroll events from a mouse wheel");
        }

        let mut state = self.state.0.lock();
        state.scroll_top = (state.scroll_top - delta.y())
            .max(0.0)
            .min(self.scroll_max.unwrap());
        ctx.dispatch_action("uniform_list:scroll", state.scroll_top);

        true
    }

    fn autoscroll(&mut self, list_height: f32, item_height: f32) {
        let mut state = self.state.0.lock();

        let scroll_max = self.item_count as f32 * item_height - list_height;
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

    fn rect(&self) -> Option<RectF> {
        try_rect(self.origin, self.size)
    }
}

impl<F, G> Element for UniformList<F, G>
where
    F: Fn(Range<usize>, &AppContext) -> G,
    G: Iterator<Item = Box<dyn Element>>,
{
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F {
        if constraint.max.y().is_infinite() {
            unimplemented!(
                "UniformList does not support being rendered with an unconstrained height"
            );
        }
        let mut size = constraint.max;
        let mut item_constraint =
            SizeConstraint::new(vec2f(size.x(), 0.0), vec2f(size.x(), f32::INFINITY));

        let first_item = (self.build_items)(0..1, app).next();
        if let Some(mut first_item) = first_item {
            let mut item_size = first_item.layout(item_constraint, ctx, app);
            item_size.set_x(size.x());
            item_constraint.min = item_size;
            item_constraint.max = item_size;

            let scroll_height = self.item_count as f32 * item_size.y();
            if scroll_height < size.y() {
                size.set_y(size.y().min(scroll_height).max(constraint.min.y()));
            }

            self.autoscroll(size.y(), item_size.y());

            let start = cmp::min(
                (self.scroll_top() / item_size.y()) as usize,
                self.item_count,
            );
            let end = cmp::min(
                self.item_count,
                start + (size.y() / item_size.y()).ceil() as usize + 1,
            );
            self.items.clear();
            self.items.extend((self.build_items)(start..end, app));

            self.scroll_max = Some(item_size.y() * self.item_count as f32 - size.y());

            for item in &mut self.items {
                item.layout(item_constraint, ctx, app);
            }
        }

        self.size = Some(size);
        size
    }

    fn after_layout(&mut self, ctx: &mut AfterLayoutContext, app: &mut MutableAppContext) {
        for item in &mut self.items {
            item.after_layout(ctx, app);
        }
    }

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
        // self.origin = Some(origin);

        // if let Some(item) = self.items.first() {
        //     ctx.canvas.save();
        //     let mut clip_path = Path2D::new();
        //     clip_path.rect(RectF::new(origin, self.size.unwrap()));
        //     ctx.canvas.clip_path(clip_path, FillRule::Winding);

        //     let item_height = item.size().unwrap().y();
        //     let mut item_origin = origin - vec2f(0.0, self.state.0.lock().scroll_top % item_height);
        //     for item in &mut self.items {
        //         item.paint(item_origin, ctx, app);
        //         item_origin += vec2f(0.0, item_height);
        //     }
        //     ctx.canvas.restore();
        // }
    }

    fn size(&self) -> Option<Vector2F> {
        self.size
    }

    fn dispatch_event(&self, event: &Event, ctx: &mut EventContext, app: &AppContext) -> bool {
        let mut handled = false;
        for item in &self.items {
            if item.dispatch_event(event, ctx, app) {
                handled = true;
            }
        }

        match event {
            Event::ScrollWheel {
                position,
                delta,
                precise,
            } => {
                if self.scroll(*position, *delta, *precise, ctx, app) {
                    handled = true;
                }
            }
            _ => {}
        }

        handled
    }
}
