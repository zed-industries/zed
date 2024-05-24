use anyhow::{anyhow, Result};
use client::DevServerProjectId;
use client::{user::UserStore, Client, ClientSettings};
use fs::Fs;
use futures::Future;
use gpui::{AppContext, AsyncAppContext, Context, Global, Model, ModelContext, Task, WeakModel};
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use postage::stream::Stream;
use project::Project;
use rpc::{proto, ErrorCode, TypedEnvelope};
use settings::Settings;
use std::{collections::HashMap, sync::Arc};
use util::{ResultExt, TryFutureExt};

pub struct DevServer {
    client: Arc<Client>,
    app_state: AppState,
    remote_shutdown: bool,
    projects: HashMap<DevServerProjectId, Model<Project>>,
    _subscriptions: Vec<client::Subscription>,
    _maintain_connection: Task<Option<()>>,
}

pub struct AppState {
    pub node_runtime: Arc<dyn NodeRuntime>,
    pub user_store: Model<UserStore>,
    pub languages: Arc<LanguageRegistry>,
    pub fs: Arc<dyn Fs>,
}

struct GlobalDevServer(Model<DevServer>);

impl Global for GlobalDevServer {}

pub fn init(client: Arc<Client>, app_state: AppState, cx: &mut AppContext) -> Task<Result<()>> {
    let dev_server = cx.new_model(|cx| DevServer::new(client.clone(), app_state, cx));
    cx.set_global(GlobalDevServer(dev_server.clone()));

    #[cfg(not(target_os = "windows"))]
    {
        use signal_hook::consts::{SIGINT, SIGTERM};
        use signal_hook::iterator::Signals;
        // Set up a handler when the dev server is shut down
        // with ctrl-c or kill
        let (tx, rx) = futures::channel::oneshot::channel();
        let mut signals = Signals::new(&[SIGTERM, SIGINT]).unwrap();
        std::thread::spawn({
            move || {
                if let Some(sig) = signals.forever().next() {
                    tx.send(sig).log_err();
                }
            }
        });
        cx.spawn(|cx| async move {
            if let Ok(sig) = rx.await {
                log::info!("received signal {sig:?}");
                cx.update(|cx| cx.quit()).log_err();
            }
        })
        .detach();
    }

    let server_url = ClientSettings::get_global(&cx).server_url.clone();
    cx.spawn(|cx| async move {
        client
            .authenticate_and_connect(false, &cx)
            .await
            .map_err(|e| anyhow!("Error connecting to '{}': {}", server_url, e))
    })
}

impl DevServer {
    pub fn global(cx: &AppContext) -> Model<DevServer> {
        cx.global::<GlobalDevServer>().0.clone()
    }

    pub fn new(client: Arc<Client>, app_state: AppState, cx: &mut ModelContext<Self>) -> Self {
        cx.on_app_quit(Self::app_will_quit).detach();

        let maintain_connection = cx.spawn({
            let client = client.clone();
            move |this, cx| Self::maintain_connection(this, client.clone(), cx).log_err()
        });

        DevServer {
            _subscriptions: vec![
                client.add_message_handler(cx.weak_model(), Self::handle_dev_server_instructions),
                client.add_request_handler(
                    cx.weak_model(),
                    Self::handle_validate_dev_server_project_request,
                ),
                client.add_message_handler(cx.weak_model(), Self::handle_shutdown),
            ],
            _maintain_connection: maintain_connection,
            projects: Default::default(),
            remote_shutdown: false,
            app_state,
            client,
        }
    }

    fn app_will_quit(&mut self, _: &mut ModelContext<Self>) -> impl Future<Output = ()> {
        let request = if self.remote_shutdown {
            None
        } else {
            Some(
                self.client
                    .request(proto::ShutdownDevServer { reason: None }),
            )
        };
        async move {
            if let Some(request) = request {
                request.await.log_err();
            }
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
                .filter(|dev_server_project_id| {
                    !envelope
                        .payload
                        .projects
                        .iter()
                        .any(|p| p.id == dev_server_project_id.0)
                })
                .cloned()
                .collect::<Vec<_>>();

            let added_projects = envelope
                .payload
                .projects
                .into_iter()
                .filter(|project| !this.projects.contains_key(&DevServerProjectId(project.id)))
                .collect::<Vec<_>>();

            (added_projects, removed_projects)
        })?;

