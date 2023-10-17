use crate::StyleRefinement;

pub trait Hover {
    fn set_hover_style(&mut self, style: StyleRefinement);

    fn hover(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.set_hover_style(f(StyleRefinement::default()));
        self
    }
}
