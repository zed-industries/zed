use crate::{ElementId, InteractiveElement, IntoElement, Render, div};

pub(crate) struct Inspector {
    selected_element: Option<ElementId>,
}

impl Default for Inspector {
    fn default() -> Self {
        Self {
            selected_element: None,
        }
    }
}

impl Inspector {}

impl Render for Inspector {
    fn render(
        &mut self,
        window: &mut crate::Window,
        cx: &mut crate::Context<Self>,
    ) -> impl IntoElement {
        div().id("GPUI_TOOLS_INSPECTOR")
    }
}
