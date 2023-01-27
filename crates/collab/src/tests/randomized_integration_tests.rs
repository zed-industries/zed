use crate::{
    db::{self, NewUserParams},
    rpc::{CLEANUP_TIMEOUT, RECONNECT_TIMEOUT},
    tests::{TestClient, TestServer},
};
use anyhow::{anyhow, Result};
use call::ActiveCall;
use client::RECEIVE_TIMEOUT;
use collections::BTreeMap;
use fs::{FakeFs, Fs as _};
use futures::StreamExt as _;
use gpui::{executor::Deterministic, ModelHandle, TestAppContext};
use language::{range_to_lsp, FakeLspAdapter, Language, LanguageConfig, PointUtf16, Rope};
use lsp::FakeLanguageServer;
use parking_lot::Mutex;
use project::{search::SearchQuery, Project};
use rand::{
    distributions::{Alphanumeric, DistString},
    prelude::*,
};
use settings::Settings;
use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};

#[gpui::test(iterations = 100)]
async fn test_random_collaboration(
    cx: &mut TestAppContext,
    deterministic: Arc<Deterministic>,
    rng: StdRng,
) {
    deterministic.forbid_parking();
    let rng = Arc::new(Mutex::new(rng));

    let max_peers = env::var("MAX_PEERS")
        .map(|i| i.parse().expect("invalid `MAX_PEERS` variable"))
        .unwrap_or(5);

    let max_operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let mut server = TestServer::start(&deterministic).await;
    let db = server.app_state.db.clone();

    let mut available_users = Vec::new();
    for ix in 0..max_peers {
        let username = format!("user-{}", ix + 1);
        let user_id = db
            .create_user(
                &format!("{username}@example.com"),
                false,
                NewUserParams {
                    github_login: username.clone(),
                    github_user_id: (ix + 1) as i32,
                    invite_count: 0,
                },
            )
            .await
            .unwrap()
            .user_id;
        available_users.push((user_id, username));
    }

    for (ix, (user_id_a, _)) in available_users.iter().enumerate() {
        for (user_id_b, _) in &available_users[ix + 1..] {
            server
                .app_state
                .db
                .send_contact_request(*user_id_a, *user_id_b)
                .await
                .unwrap();
            server
                .app_state
                .db
                .respond_to_contact_request(*user_id_b, *user_id_a, true)
                .await
                .unwrap();
        }
    }

    let mut clients = Vec::new();
    let mut user_ids = Vec::new();
    let mut op_start_signals = Vec::new();
    let mut next_entity_id = 100000;
    let allow_server_restarts = rng.lock().gen_bool(0.7);
    let allow_client_reconnection = rng.lock().gen_bool(0.7);
    let allow_client_disconnection = rng.lock().gen_bool(0.1);

    let mut operations = 0;
    while operations < max_operations {
        let distribution = rng.lock().gen_range(0..100);
        match distribution {
            0..=19 if !available_users.is_empty() => {
                let client_ix = rng.lock().gen_range(0..available_users.len());
                let (_, username) = available_users.remove(client_ix);
                log::info!("Adding new connection for {}", username);
                next_entity_id += 100000;
                let mut client_cx = TestAppContext::new(
                    cx.foreground_platform(),
                    cx.platform(),
                    deterministic.build_foreground(next_entity_id),
                    deterministic.build_background(),
                    cx.font_cache(),
                    cx.leak_detector(),
                    next_entity_id,
                    cx.function_name.clone(),
                );

                client_cx.update(|cx| cx.set_global(Settings::test(cx)));

                let op_start_signal = futures::channel::mpsc::unbounded();
                let client = server.create_client(&mut client_cx, &username).await;
                user_ids.push(client.current_user_id(&client_cx));
                op_start_signals.push(op_start_signal.0);
                clients.push(client_cx.foreground().spawn(simulate_client(
                    client,
                    op_start_signal.1,
                    allow_client_disconnection,
                    rng.clone(),
                    client_cx,
                )));

                log::info!("Added connection for {}", username);
                operations += 1;
            }

            20..=24 if clients.len() > 1 && allow_client_disconnection => {
                let client_ix = rng.lock().gen_range(1..clients.len());
                log::info!(
                    "Simulating full disconnection of user {}",
                    user_ids[client_ix]
                );
                let removed_user_id = user_ids.remove(client_ix);
                let user_connection_ids = server
                    .connection_pool
                    .lock()
                    .user_connection_ids(removed_user_id)
                    .collect::<Vec<_>>();
                assert_eq!(user_connection_ids.len(), 1);
                let removed_peer_id = user_connection_ids[0].into();
                let client = clients.remove(client_ix);
                op_start_signals.remove(client_ix);
                server.forbid_connections();
                server.disconnect_client(removed_peer_id);
                deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
                deterministic.start_waiting();
                log::info!("Waiting for user {} to exit...", removed_user_id);
                let (client, mut client_cx) = client.await;
                deterministic.finish_waiting();
                server.allow_connections();

                for project in &client.remote_projects {
                    project.read_with(&client_cx, |project, _| {
                        assert!(
                            project.is_read_only(),
                            "project {:?} should be read only",
                            project.remote_id()
                        )
                    });
                }
                for user_id in &user_ids {
                    let contacts = server.app_state.db.get_contacts(*user_id).await.unwrap();
                    let pool = server.connection_pool.lock();
                    for contact in contacts {
                        if let db::Contact::Accepted { user_id, busy, .. } = contact {
                            if user_id == removed_user_id {
                                assert!(!pool.is_user_online(user_id));
                                assert!(!busy);
                            }
                        }
                    }
                }

                log::info!("{} removed", client.username);
                available_users.push((removed_user_id, client.username.clone()));
                client_cx.update(|cx| {
                    cx.clear_globals();
                    cx.set_global(Settings::test(cx));
                    drop(client);
                });

                operations += 1;
            }

            25..=29 if clients.len() > 1 && allow_client_reconnection => {
                let client_ix = rng.lock().gen_range(1..clients.len());
                let user_id = user_ids[client_ix];
                log::info!("Simulating temporary disconnection of user {}", user_id);
                let user_connection_ids = server
                    .connection_pool
                    .lock()
                    .user_connection_ids(user_id)
                    .collect::<Vec<_>>();
                assert_eq!(user_connection_ids.len(), 1);
                let peer_id = user_connection_ids[0].into();
                server.disconnect_client(peer_id);
                deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
                operations += 1;
            }

            30..=34 if allow_server_restarts => {
                log::info!("Simulating server restart");
                server.reset().await;
                deterministic.advance_clock(RECEIVE_TIMEOUT);
                server.start().await.unwrap();
                deterministic.advance_clock(CLEANUP_TIMEOUT);
                let environment = &server.app_state.config.zed_environment;
                let stale_room_ids = server
                    .app_state
                    .db
                    .stale_room_ids(environment, server.id())
                    .await
                    .unwrap();
                assert_eq!(stale_room_ids, vec![]);
            }

            _ if !op_start_signals.is_empty() => {
                while operations < max_operations && rng.lock().gen_bool(0.7) {
                    op_start_signals
                        .choose(&mut *rng.lock())
                        .unwrap()
                        .unbounded_send(())
                        .unwrap();
                    operations += 1;
                }

                if rng.lock().gen_bool(0.8) {
                    deterministic.run_until_parked();
                }
            }
            _ => {}
        }
    }

    drop(op_start_signals);
    deterministic.start_waiting();
    let clients = futures::future::join_all(clients).await;
    deterministic.finish_waiting();
    deterministic.run_until_parked();

    for (client, client_cx) in &clients {
        for guest_project in &client.remote_projects {
            guest_project.read_with(client_cx, |guest_project, cx| {
                let host_project = clients.iter().find_map(|(client, cx)| {
                    let project = client.local_projects.iter().find(|host_project| {
                        host_project.read_with(cx, |host_project, _| {
                            host_project.remote_id() == guest_project.remote_id()
                        })
                    })?;
                    Some((project, cx))
                });

                if !guest_project.is_read_only() {
                    if let Some((host_project, host_cx)) = host_project {
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

                        assert_eq!(
                            guest_worktree_snapshots.keys().collect::<Vec<_>>(),
                            host_worktree_snapshots.keys().collect::<Vec<_>>(),
                            "{} has different worktrees than the host",
                            client.username
                        );

                        for (id, host_snapshot) in &host_worktree_snapshots {
                            let guest_snapshot = &guest_worktree_snapshots[id];
                            assert_eq!(
                                guest_snapshot.root_name(),
                                host_snapshot.root_name(),
                                "{} has different root name than the host for worktree {}",
                                client.username,
                                id
                            );
                            assert_eq!(
                                guest_snapshot.abs_path(),
                                host_snapshot.abs_path(),
                                "{} has different abs path than the host for worktree {}",
                                client.username,
                                id
                            );
                            assert_eq!(
                                guest_snapshot.entries(false).collect::<Vec<_>>(),
                                host_snapshot.entries(false).collect::<Vec<_>>(),
                                "{} has different snapshot than the host for worktree {} ({:?}) and project {:?}",
                                client.username,
                                id,
                                host_snapshot.abs_path(),
                                host_project.read_with(host_cx, |project, _| project.remote_id())
                            );
                            assert_eq!(guest_snapshot.scan_id(), host_snapshot.scan_id());
                        }
                    }
                }

                guest_project.check_invariants(cx);
            });
        }

        for (guest_project, guest_buffers) in &client.buffers {
            let project_id = if guest_project.read_with(client_cx, |project, _| {
                project.is_local() || project.is_read_only()
            }) {
                continue;
            } else {
                guest_project
                    .read_with(client_cx, |project, _| project.remote_id())
                    .unwrap()
            };
            let guest_user_id = client.user_id().unwrap();

            let host_project = clients.iter().find_map(|(client, cx)| {
                let project = client.local_projects.iter().find(|host_project| {
                    host_project.read_with(cx, |host_project, _| {
                        host_project.remote_id() == Some(project_id)
                    })
                })?;
                Some((client.user_id().unwrap(), project, cx))
            });

            let (host_user_id, host_project, host_cx) =
                if let Some((host_user_id, host_project, host_cx)) = host_project {
                    (host_user_id, host_project, host_cx)
                } else {
                    continue;
                };

            for guest_buffer in guest_buffers {
                let buffer_id = guest_buffer.read_with(client_cx, |buffer, _| buffer.remote_id());
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
                        assert_eq!(guest_file.is_deleted(), host_file.is_deleted());
                        assert_eq!(
                            guest_file.mtime(),
                            host_file.mtime(),
                            "guest {} mtime does not match host {} for path {:?} in project {}",
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

                let host_diff_base =
                    host_buffer.read_with(host_cx, |b, _| b.diff_base().map(ToString::to_string));
                let guest_diff_base = guest_buffer
                    .read_with(client_cx, |b, _| b.diff_base().map(ToString::to_string));
                assert_eq!(guest_diff_base, host_diff_base);

                let host_saved_version =
                    host_buffer.read_with(host_cx, |b, _| b.saved_version().clone());
                let guest_saved_version =
                    guest_buffer.read_with(client_cx, |b, _| b.saved_version().clone());
                assert_eq!(guest_saved_version, host_saved_version);

                let host_saved_version_fingerprint = host_buffer
                    .read_with(host_cx, |b, _| b.saved_version_fingerprint().to_string());
                let guest_saved_version_fingerprint = guest_buffer
                    .read_with(client_cx, |b, _| b.saved_version_fingerprint().to_string());
                assert_eq!(
                    guest_saved_version_fingerprint,
                    host_saved_version_fingerprint
                );

                let host_saved_mtime = host_buffer.read_with(host_cx, |b, _| b.saved_mtime());
                let guest_saved_mtime = guest_buffer.read_with(client_cx, |b, _| b.saved_mtime());
                assert_eq!(guest_saved_mtime, host_saved_mtime);

                let host_is_dirty = host_buffer.read_with(host_cx, |b, _| b.is_dirty());
                let guest_is_dirty = guest_buffer.read_with(client_cx, |b, _| b.is_dirty());
                assert_eq!(guest_is_dirty, host_is_dirty);

                let host_has_conflict = host_buffer.read_with(host_cx, |b, _| b.has_conflict());
                let guest_has_conflict = guest_buffer.read_with(client_cx, |b, _| b.has_conflict());
                assert_eq!(guest_has_conflict, host_has_conflict);
            }
        }
    }

    for (client, mut cx) in clients {
        cx.update(|cx| {
            cx.clear_globals();
            cx.set_global(Settings::test(cx));
            drop(client);
        });
    }
}

async fn simulate_client(
    mut client: TestClient,
    mut op_start_signal: futures::channel::mpsc::UnboundedReceiver<()>,
    can_hang_up: bool,
    rng: Arc<Mutex<StdRng>>,
    mut cx: TestAppContext,
) -> (TestClient, TestAppContext) {
    // Setup language server
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        None,
    );
    let _fake_language_servers = language
        .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
            name: "the-fake-language-server",
            capabilities: lsp::LanguageServer::full_capabilities(),
            initializer: Some(Box::new({
                let rng = rng.clone();
                let fs = client.fs.clone();
                move |fake_server: &mut FakeLanguageServer| {
                    fake_server.handle_request::<lsp::request::Completion, _, _>(
                        |_, _| async move {
                            Ok(Some(lsp::CompletionResponse::Array(vec![
                                lsp::CompletionItem {
                                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                                        range: lsp::Range::new(
                                            lsp::Position::new(0, 0),
                                            lsp::Position::new(0, 0),
                                        ),
                                        new_text: "the-new-text".to_string(),
                                    })),
                                    ..Default::default()
                                },
                            ])))
                        },
                    );

                    fake_server.handle_request::<lsp::request::CodeActionRequest, _, _>(
                        |_, _| async move {
                            Ok(Some(vec![lsp::CodeActionOrCommand::CodeAction(
                                lsp::CodeAction {
                                    title: "the-code-action".to_string(),
                                    ..Default::default()
                                },
                            )]))
                        },
                    );

                    fake_server.handle_request::<lsp::request::PrepareRenameRequest, _, _>(
                        |params, _| async move {
                            Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
                                params.position,
                                params.position,
                            ))))
                        },
                    );

                    fake_server.handle_request::<lsp::request::GotoDefinition, _, _>({
                        let fs = fs.clone();
                        let rng = rng.clone();
                        move |_, _| {
                            let fs = fs.clone();
                            let rng = rng.clone();
                            async move {
                                let files = fs.files().await;
                                let mut rng = rng.lock();
                                let count = rng.gen_range::<usize, _>(1..3);
                                let files = (0..count)
                                    .map(|_| files.choose(&mut *rng).unwrap())
                                    .collect::<Vec<_>>();
                                log::info!("LSP: Returning definitions in files {:?}", &files);
                                Ok(Some(lsp::GotoDefinitionResponse::Array(
                                    files
                                        .into_iter()
                                        .map(|file| lsp::Location {
                                            uri: lsp::Url::from_file_path(file).unwrap(),
                                            range: Default::default(),
                                        })
                                        .collect(),
                                )))
                            }
                        }
                    });

                    fake_server.handle_request::<lsp::request::DocumentHighlightRequest, _, _>({
                        let rng = rng.clone();
                        move |_, _| {
                            let mut highlights = Vec::new();
                            let highlight_count = rng.lock().gen_range(1..=5);
                            for _ in 0..highlight_count {
                                let start_row = rng.lock().gen_range(0..100);
                                let start_column = rng.lock().gen_range(0..100);
                                let start = PointUtf16::new(start_row, start_column);
                                let end_row = rng.lock().gen_range(0..100);
                                let end_column = rng.lock().gen_range(0..100);
                                let end = PointUtf16::new(end_row, end_column);
                                let range = if start > end { end..start } else { start..end };
                                highlights.push(lsp::DocumentHighlight {
                                    range: range_to_lsp(range.clone()),
                                    kind: Some(lsp::DocumentHighlightKind::READ),
                                });
                            }
                            highlights.sort_unstable_by_key(|highlight| {
                                (highlight.range.start, highlight.range.end)
                            });
                            async move { Ok(Some(highlights)) }
                        }
                    });
                }
            })),
            ..Default::default()
        }))
        .await;
    client.language_registry.add(Arc::new(language));

    while op_start_signal.next().await.is_some() {
        if let Err(error) =
            randomly_mutate_client(&mut client, can_hang_up, rng.clone(), &mut cx).await
        {
            log::error!("{} error: {:?}", client.username, error);
        }

        cx.background().simulate_random_delay().await;
    }
    log::info!("{}: done", client.username);

    (client, cx)
}

