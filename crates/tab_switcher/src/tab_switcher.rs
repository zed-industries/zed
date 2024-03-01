use gpui::{
    actions, rems, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    ParentElement, Render, Styled, Task, View, ViewContext, VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use std::sync::{atomic::AtomicBool, Arc};
use ui::{prelude::*, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::{
    item::ItemHandle,
    pane::{render_item_indicator, tab_details},
    ModalView, Pane, Workspace,
};

const PANEL_WIDTH_REMS: f32 = 28.;

actions!(tab_switcher, [Toggle]);

pub struct TabSwitcher {
    picker: View<Picker<TabSwitcherDelegate>>,
}

impl ModalView for TabSwitcher {}

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(TabSwitcher::register).detach();
}

impl TabSwitcher {
    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(|workspace, _: &Toggle, cx| {
            let Some(tab_switcher) = workspace.active_modal::<Self>(cx) else {
                Self::open(workspace, cx);
                return;
            };

            tab_switcher.update(cx, |tab_switcher, cx| {
                tab_switcher
                    .picker
                    .update(cx, |picker, cx| picker.cycle_selection(cx))
            });
        });
    }

    fn open(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        let weak_pane = workspace.active_pane().downgrade();
        workspace.toggle_modal(cx, |cx| {
            let delegate = TabSwitcherDelegate::new(cx.view().downgrade(), weak_pane, cx);
            TabSwitcher::new(delegate, cx)
        });
    }

    fn new(delegate: TabSwitcherDelegate, cx: &mut ViewContext<Self>) -> Self {
        Self {
            picker: cx.new_view(|cx| Picker::uniform_list(delegate, cx)),
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
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(PANEL_WIDTH_REMS))
            .child(self.picker.clone())
    }
}

struct TabMatch {
    item_index: usize,
    item: Box<dyn ItemHandle>,
    detail: usize,
}

pub struct TabSwitcherDelegate {
    tab_switcher: WeakView<TabSwitcher>,
    selected_index: usize,
    _cancel_flag: Arc<AtomicBool>,
    pane: WeakView<Pane>,
    matches: Vec<TabMatch>,
}

impl TabSwitcherDelegate {
    fn new(
        tab_switcher: WeakView<TabSwitcher>,
        pane: WeakView<Pane>,
        cx: &mut ViewContext<TabSwitcher>,
    ) -> Self {
        // cx.observe(&project, |tab_switcher, _, cx| {
        //     tab_switcher
        //         .picker
        //         .update(cx, |picker, cx| picker.refresh(cx))
        // })
        // .detach();

        let mut this = Self {
            tab_switcher,
            selected_index: 0,
            _cancel_flag: Arc::new(AtomicBool::new(false)),
            pane,
            matches: Vec::new(),
        };
        this.update_matches(cx);
        this
    }

    fn update_matches(&mut self, cx: &mut WindowContext) {
        self.matches.clear();
        let Some(pane) = self.pane.upgrade() else {
            return;
        };
        let mut items = Vec::new();
        pane.update(cx, |pane, _| {
            pane.items()
                .map(|i| i.boxed_clone())
                .for_each(|i| items.push(i));
        });
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
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Task<()> {
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
