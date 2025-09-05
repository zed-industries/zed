use std::ops::Range;

use multi_buffer::{MultiBufferPoint, MultiBufferRow};
use ui::{App, Window};

use crate::{DisplayPoint, EditorElement, EditorSnapshot, RangeToAnchorExt, element::EditorLayout};

pub struct BracketLayout {
    index: u32,
    start: Range<DisplayPoint>,
    end: Range<DisplayPoint>,
}

impl EditorElement {
    pub fn layout_brackets(
        &mut self,
        visible_buffer_range: Range<MultiBufferRow>,
        snapshot: &EditorSnapshot,
        _cx: &mut App,
    ) -> Vec<BracketLayout> {
        if true {
            return Vec::new();
        }
        // TODO kb if settings forbid, do nothing, blah
        // TODO kb cache
        snapshot
            .buffer_snapshot
            .enclosing_bracket_ranges(
                MultiBufferPoint::new(visible_buffer_range.start.0, 0)
                    ..MultiBufferPoint::new(visible_buffer_range.end.0, 0),
            )
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(index, (start, end))| BracketLayout {
                index: index as u32,
                start: start.to_display_points(snapshot),
                end: end.to_display_points(snapshot),
            })
            .collect()
    }

    fn paint_brackets(&mut self, layout: &mut EditorLayout, window: &mut Window, cx: &mut App) {
        // let space_invisible = window.text_system().shape_line(
        //     "•".into(),
        //     invisible_symbol_font_size,
        //     &[TextRun {
        //         len: "•".len(),
        //         font: self.style.text.font(),
        //         color: cx.theme().colors().editor_invisible,
        //         background_color: None,
        //         underline: None,
        //         strikethrough: None,
        //     }],
        //     None,
        // );

        // space_invisible
        //     .paint(origin, line_height, window, cx)
        //     .log_err();
        // //
    }
}
