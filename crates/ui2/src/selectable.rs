use gpui::{AnyView, WindowContext};

pub trait Selectable {
    fn selected(self, selected: bool) -> Self;
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
