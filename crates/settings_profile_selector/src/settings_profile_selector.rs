use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, Render, Task, WeakEntity, Window,
};
use picker::{Picker, PickerDelegate};
use settings::{ActiveSettingsProfileName, SettingsStore};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use workspace::{ModalView, Workspace};

pub fn init(cx: &mut App) {
    cx.on_action(|_: &zed_actions::settings_profile_selector::Toggle, cx| {
        workspace::with_active_or_new_workspace(cx, |workspace, window, cx| {
            toggle_settings_profile_selector(workspace, window, cx);
        });
    });
}

fn toggle_settings_profile_selector(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    workspace.toggle_modal(window, cx, |window, cx| {
        let delegate = SettingsProfileSelectorDelegate::new(cx.entity().downgrade(), window, cx);
        SettingsProfileSelector::new(delegate, window, cx)
    });
}

pub struct SettingsProfileSelector {
    picker: Entity<Picker<SettingsProfileSelectorDelegate>>,
}

impl ModalView for SettingsProfileSelector {}

impl EventEmitter<DismissEvent> for SettingsProfileSelector {}

impl Focusable for SettingsProfileSelector {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for SettingsProfileSelector {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl SettingsProfileSelector {
    pub fn new(
        delegate: SettingsProfileSelectorDelegate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

pub struct SettingsProfileSelectorDelegate {
    matches: Vec<StringMatch>,
    profile_names: Vec<Option<String>>,
    original_profile_name: Option<String>,
    selected_profile: Option<String>,
    selected_index: usize,
    selection_completed: bool,
    selector: WeakEntity<SettingsProfileSelector>,
}

impl SettingsProfileSelectorDelegate {
    fn new(
        selector: WeakEntity<SettingsProfileSelector>,
        _: &mut Window,
        cx: &mut Context<SettingsProfileSelector>,
    ) -> Self {
        let settings_store = cx.global::<SettingsStore>();
        let mut profile_names: Vec<String> = settings_store
            .available_profiles()
            .map(|s| s.to_string())
            .collect();

        profile_names.sort();
        let mut profile_names: Vec<_> = profile_names.into_iter().map(Some).collect();
        profile_names.insert(0, None);

        let matches = profile_names
            .iter()
            .enumerate()
            .map(|(ix, profile_name)| StringMatch {
                candidate_id: ix,
                score: 0.0,
                positions: Default::default(),
                string: display_name(profile_name),
            })
            .collect();

        let original_profile_name = cx
            .try_global::<ActiveSettingsProfileName>()
            .map(|p| p.0.clone());

        let mut this = Self {
            matches,
            profile_names,
            original_profile_name,
            selected_profile: None,
            selected_index: 0,
            selection_completed: false,
            selector,
        };

        if let Some(active_profile_name) = cx.try_global::<ActiveSettingsProfileName>() {
            this.select_if_matching(&active_profile_name.0);
        }

        this
    }

    fn select_if_matching(&mut self, profile_name: &str) {
        self.selected_index = self
            .matches
            .iter()
            .position(|mat| mat.string == profile_name)
            .unwrap_or(self.selected_index);
    }

    fn set_selected_profile(
        &self,
        cx: &mut Context<Picker<SettingsProfileSelectorDelegate>>,
    ) -> Option<String> {
        let Some(mat) = self.matches.get(self.selected_index) else {
            return None;
        };

        let name = self.profile_names.get(mat.candidate_id)?;

        return Self::update_active_profile_name_global(name.clone(), cx);
    }

    fn update_active_profile_name_global(
        name: Option<String>,
        cx: &mut Context<Picker<SettingsProfileSelectorDelegate>>,
    ) -> Option<String> {
        if let Some(name) = name {
            cx.set_global(ActiveSettingsProfileName(name.clone()));
            return Some(name.clone());
        }

        if cx.has_global::<ActiveSettingsProfileName>() {
            cx.remove_global::<ActiveSettingsProfileName>();
        }

        None
    }
}

impl PickerDelegate for SettingsProfileSelectorDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _: &mut Window, _: &mut App) -> std::sync::Arc<str> {
        "Select a settings profile...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _: &mut Window,
        cx: &mut Context<Picker<SettingsProfileSelectorDelegate>>,
    ) {
        self.selected_index = ix;
        self.selected_profile = self.set_selected_profile(cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<SettingsProfileSelectorDelegate>>,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self
            .profile_names
            .iter()
            .enumerate()
            .map(|(id, name)| StringMatchCandidate::new(id, &display_name(name)))
            .collect::<Vec<_>>();

        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| StringMatch {
                        candidate_id: index,
                        string: candidate.string,
                        positions: Vec::new(),
                        score: 0.0,
                    })
                    .collect()
            } else {
                match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update_in(cx, |this, _, cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
                this.delegate.selected_profile = this.delegate.set_selected_profile(cx);
            })
            .ok();
        })
    }

    fn confirm(
        &mut self,
        _: bool,
        _: &mut Window,
        cx: &mut Context<Picker<SettingsProfileSelectorDelegate>>,
    ) {
        self.selection_completed = true;
        self.selector
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn dismissed(
        &mut self,
        _: &mut Window,
        cx: &mut Context<Picker<SettingsProfileSelectorDelegate>>,
    ) {
        SettingsProfileSelectorDelegate::update_active_profile_name_global(
            self.original_profile_name.clone(),
            cx,
        );
        self.selector.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];
        let name = &self.profile_names[mat.candidate_id];

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(
                    display_name(name),
                    mat.positions.clone(),
                )),
        )
    }
}

fn display_name(name: &Option<String>) -> String {
    name.clone().unwrap_or("Disabled".into())
}

// TODO: Test all states manually and with tests
// TODO: Subscribe to global properly
