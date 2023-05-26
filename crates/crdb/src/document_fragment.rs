use super::DocumentFragment;
use crate::Ordering;
use rope::TextSummary;

impl sum_tree::Item for DocumentFragment {
    type Summary = DocumentFragmentSummary;

    fn summary(&self) -> Self::Summary {
        todo!()
    }
}

#[derive(Clone, Default, Debug)]
pub struct DocumentFragmentSummary {
    size: TextSummary,
    max_id: Ordering,
}

impl sum_tree::Summary for DocumentFragmentSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, cxfg: &Self::Context) {
        todo!()
    }
}
