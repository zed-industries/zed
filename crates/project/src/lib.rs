pub mod fs;
mod ignore;
mod worktree;

use anyhow::Result;
use client::Client;
use futures::Future;
use fuzzy::{PathMatch, PathMatchCandidate, PathMatchCandidateSet};
use gpui::{AppContext, Entity, ModelContext, ModelHandle, Task};
use language::LanguageRegistry;
use std::{
    path::Path,
    sync::{atomic::AtomicBool, Arc},
};
use util::{ResultExt, TryFutureExt as _};

pub use fs::*;
pub use worktree::*;

pub struct Project {
    worktrees: Vec<ModelHandle<Worktree>>,
    active_entry: Option<ProjectEntry>,
    languages: Arc<LanguageRegistry>,
    client: Arc<client::Client>,
    fs: Arc<dyn Fs>,
}

pub enum Event {
    ActiveEntryChanged(Option<ProjectEntry>),
    WorktreeRemoved(usize),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ProjectPath {
    pub worktree_id: usize,
    pub path: Arc<Path>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProjectEntry {
    pub worktree_id: usize,
    pub entry_id: usize,
}

impl Project {
    pub fn new(languages: Arc<LanguageRegistry>, rpc: Arc<Client>, fs: Arc<dyn Fs>) -> Self {
        Self {
            worktrees: Default::default(),
            active_entry: None,
            languages,
            client: rpc,
            fs,
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
        abs_path: &Path,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Worktree>>> {
        let fs = self.fs.clone();
        let rpc = self.client.clone();
        let languages = self.languages.clone();
        let path = Arc::from(abs_path);
        let language_server = languages
            .get_language("Rust")
            .map(|language| language.start_server(&path, cx));
        cx.spawn(|this, mut cx| async move {
            let language_server = language_server.and_then(|language| language.log_err().flatten());
            let worktree =
                Worktree::open_local(rpc, path, fs, languages, language_server, &mut cx).await?;
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
        let rpc = self.client.clone();
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

    pub fn set_active_path(&mut self, entry: Option<ProjectPath>, cx: &mut ModelContext<Self>) {
        let new_active_entry = entry.and_then(|project_path| {
            let worktree = self.worktree_for_id(project_path.worktree_id)?;
            let entry = worktree.read(cx).entry_for_path(project_path.path)?;
            Some(ProjectEntry {
                worktree_id: project_path.worktree_id,
                entry_id: entry.id,
            })
        });
        if new_active_entry != self.active_entry {
            self.active_entry = new_active_entry;
            cx.emit(Event::ActiveEntryChanged(new_active_entry));
        }
    }

    pub fn active_entry(&self) -> Option<ProjectEntry> {
        self.active_entry
    }

    pub fn share_worktree(&self, remote_id: u64, cx: &mut ModelContext<Self>) {
        let rpc = self.client.clone();
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
            let keep = worktree.update(cx, |worktree, cx| {
                if let Some(worktree) = worktree.as_remote_mut() {
                    if worktree.remote_id() == id {
                        worktree.close_all_buffers(cx);
                        return false;
                    }
                }
                true
            });
            if !keep {
                cx.emit(Event::WorktreeRemoved(worktree.id()));
            }
            keep
        });
    }

    pub fn match_paths<'a>(
        &self,
        query: &'a str,
        include_ignored: bool,
        smart_case: bool,
        max_results: usize,
        cancel_flag: &'a AtomicBool,
        cx: &AppContext,
    ) -> impl 'a + Future<Output = Vec<PathMatch>> {
        let include_root_name = self.worktrees.len() > 1;
        let candidate_sets = self
            .worktrees
            .iter()
            .map(|worktree| CandidateSet {
                snapshot: worktree.read(cx).snapshot(),
                include_ignored,
                include_root_name,
            })
            .collect::<Vec<_>>();

        let background = cx.background().clone();
        async move {
            fuzzy::match_paths(
                candidate_sets.as_slice(),
                query,
                smart_case,
                max_results,
                cancel_flag,
                background,
            )
            .await
        }
    }
}

struct CandidateSet {
    snapshot: Snapshot,
    include_ignored: bool,
    include_root_name: bool,
}

impl<'a> PathMatchCandidateSet<'a> for CandidateSet {
    type Candidates = CandidateSetIter<'a>;

    fn id(&self) -> usize {
        self.snapshot.id()
    }

    fn len(&self) -> usize {
        if self.include_ignored {
            self.snapshot.file_count()
        } else {
            self.snapshot.visible_file_count()
        }
    }

    fn prefix(&self) -> Arc<str> {
        if self.snapshot.root_entry().map_or(false, |e| e.is_file()) {
            self.snapshot.root_name().into()
        } else if self.include_root_name {
            format!("{}/", self.snapshot.root_name()).into()
        } else {
            "".into()
        }
    }

    fn candidates(&'a self, start: usize) -> Self::Candidates {
        CandidateSetIter {
            traversal: self.snapshot.files(self.include_ignored, start),
        }
    }
}

struct CandidateSetIter<'a> {
    traversal: Traversal<'a>,
}

impl<'a> Iterator for CandidateSetIter<'a> {
    type Item = PathMatchCandidate<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.traversal.next().map(|entry| {
            if let EntryKind::File(char_bag) = entry.kind {
                PathMatchCandidate {
                    path: &entry.path,
                    char_bag,
                }
            } else {
                unreachable!()
            }
        })
    }
}

impl Entity for Project {
    type Event = Event;
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::RealFs;
    use gpui::TestAppContext;
    use language::LanguageRegistry;
    use serde_json::json;
    use std::{os::unix, path::PathBuf};
    use util::test::temp_tree;

