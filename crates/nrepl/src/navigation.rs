//! Jumping to source locations reported by the nREPL server (definitions
//! from `info`, stacktrace frames).

use std::path::PathBuf;

use anyhow::Result;
use editor::Editor;
use gpui::{App, WeakEntity, Window};
use language::Point;
use util::ResultExt as _;
use workspace::{OpenOptions, Workspace};

/// Opens `path` in `workspace` and moves the cursor to the 1-based `line`
/// / `column`.
pub fn open_location(
    workspace: &WeakEntity<Workspace>,
    path: PathBuf,
    line: Option<u32>,
    column: Option<u32>,
    window: &mut Window,
    cx: &mut App,
) -> Result<()> {
    let open = workspace.update(cx, |workspace, cx| {
        workspace.open_abs_path(path, OpenOptions::default(), window, cx)
    })?;
    window
        .spawn(cx, async move |cx| {
            let item = open.await.log_err()?;
            let editor = item.downcast::<Editor>()?;
            let point = Point::new(
                line.unwrap_or(1).saturating_sub(1),
                column.unwrap_or(1).saturating_sub(1),
            );
            editor
                .update_in(cx, |editor, window, cx| {
                    editor.go_to_singleton_buffer_point(point, window, cx);
                })
                .log_err();
            Some(())
        })
        .detach();
    Ok(())
}
