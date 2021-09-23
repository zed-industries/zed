use super::worktree::Worktree;
use anyhow::Result;
use gpui::{Entity, ModelContext, ModelHandle, Task};

pub struct Project {
    worktrees: Vec<ModelHandle<Worktree>>,
}

pub enum Event {}

impl Project {
    pub fn new() -> Self {
        Self {
            worktrees: Default::default(),
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

    pub fn add_worktree(&mut self, worktree: ModelHandle<Worktree>) {
        self.worktrees.push(worktree);
    }

    pub fn share_worktree(
        &self,
        remote_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Option<Task<Result<u64>>> {
        for worktree in &self.worktrees {
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