    #[gpui::test]
    async fn test_populate_and_search(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "root": {
                "apple": "",
                "banana": {
                    "carrot": {
                        "date": "",
                        "endive": "",
                    }
                },
                "fennel": {
                    "grape": "",
                }
            }
        }));

        let root_link_path = dir.path().join("root_link");
        unix::fs::symlink(&dir.path().join("root"), &root_link_path).unwrap();
        unix::fs::symlink(
            &dir.path().join("root/fennel"),
            &dir.path().join("root/finnochio"),
        )
        .unwrap();

        let project = build_project(&mut cx);

        let tree = project
            .update(&mut cx, |project, cx| {
                project.add_local_worktree(&root_link_path, cx)
            })
            .await
            .unwrap();

        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        cx.read(|cx| {
            let tree = tree.read(cx);
            assert_eq!(tree.file_count(), 5);
            assert_eq!(
                tree.inode_for_path("fennel/grape"),
                tree.inode_for_path("finnochio/grape")
            );
        });

        let cancel_flag = Default::default();
        let results = project
            .read_with(&cx, |project, cx| {
                project.match_paths("bna", false, false, 10, &cancel_flag, cx)
            })
            .await;
        assert_eq!(
            results
                .into_iter()
                .map(|result| result.path)
                .collect::<Vec<Arc<Path>>>(),
            vec![
                PathBuf::from("banana/carrot/date").into(),
                PathBuf::from("banana/carrot/endive").into(),
            ]
        );
    }

    #[gpui::test]
    async fn test_search_worktree_without_files(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "root": {
                "dir1": {},
                "dir2": {
                    "dir3": {}
                }
            }
        }));

        let project = build_project(&mut cx);
        let tree = project
            .update(&mut cx, |project, cx| {
                project.add_local_worktree(&dir.path(), cx)
            })
            .await
            .unwrap();

        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        let cancel_flag = Default::default();
        let results = project
            .read_with(&cx, |project, cx| {
                project.match_paths("dir", false, false, 10, &cancel_flag, cx)
            })
            .await;

        assert!(results.is_empty());
    }

    fn build_project(cx: &mut TestAppContext) -> ModelHandle<Project> {
        let languages = Arc::new(LanguageRegistry::new());
        let fs = Arc::new(RealFs);
        let rpc = client::Client::new();
        cx.add_model(|_| Project::new(languages, rpc, fs))
    }
}
