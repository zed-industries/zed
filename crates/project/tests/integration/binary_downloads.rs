use std::{cell::RefCell, rc::Rc, time::Duration};

use fs::FakeFs;
use futures::{FutureExt, StreamExt};
use gpui::{TestAppContext, UpdateGlobal as _};
use language::{FakeLspAdapter, rust_lang};
use project::{
    Project,
    binary_downloads::{self, BinaryDownloads, BinaryDownloadsEvent, ToolInstall},
};
use serde_json::json;
use settings::{SettingsStore, WorktreeId};
use util::path;

use crate::init_test;

fn disable_downloads(cx: &mut TestAppContext) {
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(false);
            });
        });
    });
}

fn collect_install_requests(
    cx: &mut TestAppContext,
) -> Rc<RefCell<Vec<(Option<WorktreeId>, String)>>> {
    let requests: Rc<RefCell<Vec<(Option<WorktreeId>, String)>>> = Rc::default();
    cx.update({
        let requests = requests.clone();
        |cx| {
            let store = BinaryDownloads::try_get_global(cx).expect("global should be initialized");
            cx.subscribe(&store, move |_, event, _| {
                if let BinaryDownloadsEvent::InstallRequested(request) = event {
                    requests
                        .borrow_mut()
                        .push((request.worktree_id, request.tool.to_string()));
                }
            })
            .detach();
        }
    });
    requests
}

fn collect_resolved_installs(
    cx: &mut TestAppContext,
) -> Rc<RefCell<Vec<(Option<WorktreeId>, String)>>> {
    let resolved: Rc<RefCell<Vec<(Option<WorktreeId>, String)>>> = Rc::default();
    cx.update({
        let resolved = resolved.clone();
        |cx| {
            let store = BinaryDownloads::try_get_global(cx).expect("global should be initialized");
            cx.subscribe(&store, move |_, event, _| {
                if let BinaryDownloadsEvent::InstallResolved(request) = event {
                    resolved
                        .borrow_mut()
                        .push((request.worktree_id, request.tool.to_string()));
                }
            })
            .detach();
        }
    });
    resolved
}

#[gpui::test]
async fn test_install_resolved_emitted_on_approval_and_setting_flip(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(|cx| binary_downloads::init(cx));
    disable_downloads(cx);

    let resolved = collect_resolved_installs(cx);
    let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());

    // Approving a tool resolves it.
    store.update(cx, |store, cx| {
        store.request_tool_install(None, "lsp-a", cx)
    });
    store.update(cx, |store, cx| {
        store.approve_tool_install(None, "lsp-a", cx);
    });
    assert_eq!(
        resolved.borrow().clone(),
        vec![(None, "lsp-a".to_string())],
        "approving a tool emits InstallResolved"
    );

    store.update(cx, |store, cx| {
        store.request_tool_install(None, "lsp-b", cx)
    });
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(true);
            });
        });
    });
    cx.run_until_parked();

    assert_eq!(
        resolved.borrow().contains(&(None, "lsp-b".to_string())),
        true,
        "flipping the setting on resolves pending tools"
    );
}

#[gpui::test]
async fn test_install_prompt_emitted_and_starts_server_when_approved(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    cx.update(|cx| binary_downloads::init(cx));
    disable_downloads(cx);

    let requests = collect_install_requests(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let worktree_id = project.update(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "needs-download-language-server",
            ..Default::default()
        },
    );

    let (_buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();

    assert_eq!(
        requests.borrow().clone(),
        vec![(
            Some(worktree_id),
            "needs-download-language-server".to_string()
        )],
        "a single one-off install prompt should be requested while downloads are disabled"
    );

    let mut next_server = fake_servers.next().fuse();
    let mut timeout = cx.executor().timer(Duration::from_millis(200)).fuse();
    futures::select! {
        _ = next_server => panic!("server started before the install prompt was approved"),
        _ = timeout => {}
    }

    cx.update(|cx| {
        let store = BinaryDownloads::try_get_global(cx).unwrap();
        store.update(cx, |store, cx| {
            store.approve_tool_install(Some(worktree_id), "needs-download-language-server", cx);
        });
    });

    let mut next_server = fake_servers.next().fuse();
    let mut timeout = cx.executor().timer(Duration::from_secs(1)).fuse();
    futures::select! {
        server = next_server => assert_eq!(server.is_some(), true),
        _ = timeout => panic!("server should start once the install is approved"),
    }
}

#[gpui::test]
async fn test_lsp_waits_without_starting_until_setting_enabled(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    cx.update(|cx| binary_downloads::init(cx));
    disable_downloads(cx);

    let requests = collect_install_requests(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "needs-download-language-server",
            ..Default::default()
        },
    );

    let (_buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();

    let mut next_server = fake_servers.next().fuse();
    let mut timeout = cx.executor().timer(Duration::from_millis(200)).fuse();
    futures::select! {
        _ = next_server => panic!("server started while downloads were disabled"),
        _ = timeout => {}
    }
    assert_eq!(
        requests.borrow().len(),
        1,
        "the blocked server is registered for the downloads indicator and modal"
    );

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(true);
            });
        });
    });

    let mut next_server = fake_servers.next().fuse();
    let mut timeout = cx.executor().timer(Duration::from_secs(1)).fuse();
    futures::select! {
        server = next_server => assert_eq!(server.is_some(), true),
        _ = timeout => panic!("server should start after enabling downloads"),
    }
}

