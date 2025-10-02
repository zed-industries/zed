use super::{RandomizedTest, TestClient, TestError, TestServer, UserTestPlan};
use crate::{db::UserId, tests::run_randomized_test};
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use call::ActiveCall;
use collections::{BTreeMap, HashMap};
use editor::Bias;
use fs::{FakeFs, Fs as _};
use git::status::{FileStatus, StatusCode, TrackedStatus, UnmergedStatus, UnmergedStatusCode};
use gpui::{BackgroundExecutor, Entity, TestAppContext};
use language::{
    FakeLspAdapter, Language, LanguageConfig, LanguageMatcher, PointUtf16, range_to_lsp,
};
use lsp::FakeLanguageServer;
use pretty_assertions::assert_eq;
use project::{
    DEFAULT_COMPLETION_CONTEXT, Project, ProjectPath, search::SearchQuery, search::SearchResult,
};
use rand::{
    distr::{self, SampleString},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, Range},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use util::{
    ResultExt, path,
    paths::PathStyle,
    rel_path::{RelPath, RelPathBuf, rel_path},
};

#[gpui::test(
    iterations = 100,
    on_failure = "crate::tests::save_randomized_test_plan"
)]
async fn test_random_project_collaboration(
    cx: &mut TestAppContext,
    executor: BackgroundExecutor,
    rng: StdRng,
) {
    run_randomized_test::<ProjectCollaborationTest>(cx, executor, rng).await;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum ClientOperation {
    AcceptIncomingCall,
    RejectIncomingCall,
    LeaveCall,
    InviteContactToCall {
        user_id: UserId,
    },
    OpenLocalProject {
        first_root_name: String,
    },
    OpenRemoteProject {
        host_id: UserId,
        first_root_name: String,
    },
    AddWorktreeToProject {
        project_root_name: String,
        new_root_path: PathBuf,
    },
    CloseRemoteProject {
        project_root_name: String,
    },
    OpenBuffer {
        project_root_name: String,
        is_local: bool,
        full_path: RelPathBuf,
    },
    SearchProject {
        project_root_name: String,
        is_local: bool,
        query: String,
        detach: bool,
    },
    EditBuffer {
        project_root_name: String,
        is_local: bool,
        full_path: RelPathBuf,
        edits: Vec<(Range<usize>, Arc<str>)>,
    },
    CloseBuffer {
        project_root_name: String,
        is_local: bool,
        full_path: RelPathBuf,
    },
    SaveBuffer {
        project_root_name: String,
        is_local: bool,
        full_path: RelPathBuf,
        detach: bool,
    },
    RequestLspDataInBuffer {
        project_root_name: String,
        is_local: bool,
        full_path: RelPathBuf,
        offset: usize,
        kind: LspRequestKind,
        detach: bool,
    },
    CreateWorktreeEntry {
        project_root_name: String,
        is_local: bool,
        full_path: RelPathBuf,
        is_dir: bool,
    },
    WriteFsEntry {
        path: PathBuf,
        is_dir: bool,
        content: String,
    },
    GitOperation {
        operation: GitOperation,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum GitOperation {
    WriteGitIndex {
        repo_path: PathBuf,
        contents: Vec<(RelPathBuf, String)>,
    },
    WriteGitBranch {
        repo_path: PathBuf,
        new_branch: Option<String>,
    },
    WriteGitStatuses {
        repo_path: PathBuf,
        statuses: Vec<(RelPathBuf, FileStatus)>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum LspRequestKind {
    Rename,
    Completion,
    CodeAction,
    Definition,
    Highlights,
}

struct ProjectCollaborationTest;

#[async_trait(?Send)]
impl RandomizedTest for ProjectCollaborationTest {
    type Operation = ClientOperation;

    async fn initialize(server: &mut TestServer, users: &[UserTestPlan]) {
        let db = &server.app_state.db;
        for (ix, user_a) in users.iter().enumerate() {
            for user_b in &users[ix + 1..] {
                db.send_contact_request(user_a.user_id, user_b.user_id)
                    .await
                    .unwrap();
                db.respond_to_contact_request(user_b.user_id, user_a.user_id, true)
                    .await
                    .unwrap();
            }
        }
    }

    fn generate_operation(
        client: &TestClient,
        rng: &mut StdRng,
        plan: &mut UserTestPlan,
        cx: &TestAppContext,
    ) -> ClientOperation {
        let call = cx.read(ActiveCall::global);
        loop {
            match rng.random_range(0..100_u32) {
                // Mutate the call
                0..=29 => {
                    // Respond to an incoming call
                    if call.read_with(cx, |call, _| call.incoming().borrow().is_some()) {
                        break if rng.random_bool(0.7) {
                            ClientOperation::AcceptIncomingCall
                        } else {
                            ClientOperation::RejectIncomingCall
                        };
                    }

                    match rng.random_range(0..100_u32) {
                        // Invite a contact to the current call
                        0..=70 => {
                            let available_contacts =
                                client.user_store().read_with(cx, |user_store, _| {
                                    user_store
                                        .contacts()
                                        .iter()
                                        .filter(|contact| contact.online && !contact.busy)
                                        .cloned()
                                        .collect::<Vec<_>>()
                                });
                            if !available_contacts.is_empty() {
                                let contact = available_contacts.choose(rng).unwrap();
                                break ClientOperation::InviteContactToCall {
                                    user_id: UserId(contact.user.id as i32),
                                };
                            }
                        }

                        // Leave the current call
                        71.. => {
                            if plan.allow_client_disconnection
                                && call.read_with(cx, |call, _| call.room().is_some())
                            {
                                break ClientOperation::LeaveCall;
                            }
                        }
                    }
                }

                // Mutate projects
                30..=59 => match rng.random_range(0..100_u32) {
                    // Open a new project
                    0..=70 => {
                        // Open a remote project
                        if let Some(room) = call.read_with(cx, |call, _| call.room().cloned()) {
                            let existing_dev_server_project_ids = cx.read(|cx| {
                                client
                                    .dev_server_projects()
                                    .iter()
                                    .map(|p| p.read(cx).remote_id().unwrap())
                                    .collect::<Vec<_>>()
                            });
                            let new_dev_server_projects = room.read_with(cx, |room, _| {
                                room.remote_participants()
                                    .values()
                                    .flat_map(|participant| {
                                        participant.projects.iter().filter_map(|project| {
                                            if existing_dev_server_project_ids.contains(&project.id)
                                            {
                                                None
                                            } else {
                                                Some((
                                                    UserId::from_proto(participant.user.id),
                                                    project.worktree_root_names[0].clone(),
                                                ))
                                            }
                                        })
                                    })
                                    .collect::<Vec<_>>()
                            });
                            if !new_dev_server_projects.is_empty() {
                                let (host_id, first_root_name) =
                                    new_dev_server_projects.choose(rng).unwrap().clone();
                                break ClientOperation::OpenRemoteProject {
                                    host_id,
                                    first_root_name,
                                };
                            }
                        }
                        // Open a local project
                        else {
                            let first_root_name = plan.next_root_dir_name();
                            break ClientOperation::OpenLocalProject { first_root_name };
                        }
                    }

                    // Close a remote project
                    71..=80 => {
                        if !client.dev_server_projects().is_empty() {
                            let project = client.dev_server_projects().choose(rng).unwrap().clone();
                            let first_root_name = root_name_for_project(&project, cx);
                            break ClientOperation::CloseRemoteProject {
                                project_root_name: first_root_name,
                            };
                        }
                    }

                    // Mutate project worktrees
                    81.. => match rng.random_range(0..100_u32) {
                        // Add a worktree to a local project
                        0..=50 => {
                            let Some(project) = client.local_projects().choose(rng).cloned() else {
                                continue;
                            };
                            let project_root_name = root_name_for_project(&project, cx);
                            let mut paths = client.fs().paths(false);
                            paths.remove(0);
                            let new_root_path = if paths.is_empty() || rng.random() {
                                Path::new(path!("/")).join(plan.next_root_dir_name())
                            } else {
                                paths.choose(rng).unwrap().clone()
                            };
                            break ClientOperation::AddWorktreeToProject {
                                project_root_name,
                                new_root_path,
                            };
                        }

                        // Add an entry to a worktree
                        _ => {
                            let Some(project) = choose_random_project(client, rng) else {
                                continue;
                            };
                            let project_root_name = root_name_for_project(&project, cx);
                            let is_local = project.read_with(cx, |project, _| project.is_local());
                            let worktree = project.read_with(cx, |project, cx| {
                                project
                                    .worktrees(cx)
                                    .filter(|worktree| {
                                        let worktree = worktree.read(cx);
                                        worktree.is_visible()
                                            && worktree.entries(false, 0).any(|e| e.is_file())
                                            && worktree.root_entry().is_some_and(|e| e.is_dir())
                                    })
                                    .choose(rng)
                            });
                            let Some(worktree) = worktree else { continue };
                            let is_dir = rng.random::<bool>();
                            let mut full_path =
                                worktree.read_with(cx, |w, _| w.root_name().to_rel_path_buf());
                            full_path.push(rel_path(&gen_file_name(rng)));
                            if !is_dir {
                                full_path.set_extension("rs");
                            }
                            break ClientOperation::CreateWorktreeEntry {
                                project_root_name,
                                is_local,
                                full_path,
                                is_dir,
                            };
                        }
                    },
                },

                // Query and mutate buffers
                60..=90 => {
                    let Some(project) = choose_random_project(client, rng) else {
                        continue;
                    };
                    let project_root_name = root_name_for_project(&project, cx);
                    let is_local = project.read_with(cx, |project, _| project.is_local());

                    match rng.random_range(0..100_u32) {
                        // Manipulate an existing buffer
                        0..=70 => {
                            let Some(buffer) = client
                                .buffers_for_project(&project)
                                .iter()
                                .choose(rng)
                                .cloned()
                            else {
                                continue;
                            };

                            let full_path = buffer.read_with(cx, |buffer, cx| {
                                let file = buffer.file().unwrap();
                                let worktree = project
                                    .read(cx)
                                    .worktree_for_id(file.worktree_id(cx), cx)
                                    .unwrap();
                                worktree
                                    .read(cx)
                                    .root_name()
                                    .join(file.path())
                                    .to_rel_path_buf()
                            });

                            match rng.random_range(0..100_u32) {
                                // Close the buffer
                                0..=15 => {
                                    break ClientOperation::CloseBuffer {
                                        project_root_name,
                                        is_local,
                                        full_path,
                                    };
                                }
                                // Save the buffer
                                16..=29 if buffer.read_with(cx, |b, _| b.is_dirty()) => {
                                    let detach = rng.random_bool(0.3);
                                    break ClientOperation::SaveBuffer {
                                        project_root_name,
                                        is_local,
                                        full_path,
                                        detach,
                                    };
                                }
                                // Edit the buffer
                                30..=69 => {
                                    let edits = buffer
                                        .read_with(cx, |buffer, _| buffer.get_random_edits(rng, 3));
                                    break ClientOperation::EditBuffer {
                                        project_root_name,
                                        is_local,
                                        full_path,
                                        edits,
                                    };
                                }
                                // Make an LSP request
                                _ => {
                                    let offset = buffer.read_with(cx, |buffer, _| {
                                        buffer.clip_offset(
                                            rng.random_range(0..=buffer.len()),
                                            language::Bias::Left,
                                        )
                                    });
                                    let detach = rng.random();
                                    break ClientOperation::RequestLspDataInBuffer {
                                        project_root_name,
                                        full_path,
                                        offset,
                                        is_local,
                                        kind: match rng.random_range(0..5_u32) {
                                            0 => LspRequestKind::Rename,
                                            1 => LspRequestKind::Highlights,
                                            2 => LspRequestKind::Definition,
                                            3 => LspRequestKind::CodeAction,
                                            4.. => LspRequestKind::Completion,
                                        },
                                        detach,
                                    };
                                }
                            }
                        }

                        71..=80 => {
                            let query = rng.random_range('a'..='z').to_string();
                            let detach = rng.random_bool(0.3);
                            break ClientOperation::SearchProject {
                                project_root_name,
                                is_local,
                                query,
                                detach,
                            };
                        }

                        // Open a buffer
                        81.. => {
                            let worktree = project.read_with(cx, |project, cx| {
                                project
                                    .worktrees(cx)
                                    .filter(|worktree| {
                                        let worktree = worktree.read(cx);
                                        worktree.is_visible()
                                            && worktree.entries(false, 0).any(|e| e.is_file())
                                    })
                                    .choose(rng)
                            });
                            let Some(worktree) = worktree else { continue };
                            let full_path = worktree.read_with(cx, |worktree, _| {
                                let entry = worktree
                                    .entries(false, 0)
                                    .filter(|e| e.is_file())
                                    .choose(rng)
                                    .unwrap();
                                if entry.path.as_ref().is_empty() {
                                    worktree.root_name().into()
                                } else {
                                    worktree.root_name().join(&entry.path)
                                }
                            });
                            break ClientOperation::OpenBuffer {
                                project_root_name,
                                is_local,
                                full_path: full_path.to_rel_path_buf(),
                            };
                        }
                    }
                }

                // Update a git related action
                91..=95 => {
                    break ClientOperation::GitOperation {
                        operation: generate_git_operation(rng, client),
                    };
                }

                // Create or update a file or directory
                96.. => {
                    let is_dir = rng.random::<bool>();
                    let content;
                    let mut path;
                    let dir_paths = client.fs().directories(false);

                    if is_dir {
                        content = String::new();
                        path = dir_paths.choose(rng).unwrap().clone();
                        path.push(gen_file_name(rng));
                    } else {
                        content = distr::Alphanumeric.sample_string(rng, 16);

                        // Create a new file or overwrite an existing file
                        let file_paths = client.fs().files();
                        if file_paths.is_empty() || rng.random_bool(0.5) {
                            path = dir_paths.choose(rng).unwrap().clone();
                            path.push(gen_file_name(rng));
                            path.set_extension("rs");
                        } else {
                            path = file_paths.choose(rng).unwrap().clone()
                        };
                    }
                    break ClientOperation::WriteFsEntry {
                        path,
                        is_dir,
                        content,
                    };
                }
            }
        }
    }

    async fn apply_operation(
        client: &TestClient,
        operation: ClientOperation,
        cx: &mut TestAppContext,
    ) -> Result<(), TestError> {
        match operation {
            ClientOperation::AcceptIncomingCall => {
                let active_call = cx.read(ActiveCall::global);
                if active_call.read_with(cx, |call, _| call.incoming().borrow().is_none()) {
                    Err(TestError::Inapplicable)?;
                }

                log::info!("{}: accepting incoming call", client.username);
                active_call
                    .update(cx, |call, cx| call.accept_incoming(cx))
                    .await?;
            }

            ClientOperation::RejectIncomingCall => {
                let active_call = cx.read(ActiveCall::global);
                if active_call.read_with(cx, |call, _| call.incoming().borrow().is_none()) {
                    Err(TestError::Inapplicable)?;
                }

                log::info!("{}: declining incoming call", client.username);
                active_call.update(cx, |call, cx| call.decline_incoming(cx))?;
            }

            ClientOperation::LeaveCall => {
                let active_call = cx.read(ActiveCall::global);
                if active_call.read_with(cx, |call, _| call.room().is_none()) {
                    Err(TestError::Inapplicable)?;
                }

                log::info!("{}: hanging up", client.username);
                active_call.update(cx, |call, cx| call.hang_up(cx)).await?;
            }

            ClientOperation::InviteContactToCall { user_id } => {
                let active_call = cx.read(ActiveCall::global);

                log::info!("{}: inviting {}", client.username, user_id,);
                active_call
                    .update(cx, |call, cx| call.invite(user_id.to_proto(), None, cx))
                    .await
                    .log_err();
            }

            ClientOperation::OpenLocalProject { first_root_name } => {
                log::info!(
                    "{}: opening local project at {:?}",
                    client.username,
                    first_root_name
                );

                let root_path = Path::new(path!("/")).join(&first_root_name);
                client.fs().create_dir(&root_path).await.unwrap();
                client
                    .fs()
                    .create_file(&root_path.join("main.rs"), Default::default())
                    .await
                    .unwrap();
                let project = client.build_local_project(root_path, cx).await.0;
                ensure_project_shared(&project, client, cx).await;
                client.local_projects_mut().push(project.clone());
            }

            ClientOperation::AddWorktreeToProject {
                project_root_name,
                new_root_path,
            } => {
                let project = project_for_root_name(client, &project_root_name, cx)
                    .ok_or(TestError::Inapplicable)?;

                log::info!(
                    "{}: finding/creating local worktree at {:?} to project with root path {}",
                    client.username,
                    new_root_path,
                    project_root_name
                );

                ensure_project_shared(&project, client, cx).await;
                if !client.fs().paths(false).contains(&new_root_path) {
                    client.fs().create_dir(&new_root_path).await.unwrap();
                }
                project
                    .update(cx, |project, cx| {
                        project.find_or_create_worktree(&new_root_path, true, cx)
                    })
                    .await
                    .unwrap();
            }

            ClientOperation::CloseRemoteProject { project_root_name } => {
                let project = project_for_root_name(client, &project_root_name, cx)
                    .ok_or(TestError::Inapplicable)?;

                log::info!(
                    "{}: closing remote project with root path {}",
                    client.username,
                    project_root_name,
                );

                let ix = client
                    .dev_server_projects()
                    .iter()
                    .position(|p| p == &project)
                    .unwrap();
                cx.update(|_| {
                    client.dev_server_projects_mut().remove(ix);
                    client.buffers().retain(|p, _| *p != project);
                    drop(project);
                });
            }

            ClientOperation::OpenRemoteProject {
                host_id,
                first_root_name,
            } => {
                let active_call = cx.read(ActiveCall::global);
                let project = active_call
                    .update(cx, |call, cx| {
                        let room = call.room().cloned()?;
                        let participant = room
                            .read(cx)
                            .remote_participants()
                            .get(&host_id.to_proto())?;
                        let project_id = participant
                            .projects
                            .iter()
                            .find(|project| project.worktree_root_names[0] == first_root_name)?
                            .id;
                        Some(room.update(cx, |room, cx| {
                            room.join_project(
                                project_id,
                                client.language_registry().clone(),
                                FakeFs::new(cx.background_executor().clone()),
                                cx,
                            )
                        }))
                    })
                    .ok_or(TestError::Inapplicable)?;

                log::info!(
                    "{}: joining remote project of user {}, root name {}",
                    client.username,
                    host_id,
                    first_root_name,
                );

                let project = project.await?;
                client.dev_server_projects_mut().push(project);
            }

            ClientOperation::CreateWorktreeEntry {
                project_root_name,
                is_local,
                full_path,
                is_dir,
            } => {
                let project = project_for_root_name(client, &project_root_name, cx)
                    .ok_or(TestError::Inapplicable)?;
                let project_path = project_path_for_full_path(&project, &full_path, cx)
                    .ok_or(TestError::Inapplicable)?;

                log::info!(
                    "{}: creating {} at path {:?} in {} project {}",
                    client.username,
                    if is_dir { "dir" } else { "file" },
                    full_path,
                    if is_local { "local" } else { "remote" },
                    project_root_name,
                );

                ensure_project_shared(&project, client, cx).await;
                project
                    .update(cx, |p, cx| p.create_entry(project_path, is_dir, cx))
                    .await?;
            }

            ClientOperation::OpenBuffer {
                project_root_name,
                is_local,
                full_path,
            } => {
                let project = project_for_root_name(client, &project_root_name, cx)
                    .ok_or(TestError::Inapplicable)?;
                let project_path = project_path_for_full_path(&project, &full_path, cx)
                    .ok_or(TestError::Inapplicable)?;

                log::info!(
                    "{}: opening buffer {:?} in {} project {}",
                    client.username,
                    full_path,
                    if is_local { "local" } else { "remote" },
                    project_root_name,
                );

                ensure_project_shared(&project, client, cx).await;
                let buffer = project
                    .update(cx, |project, cx| project.open_buffer(project_path, cx))
                    .await?;
                client.buffers_for_project(&project).insert(buffer);
            }

            ClientOperation::EditBuffer {
                project_root_name,
                is_local,
                full_path,
                edits,
            } => {
                let project = project_for_root_name(client, &project_root_name, cx)
                    .ok_or(TestError::Inapplicable)?;
                let buffer = buffer_for_full_path(client, &project, &full_path, cx)
                    .ok_or(TestError::Inapplicable)?;

                log::info!(
                    "{}: editing buffer {:?} in {} project {} with {:?}",
                    client.username,
                    full_path,
                    if is_local { "local" } else { "remote" },
                    project_root_name,
                    edits
                );

                ensure_project_shared(&project, client, cx).await;
                buffer.update(cx, |buffer, cx| {
                    let snapshot = buffer.snapshot();
                    buffer.edit(
                        edits.into_iter().map(|(range, text)| {
                            let start = snapshot.clip_offset(range.start, Bias::Left);
                            let end = snapshot.clip_offset(range.end, Bias::Right);
                            (start..end, text)
                        }),
                        None,
                        cx,
                    );
                });
            }

            ClientOperation::CloseBuffer {
                project_root_name,
                is_local,
                full_path,
            } => {
                let project = project_for_root_name(client, &project_root_name, cx)
                    .ok_or(TestError::Inapplicable)?;
                let buffer = buffer_for_full_path(client, &project, &full_path, cx)
                    .ok_or(TestError::Inapplicable)?;

                log::info!(
                    "{}: closing buffer {:?} in {} project {}",
                    client.username,
                    full_path,
                    if is_local { "local" } else { "remote" },
                    project_root_name
                );

                ensure_project_shared(&project, client, cx).await;
                cx.update(|_| {
                    client.buffers_for_project(&project).remove(&buffer);
                    drop(buffer);
                });
            }

            ClientOperation::SaveBuffer {
                project_root_name,
                is_local,
                full_path,
                detach,
            } => {
                let project = project_for_root_name(client, &project_root_name, cx)
                    .ok_or(TestError::Inapplicable)?;
                let buffer = buffer_for_full_path(client, &project, &full_path, cx)
                    .ok_or(TestError::Inapplicable)?;

                log::info!(
                    "{}: saving buffer {:?} in {} project {}, {}",
                    client.username,
                    full_path,
                    if is_local { "local" } else { "remote" },
                    project_root_name,
                    if detach { "detaching" } else { "awaiting" }
                );

                ensure_project_shared(&project, client, cx).await;
                let requested_version = buffer.read_with(cx, |buffer, _| buffer.version());
                let save =
                    project.update(cx, |project, cx| project.save_buffer(buffer.clone(), cx));
                let save = cx.spawn(|cx| async move {
                    save.await.context("save request failed")?;
                    assert!(
                        buffer
                            .read_with(&cx, |buffer, _| { buffer.saved_version().to_owned() })
                            .expect("App should not be dropped")
                            .observed_all(&requested_version)
                    );
                    anyhow::Ok(())
                });
                if detach {
                    cx.update(|cx| save.detach_and_log_err(cx));
                } else {
                    save.await?;
                }
            }

            ClientOperation::RequestLspDataInBuffer {
                project_root_name,
                is_local,
                full_path,
                offset,
                kind,
                detach,
            } => {
                let project = project_for_root_name(client, &project_root_name, cx)
                    .ok_or(TestError::Inapplicable)?;
                let buffer = buffer_for_full_path(client, &project, &full_path, cx)
                    .ok_or(TestError::Inapplicable)?;

                log::info!(
                    "{}: request LSP {:?} for buffer {:?} in {} project {}, {}",
                    client.username,
                    kind,
                    full_path,
                    if is_local { "local" } else { "remote" },
                    project_root_name,
                    if detach { "detaching" } else { "awaiting" }
                );

                use futures::{FutureExt as _, TryFutureExt as _};
                let offset = buffer.read_with(cx, |b, _| b.clip_offset(offset, Bias::Left));

                let process_lsp_request = project.update(cx, |project, cx| match kind {
                    LspRequestKind::Rename => project
                        .prepare_rename(buffer, offset, cx)
                        .map_ok(|_| ())
                        .boxed(),
                    LspRequestKind::Completion => project
                        .completions(&buffer, offset, DEFAULT_COMPLETION_CONTEXT, cx)
                        .map_ok(|_| ())
                        .boxed(),
                    LspRequestKind::CodeAction => project
                        .code_actions(&buffer, offset..offset, None, cx)
                        .map(|_| Ok(()))
                        .boxed(),
                    LspRequestKind::Definition => project
                        .definitions(&buffer, offset, cx)
                        .map_ok(|_| ())
                        .boxed(),
                    LspRequestKind::Highlights => project
                        .document_highlights(&buffer, offset, cx)
                        .map_ok(|_| ())
                        .boxed(),
                });
                let request = cx.foreground_executor().spawn(process_lsp_request);
                if detach {
                    request.detach();
                } else {
                    request.await?;
                }
            }

            ClientOperation::SearchProject {
                project_root_name,
                is_local,
                query,
                detach,
            } => {
                let project = project_for_root_name(client, &project_root_name, cx)
                    .ok_or(TestError::Inapplicable)?;

                log::info!(
                    "{}: search {} project {} for {:?}, {}",
                    client.username,
                    if is_local { "local" } else { "remote" },
                    project_root_name,
                    query,
                    if detach { "detaching" } else { "awaiting" }
                );

                let search = project.update(cx, |project, cx| {
                    project.search(
                        SearchQuery::text(
                            query,
                            false,
                            false,
                            false,
                            Default::default(),
                            Default::default(),
                            false,
                            None,
                        )
                        .unwrap(),
                        cx,
                    )
                });
                drop(project);
                let search = cx.executor().spawn(async move {
                    let mut results = HashMap::default();
                    while let Ok(result) = search.recv().await {
                        if let SearchResult::Buffer { buffer, ranges } = result {
                            results.entry(buffer).or_insert(ranges);
                        }
                    }
                    results
                });
                search.await;
            }

            ClientOperation::WriteFsEntry {
                path,
                is_dir,
                content,
            } => {
                if !client
                    .fs()
                    .directories(false)
                    .contains(&path.parent().unwrap().to_owned())
                {
                    return Err(TestError::Inapplicable);
                }

                if is_dir {
                    log::info!("{}: creating dir at {:?}", client.username, path);
                    client.fs().create_dir(&path).await.unwrap();
                } else {
                    let exists = client.fs().metadata(&path).await?.is_some();
                    let verb = if exists { "updating" } else { "creating" };
                    log::info!("{}: {} file at {:?}", verb, client.username, path);

                    client
                        .fs()
                        .save(&path, &content.as_str().into(), text::LineEnding::Unix)
                        .await
                        .unwrap();
                }
            }

            ClientOperation::GitOperation { operation } => match operation {
                GitOperation::WriteGitIndex {
                    repo_path,
                    contents,
                } => {
                    if !client.fs().directories(false).contains(&repo_path) {
                        return Err(TestError::Inapplicable);
                    }

                    for (path, _) in contents.iter() {
                        if !client
                            .fs()
                            .files()
                            .contains(&repo_path.join(path.as_std_path()))
                        {
                            return Err(TestError::Inapplicable);
                        }
                    }

                    log::info!(
                        "{}: writing git index for repo {:?}: {:?}",
                        client.username,
                        repo_path,
                        contents
                    );

                    let dot_git_dir = repo_path.join(".git");
                    let contents = contents
                        .iter()
                        .map(|(path, contents)| (path.as_unix_str(), contents.clone()))
                        .collect::<Vec<_>>();
                    if client.fs().metadata(&dot_git_dir).await?.is_none() {
                        client.fs().create_dir(&dot_git_dir).await?;
                    }
                    client.fs().set_index_for_repo(&dot_git_dir, &contents);
                }
                GitOperation::WriteGitBranch {
                    repo_path,
                    new_branch,
                } => {
                    if !client.fs().directories(false).contains(&repo_path) {
                        return Err(TestError::Inapplicable);
                    }

                    log::info!(
                        "{}: writing git branch for repo {:?}: {:?}",
                        client.username,
                        repo_path,
                        new_branch
                    );

                    let dot_git_dir = repo_path.join(".git");
                    if client.fs().metadata(&dot_git_dir).await?.is_none() {
                        client.fs().create_dir(&dot_git_dir).await?;
                    }
                    client
                        .fs()
                        .set_branch_name(&dot_git_dir, new_branch.clone());
                }
                GitOperation::WriteGitStatuses {
                    repo_path,
                    statuses,
                } => {
                    if !client.fs().directories(false).contains(&repo_path) {
                        return Err(TestError::Inapplicable);
                    }
                    for (path, _) in statuses.iter() {
                        if !client
                            .fs()
                            .files()
                            .contains(&repo_path.join(path.as_std_path()))
                        {
                            return Err(TestError::Inapplicable);
                        }
                    }

                    log::info!(
                        "{}: writing git statuses for repo {:?}: {:?}",
                        client.username,
                        repo_path,
                        statuses
                    );

                    let dot_git_dir = repo_path.join(".git");

                    let statuses = statuses
                        .iter()
                        .map(|(path, val)| (path.as_unix_str(), *val))
                        .collect::<Vec<_>>();

                    if client.fs().metadata(&dot_git_dir).await?.is_none() {
                        client.fs().create_dir(&dot_git_dir).await?;
                    }

                    client
                        .fs()
                        .set_status_for_repo(&dot_git_dir, statuses.as_slice());
                }
            },
        }
        Ok(())
    }

    async fn on_client_added(client: &Rc<TestClient>, _: &mut TestAppContext) {
        client.language_registry().add(Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        )));
        client.language_registry().register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: "the-fake-language-server",
                capabilities: lsp::LanguageServer::full_capabilities(),
                initializer: Some(Box::new({
                    let fs = client.app_state.fs.clone();
                    move |fake_server: &mut FakeLanguageServer| {
                        fake_server.set_request_handler::<lsp::request::Completion, _, _>(
                            |_, _| async move {
                                Ok(Some(lsp::CompletionResponse::Array(vec![
                                    lsp::CompletionItem {
                                        text_edit: Some(lsp::CompletionTextEdit::Edit(
                                            lsp::TextEdit {
                                                range: lsp::Range::new(
                                                    lsp::Position::new(0, 0),
                                                    lsp::Position::new(0, 0),
                                                ),
                                                new_text: "the-new-text".to_string(),
                                            },
                                        )),
                                        ..Default::default()
                                    },
                                ])))
                            },
                        );

                        fake_server.set_request_handler::<lsp::request::CodeActionRequest, _, _>(
                            |_, _| async move {
                                Ok(Some(vec![lsp::CodeActionOrCommand::CodeAction(
                                    lsp::CodeAction {
                                        title: "the-code-action".to_string(),
                                        ..Default::default()
                                    },
                                )]))
                            },
                        );

                        fake_server
                            .set_request_handler::<lsp::request::PrepareRenameRequest, _, _>(
                                |params, _| async move {
                                    Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
                                        params.position,
                                        params.position,
                                    ))))
                                },
                            );

                        fake_server.set_request_handler::<lsp::request::GotoDefinition, _, _>({
                            let fs = fs.clone();
                            move |_, cx| {
                                let background = cx.background_executor();
                                let mut rng = background.rng();
                                let count = rng.random_range::<usize, _>(1..3);
                                let files = fs.as_fake().files();
                                let files = (0..count)
                                    .map(|_| files.choose(&mut rng).unwrap().clone())
                                    .collect::<Vec<_>>();
                                async move {
                                    log::info!("LSP: Returning definitions in files {:?}", &files);
                                    Ok(Some(lsp::GotoDefinitionResponse::Array(
                                        files
                                            .into_iter()
                                            .map(|file| lsp::Location {
                                                uri: lsp::Uri::from_file_path(file).unwrap(),
                                                range: Default::default(),
                                            })
                                            .collect(),
                                    )))
                                }
                            }
                        });

                        fake_server
                            .set_request_handler::<lsp::request::DocumentHighlightRequest, _, _>(
                                move |_, cx| {
                                    let mut highlights = Vec::new();
                                    let background = cx.background_executor();
                                    let mut rng = background.rng();

                                    let highlight_count = rng.random_range(1..=5);
                                    for _ in 0..highlight_count {
                                        let start_row = rng.random_range(0..100);
                                        let start_column = rng.random_range(0..100);
                                        let end_row = rng.random_range(0..100);
                                        let end_column = rng.random_range(0..100);
                                        let start = PointUtf16::new(start_row, start_column);
                                        let end = PointUtf16::new(end_row, end_column);
                                        let range =
                                            if start > end { end..start } else { start..end };
                                        highlights.push(lsp::DocumentHighlight {
                                            range: range_to_lsp(range.clone()).unwrap(),
                                            kind: Some(lsp::DocumentHighlightKind::READ),
                                        });
                                    }
                                    highlights.sort_unstable_by_key(|highlight| {
                                        (highlight.range.start, highlight.range.end)
                                    });
                                    async move { Ok(Some(highlights)) }
                                },
                            );
                    }
                })),
                ..Default::default()
            },
        );
    }

    async fn on_quiesce(_: &mut TestServer, clients: &mut [(Rc<TestClient>, TestAppContext)]) {
        for (client, client_cx) in clients.iter() {
            for guest_project in client.dev_server_projects().iter() {
                guest_project.read_with(client_cx, |guest_project, cx| {
                        let host_project = clients.iter().find_map(|(client, cx)| {
                            let project = client
                                .local_projects()
                                .iter()
                                .find(|host_project| {
                                    host_project.read_with(cx, |host_project, _| {
                                        host_project.remote_id() == guest_project.remote_id()
                                    })
                                })?
                                .clone();
                            Some((project, cx))
                        });

                        if !guest_project.is_disconnected(cx)
                            && let Some((host_project, host_cx)) = host_project {
                                let host_worktree_snapshots =
                                    host_project.read_with(host_cx, |host_project, cx| {
                                        host_project
                                            .worktrees(cx)
                                            .map(|worktree| {
                                                let worktree = worktree.read(cx);
                                                (worktree.id(), worktree.snapshot())
                                            })
                                            .collect::<BTreeMap<_, _>>()
                                    });
                                let guest_worktree_snapshots = guest_project
                                    .worktrees(cx)
                                    .map(|worktree| {
                                        let worktree = worktree.read(cx);
                                        (worktree.id(), worktree.snapshot())
                                    })
                                    .collect::<BTreeMap<_, _>>();
                                let host_repository_snapshots = host_project.read_with(host_cx, |host_project, cx| {
                                    host_project.git_store().read(cx).repo_snapshots(cx)
                                });
                                let guest_repository_snapshots = guest_project.git_store().read(cx).repo_snapshots(cx);

                                assert_eq!(
                                    guest_worktree_snapshots.values().map(|w| w.abs_path()).collect::<Vec<_>>(),
                                    host_worktree_snapshots.values().map(|w| w.abs_path()).collect::<Vec<_>>(),
                                    "{} has different worktrees than the host for project {:?}",
                                    client.username, guest_project.remote_id(),
                                );

                                assert_eq!(
                                    guest_repository_snapshots.values().collect::<Vec<_>>(),
                                    host_repository_snapshots.values().collect::<Vec<_>>(),
                                    "{} has different repositories than the host for project {:?}",
                                    client.username, guest_project.remote_id(),
                                );

                                for (id, host_snapshot) in &host_worktree_snapshots {
                                    let guest_snapshot = &guest_worktree_snapshots[id];
                                    assert_eq!(
                                        guest_snapshot.root_name(),
                                        host_snapshot.root_name(),
                                        "{} has different root name than the host for worktree {}, project {:?}",
                                        client.username,
                                        id,
                                        guest_project.remote_id(),
                                    );
                                    assert_eq!(
                                        guest_snapshot.abs_path(),
                                        host_snapshot.abs_path(),
                                        "{} has different abs path than the host for worktree {}, project: {:?}",
                                        client.username,
                                        id,
                                        guest_project.remote_id(),
                                    );
                                    assert_eq!(
                                        guest_snapshot.entries(false, 0).map(null_out_entry_size).collect::<Vec<_>>(),
                                        host_snapshot.entries(false, 0).map(null_out_entry_size).collect::<Vec<_>>(),
                                        "{} has different snapshot than the host for worktree {:?} ({:?}) and project {:?}",
                                        client.username,
                                        host_snapshot.abs_path(),
                                        id,
                                        guest_project.remote_id(),
                                    );
                                    assert_eq!(guest_snapshot.scan_id(), host_snapshot.scan_id(),
                                        "{} has different scan id than the host for worktree {:?} and project {:?}",
                                        client.username,
                                        host_snapshot.abs_path(),
                                        guest_project.remote_id(),
                                    );
                                }
                            }

                        for buffer in guest_project.opened_buffers(cx) {
                            let buffer = buffer.read(cx);
                            assert_eq!(
                                buffer.deferred_ops_len(),
                                0,
                                "{} has deferred operations for buffer {:?} in project {:?}",
                                client.username,
                                buffer.file().unwrap().full_path(cx),
                                guest_project.remote_id(),
                            );
                        }
                    });

                // A hack to work around a hack in
                // https://github.com/zed-industries/zed/pull/16696 that wasn't
                // detected until we upgraded the rng crate. This whole crate is
                // going away with DeltaDB soon, so we hold our nose and
                // continue.
                fn null_out_entry_size(entry: &project::Entry) -> project::Entry {
                    project::Entry {
                        size: 0,
                        ..entry.clone()
                    }
                }
            }

            let buffers = client.buffers().clone();
            for (guest_project, guest_buffers) in &buffers {
                let project_id = if guest_project.read_with(client_cx, |project, cx| {
                    project.is_local() || project.is_disconnected(cx)
                }) {
                    continue;
                } else {
                    guest_project
                        .read_with(client_cx, |project, _| project.remote_id())
                        .unwrap()
                };
                let guest_user_id = client.user_id().unwrap();

                let host_project = clients.iter().find_map(|(client, cx)| {
                    let project = client
                        .local_projects()
                        .iter()
                        .find(|host_project| {
                            host_project.read_with(cx, |host_project, _| {
                                host_project.remote_id() == Some(project_id)
                            })
                        })?
                        .clone();
                    Some((client.user_id().unwrap(), project, cx))
                });

                let (host_user_id, host_project, host_cx) =
                    if let Some((host_user_id, host_project, host_cx)) = host_project {
                        (host_user_id, host_project, host_cx)
                    } else {
                        continue;
                    };

                for guest_buffer in guest_buffers {
                    let buffer_id =
                        guest_buffer.read_with(client_cx, |buffer, _| buffer.remote_id());
                    let host_buffer = host_project.read_with(host_cx, |project, cx| {
                        project.buffer_for_id(buffer_id, cx).unwrap_or_else(|| {
                            panic!(
                                "host does not have buffer for guest:{}, peer:{:?}, id:{}",
                                client.username,
                                client.peer_id(),
                                buffer_id
                            )
                        })
                    });
                    let path = host_buffer
                        .read_with(host_cx, |buffer, cx| buffer.file().unwrap().full_path(cx));

                    assert_eq!(
                        guest_buffer.read_with(client_cx, |buffer, _| buffer.deferred_ops_len()),
                        0,
                        "{}, buffer {}, path {:?} has deferred operations",
                        client.username,
                        buffer_id,
                        path,
                    );
                    assert_eq!(
                        guest_buffer.read_with(client_cx, |buffer, _| buffer.text()),
                        host_buffer.read_with(host_cx, |buffer, _| buffer.text()),
                        "{}, buffer {}, path {:?}, differs from the host's buffer",
                        client.username,
                        buffer_id,
                        path
                    );

                    let host_file = host_buffer.read_with(host_cx, |b, _| b.file().cloned());
                    let guest_file = guest_buffer.read_with(client_cx, |b, _| b.file().cloned());
                    match (host_file, guest_file) {
                        (Some(host_file), Some(guest_file)) => {
                            assert_eq!(guest_file.path(), host_file.path());
                            assert_eq!(
                                guest_file.disk_state(),
                                host_file.disk_state(),
                                "guest {} disk_state does not match host {} for path {:?} in project {}",
                                guest_user_id,
                                host_user_id,
                                guest_file.path(),
                                project_id,
                            );
                        }
                        (None, None) => {}
                        (None, _) => panic!("host's file is None, guest's isn't"),
                        (_, None) => panic!("guest's file is None, hosts's isn't"),
                    }

                    let host_diff_base = host_project.read_with(host_cx, |project, cx| {
                        project
                            .git_store()
                            .read(cx)
                            .get_unstaged_diff(host_buffer.read(cx).remote_id(), cx)
                            .unwrap()
                            .read(cx)
                            .base_text_string()
                    });
                    let guest_diff_base = guest_project.read_with(client_cx, |project, cx| {
                        project
                            .git_store()
                            .read(cx)
                            .get_unstaged_diff(guest_buffer.read(cx).remote_id(), cx)
                            .unwrap()
                            .read(cx)
                            .base_text_string()
                    });
                    assert_eq!(
                        guest_diff_base, host_diff_base,
                        "guest {} diff base does not match host's for path {path:?} in project {project_id}",
                        client.username
                    );

                    let host_saved_version =
                        host_buffer.read_with(host_cx, |b, _| b.saved_version().clone());
                    let guest_saved_version =
                        guest_buffer.read_with(client_cx, |b, _| b.saved_version().clone());
                    assert_eq!(
                        guest_saved_version, host_saved_version,
                        "guest {} saved version does not match host's for path {path:?} in project {project_id}",
                        client.username
                    );

                    let host_is_dirty = host_buffer.read_with(host_cx, |b, _| b.is_dirty());
                    let guest_is_dirty = guest_buffer.read_with(client_cx, |b, _| b.is_dirty());
                    assert_eq!(
                        guest_is_dirty, host_is_dirty,
                        "guest {} dirty state does not match host's for path {path:?} in project {project_id}",
                        client.username
                    );

                    let host_saved_mtime = host_buffer.read_with(host_cx, |b, _| b.saved_mtime());
                    let guest_saved_mtime =
                        guest_buffer.read_with(client_cx, |b, _| b.saved_mtime());
                    assert_eq!(
                        guest_saved_mtime, host_saved_mtime,
                        "guest {} saved mtime does not match host's for path {path:?} in project {project_id}",
                        client.username
                    );

                    let host_is_dirty = host_buffer.read_with(host_cx, |b, _| b.is_dirty());
                    let guest_is_dirty = guest_buffer.read_with(client_cx, |b, _| b.is_dirty());
                    assert_eq!(
                        guest_is_dirty, host_is_dirty,
                        "guest {} dirty status does not match host's for path {path:?} in project {project_id}",
                        client.username
                    );

                    let host_has_conflict = host_buffer.read_with(host_cx, |b, _| b.has_conflict());
                    let guest_has_conflict =
                        guest_buffer.read_with(client_cx, |b, _| b.has_conflict());
                    assert_eq!(
                        guest_has_conflict, host_has_conflict,
                        "guest {} conflict status does not match host's for path {path:?} in project {project_id}",
                        client.username
                    );
                }
            }
        }
    }
}

