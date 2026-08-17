pub const LOREM_IPSUM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub struct FontMetrics {
    pub char_width: f32,
    pub char_height: f32,
}

#[allow(dead_code)]
pub enum WritingMode {
    Horizontal,
    Vertical,
}

pub struct TextContext {
    pub text_content: String,
    pub writing_mode: WritingMode,
}

pub fn text_measure_function(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
    text_context: &TextContext,
    font_metrics: &FontMetrics,
) -> taffy::geometry::Size<f32> {
    use taffy::geometry::AbsoluteAxis;
    use taffy::prelude::*;

    let inline_axis = match text_context.writing_mode {
        WritingMode::Horizontal => AbsoluteAxis::Horizontal,
        WritingMode::Vertical => AbsoluteAxis::Vertical,
    };
    let block_axis = inline_axis.other_axis();
    let words: Vec<&str> = text_context.text_content.split_whitespace().collect();

    if words.is_empty() {
        return Size::ZERO;
    }

    let min_line_length: usize = words.iter().map(|line| line.len()).max().unwrap_or(0);
    let max_line_length: usize = words.iter().map(|line| line.len()).sum();
    let inline_size =
        known_dimensions.get_abs(inline_axis).unwrap_or_else(|| match available_space.get_abs(inline_axis) {
            AvailableSpace::MinContent => min_line_length as f32 * font_metrics.char_width,
            AvailableSpace::MaxContent => max_line_length as f32 * font_metrics.char_width,
            AvailableSpace::Definite(inline_size) => inline_size
                .min(max_line_length as f32 * font_metrics.char_width)
                .max(min_line_length as f32 * font_metrics.char_width),
        });
    let block_size = known_dimensions.get_abs(block_axis).unwrap_or_else(|| {
        let inline_line_length = (inline_size / font_metrics.char_width).floor() as usize;
        let mut line_count = 1;
        let mut current_line_length = 0;
        for word in &words {
            if current_line_length == 0 {
                // first word
                current_line_length = word.len();
            } else if current_line_length + word.len() + 1 > inline_line_length {
                // every word past the first needs to check for line length including the space between words
                // note: a real implementation of this should handle whitespace characters other than ' '
                // and do something more sophisticated for long words
                line_count += 1;
                current_line_length = word.len();
            } else {
                // add the word and a space
                current_line_length += word.len() + 1;
            };
        }
        (line_count as f32) * font_metrics.char_height
    });

    match text_context.writing_mode {
        WritingMode::Horizontal => Size { width: inline_size, height: block_size },
        WritingMode::Vertical => Size { width: block_size, height: inline_size },
    }
}