async fn randomly_mutate_client(
    client: &mut TestClient,
    can_hang_up: bool,
    rng: Arc<Mutex<StdRng>>,
    cx: &mut TestAppContext,
) -> Result<()> {
    let choice = rng.lock().gen_range(0..100);
    match choice {
        0..=19 => randomly_mutate_active_call(client, can_hang_up, &rng, cx).await?,
        20..=49 => randomly_mutate_projects(client, &rng, cx).await?,
        50..=59 if !client.local_projects.is_empty() || !client.remote_projects.is_empty() => {
            randomly_mutate_worktrees(client, &rng, cx).await?;
        }
        60..=74 if !client.local_projects.is_empty() || !client.remote_projects.is_empty() => {
            randomly_query_and_mutate_buffers(client, &rng, cx).await?;
        }
        75..=84 => randomly_mutate_git(client, &rng).await,
        _ => randomly_mutate_fs(client, &rng).await,
    }

    Ok(())
}

async fn randomly_mutate_active_call(
    client: &mut TestClient,
    can_hang_up: bool,
    rng: &Mutex<StdRng>,
    cx: &mut TestAppContext,
) -> Result<()> {
    let active_call = cx.read(ActiveCall::global);
    if active_call.read_with(cx, |call, _| call.incoming().borrow().is_some()) {
        if rng.lock().gen_bool(0.7) {
            log::info!("{}: accepting incoming call", client.username);
            active_call
                .update(cx, |call, cx| call.accept_incoming(cx))
                .await?;
        } else {
            log::info!("{}: declining incoming call", client.username);
            active_call.update(cx, |call, _| call.decline_incoming())?;
        }
    } else {
        let available_contacts = client.user_store.read_with(cx, |user_store, _| {
            user_store
                .contacts()
                .iter()
                .filter(|contact| contact.online && !contact.busy)
                .cloned()
                .collect::<Vec<_>>()
        });

        let distribution = rng.lock().gen_range(0..100);
        match distribution {
            0..=29 if !available_contacts.is_empty() => {
                let contact = available_contacts.choose(&mut *rng.lock()).unwrap();
                log::info!(
                    "{}: inviting {}",
                    client.username,
                    contact.user.github_login
                );
                active_call
                    .update(cx, |call, cx| call.invite(contact.user.id, None, cx))
                    .await?;
            }
            30..=39
                if can_hang_up && active_call.read_with(cx, |call, _| call.room().is_some()) =>
            {
                log::info!("{}: hanging up", client.username);
                active_call.update(cx, |call, cx| call.hang_up(cx))?;
            }
            _ => {}
        }
    }

    Ok(())
}

