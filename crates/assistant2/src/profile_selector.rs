use std::sync::Arc;

use assistant_settings::{AgentProfile, AssistantSettings};
use fs::Fs;
use gpui::{prelude::*, Action, Entity, FocusHandle, Subscription, WeakEntity};
use indexmap::IndexMap;
use settings::{update_settings_file, Settings as _, SettingsStore};
use ui::{prelude::*, ContextMenu, ContextMenuEntry, PopoverMenu, PopoverMenuHandle, Tooltip};
use util::ResultExt as _;

use crate::{ManageProfiles, ThreadStore, ToggleProfileSelector};

pub struct ProfileSelector {
    profiles: IndexMap<Arc<str>, AgentProfile>,
    fs: Arc<dyn Fs>,
    thread_store: WeakEntity<ThreadStore>,
    focus_handle: FocusHandle,
    menu_handle: PopoverMenuHandle<ContextMenu>,
    _subscriptions: Vec<Subscription>,
}

impl ProfileSelector {
    pub fn new(
        fs: Arc<dyn Fs>,
        thread_store: WeakEntity<ThreadStore>,
        focus_handle: FocusHandle,
        cx: &mut Context<Self>,
    ) -> Self {
        let settings_subscription = cx.observe_global::<SettingsStore>(move |this, cx| {
            this.refresh_profiles(cx);
        });

        let mut this = Self {
            profiles: IndexMap::default(),
            fs,
            thread_store,
            focus_handle,
            menu_handle: PopoverMenuHandle::default(),
            _subscriptions: vec![settings_subscription],
        };
        this.refresh_profiles(cx);

        this
    }

    pub fn menu_handle(&self) -> PopoverMenuHandle<ContextMenu> {
        self.menu_handle.clone()
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
        let focus_handle = self.focus_handle.clone();
        PopoverMenu::new("profile-selector")
            .menu(move |window, cx| {
                Some(this.update(cx, |this, cx| this.build_context_menu(window, cx)))
            })
            .trigger_with_tooltip(
                Button::new("profile-selector-button", profile)
                    .style(ButtonStyle::Filled)
                    .label_size(LabelSize::Small),
                move |window, cx| {
                    Tooltip::for_action_in(
                        "Change Profile",
                        &ToggleProfileSelector,
                        &focus_handle,
                        window,
                        cx,
                    )
                },
            )
            .anchor(gpui::Corner::BottomLeft)
            .with_handle(self.menu_handle.clone())
    }
}
