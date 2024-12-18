use dap::client::DebugAdapterClientId;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{DismissEvent, EventEmitter, FocusableView, Render, View};
use gpui::{Model, Subscription};
use picker::{Picker, PickerDelegate};
use project::dap_store::DapStore;
use std::sync::Arc;
use sysinfo::System;
use ui::{prelude::*, ViewContext};
use ui::{ListItem, ListItemSpacing};
use workspace::ModalView;

#[derive(Debug, Clone)]
struct Candidate {
    pid: u32,
    name: String,
    command: String,
}

struct AttachModalDelegate {
    selected_index: usize,
    matches: Vec<StringMatch>,
    placeholder_text: Arc<str>,
    dap_store: Model<DapStore>,
    client_id: DebugAdapterClientId,
    candidates: Option<Vec<Candidate>>,
}

impl AttachModalDelegate {
    pub fn new(client_id: DebugAdapterClientId, dap_store: Model<DapStore>) -> Self {
        Self {
            client_id,
            dap_store,
            candidates: None,
            selected_index: 0,
            matches: Vec::default(),
            placeholder_text: Arc::from("Select the process you want to attach the debugger to"),
        }
    }
}

pub(crate) struct AttachModal {
    _subscription: Subscription,
    picker: View<Picker<AttachModalDelegate>>,
}

impl AttachModal {
    pub fn new(
        client_id: &DebugAdapterClientId,
        dap_store: Model<DapStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let picker = cx.new_view(|cx| {
            Picker::uniform_list(AttachModalDelegate::new(*client_id, dap_store), cx)
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

impl Render for AttachModal {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl ui::IntoElement {
        v_flex()
            .key_context("AttachModal")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

impl EventEmitter<DismissEvent> for AttachModal {}

impl FocusableView for AttachModal {
    fn focus_handle(&self, cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.picker.read(cx).focus_handle(cx)
    }
}

impl ModalView for AttachModal {}

impl PickerDelegate for AttachModalDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _cx: &mut ui::WindowContext) -> std::sync::Arc<str> {
        self.placeholder_text.clone()
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        cx.spawn(|this, mut cx| async move {
            let Some(processes) = this
                .update(&mut cx, |this, cx| {
                    if let Some(processes) = this.delegate.candidates.clone() {
                        processes
                    } else {
                        let Some(client) = this
                            .delegate
                            .dap_store
                            .read(cx)
                            .client_by_id(&this.delegate.client_id)
                        else {
                            return Vec::new();
                        };

                        let system = System::new_all();
                        let Some(processes) =
                            client.adapter().attach_processes(&system.processes())
                        else {
                            return Vec::new();
                        };

                        let processes = processes
                            .into_iter()
                            .map(|(pid, process)| Candidate {
                                pid: pid.as_u32(),
                                name: process.name().to_string_lossy().into_owned(),
                                command: process
                                    .cmd()
                                    .iter()
                                    .map(|s| s.to_string_lossy())
                                    .collect::<Vec<_>>()
                                    .join(" "),
                            })
                            .collect::<Vec<_>>();

                        let _ = this.delegate.candidates.insert(processes.clone());

                        processes
                    }
                })
                .ok()
            else {
                return;
            };

            let matches = fuzzy::match_strings(
                &processes
                    .iter()
                    .enumerate()
                    .map(|(id, candidate)| {
                        StringMatchCandidate::new(
                            id,
                            format!("{} {} {}", candidate.command, candidate.pid, candidate.name)
                                .as_str(),
                        )
                    })
                    .collect::<Vec<_>>(),
                &query,
                true,
                100,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            this.update(&mut cx, |this, _| {
                let delegate = &mut this.delegate;

                delegate.matches = matches;
                delegate.candidates = Some(processes);

                if delegate.matches.is_empty() {
                    delegate.selected_index = 0;
                } else {
                    delegate.selected_index =
                        delegate.selected_index.min(delegate.matches.len() - 1);
                }
            })
            .ok();
        })
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        let candidate = self
            .matches
            .get(self.selected_index())
            .and_then(|current_match| {
                let ix = current_match.candidate_id;
                self.candidates.as_ref().map(|candidates| &candidates[ix])
            });
        let Some(candidate) = candidate else {
            return cx.emit(DismissEvent);
        };

        self.dap_store.update(cx, |store, cx| {
            store
                .attach(&self.client_id, candidate.pid, cx)
                .detach_and_log_err(cx);
        });

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = 0;
        self.candidates.take();

        self.dap_store.update(cx, |store, cx| {
            store.shutdown_client(&self.client_id, cx).detach();
        });

        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let candidates = self.candidates.as_ref()?;
        let hit = &self.matches[ix];
        let candidate = &candidates.get(hit.candidate_id)?;

        Some(
            ListItem::new(SharedString::from(format!("attach-modal-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .items_start()
                        .child(Label::new(candidate.command.clone()))
                        .child(
                            Label::new(format!("Pid: {}, name: {}", candidate.pid, candidate.name))
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                ),
        )
    }
}
