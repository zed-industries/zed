use std::cmp;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions,
    elements::{ChildView, Label},
    Action, Element, Entity, MutableAppContext, View, ViewContext, ViewHandle,
};
use selector::{SelectorModal, SelectorModalDelegate};
use settings::Settings;
use workspace::Workspace;

mod selector;

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(CommandPalette::toggle);
    selector::init::<CommandPalette>(cx);
}

actions!(command_palette, [Toggle]);

pub struct CommandPalette {
    selector: ViewHandle<SelectorModal<Self>>,
    actions: Vec<(&'static str, Box<dyn Action>)>,
    matches: Vec<StringMatch>,
    selected_ix: usize,
    focused_view_id: usize,
}

pub enum Event {
    Dismissed,
}

impl CommandPalette {
    pub fn new(
        focused_view_id: usize,
        actions: Vec<(&'static str, Box<dyn Action>)>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let this = cx.weak_handle();
        let selector = cx.add_view(|cx| SelectorModal::new(this, cx));
        Self {
            selector,
            actions,
            matches: vec![],
            selected_ix: 0,
            focused_view_id,
        }
    }

    fn toggle(_: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        let workspace = cx.handle();
        let window_id = cx.window_id();
        let focused_view_id = cx.focused_view_id(window_id).unwrap_or(workspace.id());

        cx.as_mut().defer(move |cx| {
            let actions = cx.available_actions(window_id, focused_view_id);
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(cx, |cx, _| {
                    let selector = cx.add_view(|cx| Self::new(focused_view_id, actions, cx));
                    cx.subscribe(&selector, Self::on_event).detach();
                    selector
                });
            });
        });
    }

    fn on_event(
        workspace: &mut Workspace,
        _: ViewHandle<Self>,
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

impl Entity for CommandPalette {
    type Event = Event;
}

impl View for CommandPalette {
    fn ui_name() -> &'static str {
        "CommandPalette"
    }

    fn render(&mut self, _: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        ChildView::new(self.selector.clone()).boxed()
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.selector);
    }
}

impl SelectorModalDelegate for CommandPalette {
    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn set_selected_index(&mut self, ix: usize) {
        self.selected_ix = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut gpui::ViewContext<Self>,
    ) -> gpui::Task<()> {
        let candidates = self
            .actions
            .iter()
            .enumerate()
            .map(|(ix, (name, _))| StringMatchCandidate {
                id: ix,
                string: name.to_string(),
                char_bag: name.chars().collect(),
            })
            .collect::<Vec<_>>();
        cx.spawn(move |this, mut cx| async move {
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                true,
                10000,
                &Default::default(),
                cx.background(),
            )
            .await;
            this.update(&mut cx, |this, _| {
                this.matches = matches;
                if this.matches.is_empty() {
                    this.selected_ix = 0;
                } else {
                    this.selected_ix = cmp::min(this.selected_ix, this.matches.len() - 1);
                }
            });
        })
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed);
    }

    fn confirm(&mut self, cx: &mut ViewContext<Self>) {
        if !self.matches.is_empty() {
            let window_id = cx.window_id();
            let action_ix = self.matches[self.selected_ix].candidate_id;
            cx.dispatch_action_at(
                window_id,
                self.focused_view_id,
                self.actions[action_ix].1.as_ref(),
            )
        }
        cx.emit(Event::Dismissed);
    }

    fn render_match(&self, ix: usize, selected: bool, cx: &gpui::AppContext) -> gpui::ElementBox {
        let settings = cx.global::<Settings>();
        let theme = &settings.theme.selector;
        let style = if selected {
            &theme.active_item
        } else {
            &theme.item
        };
        Label::new(self.matches[ix].string.clone(), style.label.clone())
            .contained()
            .with_style(style.container)
            .boxed()
    }
}