#[gpui::test]
async fn test_install_prompt_requested_once_per_tool(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(|cx| binary_downloads::init(cx));
    disable_downloads(cx);

    let requests = collect_install_requests(cx);
    let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());

    let first = store.update(cx, |store, cx| {
        store.request_tool_install(None, "lsp-a", cx)
    });
    let second = store.update(cx, |store, cx| {
        store.request_tool_install(None, "lsp-a", cx)
    });

    assert_eq!(first.is_some(), true);
    assert_eq!(second.is_some(), true);
    assert_eq!(
        requests.borrow().clone(),
        vec![(None, "lsp-a".to_string())],
        "the prompt should only be requested once per tool"
    );

    // A different tool prompts independently.
    store.update(cx, |store, cx| {
        store.request_tool_install(None, "lsp-b", cx)
    });
    assert_eq!(
        requests.borrow().len(),
        2,
        "a different tool should be prompted for separately"
    );

    // Approving lets future requests proceed without prompting again.
    store.update(cx, |store, cx| {
        store.approve_tool_install(None, "lsp-a", cx);
    });
    let after_approval = store.update(cx, |store, cx| {
        store.request_tool_install(None, "lsp-a", cx)
    });
    assert_eq!(
        after_approval.is_none(),
        true,
        "an approved tool should proceed immediately on subsequent requests"
    );
    assert_eq!(
        requests.borrow().len(),
        2,
        "approving a tool must not trigger another prompt"
    );
}

#[gpui::test]
async fn test_pending_install_unblocks_when_setting_flips(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(|cx| binary_downloads::init(cx));
    disable_downloads(cx);

    let requests = collect_install_requests(cx);
    let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());

    let receiver = store
        .update(cx, |store, cx| {
            store.request_tool_install(None, "lsp-a", cx)
        })
        .expect("a waiter is returned while downloads are disabled");
    assert_eq!(*receiver.borrow(), false);

    let again = store.update(cx, |store, cx| {
        store.request_tool_install(None, "lsp-a", cx)
    });
    assert_eq!(again.is_some(), true);
    assert_eq!(
        requests.borrow().len(),
        1,
        "repeat requests must not re-emit"
    );
    assert_eq!(
        store.read_with(cx, |store, _| store.pending_tool_installs().len()),
        1,
        "repeat requests keep a single pending entry"
    );

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(true);
            });
        });
    });
    cx.run_until_parked();

    assert_eq!(
        *receiver.borrow(),
        true,
        "the waiter should fire once downloads are allowed"
    );
}

#[gpui::test]
async fn test_tool_download_allowed_query(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(|cx| binary_downloads::init(cx));
    disable_downloads(cx);

    let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());

    assert_eq!(
        store.read_with(cx, |store, cx| store
            .tool_download_allowed(None, "lsp-a", cx)),
        false,
        "an undecided tool must not download while downloads are disabled"
    );

    store.update(cx, |store, cx| {
        store.approve_tool_install(None, "lsp-b", cx);
    });
    assert_eq!(
        store.read_with(cx, |store, cx| store
            .tool_download_allowed(None, "lsp-b", cx)),
        true,
        "an approved tool may download"
    );
    assert_eq!(
        cx.update(|cx| binary_downloads::tool_download_allowed(None, "lsp-b", cx)),
        true,
        "the free function mirrors the store method"
    );

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(true);
            });
        });
    });
    assert_eq!(
        store.read_with(cx, |store, cx| store
            .tool_download_allowed(None, "lsp-a", cx)),
        true,
        "turning the setting on allows every tool"
    );
}

