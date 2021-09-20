use crate::{
    rpc::Client,
    user::{User, UserStore},
    util::TryFutureExt,
};
use anyhow::Result;
use gpui::{Entity, ModelContext, Task};
use postage::prelude::Stream;
use smol::future::FutureExt;
use std::{collections::HashSet, sync::Arc, time::Duration};
use zrpc::proto;

pub struct Presence {
    collaborators: Vec<Collaborator>,
    user_store: Arc<UserStore>,
    rpc: Arc<Client>,
    _maintain_people: Task<()>,
}

#[derive(Debug)]
struct Collaborator {
    user: Arc<User>,
    worktrees: Vec<WorktreeMetadata>,
}

#[derive(Debug)]
struct WorktreeMetadata {
    root_name: String,
    is_shared: bool,
    participants: Vec<Arc<User>>,
}

impl Presence {
    pub fn new(user_store: Arc<UserStore>, rpc: Arc<Client>, cx: &mut ModelContext<Self>) -> Self {
        let _maintain_collaborators = cx.spawn_weak(|this, mut cx| {
            let user_store = user_store.clone();
            let foreground = cx.foreground();
            async move {
                let mut current_user = user_store.watch_current_user();
                loop {
                    let timer = foreground.timer(Duration::from_secs(2));
                    let next_current_user = async {
                        current_user.recv().await;
                    };

                    next_current_user.race(timer).await;
                    if current_user.borrow().is_some() {
                        if let Some(this) = cx.read(|cx| this.upgrade(cx)) {
                            this.update(&mut cx, |this, cx| this.refresh(cx))
                                .log_err()
                                .await;
                        }
                    }
                }
            }
        });

        Self {
            collaborators: Vec::new(),
            user_store,
            rpc,
            _maintain_people: _maintain_collaborators,
        }
    }

    fn refresh(&self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        cx.spawn(|this, mut cx| {
            let rpc = self.rpc.clone();
            let user_store = self.user_store.clone();
            async move {
                //     let response = rpc.request(proto::GetCollaborators {}).await?;
                //     let mut user_ids = HashSet::new();
                //     for collaborator in &response.collaborators {
                //         user_ids.insert(collaborator.user_id);
                //         user_ids.extend(
                //             collaborator
                //                 .worktrees
                //                 .iter()
                //                 .flat_map(|w| &w.participants)
                //                 .copied(),
                //         );
                //     }
                //     user_store
                //         .load_users(user_ids.into_iter().collect())
                //         .await?;

                //     let mut collaborators = Vec::new();
                //     for collaborator in response.collaborators {
                //         collaborators.push(Collaborator::from_proto(collaborator, &user_store).await?);
                //     }

                //     this.update(&mut cx, |this, cx| {
                //         this.collaborators = collaborators;
                //         cx.notify();
                //     });

                Ok(())
            }
        })
    }
}

pub enum Event {}

impl Entity for Presence {
    type Event = Event;
}

impl Collaborator {
    async fn from_proto(
        collaborator: proto::Collaborator,
        user_store: &Arc<UserStore>,
    ) -> Result<Self> {
        let user = user_store.fetch_user(collaborator.user_id).await?;
        let mut worktrees = Vec::new();
        for worktree in collaborator.worktrees {
            let mut participants = Vec::new();
            for participant_id in worktree.participants {
                participants.push(user_store.fetch_user(participant_id).await?);
            }
            worktrees.push(WorktreeMetadata {
                root_name: worktree.root_name,
                is_shared: worktree.is_shared,
                participants,
            });
        }
        Ok(Self { user, worktrees })
    }
}
