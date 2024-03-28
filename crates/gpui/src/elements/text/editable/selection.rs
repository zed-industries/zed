use std::ops::Range;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug)]
pub struct Selection {
    /// The direction the selection is trending towards.
    pub direction: Direction,
    /// The range the selection covers.
    pub span: Range<usize>,
}

impl From<Range<usize>> for Selection {
    fn from(value: Range<usize>) -> Self {
        Self {
            direction: Direction::Right,
            span: value,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}

impl Selection {
    pub fn new(from: usize, to: usize) -> Self {
        let start = from.min(to);
        let end = from.max(to);

        Self {
            span: start..end,
            direction: if from > to {
                Direction::Left
            } else {
                Direction::Right
            },
        }
    }

    pub fn not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        Range::is_empty(&self.span)
    }

    pub fn is_left(&self) -> bool {
        matches!(self.direction, Direction::Left)
    }

    pub fn position(&self) -> usize {
        if self.is_left() {
            self.span.start
        } else {
            self.span.end
        }
    }

    pub fn get_left(&self, text: &str, from: usize) -> usize {
        text[..from]
            .grapheme_indices(true)
            .next_back()
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    pub fn get_right(&self, text: &str, from: usize) -> usize {
        text[from..]
            .grapheme_indices(true)
            .skip(1)
            .next()
            .map(|(i, _)| i + from)
            .unwrap_or(text.len())
    }

    fn get_next_word_end_index(&self, text: &str, from: usize) -> usize {
        text[from..]
            .split_word_bound_indices()
            .next()
            .map(|(i, word)| from + i + word.len())
            .unwrap_or(text.len())
    }

    fn get_previous_word_start_index(&self, text: &str, from: usize) -> usize {
        text[..from]
            .split_word_bound_indices()
            .next_back()
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    fn get_end(&self, text: &str) -> usize {
        text.grapheme_indices(true)
            .next_back()
            .map(|(i, _)| i)
            .unwrap_or(self.span.end)
    }

    pub fn get_next_line_start(&self, text: &str) -> usize {
        let from = self.position();

        if let Some(current) = text[from..].lines().next() {
            let maybe = from + current.len() + 1;

            if maybe < text.len() {
                maybe
            } else {
                // Is currently in last line.
                from
            }
        } else {
            from
        }
    }

    pub fn get_previous_line_end(&self, text: &str) -> usize {
        let from = self.position();

        if let Some(current) = text[..from].lines().next_back() {
            from - current.len() - 1
        } else {
            from
        }
    }
}

impl Selection {
    pub fn move_left(&mut self, text: &str) {
        if self.is_empty() {
            let left = self.get_left(text, self.span.start);

            self.span = left..left;
        } else {
            self.span.end = self.span.start;
        }
    }

    pub fn move_right(&mut self, text: &str) {
        if Range::is_empty(&self.span) {
            let right = self.get_right(text, self.span.start);

            self.span = right..right;
        } else {
            self.span.start = self.span.end;
        }
    }

    pub fn move_to_beginning(&mut self, _: &str) {
        self.span = 0..0;
    }

    pub fn move_to_end(&mut self, text: &str) {
        let end = text.len();

        self.span = end..end;
    }

    pub fn move_to_next_word_end(&mut self, text: &str) {
        let from = match self.direction {
            Direction::Left => self.span.start,
            Direction::Right => self.span.end,
        };

        let next_word_end = self.get_next_word_end_index(text, from);

        self.span = next_word_end..next_word_end;
    }

    pub fn move_to_previous_word_start(&mut self, text: &str) {
        let from = match self.direction {
            Direction::Left => self.span.start,
            Direction::Right => self.span.end,
        };

        let previous_word_start = self.get_previous_word_start_index(text, from);

        self.span = previous_word_start..previous_word_start;
    }

    pub fn select_all(&mut self, text: &str) {
        self.span = 0..self.get_end(text);
    }

    pub fn select_left(&mut self, text: &str) {
        if self.is_empty() || self.is_left() {
            self.span.start = self.get_left(text, self.span.start);
            self.direction = Direction::Left;
        } else {
            self.span.end = self.get_left(text, self.span.start);
        }
    }

    pub fn select_right(&mut self, text: &str) {
        if self.is_empty() || !self.is_left() {
            self.span.end = self.get_right(text, self.span.end);
            self.direction = Direction::Right;
        } else {
            self.span.start = self.get_right(text, self.span.start);
        }
    }

    pub fn select_to_beginning(&mut self, _text: &str) {
        self.span.start = 0;
        self.direction = Direction::Left;
    }

    pub fn select_to_end(&mut self, text: &str) {
        self.span.end = text.len();
        self.direction = Direction::Right;
    }

    pub fn select_to_next_word_end(&mut self, text: &str) {
        if self.is_empty() {
            self.span.end = self.get_next_word_end_index(text, self.span.end);
            self.direction = Direction::Right;
        } else if self.is_left() {
            self.span.start = self.span.end;
        } else {
            self.span.end = self.get_next_word_end_index(text, self.span.end);
        }
    }

    pub fn select_to_previous_word_start(&mut self, text: &str) {
        if self.is_empty() {
            self.span.start = self.get_previous_word_start_index(text, self.span.start);
            self.direction = Direction::Left;
        } else if self.is_left() {
            self.span.start = self.get_previous_word_start_index(text, self.span.start);
        } else {
            self.span.end = self.span.start;
        }
    }
}
