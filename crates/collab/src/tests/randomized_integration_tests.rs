use crate::{
    db::{self, NewUserParams, UserId},
    rpc::{CLEANUP_TIMEOUT, RECONNECT_TIMEOUT},
    tests::{TestClient, TestServer},
};
use anyhow::{anyhow, Result};
use call::ActiveCall;
use client::RECEIVE_TIMEOUT;
use collections::BTreeMap;
use fs::Fs as _;
use futures::StreamExt as _;
use gpui::{executor::Deterministic, ModelHandle, TestAppContext};
use language::{range_to_lsp, FakeLspAdapter, Language, LanguageConfig, PointUtf16};
use lsp::FakeLanguageServer;
use parking_lot::Mutex;
use project::{search::SearchQuery, Project};
use rand::prelude::*;
use std::{
    env,
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

#[gpui::test(iterations = 100)]
async fn test_random_collaboration(
    cx: &mut TestAppContext,
    deterministic: Arc<Deterministic>,
    mut rng: StdRng,
) {
    deterministic.forbid_parking();

    let max_peers = env::var("MAX_PEERS")
        .map(|i| i.parse().expect("invalid `MAX_PEERS` variable"))
        .unwrap_or(5);

    let max_operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let mut server = TestServer::start(&deterministic).await;
    let db = server.app_state.db.clone();

    let mut users = Vec::new();
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
        users.push(UserTestPlan {
            user_id,
            username,
            online: false,
            next_root_id: 0,
        });
    }

    for (ix, user_a) in users.iter().enumerate() {
        for user_b in &users[ix + 1..] {
            server
                .app_state
                .db
                .send_contact_request(user_a.user_id, user_b.user_id)
                .await
                .unwrap();
            server
                .app_state
                .db
                .respond_to_contact_request(user_b.user_id, user_a.user_id, true)
                .await
                .unwrap();
        }
    }

    let plan = Arc::new(Mutex::new(TestPlan {
        users,
        allow_server_restarts: rng.gen_bool(0.7),
        allow_client_reconnection: rng.gen_bool(0.7),
        allow_client_disconnection: rng.gen_bool(0.1),
        rng,
    }));

    let mut clients = Vec::new();
    let mut client_tasks = Vec::new();
    let mut operation_channels = Vec::new();
    let mut next_entity_id = 100000;

    let mut i = 0;
    while i < max_operations {
        let next_operation = plan.lock().next_operation(&clients).await;
        match next_operation {
            Operation::AddConnection { user_id } => {
                let username = {
                    let mut plan = plan.lock();
                    let mut user = plan.user(user_id);
                    user.online = true;
                    user.username.clone()
                };
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

                let (operation_tx, operation_rx) = futures::channel::mpsc::unbounded();
                let client = Rc::new(server.create_client(&mut client_cx, &username).await);
                operation_channels.push(operation_tx);
                clients.push((client.clone(), client_cx.clone()));
                client_tasks.push(client_cx.foreground().spawn(simulate_client(
                    client,
                    operation_rx,
                    plan.clone(),
                    client_cx,
                )));

                log::info!("Added connection for {}", username);
                i += 1;
            }

            Operation::RemoveConnection { user_id } => {
                log::info!("Simulating full disconnection of user {}", user_id);
                let client_ix = clients
                    .iter()
                    .position(|(client, cx)| client.current_user_id(cx) == user_id)
                    .unwrap();
                let user_connection_ids = server
                    .connection_pool
                    .lock()
                    .user_connection_ids(user_id)
                    .collect::<Vec<_>>();
                assert_eq!(user_connection_ids.len(), 1);
                let removed_peer_id = user_connection_ids[0].into();
                let (client, mut client_cx) = clients.remove(client_ix);
                let client_task = client_tasks.remove(client_ix);
                operation_channels.remove(client_ix);
                server.forbid_connections();
                server.disconnect_client(removed_peer_id);
                deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
                deterministic.start_waiting();
                log::info!("Waiting for user {} to exit...", user_id);
                client_task.await;
                deterministic.finish_waiting();
                server.allow_connections();

                for project in client.remote_projects().iter() {
                    project.read_with(&client_cx, |project, _| {
                        assert!(
                            project.is_read_only(),
                            "project {:?} should be read only",
                            project.remote_id()
                        )
                    });
                }

                for (client, cx) in &clients {
                    let contacts = server
                        .app_state
                        .db
                        .get_contacts(client.current_user_id(cx))
                        .await
                        .unwrap();
                    let pool = server.connection_pool.lock();
                    for contact in contacts {
                        if let db::Contact::Accepted { user_id: id, .. } = contact {
                            if pool.is_user_online(id) {
                                assert_ne!(
                                    id, user_id,
                                    "removed client is still a contact of another peer"
                                );
                            }
                        }
                    }
                }

                log::info!("{} removed", client.username);
                plan.lock().user(user_id).online = false;
                client_cx.update(|cx| {
                    cx.clear_globals();
                    drop(client);
                });
                i += 1;
            }

            Operation::BounceConnection { user_id } => {
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
                i += 1;
            }

            Operation::RestartServer => {
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
                i += 1;
            }

            Operation::RunUntilParked => {
                deterministic.run_until_parked();
            }

            Operation::MutateClients(user_ids) => {
                for user_id in user_ids {
                    let client_ix = clients
                        .iter()
                        .position(|(client, cx)| client.current_user_id(cx) == user_id)
                        .unwrap();
                    operation_channels[client_ix].unbounded_send(()).unwrap();
                    i += 1;
                }
            }
        }
    }

    drop(operation_channels);
    deterministic.start_waiting();
    futures::future::join_all(client_tasks).await;
    deterministic.finish_waiting();
    deterministic.run_until_parked();

    for (client, client_cx) in &clients {
        for guest_project in client.remote_projects().iter() {
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

        let buffers = client.buffers().clone();
        for (guest_project, guest_buffers) in &buffers {
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
                    (None, _) => panic!("host's file is None, guest's isn't "),
                    (_, None) => panic!("guest's file is None, hosts's isn't "),
                }
            }
        }
    }

    for (client, mut cx) in clients {
        cx.update(|cx| {
            cx.clear_globals();
            drop(client);
        });
    }
}

