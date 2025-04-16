use super::base_keymap_setting::BaseKeymap;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, Render, Task, WeakEntity, Window,
    actions,
};
use picker::{Picker, PickerDelegate};
use project::Fs;
use settings::{Settings, update_settings_file};
use std::sync::Arc;
use ui::{ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace, ui::HighlightedLabel};

actions!(welcome, [ToggleBaseKeymapSelector]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(toggle);
    })
    .detach();
}

pub fn toggle(
    workspace: &mut Workspace,
    _: &ToggleBaseKeymapSelector,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let fs = workspace.app_state().fs.clone();
    workspace.toggle_modal(window, cx, |window, cx| {
        BaseKeymapSelector::new(
            BaseKeymapSelectorDelegate::new(cx.entity().downgrade(), fs, cx),
            window,
            cx,
        )
    });
}

pub struct BaseKeymapSelector {
    picker: Entity<Picker<BaseKeymapSelectorDelegate>>,
}

impl Focusable for BaseKeymapSelector {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for BaseKeymapSelector {}
impl ModalView for BaseKeymapSelector {}

impl BaseKeymapSelector {
    pub fn new(
        delegate: BaseKeymapSelectorDelegate,
        window: &mut Window,
        cx: &mut Context<BaseKeymapSelector>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl Render for BaseKeymapSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

pub struct BaseKeymapSelectorDelegate {
    selector: WeakEntity<BaseKeymapSelector>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    fs: Arc<dyn Fs>,
}

impl BaseKeymapSelectorDelegate {
    fn new(
        selector: WeakEntity<BaseKeymapSelector>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<BaseKeymapSelector>,
    ) -> Self {
        let base = BaseKeymap::get(None, cx);
        let selected_index = BaseKeymap::OPTIONS
            .iter()
            .position(|(_, value)| value == base)
            .unwrap_or(0);
        Self {
            selector,
            matches: Vec::new(),
            selected_index,
            fs,
        }
    }
}

impl PickerDelegate for BaseKeymapSelectorDelegate {
    type ListItem = ui::ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a base keymap...".into()
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
        _: &mut Context<Picker<BaseKeymapSelectorDelegate>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<BaseKeymapSelectorDelegate>>,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let candidates = BaseKeymap::names()
            .enumerate()
            .map(|(id, name)| StringMatchCandidate::new(id, name))
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
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(cx, |this, _| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
            })
            .log_err();
        })
    }

    fn confirm(
        &mut self,
        _: bool,
        _: &mut Window,
        cx: &mut Context<Picker<BaseKeymapSelectorDelegate>>,
    ) {
        if let Some(selection) = self.matches.get(self.selected_index) {
            let base_keymap = BaseKeymap::from_names(&selection.string);

            telemetry::event!(
                "Settings Changed",
                setting = "keymap",
                value = base_keymap.to_string()
            );

            update_settings_file::<BaseKeymap>(self.fs.clone(), cx, move |setting, _| {
                *setting = Some(base_keymap)
            });
        }

        self.selector
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<BaseKeymapSelectorDelegate>>) {
        self.selector
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let keymap_match = &self.matches[ix];

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(
                    keymap_match.string.clone(),
                    keymap_match.positions.clone(),
                )),
        )
    }
}
