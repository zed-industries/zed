use gpui::{App, Context, IntoElement, ParentElement, Render, Styled, Window};
use project::project_settings::ProjectSettings;
use settings::Settings as _;
use ui::{Label, h_flex, prelude::*};
use workspace::{HideStatusItem, StatusItemView, item::ItemHandle};

pub struct BlameIndicator;

impl Render for BlameIndicator {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let row = h_flex().gap_2().min_w_0().overflow_x_hidden();

        if !ProjectSettings::get_global(cx).git.status_bar_blame.enabled {
            return row.hidden();
        }

        row.child(Label::new("blame").size(LabelSize::Small))
    }
}

impl StatusItemView for BlameIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
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
