// Jupyter runtimed handling here

#[allow(unused_imports)]
use anyhow::{Context as _, Result};
#[allow(unused_imports)]
use client::Client;
use collections::HashMap;
use editor::{
    display_map::{BlockContext, BlockDisposition, BlockProperties, BlockStyle},
    Anchor, Editor,
};
use futures::{channel::mpsc, SinkExt as _, StreamExt as _};
#[allow(unused_imports)]
use gpui::{actions, AppContext, Context, Global, Model, ModelContext, WeakView};
#[allow(unused_imports)]
use language::language_settings::all_language_settings;
use language::Point;
use runtimelib::MimeType;
#[allow(unused_imports)]
use runtimelib::{
    ExecuteRequest, JupyterClient, JupyterMessage, JupyterMessageContent, JupyterRuntime, RuntimeId,
};
use serde_json::Value;
use settings::Settings as _;
#[allow(unused_imports)]
use settings::SettingsStore;
#[allow(unused_imports)]
use std::sync::Arc;
use std::{ops::Range, path::PathBuf};
use ui::prelude::*;
use util::ResultExt;
use workspace::Workspace;

use theme::{ActiveTheme, ThemeSettings};

actions!(runtimes, [Run]);

#[derive(Clone)]
pub struct RuntimeGlobal(Model<RuntimeManager>);

/*

# Editor Document

* Users will expect one runtime per editor.

Hashmap<Identifier, Async Channel>

```python
// Status::busy()
print("This my my model");      -> StreamContent { name: "stdout", text: "This my my model" }
df = pd.read_csv("myfile.csv");
display(df.tail()); // -> DisplayData { data: {"text/html": "<table>...</table>", "text/plain": "   ...\n"} }

df.head()  -> ExecuteResult { execution_count: 1, data: {"text/html": "<table>...</table>", "text/plain": "   ...\n"} }"} }

// Status::idle()
```

Message ID -> (Anchor, Outputs)
   * Anchor
   * Vec<Output> (ExecuteResult, DisplayData, StreamContent, ErrorOutput)


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub msg_id: String,
    pub username: String,
    pub session: String,
    pub date: DateTime<Utc>,
    pub msg_type: String,
    pub version: String,
}

*/

impl Global for RuntimeGlobal {}

/** On startup, we will look for all available kernels, or so I expect */

pub fn init(cx: &mut AppContext) {
    let runtime = cx.new_model(|cx| RuntimeManager::new(cx));
    RuntimeManager::set_global(runtime.clone(), cx);

    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            // Note: this will have to both start a kernel if not already running, and run code selections
            workspace.register_action(RuntimeManager::run);
        },
    )
    .detach();
}

#[derive(Debug)]
struct ExecutionRequest {
    execution_id: ExecutionId,
    request: runtimelib::ExecuteRequest,
    response_sender: mpsc::UnboundedSender<ExecutionUpdate>,
}

pub struct RuntimeManager {
    execution_request_tx: mpsc::UnboundedSender<ExecutionRequest>,
    _runtime_handle: std::thread::JoinHandle<()>,
}

// For now, we're going to connect to a running kernel that is already running
// static HARDCODED_KERNEL: &str =
// "/Users/kylekelley/Library/Jupyter/runtime/kernel-1bd7cb84-018f-4eea-a7de-55c637581c3e.json";

static HARDCODED_KERNEL: &str =
    "/Users/kylekelley/Library/Jupyter/runtime/kernel-af08b239-ed3a-43ec-8eaa-431f7beef959.json";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ExecutionId(String);

impl ExecutionId {
    fn new() -> Self {
        ExecutionId(uuid::Uuid::new_v4().to_string())
    }
}

impl From<String> for ExecutionId {
    fn from(id: String) -> Self {
        ExecutionId(id)
    }
}

#[derive(Debug)]
struct ExecutionUpdate {
    #[allow(dead_code)]
    execution_id: ExecutionId,
    update: JupyterMessageContent,
}

