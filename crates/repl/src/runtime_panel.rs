use crate::{
    jupyter_settings::{JupyterDockPosition, JupyterSettings},
    kernels::{kernel_specifications, KernelSpecification},
    session::{Session, SessionEvent},
};
use anyhow::{anyhow, Context as _, Result};
use collections::HashMap;
use editor::{Anchor, Editor, RangeToAnchorExt};
use futures::{channel::mpsc::UnboundedSender, StreamExt as _};
use gpui::{
    actions, prelude::*, AppContext, AsyncWindowContext, EntityId, EventEmitter, FocusHandle,
    FocusOutEvent, FocusableView, Subscription, Task, View, WeakView,
};
use language::{Language, Point};
use multi_buffer::MultiBufferRow;
use project::{Fs, Item, ProjectEntryId};
use settings::{Settings as _, SettingsStore};
use std::{ops::Range, sync::Arc};
use ui::{prelude::*, ButtonLike, ElevationIndex, KeyBinding};
use util::ResultExt as _;
use workspace::{
    dock::{Panel, PanelEvent},
    Workspace,
};

actions!(repl, [Run, ClearOutputs, Interrupt, Shutdown]);
actions!(repl_panel, [ToggleFocus]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleFocus, cx| {
                workspace.toggle_panel_focus::<RuntimePanel>(cx);
            });
        },
    )
    .detach();
}

pub struct RuntimePanel {
    fs: Arc<dyn Fs>,
    enabled: bool,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
    sessions: HashMap<ProjectEntryId, View<Session>>,
    kernel_specifications: Vec<KernelSpecification>,
    _subscriptions: Vec<Subscription>,
    _editor_events_task: Task<()>,
}

pub struct ReplAction {
    editor: WeakView<Editor>,
    event: ReplEvent,
}

impl ReplAction {
    fn run(
        editor: WeakView<Editor>,
        tx: UnboundedSender<ReplAction>,
    ) -> impl Fn(&Run, &mut WindowContext) {
        move |_: &Run, cx: &mut WindowContext| {
            if !JupyterSettings::enabled(cx) {
                return;
            }
            tx.unbounded_send(ReplAction {
                editor: editor.clone(),
                event: ReplEvent::Run,
            })
            .ok();
        }
    }

    fn clear_outputs(
        editor: WeakView<Editor>,
        tx: UnboundedSender<ReplAction>,
    ) -> impl Fn(&ClearOutputs, &mut WindowContext) {
        move |_: &ClearOutputs, cx: &mut WindowContext| {
            if !JupyterSettings::enabled(cx) {
                return;
            }
            tx.unbounded_send(ReplAction {
                editor: editor.clone(),
                event: ReplEvent::ClearOutputs,
            })
            .ok();
        }
    }

    fn interrupt(
        editor: WeakView<Editor>,
        tx: UnboundedSender<ReplAction>,
    ) -> impl Fn(&Interrupt, &mut WindowContext) {
        move |_: &Interrupt, cx: &mut WindowContext| {
            if !JupyterSettings::enabled(cx) {
                return;
            }
            tx.unbounded_send(ReplAction {
                editor: editor.clone(),
                event: ReplEvent::Interrupt,
            })
            .ok();
        }
    }

    fn shutdown(
        editor: WeakView<Editor>,
        tx: UnboundedSender<ReplAction>,
    ) -> impl Fn(&Shutdown, &mut WindowContext) {
        move |_: &Shutdown, cx: &mut WindowContext| {
            if !JupyterSettings::enabled(cx) {
                return;
            }
            tx.unbounded_send(ReplAction {
                editor: editor.clone(),
                event: ReplEvent::Shutdown,
            })
            .ok();
        }
    }
}

pub enum ReplEvent {
    Run,
    ClearOutputs,
    Interrupt,
    Shutdown,
}

