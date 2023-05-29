use super::DocumentFragment;
use crate::DenseIndex;
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
    max_id: DenseIndex,
}

impl sum_tree::Summary for DocumentFragmentSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, cxfg: &Self::Context) {
        todo!()
    }
}