struct TestPlan {
    rng: StdRng,
    users: Vec<UserTestPlan>,
    allow_server_restarts: bool,
    allow_client_reconnection: bool,
    allow_client_disconnection: bool,
}

struct UserTestPlan {
    user_id: UserId,
    username: String,
    next_root_id: usize,
    online: bool,
}

#[derive(Debug)]
enum Operation {
    AddConnection { user_id: UserId },
    RemoveConnection { user_id: UserId },
    BounceConnection { user_id: UserId },
    RestartServer,
    RunUntilParked,
    MutateClients(Vec<UserId>),
}

#[derive(Debug)]
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
        full_path: PathBuf,
    },
    EditBuffer {
        project_root_name: String,
        full_path: PathBuf,
        edits: Vec<(Range<usize>, Arc<str>)>,
    },
    Other,
}

impl TestPlan {
    async fn next_operation(&mut self, clients: &[(Rc<TestClient>, TestAppContext)]) -> Operation {
        let operation = loop {
            break match self.rng.gen_range(0..100) {
                0..=19 if clients.len() < self.users.len() => {
                    let user = self
                        .users
                        .iter()
                        .filter(|u| !u.online)
                        .choose(&mut self.rng)
                        .unwrap();
                    Operation::AddConnection {
                        user_id: user.user_id,
                    }
                }
                20..=24 if clients.len() > 1 && self.allow_client_disconnection => {
                    let (client, cx) = &clients[self.rng.gen_range(0..clients.len())];
                    let user_id = client.current_user_id(cx);
                    Operation::RemoveConnection { user_id }
                }
                25..=29 if clients.len() > 1 && self.allow_client_reconnection => {
                    let (client, cx) = &clients[self.rng.gen_range(0..clients.len())];
                    let user_id = client.current_user_id(cx);
                    Operation::BounceConnection { user_id }
                }
                30..=34 if self.allow_server_restarts && clients.len() > 1 => {
                    Operation::RestartServer
                }
                35..=39 => Operation::RunUntilParked,
                _ if !clients.is_empty() => {
                    let user_ids = (0..self.rng.gen_range(0..10))
                        .map(|_| {
                            let ix = self.rng.gen_range(0..clients.len());
                            let (client, cx) = &clients[ix];
                            client.current_user_id(cx)
                        })
                        .collect();
                    Operation::MutateClients(user_ids)
                }
                _ => continue,
            };
        };
        operation
    }

