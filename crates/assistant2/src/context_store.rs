use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use collections::{HashMap, HashSet};
use gpui::{Model, ModelContext, SharedString, Task, WeakView};
use language::Buffer;
use project::{ProjectPath, Worktree};
use workspace::Workspace;

use crate::thread::Thread;
use crate::{
    context::{Context, ContextId, ContextKind},
    thread::ThreadId,
};

pub struct ContextStore {
    workspace: WeakView<Workspace>,
    context: Vec<Context>,
    next_context_id: ContextId,
    files: HashMap<PathBuf, ContextId>,
    directories: HashMap<PathBuf, ContextId>,
    threads: HashMap<ThreadId, ContextId>,
    fetched_urls: HashMap<String, ContextId>,
}

impl ContextStore {
    pub fn new(workspace: WeakView<Workspace>) -> Self {
        Self {
            workspace,
            context: Vec::new(),
            next_context_id: ContextId(0),
            files: HashMap::default(),
            directories: HashMap::default(),
            threads: HashMap::default(),
            fetched_urls: HashMap::default(),
        }
    }

    pub fn context(&self) -> &Vec<Context> {
        &self.context
    }

    pub fn clear(&mut self) {
        self.context.clear();
        self.files.clear();
        self.directories.clear();
        self.threads.clear();
        self.fetched_urls.clear();
    }

