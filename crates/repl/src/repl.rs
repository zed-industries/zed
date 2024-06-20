use anyhow::{anyhow, Context as _, Result};
use async_dispatcher::{set_dispatcher, timeout, Dispatcher, Runnable};
use collections::{HashMap, HashSet};
use editor::{
    display_map::{
        BlockContext, BlockDisposition, BlockId, BlockProperties, BlockStyle, RenderBlock,
    },
    Anchor, AnchorRangeExt, Editor,
};
use futures::{
    channel::mpsc::{self, UnboundedSender},
    future::Shared,
    Future, FutureExt, SinkExt as _, StreamExt,
};
use gpui::prelude::*;
use gpui::{
    actions, AppContext, Context, EntityId, Global, Model, ModelContext, PlatformDispatcher, Task,
    WeakView,
};
use gpui::{Entity, View};
use language::Point;
use outputs::{ExecutionStatus, ExecutionView, LineHeight as _};
use project::Fs;
use runtime_settings::JupyterSettings;
use runtimelib::JupyterMessageContent;
use settings::{Settings as _, SettingsStore};
use std::{ops::Range, time::Instant};
use std::{sync::Arc, time::Duration};
use theme::{ActiveTheme, ThemeSettings};
use ui::prelude::*;
use workspace::Workspace;

mod outputs;
// mod runtime_panel;
mod runtime_settings;
mod runtimes;
mod stdio;

use runtimes::{get_runtime_specifications, Request, RunningKernel, RuntimeSpecification};

actions!(repl, [Run]);

#[derive(Clone)]
pub struct RuntimeManagerGlobal(Model<RuntimeManager>);

impl Global for RuntimeManagerGlobal {}

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

pub fn init(fs: Arc<dyn Fs>, cx: &mut AppContext) {
    set_dispatcher(zed_dispatcher(cx));
    JupyterSettings::register(cx);

    observe_jupyter_settings_changes(fs.clone(), cx);

    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace.register_action(run);
        },
    )
    .detach();

    let settings = JupyterSettings::get_global(cx);

    if !settings.enabled {
        return;
    }

    initialize_runtime_manager(fs, cx);
}

