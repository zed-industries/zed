use language::EditBehaviorProvider;

pub struct JsxEditBehaviorProvider;

impl EditBehaviorProvider for JsxEditBehaviorProvider {
    fn should_auto_edit(
        &self,
        buffer: &language::BufferSnapshot,
        edited_ranges: Vec<std::ops::Range<usize>>,
    ) -> Option<Vec<char>> {
        todo!()
    }

    fn auto_edit(
        &self,
        buffer: language::BufferSnapshot,
        ranges: Vec<std::ops::Range<usize>>,
    ) -> gpui::Result<Vec<(std::ops::Range<usize>, String)>> {
        todo!()
    }
}
