use anyhow::Result;
use client::{user::UserStore, Client, RemoteProjectId};
use fs::Fs;
use futures::Future;
use gpui::{AppContext, AsyncAppContext, Context, Global, Model, ModelContext};
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use project::Project;
use rpc::{proto, TypedEnvelope};
use std::{collections::HashMap, sync::Arc};
use util::ResultExt;

pub struct DevServer {
    client: Arc<Client>,
    app_state: AppState,
    projects: HashMap<RemoteProjectId, Model<Project>>,
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
    cx.set_global(GlobalDevServer(dev_server.clone()));

    // Set up a handler when the dev server is shut down by the user pressing Ctrl-C

    let (tx, rx) = futures::channel::oneshot::channel();
    set_ctrlc_handler(move || tx.send(()).log_err().unwrap()).log_err();

    cx.spawn(|cx| async move {
        rx.await.log_err();
        log::info!("Received interrupt signal");
        cx.update(|cx| cx.quit()).log_err();
    })
    .detach();
}

fn set_ctrlc_handler<F>(f: F) -> Result<(), ctrlc::Error>
where
    F: FnOnce() + 'static + Send,
{
    let f = std::sync::Mutex::new(Some(f));
    ctrlc::set_handler(move || {
        if let Ok(mut guard) = f.lock() {
            let f = guard.take().expect("f can only be taken once");
            f();
        }
    })
}

impl DevServer {
    pub fn global(cx: &AppContext) -> Model<DevServer> {
        cx.global::<GlobalDevServer>().0.clone()
    }

    pub fn new(client: Arc<Client>, app_state: AppState, cx: &mut ModelContext<Self>) -> Self {
        cx.on_app_quit(Self::app_will_quit).detach();

        DevServer {
            _subscriptions: vec![
                client.add_message_handler(cx.weak_model(), Self::handle_dev_server_instructions)
            ],
            projects: Default::default(),
            app_state,
            client,
        }
    }

    fn app_will_quit(&mut self, _: &mut ModelContext<Self>) -> impl Future<Output = ()> {
        let request = self.client.request(proto::ShutdownDevServer {});
        async move {
            request.await.log_err();
        }
    }

    async fn handle_dev_server_instructions(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::DevServerInstructions>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let (added_projects, removed_projects_ids) = this.read_with(&mut cx, |this, _| {
            let removed_projects = this
                .projects
                .keys()
                .filter(|remote_project_id| {
                    !envelope
                        .payload
                        .projects
                        .iter()
                        .any(|p| p.id == remote_project_id.0)
                })
                .cloned()
                .collect::<Vec<_>>();

            let added_projects = envelope
                .payload
                .projects
                .into_iter()
                .filter(|project| !this.projects.contains_key(&RemoteProjectId(project.id)))
                .collect::<Vec<_>>();

            (added_projects, removed_projects)
        })?;

        for remote_project in added_projects {
            DevServer::share_project(this.clone(), &remote_project, &mut cx).await?;
        }

        this.update(&mut cx, |this, cx| {
            for old_project_id in &removed_projects_ids {
                this.unshare_project(old_project_id, cx)?;
            }
            Ok::<(), anyhow::Error>(())
        })??;
        Ok(())
    }

    fn unshare_project(
        &mut self,
        remote_project_id: &RemoteProjectId,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if let Some(project) = self.projects.remove(remote_project_id) {
            project.update(cx, |project, cx| project.unshare(cx))?;
        }
        Ok(())
    }

    async fn share_project(
        this: Model<Self>,
        remote_project: &proto::RemoteProject,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        let (client, project) = this.update(cx, |this, cx| {
            let project = Project::local(
                this.client.clone(),
                this.app_state.node_runtime.clone(),
                this.app_state.user_store.clone(),
                this.app_state.languages.clone(),
                this.app_state.fs.clone(),
                cx,
            );

            (this.client.clone(), project)
        })?;

        project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree(&remote_project.path, true, cx)
            })?
            .await?;

        let worktrees =
            project.read_with(cx, |project, cx| project.worktree_metadata_protos(cx))?;

        let response = client
            .request(proto::ShareRemoteProject {
                remote_project_id: remote_project.id,
                worktrees,
            })
            .await?;

        let project_id = response.project_id;
        project.update(cx, |project, cx| project.shared(project_id, cx))??;
        this.update(cx, |this, _| {
            this.projects
                .insert(RemoteProjectId(remote_project.id), project);
        })?;
        Ok(())
    }
}