fn initialize_runtime_manager(fs: Arc<dyn Fs>, cx: &mut AppContext) {
    let runtime_manager = cx.new_model(|cx| RuntimeManager::new(fs.clone(), cx));
    RuntimeManager::set_global(runtime_manager.clone(), cx);

    cx.spawn(|mut cx| async move {
        let fs = fs.clone();

        let runtime_specifications = get_runtime_specifications(fs).await?;

        runtime_manager.update(&mut cx, |this, _cx| {
            this.runtime_specifications = runtime_specifications;
        })?;

        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn observe_jupyter_settings_changes(fs: Arc<dyn Fs>, cx: &mut AppContext) {
    cx.observe_global::<SettingsStore>(move |cx| {
        let settings = JupyterSettings::get_global(cx);
        if settings.enabled && RuntimeManager::global(cx).is_none() {
            initialize_runtime_manager(fs.clone(), cx);
        } else {
            RuntimeManager::remove_global(cx);
            // todo!(): Remove action from workspace(s)
        }
    })
    .detach();
}

#[derive(Debug)]
pub enum Kernel {
    RunningKernel(RunningKernel),
    StartingKernel(Shared<Task<()>>),
    FailedLaunch,
}

// Per workspace
pub struct RuntimeManager {
    fs: Arc<dyn Fs>,
    runtime_specifications: Vec<RuntimeSpecification>,

    instances: HashMap<EntityId, Kernel>,
    editors: HashMap<WeakView<Editor>, EditorRuntimeState>,
    // todo!(): Next
    // To reduce the number of open tasks and channels we have, let's feed the response
    // messages by ID over to the paired ExecutionView
    _execution_views_by_id: HashMap<String, View<ExecutionView>>,
}

#[derive(Debug, Clone)]
struct EditorRuntimeState {
    blocks: Vec<EditorRuntimeBlock>,
    // todo!(): Store a subscription to the editor so we can drop them when the editor is dropped
    // subscription: gpui::Subscription,
}

#[derive(Debug, Clone)]
struct EditorRuntimeBlock {
    code_range: Range<Anchor>,
    _execution_id: String,
    block_id: BlockId,
    _execution_view: View<ExecutionView>,
}

impl RuntimeManager {
    pub fn new(fs: Arc<dyn Fs>, _cx: &mut AppContext) -> Self {
        Self {
            fs,
            runtime_specifications: Default::default(),
            instances: Default::default(),
            editors: Default::default(),
            _execution_views_by_id: Default::default(),
        }
    }

    fn get_or_launch_kernel(
        &mut self,
        entity_id: EntityId,
        language_name: Arc<str>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<UnboundedSender<Request>>> {
        let kernel = self.instances.get(&entity_id);
        let pending_kernel_start = match kernel {
            Some(Kernel::RunningKernel(running_kernel)) => {
                return Task::ready(anyhow::Ok(running_kernel.request_tx.clone()));
            }
            Some(Kernel::StartingKernel(task)) => task.clone(),
            Some(Kernel::FailedLaunch) | None => {
                self.instances.remove(&entity_id);

                let kernel = self.launch_kernel(entity_id, language_name, cx);
                let pending_kernel = cx
                    .spawn(|this, mut cx| async move {
                        let running_kernel = kernel.await;

                        match running_kernel {
                            Ok(running_kernel) => {
                                let _ = this.update(&mut cx, |this, _cx| {
                                    this.instances
                                        .insert(entity_id, Kernel::RunningKernel(running_kernel));
                                });
                            }
                            Err(_err) => {
                                let _ = this.update(&mut cx, |this, _cx| {
                                    this.instances.insert(entity_id, Kernel::FailedLaunch);
                                });
                            }
                        }
                    })
                    .shared();

                self.instances
                    .insert(entity_id, Kernel::StartingKernel(pending_kernel.clone()));

                pending_kernel
            }
        };

        cx.spawn(|this, mut cx| async move {
            pending_kernel_start.await;

            this.update(&mut cx, |this, _cx| {
                let kernel = this
                    .instances
                    .get(&entity_id)
                    .ok_or(anyhow!("unable to get a running kernel"))?;

                match kernel {
                    Kernel::RunningKernel(running_kernel) => Ok(running_kernel.request_tx.clone()),
                    _ => Err(anyhow!("unable to get a running kernel")),
                }
            })?
        })
    }

    fn launch_kernel(
        &mut self,
        entity_id: EntityId,
        language_name: Arc<str>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<RunningKernel>> {
        // Get first runtime that matches the language name (for now)
        let runtime_specification =
            self.runtime_specifications
                .iter()
                .find(|runtime_specification| {
                    runtime_specification.kernelspec.language == language_name.to_string()
                });

        let runtime_specification = match runtime_specification {
            Some(runtime_specification) => runtime_specification,
            None => {
                return Task::ready(Err(anyhow::anyhow!(
                    "No runtime found for language {}",
                    language_name
                )));
            }
        };

        let runtime_specification = runtime_specification.clone();

        let fs = self.fs.clone();

        cx.spawn(|_, cx| async move {
            let running_kernel =
                RunningKernel::new(runtime_specification, entity_id, fs.clone(), cx);

            let running_kernel = running_kernel.await?;

            let mut request_tx = running_kernel.request_tx.clone();

            let overall_timeout_duration = Duration::from_secs(10);

            let start_time = Instant::now();

            loop {
                if start_time.elapsed() > overall_timeout_duration {
                    // todo!(): Kill the kernel
                    return Err(anyhow::anyhow!("Kernel did not respond in time"));
                }

                let (tx, rx) = mpsc::unbounded();
                match request_tx
                    .send(Request {
                        request: runtimelib::KernelInfoRequest {}.into(),
                        responses_rx: tx,
                    })
                    .await
                {
                    Ok(_) => {}
                    Err(_err) => {
                        break;
                    }
                };

                let mut rx = rx.fuse();

                let kernel_info_timeout = Duration::from_secs(1);

                let mut got_kernel_info = false;
                while let Ok(Some(message)) = timeout(kernel_info_timeout, rx.next()).await {
                    match message {
                        JupyterMessageContent::KernelInfoReply(_) => {
                            got_kernel_info = true;
                        }
                        _ => {}
                    }
                }

                if got_kernel_info {
                    break;
                }
            }

            anyhow::Ok(running_kernel)
        })
    }

    fn execute_code(
        &mut self,
        entity_id: EntityId,
        language_name: Arc<str>,
        code: String,
        cx: &mut ModelContext<Self>,
    ) -> impl Future<Output = Result<mpsc::UnboundedReceiver<JupyterMessageContent>>> {
        let (tx, rx) = mpsc::unbounded();

        let request_tx = self.get_or_launch_kernel(entity_id, language_name, cx);

        async move {
            let request_tx = request_tx.await?;

            request_tx
                .unbounded_send(Request {
                    request: runtimelib::ExecuteRequest {
                        code,
                        allow_stdin: false,
                        silent: false,
                        store_history: true,
                        stop_on_error: true,
                        ..Default::default()
                    }
                    .into(),
                    responses_rx: tx,
                })
                .context("Failed to send execution request")?;

            Ok(rx)
        }
    }

    pub fn global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<RuntimeManagerGlobal>()
            .map(|runtime_manager| runtime_manager.0.clone())
    }

    pub fn set_global(runtime_manager: Model<Self>, cx: &mut AppContext) {
        cx.set_global(RuntimeManagerGlobal(runtime_manager));
    }

    pub fn remove_global(cx: &mut AppContext) {
        if RuntimeManager::global(cx).is_some() {
            cx.remove_global::<RuntimeManagerGlobal>();
        }
    }
}

pub fn get_active_editor(
    workspace: &mut Workspace,
    cx: &mut ViewContext<Workspace>,
) -> Option<View<Editor>> {
    workspace
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx))
}

// Gets the active selection in the editor or the current line
pub fn selection(editor: View<Editor>, cx: &mut ViewContext<Workspace>) -> Range<Anchor> {
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

pub fn run(workspace: &mut Workspace, _: &Run, cx: &mut ViewContext<Workspace>) {
    let (editor, runtime_manager) = if let (Some(editor), Some(runtime_manager)) =
        (get_active_editor(workspace, cx), RuntimeManager::global(cx))
    {
        (editor, runtime_manager)
    } else {
        log::warn!("No active editor or runtime manager found");
        return;
    };

    let anchor_range = selection(editor.clone(), cx);

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
        return;
    };

    let language_name = if let Some(language_name) = language_name {
        language_name
    } else {
        return;
    };

    let entity_id = editor.entity_id();

    let execution_view = cx.new_view(|cx| ExecutionView::new(cx));

    // If any block overlaps with the new block, remove it
    // TODO: When inserting a new block, put it in order so that search is efficient
    let blocks_to_remove = runtime_manager.update(cx, |runtime_manager, _cx| {
        // Get the current `EditorRuntimeState` for this runtime_manager, inserting it if it doesn't exist
        let editor_runtime_state = runtime_manager
            .editors
            .entry(editor.downgrade())
            .or_insert_with(|| EditorRuntimeState { blocks: Vec::new() });

        let mut blocks_to_remove: HashSet<BlockId> = HashSet::default();

        editor_runtime_state.blocks.retain(|block| {
            if anchor_range.overlaps(&block.code_range, &buffer) {
                blocks_to_remove.insert(block.block_id);
                // Drop this block
                false
            } else {
                true
            }
        });

        blocks_to_remove
    });

    let blocks_to_remove = blocks_to_remove.clone();

    let block_id = editor.update(cx, |editor, cx| {
        editor.remove_blocks(blocks_to_remove, None, cx);
        let block = BlockProperties {
            position: anchor_range.end,
            height: execution_view.num_lines(cx).saturating_add(1),
            style: BlockStyle::Sticky,
            render: create_output_area_render(execution_view.clone()),
            disposition: BlockDisposition::Below,
        };

        editor.insert_blocks([block], None, cx)[0]
    });

    let receiver = runtime_manager.update(cx, |runtime_manager, cx| {
        let editor_runtime_state = runtime_manager
            .editors
            .entry(editor.downgrade())
            .or_insert_with(|| EditorRuntimeState { blocks: Vec::new() });

        let editor_runtime_block = EditorRuntimeBlock {
            code_range: anchor_range.clone(),
            block_id,
            _execution_view: execution_view.clone(),
            _execution_id: Default::default(),
        };

        editor_runtime_state
            .blocks
            .push(editor_runtime_block.clone());

        runtime_manager.execute_code(entity_id, language_name, selected_text.clone(), cx)
    });

    cx.spawn(|_this, mut cx| async move {
        execution_view.update(&mut cx, |execution_view, cx| {
            execution_view.set_status(ExecutionStatus::ConnectingToKernel, cx);
        })?;
        let mut receiver = receiver.await?;

        let execution_view = execution_view.clone();
        while let Some(content) = receiver.next().await {
            execution_view.update(&mut cx, |execution_view, cx| {
                execution_view.push_message(&content, cx)
            })?;

            editor.update(&mut cx, |editor, cx| {
                let mut replacements = HashMap::default();
                replacements.insert(
                    block_id,
                    (
                        Some(execution_view.num_lines(cx).saturating_add(1)),
                        create_output_area_render(execution_view.clone()),
                    ),
                );
                editor.replace_blocks(replacements, None, cx);
            })?;
        }
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn create_output_area_render(execution_view: View<ExecutionView>) -> RenderBlock {
    let render = move |cx: &mut BlockContext| {
        let execution_view = execution_view.clone();
        let text_font = ThemeSettings::get_global(cx).buffer_font.family.clone();
        // Note: we'll want to use `cx.anchor_x` when someone runs something with no output -- just show a checkmark and not make the full block below the line

        let gutter_width = cx.gutter_dimensions.width;

        h_flex()
            .w_full()
            .bg(cx.theme().colors().background)
            .border_y_1()
            .border_color(cx.theme().colors().border)
            .pl(gutter_width)
            .child(
                div()
                    .font_family(text_font)
                    // .ml(gutter_width)
                    .mx_1()
                    .my_2()
                    .h_full()
                    .w_full()
                    .mr(gutter_width)
                    .child(execution_view),
            )
            .into_any_element()
    };

    Box::new(render)
}