fn generate_git_operation(rng: &mut StdRng, client: &TestClient) -> GitOperation {
    fn generate_file_paths(
        repo_path: &Path,
        rng: &mut StdRng,
        client: &TestClient,
    ) -> Vec<RelPathBuf> {
        let mut paths = client
            .fs()
            .files()
            .into_iter()
            .filter(|path| path.starts_with(repo_path))
            .collect::<Vec<_>>();

        let count = rng.random_range(0..=paths.len());
        paths.shuffle(rng);
        paths.truncate(count);

        paths
            .iter()
            .map(|path| {
                RelPath::new(path.strip_prefix(repo_path).unwrap(), PathStyle::local())
                    .unwrap()
                    .to_rel_path_buf()
            })
            .collect::<Vec<_>>()
    }

    let repo_path = client.fs().directories(false).choose(rng).unwrap().clone();

    match rng.random_range(0..100_u32) {
        0..=25 => {
            let file_paths = generate_file_paths(&repo_path, rng, client);

            let contents = file_paths
                .into_iter()
                .map(|path| (path, distr::Alphanumeric.sample_string(rng, 16)))
                .collect();

            GitOperation::WriteGitIndex {
                repo_path,
                contents,
            }
        }
        26..=63 => {
            let new_branch =
                (rng.random_range(0..10) > 3).then(|| distr::Alphanumeric.sample_string(rng, 8));

            GitOperation::WriteGitBranch {
                repo_path,
                new_branch,
            }
        }
        64..=100 => {
            let file_paths = generate_file_paths(&repo_path, rng, client);
            let statuses = file_paths
                .into_iter()
                .map(|path| (path, gen_status(rng)))
                .collect::<Vec<_>>();
            GitOperation::WriteGitStatuses {
                repo_path,
                statuses,
            }
        }
        _ => unreachable!(),
    }
}

