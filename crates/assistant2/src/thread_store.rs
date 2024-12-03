use std::sync::Arc;

use anyhow::Result;
use assistant_tool::{ToolId, ToolWorkingSet};
use collections::HashMap;
use context_server::manager::ContextServerManager;
use context_server::{ContextServerFactoryRegistry, ContextServerTool};
use gpui::{prelude::*, AppContext, Model, ModelContext, Task};
use project::Project;
use util::ResultExt as _;

pub struct ThreadStore {
    #[allow(unused)]
    project: Model<Project>,
    tools: Arc<ToolWorkingSet>,
    context_server_manager: Model<ContextServerManager>,
    context_server_tool_ids: HashMap<Arc<str>, Vec<ToolId>>,
}

impl ThreadStore {
    pub fn new(
        project: Model<Project>,
        tools: Arc<ToolWorkingSet>,
        cx: &mut AppContext,
    ) -> Task<Result<Model<Self>>> {
        cx.spawn(|mut cx| async move {
            let this = cx.new_model(|cx: &mut ModelContext<Self>| {
                let context_server_factory_registry =
                    ContextServerFactoryRegistry::default_global(cx);
                let context_server_manager = cx.new_model(|cx| {
                    ContextServerManager::new(context_server_factory_registry, project.clone(), cx)
                });

                let this = Self {
                    project,
                    tools,
                    context_server_manager,
                    context_server_tool_ids: HashMap::default(),
                };
                this.register_context_server_handlers(cx);

                this
            })?;

            Ok(this)
        })
    }

    fn register_context_server_handlers(&self, cx: &mut ModelContext<Self>) {
        cx.subscribe(
            &self.context_server_manager.clone(),
            Self::handle_context_server_event,
        )
        .detach();
    }

    fn handle_context_server_event(
        &mut self,
        context_server_manager: Model<ContextServerManager>,
        event: &context_server::manager::Event,
        cx: &mut ModelContext<Self>,
    ) {
        let tool_working_set = self.tools.clone();
        match event {
            context_server::manager::Event::ServerStarted { server_id } => {
                if let Some(server) = context_server_manager.read(cx).get_server(server_id) {
                    let context_server_manager = context_server_manager.clone();
                    cx.spawn({
                        let server = server.clone();
                        let server_id = server_id.clone();
                        |this, mut cx| async move {
                            let Some(protocol) = server.client() else {
                                return;
                            };

                            if protocol.capable(context_server::protocol::ServerCapability::Tools) {
                                if let Some(tools) = protocol.list_tools().await.log_err() {
                                    let tool_ids = tools
                                        .tools
                                        .into_iter()
                                        .map(|tool| {
                                            log::info!(
                                                "registering context server tool: {:?}",
                                                tool.name
                                            );
                                            tool_working_set.insert(Arc::new(
                                                ContextServerTool::new(
                                                    context_server_manager.clone(),
                                                    server.id(),
                                                    tool,
                                                ),
                                            ))
                                        })
                                        .collect::<Vec<_>>();

                                    this.update(&mut cx, |this, _cx| {
                                        this.context_server_tool_ids.insert(server_id, tool_ids);
                                    })
                                    .log_err();
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
            context_server::manager::Event::ServerStopped { server_id } => {
                if let Some(tool_ids) = self.context_server_tool_ids.remove(server_id) {
                    tool_working_set.remove(&tool_ids);
                }
            }
        }
    }
}
