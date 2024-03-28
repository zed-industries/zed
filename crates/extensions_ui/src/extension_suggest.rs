use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use db::kvp::KEY_VALUE_STORE;

use editor::Editor;
use extension::ExtensionStore;
use gpui::{Entity, Model, VisualContext};
use language::Buffer;
use ui::ViewContext;
use workspace::{notifications::simple_message_notification, Workspace};

pub fn suggested_extension(file_extension_or_name: &str) -> Option<Arc<str>> {
    static SUGGESTED: OnceLock<HashMap<&str, Arc<str>>> = OnceLock::new();
    SUGGESTED
        .get_or_init(|| {
            [
                ("astro", "astro"),
                ("beancount", "beancount"),
                ("dockerfile", "Dockerfile"),
                ("elisp", "el"),
                ("fish", "fish"),
                ("git-firefly", ".gitconfig"),
                ("git-firefly", ".gitignore"),
                ("git-firefly", "COMMIT_EDITMSG"),
                ("git-firefly", "EDIT_DESCRIPTION"),
                ("git-firefly", "git-rebase-todo"),
                ("git-firefly", "MERGE_MSG"),
                ("git-firefly", "NOTES_EDITMSG"),
                ("git-firefly", "TAG_EDITMSG"),
                ("gleam", "gleam"),
                ("graphql", "gql"),
                ("graphql", "graphql"),
                ("haskell", "hs"),
                ("java", "java"),
                ("kotlin", "kt"),
                ("latex", "tex"),
                ("make", "Makefile"),
                ("nix", "nix"),
                ("prisma", "prisma"),
                ("purescript", "purs"),
                ("r", "r"),
                ("r", "R"),
                ("sql", "sql"),
                ("svelte", "svelte"),
                ("swift", "swift"),
                ("templ", "templ"),
                ("wgsl", "wgsl"),
                ("zig", "zig"),
            ]
            .into_iter()
            .map(|(name, file)| (file, name.into()))
            .collect::<HashMap<&str, Arc<str>>>()
        })
        .get(file_extension_or_name)
        .map(|str| str.clone())
}

fn language_extension_key(extension_id: &str) -> String {
    format!("{}_extension_suggest", extension_id)
}

pub(crate) fn suggest(buffer: Model<Buffer>, cx: &mut ViewContext<Workspace>) {
    let Some(file_name_or_extension) = buffer.read(cx).file().and_then(|file| {
        Some(match file.path().extension() {
            Some(extension) => extension.to_str()?.to_string(),
            None => file.path().to_str()?.to_string(),
        })
    }) else {
        return;
    };

    let Some(extension_id) = suggested_extension(&file_name_or_extension) else {
        return;
    };

    let key = language_extension_key(&extension_id);
    let value = KEY_VALUE_STORE.read_kvp(&key);

    if value.is_err() || value.unwrap().is_some() {
        return;
    }

    cx.on_next_frame(move |workspace, cx| {
        let Some(editor) = workspace.active_item_as::<Editor>(cx) else {
            return;
        };

        if editor.read(cx).buffer().read(cx).as_singleton().as_ref() != Some(&buffer) {
            return;
        }

        workspace.show_notification(buffer.entity_id().as_u64() as usize, cx, |cx| {
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
        });
    })
}
