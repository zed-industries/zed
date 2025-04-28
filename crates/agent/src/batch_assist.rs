use editor::Editor;
use gpui::{Entity, EventEmitter};
use ui::{Render, Styled, div, px};
use workspace::{ToolbarItemEvent, ToolbarItemLocation};

use crate::{buffer_codegen::BufferCodegen, inline_prompt_editor::PromptEditor};

pub struct BatchAssistToolbarItem {
    visible: bool,
    assist: Option<Entity<BatchCodegen>>,
    active_editor: Option<Entity<Editor>>,
    // prompt_editor: Entity<PromptEditor>,
}

impl BatchAssistToolbarItem {
    pub fn new() -> Self {
        Self {
            visible: false,
            assist: None,
            active_editor: None,
        }
    }

    pub fn deploy(&mut self, cx: &mut ui::Context<Self>) {
        self.visible = true;
    }
}

impl EventEmitter<ToolbarItemEvent> for BatchAssistToolbarItem {}

impl workspace::ToolbarItemView for BatchAssistToolbarItem {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::ItemHandle>,
        _window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> workspace::ToolbarItemLocation {
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            self.active_editor = Some(editor);
            ToolbarItemLocation::Secondary
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

impl Render for BatchAssistToolbarItem {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        dbg!(&self.active_editor);
        if self.visible {
            div().bg(gpui::red()).w(px(100.)).h(px(100.))
        } else {
            div()
        }
    }
}

pub struct BatchCodegen {
    codegens: Vec<Entity<BufferCodegen>>,
}
