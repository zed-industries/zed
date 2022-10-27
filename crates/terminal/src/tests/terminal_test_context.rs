use std::{path::Path, time::Duration};

use alacritty_terminal::{
    index::{Column, Line, Point},
    term::cell::Cell,
};
use gpui::{ModelHandle, TestAppContext, ViewHandle};

use project::{Entry, Project, ProjectPath, Worktree};
use rand::{rngs::ThreadRng, Rng};
use workspace::{AppState, Workspace};

use crate::{IndexedCell, TerminalContent, TerminalSize};

pub struct TerminalTestContext<'a> {
    pub cx: &'a mut TestAppContext,
}

impl<'a> TerminalTestContext<'a> {
    pub fn new(cx: &'a mut TestAppContext) -> Self {
        cx.set_condition_duration(Some(Duration::from_secs(5)));

        TerminalTestContext { cx }
    }

    ///Creates a worktree with 1 file: /root.txt
    pub async fn blank_workspace(&mut self) -> (ModelHandle<Project>, ViewHandle<Workspace>) {
        let params = self.cx.update(AppState::test);

        let project = Project::test(params.fs.clone(), [], self.cx).await;
        let (_, workspace) = self.cx.add_window(|cx| {
            Workspace::new(
                Default::default(),
                project.clone(),
                |_, _| unimplemented!(),
                cx,
            )
        });

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

    pub fn create_terminal_content(
        size: TerminalSize,
        rng: &mut ThreadRng,
    ) -> (TerminalContent, Vec<Vec<char>>) {
        let mut ic = Vec::new();
        let mut cells = Vec::new();

        for row in 0..((size.height() / size.line_height()) as usize) {
            let mut row_vec = Vec::new();
            for col in 0..((size.width() / size.cell_width()) as usize) {
                let cell_char = rng.gen();
                ic.push(IndexedCell {
                    point: Point::new(Line(row as i32), Column(col)),
                    cell: Cell {
                        c: cell_char,
                        ..Default::default()
                    },
                });
                row_vec.push(cell_char)
            }
            cells.push(row_vec)
        }

        (
            TerminalContent {
                cells: ic,
                size,
                ..Default::default()
            },
            cells,
        )
    }
}

impl<'a> Drop for TerminalTestContext<'a> {
    fn drop(&mut self) {
        self.cx.set_condition_duration(None);
    }
}
