use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use db::kvp::KEY_VALUE_STORE;

use editor::{Editor, EditorMode};
use extension::ExtensionStore;
use gpui::{AppContext, VisualContext};
use ui::WindowContext;
use util::ResultExt;
use workspace::{notifications::simple_message_notification, Workspace};

pub fn suggested_extension(file_extension_or_name: &str) -> Option<Arc<str>> {
    static SUGGESTED: OnceLock<HashMap<&str, Arc<str>>> = OnceLock::new();
    SUGGESTED
        .get_or_init(|| {
            [
                ("beancount", "beancount"),
                ("dockerfile", "Dockerfile"),
                ("elisp", "el"),
                ("fish", "fish"),
                ("git-firefly", ".gitconfig"),
                ("git-firefly", ".gitignore"),
                ("git-firefly", "git-rebase-todo"),
                ("git-firefly", "COMMIT_EDITMSG"),
                ("git-firefly", "EDIT_DESCRIPTION"),
                ("git-firefly", "MERGE_MSG"),
                ("git-firefly", "NOTES_EDITMSG"),
                ("git-firefly", "TAG_EDITMSG"),
                ("graphql", "gql"),
                ("graphql", "graphql"),
                ("java", "java"),
                ("nix", "nix"),
                ("kotlin", "kt"),
                ("latex", "tex"),
                ("make", "Makefile"),
                ("r", "r"),
                ("r", "R"),
                ("sql", "sql"),
                ("swift", "swift"),
                ("templ", "templ"),
                ("wgsl", "wgsl"),
            ]
            .into_iter()
            .map(|(name, file)| (file, name.into()))
            .collect::<HashMap<&str, Arc<str>>>()
        })
        .get(file_extension_or_name)
        .map(|str| str.clone())
}

pub(crate) fn init(cx: &mut AppContext) {
    cx.observe_new_views(move |editor: &mut Editor, cx| {
        if let EditorMode::Full = editor.mode() {
            let file_name_or_extension =
                editor.buffer().read(cx).as_singleton().and_then(|buffer| {
                    let buffer = buffer.read(cx);
                    if buffer.language().is_some() {
                        None
                    } else {
                        let path = buffer.file()?.path();
                        Some(match path.extension() {
                            Some(extension) => extension.to_str()?.to_string(),
                            None => path.to_str()?.to_string(),
                        })
                    }
                });

            let Some(file_name_or_extension) = file_name_or_extension else {
                return;
            };

            check_and_suggest(&file_name_or_extension, cx).log_err();
        }
    })
    .detach();
}

fn language_extension_key(extension_id: &str) -> String {
    format!("{}_extension_suggest", extension_id)
}

fn check_and_suggest(file_name_or_extension: &str, cx: &mut WindowContext) -> anyhow::Result<()> {
    let workspace = cx
        .window_handle()
        .downcast::<Workspace>()
        .map(|handle| handle.root(cx))
        .ok_or_else(|| anyhow::anyhow!("No workspace"))??;

    let Some(extension_id) = suggested_extension(&file_name_or_extension) else {
        return Ok(());
    };

    let key = language_extension_key(&extension_id);
    let value = KEY_VALUE_STORE.read_kvp(&key)?;

    if value.is_some() {
        return Ok(());
    }

    workspace.update(cx, |workspace, cx| {
        workspace.show_notification(0 as usize, cx, |cx| {
            cx.new_view(move |_cx| {
                simple_message_notification::MessageNotification::new(format!(
                    "Do you want to install the recommended '{}' extension?",
                    file_name_or_extension
                ))
                .with_click_message("Yes")
                .on_click({
                    let extension_id = extension_id.clone();
                    move |cx| {
                        let extension_id = extension_id.clone();
                        let extension_store = ExtensionStore::global(cx);
                        extension_store.update(cx, move |store, cx| {
                            store.install_latest_extension(extension_id, cx);
                        });
                    }
                })
                .with_secondary_click_message("No")
                .on_secondary_click(move |cx| {
                    let key = language_extension_key(&extension_id);
                    db::write_and_log(cx, move || {
                        KEY_VALUE_STORE.write_kvp(key, "dismissed".to_string())
                    });
                })
            })
        })
    });

    Ok(())
}
