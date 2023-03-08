use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions,
    elements::{ChildView, Element as _, Label},
    AnyViewHandle, Entity, MutableAppContext, View, ViewContext, ViewHandle,
};
use picker::{Picker, PickerDelegate};
use settings::{settings_file::SettingsFile, BaseKeymap, Settings};
use workspace::Workspace;

pub struct BaseKeymapSelector {
    matches: Vec<StringMatch>,
    picker: ViewHandle<Picker<Self>>,
    selected_index: usize,
}

actions!(welcome, [ToggleBaseKeymapSelector]);

pub fn init(cx: &mut MutableAppContext) {
    Picker::<BaseKeymapSelector>::init(cx);
    cx.add_action({
        move |workspace, _: &ToggleBaseKeymapSelector, cx| BaseKeymapSelector::toggle(workspace, cx)
    });
}

pub enum Event {
    Dismissed,
}

impl BaseKeymapSelector {
    fn toggle(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        workspace.toggle_modal(cx, |_, cx| {
            let this = cx.add_view(|cx| Self::new(cx));
            cx.subscribe(&this, Self::on_event).detach();
            this
        });
    }

    fn new(cx: &mut ViewContext<Self>) -> Self {
        let base = cx.global::<Settings>().base_keymap;
        let selected_index = BaseKeymap::OPTIONS
            .iter()
            .position(|(_, value)| *value == base)
            .unwrap_or(0);

        let this = cx.weak_handle();
        Self {
            picker: cx.add_view(|cx| Picker::new("Select a base keymap", this, cx)),
            matches: Vec::new(),
            selected_index,
        }
    }

    fn on_event(
        workspace: &mut Workspace,
        _: ViewHandle<BaseKeymapSelector>,
        event: &Event,
        cx: &mut ViewContext<Workspace>,
    ) {
        match event {
            Event::Dismissed => {
                workspace.dismiss_modal(cx);
            }
        }
    }
}

impl Entity for BaseKeymapSelector {
    type Event = Event;
}

impl View for BaseKeymapSelector {
    fn ui_name() -> &'static str {
        "BaseKeymapSelector"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        ChildView::new(self.picker.clone(), cx).boxed()
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.picker);
        }
    }
}

impl PickerDelegate for BaseKeymapSelector {
    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Self>) {
        self.selected_index = ix;
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Self>) -> gpui::Task<()> {
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

            this.update(&mut cx, |this, cx| {
                this.matches = matches;
                this.selected_index = this
                    .selected_index
                    .min(this.matches.len().saturating_sub(1));
                cx.notify();
            });
        })
    }

    fn confirm(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.matches.get(self.selected_index) {
            let base_keymap = BaseKeymap::from_names(&selection.string);
            SettingsFile::update(cx, move |settings| settings.base_keymap = Some(base_keymap));
        }
        cx.emit(Event::Dismissed);
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed)
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut gpui::MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> gpui::ElementBox {
        let theme = &cx.global::<Settings>().theme;
        let keymap_match = &self.matches[ix];
        let style = theme.picker.item.style_for(mouse_state, selected);

        Label::new(keymap_match.string.clone(), style.label.clone())
            .with_highlights(keymap_match.positions.clone())
            .contained()
            .with_style(style.container)
            .boxed()
    }
}
