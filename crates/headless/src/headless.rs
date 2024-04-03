use anyhow::Result;
use client::{user::UserStore, Client};
use fs::Fs;
use gpui::{AppContext, AsyncAppContext, Context, Global, Model, ModelContext};
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use rpc::{proto, TypedEnvelope};
use std::sync::Arc;

pub struct DevServer {
    client: Arc<Client>,
    app_state: AppState,
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
            app_state,
            client,
        }
    }

    async fn handle_dev_server_instructions(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::DevServerInstructions>,
        _: Arc<Client>,
        cx: AsyncAppContext,
    ) -> Result<()> {
        dbg!(&envelope);
        for remote_project in envelope.payload.projects {
            this.update(&mut cx, move |this, cx| {
                this.share_project(&remote_project, cx)
            });
        }
        Ok(())
    }

    async fn share_project(
        &self,
        remote_project: &proto::RemoteProject,
        cx: &mut ModelContext<Self>,
    ) {
        let project = Project::local(
            self.client.clone(),
            self.app_state.node_runtime.clone(),
            self.app_state.user_store.clone(),
            self.app_state.languages.clone(),
            self.app_state.fs.clone(),
            cx,
        );

        let request = self.client.request(proto::ShareRemoteProject {
            remote_project_id: remote_project.id,
            worktrees: remote_project.read(cx).worktree_metadata_protos(cx),
        });
    }
}
