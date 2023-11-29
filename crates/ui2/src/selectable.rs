use gpui::{AnyView, WindowContext};

/// A trait for elements that can be selected.
pub trait Selectable {
    /// Sets whether the element is selected.
    fn selected(self, selected: bool) -> Self;

    /// Sets the tooltip that should be shown when the element is selected.
    fn selected_tooltip(
        self,
        tooltip: Box<dyn Fn(&mut WindowContext) -> AnyView + 'static>,
    ) -> Self;
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Selection {
    #[default]
    Unselected,
    Indeterminate,
    Selected,
}

impl Selection {
    pub fn inverse(&self) -> Self {
        match self {
            Self::Unselected | Self::Indeterminate => Self::Selected,
            Self::Selected => Self::Unselected,
        }
    }
}
