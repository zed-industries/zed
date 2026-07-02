use gpui::{
    Action, App, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle, Focusable, Pixels,
    WeakEntity, Window, actions, px,
};
use ui::prelude::*;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

actions!(
    database_panel,
    [
        /// Toggles the database panel.
        Toggle,
        /// Toggles focus on the database panel.
        ToggleFocus,
        /// Opens the new connection dialog.
        AddConnection,
    ]
);

pub struct DatabasePanel {
    focus_handle: FocusHandle,
    // Fields workspace: WeakEntity<Workspace>, fs: Arc<dyn Fs>, store: Entity<ConnectionStore>
    // are added by Task 5; they are intentionally omitted here to avoid dead_code warnings.
}

impl DatabasePanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            Self::new(workspace, window, cx)
        })
    }

    fn new(
        _workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| DatabasePanel {
            focus_handle: cx.focus_handle(),
        })
    }
}

impl Render for DatabasePanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DatabasePanel")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .child(
                v_flex()
                    .size_full()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .child(Label::new("No connections").color(Color::Muted))
                    .child(
                        Button::new("add-connection", "Add Connection").on_click(
                            |_, window, cx| {
                                window.dispatch_action(AddConnection.boxed_clone(), cx);
                            },
                        ),
                    ),
            )
    }
}

impl Focusable for DatabasePanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for DatabasePanel {}

impl Panel for DatabasePanel {
    fn persistent_name() -> &'static str {
        "DatabasePanel"
    }

    fn panel_key() -> &'static str {
        "DatabasePanel"
    }

    fn position(&self, _: &Window, _: &App) -> DockPosition {
        DockPosition::Left
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, _: DockPosition, _: &mut Window, _: &mut Context<Self>) {}

    fn default_size(&self, _: &Window, _: &App) -> Pixels {
        px(240.)
    }

    fn icon(&self, _: &Window, _: &App) -> Option<ui::IconName> {
        Some(ui::IconName::DatabaseZap)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("Database Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        6
    }
}
