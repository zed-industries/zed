use crate::StyleRefinement;

pub trait Active {
    fn set_active_style(&mut self, style: StyleRefinement);

    fn active(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.set_active_style(f(StyleRefinement::default()));
        self
    }
}
