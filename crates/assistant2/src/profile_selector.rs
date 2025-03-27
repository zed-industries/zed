use std::sync::Arc;

use assistant_settings::{AgentProfile, AssistantSettings};
use fs::Fs;
use gpui::{prelude::*, Action, Entity, Subscription, WeakEntity};
use indexmap::IndexMap;
use settings::{update_settings_file, Settings as _, SettingsStore};
use ui::{prelude::*, ContextMenu, ContextMenuEntry, PopoverMenu, Tooltip};
use util::ResultExt as _;

use crate::{ManageProfiles, ThreadStore};

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
                                    this.load_profile_by_id(&profile_id, cx);
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
                        window.dispatch_action(ManageProfiles.boxed_clone(), cx);
                    }),
            );

            menu
        })
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
