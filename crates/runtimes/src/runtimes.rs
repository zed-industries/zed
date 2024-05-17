// Jupyter runtimed handling here

#[allow(unused_imports)]
use anyhow::{Context as _, Result};
#[allow(unused_imports)]
use client::Client;
use collections::HashMap;
use editor::Editor;
use futures::{channel::mpsc, SinkExt as _, StreamExt as _};
#[allow(unused_imports)]
use gpui::{actions, AppContext, Context, Global, Model, ModelContext, WeakView};
#[allow(unused_imports)]
use language::language_settings::all_language_settings;
use runtimelib::{ClientIoPubConnection, ClientShellConnection};
#[allow(unused_imports)]
use runtimelib::{
    ExecuteRequest, JupyterClient, JupyterMessage, JupyterMessageContent, JupyterRuntime, RuntimeId,
};
#[allow(unused_imports)]
use settings::SettingsStore;
use std::path::PathBuf;
#[allow(unused_imports)]
use std::sync::Arc;
use ui::prelude::*;
use workspace::Workspace;

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
    _runtime_handle: std::thread::JoinHandle<anyhow::Result<()>>,
}

// For now, we're going to connect to a running kernel that is already running
static HARDCODED_KERNEL: &str =
    "/Users/kylekelley/Library/Jupyter/runtime/kernel-1bd7cb84-018f-4eea-a7de-55c637581c3e.json";

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
                    dbg!(&message);

                    if let Some(parent_header) = message.parent_header {
                        let execution_id = ExecutionId::from(parent_header.msg_id);

                        if let Some(mut execution) = executions.lock().await.get(&execution_id) {
                            dbg!("Got that update, brah");
                            execution
                                .send(ExecutionUpdate {
                                    execution_id,
                                    update: message.content,
                                })
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
                    dbg!(&message);

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
            runtime.block_on(async move {
                // Set up the kernel client here as our prototype
                let kernel_path = std::path::PathBuf::from(HARDCODED_KERNEL);
                let document_client = DocumentClient::new(&kernel_path, execution_request_rx)
                    .await
                    .unwrap();

                dbg!("We made the client!");
                let join_fut = futures::future::try_join(
                    document_client.iopub_handle,
                    document_client.shell_handle,
                );

                join_fut.await?;
                Ok(())
            })
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
            .and_then(|editor| {
                let editor = editor.read(cx);
                let range = editor.selections.newest::<usize>(cx).range();
                let buffer = editor.buffer().read(cx).snapshot(cx);

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
                Some(selected_text)
            });

        dbg!(&code_snippet);

        if let Some(code) = code_snippet {
            if let Some(model) = RuntimeManager::global(cx) {
                let execution_id = ExecutionId::new();
                let mut receiver = model.read(cx).execute_code(execution_id, code.clone());

                cx.spawn(|_this, _cx| async move {
                    async move {
                        while let Some(update) = receiver.next().await {
                            println!("Update: {:?}", update.update);
                        }
                    }
                })
                .detach();
            }
        }
    }
}
