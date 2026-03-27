use dap::{DapRegistry, DebugRequest};
use futures::channel::oneshot;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{AppContext, DismissEvent, Entity, EventEmitter, Focusable, Render, Task};
use gpui::{Subscription, WeakEntity};
use picker::{Picker, PickerDelegate};
use project::Project;
use rpc::proto;
use task::ZedDebugConfig;
use util::debug_panic;

use std::sync::Arc;

use sysinfo::{ProcessRefreshKind, RefreshKind, System, UpdateKind};
use ui::{Context, Tooltip, prelude::*};
use ui::{ListItem, ListItemSpacing};
use workspace::{ModalView, Workspace};

use crate::debugger_panel::DebugPanel;

#[derive(Debug, Clone)]
pub(super) struct Candidate {
    pub(super) pid: u32,
    pub(super) name: SharedString,
    pub(super) command: Vec<String>,
}

pub(crate) enum ModalIntent {
    ResolveProcessId(Option<oneshot::Sender<Option<i32>>>),
    AttachToProcess(ZedDebugConfig),
}

pub(crate) struct AttachModalDelegate {
    selected_index: usize,
    matches: Vec<StringMatch>,
    placeholder_text: Arc<str>,
    pub(crate) intent: ModalIntent,
    workspace: WeakEntity<Workspace>,
    candidates: Arc<[Candidate]>,
}

impl AttachModalDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        intent: ModalIntent,
        candidates: Arc<[Candidate]>,
    ) -> Self {
        Self {
            workspace,
            candidates,
            intent,
            selected_index: 0,
            matches: Vec::default(),
            placeholder_text: Arc::from("Select the process you want to attach the debugger to"),
        }
    }
}

pub struct AttachModal {
    _subscription: Subscription,
    pub(crate) picker: Entity<Picker<AttachModalDelegate>>,
}

