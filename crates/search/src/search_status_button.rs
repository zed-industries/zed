use editor::EditorSettings;
use gpui::FocusHandle;
use settings::Settings as _;
use ui::{ButtonCommon, Clickable, Context, Render, Tooltip, Window, prelude::*};
use workspace::{ItemHandle, StatusItemView};

pub const SEARCH_ICON: IconName = IconName::MagnifyingGlass;

pub struct SearchButton {
    pane_item_focus_handle: Option<FocusHandle>,
}

impl SearchButton {
    pub fn new() -> Self {
        Self {
            pane_item_focus_handle: None,
        }
    }
}

impl Render for SearchButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let button = div();

        if !EditorSettings::get_global(cx).search.button {
            return button.hidden();
        }

        let focus_handle = self.pane_item_focus_handle.clone();
        button.child(
            IconButton::new("project-search-indicator", SEARCH_ICON)
                .icon_size(IconSize::Small)
                .tooltip(move |_window, cx| {
                    if let Some(focus_handle) = &focus_handle {
                        Tooltip::for_action_in(
                            "Project Search",
                            &workspace::DeploySearch::default(),
                            focus_handle,
                            cx,
                        )
                    } else {
                        Tooltip::for_action(
                            "Project Search",
                            &workspace::DeploySearch::default(),
                            cx,
                        )
                    }
                })
                .on_click(cx.listener(|_this, _, window, cx| {
                    window.dispatch_action(Box::new(workspace::DeploySearch::default()), cx);
                })),
        )
    }
}

impl StatusItemView for SearchButton {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pane_item_focus_handle = active_pane_item.map(|item| item.item_focus_handle(cx));
    }
}
