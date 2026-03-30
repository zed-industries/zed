use std::sync::Arc;
use std::time::Duration;

use crate::Editor;
use collections::HashMap;
use gpui::AsyncApp;
use gpui::{App, Entity, Task};
use language::Buffer;
use language::Language;
use lsp::LanguageServerId;
use lsp::LanguageServerName;
use multi_buffer::Anchor;
use project::LanguageServerToQuery;
use project::LocationLink;
use project::Project;
use project::TaskSourceKind;
use project::lsp_store::lsp_ext_command::GetLspRunnables;
use smol::future::FutureExt as _;
use task::ResolvedTask;
use task::TaskContext;
use text::BufferId;
use ui::SharedString;
use util::ResultExt as _;

pub(crate) fn find_specific_language_server_in_selection<F>(
    editor: &Editor,
    cx: &mut App,
    filter_language: F,
    language_server_name: LanguageServerName,
) -> Option<(Anchor, Arc<Language>, LanguageServerId, Entity<Buffer>)>
where
    F: Fn(&Language) -> bool,
{
    let project = editor.project.clone()?;
    let multi_buffer = editor.buffer();
    let mut seen_buffer_ids = Vec::new();
    editor
        .selections
        .disjoint_anchors_arc()
        .iter()
        .find_map(|selection| {
            let head = selection.head();
            let (buffer_id, buffer) = {
                let multi_buffer_ref = multi_buffer.read(cx);
                let snapshot = multi_buffer_ref.read(cx);
                let buffer_id = snapshot
                    .buffer_id_for_anchor(head)
                    .or_else(|| snapshot.buffer_id_for_anchor(selection.tail()))?;
                drop(snapshot);
                let buffer = multi_buffer_ref.buffer(buffer_id)?;
                (buffer_id, buffer)
            };

            if seen_buffer_ids.contains(&buffer_id) {
                return None;
            }
            seen_buffer_ids.push(buffer_id);

            let language = buffer.read(cx).language_at(head.text_anchor)?;
            if filter_language(&language) {
                let server_id = buffer.update(cx, |buffer, cx| {
                    project
                        .read(cx)
                        .language_server_id_for_name(buffer, &language_server_name, cx)
                })?;
                Some((head, language, server_id, buffer))
            } else {
                None
            }
        })
}

async fn lsp_task_context(
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    cx: &mut AsyncApp,
) -> Option<TaskContext> {
    let (worktree_store, environment) = project.read_with(cx, |project, _| {
        (project.worktree_store(), project.environment().clone())
    });

    let worktree_abs_path = cx.update(|cx| {
        let worktree_id = buffer.read(cx).file().map(|f| f.worktree_id(cx));

        worktree_id
            .and_then(|worktree_id| worktree_store.read(cx).worktree_for_id(worktree_id, cx))
            .and_then(|worktree| worktree.read(cx).root_dir())
    });

    let project_env = environment
        .update(cx, |environment, cx| {
            environment.buffer_environment(buffer, &worktree_store, cx)
        })
        .await;

    Some(TaskContext {
        cwd: worktree_abs_path.map(|p| p.to_path_buf()),
        project_env: project_env.unwrap_or_default(),
        ..TaskContext::default()
    })
}

