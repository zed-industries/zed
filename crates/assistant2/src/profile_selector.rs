use std::sync::{Arc, LazyLock};

use anyhow::Result;
use assistant_settings::{AgentProfile, AssistantSettings};
use editor::scroll::Autoscroll;
use editor::Editor;
use fs::Fs;
use gpui::{prelude::*, AsyncWindowContext, Entity, Subscription, WeakEntity};
use indexmap::IndexMap;
use regex::Regex;
use settings::{update_settings_file, Settings as _, SettingsStore};
use ui::{prelude::*, ContextMenu, ContextMenuEntry, PopoverMenu, Tooltip};
use util::ResultExt as _;
use workspace::{create_and_open_local_file, Workspace};

use crate::ThreadStore;

pub struct ProfileSelector {
    profiles: IndexMap<Arc<str>, AgentProfile>,
    fs: Arc<dyn Fs>,
    thread_store: WeakEntity<ThreadStore>,
    _subscriptions: Vec<Subscription>,
}

impl ProfileSelector {
    pub fn new(
        fs: Arc<dyn Fs>,
        thread_store: WeakEntity<ThreadStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let settings_subscription = cx.observe_global::<SettingsStore>(move |this, cx| {
            this.refresh_profiles(cx);
        });

        let mut this = Self {
            profiles: IndexMap::default(),
            fs,
            thread_store,
            _subscriptions: vec![settings_subscription],
        };
        this.refresh_profiles(cx);

        this
    }

    fn refresh_profiles(&mut self, cx: &mut Context<Self>) {
        let settings = AssistantSettings::get_global(cx);

        self.profiles = settings.profiles.clone();
    }

    fn build_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, |mut menu, _window, cx| {
            let settings = AssistantSettings::get_global(cx);
            let icon_position = IconPosition::Start;

            menu = menu.header("Profiles");
            for (profile_id, profile) in self.profiles.clone() {
                menu = menu.toggleable_entry(
                    profile.name.clone(),
                    profile_id == settings.default_profile,
                    icon_position,
                    None,
                    {
                        let fs = self.fs.clone();
                        let thread_store = self.thread_store.clone();
                        move |_window, cx| {
                            update_settings_file::<AssistantSettings>(fs.clone(), cx, {
                                let profile_id = profile_id.clone();
                                move |settings, _cx| {
                                    settings.set_profile(profile_id.clone());
                                }
                            });

                            thread_store
                                .update(cx, |this, cx| {
                                    this.load_default_profile(cx);
                                })
                                .log_err();
                        }
                    },
                );
            }

            menu = menu.separator();
            menu = menu.item(
                ContextMenuEntry::new("Configure Profiles")
                    .icon(IconName::Pencil)
                    .icon_color(Color::Muted)
                    .handler(move |window, cx| {
                        if let Some(workspace) = window.root().flatten() {
                            let workspace = workspace.downgrade();
                            window
                                .spawn(cx, async |cx| {
                                    Self::open_profiles_setting_in_editor(workspace, cx).await
                                })
                                .detach_and_log_err(cx);
                        }
                    }),
            );

            menu
        })
    }

    async fn open_profiles_setting_in_editor(
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let settings_editor = workspace
            .update_in(cx, |_, window, cx| {
                create_and_open_local_file(paths::settings_file(), window, cx, || {
                    settings::initial_user_settings_content().as_ref().into()
                })
            })?
            .await?
            .downcast::<Editor>()
            .unwrap();

        settings_editor
            .downgrade()
            .update_in(cx, |editor, window, cx| {
                let text = editor.buffer().read(cx).snapshot(cx).text();

                let settings = cx.global::<SettingsStore>();

                let edits =
                    settings.edits_for_update::<AssistantSettings>(
                        &text,
                        |settings| match settings {
                            assistant_settings::AssistantSettingsContent::Versioned(settings) => {
                                match settings {
                                    assistant_settings::VersionedAssistantSettingsContent::V2(
                                        settings,
                                    ) => {
                                        settings.profiles.get_or_insert_with(IndexMap::default);
                                    }
                                    assistant_settings::VersionedAssistantSettingsContent::V1(
                                        _,
                                    ) => {}
                                }
                            }
                            assistant_settings::AssistantSettingsContent::Legacy(_) => {}
                        },
                    );

                if !edits.is_empty() {
                    editor.edit(edits.iter().cloned(), cx);
                }

                let text = editor.buffer().read(cx).snapshot(cx).text();

                static PROFILES_REGEX: LazyLock<Regex> =
                    LazyLock::new(|| Regex::new(r#"(?P<key>"profiles":)\s*\{"#).unwrap());
                let range = PROFILES_REGEX.captures(&text).and_then(|captures| {
                    captures
                        .name("key")
                        .map(|inner_match| inner_match.start()..inner_match.end())
                });
                if let Some(range) = range {
                    editor.change_selections(
                        Some(Autoscroll::newest()),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges(vec![range]);
                        },
                    );
                }
            })?;

        anyhow::Ok(())
    }
}

impl Render for ProfileSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = AssistantSettings::get_global(cx);
        let profile = settings
            .profiles
            .get(&settings.default_profile)
            .map(|profile| profile.name.clone())
            .unwrap_or_else(|| "Unknown".into());

        let this = cx.entity().clone();
        PopoverMenu::new("tool-selector")
            .menu(move |window, cx| {
                Some(this.update(cx, |this, cx| this.build_context_menu(window, cx)))
            })
            .trigger_with_tooltip(
                Button::new("profile-selector-button", profile)
                    .style(ButtonStyle::Filled)
                    .label_size(LabelSize::Small),
                Tooltip::text("Change Profile"),
            )
            .anchor(gpui::Corner::BottomLeft)
    }
}
