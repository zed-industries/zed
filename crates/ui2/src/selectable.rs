use gpui::{AnyView, WindowContext};

pub trait Selectable {
    fn selected(&mut self, selected: bool) -> &mut Self;
    fn selected_tooltip(
        &mut self,
        tooltip: Box<dyn Fn(&mut WindowContext) -> AnyView + 'static>,
    ) -> &mut Self;
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
