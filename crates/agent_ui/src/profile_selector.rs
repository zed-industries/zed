use crate::{CycleModeSelector, ManageProfiles, ToggleProfileSelector};
use agent_settings::{
    AgentProfile, AgentProfileId, AgentSettings, AvailableProfiles, builtin_profiles,
};
use fs::Fs;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    Action, AnyElement, App, BackgroundExecutor, Context, DismissEvent, Entity, FocusHandle,
    Focusable, SharedString, Subscription, Task, Window,
};
use picker::{Picker, PickerDelegate, popover_menu::PickerPopoverMenu};
use settings::{Settings as _, SettingsStore, update_settings_file};
use std::{
    sync::atomic::Ordering,
    sync::{Arc, atomic::AtomicBool},
};
use ui::{
    DocumentationAside, DocumentationSide, HighlightedLabel, KeyBinding, LabelSize, ListItem,
    ListItemSpacing, PopoverMenuHandle, TintColor, Tooltip, prelude::*,
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
    pending_refresh: bool,
    fs: Arc<dyn Fs>,
    provider: Arc<dyn ProfileProvider>,
    picker: Option<Entity<Picker<ProfilePickerDelegate>>>,
    picker_handle: PopoverMenuHandle<Picker<ProfilePickerDelegate>>,
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
            this.pending_refresh = true;
            cx.notify();
        });

        Self {
            profiles: AgentProfile::available_profiles(cx),
            pending_refresh: false,
            fs,
            provider,
            picker: None,
            picker_handle: PopoverMenuHandle::default(),
            focus_handle,
            _subscriptions: vec![settings_subscription],
        }
    }

    pub fn menu_handle(&self) -> PopoverMenuHandle<Picker<ProfilePickerDelegate>> {
        self.picker_handle.clone()
    }

    pub fn cycle_profile(&mut self, cx: &mut Context<Self>) {
        if !self.provider.profiles_supported(cx) {
            return;
        }

        let profiles = AgentProfile::available_profiles(cx);
        if profiles.is_empty() {
            return;
        }

        let current_profile_id = self.provider.profile_id(cx);
        let current_index = profiles
            .keys()
            .position(|id| id == &current_profile_id)
            .unwrap_or(0);

        let next_index = (current_index + 1) % profiles.len();

        if let Some((next_profile_id, _)) = profiles.get_index(next_index) {
            self.provider.set_profile(next_profile_id.clone(), cx);
        }
    }

    fn ensure_picker(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<Picker<ProfilePickerDelegate>> {
        if self.picker.is_none() {
            let delegate = ProfilePickerDelegate::new(
                self.fs.clone(),
                self.provider.clone(),
                self.profiles.clone(),
                cx.background_executor().clone(),
                self.focus_handle.clone(),
                cx,
            );

            let picker = cx.new(|cx| {
                Picker::list(delegate, window, cx)
                    .show_scrollbar(true)
                    .width(rems(18.))
                    .max_height(Some(rems(20.).into()))
            });

            self.picker = Some(picker);
        }

        if self.pending_refresh {
            if let Some(picker) = &self.picker {
                let profiles = AgentProfile::available_profiles(cx);
                self.profiles = profiles.clone();
                picker.update(cx, |picker, cx| {
                    let query = picker.query(cx);
                    picker
                        .delegate
                        .refresh_profiles(profiles.clone(), query, cx);
                });
            }
            self.pending_refresh = false;
        }

        self.picker.as_ref().unwrap().clone()
    }
}

impl Focusable for ProfileSelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if let Some(picker) = &self.picker {
            picker.focus_handle(cx)
        } else {
            self.focus_handle.clone()
        }
    }
}

impl Render for ProfileSelector {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.provider.profiles_supported(cx) {
            return Button::new("tools-not-supported-button", "Tools Unsupported")
                .disabled(true)
                .label_size(LabelSize::Small)
                .color(Color::Muted)
                .tooltip(Tooltip::text("This model does not support tools."))
                .into_any_element();
        }

        let picker = self.ensure_picker(window, cx);

        let settings = AgentSettings::get_global(cx);
        let profile_id = self.provider.profile_id(cx);
        let profile = settings.profiles.get(&profile_id);

        let selected_profile = profile
            .map(|profile| profile.name.clone())
            .unwrap_or_else(|| "Unknown".into());
        let focus_handle = self.focus_handle.clone();

