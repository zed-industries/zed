use language::{Anchor, Bias, BufferSnapshot};
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SearchDirection {
    Up,
    Left,
    Diagonal,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SearchState {
    cost: u32,
    direction: SearchDirection,
}

impl SearchState {
    fn new(cost: u32, direction: SearchDirection) -> Self {
        Self { cost, direction }
    }
}

struct SearchMatrix {
    cols: usize,
    data: Vec<SearchState>,
}

impl SearchMatrix {
    fn new(rows: usize, cols: usize) -> Self {
        SearchMatrix {
            cols,
            data: vec![SearchState::new(0, SearchDirection::Diagonal); rows * cols],
        }
    }

    fn get(&self, row: usize, col: usize) -> SearchState {
        self.data[row * self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, cost: SearchState) {
        self.data[row * self.cols + col] = cost;
    }
}

pub fn resolve_search_block(buffer: &BufferSnapshot, search_query: &str) -> Range<Anchor> {
    const INSERTION_COST: u32 = 3;
    const DELETION_COST: u32 = 10;
    const WHITESPACE_INSERTION_COST: u32 = 1;
    const WHITESPACE_DELETION_COST: u32 = 1;

    let buffer_len = buffer.len();
    let query_len = search_query.len();
    let mut matrix = SearchMatrix::new(query_len + 1, buffer_len + 1);
    let mut leading_deletion_cost = 0_u32;
    for (row, query_byte) in search_query.bytes().enumerate() {
        let deletion_cost = if query_byte.is_ascii_whitespace() {
            WHITESPACE_DELETION_COST
        } else {
            DELETION_COST
        };

        leading_deletion_cost = leading_deletion_cost.saturating_add(deletion_cost);
        matrix.set(
            row + 1,
            0,
            SearchState::new(leading_deletion_cost, SearchDirection::Diagonal),
        );

        for (col, buffer_byte) in buffer.bytes_in_range(0..buffer.len()).flatten().enumerate() {
            let insertion_cost = if buffer_byte.is_ascii_whitespace() {
                WHITESPACE_INSERTION_COST
            } else {
                INSERTION_COST
            };

            let up = SearchState::new(
                matrix.get(row, col + 1).cost.saturating_add(deletion_cost),
                SearchDirection::Up,
            );
            let left = SearchState::new(
                matrix.get(row + 1, col).cost.saturating_add(insertion_cost),
                SearchDirection::Left,
            );
            let diagonal = SearchState::new(
                if query_byte == *buffer_byte {
                    matrix.get(row, col).cost
                } else {
                    matrix
                        .get(row, col)
                        .cost
                        .saturating_add(deletion_cost + insertion_cost)
                },
                SearchDirection::Diagonal,
            );
            matrix.set(row + 1, col + 1, up.min(left).min(diagonal));
        }
    }

    // Traceback to find the best match
    let mut best_buffer_end = buffer_len;
    let mut best_cost = u32::MAX;
    for col in 1..=buffer_len {
        let cost = matrix.get(query_len, col).cost;
        if cost < best_cost {
            best_cost = cost;
            best_buffer_end = col;
        }
    }

    let mut query_ix = query_len;
    let mut buffer_ix = best_buffer_end;
    while query_ix > 0 && buffer_ix > 0 {
        let current = matrix.get(query_ix, buffer_ix);
        match current.direction {
            SearchDirection::Diagonal => {
                query_ix -= 1;
                buffer_ix -= 1;
            }
            SearchDirection::Up => {
                query_ix -= 1;
            }
            SearchDirection::Left => {
                buffer_ix -= 1;
            }
        }
    }

    let mut start = buffer.offset_to_point(buffer.clip_offset(buffer_ix, Bias::Left));
    start.column = 0;
    let mut end = buffer.offset_to_point(buffer.clip_offset(best_buffer_end, Bias::Right));
    if end.column > 0 {
        end.column = buffer.line_len(end.row);
    }

    buffer.anchor_after(start)..buffer.anchor_before(end)
}

#[cfg(test)]
mod tests {
    use crate::edit_files_tool::resolve_search_block::resolve_search_block;
    use gpui::{prelude::*, App};
    use language::{Buffer, OffsetRangeExt as _};
    use unindent::Unindent as _;
    use util::test::{generate_marked_text, marked_text_ranges};

    #[gpui::test]
    fn test_resolve_search_block(cx: &mut App) {
        assert_resolved(
            concat!(
                "    Lorem\n",
                "«    ipsum\n",
                "    dolor sit amet»\n",
                "    consecteur",
            ),
            "ipsum\ndolor",
            cx,
        );

        assert_resolved(
            &"
            «fn foo1(a: usize) -> usize {
                40
            }»

            fn foo2(b: usize) -> usize {
                42
            }
            "
            .unindent(),
            "fn foo1(b: usize) {\n40\n}",
            cx,
        );

        assert_resolved(
            &"
            fn main() {
            «    Foo
                    .bar()
                    .baz()
                    .qux()»
            }

            fn foo2(b: usize) -> usize {
                42
            }
            "
            .unindent(),
            "Foo.bar.baz.qux()",
            cx,
        );

        assert_resolved(
            &"
            class Something {
                one() { return 1; }
            «    two() { return 2222; }
                three() { return 333; }
                four() { return 4444; }
                five() { return 5555; }
                six() { return 6666; }
            »    seven() { return 7; }
                eight() { return 8; }
            }
            "
            .unindent(),
            &"
                two() { return 2222; }
                four() { return 4444; }
                five() { return 5555; }
                six() { return 6666; }
            "
            .unindent(),
            cx,
        );
    }

    #[track_caller]
    fn assert_resolved(text_with_expected_range: &str, query: &str, cx: &mut App) {
        let (text, _) = marked_text_ranges(text_with_expected_range, false);
        let buffer = cx.new(|cx| Buffer::local(text.clone(), cx));
        let snapshot = buffer.read(cx).snapshot();
        let range = resolve_search_block(&snapshot, query).to_offset(&snapshot);
        let text_with_actual_range = generate_marked_text(&text, &[range], false);
        pretty_assertions::assert_eq!(text_with_actual_range, text_with_expected_range);
    }
}