async fn randomly_mutate_git(client: &mut TestClient, rng: &Mutex<StdRng>) {
    let directories = client.fs.directories().await;
    let mut dir_path = directories.choose(&mut *rng.lock()).unwrap().clone();
    if dir_path.file_name() == Some(OsStr::new(".git")) {
        dir_path.pop();
    }
    let mut git_dir_path = dir_path.clone();
    git_dir_path.push(".git");

    if !directories.contains(&git_dir_path) {
        log::info!(
            "{}: creating git directory at {:?}",
            client.username,
            git_dir_path
        );
        client.fs.create_dir(&git_dir_path).await.unwrap();
    }

    let mut child_file_paths = child_file_paths(client, &dir_path).await;
    let count = rng.lock().gen_range(0..=child_file_paths.len());
    child_file_paths.shuffle(&mut *rng.lock());
    child_file_paths.truncate(count);

    let mut new_index = Vec::new();
    for abs_child_file_path in &child_file_paths {
        let child_file_path = abs_child_file_path.strip_prefix(&dir_path).unwrap();
        let new_base = Alphanumeric.sample_string(&mut *rng.lock(), 16);
        new_index.push((child_file_path, new_base));
    }
    log::info!(
        "{}: updating Git index at {:?}: {:#?}",
        client.username,
        git_dir_path,
        new_index
    );
    client
        .fs
        .set_index_for_repo(&git_dir_path, &new_index)
        .await;
}

