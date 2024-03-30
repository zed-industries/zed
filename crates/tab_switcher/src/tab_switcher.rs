#[cfg(test)]
mod tab_switcher_tests;

use collections::HashMap;
use gpui::{
    impl_actions, rems, Action, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    Modifiers, ModifiersChangedEvent, ParentElement, Render, Styled, Task, View, ViewContext,
    VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use serde::Deserialize;
use std::sync::Arc;
use ui::{prelude::*, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::{
    item::ItemHandle,
    pane::{render_item_indicator, tab_details, Event as PaneEvent},
    ModalView, Pane, Workspace,
};

const PANEL_WIDTH_REMS: f32 = 28.;

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct Toggle {
    #[serde(default)]
    pub select_last: bool,
}

impl_actions!(tab_switcher, [Toggle]);

pub struct TabSwitcher {
    picker: View<Picker<TabSwitcherDelegate>>,
    init_modifiers: Option<Modifiers>,
}

impl ModalView for TabSwitcher {}

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(TabSwitcher::register).detach();
}

impl TabSwitcher {
    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(|workspace, action: &Toggle, cx| {
            let Some(tab_switcher) = workspace.active_modal::<Self>(cx) else {
                Self::open(action, workspace, cx);
                return;
            };

            tab_switcher.update(cx, |tab_switcher, cx| {
                tab_switcher
                    .picker
                    .update(cx, |picker, cx| picker.cycle_selection(cx))
            });
        });
    }

    fn open(action: &Toggle, workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        let terminal = workspace.panel::<terminal_view::terminal_panel::TerminalPanel>(cx);
        let terminal_pane = terminal.and_then(|terminal| {
            terminal
                .focus_handle(cx)
                .contains_focused(cx)
                .then(|| terminal.read(cx).pane())
        });
        let weak_pane = terminal_pane
            .unwrap_or_else(|| workspace.active_pane())
            .downgrade();
        workspace.toggle_modal(cx, |cx| {
            let delegate = TabSwitcherDelegate::new(action, cx.view().downgrade(), weak_pane, cx);
            TabSwitcher::new(delegate, cx)
        });
    }

    fn new(delegate: TabSwitcherDelegate, cx: &mut ViewContext<Self>) -> Self {
        Self {
            picker: cx.new_view(|cx| Picker::nonsearchable_uniform_list(delegate, cx)),
            init_modifiers: cx.modifiers().modified().then_some(cx.modifiers()),
        }
    }

    fn handle_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(init_modifiers) = self.init_modifiers else {
            return;
        };
        if !event.modified() || !init_modifiers.is_subset_of(event) {
            self.init_modifiers = None;
            if self.picker.read(cx).delegate.matches.is_empty() {
                cx.emit(DismissEvent)
            } else {
                cx.dispatch_action(menu::Confirm.boxed_clone());
            }
        }
    }
}

impl EventEmitter<DismissEvent> for TabSwitcher {}

impl FocusableView for TabSwitcher {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for TabSwitcher {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("TabSwitcher")
            .w(rems(PANEL_WIDTH_REMS))
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .child(self.picker.clone())
    }
}

struct TabMatch {
    item_index: usize,
    item: Box<dyn ItemHandle>,
    detail: usize,
}

pub struct TabSwitcherDelegate {
    select_last: bool,
    tab_switcher: WeakView<TabSwitcher>,
    selected_index: usize,
    pane: WeakView<Pane>,
    matches: Vec<TabMatch>,
}

impl TabSwitcherDelegate {
    fn new(
        action: &Toggle,
        tab_switcher: WeakView<TabSwitcher>,
        pane: WeakView<Pane>,
        cx: &mut ViewContext<TabSwitcher>,
    ) -> Self {
        Self::subscribe_to_updates(&pane, cx);
        Self {
            select_last: action.select_last,
            tab_switcher,
            selected_index: 0,
            pane,
            matches: Vec::new(),
        }
    }

    fn subscribe_to_updates(pane: &WeakView<Pane>, cx: &mut ViewContext<TabSwitcher>) {
        let Some(pane) = pane.upgrade() else {
            return;
        };
        cx.subscribe(&pane, |tab_switcher, _, event, cx| {
            match event {
                PaneEvent::AddItem { .. } | PaneEvent::RemoveItem { .. } | PaneEvent::Remove => {
                    tab_switcher
                        .picker
                        .update(cx, |picker, cx| picker.refresh(cx))
                }
                _ => {}
            };
        })
        .detach();
    }

    fn update_matches(&mut self, cx: &mut WindowContext) {
        self.matches.clear();
        let Some(pane) = self.pane.upgrade() else {
            return;
        };

        let pane = pane.read(cx);
        let mut history_indices = HashMap::default();
        pane.activation_history().iter().rev().enumerate().for_each(
            |(history_index, entity_id)| {
                history_indices.insert(entity_id, history_index);
            },
        );

        let items: Vec<Box<dyn ItemHandle>> = pane.items().map(|item| item.boxed_clone()).collect();
        items
            .iter()
            .enumerate()
            .zip(tab_details(&items, cx))
            .map(|((item_index, item), detail)| TabMatch {
                item_index,
                item: item.boxed_clone(),
                detail,
            })
            .for_each(|tab_match| self.matches.push(tab_match));

        let non_history_base = history_indices.len();
        self.matches.sort_by(move |a, b| {
            let a_score = *history_indices
                .get(&a.item.item_id())
                .unwrap_or(&(a.item_index + non_history_base));
            let b_score = *history_indices
                .get(&b.item.item_id())
                .unwrap_or(&(b.item_index + non_history_base));
            a_score.cmp(&b_score)
        });

        if self.matches.len() > 1 {
            if self.select_last {
                self.selected_index = self.matches.len() - 1;
            } else {
                self.selected_index = 1;
            }
        }
    }
}

impl PickerDelegate for TabSwitcherDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        Vec::new()
    }

    fn update_matches(
        &mut self,
        _raw_query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Task<()> {
        self.update_matches(cx);
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<TabSwitcherDelegate>>) {
        let Some(pane) = self.pane.upgrade() else {
            return;
        };
        let Some(selected_match) = self.matches.get(self.selected_index()) else {
            return;
        };
        pane.update(cx, |pane, cx| {
            pane.activate_item(selected_match.item_index, true, true, cx);
        });
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<TabSwitcherDelegate>>) {
        self.tab_switcher
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let tab_match = self
            .matches
            .get(ix)
            .expect("Invalid matches state: no element for index {ix}");

        let label = tab_match.item.tab_content(Some(tab_match.detail), true, cx);
        let indicator = render_item_indicator(tab_match.item.boxed_clone(), cx);

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .inset(true)
                .selected(selected)
                .child(h_flex().w_full().child(label))
                .children(indicator),
        )
    }
}
