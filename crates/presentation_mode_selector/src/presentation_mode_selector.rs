use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, Render, Task, WeakEntity, Window,
};
use picker::{Picker, PickerDelegate};
use settings::Settings;
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use workspace::{ModalView, Workspace};
mod presentation_mode_settings;
use presentation_mode_settings::{PresentationMode, PresentationModeSettings};

pub fn init(cx: &mut App) {
    PresentationModeSettings::register(cx);

    cx.on_action(|_: &zed_actions::presentation_mode_selector::Toggle, cx| {
        workspace::with_active_or_new_workspace(cx, |workspace, window, cx| {
            toggle_presentation_mode_selector(workspace, window, cx);
        });
    });
}

fn toggle_presentation_mode_selector(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    workspace.toggle_modal(window, cx, |window, cx| {
        let settings = PresentationModeSettings::get_global(cx);
        let delegate = PresentationModeSelectorDelegate::new(
            cx.entity().downgrade(),
            settings.presentation_modes.clone(),
            cx,
        );
        PresentationModeSelector::new(delegate, window, cx)
    });
}

pub struct PresentationModeSelector {
    picker: Entity<Picker<PresentationModeSelectorDelegate>>,
}

impl ModalView for PresentationModeSelector {}

impl EventEmitter<DismissEvent> for PresentationModeSelector {}

impl Focusable for PresentationModeSelector {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for PresentationModeSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl PresentationModeSelector {
    pub fn new(
        delegate: PresentationModeSelectorDelegate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

pub struct PresentationModeSelectorDelegate {
    configurations: Vec<Option<PresentationMode>>,
    matches: Vec<StringMatch>,
    selected_mode: Option<PresentationMode>,
    selected_index: usize,
    selection_completed: bool,
    selector: WeakEntity<PresentationModeSelector>,
}

impl PresentationModeSelectorDelegate {
    fn new(
        selector: WeakEntity<PresentationModeSelector>,
        mut configurations: Vec<PresentationMode>,
        _cx: &mut Context<PresentationModeSelector>,
    ) -> Self {
        configurations.sort_by_key(|c| c.name.clone());
        let mut configurations: Vec<_> = configurations.into_iter().map(Some).collect();
        configurations.insert(0, None);

        let matches = configurations
            .iter()
            .enumerate()
            .map(|(ix, mode)| StringMatch {
                candidate_id: ix,
                score: 0.0,
                positions: Default::default(),
                string: PresentationMode::display_name(mode),
            })
            .collect();

        Self {
            configurations,
            matches,
            selected_mode: None,
            selected_index: 0,
            selection_completed: false,
            selector,
        }
    }

    fn apply_presentation_mode(
        &mut self,
        _cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
    ) -> Option<PresentationMode> {
        let Some(mat) = self.matches.get(self.selected_index) else {
            return None;
        };

        let Some(Some(configuration)) = self.configurations.get(mat.candidate_id) else {
            return None;
        };

        // TODO: Actually apply the presentation mode settings
        // This would involve updating the settings store with the mode's configuration

        Some(configuration.clone())
    }
}

impl PickerDelegate for PresentationModeSelectorDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> std::sync::Arc<str> {
        "Select a presentation mode...".into()
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
        _window: &mut Window,
        cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
    ) {
        self.selected_index = ix;
        self.selected_mode = self.apply_presentation_mode(cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self
            .configurations
            .iter()
            .enumerate()
            .map(|(id, mode)| StringMatchCandidate::new(id, &PresentationMode::display_name(mode)))
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

            this.update(cx, |this, cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
                this.delegate.selected_mode = this.delegate.apply_presentation_mode(cx);
            })
            .ok();
        })
    }

    fn confirm(
        &mut self,
        _: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
    ) {
        self.selection_completed = true;

        let Some(mode) = &self.selected_mode else {
            return;
        };

        let Some(buffer_font_family) = &mode.settings.buffer_font_family else {
            return;
        };

        // TODO: Apply the settings (emit)
        dbg!(&buffer_font_family);

        self.selector
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn dismissed(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Picker<PresentationModeSelectorDelegate>>,
    ) {
        if !self.selection_completed {
            // TODO: Restore last setting if preview was applied
        }

        self.selector.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];
        let mode = &self.configurations[mat.candidate_id];

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(
                    PresentationMode::display_name(mode),
                    mat.positions.clone(),
                )),
        )
    }
}
