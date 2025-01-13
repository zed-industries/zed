mod appearance_settings_controls;

use std::any::TypeId;

use command_palette_hooks::CommandPaletteFilter;
use editor::EditorSettingsControls;
use feature_flags::{FeatureFlag, FeatureFlagViewExt};
use gpui::{actions, AppContext, EventEmitter, FocusHandle, Focusable, Model};
use ui::prelude::*;
use workspace::item::{Item, ItemEvent};
use workspace::Workspace;

use crate::appearance_settings_controls::AppearanceSettingsControls;

pub struct SettingsUiFeatureFlag;

impl FeatureFlag for SettingsUiFeatureFlag {
    const NAME: &'static str = "settings-ui";
}

actions!(zed, [OpenSettingsEditor]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_window_models(|workspace: &mut Workspace, window, cx| {
        workspace.register_action(|workspace, _: &OpenSettingsEditor, window, cx| {
            let existing = workspace
                .active_pane()
                .read(cx)
                .items()
                .find_map(|item| item.downcast::<SettingsPage>());

            if let Some(existing) = existing {
                workspace.activate_item(&existing, true, true, window, cx);
            } else {
                let settings_page = SettingsPage::new(workspace, window, cx);
                workspace.add_item_to_active_pane(Box::new(settings_page), None, true, window, cx)
            }
        });

        let settings_ui_actions = [TypeId::of::<OpenSettingsEditor>()];

        CommandPaletteFilter::update_global(cx, |filter, _cx| {
            filter.hide_action_types(&settings_ui_actions);
        });

        cx.observe_flag::<SettingsUiFeatureFlag, _>(
            window,
            move |is_enabled, _view, window, cx| {
                if is_enabled {
                    CommandPaletteFilter::update_global(cx, |filter, _cx| {
                        filter.show_action_types(settings_ui_actions.iter());
                    });
                } else {
                    CommandPaletteFilter::update_global(cx, |filter, _cx| {
                        filter.hide_action_types(&settings_ui_actions);
                    });
                }
            },
        )
        .detach();
    })
    .detach();
}

pub struct SettingsPage {
    focus_handle: FocusHandle,
}

impl SettingsPage {
    pub fn new(
        _workspace: &Workspace,
        window: &mut Window,
        cx: &mut ModelContext<Workspace>,
    ) -> Model<Self> {
        cx.new_model(|cx| Self {
            focus_handle: cx.focus_handle(),
        })
    }
}

impl EventEmitter<ItemEvent> for SettingsPage {}

impl Focusable for SettingsPage {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for SettingsPage {
    type Event = ItemEvent;

    fn tab_icon(&self, _window: &Window, _cx: &AppContext) -> Option<Icon> {
        Some(Icon::new(IconName::Settings))
    }

    fn tab_content_text(&self, _window: &Window, _cx: &AppContext) -> Option<SharedString> {
        Some("Settings".into())
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        f(*event)
    }
}

impl Render for SettingsPage {
    fn render(&mut self, window: &mut Window, cx: &mut ModelContext<Self>) -> impl IntoElement {
        v_flex()
            .p_4()
            .size_full()
            .gap_4()
            .child(Label::new("Settings").size(LabelSize::Large))
            .child(
                v_flex().gap_1().child(Label::new("Appearance")).child(
                    v_flex()
                        .elevation_2(window, cx)
                        .child(AppearanceSettingsControls::new()),
                ),
            )
            .child(
                v_flex().gap_1().child(Label::new("Editor")).child(
                    v_flex()
                        .elevation_2(window, cx)
                        .child(EditorSettingsControls::new()),
                ),
            )
    }
}
