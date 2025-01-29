use dap::client::DebugAdapterClientId;
use dap::session::DebugSessionId;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::Subscription;
use gpui::{DismissEvent, Entity, EventEmitter, Focusable, Render};
use picker::{Picker, PickerDelegate};
use project::dap_store::DapStore;
use std::sync::Arc;
use sysinfo::System;
use ui::{prelude::*, Context};
use ui::{ListItem, ListItemSpacing};
use workspace::ModalView;

#[derive(Debug, Clone)]
struct Candidate {
    pid: u32,
    name: String,
    command: String,
}

pub(crate) struct AttachModalDelegate {
    selected_index: usize,
    matches: Vec<StringMatch>,
    session_id: DebugSessionId,
    placeholder_text: Arc<str>,
    dap_store: Entity<DapStore>,
    client_id: DebugAdapterClientId,
    candidates: Option<Vec<Candidate>>,
}

impl AttachModalDelegate {
    pub fn new(
        session_id: DebugSessionId,
        client_id: DebugAdapterClientId,
        dap_store: Entity<DapStore>,
    ) -> Self {
        Self {
            client_id,
            dap_store,
            session_id,
            candidates: None,
            selected_index: 0,
            matches: Vec::default(),
            placeholder_text: Arc::from("Select the process you want to attach the debugger to"),
        }
    }
}

pub(crate) struct AttachModal {
    _subscription: Subscription,
    pub(crate) picker: Entity<Picker<AttachModalDelegate>>,
}

impl AttachModal {
    pub fn new(
        session_id: &DebugSessionId,
        client_id: &DebugAdapterClientId,
        dap_store: Entity<DapStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| {
            Picker::uniform_list(
                AttachModalDelegate::new(*session_id, *client_id, dap_store),
                window,
                cx,
            )
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
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl ui::IntoElement {
        v_flex()
            .key_context("AttachModal")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

impl EventEmitter<DismissEvent> for AttachModal {}

impl Focusable for AttachModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
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

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> std::sync::Arc<str> {
        self.placeholder_text.clone()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        cx.spawn(|this, mut cx| async move {
            let Some(processes) = this
                .update(&mut cx, |this, cx| {
                    if let Some(processes) = this.delegate.candidates.clone() {
                        processes
                    } else {
                        let Some((_, client)) = this.delegate.dap_store.update(cx, |store, cx| {
                            store.client_by_id(&this.delegate.client_id, cx)
                        }) else {
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

    fn confirm(&mut self, _: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
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
                .attach(&self.session_id, &self.client_id, candidate.pid, cx)
                .detach_and_log_err(cx);
        });

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = 0;
        self.candidates.take();

        self.dap_store.update(cx, |store, cx| {
            store.shutdown_session(&self.session_id, cx).detach();
        });

        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
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

#[allow(dead_code)]
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn procss_names(modal: &AttachModal, cx: &mut Context<AttachModal>) -> Vec<String> {
    modal.picker.update(cx, |picker, _| {
        picker
            .delegate
            .matches
            .iter()
            .map(|hit| hit.string.clone())
            .collect::<Vec<_>>()
    })
}
