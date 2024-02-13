use std::{path::PathBuf, sync::Arc};

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, rems, Action, DismissEvent, EventEmitter, FocusableView, InteractiveElement, Model,
    ParentElement, Render, SharedString, Styled, Subscription, Task, View, ViewContext,
    VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use project::Inventory;
use runnable::RunnableToken;
use ui::{v_flex, HighlightedLabel, ListItem, ListItemSpacing, Selectable};
use util::ResultExt;
use workspace::{ModalView, Workspace};

use crate::RunnablesPanel;

actions!(runnables, [New]);
/// A modal used to spawn new runnables.
pub(crate) struct RunnablesModalDelegate {
    inventory: Model<Inventory>,
    candidates: Vec<RunnableToken>,
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
            candidates: vec![],
            matches: vec![],
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
        let picker =
            cx.new_view(|cx| Picker::new(RunnablesModalDelegate::new(inventory, workspace), cx));
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
    fn render(
        &mut self,
        cx: &mut ui::prelude::ViewContext<Self>,
    ) -> impl gpui::prelude::IntoElement {
        v_flex()
            .w(rems(20.))
            .child(self.picker.clone())
            .on_mouse_down_out(cx.listener(|this, _, cx| {
                this.picker.update(cx, |this, cx| {
                    this.cancel(&Default::default(), cx);
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

    fn set_selected_index(
        &mut self,
        ix: usize,
        cx: &mut ui::prelude::ViewContext<picker::Picker<Self>>,
    ) {
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
                .update(&mut cx, |this, cx| {
                    let path = &PathBuf::new();
                    this.delegate.candidates = this
                        .delegate
                        .inventory
                        .update(cx, |this, cx| this.list_runnables(path, cx));
                    this.delegate
                        .candidates
                        .retain(|runnable| !runnable.was_scheduled(cx));

                    this.delegate
                        .candidates
                        .iter()
                        .enumerate()
                        .map(|(index, candidate)| StringMatchCandidate {
                            id: index,
                            char_bag: candidate.metadata().display_name().chars().collect(),
                            string: candidate.metadata().display_name().into(),
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
                            core::cmp::min(delegate.selected_index, delegate.matches.len() - 1);
                    }
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<picker::Picker<Self>>) {
        let current_match_index = self.selected_index();
        let ix = self.matches[current_match_index].candidate_id;
        if let Some(handle) = self.candidates[ix].schedule(cx).log_err() {
            if let Some(output) = handle.output.as_ref() {
                self.workspace
                    .update(cx, |_, cx| {
                        cx.dispatch_action(
                            workspace::OpenTerminalStream {
                                source: Some(output.subscribe()),
                            }
                            .boxed_clone(),
                        );
                    })
                    .log_err();
            }
            self.workspace
                .update(cx, |workspace, cx| {
                    let Some(panel) = workspace.panel::<RunnablesPanel>(cx) else {
                        return;
                    };
                    panel.update(cx, |this, cx| {
                        if let Some(tracker) = this.status_bar_tracker.as_ref() {
                            tracker.update(cx, |this, cx| this.push(handle, cx));
                            cx.notify();
                        }
                    });
                })
                .ok();
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<picker::Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ui::prelude::ViewContext<picker::Picker<Self>>,
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
