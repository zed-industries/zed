use gpui::DefiniteLength;

pub trait FixedWidth {
    fn width(self, width: DefiniteLength) -> Self;
    fn full_width(self) -> Self;
}