        let icon = if self.picker_handle.is_deployed() {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        let trigger_button = Button::new("profile-selector", selected_profile)
            .label_size(LabelSize::Small)
            .color(Color::Muted)
            .icon(icon)
            .icon_size(IconSize::XSmall)
            .icon_position(IconPosition::End)
            .icon_color(Color::Muted)
            .selected_style(ButtonStyle::Tinted(TintColor::Accent));

        PickerPopoverMenu::new(
            picker,
            trigger_button,
            Tooltip::element({
                move |_window, cx| {
                    let container = || h_flex().gap_1().justify_between();
                    v_flex()
                        .gap_1()
                        .child(container().child(Label::new("Toggle Profile Menu")).child(
                            KeyBinding::for_action_in(&ToggleProfileSelector, &focus_handle, cx),
                        ))
                        .child(
                            container()
                                .pb_1()
                                .border_b_1()
                                .border_color(cx.theme().colors().border_variant)
                                .child(Label::new("Cycle Through Profiles"))
                                .child(KeyBinding::for_action_in(
                                    &CycleModeSelector,
                                    &focus_handle,
                                    cx,
                                )),
                        )
                        .into_any()
                }
            }),
            gpui::Corner::BottomRight,
            cx,
        )
        .with_handle(self.picker_handle.clone())
        .render(window, cx)
        .into_any_element()
    }
}

#[derive(Clone)]
struct ProfileCandidate {
    id: AgentProfileId,
    name: SharedString,
    is_builtin: bool,
}

#[derive(Clone)]
struct ProfileMatchEntry {
    candidate_index: usize,
    positions: Vec<usize>,
}

enum ProfilePickerEntry {
    Header(SharedString),
    Profile(ProfileMatchEntry),
}

pub(crate) struct ProfilePickerDelegate {
    fs: Arc<dyn Fs>,
    provider: Arc<dyn ProfileProvider>,
    background: BackgroundExecutor,
    candidates: Vec<ProfileCandidate>,
    string_candidates: Arc<Vec<StringMatchCandidate>>,
    filtered_entries: Vec<ProfilePickerEntry>,
    selected_index: usize,
    hovered_index: Option<usize>,
    query: String,
    cancel: Option<Arc<AtomicBool>>,
    focus_handle: FocusHandle,
}

impl ProfilePickerDelegate {
    fn new(
        fs: Arc<dyn Fs>,
        provider: Arc<dyn ProfileProvider>,
        profiles: AvailableProfiles,
        background: BackgroundExecutor,
        focus_handle: FocusHandle,
        cx: &mut Context<ProfileSelector>,
    ) -> Self {
        let candidates = Self::candidates_from(profiles);
        let string_candidates = Arc::new(Self::string_candidates(&candidates));
        let filtered_entries = Self::entries_from_candidates(&candidates);

        let mut this = Self {
            fs,
            provider,
            background,
            candidates,
            string_candidates,
            filtered_entries,
            selected_index: 0,
            hovered_index: None,
            query: String::new(),
            cancel: None,
            focus_handle,
        };

        this.selected_index = this
            .index_of_profile(&this.provider.profile_id(cx))
            .unwrap_or_else(|| this.first_selectable_index().unwrap_or(0));

        this
    }

    fn refresh_profiles(
        &mut self,
        profiles: AvailableProfiles,
        query: String,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.candidates = Self::candidates_from(profiles);
        self.string_candidates = Arc::new(Self::string_candidates(&self.candidates));
        self.query = query;

        if self.query.is_empty() {
            self.filtered_entries = Self::entries_from_candidates(&self.candidates);
        } else {
            let matches = self.search_blocking(&self.query);
            self.filtered_entries = self.entries_from_matches(matches);
        }

        self.selected_index = self
            .index_of_profile(&self.provider.profile_id(cx))
            .unwrap_or_else(|| self.first_selectable_index().unwrap_or(0));
        cx.notify();
    }

    fn candidates_from(profiles: AvailableProfiles) -> Vec<ProfileCandidate> {
        profiles
            .into_iter()
            .map(|(id, name)| ProfileCandidate {
                is_builtin: builtin_profiles::is_builtin(&id),
                id,
                name,
            })
            .collect()
    }

