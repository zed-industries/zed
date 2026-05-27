use gpui::{App, Entity, WeakEntity};
use terminal::Terminal;
use terminal_view::{TerminalView, terminal_panel::TerminalPanel};
use workspace::Workspace;

pub fn send_to_terminal(
    text: &str,
    bracketed_paste: bool,
    workspace: &WeakEntity<Workspace>,
    cx: &mut App,
) {
    let terminal = match workspace.update(cx, |workspace, cx| find_active_terminal(workspace, cx)) {
        Ok(terminal) => terminal,
        Err(error) => {
            log::warn!("send_code: workspace was dropped before sending to terminal: {error}");
            return;
        }
    };
    let Some(terminal) = terminal else {
        log::warn!("send_code: no active terminal found in terminal panel");
        return;
    };

    let is_multiline = text.contains('\n') && text.trim_end_matches('\n').contains('\n');
    let use_bracketed = bracketed_paste && is_multiline;

    // Preserve any extra trailing newlines the caller included (e.g. for REPLs
    // that need a blank line to terminate a multi-line block) by counting before
    // we trim.
    let trailing_newlines = text.len() - text.trim_end_matches('\n').len();
    let text = text
        .trim_end_matches('\n')
        .trim_end_matches('\r')
        .to_string();

    if use_bracketed {
        terminal.update(cx, |terminal, _| {
            let sanitized = text.replace('\x1b', "");
            let paste_text = format!("\x1b[200~{}\x1b[201~", sanitized);
            terminal.input(paste_text.into_bytes());
            terminal.input(b"\r".to_vec());
            for _ in 1..trailing_newlines {
                terminal.input(b"\r".to_vec());
            }
        });
    } else {
        let extra_enters = trailing_newlines.saturating_sub(1);
        let lines: Vec<String> = text.lines().map(|l| l.to_string()).collect();
        cx.spawn({
            let terminal = terminal.downgrade();
            async move |cx| {
                for (i, line) in lines.iter().enumerate() {
                    let line = line.clone();
                    if terminal
                        .update(cx, |terminal, _| {
                            terminal.input(line.into_bytes());
                            terminal.input(b"\r".to_vec());
                        })
                        .is_err()
                    {
                        break;
                    }
                    if i < lines.len() - 1 {
                        cx.background_executor()
                            .timer(std::time::Duration::from_millis(50))
                            .await;
                    }
                }
                for _ in 0..extra_enters {
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(50))
                        .await;
                    if terminal
                        .update(cx, |terminal, _| {
                            terminal.input(b"\r".to_vec());
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            }
        })
        .detach();
    }
}

fn find_active_terminal(workspace: &Workspace, cx: &App) -> Option<Entity<Terminal>> {
    let terminal_panel = workspace.panel::<TerminalPanel>(cx)?;
    let active_pane = terminal_panel.read(cx).active_terminal_pane().clone();
    let terminal_view = active_pane
        .read(cx)
        .active_item()
        .and_then(|item| item.downcast::<TerminalView>())?;
    Some(terminal_view.read(cx).terminal().clone())
}