// Associates execution IDs with outputs and other messages
struct DocumentClient {
    iopub_handle: tokio::task::JoinHandle<anyhow::Result<()>>,
    shell_handle: tokio::task::JoinHandle<anyhow::Result<()>>,
    executions:
        Arc<tokio::sync::Mutex<HashMap<ExecutionId, mpsc::UnboundedSender<ExecutionUpdate>>>>,
}

impl DocumentClient {
    async fn new(
        kernel_path: &PathBuf,
        mut execution_request_rx: mpsc::UnboundedReceiver<ExecutionRequest>,
    ) -> Result<Self> {
        let connection_info = runtimelib::ConnectionInfo::from_path(kernel_path).await?;

        let mut iopub = connection_info.create_client_iopub_connection("").await?;
        let mut shell = connection_info.create_client_shell_connection().await?;

        let executions: Arc<
            tokio::sync::Mutex<HashMap<ExecutionId, mpsc::UnboundedSender<ExecutionUpdate>>>,
        > = Default::default();

        let iopub_handle = tokio::spawn({
            let executions = executions.clone();
            async move {
                loop {
                    let message = iopub.read().await?;

                    if let Some(parent_header) = message.parent_header {
                        let execution_id = ExecutionId::from(parent_header.msg_id);

                        if let Some(mut execution) = executions.lock().await.get(&execution_id) {
                            execution
                                .send(dbg!(ExecutionUpdate {
                                    execution_id,
                                    update: message.content,
                                }))
                                .await
                                .ok();
                        }
                    }
                }

                // anyhow::Ok(())
            }
        });

        let shell_handle = tokio::spawn({
            let executions = executions.clone();
            async move {
                while let Some(execution) = execution_request_rx.next().await {
                    let mut message: JupyterMessage = execution.request.into();
                    message.header.msg_id = execution.execution_id.0.clone();

                    executions
                        .lock()
                        .await
                        .insert(execution.execution_id, execution.response_sender);

                    shell
                        .send(message)
                        .await
                        .map_err(|e| anyhow::anyhow!("Failed to send execute request: {e:?}"))?;
                }
                anyhow::Ok(())
            }
        });

        let document_client = Self {
            iopub_handle,
            shell_handle,
            executions: Default::default(),
        };

        Ok(document_client)
    }
}

impl RuntimeManager {
    pub fn new(_cx: &mut AppContext) -> Self {
        let (execution_request_tx, execution_request_rx) = mpsc::unbounded::<ExecutionRequest>();

        let _runtime_handle = std::thread::spawn(|| {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("tokio failed to start");

            // TODO: Will need a signal handler to shutdown the runtime
            runtime
                .block_on(async move {
                    // Set up the kernel client here as our prototype
                    let kernel_path = std::path::PathBuf::from(HARDCODED_KERNEL);
                    let document_client =
                        DocumentClient::new(&kernel_path, execution_request_rx).await?;

                    let join_fut = futures::future::try_join(
                        document_client.iopub_handle,
                        document_client.shell_handle,
                    );

                    let results = join_fut.await?;

                    if let Err(e) = results.0 {
                        log::error!("iopub error: {e:?}");
                    }
                    if let Err(e) = results.1 {
                        log::error!("shell error: {e:?}");
                    }

                    anyhow::Ok(())
                })
                .log_err();
        });
        Self {
            execution_request_tx,
            _runtime_handle,
        }
    }

    fn execute_code(
        &self,
        execution_id: ExecutionId,
        code: String,
    ) -> mpsc::UnboundedReceiver<ExecutionUpdate> {
        let (tx, rx) = mpsc::unbounded();

        self.execution_request_tx
            .unbounded_send(ExecutionRequest {
                execution_id,
                request: runtimelib::ExecuteRequest {
                    code,
                    allow_stdin: false,
                    silent: false,
                    store_history: true,
                    user_expressions: None,
                    stop_on_error: false,
                    // TODO(runtimelib): set up Default::default() for the rest of the fields
                    // ..Default::default()
                },
                response_sender: tx,
            })
            .expect("Failed to send execution request");

        rx
    }