#[gpui::test]
async fn test_tool_approval_is_scoped_to_tool_and_worktree(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(|cx| binary_downloads::init(cx));
    disable_downloads(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;
    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let worktree_id = project.update(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());

    store.update(cx, |store, cx| {
        store.approve_tool_install(None, "tool-x", cx);
    });

    assert_eq!(
        store.read_with(cx, |store, cx| store
            .tool_download_allowed(None, "tool-x", cx)),
        true,
        "the approved tool may download in its own scope"
    );
    assert_eq!(
        store.read_with(cx, |store, cx| store
            .tool_download_allowed(None, "tool-y", cx)),
        false,
        "approving one tool must not unlock another tool"
    );
    let other_tool_request = store.update(cx, |store, cx| {
        store.request_tool_install(None, "tool-y", cx)
    });
    assert_eq!(
        other_tool_request.is_some(),
        true,
        "another tool still has to request an install"
    );
    assert_eq!(
        store.read_with(cx, |store, cx| store.tool_download_allowed(
            Some(worktree_id),
            "tool-x",
            cx
        )),
        false,
        "a global approval must not unlock the worktree scope"
    );

    store.update(cx, |store, cx| {
        store.approve_tool_install(Some(worktree_id), "tool-z", cx);
    });
    assert_eq!(
        store.read_with(cx, |store, cx| store.tool_download_allowed(
            Some(worktree_id),
            "tool-z",
            cx
        )),
        true,
        "the worktree-scoped approval unlocks its own scope"
    );
    assert_eq!(
        store.read_with(cx, |store, cx| store
            .tool_download_allowed(None, "tool-z", cx)),
        false,
        "a worktree-scoped approval must not unlock the global scope"
    );
}

#[gpui::test]
async fn test_silent_waiter_wakes_on_approval_and_is_not_a_pending_install(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    cx.update(|cx| binary_downloads::init(cx));
    disable_downloads(cx);

    let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());

    let receiver = store
        .update(cx, |store, cx| {
            store.wait_until_tool_allowed(None, "tool-a", cx)
        })
        .expect("a silent waiter is returned while downloads are disabled");
    assert_eq!(*receiver.borrow(), false);
    assert_eq!(
        store.read_with(cx, |store, _| store.pending_tool_installs()),
        Vec::new(),
        "silent waiters must not show up as pending installs"
    );

    store.update(cx, |store, cx| {
        store.approve_tool_install(None, "tool-a", cx);
    });
    assert_eq!(
        *receiver.borrow(),
        true,
        "a one-off approval wakes the silent waiter"
    );
    let after_approval = store.update(cx, |store, cx| {
        store.wait_until_tool_allowed(None, "tool-a", cx)
    });
    assert_eq!(
        after_approval.is_none(),
        true,
        "an approved tool needs no waiter"
    );
}

#[gpui::test]
async fn test_worktree_removal_purges_waiters_and_cancels_receivers(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(|cx| binary_downloads::init(cx));
    disable_downloads(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;
    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let worktree_id = project.update(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());

    let receiver = store
        .update(cx, |store, cx| {
            store.request_tool_install(Some(worktree_id), "tool-a", cx)
        })
        .expect("a waiter is returned while downloads are disabled");
    assert_eq!(
        store.read_with(cx, |store, _| store.pending_tool_installs()),
        vec![ToolInstall {
            worktree_id: Some(worktree_id),
            tool: "tool-a".into(),
        }],
    );

    project.update(cx, |project, cx| {
        project.remove_worktree(worktree_id, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        store.read_with(cx, |store, _| store.pending_tool_installs()),
        Vec::new(),
        "removing the worktree purges its pending install"
    );
    assert_eq!(
        binary_downloads::await_downloads_allowed(Some(receiver), "tool-a").await,
        false,
        "the held receiver must resolve to a fail-closed cancel"
    );
}

#[gpui::test]
async fn test_npm_install_backstop_follows_setting_and_any_approval(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(|cx| binary_downloads::init(cx));
    disable_downloads(cx);

    let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());

    assert_eq!(
        store.read_with(cx, |store, cx| store
            .npm_install_backstop_permitted(None, "tool-a", cx)),
        false,
        "the backstop is closed while downloads are off and nothing is approved"
    );

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(true);
            });
        });
    });
    assert_eq!(
        store.read_with(cx, |store, cx| store
            .npm_install_backstop_permitted(None, "tool-a", cx)),
        true,
        "the backstop opens when the setting is on"
    );

    disable_downloads(cx);
    assert_eq!(
        store.read_with(cx, |store, cx| store
            .npm_install_backstop_permitted(None, "tool-a", cx)),
        false,
        "the backstop closes again when the setting flips off"
    );

    store.update(cx, |store, cx| {
        store.approve_tool_install(None, "tool-b", cx);
    });
    assert_eq!(
        store.read_with(cx, |store, cx| store
            .npm_install_backstop_permitted(None, "tool-a", cx)),
        true,
        "approving any tool opens the backstop for npm installs"
    );
}
