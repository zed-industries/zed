mod dev_server_modal;

use anyhow::Result;
use gpui::{AppContext, Global, Model, ModelContext, Task};
use rpc::{
    proto::{self, DevServerStatus},
    TypedEnvelope,
};
use std::{collections::HashMap, sync::Arc};

use client::{Client, ProjectId};
use ui::{Context, SharedString};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct DevServerId(pub u64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct RemoteProjectId(pub u64);

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
    pub name: SharedString,
    pub path: SharedString,
    pub dev_server_id: DevServerId,
}

impl From<proto::RemoteProject> for RemoteProject {
    fn from(project: proto::RemoteProject) -> Self {
        Self {
            id: RemoteProjectId(project.id),
            project_id: project.project_id.map(|id| ProjectId(id)),
            name: project.name.into(),
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

pub fn init(client: Arc<Client>, cx: &AppContext) {
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

    fn handle_remote_projects_update(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::RemoteProjectsUpdate>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(cx, |this| {
            this.dev_servers = envelope
                .payload
                .dev_servers
                .into_iter()
                .map(|dev_server| (dev_server.id, dev_server.into()))
                .collect();
            this.remote_projects = envelope
                .payload
                .remote_projects
                .into_iter()
                .map(|project| (project.id, project.into()))
                .collect();
        })
    }

    pub fn create_remote_project(
        &mut self,
        dev_server_id: DevServerId,
        name: String,
        path: String,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<proto::CreateRemoteProjectResponse>> {
        let client = self.client.clone();
        cx.background_executor().spawn(async move {
            client
                .request(proto::CreateRemoteProject {
                    dev_server_id: dev_server_id.0,
                    name,
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
}