async fn randomly_mutate_fs(client: &mut TestClient, rng: &Mutex<StdRng>) {
    let parent_dir_path = client
        .fs
        .directories()
        .await
        .choose(&mut *rng.lock())
        .unwrap()
        .clone();

    let is_dir = rng.lock().gen::<bool>();
    if is_dir {
        let mut dir_path = parent_dir_path.clone();
        dir_path.push(gen_file_name(rng));
        log::info!("{}: creating local dir at {:?}", client.username, dir_path);
        client.fs.create_dir(&dir_path).await.unwrap();
    } else {
        let child_file_paths = child_file_paths(client, &parent_dir_path).await;
        let create_new_file = child_file_paths.is_empty() || rng.lock().gen();
        let text = Alphanumeric.sample_string(&mut *rng.lock(), 16);
        if create_new_file {
            let mut file_path = parent_dir_path.clone();
            file_path.push(gen_file_name(rng));
            file_path.set_extension("rs");
            log::info!(
                "{}: creating local file at {:?}",
                client.username,
                file_path
            );
            client
                .fs
                .create_file(&file_path, Default::default())
                .await
                .unwrap();
            log::info!(
                "{}: setting local file {:?} text to {:?}",
                client.username,
                file_path,
                text
            );
            client
                .fs
                .save(&file_path, &Rope::from(text.as_str()), fs::LineEnding::Unix)
                .await
                .unwrap();
        } else {
            let file_path = child_file_paths.choose(&mut *rng.lock()).unwrap();
            log::info!(
                "{}: setting local file {:?} text to {:?}",
                client.username,
                file_path,
                text
            );
            client
                .fs
                .save(file_path, &Rope::from(text.as_str()), fs::LineEnding::Unix)
                .await
                .unwrap();
        }
    }
}

