use anyhow::Result;
use gpui::{AppContext, AsyncAppContext, Context, Global, Model, ModelContext, SharedString, Task};
use rpc::{
    proto::{self, DevServerStatus},
    TypedEnvelope,
};
use std::{collections::HashMap, sync::Arc};

use client::{Client, ProjectId};
pub use client::{DevServerId, DevServerProjectId};

pub struct Store {
    dev_server_projects: HashMap<DevServerProjectId, DevServerProject>,
    dev_servers: HashMap<DevServerId, DevServer>,
    _subscriptions: Vec<client::Subscription>,
    client: Arc<Client>,
}

#[derive(Debug, Clone)]
pub struct DevServerProject {
    pub id: DevServerProjectId,
    pub project_id: Option<ProjectId>,
    pub path: SharedString,
    pub dev_server_id: DevServerId,
}

impl From<proto::DevServerProject> for DevServerProject {
    fn from(project: proto::DevServerProject) -> Self {
        Self {
            id: DevServerProjectId(project.id),
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
    pub ssh_connection_string: Option<SharedString>,
    pub status: DevServerStatus,
}

impl From<proto::DevServer> for DevServer {
    fn from(dev_server: proto::DevServer) -> Self {
        Self {
            id: DevServerId(dev_server.dev_server_id),
            status: dev_server.status(),
            name: dev_server.name.into(),
            ssh_connection_string: dev_server.ssh_connection_string.map(|s| s.into()),
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
            dev_server_projects: Default::default(),
            dev_servers: Default::default(),
            _subscriptions: vec![client
                .add_message_handler(cx.weak_model(), Self::handle_dev_server_projects_update)],
            client,
        }
    }

    pub fn projects_for_server(&self, id: DevServerId) -> Vec<DevServerProject> {
        let mut projects: Vec<DevServerProject> = self
            .dev_server_projects
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

    pub fn dev_server_projects(&self) -> Vec<DevServerProject> {
        let mut projects: Vec<DevServerProject> =
            self.dev_server_projects.values().cloned().collect();
        projects.sort_by_key(|p| (p.path.clone(), p.id));
        projects
    }

    pub fn dev_server_project(&self, id: DevServerProjectId) -> Option<&DevServerProject> {
        self.dev_server_projects.get(&id)
    }

    pub fn dev_server_for_project(&self, id: DevServerProjectId) -> Option<&DevServer> {
        self.dev_server_project(id)
            .and_then(|project| self.dev_server(project.dev_server_id))
    }

    async fn handle_dev_server_projects_update(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::DevServerProjectsUpdate>,
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
            this.dev_server_projects = envelope
                .payload
                .dev_server_projects
                .into_iter()
                .map(|project| (DevServerProjectId(project.id), project.into()))
                .collect();

            cx.notify();
        })?;
        Ok(())
    }

    pub fn create_dev_server_project(
        &mut self,
        dev_server_id: DevServerId,
        path: String,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<proto::CreateDevServerProjectResponse>> {
        let client = self.client.clone();
        cx.background_executor().spawn(async move {
            client
                .request(proto::CreateDevServerProject {
                    dev_server_id: dev_server_id.0,
                    path,
                })
                .await
        })
    }

    pub fn create_dev_server(
        &mut self,
        name: String,
        ssh_connection_string: Option<String>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<proto::CreateDevServerResponse>> {
        let client = self.client.clone();
        cx.background_executor().spawn(async move {
            let result = client
                .request(proto::CreateDevServer {
                    name,
                    ssh_connection_string,
                })
                .await?;
            Ok(result)
        })
    }

    pub fn rename_dev_server(
        &mut self,
        dev_server_id: DevServerId,
        name: String,
        ssh_connection_string: Option<String>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.background_executor().spawn(async move {
            client
                .request(proto::RenameDevServer {
                    dev_server_id: dev_server_id.0,
                    name,
                    ssh_connection_string,
                })
                .await?;
            Ok(())
        })
    }

    pub fn regenerate_dev_server_token(
        &mut self,
        dev_server_id: DevServerId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<proto::RegenerateDevServerTokenResponse>> {
        let client = self.client.clone();
        cx.background_executor().spawn(async move {
            client
                .request(proto::RegenerateDevServerToken {
                    dev_server_id: dev_server_id.0,
                })
                .await
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

    pub fn delete_dev_server_project(
        &mut self,
        id: DevServerProjectId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.background_executor().spawn(async move {
            client
                .request(proto::DeleteDevServerProject {
                    dev_server_project_id: id.0,
                })
                .await?;
            Ok(())
        })
    }
}
