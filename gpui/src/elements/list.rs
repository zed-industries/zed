use crate::sum_tree::{self, SumTree};
use parking_lot::Mutex;
use std::sync::Arc;

use crate::ElementBox;

pub struct List {
    state: ListState,
}

pub struct ListState(Arc<Mutex<StateInner>>);

struct StateInner {
    elements: Vec<ElementBox>,
    element_heights: SumTree<ElementHeight>,
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