fn buffer_for_full_path(
    client: &TestClient,
    project: &Entity<Project>,
    full_path: &RelPath,
    cx: &TestAppContext,
) -> Option<Entity<language::Buffer>> {
    client
        .buffers_for_project(project)
        .iter()
        .find(|buffer| {
            buffer.read_with(cx, |buffer, cx| {
                let file = buffer.file().unwrap();
                let Some(worktree) = project.read(cx).worktree_for_id(file.worktree_id(cx), cx)
                else {
                    return false;
                };
                worktree.read(cx).root_name().join(&file.path()).as_ref() == full_path
            })
        })
        .cloned()
}

fn project_for_root_name(
    client: &TestClient,
    root_name: &str,
    cx: &TestAppContext,
) -> Option<Entity<Project>> {
    if let Some(ix) = project_ix_for_root_name(client.local_projects().deref(), root_name, cx) {
        return Some(client.local_projects()[ix].clone());
    }
    if let Some(ix) = project_ix_for_root_name(client.dev_server_projects().deref(), root_name, cx)
    {
        return Some(client.dev_server_projects()[ix].clone());
    }
    None
}

fn project_ix_for_root_name(
    projects: &[Entity<Project>],
    root_name: &str,
    cx: &TestAppContext,
) -> Option<usize> {
    projects.iter().position(|project| {
        project.read_with(cx, |project, cx| {
            let worktree = project.visible_worktrees(cx).next().unwrap();
            worktree.read(cx).root_name() == root_name
        })
    })
}

