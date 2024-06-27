use anyhow::Result;
use async_dispatcher::{set_dispatcher, Dispatcher, Runnable};
use client::telemetry::Telemetry;
use collections::HashMap;
use editor::{Anchor, Editor};
use gpui::{
    actions, prelude::*, AppContext, AsyncWindowContext, Entity, EntityId, EventEmitter,
    FocusHandle, FocusOutEvent, FocusableView, Model, PlatformDispatcher, Subscription, Task, View,
    WeakView,
};
use language::Point;
use project::Fs;
use settings::{Settings as _, SettingsStore};
use std::{ops::Range, sync::Arc, time::Duration};
use ui::prelude::*;
use workspace::{
    dock::{Panel, PanelEvent},
    Workspace,
};

use crate::{
    runtime_session::ExecutionId, runtimes::Kernel, ExecutionView, JupyterSettings, RuntimeManager,
    Session,
};

actions!(repl, [Run, ToggleFocus]);

pub fn zed_dispatcher(cx: &mut AppContext) -> impl Dispatcher {
    struct ZedDispatcher {
        dispatcher: Arc<dyn PlatformDispatcher>,
    }

    // PlatformDispatcher is _super_ close to the same interface we put in
    // async-dispatcher, except for the task label in dispatch. Later we should
    // just make that consistent so we have this dispatcher ready to go for
    // other crates in Zed.
    impl Dispatcher for ZedDispatcher {
        fn dispatch(&self, runnable: Runnable) {
            self.dispatcher.dispatch(runnable, None)
        }

        fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
            self.dispatcher.dispatch_after(duration, runnable);
        }
    }

    ZedDispatcher {
        dispatcher: cx.background_executor().dispatcher.clone(),
    }
}

pub fn init(cx: &mut AppContext) {
    set_dispatcher(zed_dispatcher(cx));
    JupyterSettings::register(cx);
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace
                .register_action(|workspace, _: &ToggleFocus, cx| {
                    let settings = JupyterSettings::get_global(cx);
                    if !settings.enabled {
                        return;
                    }

                    workspace.toggle_panel_focus::<RuntimePanel>(cx);
                })
                .register_action(run);
            // .register_action(RuntimePanel::start_kernel)
            // .register_action(RuntimePanel::stop_kernel)
            // .register_action(RuntimePanel::execute_code);
        },
    )
    .detach();
}

#[allow(unused)]
pub struct RuntimePanel {
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
    sessions: HashMap<EntityId, View<Session>>,
    running_kernels: HashMap<EntityId, Kernel>,
    runtime_manager: Model<RuntimeManager>,
    execution_views: HashMap<String, View<ExecutionView>>,
    telemetry: Arc<Telemetry>,
    fs: Arc<dyn Fs>,
    _subscriptions: Vec<Subscription>,
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

                    let runtime_manager = cx.new_model(|cx| RuntimeManager::new(fs.clone(), cx));
                    RuntimeManager::set_global(runtime_manager.clone(), cx);

                    let subscriptions = vec![
                        cx.on_focus_in(&focus_handle, Self::focus_in),
                        cx.on_focus_out(&focus_handle, Self::focus_out),
                        cx.observe(&runtime_manager, Self::handle_update),
                        cx.observe_global::<SettingsStore>(move |_this, cx| {
                            let settings = JupyterSettings::get_global(cx);
                            if settings.enabled && RuntimeManager::global(cx).is_none() {
                                // todo!(): turn panel and operations on and off
                            } else {
                                // todo!()
                            }
                        }),
                    ];

