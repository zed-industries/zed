use crate::{
    fs::Fs,
    fuzzy::{CharBag, Matcher, PathMatchCandidate},
    language::LanguageRegistry,
    rpc::Client,
    util::{self, TryFutureExt as _},
    worktree::{self, EntryKind, PathMatch, Snapshot, Worktree},
    AppState,
};
use anyhow::Result;
use futures::Future;
use gpui::{AppContext, Entity, ModelContext, ModelHandle, Task};
use std::{
    cmp,
    path::Path,
    sync::{atomic::AtomicBool, Arc},
};

pub struct Project {
    worktrees: Vec<ModelHandle<Worktree>>,
    active_entry: Option<(usize, usize)>,
    languages: Arc<LanguageRegistry>,
    rpc: Arc<Client>,
    fs: Arc<dyn Fs>,
}

pub enum Event {
    ActiveEntryChanged(Option<(usize, usize)>),
    WorktreeRemoved(usize),
}

impl Project {
    pub fn new(app_state: &AppState) -> Self {
        Self {
            worktrees: Default::default(),
            active_entry: None,
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
        abs_path: &Path,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Worktree>>> {
        let fs = self.fs.clone();
        let rpc = self.rpc.clone();
        let languages = self.languages.clone();
        let path = Arc::from(abs_path);
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

    pub fn set_active_entry(
        &mut self,
        entry: Option<(usize, Arc<Path>)>,
        cx: &mut ModelContext<Self>,
    ) {
        let new_active_entry = entry.and_then(|(worktree_id, path)| {
            let worktree = self.worktree_for_id(worktree_id)?;
            let entry = worktree.read(cx).entry_for_path(path)?;
            Some((worktree_id, entry.id))
        });
        if new_active_entry != self.active_entry {
            self.active_entry = new_active_entry;
            cx.emit(Event::ActiveEntryChanged(new_active_entry));
        }
    }

    pub fn active_entry(&self) -> Option<(usize, usize)> {
        self.active_entry
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
        query: &str,
        include_ignored: bool,
        smart_case: bool,
        max_results: usize,
        cancel_flag: &'a AtomicBool,
        cx: &AppContext,
    ) -> impl 'a + Future<Output = Vec<PathMatch>> {
        let snapshots = self
            .worktrees
            .iter()
            .map(|worktree| worktree.read(cx).snapshot())
            .collect::<Vec<_>>();

        let path_count: usize = if include_ignored {
            snapshots.iter().map(Snapshot::file_count).sum()
        } else {
            snapshots.iter().map(Snapshot::visible_file_count).sum()
        };

        let lowercase_query = query.to_lowercase().chars().collect::<Vec<_>>();
        let query = query.chars().collect::<Vec<_>>();
        let query_char_bag = CharBag::from(&lowercase_query[..]);

        let background = cx.background().clone();

        async move {
            if path_count == 0 {
                return Vec::new();
            }

            let num_cpus = background.num_cpus().min(path_count);
            let segment_size = (path_count + num_cpus - 1) / num_cpus;
            let mut segment_results = (0..num_cpus)
                .map(|_| Vec::with_capacity(max_results))
                .collect::<Vec<_>>();

            let lowercase_query = &lowercase_query;
            let query = &query;
            let snapshots = snapshots.as_slice();

            background
                .scoped(|scope| {
                    for (segment_idx, results) in segment_results.iter_mut().enumerate() {
                        scope.spawn(async move {
                            let segment_start = segment_idx * segment_size;
                            let segment_end = segment_start + segment_size;
                            let mut matcher = Matcher::new(
                                query,
                                lowercase_query,
                                query_char_bag,
                                smart_case,
                                max_results,
                            );

                            let mut tree_start = 0;
                            for snapshot in snapshots {
                                let tree_end = if include_ignored {
                                    tree_start + snapshot.file_count()
                                } else {
                                    tree_start + snapshot.visible_file_count()
                                };

                                if tree_start < segment_end && segment_start < tree_end {
                                    let path_prefix: Arc<str> =
                                        if snapshot.root_entry().map_or(false, |e| e.is_file()) {
                                            snapshot.root_name().into()
                                        } else if snapshots.len() > 1 {
                                            format!("{}/", snapshot.root_name()).into()
                                        } else {
                                            "".into()
                                        };

                                    let start = cmp::max(tree_start, segment_start) - tree_start;
                                    let end = cmp::min(tree_end, segment_end) - tree_start;
                                    let paths = snapshot
                                        .files(include_ignored, start)
                                        .take(end - start)
                                        .map(|entry| {
                                            if let EntryKind::File(char_bag) = entry.kind {
                                                PathMatchCandidate {
                                                    path: &entry.path,
                                                    char_bag,
                                                }
                                            } else {
                                                unreachable!()
                                            }
                                        });

                                    matcher.match_paths(
                                        snapshot.id(),
                                        path_prefix,
                                        paths,
                                        results,
                                        &cancel_flag,
                                    );
                                }
                                if tree_end >= segment_end {
                                    break;
                                }
                                tree_start = tree_end;
                            }
                        })
                    }
                })
                .await;

            let mut results = Vec::new();
            for segment_result in segment_results {
                if results.is_empty() {
                    results = segment_result;
                } else {
                    util::extend_sorted(&mut results, segment_result, max_results, |a, b| {
                        b.cmp(&a)
                    });
                }
            }
            results
        }
    }
}

impl Entity for Project {
    type Event = Event;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fs::RealFs,
        test::{temp_tree, test_app_state},
    };
    use serde_json::json;
    use std::{os::unix, path::PathBuf};

    #[gpui::test]
    async fn test_populate_and_search(mut cx: gpui::TestAppContext) {
        let mut app_state = cx.update(test_app_state);
        Arc::get_mut(&mut app_state).unwrap().fs = Arc::new(RealFs);
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

        let project = cx.add_model(|_| Project::new(app_state.as_ref()));
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
        let mut app_state = cx.update(test_app_state);
        Arc::get_mut(&mut app_state).unwrap().fs = Arc::new(RealFs);
        let dir = temp_tree(json!({
            "root": {
                "dir1": {},
                "dir2": {
                    "dir3": {}
                }
            }
        }));

        let project = cx.add_model(|_| Project::new(app_state.as_ref()));
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
}