impl AttachModal {
    pub(crate) fn new(
        intent: ModalIntent,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        modal: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let processes_task = get_processes_for_project(&project, cx);

        let modal = Self::with_processes(workspace, Arc::new([]), modal, intent, window, cx);

        cx.spawn_in(window, async move |this, cx| {
            let processes = processes_task.await;
            this.update_in(cx, |modal, window, cx| {
                modal.picker.update(cx, |picker, cx| {
                    picker.delegate.candidates = processes;
                    picker.refresh(window, cx);
                });
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        modal
    }

    pub(super) fn with_processes(
        workspace: WeakEntity<Workspace>,
        processes: Arc<[Candidate]>,
        modal: bool,
        intent: ModalIntent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| {
            Picker::uniform_list(
                AttachModalDelegate::new(workspace, intent, processes),
                window,
                cx,
            )
            .modal(modal)
        });
        Self {
            _subscription: cx.subscribe(&picker, |_, _, _, cx| {
                cx.emit(DismissEvent);
            }),
            picker,
        }
    }
}

impl Render for AttachModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        v_flex()
            .key_context("AttachModal")
            .track_focus(&self.focus_handle(cx))
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
        cx.spawn(async move |this, cx| {
            let Some(processes) = this
                .read_with(cx, |this, _| this.delegate.candidates.clone())
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
                            format!(
                                "{} {} {}",
                                candidate.command.join(" "),
                                candidate.pid,
                                candidate.name
                            )
                            .as_str(),
                        )
                    })
                    .collect::<Vec<_>>(),
                &query,
                true,
                true,
                100,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            this.update(cx, |this, _| {
                let delegate = &mut this.delegate;

                delegate.matches = matches;

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

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let candidate = self
            .matches
            .get(self.selected_index())
            .and_then(|current_match| {
                let ix = current_match.candidate_id;
                self.candidates.get(ix)
            });

        match &mut self.intent {
            ModalIntent::ResolveProcessId(sender) => {
                cx.emit(DismissEvent);

                if let Some(sender) = sender.take() {
                    sender
                        .send(candidate.map(|candidate| candidate.pid as i32))
                        .ok();
                }
            }
            ModalIntent::AttachToProcess(definition) => {
                let Some(candidate) = candidate else {
                    return cx.emit(DismissEvent);
                };

                match &mut definition.request {
                    DebugRequest::Attach(config) => {
                        config.process_id = Some(candidate.pid);
                    }
                    DebugRequest::Launch(_) => {
                        debug_panic!("Debugger attach modal used on launch debug config");
                        return;
                    }
                }

                let workspace = self.workspace.clone();
                let Some(panel) = workspace
                    .update(cx, |workspace, cx| workspace.panel::<DebugPanel>(cx))
                    .ok()
                    .flatten()
                else {
                    return;
                };

                let Some(adapter) = cx.read_global::<DapRegistry, _>(|registry, _| {
                    registry.adapter(&definition.adapter)
                }) else {
                    return;
                };

                let definition = definition.clone();
                cx.spawn_in(window, async move |this, cx| {
                    let Ok(scenario) = adapter.config_from_zed_format(definition).await else {
                        return;
                    };

                    panel
                        .update_in(cx, |panel, window, cx| {
                            panel.start_session(
                                scenario,
                                Default::default(),
                                None,
                                None,
                                window,
                                cx,
                            );
                        })
                        .ok();
                    this.update(cx, |_, cx| {
                        cx.emit(DismissEvent);
                    })
                    .ok();
                })
                .detach();
            }
        }
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = 0;

        match &mut self.intent {
            ModalIntent::ResolveProcessId(sender) => {
                if let Some(sender) = sender.take() {
                    sender.send(None).ok();
                }
            }
            ModalIntent::AttachToProcess(_) => {}
        }

        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let hit = &self.matches.get(ix)?;
        let candidate = self.candidates.get(hit.candidate_id)?;

        Some(
            ListItem::new(format!("process-entry-{ix}"))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .items_start()
                        .child(Label::new(format!("{} {}", candidate.name, candidate.pid)))
                        .child(
                            div()
                                .id(format!("process-entry-{ix}-command"))
                                .tooltip(Tooltip::text(
                                    candidate
                                        .command
                                        .clone()
                                        .into_iter()
                                        .collect::<Vec<_>>()
                                        .join(" "),
                                ))
                                .child(
                                    Label::new(format!(
                                        "{} {}",
                                        candidate.name,
                                        candidate
                                            .command
                                            .clone()
                                            .into_iter()
                                            .skip(1)
                                            .collect::<Vec<_>>()
                                            .join(" ")
                                    ))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                                ),
                        ),
                ),
        )
    }
}

fn get_processes_for_project(project: &Entity<Project>, cx: &mut App) -> Task<Arc<[Candidate]>> {
    let project = project.read(cx);

    if let Some(remote_client) = project.remote_client() {
        let proto_client = remote_client.read(cx).proto_client();
        cx.background_spawn(async move {
            let response = proto_client
                .request(proto::GetProcesses {
                    project_id: proto::REMOTE_SERVER_PROJECT_ID,
                })
                .await
                .unwrap_or_else(|_| proto::GetProcessesResponse {
                    processes: Vec::new(),
                });

            let mut processes: Vec<Candidate> = response
                .processes
                .into_iter()
                .map(|p| Candidate {
                    pid: p.pid,
                    name: p.name.into(),
                    command: p.command,
                })
                .collect();

            processes.sort_by_key(|k| k.name.clone());
            Arc::from(processes.into_boxed_slice())
        })
    } else {
        let refresh_kind = RefreshKind::nothing().with_processes(
            ProcessRefreshKind::nothing()
                .without_tasks()
                .with_cmd(UpdateKind::Always),
        );
        let mut processes: Box<[_]> = System::new_with_specifics(refresh_kind)
            .processes()
            .values()
            .map(|process| {
                let name = process.name().to_string_lossy().into_owned();
                Candidate {
                    name: name.into(),
                    pid: process.pid().as_u32(),
                    command: process
                        .cmd()
                        .iter()
                        .map(|s| s.to_string_lossy().into_owned())
                        .collect::<Vec<_>>(),
                }
            })
            .collect();
        processes.sort_by_key(|k| k.name.clone());
        let processes = processes.into_iter().collect();
        Task::ready(processes)
    }
}

#[cfg(test)]
pub(crate) fn set_candidates(
    modal: &AttachModal,
    candidates: Arc<[Candidate]>,
    window: &mut Window,
    cx: &mut Context<AttachModal>,
) {
    modal.picker.update(cx, |picker, cx| {
        picker.delegate.candidates = candidates;
        picker.refresh(window, cx);
    });
}

#[cfg(test)]
pub(crate) fn process_names(modal: &AttachModal, cx: &mut Context<AttachModal>) -> Vec<String> {
    modal.picker.read_with(cx, |picker, _| {
        picker
            .delegate
            .matches
            .iter()
            .map(|hit| hit.string.clone())
            .collect::<Vec<_>>()
    })
}