    pub fn global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<RuntimeGlobal>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(runtime: Model<Self>, cx: &mut AppContext) {
        cx.set_global(RuntimeGlobal(runtime));
    }

    pub fn run(workspace: &mut Workspace, _: &Run, cx: &mut ViewContext<Workspace>) {
        let code_snippet = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
            .and_then(|editor_view| {
                let editor = editor_view.read(cx);
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

                // // TODO(): Put block decoration after last bit of code that isn't whitespace.
                // //         There is no `char_at`. There's a `chars_at`, but you've gotta iterate
                // while end > range.start && buffer.char_at(end - 1).next().is_whitespace() {
                //     end -= 1;
                // }

                let anchor = buffer.anchor_after(range.end);

                // For handling of markdown documents, we'll need to get the language name
                // based on where we're at in the document.
                //
                // let start_language = buffer.language_at(range.start);
                // let end_language = buffer.language_at(range.end);
                // let language_name = if start_language == end_language {
                //     start_language.map(|language| language.code_fence_block_name())
                // } else {
                //     None
                // };
                // let language_name = language_name.as_deref().unwrap_or("");

                let selected_text = buffer.text_for_range(range).collect::<String>();

                Some((selected_text, anchor, editor_view))
            });

        if let Some((code, anchor, editor)) = code_snippet {
            if let Some(model) = RuntimeManager::global(cx) {
                let execution_id = ExecutionId::new();
                let mut receiver = model
                    .read(cx)
                    .execute_code(execution_id.clone(), code.clone());

                cx.spawn(|_this, mut cx| async move {
                    while let Some(update) = receiver.next().await {
                        let priority_order = vec![MimeType::Plain, MimeType::Markdown];

                        // A bit hacky for the moment
                        let output: Option<(MimeType, Value)> = match update.update {
                            JupyterMessageContent::ExecuteResult(result) => {
                                result.data.richest(&priority_order)
                            }
                            JupyterMessageContent::DisplayData(result) => {
                                result.data.richest(&priority_order)
                            }
                            JupyterMessageContent::StreamContent(result) => {
                                Some((MimeType::Plain, Value::from(result.text)))
                            }
                            JupyterMessageContent::ErrorOutput(result) => {
                                Some((MimeType::Other, Value::from(result.ename)))
                            }
                            _ => continue,
                        };

                        let output = match output {
                            Some((_mime_type, value)) => value.as_str().unwrap_or("").to_string(),
                            None => continue,
                        };

                        editor.update(&mut cx, |editor, cx| {
                            render_output_block(editor, output.into(), anchor, cx);
                        })?;
                    }
                    anyhow::Ok(())
                })
                .detach();
            }
        }
    }
}

fn render_output_block(
    editor: &mut Editor,
    output: SharedString,
    position: Anchor,
    cx: &mut ViewContext<Editor>,
) {
    // This will only work for plain text output, not sure how we'll handle images
    let height = output.lines().count() as u8;

    dbg!(height);

    let render = move |cx: &mut BlockContext| {
        let text_font = ThemeSettings::get_global(cx).buffer_font.family.clone();
        let anchor_x = cx.anchor_x;
        let gutter_width = cx.gutter_dimensions.width;

        h_flex()
            .w_full()
            .border_y_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    // .justify_center()
                    .w(gutter_width), // .child(Icon::new(IconName::Screen).color(Color::Hint)),
            )
            .child(
                h_flex()
                    .font_family(text_font)
                    .w_full()
                    .ml(anchor_x - gutter_width)
                    .mt_2()
                    .child(output.clone()),
            )
            .into_any_element()
    };

    editor.insert_blocks(
        [BlockProperties {
            position,
            height: height + 1,
            style: BlockStyle::Sticky,
            render: Box::new(render),
            disposition: BlockDisposition::Below,
        }],
        None,
        cx,
    );
}
