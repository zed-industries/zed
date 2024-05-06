use anyhow::{anyhow, Result};
use gpui::{AppContext, Model, Task, WeakModel};
use project::{Fs, Project, ProjectPath, Worktree};
use std::{cmp::Ordering, fmt::Write as _, ops::Range, sync::Arc};
use sum_tree::TreeMap;

pub struct ProjectContext {
    files: TreeMap<ProjectPath, PathState>,
    project: WeakModel<Project>,
    fs: Arc<dyn Fs>,
}

#[derive(Debug, Clone)]
enum PathState {
    PathOnly,
    EntireFile,
    Excerpts { ranges: Vec<Range<usize>> },
}

impl ProjectContext {
    pub fn new(project: WeakModel<Project>, fs: Arc<dyn Fs>) -> Self {
        Self {
            files: TreeMap::default(),
            fs,
            project,
        }
    }

    pub fn add_path(&mut self, project_path: ProjectPath) {
        if self.files.get(&project_path).is_none() {
            self.files.insert(project_path, PathState::PathOnly);
        }
    }

    pub fn add_excerpts(&mut self, project_path: ProjectPath, new_ranges: &[Range<usize>]) {
        let previous_state = self
            .files
            .get(&project_path)
            .unwrap_or(&PathState::PathOnly);

        let mut ranges = match previous_state {
            PathState::EntireFile => return,
            PathState::PathOnly => Vec::new(),
            PathState::Excerpts { ranges } => ranges.to_vec(),
        };

        for new_range in new_ranges {
            let ix = ranges.binary_search_by(|probe| {
                if probe.end < new_range.start {
                    Ordering::Less
                } else if probe.start > new_range.end {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            match ix {
                Ok(mut ix) => {
                    let existing = &mut ranges[ix];
                    existing.start = existing.start.min(new_range.start);
                    existing.end = existing.end.max(new_range.end);
                    while ix + 1 < ranges.len() && ranges[ix + 1].start <= ranges[ix].end {
                        ranges[ix].end = ranges[ix].end.max(ranges[ix + 1].end);
                        ranges.remove(ix + 1);
                    }
                    while ix > 0 && ranges[ix - 1].end >= ranges[ix].start {
                        ranges[ix].start = ranges[ix].start.min(ranges[ix - 1].start);
                        ranges.remove(ix - 1);
                        ix -= 1;
                    }
                }
                Err(ix) => {
                    ranges.insert(ix, new_range.clone());
                }
            }
        }

        self.files
            .insert(project_path, PathState::Excerpts { ranges });
    }

    pub fn add_file(&mut self, project_path: ProjectPath) {
        self.files.insert(project_path, PathState::EntireFile);
    }

    pub fn generate_system_message(&self, cx: &mut AppContext) -> Task<Result<String>> {
        let project = self
            .project
            .upgrade()
            .ok_or_else(|| anyhow!("project dropped"));
        let files = self.files.clone();
        let fs = self.fs.clone();
        cx.spawn(|cx| async move {
            let project = project?;
            let mut result = "project structure:\n".to_string();

            let mut last_worktree: Option<Model<Worktree>> = None;
            for (project_path, path_state) in files.iter() {
                if let Some(worktree) = &last_worktree {
                    if worktree.read_with(&cx, |tree, _| tree.id())? != project_path.worktree_id {
                        last_worktree = None;
                    }
                }

                let worktree;
                if let Some(last_worktree) = &last_worktree {
                    worktree = last_worktree.clone();
                } else if let Some(tree) = project.read_with(&cx, |project, cx| {
                    project.worktree_for_id(project_path.worktree_id, cx)
                })? {
                    worktree = tree;
                    last_worktree = Some(worktree.clone());
                    let worktree_name =
                        worktree.read_with(&cx, |tree, _cx| tree.root_name().to_string())?;
                    writeln!(&mut result, "# {}", worktree_name).unwrap();
                } else {
                    continue;
                }

                let worktree_abs_path = worktree.read_with(&cx, |tree, _cx| tree.abs_path())?;
                let path = &project_path.path;
                writeln!(&mut result, "## {}", path.display()).unwrap();

                match path_state {
                    PathState::PathOnly => {}
                    PathState::EntireFile => {
                        let text = fs.load(&worktree_abs_path.join(&path)).await?;
                        writeln!(&mut result, "~~~\n{text}\n~~~").unwrap();
                    }
                    PathState::Excerpts { ranges } => {
                        let text = fs.load(&worktree_abs_path.join(&path)).await?;

                        writeln!(&mut result, "~~~").unwrap();

                        // Assumption: ranges are in order, not overlapping
                        let mut prev_range_end = 0;
                        for range in ranges {
                            if range.start > prev_range_end {
                                writeln!(&mut result, "...").unwrap();
                                prev_range_end = range.end;
                            }

                            let mut start = range.start;
                            let mut end = range.end.min(text.len());
                            while !text.is_char_boundary(start) {
                                start += 1;
                            }
                            while !text.is_char_boundary(end) {
                                end -= 1;
                            }
                            result.push_str(&text[start..end]);
                            if !result.ends_with('\n') {
                                result.push('\n');
                            }
                        }

                        if prev_range_end < text.len() {
                            writeln!(&mut result, "...").unwrap();
                        }

                        writeln!(&mut result, "~~~").unwrap();
                    }
                }
            }
            Ok(result)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use gpui::TestAppContext;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;

    use unindent::Unindent as _;

    #[gpui::test]
    async fn test_system_message_generation(cx: &mut TestAppContext) {
        init_test(cx);

        let file_3_contents = r#"
            fn test1() {}
            fn test2() {}
            fn test3() {}
        "#
        .unindent();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/code",
            json!({
                "root1": {
                    "lib": {
                        "file1.rs": "mod example;",
                        "file2.rs": "",
                    },
                    "test": {
                        "file3.rs": file_3_contents,
                    }
                },
                "root2": {
                    "src": {
                        "main.rs": ""
                    }
                }
            }),
        )
        .await;

        let project = Project::test(
            fs.clone(),
            ["/code/root1".as_ref(), "/code/root2".as_ref()],
            cx,
        )
        .await;

        let worktree_ids = project.read_with(cx, |project, cx| {
            project
                .worktrees()
                .map(|worktree| worktree.read(cx).id())
                .collect::<Vec<_>>()
        });

        let mut ax = ProjectContext::new(project.downgrade(), fs);

        ax.add_file(ProjectPath {
            worktree_id: worktree_ids[0],
            path: Path::new("lib/file1.rs").into(),
        });

        let message = cx
            .update(|cx| ax.generate_system_message(cx))
            .await
            .unwrap();
        assert_eq!(
            r#"
            project structure:
            # root1
            ## lib/file1.rs
            ~~~
            mod example;
            ~~~
            "#
            .unindent(),
            message
        );

        ax.add_excerpts(
            ProjectPath {
                worktree_id: worktree_ids[0],
                path: Path::new("test/file3.rs").into(),
            },
            &[
                file_3_contents.find("fn test2").unwrap()
                    ..file_3_contents.find("fn test3").unwrap(),
            ],
        );

        let message = cx
            .update(|cx| ax.generate_system_message(cx))
            .await
            .unwrap();
        assert_eq!(
            r#"
            project structure:
            # root1
            ## lib/file1.rs
            ~~~
            mod example;
            ~~~
            ## test/file3.rs
            ~~~
            ...
            fn test2() {}
            ...
            ~~~
            "#
            .unindent(),
            message
        );
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
        });
    }
}