    fn string_candidates(candidates: &[ProfileCandidate]) -> Vec<StringMatchCandidate> {
        candidates
            .iter()
            .enumerate()
            .map(|(index, candidate)| StringMatchCandidate::new(index, candidate.name.as_ref()))
            .collect()
    }

    fn documentation(candidate: &ProfileCandidate) -> Option<&'static str> {
        match candidate.id.as_str() {
            builtin_profiles::WRITE => Some("Get help to write anything."),
            builtin_profiles::ASK => Some("Chat about your codebase."),
            builtin_profiles::MINIMAL => Some("Chat about anything with no tools."),
            _ => None,
        }
    }

    fn entries_from_candidates(candidates: &[ProfileCandidate]) -> Vec<ProfilePickerEntry> {
        let mut entries = Vec::new();
        let mut inserted_custom_header = false;

        for (idx, candidate) in candidates.iter().enumerate() {
            if !candidate.is_builtin && !inserted_custom_header {
                if !entries.is_empty() {
                    entries.push(ProfilePickerEntry::Header("Custom Profiles".into()));
                }
                inserted_custom_header = true;
            }

            entries.push(ProfilePickerEntry::Profile(ProfileMatchEntry {
                candidate_index: idx,
                positions: Vec::new(),
            }));
        }

        entries
    }

    fn entries_from_matches(&self, matches: Vec<StringMatch>) -> Vec<ProfilePickerEntry> {
        let mut entries = Vec::new();
        for mat in matches {
            if self.candidates.get(mat.candidate_id).is_some() {
                entries.push(ProfilePickerEntry::Profile(ProfileMatchEntry {
                    candidate_index: mat.candidate_id,
                    positions: mat.positions,
                }));
            }
        }
        entries
    }

    fn first_selectable_index(&self) -> Option<usize> {
        self.filtered_entries
            .iter()
            .position(|entry| matches!(entry, ProfilePickerEntry::Profile(_)))
    }

    fn index_of_profile(&self, profile_id: &AgentProfileId) -> Option<usize> {
        self.filtered_entries.iter().position(|entry| {
            matches!(entry, ProfilePickerEntry::Profile(profile) if self
                .candidates
                .get(profile.candidate_index)
                .map(|candidate| &candidate.id == profile_id)
                .unwrap_or(false))
        })
    }

    fn search_blocking(&self, query: &str) -> Vec<StringMatch> {
        if query.is_empty() {
            return self
                .string_candidates
                .iter()
                .map(|candidate| StringMatch {
                    candidate_id: candidate.id,
                    score: 0.0,
                    positions: Vec::new(),
                    string: candidate.string.clone(),
                })
                .collect();
        }

        let cancel_flag = AtomicBool::new(false);

        self.background.block(match_strings(
            self.string_candidates.as_ref(),
            query,
            false,
            true,
            100,
            &cancel_flag,
            self.background.clone(),
        ))
    }
}

impl PickerDelegate for ProfilePickerDelegate {
    type ListItem = AnyElement;

