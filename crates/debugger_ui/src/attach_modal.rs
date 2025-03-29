use dap::DebugRequestType;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::Subscription;
use gpui::{DismissEvent, Entity, EventEmitter, Focusable, Render};
use picker::{Picker, PickerDelegate};

use std::cell::LazyCell;
use std::sync::Arc;
use sysinfo::System;
use ui::{prelude::*, Context, Tooltip};
use ui::{ListItem, ListItemSpacing};
use util::debug_panic;
use workspace::ModalView;

#[derive(Debug, Clone)]
pub(super) struct Candidate {
    pub(super) pid: u32,
    pub(super) name: SharedString,
    pub(super) command: Vec<String>,
}

pub(crate) struct AttachModalDelegate {
    selected_index: usize,
    matches: Vec<StringMatch>,
    placeholder_text: Arc<str>,
    project: Entity<project::Project>,
    debug_config: task::DebugTaskDefinition,
    candidates: Arc<[Candidate]>,
}

impl AttachModalDelegate {
    fn new(
        project: Entity<project::Project>,
        debug_config: task::DebugTaskDefinition,
        candidates: Arc<[Candidate]>,
    ) -> Self {
        Self {
            project,
            debug_config,
            candidates,
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
    pub fn new(
        project: Entity<project::Project>,
        debug_config: task::DebugTaskDefinition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut processes: Vec<_> = System::new_all()
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
                        .map(|s| s.to_string_lossy().to_string())
                        .collect::<Vec<_>>(),
                }
            })
            .collect();
        processes.sort_by_key(|k| k.name.clone());
        Self::with_processes(project, debug_config, processes, window, cx)
    }

    pub(super) fn with_processes(
        project: Entity<project::Project>,
        debug_config: task::DebugTaskDefinition,
        processes: Vec<Candidate>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let adapter = project
            .read(cx)
            .debug_adapters()
            .adapter(&debug_config.adapter);
        let filter = LazyCell::new(|| adapter.map(|adapter| adapter.attach_processes_filter()));
        let processes = processes
            .into_iter()
            .filter(|process| {
                filter
                    .as_ref()
                    .map_or(false, |filter| filter.is_match(&process.name))
            })
            .collect();
        let picker = cx.new(|cx| {
            Picker::uniform_list(
                AttachModalDelegate::new(project, debug_config, processes),
                window,
                cx,
            )
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
        cx.spawn(async move |this, cx| {
            let Some(processes) = this
                .update(cx, |this, _| this.delegate.candidates.clone())
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

    fn confirm(&mut self, _: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let candidate = self
            .matches
            .get(self.selected_index())
            .and_then(|current_match| {
                let ix = current_match.candidate_id;
                self.candidates.get(ix)
            });

        let Some(candidate) = candidate else {
            return cx.emit(DismissEvent);
        };

        match &mut self.debug_config.request {
            DebugRequestType::Attach(config) => {
                config.process_id = Some(candidate.pid);
            }
            DebugRequestType::Launch(_) => {
                debug_panic!("Debugger attach modal used on launch debug config");
                return;
            }
        }

        let config = self.debug_config.clone();
        self.project
            .update(cx, |project, cx| {
                #[cfg(any(test, feature = "test-support"))]
                let ret = project.fake_debug_session(config.request, None, false, cx);
                #[cfg(not(any(test, feature = "test-support")))]
                let ret = project.start_debug_session(config.into(), cx);
                ret
            })
            .detach_and_log_err(cx);

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = 0;

        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let hit = &self.matches[ix];
        let candidate = self.candidates.get(hit.candidate_id)?;

        Some(
            ListItem::new(SharedString::from(format!("process-entry-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .items_start()
                        .child(Label::new(format!("{} {}", candidate.name, candidate.pid)))
                        .child(
                            div()
                                .id(SharedString::from(format!("process-entry-{ix}-command")))
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

#[cfg(any(test, feature = "test-support"))]
pub(crate) fn _process_names(modal: &AttachModal, cx: &mut Context<AttachModal>) -> Vec<String> {
    modal.picker.update(cx, |picker, _| {
        picker
            .delegate
            .matches
            .iter()
            .map(|hit| hit.string.clone())
            .collect::<Vec<_>>()
    })
}
