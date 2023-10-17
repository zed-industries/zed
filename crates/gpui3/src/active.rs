use crate::{SharedString, StyleRefinement};

pub trait Active {
    fn set_active_style(&mut self, group_name: Option<SharedString>, style: StyleRefinement);

    fn active(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.set_active_style(None, f(StyleRefinement::default()));
        self
    }

    fn group_active(
        mut self,
        group_name: impl Into<SharedString>,
        f: impl FnOnce(StyleRefinement) -> StyleRefinement,
    ) -> Self
    where
        Self: Sized,
    {
        self.set_active_style(Some(group_name.into()), f(StyleRefinement::default()));
        self
    }
}
