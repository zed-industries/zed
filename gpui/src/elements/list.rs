use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    sum_tree::{self, SumTree},
    Element,
};
use parking_lot::Mutex;
use std::sync::Arc;

use crate::ElementBox;

pub struct List {
    state: ListState,
}

pub struct ListState(Arc<Mutex<StateInner>>);

struct StateInner {
    elements: Vec<ElementBox>,
    heights: SumTree<ElementHeight>,
}

#[derive(Clone, Debug)]
enum ElementHeight {
    Pending,
    Ready(f32),
}

#[derive(Clone, Debug, Default)]
struct ElementHeightSummary {
    pending_count: usize,
    height: f32,
}

impl Element for List {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: crate::SizeConstraint,
        cx: &mut crate::LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        todo!()
    }

    fn after_layout(
        &mut self,
        size: Vector2F,
        layout: &mut Self::LayoutState,
        cx: &mut crate::AfterLayoutContext,
    ) {
        todo!()
    }

    fn paint(
        &mut self,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut crate::PaintContext,
    ) -> Self::PaintState {
        todo!()
    }

    fn dispatch_event(
        &mut self,
        event: &crate::Event,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        paint: &mut Self::PaintState,
        cx: &mut crate::EventContext,
    ) -> bool {
        todo!()
    }

    fn debug(
        &self,
        bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        cx: &crate::DebugContext,
    ) -> serde_json::Value {
        todo!()
    }
}

impl ListState {
    pub fn new(elements: Vec<ElementBox>) -> Self {
        let mut heights = SumTree::new();
        heights.extend(elements.iter().map(|_| ElementHeight::Pending), &());
        Self(Arc::new(Mutex::new(StateInner { elements, heights })))
    }
}

impl sum_tree::Item for ElementHeight {
    type Summary = ElementHeightSummary;

    fn summary(&self) -> Self::Summary {
        todo!()
    }
}

impl sum_tree::Summary for ElementHeightSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, cx: &Self::Context) {
        self.pending_count += summary.pending_count;
        self.height += summary.height;
    }
}
