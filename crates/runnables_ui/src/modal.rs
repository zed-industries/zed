use std::sync::Arc;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, rems, DismissEvent, EventEmitter, FocusableView, InteractiveElement, Model,
    ParentElement, Render, SharedString, Styled, Subscription, Task, View, ViewContext,
    VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use project::Inventory;
use runnable::Runnable;
use ui::{v_flex, HighlightedLabel, ListItem, ListItemSpacing, Selectable};
use util::ResultExt;
use workspace::{ModalView, Workspace};

use crate::schedule_runnable;

actions!(runnables, [Spawn, Rerun]);

/// A modal used to spawn new runnables.
pub(crate) struct RunnablesModalDelegate {
    inventory: Model<Inventory>,
    candidates: Vec<Arc<dyn Runnable>>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    placeholder_text: Arc<str>,
    workspace: WeakView<Workspace>,
}

impl RunnablesModalDelegate {
    fn new(inventory: Model<Inventory>, workspace: WeakView<Workspace>) -> Self {
        Self {
            inventory,
            workspace,
            candidates: Vec::new(),
            matches: Vec::new(),
            selected_index: 0,
            placeholder_text: Arc::from("Select runnable..."),
        }
    }
}

pub(crate) struct RunnablesModal {
    picker: View<Picker<RunnablesModalDelegate>>,
    _subscription: Subscription,
}

impl RunnablesModal {
    pub(crate) fn new(
        inventory: Model<Inventory>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let picker = cx.new_view(|cx| {
            Picker::uniform_list(RunnablesModalDelegate::new(inventory, workspace), cx)
        });
        let _subscription = cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        });
        Self {
            picker,
            _subscription,
        }
    }
}
impl Render for RunnablesModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        v_flex()
            .w(rems(34.))
            .child(self.picker.clone())
            .on_mouse_down_out(cx.listener(|modal, _, cx| {
                modal.picker.update(cx, |picker, cx| {
                    picker.cancel(&Default::default(), cx);
                })
            }))
    }
}

impl EventEmitter<DismissEvent> for RunnablesModal {}
impl FocusableView for RunnablesModal {
    fn focus_handle(&self, cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.picker.read(cx).focus_handle(cx)
    }
}
impl ModalView for RunnablesModal {}

impl PickerDelegate for RunnablesModalDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<picker::Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self) -> Arc<str> {
        self.placeholder_text.clone()
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<picker::Picker<Self>>,
    ) -> Task<()> {
        cx.spawn(move |picker, mut cx| async move {
            let Some(candidates) = picker
                .update(&mut cx, |picker, cx| {
                    picker.delegate.candidates = picker
                        .delegate
                        .inventory
                        .update(cx, |inventory, cx| inventory.list_runnables(None, cx));
                    picker
                        .delegate
                        .candidates
                        .sort_by(|a, b| a.name().cmp(&b.name()));

                    picker
                        .delegate
                        .candidates
                        .iter()
                        .enumerate()
                        .map(|(index, candidate)| StringMatchCandidate {
                            id: index,
                            char_bag: candidate.name().chars().collect(),
                            string: candidate.name().into(),
                        })
                        .collect::<Vec<_>>()
                })
                .ok()
            else {
                return;
            };
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                true,
                1000,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;
            picker
                .update(&mut cx, |picker, _| {
                    let delegate = &mut picker.delegate;
                    delegate.matches = matches;

                    if delegate.matches.is_empty() {
                        delegate.selected_index = 0;
                    } else {
                        delegate.selected_index =
                            delegate.selected_index.min(delegate.matches.len() - 1);
                    }
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<picker::Picker<Self>>) {
        let current_match_index = self.selected_index();
        let Some(current_match) = self.matches.get(current_match_index) else {
            return;
        };

        let ix = current_match.candidate_id;
        let runnable = &self.candidates[ix];
        self.workspace
            .update(cx, |workspace, cx| {
                schedule_runnable(workspace, runnable.as_ref(), cx);
            })
            .ok();
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<picker::Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ViewContext<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let hit = &self.matches[ix];
        //let runnable = self.candidates[target_index].metadata();
        let highlights: Vec<_> = hit.positions.iter().copied().collect();
        Some(
            ListItem::new(SharedString::from(format!("runnables-modal-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .start_slot(HighlightedLabel::new(hit.string.clone(), highlights)),
        )
    }
}
