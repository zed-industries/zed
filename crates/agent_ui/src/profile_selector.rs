use crate::{ManageProfiles, ToggleProfileSelector};
use agent::agent_profile::{AgentProfile, AvailableProfiles};
use agent_settings::{AgentDockPosition, AgentProfileId, AgentSettings, builtin_profiles};
use fs::Fs;
use gpui::{Action, Entity, FocusHandle, Subscription, prelude::*};
use settings::{Settings as _, SettingsStore, update_settings_file};
use std::sync::Arc;
use ui::{
    ContextMenu, ContextMenuEntry, DocumentationSide, PopoverMenu, PopoverMenuHandle, Tooltip,
    prelude::*,
};

/// Trait for types that can provide and manage agent profiles
pub trait ProfileProvider {
    /// Get the current profile ID
    fn profile_id(&self, cx: &App) -> AgentProfileId;

    /// Set the profile ID
    fn set_profile(&self, profile_id: AgentProfileId, cx: &mut App);

    /// Check if profiles are supported in the current context (e.g. if the model that is selected has tool support)
    fn profiles_supported(&self, cx: &App) -> bool;
}

pub struct ProfileSelector {
    profiles: AvailableProfiles,
    fs: Arc<dyn Fs>,
    provider: Arc<dyn ProfileProvider>,
    menu_handle: PopoverMenuHandle<ContextMenu>,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl ProfileSelector {
    pub fn new(
        fs: Arc<dyn Fs>,
        provider: Arc<dyn ProfileProvider>,
        focus_handle: FocusHandle,
        cx: &mut Context<Self>,
    ) -> Self {
        let settings_subscription = cx.observe_global::<SettingsStore>(move |this, cx| {
            this.refresh_profiles(cx);
        });

        Self {
            profiles: AgentProfile::available_profiles(cx),
            fs,
            provider,
            menu_handle: PopoverMenuHandle::default(),
            focus_handle,
            _subscriptions: vec![settings_subscription],
        }
    }

    pub fn menu_handle(&self) -> PopoverMenuHandle<ContextMenu> {
        self.menu_handle.clone()
    }

    fn refresh_profiles(&mut self, cx: &mut Context<Self>) {
        self.profiles = AgentProfile::available_profiles(cx);
    }

    fn build_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, |mut menu, _window, cx| {
            let settings = AgentSettings::get_global(cx);

            let mut found_non_builtin = false;
            for (profile_id, profile_name) in self.profiles.iter() {
                if !builtin_profiles::is_builtin(profile_id) {
                    found_non_builtin = true;
                    continue;
                }
                menu = menu.item(self.menu_entry_for_profile(
                    profile_id.clone(),
                    profile_name,
                    settings,
                    cx,
                ));
            }

            if found_non_builtin {
                menu = menu.separator().header("Custom Profiles");
                for (profile_id, profile_name) in self.profiles.iter() {
                    if builtin_profiles::is_builtin(profile_id) {
                        continue;
                    }
                    menu = menu.item(self.menu_entry_for_profile(
                        profile_id.clone(),
                        profile_name,
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
        profile_name: &SharedString,
        settings: &AgentSettings,
        cx: &App,
    ) -> ContextMenuEntry {
        let documentation = match profile_name.to_lowercase().as_str() {
            builtin_profiles::WRITE => Some("Get help to write anything."),
            builtin_profiles::ASK => Some("Chat about your codebase."),
            builtin_profiles::MINIMAL => Some("Chat about anything with no tools."),
            _ => None,
        };
        let thread_profile_id = self.provider.profile_id(cx);

        let entry = ContextMenuEntry::new(profile_name.clone())
            .toggleable(IconPosition::End, profile_id == thread_profile_id);

        let entry = if let Some(doc_text) = documentation {
            entry.documentation_aside(documentation_side(settings.dock), move |_| {
                Label::new(doc_text).into_any_element()
            })
        } else {
            entry
        };

        entry.handler({
            let fs = self.fs.clone();
            let provider = self.provider.clone();
            move |_window, cx| {
                update_settings_file::<AgentSettings>(fs.clone(), cx, {
                    let profile_id = profile_id.clone();
                    move |settings, _cx| {
                        settings.set_profile(profile_id);
                    }
                });

                provider.set_profile(profile_id.clone(), cx);
            }
        })
    }
}

impl Render for ProfileSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = AgentSettings::get_global(cx);
        let profile_id = self.provider.profile_id(cx);
        let profile = settings.profiles.get(&profile_id);

        let selected_profile = profile
            .map(|profile| profile.name.clone())
            .unwrap_or_else(|| "Unknown".into());

        if self.provider.profiles_supported(cx) {
            let this = cx.entity();
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

fn documentation_side(position: AgentDockPosition) -> DocumentationSide {
    match position {
        AgentDockPosition::Left => DocumentationSide::Right,
        AgentDockPosition::Bottom => DocumentationSide::Left,
        AgentDockPosition::Right => DocumentationSide::Left,
    }
}
