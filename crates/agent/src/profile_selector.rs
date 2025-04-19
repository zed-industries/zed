use std::sync::Arc;

use assistant_settings::{AgentProfile, AgentProfileId, AssistantSettings};
use fs::Fs;
use gpui::{Action, Entity, FocusHandle, Subscription, WeakEntity, prelude::*};
use indexmap::IndexMap;
use language_model::LanguageModelRegistry;
use settings::{Settings as _, SettingsStore, update_settings_file};
use ui::{
    ButtonLike, ContextMenu, ContextMenuEntry, KeyBinding, PopoverMenu, PopoverMenuHandle, Tooltip,
    prelude::*,
};
use util::ResultExt as _;

use crate::{ManageProfiles, ThreadStore, ToggleProfileSelector};

pub struct ProfileSelector {
    profiles: IndexMap<AgentProfileId, AgentProfile>,
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
            let icon_position = IconPosition::End;

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
                                    this.load_profile_by_id(profile_id.clone(), cx);
                                })
                                .log_err();
                        }
                    },
                );
            }

            menu = menu.separator();
            menu = menu.header("Customize Current Profile");
            menu = menu.item(ContextMenuEntry::new("Tools…").handler({
                let profile_id = settings.default_profile.clone();
                move |window, cx| {
                    window.dispatch_action(
                        ManageProfiles::customize_tools(profile_id.clone()).boxed_clone(),
                        cx,
                    );
                }
            }));

            menu = menu.separator();
            menu = menu.item(ContextMenuEntry::new("Configure Profiles…").handler(
                move |window, cx| {
                    window.dispatch_action(ManageProfiles::default().boxed_clone(), cx);
                },
            ));

            menu
        })
    }
}

impl Render for ProfileSelector {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = AssistantSettings::get_global(cx);
        let profile_id = &settings.default_profile;
        let profile = settings.profiles.get(profile_id);

        let selected_profile = profile
            .map(|profile| profile.name.clone())
            .unwrap_or_else(|| "Unknown".into());

        let model_registry = LanguageModelRegistry::read_global(cx);
        let supports_tools = model_registry
            .default_model()
            .map_or(false, |default| default.model.supports_tools());

        let icon = match profile_id.as_str() {
            "write" => IconName::Pencil,
            "ask" => IconName::MessageBubbles,
            _ => IconName::UserRoundPen,
        };

        let this = cx.entity().clone();
        let focus_handle = self.focus_handle.clone();
        PopoverMenu::new("profile-selector")
            .menu(move |window, cx| {
                Some(this.update(cx, |this, cx| this.build_context_menu(window, cx)))
            })
            .trigger(if supports_tools {
                ButtonLike::new("profile-selector-button").child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted))
                        .child(
                            Label::new(selected_profile)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .child(
                            Icon::new(IconName::ChevronDown)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                        .child(div().opacity(0.5).children({
                            let focus_handle = focus_handle.clone();
                            KeyBinding::for_action_in(
                                &ToggleProfileSelector,
                                &focus_handle,
                                window,
                                cx,
                            )
                            .map(|kb| kb.size(rems_from_px(10.)))
                        })),
                )
            } else {
                ButtonLike::new("tools-not-supported-button")
                    .disabled(true)
                    .child(
                        h_flex().gap_1().child(
                            Label::new("No Tools")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .tooltip(Tooltip::text("The current model does not support tools."))
            })
            .anchor(gpui::Corner::BottomLeft)
            .with_handle(self.menu_handle.clone())
    }
}
