use core::cmp::{max, min};

const ELLIPSIS_PASSING: usize = 6;
const LONG_WHITESPACE: usize = 20;
const LONG_WHITESPACE_PADDING: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Margin {
    /// The available whitespace in the left that can be consumed when centering.
    whitespace_left: usize,
    /// The column of the beginning of left-most span.
    span_left: usize,
    /// The column of the end of right-most span.
    span_right: usize,
    /// The beginning of the line to be displayed.
    computed_left: usize,
    /// The end of the line to be displayed.
    computed_right: usize,
    /// The current width of the terminal. 140 by default and in tests.
    pub(crate) term_width: usize,
    /// The end column of a span label, including the span. Doesn't account for labels not in the
    /// same line as the span.
    label_right: usize,
}

impl Margin {
    pub(crate) fn new(
        whitespace_left: usize,
        span_left: usize,
        span_right: usize,
        label_right: usize,
        term_width: usize,
        max_line_len: usize,
    ) -> Self {
        // The 6 is padding to give a bit of room for `...` when displaying:
        // ```
        // error: message
        //   --> file.rs:16:58
        //    |
        // 16 | ... fn foo(self) -> Self::Bar {
        //    |                     ^^^^^^^^^
        // ```

        let mut m = Margin {
            whitespace_left: whitespace_left.saturating_sub(ELLIPSIS_PASSING),
            span_left: span_left.saturating_sub(ELLIPSIS_PASSING),
            span_right: span_right + ELLIPSIS_PASSING,
            computed_left: 0,
            computed_right: 0,
            term_width,
            label_right: label_right + ELLIPSIS_PASSING,
        };
        m.compute(max_line_len);
        m
    }

    pub(crate) fn was_cut_left(&self) -> bool {
        self.computed_left > 0
    }

    fn compute(&mut self, max_line_len: usize) {
        // When there's a lot of whitespace (>20), we want to trim it as it is useless.
        self.computed_left = if self.whitespace_left > LONG_WHITESPACE {
            self.whitespace_left - (LONG_WHITESPACE - LONG_WHITESPACE_PADDING) // We want some padding.
        } else {
            0
        };
        // We want to show as much as possible, max_line_len is the right-most boundary for the
        // relevant code.
        self.computed_right = max(max_line_len, self.computed_left);

        if self.computed_right - self.computed_left > self.term_width {
            // Trimming only whitespace isn't enough, let's get craftier.
            if self.label_right.saturating_sub(self.whitespace_left) <= self.term_width
                // Trimming whitespace when the right-most label is somewhrere
                // within it would result in the label pointing to the wrong
                // place
                && self.label_right >= self.whitespace_left
            {
                // Attempt to fit the code window only trimming whitespace.
                self.computed_left = self.whitespace_left;
                self.computed_right = self.computed_left + self.term_width;
            } else if self.label_right - self.span_left <= self.term_width {
                // Attempt to fit the code window considering only the spans and labels.
                let padding_left = (self.term_width - (self.label_right - self.span_left)) / 2;
                self.computed_left = self.span_left.saturating_sub(padding_left);
                self.computed_right = self.computed_left + self.term_width;
            } else if self.span_right - self.span_left <= self.term_width {
                // Attempt to fit the code window considering the spans and labels plus padding.
                let padding_left = (self.term_width - (self.span_right - self.span_left)) / 5 * 2;
                self.computed_left = self.span_left.saturating_sub(padding_left);
                self.computed_right = self.computed_left + self.term_width;
            } else {
                // Mostly give up but still don't show the full line.
                self.computed_left = self.span_left;
                self.computed_right = self.span_right;
            }
        }
    }

    pub(crate) fn left(&self, line_len: usize) -> usize {
        min(self.computed_left, line_len)
    }

    pub(crate) fn right(&self, line_len: usize) -> usize {
        if line_len.saturating_sub(self.computed_left) <= self.term_width {
            line_len
        } else {
            min(line_len, self.computed_right)
        }
    }
}