pub fn lsp_tasks(
    project: Entity<Project>,
    task_sources: &HashMap<LanguageServerName, Vec<BufferId>>,
    for_position: Option<text::Anchor>,
    cx: &mut App,
) -> Task<Vec<(TaskSourceKind, Vec<(Option<LocationLink>, ResolvedTask)>)>> {
    let lsp_task_sources = task_sources
        .iter()
        .filter_map(|(name, buffer_ids)| {
            let buffers = buffer_ids
                .iter()
                .filter(|&&buffer_id| match for_position {
                    Some(for_position) => for_position.buffer_id == Some(buffer_id),
                    None => true,
                })
                .filter_map(|&buffer_id| project.read(cx).buffer_for_id(buffer_id, cx))
                .collect::<Vec<_>>();

            let server_id = buffers.iter().find_map(|buffer| {
                project.read_with(cx, |project, cx| {
                    project.language_server_id_for_name(buffer.read(cx), name, cx)
                })
            });
            server_id.zip(Some(buffers))
        })
        .collect::<Vec<_>>();

    cx.spawn(async move |cx| {
        cx.spawn(async move |cx| {
            let mut lsp_tasks = HashMap::default();
            for (server_id, buffers) in lsp_task_sources {
                let mut new_lsp_tasks = Vec::new();
                for buffer in buffers {
                    let source_kind = match buffer.update(cx, |buffer, _| {
                        buffer.language().map(|language| language.name())
                    }) {
                        Some(language_name) => TaskSourceKind::Lsp {
                            server: server_id,
                            language_name: SharedString::from(language_name),
                        },
                        None => continue,
                    };
                    let id_base = source_kind.to_id_base();
                    let lsp_buffer_context = lsp_task_context(&project, &buffer, cx)
                        .await
                        .unwrap_or_default();

                    let runnables_task = project.update(cx, |project, cx| {
                        let buffer_id = buffer.read(cx).remote_id();
                        project.request_lsp(
                            buffer,
                            LanguageServerToQuery::Other(server_id),
                            GetLspRunnables {
                                buffer_id,
                                position: for_position,
                            },
                            cx,
                        )
                    });
                    if let Some(new_runnables) = runnables_task.await.log_err() {
                        new_lsp_tasks.extend(new_runnables.runnables.into_iter().filter_map(
                            |(location, runnable)| {
                                let resolved_task =
                                    runnable.resolve_task(&id_base, &lsp_buffer_context)?;
                                Some((location, resolved_task))
                            },
                        ));
                    }
                    lsp_tasks
                        .entry(source_kind)
                        .or_insert_with(Vec::new)
                        .append(&mut new_lsp_tasks);
                }
            }
            lsp_tasks.into_iter().collect()
        })
        .race({
            // `lsp::DEFAULT_LSP_REQUEST_TIMEOUT` is larger than we want for the modal to open fast
            let timer = cx.background_executor().timer(Duration::from_millis(200));
            async move {
                timer.await;
                log::info!("Timed out waiting for LSP tasks");
                Vec::new()
            }
        })
        .await
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::StreamExt as _;
    use gpui::{App, AppContext as _, Entity, TestAppContext};
    use language::{FakeLspAdapter, Language};
    use languages::rust_lang;
    use lsp::{LanguageServerId, LanguageServerName};
    use multi_buffer::{Anchor, MultiBuffer, MultiBufferOffset};
    use project::{FakeFs, Project};
    use util::path;

    use crate::{SelectionEffects, editor_tests::init_test, test::build_editor_with_project};

    use super::find_specific_language_server_in_selection;

    #[gpui::test]
    async fn test_find_language_server_at_end_of_file(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_file(path!("/file.rs"), "fn main() {}".into())
            .await;

        let project = Project::test(fs, [path!("/file.rs").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());
        let mut fake_servers =
            language_registry.register_fake_lsp("Rust", FakeLspAdapter::default());

        let underlying_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/file.rs"), cx)
            })
            .await
            .unwrap();

        let buffer = cx.new(|cx| MultiBuffer::singleton(underlying_buffer.clone(), cx));
        let (editor, cx) = cx.add_window_view(|window, cx| {
            build_editor_with_project(project.clone(), buffer, window, cx)
        });

        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().run_until_parked();

        let expected_server_id = fake_server.server.server_id();
        let language_server_name = LanguageServerName::new_static("the-fake-language-server");
        let filter = |language: &Language| language.name().as_ref() == "Rust";

        let assert_result = |result: Option<(
            Anchor,
            Arc<Language>,
            LanguageServerId,
            Entity<language::Buffer>,
        )>,
                             cx: &App,
                             message: &str| {
            let (_, language, server_id, buffer) = result.expect(message);
            assert_eq!(
                language.name().as_ref(),
                "Rust",
                "{message}: wrong language"
            );
            assert_eq!(server_id, expected_server_id, "{message}: wrong server ID");
            assert_eq!(buffer, underlying_buffer, "{message}: wrong buffer");
            assert!(
                buffer.read(cx).file().is_some(),
                "{message}: buffer should have a file"
            );
        };

        editor.update(cx, |editor, cx| {
            assert_result(
                find_specific_language_server_in_selection(
                    editor,
                    cx,
                    filter,
                    language_server_name.clone(),
                ),
                cx,
                "should find correct language server at beginning of file",
            );
        });

        // Move cursor to end of file.
        editor.update_in(cx, |editor, window, cx| {
            let text_len = editor.text(cx).len();
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges([MultiBufferOffset(text_len)..MultiBufferOffset(text_len)])
            });
        });

        editor.update(cx, |editor, cx| {
            assert_result(
                find_specific_language_server_in_selection(
                    editor,
                    cx,
                    filter,
                    language_server_name.clone(),
                ),
                cx,
                "should find correct language server at end of file",
            );
        });
    }
}