                    Self {
                        width: None,
                        focus_handle,
                        runtime_manager: runtime_manager.clone(),
                        sessions: Default::default(),
                        workspace: workspace.weak_handle(),
                        running_kernels: Default::default(),
                        // editors: Default::default(),
                        execution_views: Default::default(),
                        telemetry: workspace.client().telemetry().clone(),
                        fs: fs.clone(),
                        _subscriptions: subscriptions,
                    }
                })
            })?;

            view.update(&mut cx, |this, cx| {
                this.runtime_manager
                    .update(cx, |runtime_manager, cx| runtime_manager.load(cx))
            })?;

            Ok(view)
        })
    }

    pub fn handle_update(&mut self, _model: Model<RuntimeManager>, cx: &mut ViewContext<Self>) {
        dbg!(_model);
        cx.notify();
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn focus_out(&mut self, _event: FocusOutEvent, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    // Gets the active selection in the editor or the current line
    pub fn selection(&self, editor: View<Editor>, cx: &mut ViewContext<Self>) -> Range<Anchor> {
        let editor = editor.read(cx);
        let selection = editor.selections.newest::<usize>(cx);
        let buffer = editor.buffer().read(cx).snapshot(cx);

        let range = if selection.is_empty() {
            let cursor = selection.head();

            let line_start = buffer.offset_to_point(cursor).row;
            let mut start_offset = buffer.point_to_offset(Point::new(line_start, 0));

            // Iterate backwards to find the start of the line
            while start_offset > 0 {
                let ch = buffer.chars_at(start_offset - 1).next().unwrap_or('\0');
                if ch == '\n' {
                    break;
                }
                start_offset -= 1;
            }

            let mut end_offset = cursor;

            // Iterate forwards to find the end of the line
            while end_offset < buffer.len() {
                let ch = buffer.chars_at(end_offset).next().unwrap_or('\0');
                if ch == '\n' {
                    break;
                }
                end_offset += 1;
            }

            // Create a range from the start to the end of the line
            start_offset..end_offset
        } else {
            selection.range()
        };

        let anchor_range = buffer.anchor_before(range.start)..buffer.anchor_after(range.end);
        anchor_range
    }

    pub fn snippet(
        &self,
        editor: View<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> Option<(String, Arc<str>)> {
        let anchor_range = self.selection(editor.clone(), cx);

        let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);

        let selected_text = buffer
            .text_for_range(anchor_range.clone())
            .collect::<String>();

        let start_language = buffer.language_at(anchor_range.start);
        let end_language = buffer.language_at(anchor_range.end);

        let language_name = if start_language == end_language {
            start_language
                .map(|language| language.code_fence_block_name())
                .filter(|lang| **lang != *"markdown")
        } else {
            // If the selection spans multiple languages, don't run it
            return None;
        };

        let language_name = if let Some(language_name) = language_name {
            language_name
        } else {
            return None;
        };

        return Some((selected_text, language_name));
    }

    pub fn run(
        &mut self,
        editor: View<Editor>,
        fs: Arc<dyn Fs>,
        cx: &mut ViewContext<Self>,
    ) -> anyhow::Result<()> {
        let (selected_text, language_name) = match self.snippet(editor.clone(), cx) {
            Some(snippet) => snippet,
            None => return anyhow::Ok(()),
        };

        let entity_id = editor.entity_id();

        let runtime_manager = self.runtime_manager.clone();
        let runtime_manager = runtime_manager.read(cx);

        let runtime_specification = runtime_manager
            .kernelspec(language_name.clone())
            .ok_or_else(|| anyhow::anyhow!("No kernel found for language: {}", language_name))?;

        let session = self.sessions.entry(entity_id).or_insert_with(|| {
            cx.new_view(|cx| Session::new(editor, fs, runtime_specification, cx))
        });

        // todo!(): Check if session uses the same language as the snippet

        session.update(cx, |session, cx| {
            let execution_id = ExecutionId::new();

            session.run(&execution_id, &selected_text, cx).ok();
        });

        anyhow::Ok(())
    }
}

pub fn run(workspace: &mut Workspace, _: &Run, cx: &mut ViewContext<Workspace>) {
    let settings = JupyterSettings::get_global(cx);
    if !settings.enabled {
        return;
    }

    let fs = workspace.app_state().fs.clone();

    let editor = workspace
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx));

    if let (Some(editor), Some(runtime_panel)) = (editor, workspace.panel::<RuntimePanel>(cx)) {
        runtime_panel.update(cx, |runtime_panel, cx| {
            runtime_panel.run(editor, fs, cx);
        });
    }
}

impl Panel for RuntimePanel {
    fn persistent_name() -> &'static str {
        "RuntimePanel"
    }

    fn position(&self, _cx: &ui::WindowContext) -> workspace::dock::DockPosition {
        // todo!(): Pull from settings
        workspace::dock::DockPosition::Right
    }

    fn position_is_valid(&self, _position: workspace::dock::DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        _position: workspace::dock::DockPosition,
        _cx: &mut ViewContext<Self>,
    ) {
        // todo!()
    }

    fn size(&self, cx: &ui::WindowContext) -> Pixels {
        let settings = JupyterSettings::get_global(cx);

        self.width.unwrap_or(settings.default_width)
    }

    fn set_size(&mut self, size: Option<ui::Pixels>, _cx: &mut ViewContext<Self>) {
        self.width = size;
    }

    fn icon(&self, _cx: &ui::WindowContext) -> Option<ui::IconName> {
        // todo!(): get an icon for runtime panel
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
        let runtime_manager = self.runtime_manager.read(cx);

        v_flex()
            .child(Label::new("Runtime Panel"))
            .children(
                runtime_manager
                    .runtime_specifications
                    .iter()
                    .map(|spec| div().child(spec.name.clone())),
            )
            .child(Label::new("Sessions"))
            .children(self.sessions.iter().map(|(entity_id, session)| {
                let entity_id = *entity_id;

                let session = session.clone();

                div()
                    .child(format!("Entity: {}", entity_id))
                    .child(session.into_any_element())
            }))
            .into_any_element()
    }
}

// Goal: move the execution views to be owned by the runtime panel
//       and to have all messages get collected as one stream