impl RuntimePanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let view = workspace.update(&mut cx, |workspace, cx| {
                cx.new_view::<Self>(|cx| {
                    let focus_handle = cx.focus_handle();

                    let fs = workspace.app_state().fs.clone();

                    // Make a channel that we receive editor events on (for repl::Run, repl::ClearOutputs)
                    // This allows us to inject actions on the editor from the repl panel without requiring the editor to
                    // depend on the `repl` crate.
                    let (repl_editor_event_tx, mut repl_editor_event_rx) =
                        futures::channel::mpsc::unbounded::<ReplAction>();

                    let subscriptions = vec![
                        cx.on_focus_in(&focus_handle, Self::focus_in),
                        cx.on_focus_out(&focus_handle, Self::focus_out),
                        cx.observe_global::<SettingsStore>(move |this, cx| {
                            this.set_enabled(JupyterSettings::enabled(cx), cx);
                        }),
                        cx.observe_new_views(
                            // For every editor that is a singleton, register actions on it
                            move |editor: &mut Editor, cx: &mut ViewContext<Editor>| {
                                let editor_view = cx.view().downgrade();
                                let clear_event_tx = repl_editor_event_tx.clone();

                                editor
                                    .register_action(ReplAction::run(
                                        editor_view.clone(),
                                        repl_editor_event_tx.clone(),
                                    ))
                                    .detach();

                                editor
                                    .register_action(ReplAction::clear_outputs(
                                        editor_view.clone(),
                                        repl_editor_event_tx.clone(),
                                    ))
                                    .detach();

                                editor
                                    .register_action(ReplAction::interrupt(
                                        editor_view.clone(),
                                        repl_editor_event_tx.clone(),
                                    ))
                                    .detach();

                                editor
                                    .register_action(ReplAction::shutdown(
                                        editor_view.clone(),
                                        repl_editor_event_tx.clone(),
                                    ))
                                    .detach();
                            },
                        ),
                    ];

                    // Listen for events from the editor on the `repl_editor_event_rx` channel
                    let _editor_events_task = cx.spawn(
                        move |this: WeakView<RuntimePanel>, mut cx: AsyncWindowContext| async move {
                            while let Some(action) = repl_editor_event_rx.next().await {
                                this.update(&mut cx, |runtime_panel, cx| {
                                    let entry_id =
                                        match RuntimePanel::entry_id_for_editor(&action.editor, cx)
                                        {
                                            Some(entry_id) => entry_id,
                                            None => return,
                                        };

                                    match action.event {
                                        ReplEvent::Run => {
                                            runtime_panel
                                                .run(action.editor, &entry_id, cx)
                                                .log_err();
                                        }
                                        ReplEvent::ClearOutputs => {
                                            runtime_panel.clear_outputs(&entry_id, cx);
                                        }
                                        ReplEvent::Interrupt => {
                                            runtime_panel.interrupt(&entry_id, cx);
                                        }
                                        ReplEvent::Shutdown => {
                                            runtime_panel.shutdown(&entry_id, cx);
                                        }
                                    }
                                })
                                .ok();
                            }
                        },
                    );

                    let runtime_panel = Self {
                        fs: fs.clone(),
                        width: None,
                        focus_handle,
                        kernel_specifications: Vec::new(),
                        sessions: Default::default(),
                        _subscriptions: subscriptions,
                        enabled: JupyterSettings::enabled(cx),
                        _editor_events_task,
                    };

                    runtime_panel
                })
            })?;

            view.update(&mut cx, |this, cx| this.refresh_kernelspecs(cx))?
                .await?;

            Ok(view)
        })
    }

    fn set_enabled(&mut self, enabled: bool, cx: &mut ViewContext<Self>) {
        if self.enabled != enabled {
            self.enabled = enabled;
            cx.notify();
        }
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn focus_out(&mut self, _event: FocusOutEvent, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    pub fn snippet(
        &self,
        editor: WeakView<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> Option<(ProjectEntryId, String, Arc<Language>, Range<Anchor>)> {
        let editor = editor.upgrade()?;
        let editor = editor.read(cx);

        let buffer = editor.buffer().read(cx);
        let buffer_snapshot = buffer.snapshot(cx);

        // Only allow code snippets for singletons
        let singleton = editor.buffer().read(cx).as_singleton()?;
        let entry_id = singleton.read(cx).entry_id(cx)?;

        let selection = editor.selections.newest::<usize>(cx);

        let range = if selection.is_empty() {
            let cursor = selection.head();

            let cursor_row = buffer_snapshot.offset_to_point(cursor).row;
            let start_offset = buffer_snapshot.point_to_offset(Point::new(cursor_row, 0));

            let end_point = Point::new(
                cursor_row,
                buffer_snapshot.line_len(MultiBufferRow(cursor_row)),
            );
            let end_offset = start_offset.saturating_add(end_point.column as usize);

            // Create a range from the start to the end of the line
            start_offset..end_offset
        } else {
            selection.range()
        };

        let anchor_range = range.to_anchors(&buffer_snapshot);

        let selected_text = buffer_snapshot
            .text_for_range(anchor_range.clone())
            .collect::<String>();

        let start_language = buffer_snapshot.language_at(anchor_range.start)?;
        let end_language = buffer_snapshot.language_at(anchor_range.end)?;
        if start_language != end_language {
            return None;
        }

        Some((
            entry_id,
            selected_text,
            start_language.clone(),
            anchor_range,
        ))
    }

    pub fn language(
        &self,
        editor: WeakView<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> Option<Arc<Language>> {
        let editor = editor.upgrade()?;
        let selection = editor.read(cx).selections.newest::<usize>(cx);
        let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
        buffer.language_at(selection.head()).cloned()
    }

    pub fn refresh_kernelspecs(&mut self, cx: &mut ViewContext<Self>) -> Task<anyhow::Result<()>> {
        let kernel_specifications = kernel_specifications(self.fs.clone());
        cx.spawn(|this, mut cx| async move {
            let kernel_specifications = kernel_specifications.await?;

            this.update(&mut cx, |this, cx| {
                this.kernel_specifications = kernel_specifications;
                cx.notify();
            })
        })
    }

    pub fn kernelspec(
        &self,
        language: &Language,
        cx: &mut ViewContext<Self>,
    ) -> Option<KernelSpecification> {
        let settings = JupyterSettings::get_global(cx);
        let language_name = language.code_fence_block_name();
        let selected_kernel = settings.kernel_selections.get(language_name.as_ref());

        self.kernel_specifications
            .iter()
            .find(|runtime_specification| {
                if let Some(selected) = selected_kernel {
                    // Top priority is the selected kernel
                    runtime_specification.name.to_lowercase() == selected.to_lowercase()
                } else {
                    // Otherwise, we'll try to find a kernel that matches the language
                    runtime_specification.kernelspec.language.to_lowercase()
                        == language_name.to_lowercase()
                }
            })
            .cloned()
    }

    pub fn run(
        &mut self,
        editor: WeakView<Editor>,
        _entry_id: &ProjectEntryId,
        cx: &mut ViewContext<Self>,
    ) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let (entry_id, selected_text, language, anchor_range) =
            match self.snippet(editor.clone(), cx) {
                Some(snippet) => snippet,
                None => return Ok(()),
            };

        let kernel_specification = self
            .kernelspec(&language, cx)
            .with_context(|| format!("No kernel found for language: {}", language.name()))?;

        let session = self.sessions.entry(entry_id).or_insert_with(|| {
            let view =
                cx.new_view(|cx| Session::new(editor, self.fs.clone(), kernel_specification, cx));
            cx.notify();

            let subscription = cx.subscribe(
                &view,
                |panel: &mut RuntimePanel, _session: View<Session>, event: &SessionEvent, cx| {
                    match event {
                        SessionEvent::Shutdown(editor) => {
                            let entry_id = RuntimePanel::entry_id_for_editor(editor, cx);

                            match entry_id {
                                Some(entry_id) => {
                                    panel.sessions.remove(&entry_id);
                                }
                                None => {
                                    log::error!(
                                        "No entry_id found for editor with a kernel session"
                                    );
                                    return;
                                }
                            }
                        }
                    }
                },
            );

            subscription.detach();

            view
        });

        session.update(cx, |session, cx| {
            session.execute(&selected_text, anchor_range, cx);
        });

        anyhow::Ok(())
    }

    pub fn clear_outputs(&mut self, entry_id: &ProjectEntryId, cx: &mut ViewContext<Self>) {
        if let Some(session) = self.sessions.get_mut(entry_id) {
            session.update(cx, |session, cx| {
                session.clear_outputs(cx);
            });
            cx.notify();
        }
    }

    pub fn interrupt(&mut self, entry_id: &ProjectEntryId, cx: &mut ViewContext<Self>) {
        if let Some(session) = self.sessions.get_mut(entry_id) {
            session.update(cx, |session, cx| {
                session.interrupt(cx);
            });
            cx.notify();
        }
    }

    pub fn shutdown(&self, entry_id: &ProjectEntryId, cx: &mut ViewContext<RuntimePanel>) {
        if let Some(session) = self.sessions.get(entry_id) {
            session.update(cx, |session, cx| {
                session.shutdown(cx);
            });
            cx.notify();
        }
    }
}

pub enum SessionSupport {
    ActiveSession(View<Session>),
    Inactive(Box<KernelSpecification>),
    RequiresSetup(Arc<str>),
    Unsupported,
}

impl RuntimePanel {
    pub fn entry_id_for_editor(
        editor: &WeakView<Editor>,
        cx: &mut WindowContext,
    ) -> Option<ProjectEntryId> {
        let editor = editor.upgrade()?;
        editor.update(cx, |editor, cx| editor.entry_id(cx))
    }

    pub fn session(
        &mut self,
        editor: WeakView<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> SessionSupport {
        let entry_id = match RuntimePanel::entry_id_for_editor(&editor, cx) {
            Some(entry_id) => entry_id,
            None => return SessionSupport::Unsupported,
        };

        let session = self.sessions.get(&entry_id).cloned();

        match session {
            Some(session) => SessionSupport::ActiveSession(session),
            None => {
                let language = self.language(editor, cx);
                let language = match language {
                    Some(language) => language,
                    None => return SessionSupport::Unsupported,
                };
                // Check for kernelspec
                let kernelspec = self.kernelspec(&language, cx);

                match kernelspec {
                    Some(kernelspec) => SessionSupport::Inactive(Box::new(kernelspec)),
                    None => {
                        // If no kernelspec but language is one of typescript or python
                        // then we return RequiresSetup
                        match language.name().as_ref() {
                            "TypeScript" | "Python" => {
                                SessionSupport::RequiresSetup(language.name())
                            }
                            _ => SessionSupport::Unsupported,
                        }
                    }
                }
            }
        }
    }
}

impl Panel for RuntimePanel {
    fn persistent_name() -> &'static str {
        "RuntimePanel"
    }

    fn position(&self, cx: &ui::WindowContext) -> workspace::dock::DockPosition {
        match JupyterSettings::get_global(cx).dock {
            JupyterDockPosition::Left => workspace::dock::DockPosition::Left,
            JupyterDockPosition::Right => workspace::dock::DockPosition::Right,
            JupyterDockPosition::Bottom => workspace::dock::DockPosition::Bottom,
        }
    }

    fn position_is_valid(&self, _position: workspace::dock::DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        position: workspace::dock::DockPosition,
        cx: &mut ViewContext<Self>,
    ) {
        settings::update_settings_file::<JupyterSettings>(self.fs.clone(), cx, move |settings| {
            let dock = match position {
                workspace::dock::DockPosition::Left => JupyterDockPosition::Left,
                workspace::dock::DockPosition::Right => JupyterDockPosition::Right,
                workspace::dock::DockPosition::Bottom => JupyterDockPosition::Bottom,
            };
            settings.set_dock(dock);
        })
    }

    fn size(&self, cx: &ui::WindowContext) -> Pixels {
        let settings = JupyterSettings::get_global(cx);

        self.width.unwrap_or(settings.default_width)
    }

    fn set_size(&mut self, size: Option<ui::Pixels>, _cx: &mut ViewContext<Self>) {
        self.width = size;
    }

    fn icon(&self, _cx: &ui::WindowContext) -> Option<ui::IconName> {
        if !self.enabled {
            return None;
        }

        Some(IconName::Code)
    }

    fn icon_tooltip(&self, _cx: &ui::WindowContext) -> Option<&'static str> {
        Some("Runtime Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
}

impl EventEmitter<PanelEvent> for RuntimePanel {}

impl FocusableView for RuntimePanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RuntimePanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // When there are no kernel specifications, show a link to the Zed docs explaining how to
        // install kernels. It can be assumed they don't have a running kernel if we have no
        // specifications.
        if self.kernel_specifications.is_empty() {
            return v_flex()
                .p_4()
                .size_full()
                .gap_2()
                        .child(Label::new("No Jupyter Kernels Available").size(LabelSize::Large))
                        .child(
                            Label::new("To start interactively running code in your editor, you need to install and configure Jupyter kernels.")
                                .size(LabelSize::Default),
                        )
                        .child(
                            h_flex().w_full().p_4().justify_center().gap_2().child(
                                ButtonLike::new("install-kernels")
                                    .style(ButtonStyle::Filled)
                                    .size(ButtonSize::Large)
                                    .layer(ElevationIndex::ModalSurface)
                                    .child(Label::new("Install Kernels"))
                                    .on_click(move |_, cx| {
                                        cx.open_url(
                                        "https://docs.jupyter.org/en/latest/install/kernels.html",
                                    )
                                    }),
                            ),
                        )
                .into_any_element();
        }

        // When there are no sessions, show the command to run code in an editor
        if self.sessions.is_empty() {
            return v_flex()
                .p_4()
                .size_full()
                .gap_2()
                .child(Label::new("No Jupyter Kernel Sessions").size(LabelSize::Large))
                .child(
                    v_flex().child(
                        Label::new("To run code in a Jupyter kernel, select some code and use the 'repl::Run' command.")
                            .size(LabelSize::Default)
                    )
                    .children(
                            KeyBinding::for_action(&Run, cx)
                            .map(|binding|
                                binding.into_any_element()
                            )
                    )
                )

                .into_any_element();
        }

        v_flex()
            .p_4()
            .child(Label::new("Jupyter Kernel Sessions").size(LabelSize::Large))
            .children(
                self.sessions
                    .values()
                    .map(|session| session.clone().into_any_element()),
            )
            .into_any_element()
    }
}
