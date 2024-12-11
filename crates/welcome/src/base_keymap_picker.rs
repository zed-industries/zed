use super::base_keymap_setting::BaseKeymap;
use client::telemetry::Telemetry;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions, AppContext, AppContext, DismissEvent, EventEmitter, FocusableView, Render, Task, View,
    VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use project::Fs;
use settings::{update_settings_file, Settings};
use std::sync::Arc;
use ui::{prelude::*, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::{ui::HighlightedLabel, ModalView, Workspace};

actions!(welcome, [ToggleBaseKeymapSelector]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _cx| {
        workspace.register_action(toggle);
    })
    .detach();
}

pub fn toggle(
    workspace: &mut Workspace,
    _: &ToggleBaseKeymapSelector,
    model: &Model<Workspace>,
    cx: &mut AppContext,
) {
    let fs = workspace.app_state().fs.clone();
    let telemetry = workspace.client().telemetry().clone();
    workspace.toggle_modal(cx, |cx| {
        BaseKeymapSelector::new(
            BaseKeymapSelectorDelegate::new(model.downgrade(), fs, telemetry, cx),
            model,
            cx,
        )
    });
}

pub struct BaseKeymapSelector {
    picker: Model<Picker<BaseKeymapSelectorDelegate>>,
}

impl FocusableView for BaseKeymapSelector {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for BaseKeymapSelector {}
impl ModalView for BaseKeymapSelector {}

impl BaseKeymapSelector {
    pub fn new(
        delegate: BaseKeymapSelectorDelegate,
        model: &Model<BaseKeymapSelector>,
        cx: &mut AppContext,
    ) -> Self {
        let picker = cx.new_model(|model, cx| Picker::uniform_list(delegate, cx));
        Self { picker }
    }
}

impl Render for BaseKeymapSelector {
    fn render(&mut self, model: &Model<Self>, _cx: &mut AppContext) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

pub struct BaseKeymapSelectorDelegate {
    view: WeakModel<BaseKeymapSelector>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    telemetry: Arc<Telemetry>,
    fs: Arc<dyn Fs>,
}

impl BaseKeymapSelectorDelegate {
    fn new(
        weak_view: WeakModel<BaseKeymapSelector>,
        fs: Arc<dyn Fs>,
        telemetry: Arc<Telemetry>,
        model: &Model<BaseKeymapSelector>,
        cx: &mut AppContext,
    ) -> Self {
        let base = BaseKeymap::get(None, cx);
        let selected_index = BaseKeymap::OPTIONS
            .iter()
            .position(|(_, value)| value == base)
            .unwrap_or(0);
        Self {
            view: weak_view,
            matches: Vec::new(),
            selected_index,
            telemetry,
            fs,
        }
    }
}

impl PickerDelegate for BaseKeymapSelectorDelegate {
    type ListItem = ui::ListItem;

    fn placeholder_text(&self, _window: &mut gpui::Window, _cx: &mut gpui::AppContext) -> Arc<str> {
        "Select a base keymap...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &Model<Picker>, _: &mut AppContext) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        model: &Model<Picker>,
        cx: &mut AppContext,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let candidates = BaseKeymap::names()
            .enumerate()
            .map(|(id, name)| StringMatchCandidate {
                id,
                char_bag: name.into(),
                string: name.into(),
            })
            .collect::<Vec<_>>();

        model.spawn(cx, |this, mut cx| async move {
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

            this.update(&mut cx, |this, _, _| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _: bool, model: &Model<Picker>, cx: &mut AppContext) {
        if let Some(selection) = self.matches.get(self.selected_index) {
            let base_keymap = BaseKeymap::from_names(&selection.string);

            self.telemetry
                .report_setting_event("keymap", base_keymap.to_string());

            update_settings_file::<BaseKeymap>(self.fs.clone(), cx, move |setting, _| {
                *setting = Some(base_keymap)
            });
        }

        self.view
            .update(cx, |_, model, cx| {
                model.emit(DismissEvent, cx);
            })
            .ok();
    }

    fn dismissed(&mut self, model: &Model<Picker>, cx: &mut AppContext) {
        self.view
            .update(cx, |_, model, cx| {
                model.emit(DismissEvent, cx);
            })
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        model: &Model<Picker>,
        _cx: &mut AppContext,
    ) -> Option<Self::ListItem> {
        let keymap_match = &self.matches[ix];

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(HighlightedLabel::new(
                    keymap_match.string.clone(),
                    keymap_match.positions.clone(),
                )),
        )
    }
}
