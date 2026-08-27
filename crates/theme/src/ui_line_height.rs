/// The row height for entries in UI panels, as a multiple of the UI font size.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum UiLineHeight {
    /// A less dense row height.
    #[default]
    Comfortable,
    /// A denser row height.
    Standard,
    /// A custom row height, where 1.0 is the UI font's size. Must be at least 1.0.
    Custom(f32),
}

impl UiLineHeight {
    /// Returns the value of the row height.
    pub fn value(&self) -> f32 {
        match self {
            UiLineHeight::Comfortable => 1.5,
            UiLineHeight::Standard => 1.3,
            UiLineHeight::Custom(line_height) => *line_height,
        }
    }
}
