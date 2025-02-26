use std::ops::Range;

use language::{Anchor, EditBehaviorProvider};

pub struct JsxEditBehaviorProvider;

pub struct JsxTagCompletionState {
    edit_index: usize,
}

impl EditBehaviorProvider for JsxEditBehaviorProvider {
    type AutoEditState = Vec<JsxTagCompletionState>;

    fn should_auto_edit(
        &self,
        buffer: &language::BufferSnapshot,
        edited_ranges: &[Range<usize>],
    ) -> Option<Self::AutoEditState> {
        let to_auto_edit = vec![];
        dbg!(buffer.text());
        for edited_range in edited_ranges {
            let text = buffer
                .text_for_range(edited_range.clone())
                .collect::<String>();
            if dbg!(!text.ends_with(">")) {
                continue;
            }
            let Some(layer) = dbg!(buffer.syntax_layer_at(edited_range.start)) else {
                continue;
            };
            let language_name = dbg!(layer.language.name());
            if dbg!(
                !(language_name.as_ref().eq_ignore_ascii_case("jsx")
                    || language_name.as_ref().eq_ignore_ascii_case("tsx"))
            ) {
                continue;
            }
            dbg!(layer.node().to_sexp());
            // todo! if buffer.settings_at
            let Some(node) = dbg!(layer
                .node()
                .descendant_for_byte_range(edited_range.start, edited_range.end))
            else {
                continue;
            };

            dbg!(node);
        }
        dbg!(&to_auto_edit.len());
        if to_auto_edit.is_empty() {
            return None;
        } else {
            return Some(to_auto_edit);
        }
    }

    fn auto_edit(
        &self,
        _buffer: language::BufferSnapshot,
        _ranges: &[Range<usize>],
        _state: Self::AutoEditState,
    ) -> gpui::Result<Vec<(std::ops::Range<Anchor>, String)>> {
        dbg!(_state.first().map(|s| s.edit_index));
        Ok(vec![])
    }
}
