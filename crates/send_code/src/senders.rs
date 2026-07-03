use gpui::{App, Entity, WeakEntity};
use terminal::Terminal;
use terminal_view::{TerminalView, terminal_panel::TerminalPanel};
use workspace::{Workspace, dock::Panel};

pub fn send_to_terminal(
    text: &str,
    bracketed_paste: bool,
    language_name: Option<&str>,
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

    let trailing_newlines = text.len() - text.trim_end_matches('\n').len();
    let text = text
        .trim_end_matches(|character| character == '\n' || character == '\r')
        .to_string();

    if text.is_empty() {
        terminal.update(cx, |terminal, _| {
            for _ in 0..trailing_newlines.max(1) {
                terminal.input(b"\r".to_vec());
            }
        });
        return;
    }

    let is_multiline = text.contains('\n');
    let use_bracketed = bracketed_paste && is_multiline;

    if language_name == Some("Python") && is_multiline && is_ipython_terminal(&terminal, cx) {
        send_to_ipython_cpaste(&text, terminal, cx);
        return;
    }

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
        let lines: Vec<String> = text.lines().map(str::to_string).collect();
        cx.spawn({
            let terminal = terminal.downgrade();
            async move |cx| {
                for (index, line) in lines.iter().enumerate() {
                    let line = line.clone();
                    if let Err(error) = terminal.update(cx, |terminal, _| {
                        terminal.input(line.into_bytes());
                        terminal.input(b"\r".to_vec());
                    }) {
                        log::warn!("send_code: terminal was dropped while sending input: {error}");
                        break;
                    }
                    if index < lines.len() - 1 {
                        cx.background_executor()
                            .timer(std::time::Duration::from_millis(50))
                            .await;
                    }
                }
                for _ in 0..extra_enters {
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(50))
                        .await;
                    if let Err(error) = terminal.update(cx, |terminal, _| {
                        terminal.input(b"\r".to_vec());
                    }) {
                        log::warn!(
                            "send_code: terminal was dropped while sending extra enter: {error}"
                        );
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
    let active_pane = terminal_panel.read(cx).pane()?;
    let terminal_view = active_pane
        .read(cx)
        .active_item()
        .and_then(|item| item.downcast::<TerminalView>())?;
    Some(terminal_view.read(cx).terminal().clone())
}

fn is_ipython_terminal(terminal: &Entity<Terminal>, cx: &App) -> bool {
    let content = terminal.read(cx).last_content();
    let cursor_line = content.cursor.point.line;
    let mut text = String::new();
    for cell in &content.cells {
        if cell.point.line == cursor_line {
            text.push(cell.character());
        }
    }
    is_ipython_prompt(&text)
}

fn is_ipython_prompt(text: &str) -> bool {
    text.contains("In [") && text.contains("]:")
}

fn send_to_ipython_cpaste(text: &str, terminal: Entity<Terminal>, cx: &mut App) {
    let text = format!("{}\r--\r", text.replace('\n', "\r"));
    terminal.update(cx, |terminal, _| {
        terminal.input(b"%cpaste -q\r".to_vec());
    });

    cx.spawn({
        let terminal = terminal.downgrade();
        async move |cx| {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(100))
                .await;

            if let Err(error) = terminal.update(cx, |terminal, _| {
                terminal.input(text.into_bytes());
            }) {
                log::warn!("send_code: terminal was dropped while sending IPython cpaste: {error}");
            }
        }
    })
    .detach();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_ipython_prompt() {
        assert!(is_ipython_prompt("In [12]: "));
        assert!(!is_ipython_prompt(">>> "));
        assert!(!is_ipython_prompt("... "));
    }
}
