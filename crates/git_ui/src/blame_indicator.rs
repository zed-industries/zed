use editor::{Editor, ToPoint as _};
use gpui::{
    App, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, WeakEntity,
    Window,
};
use project::project_settings::ProjectSettings;
use settings::Settings as _;
use ui::{Label, h_flex, prelude::*};
use workspace::{HideStatusItem, StatusItemView, item::ItemHandle};

pub struct BlameIndicator {
    active_editor: Option<WeakEntity<Editor>>,
    current_line: Option<u32>,
    _observe_active_editor: Option<Subscription>,
}

impl BlameIndicator {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.observe_global::<settings::SettingsStore>(|_, cx| cx.notify())
            .detach();
        Self {
            active_editor: None,
            current_line: None,
            _observe_active_editor: None,
        }
    }

    fn on_editor_event(
        &mut self,
        _editor: Entity<Editor>,
        event: &editor::EditorEvent,
        cx: &mut Context<Self>,
    ) {
        if let editor::EditorEvent::SelectionsChanged { .. } = event {
            self.update_current_line(cx);
            cx.notify();
        }
    }

    fn update_current_line(&mut self, cx: &mut App) {
        let Some(editor) = self.active_editor.as_ref().and_then(|editor| editor.upgrade()) else {
            self.current_line = None;
            return;
        };
        let editor = editor.read(cx);
        let cursor = editor.selections.newest_anchor().head();
        let snapshot = editor.buffer().read(cx).read(cx);
        self.current_line = Some(cursor.to_point(&snapshot).row);
    }
}

impl Render for BlameIndicator {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let row = h_flex().gap_2().min_w_0().overflow_x_hidden();

        if !ProjectSettings::get_global(cx).git.status_bar_blame.enabled {
            return row.hidden();
        }

        let Some(line) = self.current_line else {
            return row.hidden();
        };

        row.child(Label::new(format!("line {}", line + 1)).size(LabelSize::Small))
    }
}

impl StatusItemView for BlameIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            self.active_editor = Some(editor.downgrade());
            self._observe_active_editor = Some(cx.subscribe(&editor, Self::on_editor_event));
            self.update_current_line(cx);
        } else {
            self.active_editor = None;
            self.current_line = None;
            self._observe_active_editor = None;
        }
        cx.notify();
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings
                .git
                .get_or_insert_default()
                .status_bar_blame
                .get_or_insert_default()
                .enabled = Some(false);
        }))
    }
}
