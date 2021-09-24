use crate::{
    fs::Fs,
    language::LanguageRegistry,
    rpc::Client,
    util::TryFutureExt as _,
    worktree::{self, Worktree},
    AppState,
};
use anyhow::Result;
use gpui::{Entity, ModelContext, ModelHandle, Task};
use std::{path::Path, sync::Arc};

pub struct Project {
    worktrees: Vec<ModelHandle<Worktree>>,
    languages: Arc<LanguageRegistry>,
    rpc: Arc<Client>,
    fs: Arc<dyn Fs>,
}

pub enum Event {}

impl Project {
    pub fn new(app_state: &AppState) -> Self {
        Self {
            worktrees: Default::default(),
            languages: app_state.languages.clone(),
            rpc: app_state.rpc.clone(),
            fs: app_state.fs.clone(),
        }
    }

    pub fn worktrees(&self) -> &[ModelHandle<Worktree>] {
        &self.worktrees
    }

    pub fn worktree_for_id(&self, id: usize) -> Option<ModelHandle<Worktree>> {
        self.worktrees
            .iter()
            .find(|worktree| worktree.id() == id)
            .cloned()
    }

    pub fn add_local_worktree(
        &mut self,
        path: &Path,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Worktree>>> {
        let fs = self.fs.clone();
        let rpc = self.rpc.clone();
        let languages = self.languages.clone();
        let path = Arc::from(path);
        cx.spawn(|this, mut cx| async move {
            let worktree = Worktree::open_local(rpc, path, fs, languages, &mut cx).await?;
            this.update(&mut cx, |this, cx| {
                this.add_worktree(worktree.clone(), cx);
            });
            Ok(worktree)
        })
    }

    pub fn add_remote_worktree(
        &mut self,
        remote_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Worktree>>> {
        let rpc = self.rpc.clone();
        let languages = self.languages.clone();
        cx.spawn(|this, mut cx| async move {
            rpc.authenticate_and_connect(&cx).await?;
            let worktree =
                Worktree::open_remote(rpc.clone(), remote_id, languages, &mut cx).await?;
            this.update(&mut cx, |this, cx| {
                cx.subscribe(&worktree, move |this, _, event, cx| match event {
                    worktree::Event::Closed => {
                        this.close_remote_worktree(remote_id, cx);
                        cx.notify();
                    }
                })
                .detach();
                this.add_worktree(worktree.clone(), cx);
            });
            Ok(worktree)
        })
    }

    fn add_worktree(&mut self, worktree: ModelHandle<Worktree>, cx: &mut ModelContext<Self>) {
        cx.observe(&worktree, |_, _, cx| cx.notify()).detach();
        self.worktrees.push(worktree);
        cx.notify();
    }

    pub fn share_worktree(&self, remote_id: u64, cx: &mut ModelContext<Self>) {
        let rpc = self.rpc.clone();
        cx.spawn(|this, mut cx| {
            async move {
                rpc.authenticate_and_connect(&cx).await?;

                let task = this.update(&mut cx, |this, cx| {
                    for worktree in &this.worktrees {
                        let task = worktree.update(cx, |worktree, cx| {
                            worktree.as_local_mut().and_then(|worktree| {
                                if worktree.remote_id() == Some(remote_id) {
                                    Some(worktree.share(cx))
                                } else {
                                    None
                                }
                            })
                        });
                        if task.is_some() {
                            return task;
                        }
                    }
                    None
                });

                if let Some(task) = task {
                    task.await?;
                }

                Ok(())
            }
            .log_err()
        })
        .detach();
    }

    pub fn unshare_worktree(&mut self, remote_id: u64, cx: &mut ModelContext<Self>) {
        for worktree in &self.worktrees {
            if worktree.update(cx, |worktree, cx| {
                if let Some(worktree) = worktree.as_local_mut() {
                    if worktree.remote_id() == Some(remote_id) {
                        worktree.unshare(cx);
                        return true;
                    }
                }
                false
            }) {
                break;
            }
        }
    }

    pub fn close_remote_worktree(&mut self, id: u64, cx: &mut ModelContext<Self>) {
        self.worktrees.retain(|worktree| {
            worktree.update(cx, |worktree, cx| {
                if let Some(worktree) = worktree.as_remote_mut() {
                    if worktree.remote_id() == id {
                        worktree.close_all_buffers(cx);
                        return false;
                    }
                }
                true
            })
        });
    }
}

impl Entity for Project {
    type Event = Event;
}
