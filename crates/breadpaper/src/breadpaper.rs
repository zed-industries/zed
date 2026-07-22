pub mod areas;
pub mod history;
pub mod notes;
pub mod timeline_panel;
pub mod vault;

use anyhow::{Context as _, Result};
use editor::Editor;
use gpui::{App, AppContext as _, AsyncWindowContext, Task, WeakEntity};
use markdown_preview::markdown_preview_view::MarkdownPreviewView;
use std::path::PathBuf;
use std::sync::Arc;
use workspace::{AppState, OpenOptions, OpenVisible, Workspace};

pub use timeline_panel::{TimelinePanel, init, show_panel_if_vault};
pub use vault::{Vault, VaultStatus, default_vault_path, scaffold_vault};

/// Opens `path` and lands the user on a rendered markdown preview of it
/// ("viewing mode") instead of the raw buffer. There is no one-shot
/// open-as-preview API, so this opens the file as an editor first and then
/// attaches an independent preview item to the same pane.
pub async fn open_abs_path_as_preview(
    workspace: WeakEntity<Workspace>,
    path: PathBuf,
    cx: &mut AsyncWindowContext,
) -> Result<()> {
    let item = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_abs_path(
                path.clone(),
                OpenOptions {
                    visible: Some(OpenVisible::All),
                    ..Default::default()
                },
                window,
                cx,
            )
        })?
        .await?;
    let editor = item
        .downcast::<Editor>()
        .with_context(|| format!("{} did not open as a markdown editor", path.display()))?;
    workspace.update_in(cx, |workspace, window, cx| {
        let pane = workspace.active_pane().clone();
        MarkdownPreviewView::open_preview_in_pane(workspace, editor, pane, window, cx);
    })?;
    Ok(())
}

/// Opens the default vault as the workspace, scaffolding the sample vault
/// first if it doesn't exist yet. On a fresh scaffold, `welcome.md` is opened
/// alongside so the user lands on something oriented. Used at startup when
/// there is no previous session to restore.
pub fn open_startup_vault(app_state: Arc<AppState>, cx: &mut App) -> Task<Result<()>> {
    let vault_root = vault::default_vault_path();
    let scaffold = cx.background_spawn({
        let vault_root = vault_root.clone();
        async move {
            let already_vault = vault_root
                .join(vault::VAULT_MARKER_DIR)
                .join(vault::VAULT_CONFIG_FILE)
                .is_file();
            if !already_vault {
                vault::scaffold_vault(&vault_root)?;
            }
            anyhow::Ok(!already_vault)
        }
    });

    cx.spawn(async move |cx| {
        let open_result = async {
            let freshly_scaffolded = scaffold.await?;
            let mut paths = vec![vault_root.clone()];
            if freshly_scaffolded {
                paths.push(vault_root.join(vault::WELCOME_FILE));
            }
            cx.update(|cx| {
                workspace::open_paths(
                    &paths,
                    app_state.clone(),
                    workspace::OpenOptions::default(),
                    cx,
                )
            })
            .await?;
            anyhow::Ok(())
        }
        .await;

        if let Err(error) = open_result {
            log::error!(
                "BreadPaper: couldn't open the default vault, falling back to an empty workspace: {error:?}"
            );
            cx.update(|cx| workspace::open_new(Default::default(), app_state, cx, |_, _, _| {}))
                .await?;
        }
        Ok(())
    })
}