    pub fn add_file(
        &mut self,
        project_path: ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let workspace = self.workspace.clone();
        let Some(project) = workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).project().clone())
        else {
            return Task::ready(Err(anyhow!("failed to read project")));
        };

        let already_included = match self.included_file(&project_path.path) {
            Some(IncludedFile::Direct(context_id)) => {
                self.remove_context(&context_id);
                true
            }
            Some(IncludedFile::InDirectory(_)) => true,
            None => false,
        };
        if already_included {
            return Task::ready(Ok(()));
        }

        cx.spawn(|this, mut cx| async move {
            let open_buffer_task =
                project.update(&mut cx, |project, cx| project.open_buffer(project_path, cx))?;

            let buffer = open_buffer_task.await?;
            this.update(&mut cx, |this, cx| {
                this.insert_file(buffer.read(cx));
            })?;

            anyhow::Ok(())
        })
    }

    pub fn insert_file(&mut self, buffer: &Buffer) {
        let Some(file) = buffer.file() else {
            return;
        };

        let path = file.path();

        let id = self.next_context_id.post_inc();
        self.files.insert(path.to_path_buf(), id);

        let full_path: SharedString = path.to_string_lossy().into_owned().into();

        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().into_owned().into(),
            None => full_path.clone(),
        };

        let parent = path
            .parent()
            .and_then(|p| p.file_name())
            .map(|p| p.to_string_lossy().into_owned().into());

        let mut text = String::new();
        push_fenced_codeblock(path, buffer.text(), &mut text);

        self.context.push(Context {
            id,
            name,
            parent,
            tooltip: Some(full_path),
            kind: ContextKind::File,
            text: text.into(),
        });
    }

    pub fn add_directory(
        &mut self,
        project_path: ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let workspace = self.workspace.clone();
        let Some(project) = workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).project().clone())
        else {
            return Task::ready(Err(anyhow!("failed to read project")));
        };

        let already_included = if let Some(context_id) = self.included_directory(&project_path.path)
        {
            self.remove_context(&context_id);
            true
        } else {
            false
        };
        if already_included {
            return Task::ready(Ok(()));
        }

        let worktree_id = project_path.worktree_id;
        cx.spawn(|this, mut cx| async move {
            let worktree = project.update(&mut cx, |project, cx| {
                project
                    .worktree_for_id(worktree_id, cx)
                    .ok_or_else(|| anyhow!("no worktree found for {worktree_id:?}"))
            })??;

            let files = worktree.update(&mut cx, |worktree, _cx| {
                collect_files_in_path(worktree, &project_path.path)
            })?;

            let open_buffer_tasks = project.update(&mut cx, |project, cx| {
                files
                    .into_iter()
                    .map(|file_path| {
                        project.open_buffer(
                            ProjectPath {
                                worktree_id,
                                path: file_path.clone(),
                            },
                            cx,
                        )
                    })
                    .collect::<Vec<_>>()
            })?;

            let buffers = futures::future::join_all(open_buffer_tasks).await;

            this.update(&mut cx, |this, cx| {
                let mut text = String::new();
                let mut added_files = 0;

                for buffer in buffers.into_iter().flatten() {
                    let buffer = buffer.read(cx);
                    let path = buffer.file().map_or(&project_path.path, |file| file.path());
                    push_fenced_codeblock(&path, buffer.text(), &mut text);
                    added_files += 1;
                }

                if added_files == 0 {
                    bail!(
                        "could not read any text files from {}",
                        &project_path.path.display()
                    );
                }

                this.insert_directory(&project_path.path, text);

                anyhow::Ok(())
            })??;

            anyhow::Ok(())
        })
    }

    pub fn insert_directory(&mut self, path: &Path, text: impl Into<SharedString>) {
        let id = self.next_context_id.post_inc();
        self.directories.insert(path.to_path_buf(), id);

        let full_path: SharedString = path.to_string_lossy().into_owned().into();

        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().into_owned().into(),
            None => full_path.clone(),
        };

        let parent = path
            .parent()
            .and_then(|p| p.file_name())
            .map(|p| p.to_string_lossy().into_owned().into());

        self.context.push(Context {
            id,
            name,
            parent,
            tooltip: Some(full_path),
            kind: ContextKind::Directory,
            text: text.into(),
        });
    }

    pub fn add_thread(&mut self, thread: Model<Thread>, cx: &mut ModelContext<Self>) {
        if let Some(context_id) = self.included_thread(&thread.read(cx).id()) {
            self.remove_context(&context_id);
        } else {
            self.insert_thread(thread.read(cx));
        }
    }

    pub fn insert_thread(&mut self, thread: &Thread) {
        let context_id = self.next_context_id.post_inc();
        self.threads.insert(thread.id().clone(), context_id);

        self.context.push(Context {
            id: context_id,
            name: thread.summary().unwrap_or("New thread".into()),
            parent: None,
            tooltip: None,
            kind: ContextKind::Thread,
            text: thread.text().into(),
        });
    }

    pub fn insert_fetched_url(&mut self, url: String, text: impl Into<SharedString>) {
        let context_id = self.next_context_id.post_inc();
        self.fetched_urls.insert(url.clone(), context_id);

        self.context.push(Context {
            id: context_id,
            name: url.into(),
            parent: None,
            tooltip: None,
            kind: ContextKind::FetchedUrl,
            text: text.into(),
        });
    }

    pub fn remove_context(&mut self, id: &ContextId) {
        let Some(ix) = self.context.iter().position(|context| context.id == *id) else {
            return;
        };

        match self.context.remove(ix).kind {
            ContextKind::File => {
                self.files.retain(|_, context_id| context_id != id);
            }
            ContextKind::Directory => {
                self.directories.retain(|_, context_id| context_id != id);
            }
            ContextKind::FetchedUrl => {
                self.fetched_urls.retain(|_, context_id| context_id != id);
            }
            ContextKind::Thread => {
                self.threads.retain(|_, context_id| context_id != id);
            }
        }
    }

    pub fn included_file(&self, path: &Path) -> Option<IncludedFile> {
        if let Some(id) = self.files.get(path) {
            return Some(IncludedFile::Direct(*id));
        }

        if self.directories.is_empty() {
            return None;
        }

        let mut buf = path.to_path_buf();

        while buf.pop() {
            if let Some(_) = self.directories.get(&buf) {
                return Some(IncludedFile::InDirectory(buf));
            }
        }

        None
    }

    pub fn included_directory(&self, path: &Path) -> Option<ContextId> {
        self.directories.get(path).copied()
    }

    pub fn included_thread(&self, thread_id: &ThreadId) -> Option<ContextId> {
        self.threads.get(thread_id).copied()
    }

    pub fn included_url(&self, url: &str) -> Option<ContextId> {
        self.fetched_urls.get(url).copied()
    }

    pub fn duplicated_names(&self) -> HashSet<SharedString> {
        let mut seen = HashSet::default();
        let mut dupes = HashSet::default();

        for context in self.context().iter() {
            if !seen.insert(&context.name) {
                dupes.insert(context.name.clone());
            }
        }

        dupes
    }
}

pub enum IncludedFile {
    Direct(ContextId),
    InDirectory(PathBuf),
}

pub(crate) fn push_fenced_codeblock(path: &Path, content: String, buffer: &mut String) {
    buffer.reserve(content.len() + 64);

    write!(buffer, "```").unwrap();

    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        write!(buffer, "{} ", extension).unwrap();
    }

    write!(buffer, "{}", path.display()).unwrap();

    buffer.push('\n');
    buffer.push_str(&content);

    if !buffer.ends_with('\n') {
        buffer.push('\n');
    }

    buffer.push_str("```\n");
}

fn collect_files_in_path(worktree: &Worktree, path: &Path) -> Vec<Arc<Path>> {
    let mut files = Vec::new();

    for entry in worktree.child_entries(path) {
        if entry.is_dir() {
            files.extend(collect_files_in_path(worktree, &entry.path));
        } else if entry.is_file() {
            files.push(entry.path.clone());
        }
    }

    files
}
