use std::sync::Arc;

use futures::StreamExt;
use gpui::AppContext as _;
use settings::{DEFAULT_KEYMAP_PATH, KeymapFile, SettingsStore, watch_config_file};
use settings_ui::open_settings_editor;
use ui::BorrowAppContext;

fn merge_paths(a: &std::path::Path, b: &std::path::Path) -> std::path::PathBuf {
    let a_parts: Vec<_> = a.components().collect();
    let b_parts: Vec<_> = b.components().collect();

    let mut overlap = 0;
    for i in 0..=a_parts.len().min(b_parts.len()) {
        if a_parts[a_parts.len() - i..] == b_parts[..i] {
            overlap = i;
        }
    }

    let mut result = std::path::PathBuf::new();
    for part in &a_parts {
        result.push(part.as_os_str());
    }
    for part in &b_parts[overlap..] {
        result.push(part.as_os_str());
    }
    result
}

fn main() {
    zlog::init();
    zlog::init_output_stderr();

    let [crate_path, file_path] = [env!("CARGO_MANIFEST_DIR"), file!()].map(std::path::Path::new);
    let example_dir_abs_path = merge_paths(crate_path, file_path)
        .parent()
        .unwrap()
        .to_path_buf();

    let app = gpui::Application::new().with_assets(assets::Assets);

    let fs = Arc::new(fs::RealFs::new(None, app.background_executor()));
    let mut user_settings_file_rx = watch_config_file(
        &app.background_executor(),
        fs.clone(),
        paths::settings_file().clone(),
    );

    app.run(move |cx| {
        <dyn fs::Fs>::set_global(fs.clone(), cx);
        settings::init(cx);
        settings_ui::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        client::init_settings(cx);
        workspace::init_settings(cx);
        // production client because fake client requires gpui/test-support
        // and that causes issues with the real stuff we want to do
        let client = client::Client::production(cx);
        let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
        let languages = Arc::new(language::LanguageRegistry::new(
            cx.background_executor().clone(),
        ));

        client::init(&client, cx);

        project::Project::init(&client, cx);

        zlog::info!(
            "Creating fake worktree in {}",
            example_dir_abs_path.display(),
        );
        let project = project::Project::local(
            client.clone(),
            node_runtime::NodeRuntime::unavailable(),
            user_store,
            languages,
            fs.clone(),
            Some(Default::default()), // WARN: if None is passed here, prepare to be process bombed
            cx,
        );
        let worktree_task = project.update(cx, |project, cx| {
            project.create_worktree(example_dir_abs_path, true, cx)
        });
        cx.spawn(async move |_| {
            let worktree = worktree_task.await.unwrap();
            std::mem::forget(worktree);
        })
        .detach();
        std::mem::forget(project);

        language::init(cx);
        editor::init(cx);
        menu::init();

        let keybindings =
            KeymapFile::load_asset_allow_partial_failure(DEFAULT_KEYMAP_PATH, cx).unwrap();
        cx.bind_keys(keybindings);
        cx.spawn(async move |cx| {
            while let Some(content) = user_settings_file_rx.next().await {
                cx.update(|cx| {
                    cx.update_global(|store: &mut SettingsStore, cx| {
                        store.set_user_settings(&content, cx).unwrap()
                    })
                })
                .ok();
            }
        })
        .detach();

        open_settings_editor(cx).unwrap();
        cx.activate(true);
    });
}
