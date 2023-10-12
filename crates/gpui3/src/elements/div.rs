use crate::{
    AnyElement, BorrowWindow, Bounds, Cascade, Element, ElementId, IdentifiedElement, Interactive,
    LayoutId, MouseEventListeners, Overflow, ParentElement, Pixels, Point, Refineable, Style,
    Styled, ViewContext,
};
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::{marker::PhantomData, sync::Arc};

pub enum HasId {}

pub struct Div<S: 'static, I = ()> {
    styles: Cascade<Style>,
    id: Option<ElementId>,
    listeners: MouseEventListeners<S>,
    children: SmallVec<[AnyElement<S>; 2]>,
    scroll_state: Option<ScrollState>,
    identified: PhantomData<I>,
}

pub fn div<S>() -> Div<S> {
    Div {
        styles: Default::default(),
        id: None,
        listeners: Default::default(),
        children: Default::default(),
        scroll_state: None,
        identified: PhantomData,
    }
}

impl<S: 'static + Send + Sync, Marker: 'static + Send + Sync> Element for Div<S, Marker> {
    type ViewState = S;
    type ElementState = ();

    fn element_id(&self) -> Option<ElementId> {
        self.id.clone()
    }

    fn layout(
        &mut self,
        view: &mut S,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<S>,
    ) -> (LayoutId, Self::ElementState) {
        let style = self.computed_style();
        let child_layout_ids = style.apply_text_style(cx, |cx| {
            self.with_element_id(cx, |this, cx| this.layout_children(view, cx))
        });
        let layout_id = cx.request_layout(style.into(), child_layout_ids.clone());
        (layout_id, ())
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut S,
        _: &mut (),
        cx: &mut ViewContext<S>,
    ) {
        let style = self.computed_style();
        cx.stack(0, |cx| style.paint(bounds, cx));

        let overflow = &style.overflow;

        style.apply_text_style(cx, |cx| {
            cx.stack(1, |cx| {
                style.apply_overflow(bounds, cx, |cx| {
                    self.with_element_id(cx, |this, cx| {
                        this.listeners.paint(bounds, cx);
                        this.paint_children(overflow, state, cx)
                    });
                })
            })
        });
    }
}

impl<S> Div<S, ()>
where
    S: 'static + Send + Sync,
{
    pub fn id(self, id: impl Into<ElementId>) -> Div<S, HasId> {
        Div {
            styles: self.styles,
            id: Some(id.into()),
            listeners: self.listeners,
            children: self.children,
            scroll_state: self.scroll_state,
            identified: PhantomData,
        }
    }
}

impl<S, Marker> Div<S, Marker>
where
    S: 'static + Send + Sync,
    Marker: 'static + Send + Sync,
{
    pub fn overflow_hidden(mut self) -> Self {
        self.declared_style().overflow.x = Some(Overflow::Hidden);
        self.declared_style().overflow.y = Some(Overflow::Hidden);
        self
    }

    pub fn overflow_hidden_x(mut self) -> Self {
        self.declared_style().overflow.x = Some(Overflow::Hidden);
        self
    }

    pub fn overflow_hidden_y(mut self) -> Self {
        self.declared_style().overflow.y = Some(Overflow::Hidden);
        self
    }

    pub fn overflow_scroll(mut self, scroll_state: ScrollState) -> Self {
        self.scroll_state = Some(scroll_state);
        self.declared_style().overflow.x = Some(Overflow::Scroll);
        self.declared_style().overflow.y = Some(Overflow::Scroll);
        self
    }

    pub fn overflow_x_scroll(mut self, scroll_state: ScrollState) -> Self {
        self.scroll_state = Some(scroll_state);
        self.declared_style().overflow.x = Some(Overflow::Scroll);
        self
    }

    pub fn overflow_y_scroll(mut self, scroll_state: ScrollState) -> Self {
        self.scroll_state = Some(scroll_state);
        self.declared_style().overflow.y = Some(Overflow::Scroll);
        self
    }

    fn scroll_offset(&self, overflow: &Point<Overflow>) -> Point<Pixels> {
        let mut offset = Point::default();
        if overflow.y == Overflow::Scroll {
            offset.y = self.scroll_state.as_ref().unwrap().y();
        }
        if overflow.x == Overflow::Scroll {
            offset.x = self.scroll_state.as_ref().unwrap().x();
        }

        offset
    }

    fn layout_children(&mut self, view: &mut S, cx: &mut ViewContext<S>) -> Vec<LayoutId> {
        self.children
            .iter_mut()
            .map(|child| child.layout(view, cx))
            .collect()
    }

    fn paint_children(
        &mut self,
        overflow: &Point<Overflow>,
        state: &mut S,
        cx: &mut ViewContext<S>,
    ) {
        let scroll_offset = self.scroll_offset(overflow);
        for child in &mut self.children {
            child.paint(state, Some(scroll_offset), cx);
        }
    }

    fn with_element_id<R>(
        &mut self,
        cx: &mut ViewContext<S>,
        f: impl FnOnce(&mut Self, &mut ViewContext<S>) -> R,
    ) -> R {
        if let Some(element_id) = self.element_id() {
            cx.with_element_id(element_id, |cx| f(self, cx))
        } else {
            f(self, cx)
        }
    }
}

impl<V: 'static + Send + Sync, Marker: 'static + Send + Sync> Styled for Div<V, Marker> {
    type Style = Style;

    fn style_cascade(&mut self) -> &mut Cascade<Self::Style> {
        &mut self.styles
    }

    fn declared_style(&mut self) -> &mut <Self::Style as Refineable>::Refinement {
        self.styles.base()
    }
}

impl<V: Send + Sync + 'static> IdentifiedElement for Div<V, HasId> {}

impl<V: Send + Sync + 'static> Interactive<V> for Div<V, HasId> {
    fn listeners(&mut self) -> &mut MouseEventListeners<V> {
        &mut self.listeners
    }
}

impl<V: 'static, Marker: 'static + Send + Sync> ParentElement for Div<V, Marker> {
    type State = V;

    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        &mut self.children
    }
}

#[derive(Default, Clone)]
pub struct ScrollState(Arc<Mutex<Point<Pixels>>>);

impl ScrollState {
    pub fn x(&self) -> Pixels {
        self.0.lock().x
    }

    pub fn set_x(&self, value: Pixels) {
        self.0.lock().x = value;
    }

    pub fn y(&self) -> Pixels {
        self.0.lock().y
    }

    pub fn set_y(&self, value: Pixels) {
        self.0.lock().y = value;
    }
}