    async fn next_client_operation(
        &mut self,
        client: &TestClient,
        cx: &TestAppContext,
    ) -> ClientOperation {
        let user_id = client.current_user_id(cx);
        let call = cx.read(ActiveCall::global);
        let operation = loop {
            match self.rng.gen_range(0..100) {
                // Mutate the call
                0..=19 => match self.rng.gen_range(0..100_u32) {
                    // Respond to an incoming call
                    0..=39 => {
                        if call.read_with(cx, |call, _| call.incoming().borrow().is_some()) {
                            break if self.rng.gen_bool(0.7) {
                                ClientOperation::AcceptIncomingCall
                            } else {
                                ClientOperation::RejectIncomingCall
                            };
                        }
                    }

                    // Invite a contact to the current call
                    30..=89 => {
                        let available_contacts =
                            client.user_store.read_with(cx, |user_store, _| {
                                user_store
                                    .contacts()
                                    .iter()
                                    .filter(|contact| contact.online && !contact.busy)
                                    .cloned()
                                    .collect::<Vec<_>>()
                            });
                        if !available_contacts.is_empty() {
                            let contact = available_contacts.choose(&mut self.rng).unwrap();
                            break ClientOperation::InviteContactToCall {
                                user_id: UserId(contact.user.id as i32),
                            };
                        }
                    }

                    // Leave the current call
                    90.. => {
                        if self.allow_client_disconnection
                            && call.read_with(cx, |call, _| call.room().is_some())
                        {
                            break ClientOperation::LeaveCall;
                        }
                    }
                },

                // Mutate projects
                20..=39 => match self.rng.gen_range(0..100_u32) {
                    // Open a remote project
                    0..=30 => {
                        if let Some(room) = call.read_with(cx, |call, _| call.room().cloned()) {
                            let remote_projects = room.read_with(cx, |room, _| {
                                room.remote_participants()
                                    .values()
                                    .flat_map(|participant| {
                                        participant.projects.iter().map(|project| {
                                            (
                                                UserId::from_proto(participant.user.id),
                                                project.worktree_root_names[0].clone(),
                                            )
                                        })
                                    })
                                    .collect::<Vec<_>>()
                            });
                            if !remote_projects.is_empty() {
                                let (host_id, first_root_name) =
                                    remote_projects.choose(&mut self.rng).unwrap().clone();
                                break ClientOperation::OpenRemoteProject {
                                    host_id,
                                    first_root_name,
                                };
                            }
                        }
                    }

                    // Close a remote project
                    31..=40 => {
                        if !client.remote_projects().is_empty() {
                            let project = client
                                .remote_projects()
                                .choose(&mut self.rng)
                                .unwrap()
                                .clone();
                            let first_root_name = root_name_for_project(&project, cx);
                            break ClientOperation::CloseRemoteProject {
                                project_root_name: first_root_name,
                            };
                        }
                    }

                    // Open a local project
                    41..=60 => {
                        let first_root_name = self.next_root_dir_name(user_id);
                        break ClientOperation::OpenLocalProject { first_root_name };
                    }

                    // Add a worktree to a local project
                    61.. => {
                        if !client.local_projects().is_empty() {
                            let project = client
                                .local_projects()
                                .choose(&mut self.rng)
                                .unwrap()
                                .clone();
                            let project_root_name = root_name_for_project(&project, cx);

                            let mut paths = client.fs.paths().await;
                            paths.remove(0);
                            let new_root_path = if paths.is_empty() || self.rng.gen() {
                                Path::new("/").join(&self.next_root_dir_name(user_id))
                            } else {
                                paths.choose(&mut self.rng).unwrap().clone()
                            };

                            break ClientOperation::AddWorktreeToProject {
                                project_root_name,
                                new_root_path,
                            };
                        }
                    }
                },

                // Mutate buffers
                40..=79 => {
                    let Some(project) = choose_random_project(client, &mut self.rng) else { continue };
                    let project_root_name = root_name_for_project(&project, cx);

                    match self.rng.gen_range(0..100_u32) {
                        // Manipulate an existing buffer
                        0..=80 => {
                            let Some(buffer) = client
                                .buffers_for_project(&project)
                                .iter()
                                .choose(&mut self.rng)
                                .cloned() else { continue };

                            match self.rng.gen_range(0..100_u32) {
                                0..=9 => {
                                    let (full_path, edits) = buffer.read_with(cx, |buffer, cx| {
                                        (
                                            buffer.file().unwrap().full_path(cx),
                                            buffer.get_random_edits(&mut self.rng, 3),
                                        )
                                    });
                                    break ClientOperation::EditBuffer {
                                        project_root_name,
                                        full_path,
                                        edits,
                                    };
                                }
                                _ => {}
                            }
                        }

                        // Open a buffer
                        81.. => {
                            let worktree = project.read_with(cx, |project, cx| {
                                project
                                    .worktrees(cx)
                                    .filter(|worktree| {
                                        let worktree = worktree.read(cx);
                                        worktree.is_visible()
                                            && worktree.entries(false).any(|e| e.is_file())
                                    })
                                    .choose(&mut self.rng)
                            });
                            let Some(worktree) = worktree else { continue };
                            let full_path = worktree.read_with(cx, |worktree, _| {
                                let entry = worktree
                                    .entries(false)
                                    .filter(|e| e.is_file())
                                    .choose(&mut self.rng)
                                    .unwrap();
                                if entry.path.as_ref() == Path::new("") {
                                    Path::new(worktree.root_name()).into()
                                } else {
                                    Path::new(worktree.root_name()).join(&entry.path)
                                }
                            });
                            break ClientOperation::OpenBuffer {
                                project_root_name,
                                full_path,
                            };
                        }
                    }
                }

                _ => break ClientOperation::Other,
            }
        };
        operation
    }

