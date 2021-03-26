use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    AfterLayoutContext, Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};
use core::panic;
use replace_with::replace_with_or_abort;
use std::any::Any;

trait AnyElement {
    fn layout(&mut self, constraint: SizeConstraint, ctx: &mut LayoutContext) -> Vector2F;
    fn after_layout(&mut self, _: &mut AfterLayoutContext) {}
    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext);
    fn dispatch_event(&mut self, event: &Event, ctx: &mut EventContext) -> bool;

    fn size(&self) -> Vector2F;
    fn metadata(&self) -> Option<&dyn Any>;
}

pub trait Element {
    type LayoutState;
    type PaintState;

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState);

    fn after_layout(
        &mut self,
        size: Vector2F,
        layout: &mut Self::LayoutState,
        ctx: &mut AfterLayoutContext,
    );

    fn paint(
        &mut self,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        ctx: &mut PaintContext,
    ) -> Self::PaintState;

    fn dispatch_event(
        &mut self,
        event: &Event,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        paint: &mut Self::PaintState,
        ctx: &mut EventContext,
    ) -> bool;

    fn metadata(&self) -> Option<&dyn Any> {
        None
    }

    fn boxed(self) -> ElementBox
    where
        Self: 'static + Sized,
    {
        ElementBox(Box::new(Lifecycle::Init { element: self }))
    }
}

pub enum Lifecycle<T: Element> {
    Init {
        element: T,
    },
    PostLayout {
        element: T,
        size: Vector2F,
        layout: T::LayoutState,
    },
    PostPaint {
        element: T,
        bounds: RectF,
        layout: T::LayoutState,
        paint: T::PaintState,
    },
}
pub struct ElementBox(Box<dyn AnyElement>);

impl<T: Element> AnyElement for Lifecycle<T> {
    fn layout(&mut self, constraint: SizeConstraint, ctx: &mut LayoutContext) -> Vector2F {
        let mut result = None;
        replace_with_or_abort(self, |me| match me {
            Lifecycle::Init { mut element }
            | Lifecycle::PostLayout { mut element, .. }
            | Lifecycle::PostPaint { mut element, .. } => {
                let (size, layout) = element.layout(constraint, ctx);
                debug_assert!(size.x().is_finite());
                debug_assert!(size.y().is_finite());

                result = Some(size);
                Lifecycle::PostLayout {
                    element,
                    size,
                    layout,
                }
            }
        });
        result.unwrap()
    }

    fn after_layout(&mut self, ctx: &mut AfterLayoutContext) {
        if let Lifecycle::PostLayout {
            element,
            size,
            layout,
        } = self
        {
            element.after_layout(*size, layout, ctx);
        } else {
            panic!("invalid element lifecycle state");
        }
    }

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext) {
        replace_with_or_abort(self, |me| {
            if let Lifecycle::PostLayout {
                mut element,
                size,
                mut layout,
            } = me
            {
                let bounds = RectF::new(origin, size);
                let paint = element.paint(bounds, &mut layout, ctx);
                Lifecycle::PostPaint {
                    element,
                    bounds,
                    layout,
                    paint,
                }
            } else {
                panic!("invalid element lifecycle state");
            }
        });
    }

    fn dispatch_event(&mut self, event: &Event, ctx: &mut EventContext) -> bool {
        if let Lifecycle::PostPaint {
            element,
            bounds,
            layout,
            paint,
        } = self
        {
            element.dispatch_event(event, *bounds, layout, paint, ctx)
        } else {
            panic!("invalid element lifecycle state");
        }
    }

    fn size(&self) -> Vector2F {
        match self {
            Lifecycle::Init { .. } => panic!("invalid element lifecycle state"),
            Lifecycle::PostLayout { size, .. } => *size,
            Lifecycle::PostPaint { bounds, .. } => bounds.size(),
        }
    }

    fn metadata(&self) -> Option<&dyn Any> {
        match self {
            Lifecycle::Init { element }
            | Lifecycle::PostLayout { element, .. }
            | Lifecycle::PostPaint { element, .. } => element.metadata(),
        }
    }
}

impl ElementBox {
    pub fn layout(&mut self, constraint: SizeConstraint, ctx: &mut LayoutContext) -> Vector2F {
        self.0.layout(constraint, ctx)
    }

    pub fn after_layout(&mut self, ctx: &mut AfterLayoutContext) {
        self.0.after_layout(ctx);
    }

    pub fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext) {
        self.0.paint(origin, ctx);
    }

    pub fn dispatch_event(&mut self, event: &Event, ctx: &mut EventContext) -> bool {
        self.0.dispatch_event(event, ctx)
    }

    pub fn size(&self) -> Vector2F {
        self.0.size()
    }

    pub fn metadata(&self) -> Option<&dyn Any> {
        self.0.metadata()
    }
}
