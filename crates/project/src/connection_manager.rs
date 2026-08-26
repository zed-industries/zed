use super::Project;
use anyhow::Result;
use client::Client;
use collections::{HashMap, HashSet};
use futures::{FutureExt, StreamExt};
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, Global, Task, WeakEntity};
use postage::stream::Stream;
use rpc::proto;
use std::{sync::Arc, time::Duration};
use util::ResultExt;

impl Global for GlobalManager {}
struct GlobalManager(Entity<Manager>);

pub const RECONNECT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct Manager {
    client: Arc<Client>,
    maintain_connection: Option<Task<Option<()>>>,
    projects: HashSet<WeakEntity<Project>>,
}

pub fn init(client: Arc<Client>, cx: &mut App) {
    let manager = cx.new(|_| Manager {
        client,
        maintain_connection: None,
        projects: HashSet::default(),
    });
    cx.set_global(GlobalManager(manager));
}

impl Manager {
    pub fn global(cx: &App) -> Entity<Manager> {
        cx.global::<GlobalManager>().0.clone()
    }

    pub fn maintain_project_connection(
        &mut self,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) {
        let manager = cx.weak_entity();
        project.update(cx, |_, cx| {
            let manager = manager.clone();
            cx.on_release(move |project, cx| {
                manager
                    .update(cx, |manager, cx| {
                        manager.projects.retain(|p| {
                            if let Some(p) = p.upgrade() {
                                p.read(cx).remote_id() != project.remote_id()
                            } else {
                                false
                            }
                        });
                        if manager.projects.is_empty() {
                            manager.maintain_connection.take();
                        }
                    })
                    .ok();
            })
            .detach();
        });

        self.projects.insert(project.downgrade());
        if self.maintain_connection.is_none() {
            self.maintain_connection = Some(cx.spawn({
                let client = self.client.clone();
                async move |_, cx| {
                    Self::maintain_connection(manager, client.clone(), cx)
                        .await
                        .log_err()
                }
            }));
        }
    }

    fn reconnected(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let mut projects = HashMap::default();

        let request = self.client.request_envelope(proto::RejoinRemoteProjects {
            rejoined_projects: self
                .projects
                .iter()
                .filter_map(|project| {
                    if let Some(handle) = project.upgrade() {
                        let project = handle.read(cx);
                        let project_id = project.remote_id()?;
                        projects.insert(project_id, handle.clone());
                        let mut worktrees = Vec::new();
                        let mut repositories = Vec::new();
                        for (id, repository) in project.repositories(cx) {
                            repositories.push(proto::RejoinRepository {
                                id: id.to_proto(),
                                scan_id: repository.read(cx).scan_id,
                            });
                        }
                        for worktree in project.worktrees(cx) {
                            let worktree = worktree.read(cx);
                            worktrees.push(proto::RejoinWorktree {
                                id: worktree.id().to_proto(),
                                scan_id: worktree.completed_scan_id() as u64,
                            });
                        }
                        Some(proto::RejoinProject {
                            id: project_id,
                            worktrees,
                            repositories,
                        })
                    } else {
                        None
                    }
                })
                .collect(),
        });

        cx.spawn(async move |this, cx| {
            let response = request.await?;
            let message_id = response.message_id;

            this.update(cx, |_, cx| {
                for rejoined_project in response.payload.rejoined_projects {
                    if let Some(project) = projects.get(&rejoined_project.id) {
                        project.update(cx, |project, cx| {
                            project.rejoined(rejoined_project, message_id, cx).log_err();
                        });
                    }
                }
            })
        })
    }

    fn connection_lost(&mut self, cx: &mut Context<Self>) {
        for project in self.projects.drain() {
            if let Some(project) = project.upgrade() {
                project.update(cx, |project, cx| {
                    project.disconnected_from_host(cx);
                    project.close(cx);
                });
            }
        }
        self.maintain_connection.take();
    }

    async fn maintain_connection(
        this: WeakEntity<Self>,
        client: Arc<Client>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let mut client_status = client.status();
        loop {
            let _ = client_status.try_recv();

            let is_connected = client_status.borrow().is_connected();
            // Even if we're initially connected, any future change of the status means we momentarily disconnected.
            if !is_connected || client_status.next().await.is_some() {
                log::info!("detected client disconnection");

                // Wait for client to re-establish a connection to the server.
                {
                    let mut reconnection_timeout =
                        cx.background_executor().timer(RECONNECT_TIMEOUT).fuse();
                    let client_reconnection = async {
                        let mut remaining_attempts = 3;
                        while remaining_attempts > 0 {
                            if client_status.borrow().is_connected() {
                                log::info!("client reconnected, attempting to rejoin projects");

                                let Some(this) = this.upgrade() else { break };
                                let task = this.update(cx, |this, cx| this.reconnected(cx));
                                if task.await.log_err().is_some() {
                                    return true;
                                } else {
                                    remaining_attempts -= 1;
                                }
                            } else if client_status.borrow().is_signed_out() {
                                return false;
                            }

                            log::info!(
                                "waiting for client status change, remaining attempts {}",
                                remaining_attempts
                            );
                            client_status.next().await;
                        }
                        false
                    }
                    .fuse();
                    futures::pin_mut!(client_reconnection);

                    futures::select_biased! {
                        reconnected = client_reconnection => {
                            if reconnected {
                                log::info!("successfully reconnected");
                                // If we successfully joined the room, go back around the loop
                                // waiting for future connection status changes.
                                continue;
                            }
                        }
                        _ = reconnection_timeout => {
                            log::info!("rejoin project reconnection timeout expired");
                        }
                    }
                }

                break;
            }
        }

        // The client failed to re-establish a connection to the server
        // or an error occurred while trying to re-join the room. Either way
        // we leave the room and return an error.
        if let Some(this) = this.upgrade() {
            log::info!("reconnection failed, disconnecting projects");
            this.update(cx, |this, cx| this.connection_lost(cx));
        }

        Ok(())
    }
}
