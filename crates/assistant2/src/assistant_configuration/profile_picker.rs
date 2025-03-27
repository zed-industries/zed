use std::sync::Arc;

use assistant_settings::AssistantSettings;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, SharedString, Task, WeakEntity,
    Window,
};
use picker::{Picker, PickerDelegate};
use settings::Settings;
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::ResultExt as _;

pub struct ProfilePicker {
    picker: Entity<Picker<ProfilePickerDelegate>>,
}

impl ProfilePicker {
    pub fn new(
        delegate: ProfilePickerDelegate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl EventEmitter<DismissEvent> for ProfilePicker {}

impl Focusable for ProfilePicker {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ProfilePicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

#[derive(Debug)]
pub struct ProfileEntry {
    pub id: Arc<str>,
    pub name: SharedString,
}

pub struct ProfilePickerDelegate {
    profile_picker: WeakEntity<ProfilePicker>,
    profiles: Vec<ProfileEntry>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    on_confirm: Arc<dyn Fn(&Arc<str>, &mut Window, &mut App) + 'static>,
}

impl ProfilePickerDelegate {
    pub fn new(
        on_confirm: impl Fn(&Arc<str>, &mut Window, &mut App) + 'static,
        cx: &mut Context<ProfilePicker>,
    ) -> Self {
        let settings = AssistantSettings::get_global(cx);

        let profiles = settings
            .profiles
            .iter()
            .map(|(id, profile)| ProfileEntry {
                id: id.clone(),
                name: profile.name.clone(),
            })
            .collect::<Vec<_>>();

        Self {
            profile_picker: cx.entity().downgrade(),
            profiles,
            matches: Vec::new(),
            selected_index: 0,
            on_confirm: Arc::new(on_confirm),
        }
    }
}

impl PickerDelegate for ProfilePickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search profiles…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self
            .profiles
            .iter()
            .enumerate()
            .map(|(id, profile)| StringMatchCandidate::new(id, profile.name.as_ref()))
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
                        score: 0.,
                    })
                    .collect()
            } else {
                match_strings(
                    &candidates,
                    &query,
                    false,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(cx, |this, _cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if self.matches.is_empty() {
            self.dismissed(window, cx);
            return;
        }

        let candidate_id = self.matches[self.selected_index].candidate_id;
        let profile = &self.profiles[candidate_id];

        (self.on_confirm)(&profile.id, window, cx);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.profile_picker
            .update(cx, |_this, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let profile_match = &self.matches[ix];

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(
                    profile_match.string.clone(),
                    profile_match.positions.clone(),
                )),
        )
    }
}