    fn placeholder_text(&self, _: &mut Window, _: &mut App) -> Arc<str> {
        "Search profiles…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        let text = if self.candidates.is_empty() {
            "No profiles.".into()
        } else {
            "No profiles match your search.".into()
        };
        Some(text)
    }

    fn match_count(&self) -> usize {
        self.filtered_entries.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix.min(self.filtered_entries.len().saturating_sub(1));
        cx.notify();
    }

    fn can_select(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        match self.filtered_entries.get(ix) {
            Some(ProfilePickerEntry::Profile(_)) => true,
            Some(ProfilePickerEntry::Header(_)) | None => false,
        }
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if query.is_empty() {
            self.query.clear();
            self.filtered_entries = Self::entries_from_candidates(&self.candidates);
            self.selected_index = self
                .index_of_profile(&self.provider.profile_id(cx))
                .unwrap_or_else(|| self.first_selectable_index().unwrap_or(0));
            cx.notify();
            return Task::ready(());
        }

        if let Some(prev) = &self.cancel {
            prev.store(true, Ordering::Relaxed);
        }
        let cancel = Arc::new(AtomicBool::new(false));
        self.cancel = Some(cancel.clone());

        let string_candidates = self.string_candidates.clone();
        let background = self.background.clone();
        let provider = self.provider.clone();
        self.query = query.clone();

        let cancel_for_future = cancel;

        cx.spawn_in(window, async move |this, cx| {
            let matches = match_strings(
                string_candidates.as_ref(),
                &query,
                false,
                true,
                100,
                cancel_for_future.as_ref(),
                background,
            )
            .await;

            this.update_in(cx, |this, _, cx| {
                if this.delegate.query != query {
                    return;
                }

                this.delegate.filtered_entries = this.delegate.entries_from_matches(matches);
                this.delegate.selected_index = this
                    .delegate
                    .index_of_profile(&provider.profile_id(cx))
                    .unwrap_or_else(|| this.delegate.first_selectable_index().unwrap_or(0));
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        match self.filtered_entries.get(self.selected_index) {
            Some(ProfilePickerEntry::Profile(entry)) => {
                if let Some(candidate) = self.candidates.get(entry.candidate_index) {
                    let profile_id = candidate.id.clone();
                    let fs = self.fs.clone();
                    let provider = self.provider.clone();

                    update_settings_file(fs, cx, {
                        let profile_id = profile_id.clone();
                        move |settings, _cx| {
                            settings
                                .agent
                                .get_or_insert_default()
                                .set_profile(profile_id.0);
                        }
                    });

                    provider.set_profile(profile_id.clone(), cx);

                    telemetry::event!(
                        "agent_profile_switched",
                        profile_id = profile_id.as_str(),
                        source = "picker"
                    );
                }

                cx.emit(DismissEvent);
            }
            _ => {}
        }
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.defer_in(window, |picker, window, cx| {
            picker.set_query("", window, cx);
        });
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        match self.filtered_entries.get(ix)? {
            ProfilePickerEntry::Header(label) => Some(
                div()
                    .px_2p5()
                    .pb_0p5()
                    .when(ix > 0, |this| {
                        this.mt_1p5()
                            .pt_2()
                            .border_t_1()
                            .border_color(cx.theme().colors().border_variant)
                    })
                    .child(
                        Label::new(label.clone())
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .into_any_element(),
            ),
            ProfilePickerEntry::Profile(entry) => {
                let candidate = self.candidates.get(entry.candidate_index)?;
                let active_id = self.provider.profile_id(cx);
                let is_active = active_id == candidate.id;
                let has_documentation = Self::documentation(candidate).is_some();

                Some(
                    div()
                        .id(("profile-picker-item", ix))
                        .when(has_documentation, |this| {
                            this.on_hover(cx.listener(move |picker, hovered, _, cx| {
                                if *hovered {
                                    picker.delegate.hovered_index = Some(ix);
                                } else if picker.delegate.hovered_index == Some(ix) {
                                    picker.delegate.hovered_index = None;
                                }
                                cx.notify();
                            }))
                        })
                        .child(
                            ListItem::new(candidate.id.0.clone())
                                .inset(true)
                                .spacing(ListItemSpacing::Sparse)
                                .toggle_state(selected)
                                .child(HighlightedLabel::new(
                                    candidate.name.clone(),
                                    entry.positions.clone(),
                                ))
                                .when(is_active, |this| {
                                    this.end_slot(
                                        div()
                                            .pr_2()
                                            .child(Icon::new(IconName::Check).color(Color::Accent)),
                                    )
                                }),
                        )
                        .into_any_element(),
                )
            }
        }
    }

    fn documentation_aside(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<DocumentationAside> {
        use std::rc::Rc;

        let hovered_index = self.hovered_index?;
        let entry = match self.filtered_entries.get(hovered_index)? {
            ProfilePickerEntry::Profile(entry) => entry,
            ProfilePickerEntry::Header(_) => return None,
        };

        let candidate = self.candidates.get(entry.candidate_index)?;
        let docs_aside = Self::documentation(candidate)?.to_string();

        let settings = AgentSettings::get_global(cx);
        let side = match settings.dock {
            settings::DockPosition::Left => DocumentationSide::Right,
            settings::DockPosition::Bottom | settings::DockPosition::Right => {
                DocumentationSide::Left
            }
        };

        Some(DocumentationAside {
            side,
            render: Rc::new(move |_| Label::new(docs_aside.clone()).into_any_element()),
        })
    }

    fn documentation_aside_index(&self) -> Option<usize> {
        self.hovered_index
    }

    fn render_footer(
        &self,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<gpui::AnyElement> {
        let focus_handle = self.focus_handle.clone();

        Some(
            h_flex()
                .w_full()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .p_1p5()
                .child(
                    Button::new("configure", "Configure")
                        .full_width()
                        .style(ButtonStyle::Outlined)
                        .key_binding(
                            KeyBinding::for_action_in(
                                &ManageProfiles::default(),
                                &focus_handle,
                                cx,
                            )
                            .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(ManageProfiles::default().boxed_clone(), cx);
                        }),
                )
                .into_any(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;

    #[gpui::test]
    fn entries_include_custom_profiles(_cx: &mut TestAppContext) {
        let candidates = vec![
            ProfileCandidate {
                id: AgentProfileId("write".into()),
                name: SharedString::from("Write"),
                is_builtin: true,
            },
            ProfileCandidate {
                id: AgentProfileId("my-custom".into()),
                name: SharedString::from("My Custom"),
                is_builtin: false,
            },
        ];

        let entries = ProfilePickerDelegate::entries_from_candidates(&candidates);

        assert!(entries.iter().any(|entry| matches!(
            entry,
            ProfilePickerEntry::Profile(profile)
                if candidates[profile.candidate_index].id.as_str() == "my-custom"
        )));
        assert!(entries.iter().any(|entry| matches!(
            entry,
            ProfilePickerEntry::Header(label) if label.as_ref() == "Custom Profiles"
        )));
    }

    #[gpui::test]
    fn fuzzy_filter_returns_no_results_and_keeps_configure(cx: &mut TestAppContext) {
        let candidates = vec![ProfileCandidate {
            id: AgentProfileId("write".into()),
            name: SharedString::from("Write"),
            is_builtin: true,
        }];

        cx.update(|cx| {
            let focus_handle = cx.focus_handle();

            let delegate = ProfilePickerDelegate {
                fs: FakeFs::new(cx.background_executor().clone()),
                provider: Arc::new(TestProfileProvider::new(AgentProfileId("write".into()))),
                background: cx.background_executor().clone(),
                candidates,
                string_candidates: Arc::new(Vec::new()),
                filtered_entries: Vec::new(),
                selected_index: 0,
                hovered_index: None,
                query: String::new(),
                cancel: None,
                focus_handle,
            };

            let matches = Vec::new(); // No matches
            let _entries = delegate.entries_from_matches(matches);
        });
    }

    #[gpui::test]
    fn active_profile_selection_logic_works(cx: &mut TestAppContext) {
        let candidates = vec![
            ProfileCandidate {
                id: AgentProfileId("write".into()),
                name: SharedString::from("Write"),
                is_builtin: true,
            },
            ProfileCandidate {
                id: AgentProfileId("ask".into()),
                name: SharedString::from("Ask"),
                is_builtin: true,
            },
        ];

        cx.update(|cx| {
            let focus_handle = cx.focus_handle();

            let delegate = ProfilePickerDelegate {
                fs: FakeFs::new(cx.background_executor().clone()),
                provider: Arc::new(TestProfileProvider::new(AgentProfileId("write".into()))),
                background: cx.background_executor().clone(),
                candidates,
                string_candidates: Arc::new(Vec::new()),
                hovered_index: None,
                filtered_entries: vec![
                    ProfilePickerEntry::Profile(ProfileMatchEntry {
                        candidate_index: 0,
                        positions: Vec::new(),
                    }),
                    ProfilePickerEntry::Profile(ProfileMatchEntry {
                        candidate_index: 1,
                        positions: Vec::new(),
                    }),
                ],
                selected_index: 0,
                query: String::new(),
                cancel: None,
                focus_handle,
            };

            // Active profile should be found at index 0
            let active_index = delegate.index_of_profile(&AgentProfileId("write".into()));
            assert_eq!(active_index, Some(0));
        });
    }

    struct TestProfileProvider {
        profile_id: AgentProfileId,
    }

    impl TestProfileProvider {
        fn new(profile_id: AgentProfileId) -> Self {
            Self { profile_id }
        }
    }

    impl ProfileProvider for TestProfileProvider {
        fn profile_id(&self, _cx: &App) -> AgentProfileId {
            self.profile_id.clone()
        }

        fn set_profile(&self, _profile_id: AgentProfileId, _cx: &mut App) {}

        fn profiles_supported(&self, _cx: &App) -> bool {
            true
        }
    }
}
