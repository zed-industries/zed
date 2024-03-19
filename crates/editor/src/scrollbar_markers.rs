use gpui::{px, Pixels};
use ui::ElementContext;

struct MarkerHunk {
    start_row: usize,
    end_row: usize,
}

struct MarkerHunks {
    hunks: Vec<MarkerHunk>,
}

pub(crate) struct Markers {
    git_diffs: MarkerHunks,
    highlights: MarkerHunks,
    symbol_selections: MarkerHunks,
    diagnostic_errors: MarkerHunks,
    diagnostic_warnings: MarkerHunks,
    diagnostic_infos: MarkerHunks,
}

impl MarkerHunks {
    fn new() -> Self {
        Self { hunks: Vec::new() }
    }

    fn add_marker(&mut self, start_row: usize, end_row: usize) {
        assert!(start_row <= end_row);
        if let Some(mut hunk) = self.hunks.last() {
            assert!(start_row >= hunk.start_row);
            if start_row <= hunk.end_row {
                if end_row > hunk.end_row {
                    hunk.end_row = end_row;
                }
                return;
            }
        }
        self.hunks.push(MarkerHunk { start_row, end_row });
    }
}

impl Markers {
    pub fn new() -> Self {
        Self {
            git_diffs: MarkerCollection::new(),
            highlights: MarkerCollection::new(),
            symbol_selections: MarkerCollection::new(),
            diagnostic_errors: MarkerCollection::new(),
            diagnostic_warnings: MarkerCollection::new(),
            diagnostic_infos: MarkerCollection::new(),
        }
    }

    pub fn add_git_diff(&mut self, start_row: usize, end_row: usize) {
        self.git_diff.add_marker(start_row, end_row);
    }

    pub fn add_highlight(&mut self, start_row: usize, end_row: usize) {
        self.highlight.add_marker(start_row, end_row);
    }

    pub fn add_symbol_selection(&mut self, start_row: usize, end_row: usize) {
        self.symbol_selections.add_marker(start_row, end_row);
    }

    pub fn add_diagnostic_error(&mut self, start_row: usize, end_row: usize) {
        self.diagnostic_errors.add_marker(start_row, end_row);
    }

    pub fn add_diagnostic_warning(&mut self, start_row: usize, end_row: usize) {
        self.diagnostic_warnings.add_marker(start_row, end_row);
    }

    pub fn add_diagnostic_info(&mut self, start_row: usize, end_row: usize) {
        self.diagnostic_infos.add_marker(start_row, end_row);
    }

    // pub fn paint(&self, cx: &mut ElementContext<'_>) {
    //     Self::paint_hunks(&self.git_diffs, cx);
    // }

    // fn paint_hunks(hunks: &MarkerHunks, cx: &mut ElementContext<'_>) {
    //     for hunk in hunks {
    //         cx.paint_quad(quad(
    //             bounds,
    //             Corners::default(),
    //             color,
    //             Edges::default(),
    //             cx.theme().colors().scrollbar_thumb_border,
    //         ));
    //     }
    // }
}

// impl MarkerPainter {
//     fn new(hunk_painter: fn(Pixels, Pixels)) -> Self {
//         Self {
//             hunk_range: None,
//             hunk_painter,
//         }
//     }

//     fn add_marker(&mut self, start_y: Pixels, end_y: Pixels) {
//         assert!(start_y <= end_y);
//         let Some((hunk_start_y, hunk_end_y)) = self.hunk_range else {
//             self.hunk_range = Some((start_y, end_y));
//             return;
//         }
//         assert!(start_y >= hunk_start_y);
//         if start_y <= rang {
//             if end_y > self.end_y {
//                 self.end_y = end_y;
//             }
//             return;
//         }
//         self.paint_hunk();
//         self.start_y = start_y;
//         self.end_y = end_y;
//     }

//     fn finish(&mut self) {
//         if self.start_y >= 0 {
//             paint_hunk(&self);
//         }
//         self.start_y = -1;
//         self.end_y = -1;
//     }

//     fn paint_hunk(&self) {
//         (self.hunk_painter)(self.start_y, self.end_y);
//     }
// }
