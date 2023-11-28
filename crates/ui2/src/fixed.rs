use gpui::DefiniteLength;

pub trait FixedWidth {
    fn width(&mut self, width: DefiniteLength) -> &mut Self;
    fn full_width(&mut self) -> &mut Self;
}
