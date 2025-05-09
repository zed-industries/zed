use assistant_settings::{
    AgentProfile, AgentProfileId, AssistantDockPosition, AssistantSettings, GroupedAgentProfiles,
    builtin_profiles,
};
use gpui::{Action, Entity, FocusHandle, Subscription, WeakEntity, prelude::*};
use language_model::LanguageModelRegistry;
use settings::{Settings as _, SettingsStore};
use ui::{
    ContextMenu, ContextMenuEntry, DocumentationSide, PopoverMenu, PopoverMenuHandle, Tooltip,
    prelude::*,
};
use util::ResultExt as _;

use crate::{ManageProfiles, Thread, ThreadStore, ToggleProfileSelector};

pub struct ProfileSelector {
    profiles: GroupedAgentProfiles,
    thread: Entity<Thread>,
    thread_store: WeakEntity<ThreadStore>,
    menu_handle: PopoverMenuHandle<ContextMenu>,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl ProfileSelector {
    pub fn new(
        thread: Entity<Thread>,
        thread_store: WeakEntity<ThreadStore>,
        focus_handle: FocusHandle,
        cx: &mut Context<Self>,
    ) -> Self {
        let settings_subscription = cx.observe_global::<SettingsStore>(move |this, cx| {
            this.refresh_profiles(cx);
        });

        Self {
            profiles: GroupedAgentProfiles::from_settings(AssistantSettings::get_global(cx)),
            thread,
            thread_store,
            menu_handle: PopoverMenuHandle::default(),
            focus_handle,
            _subscriptions: vec![settings_subscription],
        }
    }

    pub fn menu_handle(&self) -> PopoverMenuHandle<ContextMenu> {
        self.menu_handle.clone()
    }

    fn refresh_profiles(&mut self, cx: &mut Context<Self>) {
        self.profiles = GroupedAgentProfiles::from_settings(AssistantSettings::get_global(cx));
    }

    fn build_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, |mut menu, _window, cx| {
            let settings = AssistantSettings::get_global(cx);
            for (profile_id, profile) in self.profiles.builtin.iter() {
                menu = menu.item(self.menu_entry_for_profile(
                    profile_id.clone(),
                    profile,
                    settings,
                    cx,
                ));
            }

            if !self.profiles.custom.is_empty() {
                menu = menu.separator().header("Custom Profiles");
                for (profile_id, profile) in self.profiles.custom.iter() {
                    menu = menu.item(self.menu_entry_for_profile(
                        profile_id.clone(),
                        profile,
                        settings,
                        cx,
                    ));
                }
            }

            menu = menu.separator();
            menu = menu.item(ContextMenuEntry::new("Configure Profiles…").handler(
                move |window, cx| {
                    window.dispatch_action(ManageProfiles::default().boxed_clone(), cx);
                },
            ));

            menu
        })
    }

    fn menu_entry_for_profile(
        &self,
        profile_id: AgentProfileId,
        profile: &AgentProfile,
        settings: &AssistantSettings,
        cx: &App,
    ) -> ContextMenuEntry {
        let documentation = match profile.name.to_lowercase().as_str() {
            builtin_profiles::WRITE => Some("Get help to write anything."),
            builtin_profiles::ASK => Some("Chat about your codebase."),
            builtin_profiles::MINIMAL => Some("Chat about anything with no tools."),
            _ => None,
        };

        let current_profile_id = self.thread.read(cx).configured_profile_id();

        let entry = ContextMenuEntry::new(profile.name.clone()).toggleable(
            IconPosition::End,
            Some(profile_id.clone()) == current_profile_id,
        );

        let entry = if let Some(doc_text) = documentation {
            entry.documentation_aside(documentation_side(settings.dock), move |_| {
                Label::new(doc_text).into_any_element()
            })
        } else {
            entry
        };

        entry.handler({
            let thread_store = self.thread_store.clone();
            let profile_id = profile_id.clone();
            let thread = self.thread.clone();

            move |_window, cx| {
                thread.update(cx, |thread, cx| {
                    thread.set_configured_profile_id(Some(profile_id.clone()), cx);
                });

                thread_store
                    .update(cx, |this, cx| {
                        this.load_profile_by_id(profile_id.clone(), cx);
                    })
                    .log_err();
            }
        })
    }
}

impl Render for ProfileSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = AssistantSettings::get_global(cx);
        let profile_id = self
            .thread
            .read(cx)
            .configured_profile_id()
            .unwrap_or(settings.default_profile.clone());
        let profile = settings.profiles.get(&profile_id).cloned();

        let selected_profile = profile
            .map(|profile| profile.name.clone())
            .unwrap_or_else(|| "Unknown".into());

        let configured_model = self
            .thread
            .read_with(cx, |thread, _cx| thread.configured_model())
            .or_else(|| {
                let model_registry = LanguageModelRegistry::read_global(cx);
                model_registry.default_model()
            });
        let supports_tools =
            configured_model.map_or(false, |default| default.model.supports_tools());

        if supports_tools {
            let this = cx.entity().clone();
            let focus_handle = self.focus_handle.clone();
            let trigger_button = Button::new("profile-selector-model", selected_profile)
                .label_size(LabelSize::Small)
                .color(Color::Muted)
                .icon(IconName::ChevronDown)
                .icon_size(IconSize::XSmall)
                .icon_position(IconPosition::End)
                .icon_color(Color::Muted);

            PopoverMenu::new("profile-selector")
                .trigger_with_tooltip(trigger_button, {
                    let focus_handle = focus_handle.clone();
                    move |window, cx| {
                        Tooltip::for_action_in(
                            "Toggle Profile Menu",
                            &ToggleProfileSelector,
                            &focus_handle,
                            window,
                            cx,
                        )
                    }
                })
                .anchor(
                    if documentation_side(settings.dock) == DocumentationSide::Left {
                        gpui::Corner::BottomRight
                    } else {
                        gpui::Corner::BottomLeft
                    },
                )
                .with_handle(self.menu_handle.clone())
                .menu(move |window, cx| {
                    Some(this.update(cx, |this, cx| this.build_context_menu(window, cx)))
                })
                .into_any_element()
        } else {
            Button::new("tools-not-supported-button", "Tools Unsupported")
                .disabled(true)
                .label_size(LabelSize::Small)
                .color(Color::Muted)
                .tooltip(Tooltip::text("This model does not support tools."))
                .into_any_element()
        }
    }
}

fn documentation_side(position: AssistantDockPosition) -> DocumentationSide {
    match position {
        AssistantDockPosition::Left => DocumentationSide::Right,
        AssistantDockPosition::Bottom => DocumentationSide::Left,
        AssistantDockPosition::Right => DocumentationSide::Left,
    }
}
