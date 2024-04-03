use anyhow::Result;
use client::{user::UserStore, Client};
use fs::Fs;
use gpui::{AppContext, AsyncAppContext, Context, Global, Model, ModelContext};
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use project::Project;
use rpc::{proto, TypedEnvelope};
use std::{collections::HashMap, future::Future, sync::Arc};

pub struct DevServer {
    client: Arc<Client>,
    app_state: AppState,
    projects: HashMap<u64, Model<Project>>,
    _subscriptions: Vec<client::Subscription>,
}

pub struct AppState {
    pub node_runtime: Arc<dyn NodeRuntime>,
    pub user_store: Model<UserStore>,
    pub languages: Arc<LanguageRegistry>,
    pub fs: Arc<dyn Fs>,
}

struct GlobalDevServer(Model<DevServer>);

impl Global for GlobalDevServer {}

pub fn init(client: Arc<Client>, app_state: AppState, cx: &mut AppContext) {
    let dev_server = cx.new_model(|cx| DevServer::new(client, app_state, cx));
    cx.set_global(GlobalDevServer(dev_server));
}

impl DevServer {
    pub fn global(cx: &AppContext) -> Model<DevServer> {
        cx.global::<GlobalDevServer>().0.clone()
    }

    pub fn new(client: Arc<Client>, app_state: AppState, cx: &mut ModelContext<Self>) -> Self {
        DevServer {
            _subscriptions: vec![
                client.add_message_handler(cx.weak_model(), Self::handle_dev_server_instructions)
            ],
            projects: Default::default(),
            app_state,
            client,
        }
    }

    async fn handle_dev_server_instructions(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::DevServerInstructions>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        dbg!(&envelope);
        for remote_project in &envelope.payload.projects {
            DevServer::share_project(this.clone(), remote_project, &mut cx).await?;
        }
        Ok(())
    }

    async fn share_project(
        this: Model<Self>,
        remote_project: &proto::RemoteProject,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        let (client, project, worktrees) = this.update(cx, |this, cx| {
            let project = Project::local(
                this.client.clone(),
                this.app_state.node_runtime.clone(),
                this.app_state.user_store.clone(),
                this.app_state.languages.clone(),
                this.app_state.fs.clone(),
                cx,
            );
            let worktrees = project.read(cx).worktree_metadata_protos(cx);
            (this.client.clone(), project, worktrees)
        })?;

        let request = client.request(proto::ShareRemoteProject {
            remote_project_id: remote_project.id,
            worktrees,
        });

        let response = request.await?;
        dbg!(&response);
        this.update(cx, |this, _| {
            this.projects.insert(response.project_id, project);
        });
        Ok(())
    }
}