async fn randomly_mutate_projects(
    client: &mut TestClient,
    rng: &Mutex<StdRng>,
    cx: &mut TestAppContext,
) -> Result<()> {
    let active_call = cx.read(ActiveCall::global);
    let remote_projects =
        if let Some(room) = active_call.read_with(cx, |call, _| call.room().cloned()) {
            room.read_with(cx, |room, _| {
                room.remote_participants()
                    .values()
                    .flat_map(|participant| participant.projects.clone())
                    .collect::<Vec<_>>()
            })
        } else {
            Default::default()
        };

    let project = if remote_projects.is_empty() || rng.lock().gen() {
        if client.local_projects.is_empty() || rng.lock().gen() {
            let paths = client.fs.paths().await;
            let local_project = if paths.is_empty() || rng.lock().gen() {
                let root_path = client.create_new_root_dir();
                client.fs.create_dir(&root_path).await.unwrap();
                client
                    .fs
                    .create_file(&root_path.join("main.rs"), Default::default())
                    .await
                    .unwrap();
                log::info!(
                    "{}: opening local project at {:?}",
                    client.username,
                    root_path
                );
                client.build_local_project(root_path, cx).await.0
            } else {
                let root_path = paths.choose(&mut *rng.lock()).unwrap();
                log::info!(
                    "{}: opening local project at {:?}",
                    client.username,
                    root_path
                );
                client.build_local_project(root_path, cx).await.0
            };
            client.local_projects.push(local_project.clone());
            local_project
        } else {
            client
                .local_projects
                .choose(&mut *rng.lock())
                .unwrap()
                .clone()
        }
    } else {
        if client.remote_projects.is_empty() || rng.lock().gen() {
            let remote_project_id = remote_projects.choose(&mut *rng.lock()).unwrap().id;
            let remote_project = if let Some(project) =
                client.remote_projects.iter().find(|project| {
                    project.read_with(cx, |project, _| {
                        project.remote_id() == Some(remote_project_id)
                    })
                }) {
                project.clone()
            } else {
                log::info!(
                    "{}: opening remote project {}",
                    client.username,
                    remote_project_id
                );
                let call = cx.read(ActiveCall::global);
                let room = call.read_with(cx, |call, _| call.room().unwrap().clone());
                let remote_project = room
                    .update(cx, |room, cx| {
                        room.join_project(
                            remote_project_id,
                            client.language_registry.clone(),
                            FakeFs::new(cx.background().clone()),
                            cx,
                        )
                    })
                    .await?;
                client.remote_projects.push(remote_project.clone());
                remote_project
            };

            remote_project
        } else {
            client
                .remote_projects
                .choose(&mut *rng.lock())
                .unwrap()
                .clone()
        }
    };

    if active_call.read_with(cx, |call, _| call.room().is_some())
        && project.read_with(cx, |project, _| project.is_local() && !project.is_shared())
    {
        match active_call
            .update(cx, |call, cx| call.share_project(project.clone(), cx))
            .await
        {
            Ok(project_id) => {
                log::info!("{}: shared project with id {}", client.username, project_id);
            }
            Err(error) => {
                log::error!("{}: error sharing project, {:?}", client.username, error);
            }
        }
    }

    let choice = rng.lock().gen_range(0..100);
    match choice {
        0..=19 if project.read_with(cx, |project, _| project.is_local()) => {
            let paths = client.fs.paths().await;
            let path = paths.choose(&mut *rng.lock()).unwrap();
            log::info!(
                "{}: finding/creating local worktree for path {:?}",
                client.username,
                path
            );
            project
                .update(cx, |project, cx| {
                    project.find_or_create_local_worktree(&path, true, cx)
                })
                .await
                .unwrap();
        }
        20..=24 if project.read_with(cx, |project, _| project.is_remote()) => {
            log::info!(
                "{}: dropping remote project {}",
                client.username,
                project.read_with(cx, |project, _| project.remote_id().unwrap())
            );

            cx.update(|_| {
                client
                    .remote_projects
                    .retain(|remote_project| *remote_project != project);
                client.buffers.remove(&project);
                drop(project);
            });
        }
        _ => {}
    }

    Ok(())
}

