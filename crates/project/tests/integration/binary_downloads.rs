use std::{cell::RefCell, rc::Rc, time::Duration};

use fs::FakeFs;
use futures::{FutureExt, StreamExt};
use gpui::{TestAppContext, UpdateGlobal as _};
use language::{FakeLspAdapter, rust_lang};
use project::{
    Project,
    binary_downloads::{self, BinaryDownloads, BinaryDownloadsEvent},
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

fn set_prompt_to_install(prompt: bool, cx: &mut TestAppContext) {
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.prompt_to_install_binaries = Some(prompt);
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
        store.update(cx, |store, _| {
            store.resolve_tool_install(Some(worktree_id), "needs-download-language-server", true);
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
async fn test_no_install_prompt_when_prompting_disabled(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    cx.update(|cx| binary_downloads::init(cx));
    disable_downloads(cx);
    set_prompt_to_install(false, cx);

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
        requests.borrow().is_empty(),
        true,
        "no install prompt should be requested when prompting is disabled"
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
    store.update(cx, |store, _| {
        store.resolve_tool_install(None, "lsp-a", true);
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
async fn test_declined_install_still_unblocks_when_setting_flips(cx: &mut TestAppContext) {
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

    store.update(cx, |store, _| {
        store.resolve_tool_install(None, "lsp-a", false);
    });

    // Declining does not re-prompt on a subsequent request.
    let again = store.update(cx, |store, cx| {
        store.request_tool_install(None, "lsp-a", cx)
    });
    assert_eq!(again.is_some(), true);
    assert_eq!(requests.borrow().len(), 1, "declining must not re-prompt");

    // Flipping the global setting still unblocks the pending download.
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
        "the waiter should fire once downloads are allowed, even after declining"
    );
}
