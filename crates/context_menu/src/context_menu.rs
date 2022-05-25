use gpui::{Entity, View};

enum ContextMenuItem {
    Item {
        label: String,
        action: Box<dyn Action>,
    },
    Separator,
}

pub struct ContextMenu {
    position: Vector2F,
    items: Vec<ContextMenuItem>,
}

impl Entity for ContextMenu {
    type Event = ();
}

impl View for ContextMenu {
    fn ui_name() -> &'static str {
        "ContextMenu"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        Overlay::new().with_abs_position(self.position).boxed()
    }
}
