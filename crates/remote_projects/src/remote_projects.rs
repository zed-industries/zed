use anyhow::Result;
use gpui::{AppContext, AsyncAppContext, Context, Global, Model, ModelContext, SharedString, Task};
use rpc::{
    proto::{self, DevServerStatus},
    TypedEnvelope,
};
use std::{collections::HashMap, sync::Arc};

use client::{Client, ProjectId};
pub use client::{DevServerId, RemoteProjectId};

pub struct Store {
    remote_projects: HashMap<RemoteProjectId, RemoteProject>,
    dev_servers: HashMap<DevServerId, DevServer>,
    _subscriptions: Vec<client::Subscription>,
    client: Arc<Client>,
}

#[derive(Debug, Clone)]
pub struct RemoteProject {
    pub id: RemoteProjectId,
    pub project_id: Option<ProjectId>,
    pub path: SharedString,
    pub dev_server_id: DevServerId,
}

impl From<proto::RemoteProject> for RemoteProject {
    fn from(project: proto::RemoteProject) -> Self {
        Self {
            id: RemoteProjectId(project.id),
            project_id: project.project_id.map(|id| ProjectId(id)),
            path: project.path.into(),
            dev_server_id: DevServerId(project.dev_server_id),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DevServer {
    pub id: DevServerId,
    pub name: SharedString,
    pub status: DevServerStatus,
}

impl From<proto::DevServer> for DevServer {
    fn from(dev_server: proto::DevServer) -> Self {
        Self {
            id: DevServerId(dev_server.dev_server_id),
            status: dev_server.status(),
            name: dev_server.name.into(),
        }
    }
}

struct GlobalStore(Model<Store>);

impl Global for GlobalStore {}

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let store = cx.new_model(|cx| Store::new(client, cx));
    cx.set_global(GlobalStore(store));
}

impl Store {
    pub fn global(cx: &AppContext) -> Model<Store> {
        cx.global::<GlobalStore>().0.clone()
    }

    pub fn new(client: Arc<Client>, cx: &ModelContext<Self>) -> Self {
        Self {
            remote_projects: Default::default(),
            dev_servers: Default::default(),
            _subscriptions: vec![
                client.add_message_handler(cx.weak_model(), Self::handle_remote_projects_update)
            ],
            client,
        }
    }

    pub fn remote_projects_for_server(&self, id: DevServerId) -> Vec<RemoteProject> {
        let mut projects: Vec<RemoteProject> = self
            .remote_projects
            .values()
            .filter(|project| project.dev_server_id == id)
            .cloned()
            .collect();
        projects.sort_by_key(|p| (p.path.clone(), p.id));
        projects
    }

    pub fn dev_servers(&self) -> Vec<DevServer> {
        let mut dev_servers: Vec<DevServer> = self.dev_servers.values().cloned().collect();
        dev_servers.sort_by_key(|d| (d.status == DevServerStatus::Offline, d.name.clone(), d.id));
        dev_servers
    }

    pub fn dev_server(&self, id: DevServerId) -> Option<&DevServer> {
        self.dev_servers.get(&id)
    }

    pub fn dev_server_status(&self, id: DevServerId) -> DevServerStatus {
        self.dev_server(id)
            .map(|server| server.status)
            .unwrap_or(DevServerStatus::Offline)
    }

    pub fn remote_projects(&self) -> Vec<RemoteProject> {
        let mut projects: Vec<RemoteProject> = self.remote_projects.values().cloned().collect();
        projects.sort_by_key(|p| (p.path.clone(), p.id));
        projects
    }

    pub fn remote_project(&self, id: RemoteProjectId) -> Option<&RemoteProject> {
        self.remote_projects.get(&id)
    }

    pub fn dev_server_for_project(&self, id: RemoteProjectId) -> Option<&DevServer> {
        self.remote_project(id)
            .and_then(|project| self.dev_server(project.dev_server_id))
    }

    async fn handle_remote_projects_update(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::RemoteProjectsUpdate>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.dev_servers = envelope
                .payload
                .dev_servers
                .into_iter()
                .map(|dev_server| (DevServerId(dev_server.dev_server_id), dev_server.into()))
                .collect();
            this.remote_projects = envelope
                .payload
                .remote_projects
                .into_iter()
                .map(|project| (RemoteProjectId(project.id), project.into()))
                .collect();

            cx.notify();
        })?;
        Ok(())
    }

    pub fn create_remote_project(
        &mut self,
        dev_server_id: DevServerId,
        path: String,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<proto::CreateRemoteProjectResponse>> {
        let client = self.client.clone();
        cx.background_executor().spawn(async move {
            client
                .request(proto::CreateRemoteProject {
                    dev_server_id: dev_server_id.0,
                    path,
                })
                .await
        })
    }

    pub fn create_dev_server(
        &mut self,
        name: String,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<proto::CreateDevServerResponse>> {
        let client = self.client.clone();
        cx.background_executor().spawn(async move {
            let result = client.request(proto::CreateDevServer { name }).await?;
            Ok(result)
        })
    }

    pub fn delete_dev_server(
        &mut self,
        id: DevServerId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.background_executor().spawn(async move {
            client
                .request(proto::DeleteDevServer {
                    dev_server_id: id.0,
                })
                .await?;
            Ok(())
        })
    }
}
