use crate::{SharedString, StyleRefinement};

pub trait Hover {
    fn set_hover_style(&mut self, group_name: Option<SharedString>, style: StyleRefinement);

    fn hover(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.set_hover_style(None, f(StyleRefinement::default()));
        self
    }

    fn group_hover(
        mut self,
        group_name: impl Into<SharedString>,
        f: impl FnOnce(StyleRefinement) -> StyleRefinement,
    ) -> Self
    where
        Self: Sized,
    {
        self.set_hover_style(Some(group_name.into()), f(StyleRefinement::default()));
        self
    }
}
