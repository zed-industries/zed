use ui::{IconButtonShape, Tooltip, prelude::*};
use workspace::StatusItemView;

pub struct LspTool {}

impl LspTool {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatusItemView for LspTool {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn workspace::ItemHandle>,
        _window: &mut ui::Window,
        _cx: &mut ui::Context<Self>,
    ) {
        // TODO kb see inline_completion_button.rs
    }
}

impl Render for LspTool {
    // TODO kb won't work for remote clients, disable for now
    // TODO kb add a setting to remove this button out of the status bar
    fn render(
        &mut self,
        _window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        div().child(
            IconButton::new("zed-lsp-tool-button", IconName::Bolt)
                .shape(IconButtonShape::Square)
                .icon_size(IconSize::XSmall)
                .indicator_border_color(Some(cx.theme().colors().status_bar_background))
                .tooltip(move |_, cx| Tooltip::simple("Language servers", cx))
                .on_click(cx.listener(move |_, _, _window, _cx| {
                    dbg!("????????");
                })),
        )
    }
}
