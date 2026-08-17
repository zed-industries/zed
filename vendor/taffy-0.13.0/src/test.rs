#![allow(dead_code)]

use taffy::{AvailableSpace, NodeId, Size, Style};

/// A shared measure function for tests which means that tests compiled with separate crates
/// and using different styles of measure function. This saves on compile time when running tests.

#[derive(Debug, Copy, Clone)]
pub struct TestNodeContext {
    /// How many times the measure function has been called
    pub count: usize,
    /// The measurement data to compute the intrinsic size from
    pub measure_data: TestMeasureData,
}

impl TestNodeContext {
    /// Create a new `TestNodeContext` from `TestMeasureData`
    pub const fn new(measure_data: TestMeasureData) -> Self {
        Self { count: 0, measure_data }
    }

    /// Create a `TestNodeContext` for a zero-sized node
    pub const fn zero() -> Self {
        Self::new(TestMeasureData::Zero)
    }

    /// Create a `TestNodeContext` for a fixed-sized node
    pub const fn fixed(size: Size<f32>) -> Self {
        Self::new(TestMeasureData::Fixed(size))
    }

    /// Create a `TestNodeContext` for a node with a width and aspect-ratio
    pub const fn aspect_ratio(width: f32, height_ratio: f32) -> Self {
        let data = AspectRatioMeasureData { width, height_ratio };
        Self::new(TestMeasureData::AspectRatio(data))
    }

    /// Create a `TestNodeContext` for a node with text using the Ahem font
    pub const fn ahem_text(text_content: &'static str, writing_mode: WritingMode) -> Self {
        let data = AhemTextMeasureData { text_content, writing_mode };
        Self::new(TestMeasureData::AhemText(data))
    }
}

/// The measurement data for the node
#[derive(Debug, Copy, Clone)]
pub enum TestMeasureData {
    /// A zero-sized node
    Zero,
    /// A node with a fixed size
    Fixed(Size<f32>),
    /// A node with a fixed size
    AspectRatio(AspectRatioMeasureData),
    /// A node with text using the Ahem font
    AhemText(AhemTextMeasureData),
}

/// A measure function for tests that works with `TestNodeContext`
pub fn test_measure_function(
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    _node_id: NodeId,
    context: Option<&mut TestNodeContext>,
    _style: &Style,
) -> Size<f32> {
    if let Size { width: Some(width), height: Some(height) } = known_dimensions {
        return Size { width, height };
    }

    let Some(context) = context else { return known_dimensions.map(|d| d.unwrap_or(0.0)) };

    // Increment count
    context.count += 1;

    let compute_size = match &context.measure_data {
        TestMeasureData::Zero => Size::ZERO,
        TestMeasureData::Fixed(size) => *size,
        TestMeasureData::AspectRatio(data) => data.measure(known_dimensions),
        TestMeasureData::AhemText(data) => data.measure(known_dimensions, available_space),
    };

    Size {
        width: known_dimensions.width.unwrap_or(compute_size.width),
        height: known_dimensions.height.unwrap_or(compute_size.height),
    }
}

/// Measure data for nodes that returns results based on an intrinsic aspect ratio
#[derive(Debug, Copy, Clone)]
pub struct AspectRatioMeasureData {
    width: f32,
    height_ratio: f32,
}
impl AspectRatioMeasureData {
    fn measure(&self, known_dimensions: Size<Option<f32>>) -> Size<f32> {
        let width = known_dimensions.width.unwrap_or(self.width);
        let height = known_dimensions.height.unwrap_or(width * self.height_ratio);
        Size { width, height }
    }
}

/// Whether text is horizontal or vertical
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WritingMode {
    /// Horizontal text
    Horizontal,
    /// Vertical text
    Vertical,
}
/// Measure data for nodes contain text using the Ahem testing font
#[derive(Debug, Clone, Copy)]
pub struct AhemTextMeasureData {
    /// The text string
    pub text_content: &'static str,
    /// The writing mode
    pub writing_mode: WritingMode,
}
impl AhemTextMeasureData {
    fn measure(
        &self,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::Size<f32> {
        use taffy::prelude::*;
        use taffy::AbsoluteAxis;

        const ZWS: char = '\u{200B}';
        const H_WIDTH: f32 = 10.0;
        const H_HEIGHT: f32 = 10.0;

        let inline_axis = match self.writing_mode {
            WritingMode::Horizontal => AbsoluteAxis::Horizontal,
            WritingMode::Vertical => AbsoluteAxis::Vertical,
        };
        let block_axis = inline_axis.other_axis();
        let lines: Vec<&str> = self.text_content.split(ZWS).collect();

        if lines.is_empty() {
            return Size::ZERO;
        }

        let min_line_length: usize = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let max_line_length: usize = lines.iter().map(|line| line.len()).sum();
        let inline_size = known_dimensions
            .get_abs(inline_axis)
            .unwrap_or_else(|| match available_space.get_abs(inline_axis) {
                AvailableSpace::MinContent => min_line_length as f32 * H_WIDTH,
                AvailableSpace::MaxContent => max_line_length as f32 * H_WIDTH,
                AvailableSpace::Definite(inline_size) => inline_size.min(max_line_length as f32 * H_WIDTH),
            })
            .max(min_line_length as f32 * H_WIDTH);
        let block_size = known_dimensions.get_abs(block_axis).unwrap_or_else(|| {
            let inline_line_length = (inline_size / H_WIDTH).floor() as usize;
            let mut line_count = 1;
            let mut current_line_length = 0;
            for line in &lines {
                if current_line_length + line.len() > inline_line_length {
                    if current_line_length > 0 {
                        line_count += 1
                    };
                    current_line_length = line.len();
                } else {
                    current_line_length += line.len();
                };
            }
            (line_count as f32) * H_HEIGHT
        });

        match self.writing_mode {
            WritingMode::Horizontal => Size { width: inline_size, height: block_size },
            WritingMode::Vertical => Size { width: block_size, height: inline_size },
        }
    }
}