async fn randomly_mutate_worktrees(
    client: &mut TestClient,
    rng: &Mutex<StdRng>,
    cx: &mut TestAppContext,
) -> Result<()> {
    let project = choose_random_project(client, rng).unwrap();
    let Some(worktree) = project.read_with(cx, |project, cx| {
        project
            .worktrees(cx)
            .filter(|worktree| {
                let worktree = worktree.read(cx);
                worktree.is_visible()
                    && worktree.entries(false).any(|e| e.is_file())
                    && worktree.root_entry().map_or(false, |e| e.is_dir())
            })
            .choose(&mut *rng.lock())
    }) else {
        return Ok(())
    };

    let (worktree_id, worktree_root_name) = worktree.read_with(cx, |worktree, _| {
        (worktree.id(), worktree.root_name().to_string())
    });

    let is_dir = rng.lock().gen::<bool>();
    let mut new_path = PathBuf::new();
    new_path.push(gen_file_name(rng));
    if !is_dir {
        new_path.set_extension("rs");
    }
    log::info!(
        "{}: creating {:?} in worktree {} ({})",
        client.username,
        new_path,
        worktree_id,
        worktree_root_name,
    );
    project
        .update(cx, |project, cx| {
            project.create_entry((worktree_id, new_path), is_dir, cx)
        })
        .unwrap()
        .await?;
    Ok(())
}

