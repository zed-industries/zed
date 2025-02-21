use std::ops::Range;

use language::EditBehaviorProvider;

pub struct JsxEditBehaviorProvider;

impl EditBehaviorProvider for JsxEditBehaviorProvider {
    type AutoEditState = ();

    fn should_auto_edit(
        &self,
        buffer: &language::BufferSnapshot,
        edited_ranges: &[Range<usize>],
    ) -> Option<Self::AutoEditState> {
        return Some(());
    }

    fn auto_edit(
        &self,
        buffer: language::BufferSnapshot,
        ranges: &[Range<usize>],
        state: Self::AutoEditState,
    ) -> gpui::Result<Vec<(std::ops::Range<usize>, String)>> {
        Ok(vec![])
    }
}