    fn next_root_dir_name(&mut self, user_id: UserId) -> String {
        let user_ix = self
            .users
            .iter()
            .position(|user| user.user_id == user_id)
            .unwrap();
        let root_id = util::post_inc(&mut self.users[user_ix].next_root_id);
        format!("dir-{user_id}-{root_id}")
    }

    fn user(&mut self, user_id: UserId) -> &mut UserTestPlan {
        let ix = self
            .users
            .iter()
            .position(|user| user.user_id == user_id)
            .unwrap();
        &mut self.users[ix]
    }
}

async fn simulate_client(
    client: Rc<TestClient>,
    mut operation_rx: futures::channel::mpsc::UnboundedReceiver<()>,
    plan: Arc<Mutex<TestPlan>>,
    mut cx: TestAppContext,
) {
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
                let plan = plan.clone();
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
                        let plan = plan.clone();
                        move |_, _| {
                            let fs = fs.clone();
                            let plan = plan.clone();
                            async move {
                                let files = fs.files().await;
                                let mut plan = plan.lock();
                                let count = plan.rng.gen_range::<usize, _>(1..3);
                                let files = (0..count)
                                    .map(|_| files.choose(&mut plan.rng).unwrap())
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
                        let plan = plan.clone();
                        move |_, _| {
                            let mut highlights = Vec::new();
                            let highlight_count = plan.lock().rng.gen_range(1..=5);
                            for _ in 0..highlight_count {
                                let start_row = plan.lock().rng.gen_range(0..100);
                                let start_column = plan.lock().rng.gen_range(0..100);
                                let start = PointUtf16::new(start_row, start_column);
                                let end_row = plan.lock().rng.gen_range(0..100);
                                let end_column = plan.lock().rng.gen_range(0..100);
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

    while operation_rx.next().await.is_some() {
        let operation = plan.lock().next_client_operation(&client, &cx).await;
        if let Err(error) = apply_client_operation(&client, plan.clone(), operation, &mut cx).await
        {
            log::error!("{} error: {:?}", client.username, error);
        }

        cx.background().simulate_random_delay().await;
    }
    log::info!("{}: done", client.username);
}

async fn apply_client_operation(
    client: &TestClient,
    plan: Arc<Mutex<TestPlan>>,
    operation: ClientOperation,
    cx: &mut TestAppContext,
) -> Result<()> {
    match operation {
        ClientOperation::AcceptIncomingCall => {
            log::info!("{}: accepting incoming call", client.username);
            let active_call = cx.read(ActiveCall::global);
            active_call
                .update(cx, |call, cx| call.accept_incoming(cx))
                .await?;
        }

        ClientOperation::RejectIncomingCall => {
            log::info!("{}: declining incoming call", client.username);
            let active_call = cx.read(ActiveCall::global);
            active_call.update(cx, |call, _| call.decline_incoming())?;
        }

        ClientOperation::LeaveCall => {
            log::info!("{}: hanging up", client.username);
            let active_call = cx.read(ActiveCall::global);
            active_call.update(cx, |call, cx| call.hang_up(cx))?;
        }

        ClientOperation::InviteContactToCall { user_id } => {
            log::info!("{}: inviting {}", client.username, user_id,);
            let active_call = cx.read(ActiveCall::global);
            active_call
                .update(cx, |call, cx| call.invite(user_id.to_proto(), None, cx))
                .await?;
        }

        ClientOperation::OpenLocalProject { first_root_name } => {
            log::info!(
                "{}: opening local project at {:?}",
                client.username,
                first_root_name
            );
            let root_path = Path::new("/").join(&first_root_name);
            client.fs.create_dir(&root_path).await.unwrap();
            client
                .fs
                .create_file(&root_path.join("main.rs"), Default::default())
                .await
                .unwrap();
            let project = client.build_local_project(root_path, cx).await.0;

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

            client.local_projects_mut().push(project.clone());
        }

        ClientOperation::AddWorktreeToProject {
            project_root_name,
            new_root_path,
        } => {
            log::info!(
                "{}: finding/creating local worktree at {:?} to project with root path {}",
                client.username,
                new_root_path,
                project_root_name
            );
            let project = project_for_root_name(client, &project_root_name, cx)
                .expect("invalid project in test operation");
            if !client.fs.paths().await.contains(&new_root_path) {
                client.fs.create_dir(&new_root_path).await.unwrap();
            }
            project
                .update(cx, |project, cx| {
                    project.find_or_create_local_worktree(&new_root_path, true, cx)
                })
                .await
                .unwrap();
        }

        ClientOperation::CloseRemoteProject { project_root_name } => {
            log::info!(
                "{}: dropping project with root path {}",
                client.username,
                project_root_name,
            );
            let ix = project_ix_for_root_name(&*client.remote_projects(), &project_root_name, cx)
                .expect("invalid project in test operation");
            client.remote_projects_mut().remove(ix);
        }

        ClientOperation::OpenRemoteProject {
            host_id,
            first_root_name,
        } => {
            log::info!(
                "{}: joining remote project of user {}, root name {}",
                client.username,
                host_id,
                first_root_name,
            );
            let active_call = cx.read(ActiveCall::global);
            let project_id = active_call
                .read_with(cx, |call, cx| {
                    let room = call.room().cloned()?;
                    let participant = room
                        .read(cx)
                        .remote_participants()
                        .get(&host_id.to_proto())?;
                    let project = participant
                        .projects
                        .iter()
                        .find(|project| project.worktree_root_names[0] == first_root_name)?;
                    Some(project.id)
                })
                .expect("invalid project in test operation");
            let project = client.build_remote_project(project_id, cx).await;
            client.remote_projects_mut().push(project);
        }

        ClientOperation::OpenBuffer {
            project_root_name,
            full_path,
        } => {
            log::info!(
                "{}: opening path {:?} in project {}",
                client.username,
                full_path,
                project_root_name,
            );
            let project = project_for_root_name(client, &project_root_name, cx)
                .expect("invalid project in test operation");
            let mut components = full_path.components();
            let root_name = components.next().unwrap().as_os_str().to_str().unwrap();
            let path = components.as_path();
            let worktree_id = project
                .read_with(cx, |project, cx| {
                    project.worktrees(cx).find_map(|worktree| {
                        let worktree = worktree.read(cx);
                        if worktree.root_name() == root_name {
                            Some(worktree.id())
                        } else {
                            None
                        }
                    })
                })
                .expect("invalid buffer path in test operation");
            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer((worktree_id, &path), cx)
                })
                .await?;
            client.buffers_for_project(&project).insert(buffer);
        }

        ClientOperation::EditBuffer {
            project_root_name,
            full_path,
            edits,
        } => {
            log::info!(
                "{}: editing buffer {:?} in project {} with {:?}",
                client.username,
                full_path,
                project_root_name,
                edits
            );
            let project = project_for_root_name(client, &project_root_name, cx)
                .expect("invalid project in test operation");
            let buffer = client
                .buffers_for_project(&project)
                .iter()
                .find(|buffer| {
                    buffer.read_with(cx, |buffer, cx| {
                        buffer.file().unwrap().full_path(cx) == full_path
                    })
                })
                .cloned()
                .expect("invalid buffer path in test operation");
            buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });
        }

        _ => {
            let choice = plan.lock().rng.gen_range(0..100);
            match choice {
                50..=59
                    if !client.local_projects().is_empty()
                        || !client.remote_projects().is_empty() =>
                {
                    randomly_mutate_worktrees(client, &plan, cx).await?;
                }
                60..=84
                    if !client.local_projects().is_empty()
                        || !client.remote_projects().is_empty() =>
                {
                    randomly_query_and_mutate_buffers(client, &plan, cx).await?;
                }
                _ => randomly_mutate_fs(client, &plan).await,
            }
        }
    }
    Ok(())
}