fn root_name_for_project(project: &Entity<Project>, cx: &TestAppContext) -> String {
    project.read_with(cx, |project, cx| {
        project
            .visible_worktrees(cx)
            .next()
            .unwrap()
            .read(cx)
            .root_name_str()
            .to_string()
    })
}

fn project_path_for_full_path(
    project: &Entity<Project>,
    full_path: &RelPath,
    cx: &TestAppContext,
) -> Option<ProjectPath> {
    let mut components = full_path.components();
    let root_name = components.next().unwrap();
    let path = components.rest().into();
    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).find_map(|worktree| {
            let worktree = worktree.read(cx);
            if worktree.root_name_str() == root_name {
                Some(worktree.id())
            } else {
                None
            }
        })
    })?;
    Some(ProjectPath { worktree_id, path })
}

async fn ensure_project_shared(
    project: &Entity<Project>,
    client: &TestClient,
    cx: &mut TestAppContext,
) {
    let first_root_name = root_name_for_project(project, cx);
    let active_call = cx.read(ActiveCall::global);
    if active_call.read_with(cx, |call, _| call.room().is_some())
        && project.read_with(cx, |project, _| project.is_local() && !project.is_shared())
    {
        match active_call
            .update(cx, |call, cx| call.share_project(project.clone(), cx))
            .await
        {
            Ok(project_id) => {
                log::info!(
                    "{}: shared project {} with id {}",
                    client.username,
                    first_root_name,
                    project_id
                );
            }
            Err(error) => {
                log::error!(
                    "{}: error sharing project {}: {:?}",
                    client.username,
                    first_root_name,
                    error
                );
            }
        }
    }
}