        for dev_server_project in added_projects {
            DevServer::share_project(this.clone(), &dev_server_project, &mut cx).await?;
        }

        this.update(&mut cx, |this, cx| {
            for old_project_id in &removed_projects_ids {
                this.unshare_project(old_project_id, cx)?;
            }
            Ok::<(), anyhow::Error>(())
        })??;
        Ok(())
    }

    async fn handle_validate_dev_server_project_request(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ValidateDevServerProjectRequest>,
        _: Arc<Client>,
        cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        let expanded = shellexpand::tilde(&envelope.payload.path).to_string();
        let path = std::path::Path::new(&expanded);
        let fs = cx.read_model(&this, |this, _| this.app_state.fs.clone())?;

        let path_exists = fs.metadata(path).await.is_ok_and(|result| result.is_some());
        if !path_exists {
            return Err(anyhow!(ErrorCode::DevServerProjectPathDoesNotExist))?;
        }

        Ok(proto::Ack {})
    }

    async fn handle_shutdown(
        this: Model<Self>,
        _envelope: TypedEnvelope<proto::ShutdownDevServer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.remote_shutdown = true;
            cx.quit();
        })
    }

    fn unshare_project(
        &mut self,
        dev_server_project_id: &DevServerProjectId,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if let Some(project) = self.projects.remove(dev_server_project_id) {
            project.update(cx, |project, cx| project.unshare(cx))?;
        }
        Ok(())
    }

    async fn share_project(
        this: Model<Self>,
        dev_server_project: &proto::DevServerProject,
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

        let path = shellexpand::tilde(&dev_server_project.path).to_string();

        let (worktree, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree(&path, true, cx)
            })?
            .await?;

        worktree.update(cx, |worktree, cx| {
            worktree.as_local_mut().unwrap().share_private_files(cx)
        })?;

        let worktrees =
            project.read_with(cx, |project, cx| project.worktree_metadata_protos(cx))?;

        let response = client
            .request(proto::ShareDevServerProject {
                dev_server_project_id: dev_server_project.id,
                worktrees,
            })
            .await?;

        let project_id = response.project_id;
        project.update(cx, |project, cx| project.shared(project_id, cx))??;
        this.update(cx, |this, _| {
            this.projects
                .insert(DevServerProjectId(dev_server_project.id), project);
        })?;
        Ok(())
    }

    async fn maintain_connection(
        this: WeakModel<Self>,
        client: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let mut client_status = client.status();

        let _ = client_status.try_recv();
        let current_status = *client_status.borrow();
        if current_status.is_connected() {
            // wait for first disconnect
            client_status.recv().await;
        }

        loop {
            let Some(current_status) = client_status.recv().await else {
                return Ok(());
            };
            let Some(this) = this.upgrade() else {
                return Ok(());
            };

            if !current_status.is_connected() {
                continue;
            }

            this.update(&mut cx, |this, cx| this.rejoin(cx))?.await?;
        }
    }

    fn rejoin(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let mut projects: HashMap<u64, Model<Project>> = HashMap::default();
        let request = self.client.request(proto::ReconnectDevServer {
            reshared_projects: self
                .projects
                .iter()
                .flat_map(|(_, handle)| {
                    let project = handle.read(cx);
                    let project_id = project.remote_id()?;
                    projects.insert(project_id, handle.clone());
                    Some(proto::UpdateProject {
                        project_id,
                        worktrees: project.worktree_metadata_protos(cx),
                    })
                })
                .collect(),
        });
        cx.spawn(|_, mut cx| async move {
            let response = request.await?;

            for reshared_project in response.reshared_projects {
                if let Some(project) = projects.get(&reshared_project.id) {
                    project.update(&mut cx, |project, cx| {
                        project.reshared(reshared_project, cx).log_err();
                    })?;
                }
            }
            Ok(())
        })
    }
}
