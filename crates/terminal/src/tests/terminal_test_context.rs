use std::{path::Path, time::Duration};

use gpui::{AppContext, ModelHandle, ReadModelWith, TestAppContext, ViewHandle};

use itertools::Itertools;
use project::{Entry, Project, ProjectPath, Worktree};
use workspace::{AppState, Workspace};

use crate::{TermDimensions, Terminal, TerminalBuilder};

pub struct TerminalTestContext<'a> {
    pub cx: &'a mut TestAppContext,
    pub connection: Option<ModelHandle<Terminal>>,
}

impl<'a> TerminalTestContext<'a> {
    pub fn new(cx: &'a mut TestAppContext, term: bool) -> Self {
        cx.set_condition_duration(Some(Duration::from_secs(5)));

        let size_info = TermDimensions::default();

        let connection = term.then(|| {
            cx.add_model(|cx| {
                TerminalBuilder::new(None, None, None, size_info)
                    .unwrap()
                    .subscribe(cx)
            })
        });

        TerminalTestContext { cx, connection }
    }

    pub async fn execute_and_wait<F>(&mut self, command: &str, f: F) -> String
    where
        F: Fn(String, &AppContext) -> bool,
    {
        let connection = self.connection.take().unwrap();

        let command = command.to_string();
        connection.update(self.cx, |connection, _| {
            connection.write_to_pty(command);
            connection.write_to_pty("\r".to_string());
        });

        connection
            .condition(self.cx, |term, cx| {
                let content = Self::grid_as_str(term);

                f(content, cx)
            })
            .await;

        let res = self
            .cx
            .read_model_with(&connection, &mut |conn, _: &AppContext| {
                Self::grid_as_str(conn)
            });

        self.connection = Some(connection);

        res
    }

    ///Creates a worktree with 1 file: /root.txt
    pub async fn blank_workspace(&mut self) -> (ModelHandle<Project>, ViewHandle<Workspace>) {
        let params = self.cx.update(AppState::test);

        let project = Project::test(params.fs.clone(), [], self.cx).await;
        let (_, workspace) = self.cx.add_window(|cx| Workspace::new(project.clone(), cx));

        (project, workspace)
    }

    ///Creates a worktree with 1 folder: /root{suffix}/
    pub async fn create_folder_wt(
        &mut self,
        project: ModelHandle<Project>,
        path: impl AsRef<Path>,
    ) -> (ModelHandle<Worktree>, Entry) {
        self.create_wt(project, true, path).await
    }

    ///Creates a worktree with 1 file: /root{suffix}.txt
    pub async fn create_file_wt(
        &mut self,
        project: ModelHandle<Project>,
        path: impl AsRef<Path>,
    ) -> (ModelHandle<Worktree>, Entry) {
        self.create_wt(project, false, path).await
    }

    async fn create_wt(
        &mut self,
        project: ModelHandle<Project>,
        is_dir: bool,
        path: impl AsRef<Path>,
    ) -> (ModelHandle<Worktree>, Entry) {
        let (wt, _) = project
            .update(self.cx, |project, cx| {
                project.find_or_create_local_worktree(path, true, cx)
            })
            .await
            .unwrap();

        let entry = self
            .cx
            .update(|cx| {
                wt.update(cx, |wt, cx| {
                    wt.as_local()
                        .unwrap()
                        .create_entry(Path::new(""), is_dir, cx)
                })
            })
            .await
            .unwrap();

        (wt, entry)
    }

    pub fn insert_active_entry_for(
        &mut self,
        wt: ModelHandle<Worktree>,
        entry: Entry,
        project: ModelHandle<Project>,
    ) {
        self.cx.update(|cx| {
            let p = ProjectPath {
                worktree_id: wt.read(cx).id(),
                path: entry.path,
            };
            project.update(cx, |project, cx| project.set_active_path(Some(p), cx));
        });
    }

    fn grid_as_str(connection: &Terminal) -> String {
        let content = connection.grid();

        let lines = content.display_iter().group_by(|i| i.point.line.0);
        lines
            .into_iter()
            .map(|(_, line)| line.map(|i| i.c).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<'a> Drop for TerminalTestContext<'a> {
    fn drop(&mut self) {
        self.cx.set_condition_duration(None);
    }
}