fn choose_random_project(client: &TestClient, rng: &mut StdRng) -> Option<Entity<Project>> {
    client
        .local_projects()
        .deref()
        .iter()
        .chain(client.dev_server_projects().iter())
        .choose(rng)
        .cloned()
}

fn gen_file_name(rng: &mut StdRng) -> String {
    let mut name = String::new();
    for _ in 0..10 {
        let letter = rng.random_range('a'..='z');
        name.push(letter);
    }
    name
}

fn gen_status(rng: &mut StdRng) -> FileStatus {
    fn gen_tracked_status(rng: &mut StdRng) -> TrackedStatus {
        match rng.random_range(0..3) {
            0 => TrackedStatus {
                index_status: StatusCode::Unmodified,
                worktree_status: StatusCode::Unmodified,
            },
            1 => TrackedStatus {
                index_status: StatusCode::Modified,
                worktree_status: StatusCode::Modified,
            },
            2 => TrackedStatus {
                index_status: StatusCode::Added,
                worktree_status: StatusCode::Modified,
            },
            3 => TrackedStatus {
                index_status: StatusCode::Added,
                worktree_status: StatusCode::Unmodified,
            },
            _ => unreachable!(),
        }
    }

    fn gen_unmerged_status_code(rng: &mut StdRng) -> UnmergedStatusCode {
        match rng.random_range(0..3) {
            0 => UnmergedStatusCode::Updated,
            1 => UnmergedStatusCode::Added,
            2 => UnmergedStatusCode::Deleted,
            _ => unreachable!(),
        }
    }

    match rng.random_range(0..2) {
        0 => FileStatus::Unmerged(UnmergedStatus {
            first_head: gen_unmerged_status_code(rng),
            second_head: gen_unmerged_status_code(rng),
        }),
        1 => FileStatus::Tracked(gen_tracked_status(rng)),
        _ => unreachable!(),
    }
}
