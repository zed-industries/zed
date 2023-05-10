use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions,
    elements::{Element as _, Label},
    AppContext, Task, ViewContext,
};
use picker::{Picker, PickerDelegate, PickerEvent};
use project::Fs;
use settings::{update_settings_file, BaseKeymap, Settings};
use std::sync::Arc;
use util::ResultExt;
use workspace::Workspace;

actions!(welcome, [ToggleBaseKeymapSelector]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(toggle);
    BaseKeymapSelector::init(cx);
}

pub fn toggle(
    workspace: &mut Workspace,
    _: &ToggleBaseKeymapSelector,
    cx: &mut ViewContext<Workspace>,
) {
    workspace.toggle_modal(cx, |workspace, cx| {
        let fs = workspace.app_state().fs.clone();
        cx.add_view(|cx| BaseKeymapSelector::new(BaseKeymapSelectorDelegate::new(fs, cx), cx))
    });
}

pub type BaseKeymapSelector = Picker<BaseKeymapSelectorDelegate>;

pub struct BaseKeymapSelectorDelegate {
    matches: Vec<StringMatch>,
    selected_index: usize,
    fs: Arc<dyn Fs>,
}

impl BaseKeymapSelectorDelegate {
    fn new(fs: Arc<dyn Fs>, cx: &mut ViewContext<BaseKeymapSelector>) -> Self {
        let base = cx.global::<Settings>().base_keymap;
        let selected_index = BaseKeymap::OPTIONS
            .iter()
            .position(|(_, value)| *value == base)
            .unwrap_or(0);
        Self {
            matches: Vec::new(),
            selected_index,
            fs,
        }
    }
}

impl PickerDelegate for BaseKeymapSelectorDelegate {
    fn placeholder_text(&self) -> Arc<str> {
        "Select a base keymap...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<BaseKeymapSelector>) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<BaseKeymapSelector>,
    ) -> Task<()> {
        let background = cx.background().clone();
        let candidates = BaseKeymap::names()
            .enumerate()
            .map(|(id, name)| StringMatchCandidate {
                id,
                char_bag: name.into(),
                string: name.into(),
            })
            .collect::<Vec<_>>();

        cx.spawn(|this, mut cx| async move {
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

            this.update(&mut cx, |this, _| {
                let delegate = this.delegate_mut();
                delegate.matches = matches;
                delegate.selected_index = delegate
                    .selected_index
                    .min(delegate.matches.len().saturating_sub(1));
            })
            .log_err();
        })
    }

    fn confirm(&mut self, cx: &mut ViewContext<BaseKeymapSelector>) {
        if let Some(selection) = self.matches.get(self.selected_index) {
            let base_keymap = BaseKeymap::from_names(&selection.string);
            update_settings_file::<Settings>(self.fs.clone(), cx, move |settings| {
                settings.base_keymap = Some(base_keymap)
            });
        }
        cx.emit(PickerEvent::Dismiss);
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<BaseKeymapSelector>) {}

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut gpui::MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> gpui::AnyElement<Picker<Self>> {
        let theme = &cx.global::<Settings>().theme;
        let keymap_match = &self.matches[ix];
        let style = theme.picker.item.style_for(mouse_state, selected);

        Label::new(keymap_match.string.clone(), style.label.clone())
            .with_highlights(keymap_match.positions.clone())
            .contained()
            .with_style(style.container)
            .into_any()
    }
}
