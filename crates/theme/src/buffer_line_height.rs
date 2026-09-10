/// The buffer's line height.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum BufferLineHeight {
    /// A less dense line height.
    #[default]
    Comfortable,
    /// The default line height.
    Standard,
    /// A custom line height, where 1.0 is the font's height. Must be at least 1.0.
    Custom(f32),
}

impl BufferLineHeight {
    /// Returns the value of the line height.
    pub fn value(&self) -> f32 {
        match self {
            BufferLineHeight::Comfortable => 1.618,
            BufferLineHeight::Standard => 1.3,
            BufferLineHeight::Custom(line_height) => *line_height,
        }
    }
}