async fn randomly_query_and_mutate_buffers(
    client: &mut TestClient,
    rng: &Mutex<StdRng>,
    cx: &mut TestAppContext,
) -> Result<()> {
    let project = choose_random_project(client, rng).unwrap();
    let buffers = client.buffers.entry(project.clone()).or_default();
    let buffer = if buffers.is_empty() || rng.lock().gen() {
        let Some(worktree) = project.read_with(cx, |project, cx| {
            project
                .worktrees(cx)
                .filter(|worktree| {
                    let worktree = worktree.read(cx);
                    worktree.is_visible() && worktree.entries(false).any(|e| e.is_file())
                })
                .choose(&mut *rng.lock())
        }) else {
            return Ok(());
        };

        let (worktree_root_name, project_path) = worktree.read_with(cx, |worktree, _| {
            let entry = worktree
                .entries(false)
                .filter(|e| e.is_file())
                .choose(&mut *rng.lock())
                .unwrap();
            (
                worktree.root_name().to_string(),
                (worktree.id(), entry.path.clone()),
            )
        });
        log::info!(
            "{}: opening path {:?} in worktree {} ({})",
            client.username,
            project_path.1,
            project_path.0,
            worktree_root_name,
        );
        let buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(project_path.clone(), cx)
            })
            .await?;
        log::info!(
            "{}: opened path {:?} in worktree {} ({}) with buffer id {}",
            client.username,
            project_path.1,
            project_path.0,
            worktree_root_name,
            buffer.read_with(cx, |buffer, _| buffer.remote_id())
        );
        buffers.insert(buffer.clone());
        buffer
    } else {
        buffers.iter().choose(&mut *rng.lock()).unwrap().clone()
    };

    let choice = rng.lock().gen_range(0..100);
    match choice {
        0..=9 => {
            cx.update(|cx| {
                log::info!(
                    "{}: dropping buffer {:?}",
                    client.username,
                    buffer.read(cx).file().unwrap().full_path(cx)
                );
                buffers.remove(&buffer);
                drop(buffer);
            });
        }
        10..=19 => {
            let completions = project.update(cx, |project, cx| {
                log::info!(
                    "{}: requesting completions for buffer {} ({:?})",
                    client.username,
                    buffer.read(cx).remote_id(),
                    buffer.read(cx).file().unwrap().full_path(cx)
                );
                let offset = rng.lock().gen_range(0..=buffer.read(cx).len());
                project.completions(&buffer, offset, cx)
            });
            let completions = cx.background().spawn(async move {
                completions
                    .await
                    .map_err(|err| anyhow!("completions request failed: {:?}", err))
            });
            if rng.lock().gen_bool(0.3) {
                log::info!("{}: detaching completions request", client.username);
                cx.update(|cx| completions.detach_and_log_err(cx));
            } else {
                completions.await?;
            }
        }
        20..=29 => {
            let code_actions = project.update(cx, |project, cx| {
                log::info!(
                    "{}: requesting code actions for buffer {} ({:?})",
                    client.username,
                    buffer.read(cx).remote_id(),
                    buffer.read(cx).file().unwrap().full_path(cx)
                );
                let range = buffer.read(cx).random_byte_range(0, &mut *rng.lock());
                project.code_actions(&buffer, range, cx)
            });
            let code_actions = cx.background().spawn(async move {
                code_actions
                    .await
                    .map_err(|err| anyhow!("code actions request failed: {:?}", err))
            });
            if rng.lock().gen_bool(0.3) {
                log::info!("{}: detaching code actions request", client.username);
                cx.update(|cx| code_actions.detach_and_log_err(cx));
            } else {
                code_actions.await?;
            }
        }
        30..=39 if buffer.read_with(cx, |buffer, _| buffer.is_dirty()) => {
            let (requested_version, save) = buffer.update(cx, |buffer, cx| {
                log::info!(
                    "{}: saving buffer {} ({:?})",
                    client.username,
                    buffer.remote_id(),
                    buffer.file().unwrap().full_path(cx)
                );
                (buffer.version(), buffer.save(cx))
            });
            let save = cx.background().spawn(async move {
                let (saved_version, _, _) = save
                    .await
                    .map_err(|err| anyhow!("save request failed: {:?}", err))?;
                assert!(saved_version.observed_all(&requested_version));
                Ok::<_, anyhow::Error>(())
            });
            if rng.lock().gen_bool(0.3) {
                log::info!("{}: detaching save request", client.username);
                cx.update(|cx| save.detach_and_log_err(cx));
            } else {
                save.await?;
            }
        }
        40..=44 => {
            let prepare_rename = project.update(cx, |project, cx| {
                log::info!(
                    "{}: preparing rename for buffer {} ({:?})",
                    client.username,
                    buffer.read(cx).remote_id(),
                    buffer.read(cx).file().unwrap().full_path(cx)
                );
                let offset = rng.lock().gen_range(0..=buffer.read(cx).len());
                project.prepare_rename(buffer, offset, cx)
            });
            let prepare_rename = cx.background().spawn(async move {
                prepare_rename
                    .await
                    .map_err(|err| anyhow!("prepare rename request failed: {:?}", err))
            });
            if rng.lock().gen_bool(0.3) {
                log::info!("{}: detaching prepare rename request", client.username);
                cx.update(|cx| prepare_rename.detach_and_log_err(cx));
            } else {
                prepare_rename.await?;
            }
        }
        45..=49 => {
            let definitions = project.update(cx, |project, cx| {
                log::info!(
                    "{}: requesting definitions for buffer {} ({:?})",
                    client.username,
                    buffer.read(cx).remote_id(),
                    buffer.read(cx).file().unwrap().full_path(cx)
                );
                let offset = rng.lock().gen_range(0..=buffer.read(cx).len());
                project.definition(&buffer, offset, cx)
            });
            let definitions = cx.background().spawn(async move {
                definitions
                    .await
                    .map_err(|err| anyhow!("definitions request failed: {:?}", err))
            });
            if rng.lock().gen_bool(0.3) {
                log::info!("{}: detaching definitions request", client.username);
                cx.update(|cx| definitions.detach_and_log_err(cx));
            } else {
                buffers.extend(definitions.await?.into_iter().map(|loc| loc.target.buffer));
            }
        }
        50..=54 => {
            let highlights = project.update(cx, |project, cx| {
                log::info!(
                    "{}: requesting highlights for buffer {} ({:?})",
                    client.username,
                    buffer.read(cx).remote_id(),
                    buffer.read(cx).file().unwrap().full_path(cx)
                );
                let offset = rng.lock().gen_range(0..=buffer.read(cx).len());
                project.document_highlights(&buffer, offset, cx)
            });
            let highlights = cx.background().spawn(async move {
                highlights
                    .await
                    .map_err(|err| anyhow!("highlights request failed: {:?}", err))
            });
            if rng.lock().gen_bool(0.3) {
                log::info!("{}: detaching highlights request", client.username);
                cx.update(|cx| highlights.detach_and_log_err(cx));
            } else {
                highlights.await?;
            }
        }
        55..=59 => {
            let search = project.update(cx, |project, cx| {
                let query = rng.lock().gen_range('a'..='z');
                log::info!("{}: project-wide search {:?}", client.username, query);
                project.search(SearchQuery::text(query, false, false), cx)
            });
            let search = cx.background().spawn(async move {
                search
                    .await
                    .map_err(|err| anyhow!("search request failed: {:?}", err))
            });
            if rng.lock().gen_bool(0.3) {
                log::info!("{}: detaching search request", client.username);
                cx.update(|cx| search.detach_and_log_err(cx));
            } else {
                buffers.extend(search.await?.into_keys());
            }
        }
        _ => {
            buffer.update(cx, |buffer, cx| {
                log::info!(
                    "{}: updating buffer {} ({:?})",
                    client.username,
                    buffer.remote_id(),
                    buffer.file().unwrap().full_path(cx)
                );
                if rng.lock().gen_bool(0.7) {
                    buffer.randomly_edit(&mut *rng.lock(), 5, cx);
                } else {
                    buffer.randomly_undo_redo(&mut *rng.lock(), cx);
                }
            });
        }
    }

    Ok(())
}

fn choose_random_project(
    client: &mut TestClient,
    rng: &Mutex<StdRng>,
) -> Option<ModelHandle<Project>> {
    client
        .local_projects
        .iter()
        .chain(&client.remote_projects)
        .choose(&mut *rng.lock())
        .cloned()
}

fn gen_file_name(rng: &Mutex<StdRng>) -> String {
    let mut name = String::new();
    for _ in 0..10 {
        let letter = rng.lock().gen_range('a'..='z');
        name.push(letter);
    }
    name
}

async fn child_file_paths(client: &TestClient, dir_path: &Path) -> Vec<PathBuf> {
    let mut child_paths = client.fs.read_dir(dir_path).await.unwrap();
    let mut child_file_paths = Vec::new();
    while let Some(child_path) = child_paths.next().await {
        let child_path = child_path.unwrap();
        if client.fs.is_file(&child_path).await {
            child_file_paths.push(child_path);
        }
    }
    child_file_paths
}