fn project_for_root_name(
    client: &TestClient,
    root_name: &str,
    cx: &TestAppContext,
) -> Option<ModelHandle<Project>> {
    if let Some(ix) = project_ix_for_root_name(&*client.local_projects(), root_name, cx) {
        return Some(client.local_projects()[ix].clone());
    }
    if let Some(ix) = project_ix_for_root_name(&*client.remote_projects(), root_name, cx) {
        return Some(client.remote_projects()[ix].clone());
    }
    None
}

fn project_ix_for_root_name(
    projects: &[ModelHandle<Project>],
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

fn root_name_for_project(project: &ModelHandle<Project>, cx: &TestAppContext) -> String {
    project.read_with(cx, |project, cx| {
        project
            .visible_worktrees(cx)
            .next()
            .unwrap()
            .read(cx)
            .root_name()
            .to_string()
    })
}

async fn randomly_mutate_fs(client: &TestClient, plan: &Arc<Mutex<TestPlan>>) {
    let is_dir = plan.lock().rng.gen::<bool>();
    let mut new_path = client
        .fs
        .directories()
        .await
        .choose(&mut plan.lock().rng)
        .unwrap()
        .clone();
    new_path.push(gen_file_name(&mut plan.lock().rng));
    if is_dir {
        log::info!("{}: creating local dir at {:?}", client.username, new_path);
        client.fs.create_dir(&new_path).await.unwrap();
    } else {
        new_path.set_extension("rs");
        log::info!("{}: creating local file at {:?}", client.username, new_path);
        client
            .fs
            .create_file(&new_path, Default::default())
            .await
            .unwrap();
    }
}

async fn randomly_mutate_worktrees(
    client: &TestClient,
    plan: &Arc<Mutex<TestPlan>>,
    cx: &mut TestAppContext,
) -> Result<()> {
    let project = choose_random_project(client, &mut plan.lock().rng).unwrap();
    let Some(worktree) = project.read_with(cx, |project, cx| {
        project
            .worktrees(cx)
            .filter(|worktree| {
                let worktree = worktree.read(cx);
                worktree.is_visible()
                    && worktree.entries(false).any(|e| e.is_file())
                    && worktree.root_entry().map_or(false, |e| e.is_dir())
            })
            .choose(&mut plan.lock().rng)
    }) else {
        return Ok(())
    };

    let (worktree_id, worktree_root_name) = worktree.read_with(cx, |worktree, _| {
        (worktree.id(), worktree.root_name().to_string())
    });

    let is_dir = plan.lock().rng.gen::<bool>();
    let mut new_path = PathBuf::new();
    new_path.push(gen_file_name(&mut plan.lock().rng));
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
    client: &TestClient,
    plan: &Arc<Mutex<TestPlan>>,
    cx: &mut TestAppContext,
) -> Result<()> {
    let project = choose_random_project(client, &mut plan.lock().rng).unwrap();
    let has_buffers_for_project = !client.buffers_for_project(&project).is_empty();
    let buffer = if !has_buffers_for_project || plan.lock().rng.gen() {
        let Some(worktree) = project.read_with(cx, |project, cx| {
            project
                .worktrees(cx)
                .filter(|worktree| {
                    let worktree = worktree.read(cx);
                    worktree.is_visible() && worktree.entries(false).any(|e| e.is_file())
                })
                .choose(&mut plan.lock().rng)
        }) else {
            return Ok(());
        };

        let (worktree_root_name, project_path) = worktree.read_with(cx, |worktree, _| {
            let entry = worktree
                .entries(false)
                .filter(|e| e.is_file())
                .choose(&mut plan.lock().rng)
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
        client.buffers_for_project(&project).insert(buffer.clone());
        buffer
    } else {
        client
            .buffers_for_project(&project)
            .iter()
            .choose(&mut plan.lock().rng)
            .unwrap()
            .clone()
    };

    let choice = plan.lock().rng.gen_range(0..100);
    match choice {
        0..=9 => {
            cx.update(|cx| {
                log::info!(
                    "{}: dropping buffer {:?}",
                    client.username,
                    buffer.read(cx).file().unwrap().full_path(cx)
                );
                client.buffers_for_project(&project).remove(&buffer);
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
                let offset = plan.lock().rng.gen_range(0..=buffer.read(cx).len());
                project.completions(&buffer, offset, cx)
            });
            let completions = cx.background().spawn(async move {
                completions
                    .await
                    .map_err(|err| anyhow!("completions request failed: {:?}", err))
            });
            if plan.lock().rng.gen_bool(0.3) {
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
                let range = buffer.read(cx).random_byte_range(0, &mut plan.lock().rng);
                project.code_actions(&buffer, range, cx)
            });
            let code_actions = cx.background().spawn(async move {
                code_actions
                    .await
                    .map_err(|err| anyhow!("code actions request failed: {:?}", err))
            });
            if plan.lock().rng.gen_bool(0.3) {
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
            if plan.lock().rng.gen_bool(0.3) {
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
                let offset = plan.lock().rng.gen_range(0..=buffer.read(cx).len());
                project.prepare_rename(buffer, offset, cx)
            });
            let prepare_rename = cx.background().spawn(async move {
                prepare_rename
                    .await
                    .map_err(|err| anyhow!("prepare rename request failed: {:?}", err))
            });
            if plan.lock().rng.gen_bool(0.3) {
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
                let offset = plan.lock().rng.gen_range(0..=buffer.read(cx).len());
                project.definition(&buffer, offset, cx)
            });
            let definitions = cx.background().spawn(async move {
                definitions
                    .await
                    .map_err(|err| anyhow!("definitions request failed: {:?}", err))
            });
            if plan.lock().rng.gen_bool(0.3) {
                log::info!("{}: detaching definitions request", client.username);
                cx.update(|cx| definitions.detach_and_log_err(cx));
            } else {
                let definitions = definitions.await?;
                client
                    .buffers_for_project(&project)
                    .extend(definitions.into_iter().map(|loc| loc.target.buffer));
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
                let offset = plan.lock().rng.gen_range(0..=buffer.read(cx).len());
                project.document_highlights(&buffer, offset, cx)
            });
            let highlights = cx.background().spawn(async move {
                highlights
                    .await
                    .map_err(|err| anyhow!("highlights request failed: {:?}", err))
            });
            if plan.lock().rng.gen_bool(0.3) {
                log::info!("{}: detaching highlights request", client.username);
                cx.update(|cx| highlights.detach_and_log_err(cx));
            } else {
                highlights.await?;
            }
        }
        55..=59 => {
            let search = project.update(cx, |project, cx| {
                let query = plan.lock().rng.gen_range('a'..='z');
                log::info!("{}: project-wide search {:?}", client.username, query);
                project.search(SearchQuery::text(query, false, false), cx)
            });
            let search = cx.background().spawn(async move {
                search
                    .await
                    .map_err(|err| anyhow!("search request failed: {:?}", err))
            });
            if plan.lock().rng.gen_bool(0.3) {
                log::info!("{}: detaching search request", client.username);
                cx.update(|cx| search.detach_and_log_err(cx));
            } else {
                let search = search.await?;
                client
                    .buffers_for_project(&project)
                    .extend(search.into_keys());
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
                if plan.lock().rng.gen_bool(0.7) {
                    buffer.randomly_edit(&mut plan.lock().rng, 5, cx);
                } else {
                    buffer.randomly_undo_redo(&mut plan.lock().rng, cx);
                }
            });
        }
    }

    Ok(())
}

fn choose_random_project(client: &TestClient, rng: &mut StdRng) -> Option<ModelHandle<Project>> {
    client
        .local_projects()
        .iter()
        .chain(client.remote_projects().iter())
        .choose(rng)
        .cloned()
}

fn gen_file_name(rng: &mut StdRng) -> String {
    let mut name = String::new();
    for _ in 0..10 {
        let letter = rng.gen_range('a'..='z');
        name.push(letter);
    }
    name
}
