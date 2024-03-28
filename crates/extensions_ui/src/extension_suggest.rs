use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, OnceLock};

use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use extension::ExtensionStore;
use gpui::{Entity, Model, VisualContext};
use language::Buffer;
use ui::ViewContext;
use workspace::{notifications::simple_message_notification, Workspace};

fn suggested_extensions() -> &'static HashMap<&'static str, Arc<str>> {
    static SUGGESTED: OnceLock<HashMap<&str, Arc<str>>> = OnceLock::new();
    SUGGESTED.get_or_init(|| {
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
            ("git-firefly", "MERGE_MSG"),
            ("git-firefly", "NOTES_EDITMSG"),
            ("git-firefly", "TAG_EDITMSG"),
            ("git-firefly", "git-rebase-todo"),
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
            ("toml", "Cargo.lock"),
            ("toml", "toml"),
            ("wgsl", "wgsl"),
            ("zig", "zig"),
        ]
        .into_iter()
        .map(|(name, file)| (file, name.into()))
        .collect()
    })
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SuggestedExtension {
    pub extension_id: Arc<str>,
    pub file_name_or_extension: Arc<str>,
}

/// Returns the suggested extension for the given [`Path`].
fn suggested_extension(path: impl AsRef<Path>) -> Option<SuggestedExtension> {
    let path = path.as_ref();

    let file_extension: Option<Arc<str>> = path
        .extension()
        .and_then(|extension| Some(extension.to_str()?.into()));
    let file_name: Option<Arc<str>> = path
        .file_name()
        .and_then(|file_name| Some(file_name.to_str()?.into()));

    let (file_name_or_extension, extension_id) = None
        // We suggest against file names first, as these suggestions will be more
        // specific than ones based on the file extension.
        .or_else(|| {
            file_name.clone().zip(
                file_name
                    .as_deref()
                    .and_then(|file_name| suggested_extensions().get(file_name)),
            )
        })
        .or_else(|| {
            file_extension.clone().zip(
                file_extension
                    .as_deref()
                    .and_then(|file_extension| suggested_extensions().get(file_extension)),
            )
        })?;

    Some(SuggestedExtension {
        extension_id: extension_id.clone(),
        file_name_or_extension,
    })
}

fn language_extension_key(extension_id: &str) -> String {
    format!("{}_extension_suggest", extension_id)
}

pub(crate) fn suggest(buffer: Model<Buffer>, cx: &mut ViewContext<Workspace>) {
    let Some(file) = buffer.read(cx).file().cloned() else {
        return;
    };

    let Some(SuggestedExtension {
        extension_id,
        file_name_or_extension,
    }) = suggested_extension(file.path())
    else {
        return;
    };

    let key = language_extension_key(&extension_id);
    let Ok(None) = KEY_VALUE_STORE.read_kvp(&key) else {
        return;
    };

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
                    "Do you want to install the recommended '{}' extension for '{}' files?",
                    extension_id, file_name_or_extension
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_suggested_extension() {
        assert_eq!(
            suggested_extension("Cargo.toml"),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "toml".into()
            })
        );
        assert_eq!(
            suggested_extension("Cargo.lock"),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "Cargo.lock".into()
            })
        );
        assert_eq!(
            suggested_extension("Dockerfile"),
            Some(SuggestedExtension {
                extension_id: "dockerfile".into(),
                file_name_or_extension: "Dockerfile".into()
            })
        );
        assert_eq!(
            suggested_extension("a/b/c/d/.gitignore"),
            Some(SuggestedExtension {
                extension_id: "git-firefly".into(),
                file_name_or_extension: ".gitignore".into()
            })
        );
        assert_eq!(
            suggested_extension("a/b/c/d/test.gleam"),
            Some(SuggestedExtension {
                extension_id: "gleam".into(),
                file_name_or_extension: "gleam".into()
            })
        );
    }
}
