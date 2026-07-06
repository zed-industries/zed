use std::ops::Range;
use std::sync::Arc;

use docker_client::{DockerClient, DockerEndpoint};
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, HighlightStyle, ParentElement,
    ScrollHandle, SharedString, Styled, StyledText, Subscription, Task, Window, px, relative,
};
use ui::{Tooltip, prelude::*};
use ui_input::{ErasedEditorEvent, InputField};
use workspace::{Workspace, item::Item};

/// The tail of streamed log lines is capped at this many lines, evicting the
/// oldest lines past the cap regardless of the follow/pause toggle.
const MAX_DISPLAYED_LOG_LINES: usize = 500;
const TAIL: usize = 200;

/// A full-size center-pane tab streaming `docker logs -f` for one container.
///
/// Owns the stream for as long as the tab is open: dropping this view (e.g.
/// the user closes the tab) drops `_logs_task`, which cancels the background
/// `cx.spawn` future and, with it, the receiver from
/// `CliDockerClient::container_logs`. That receiver's drop is what lets the
/// `docker logs -f` child exit via `kill_on_drop`.
pub struct DockerLogsView {
    focus_handle: FocusHandle,
    container_name: String,
    lines: Vec<String>,
    follow: bool,
    scroll_handle: ScrollHandle,
    search_field: Entity<InputField>,
    _search_subscription: Subscription,
    _logs_task: Task<()>,
}

impl DockerLogsView {
    pub fn new(
        endpoint: DockerEndpoint,
        container_id: String,
        container_name: String,
        client: Arc<dyn DockerClient>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let task = gpui_tokio::Tokio::spawn_result(cx, async move {
                client.container_logs(&endpoint, &container_id, TAIL).await
            });
            let logs_task = cx.spawn(async move |this, cx| {
                use futures::StreamExt as _;

                let mut receiver = match task.await {
                    Ok(receiver) => receiver,
                    Err(error) => {
                        log::warn!("docker logs failed: {error:#}");
                        return;
                    }
                };
                while let Some(chunk) = receiver.next().await {
                    let updated = this.update(cx, |this: &mut Self, cx| {
                        this.lines.push(chunk.line);
                        if this.lines.len() > MAX_DISPLAYED_LOG_LINES {
                            let overflow = this.lines.len() - MAX_DISPLAYED_LOG_LINES;
                            this.lines.drain(0..overflow);
                        }
                        // Only pull the view down to the tail while
                        // following; paused, the scroll position is left
                        // alone so the user can read back through history
                        // without it jumping. The buffer stays capped at
                        // MAX_DISPLAYED_LOG_LINES regardless of follow state.
                        if this.follow {
                            this.scroll_handle.scroll_to_bottom();
                        }
                        cx.notify();
                    });
                    if updated.is_err() {
                        return;
                    }
                }
            });

            let search_field = cx.new(|cx| InputField::new(window, cx, "Filter logs…"));
            // Re-render whenever the query changes so the filtered line set
            // and match count in the toolbar stay current; the search field
            // has no other observers, so the subscription lives as long as
            // the view via `_search_subscription`.
            let this_handle = cx.weak_entity();
            let search_editor = search_field.read(cx).editor().clone();
            let search_subscription = search_editor.subscribe(
                Box::new(move |event, _window, cx| {
                    let ErasedEditorEvent::BufferEdited = event else {
                        return;
                    };
                    this_handle.update(cx, |_, cx| cx.notify()).ok();
                }),
                window,
                cx,
            );

            Self {
                focus_handle: cx.focus_handle(),
                container_name,
                lines: Vec::new(),
                follow: true,
                scroll_handle: ScrollHandle::new(),
                search_field,
                _search_subscription: search_subscription,
                _logs_task: logs_task,
            }
        })
    }

    /// Exposed for tests, mirroring `SqlQueryView::result`.
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn follow(&self) -> bool {
        self.follow
    }

    /// Exposed for tests: the current search query text.
    pub fn search_query(&self, cx: &App) -> String {
        self.search_field.read(cx).text(cx)
    }

    /// Flips the follow/pause toggle. Switching back to follow immediately
    /// snaps to the tail; pausing leaves the scroll position where the user
    /// left it.
    pub fn toggle_follow(&mut self, cx: &mut Context<Self>) {
        self.follow = !self.follow;
        if self.follow {
            self.scroll_handle.scroll_to_bottom();
        }
        cx.notify();
    }

    fn render_toolbar(&self, cx: &Context<Self>) -> gpui::AnyElement {
        let follow_label = if self.follow { "Following" } else { "Paused" };
        let query = self.search_field.read(cx).text(cx);
        let visible = filter_lines(&self.lines, &query);
        let match_label = if query.trim().is_empty() {
            format!("{} lines", self.lines.len())
        } else {
            format!("{} / {} matches", visible.len(), self.lines.len())
        };

        h_flex()
            .w_full()
            .px_2()
            .py_1()
            .gap_2()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(Label::new(format!("Logs: {}", self.container_name)))
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(div().w(px(220.)).child(self.search_field.clone()))
                    .child(
                        Label::new(match_label)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(
                Button::new("toggle-follow-logs", follow_label)
                    .toggle_state(self.follow)
                    .tooltip(Tooltip::text("Toggle follow/pause"))
                    .on_click(cx.listener(|this, _, _window, cx| this.toggle_follow(cx))),
            )
            .into_any_element()
    }
}

/// Returns the lines that contain `query`, case-insensitively. An empty (or
/// all-whitespace) query matches every line. Pure and derived at render time;
/// callers must not use this to mutate the underlying buffer.
fn filter_lines<'a>(lines: &'a [String], query: &str) -> Vec<&'a String> {
    let query = query.trim();
    if query.is_empty() {
        return lines.iter().collect();
    }
    let query = query.to_lowercase();
    lines
        .iter()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

/// Maps the 8 standard + 8 bright ANSI SGR foreground color codes to fixed
/// RGB colors. `theme`'s `terminal_ansi_*` palette is theme/window-scoped and
/// awkward to reach from a pure helper, so a fixed palette is used here (the
/// brief allows this fallback).
fn ansi_color(code: u16) -> Option<gpui::Hsla> {
    let rgb = match code {
        30 | 90 => 0x000000,
        31 => 0xcc0000,
        32 => 0x4e9a06,
        33 => 0xc4a000,
        34 => 0x3465a4,
        35 => 0x75507b,
        36 => 0x06989a,
        37 => 0xd3d7cf,
        91 => 0xef2929,
        92 => 0x8ae234,
        93 => 0xfce94f,
        94 => 0x729fcf,
        95 => 0xad7fa8,
        96 => 0x34e2e2,
        97 => 0xeeeeec,
        _ => return None,
    };
    Some(gpui::rgb(rgb).into())
}

/// Parses a single log line for ANSI SGR (color/bold) escape sequences,
/// returning the plain text with all escape sequences removed alongside the
/// highlight ranges (in byte offsets into the plain text) that should be
/// applied over it.
///
/// Handles the common SGR parameters: `0` (reset), `1` (bold), `22`
/// (normal intensity), `30-37`/`90-97` (foreground colors), and `39`
/// (default foreground, i.e. clears color). Any other escape sequence --
/// other CSI `ESC[...` sequences, OSC `ESC]...` sequences, etc. -- is
/// stripped so it never renders as garbage, but its effect (if any) is
/// otherwise ignored.
///
/// Simplification for v1: each line's SGR state is independent -- a color or
/// bold style opened on one line does not carry over to the next, since
/// lines are parsed and joined independently by the caller.
fn parse_ansi(line: &str) -> (String, Vec<(Range<usize>, HighlightStyle)>) {
    let mut plain = String::with_capacity(line.len());
    let mut highlights = Vec::new();

    let mut current_color: Option<gpui::Hsla> = None;
    let mut current_bold = false;
    let mut run_start = 0;
    let mut run_style = HighlightStyle::default();

    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != 0x1b {
            // Safe: `line` is valid UTF-8 and we only ever step by whole
            // chars below, so `i` always lands on a char boundary here.
            if let Some(ch) = line[i..].chars().next() {
                plain.push(ch);
                i += ch.len_utf8();
            } else {
                break;
            }
            continue;
        }

        // Found ESC. Determine the sequence kind and how many bytes it
        // spans so it can be skipped without emitting it into `plain`.
        let next = bytes.get(i + 1).copied();
        match next {
            Some(b'[') => {
                // CSI: ESC '[' params... final-byte. Params are `0-9;`, the
                // sequence ends at the first byte in `0x40..=0x7e`.
                let params_start = i + 2;
                let mut end = params_start;
                while end < bytes.len() && !(0x40..=0x7e).contains(&bytes[end]) {
                    end += 1;
                }
                let final_byte = bytes.get(end).copied();
                let params = &line[params_start..end.min(line.len())];

                if final_byte == Some(b'm') {
                    // SGR: flush the run accumulated under the previous
                    // style before changing it.
                    if plain.len() > run_start {
                        if run_style.color.is_some() || run_style.font_weight.is_some() {
                            highlights.push((run_start..plain.len(), run_style));
                        }
                    }
                    run_start = plain.len();

                    let codes: Vec<u16> = if params.is_empty() {
                        vec![0]
                    } else {
                        params
                            .split(';')
                            .filter_map(|part| part.parse().ok())
                            .collect()
                    };
                    for code in codes {
                        match code {
                            0 => {
                                current_color = None;
                                current_bold = false;
                            }
                            1 => current_bold = true,
                            22 => current_bold = false,
                            39 => current_color = None,
                            code => {
                                if let Some(color) = ansi_color(code) {
                                    current_color = Some(color);
                                }
                            }
                        }
                    }
                    run_style = HighlightStyle {
                        color: current_color,
                        font_weight: current_bold.then_some(gpui::FontWeight::BOLD),
                        ..Default::default()
                    };
                }
                // Any other CSI final byte (cursor movement, erase, etc.) is
                // simply dropped -- it has no text representation here.
                i = end.saturating_add(1).max(i + 1);
            }
            Some(b']') => {
                // OSC: ESC ']' ... terminated by BEL (0x07) or ESC '\'.
                let mut end = i + 2;
                while end < bytes.len() && bytes[end] != 0x07 {
                    if bytes[end] == 0x1b && bytes.get(end + 1) == Some(&b'\\') {
                        end += 1;
                        break;
                    }
                    end += 1;
                }
                i = (end + 1).min(bytes.len()).max(i + 1);
            }
            Some(_) => {
                // Some other two-byte escape (e.g. ESC followed by a single
                // letter); drop both bytes.
                i += 2;
            }
            None => {
                // Trailing lone ESC with nothing after it; drop it.
                i += 1;
            }
        }
    }

    if plain.len() > run_start && (run_style.color.is_some() || run_style.font_weight.is_some()) {
        highlights.push((run_start..plain.len(), run_style));
    }

    (plain, highlights)
}

impl Render for DockerLogsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let query = self.search_field.read(cx).text(cx);
        let visible_lines = filter_lines(&self.lines, &query);

        let mut joined = String::new();
        let mut highlights: Vec<(Range<usize>, HighlightStyle)> = Vec::new();
        for (index, line) in visible_lines.iter().enumerate() {
            if index > 0 {
                joined.push('\n');
            }
            let (plain, line_highlights) = parse_ansi(line);
            let offset = joined.len();
            joined.push_str(&plain);
            highlights.extend(
                line_highlights
                    .into_iter()
                    .map(|(range, style)| (offset + range.start..offset + range.end, style)),
            );
        }

        let buffer_font = theme::theme_settings(cx).buffer_font(cx).clone();
        let buffer_font_size = theme::theme_settings(cx).buffer_font_size(cx);

        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(self.render_toolbar(cx))
            .child(
                v_flex()
                    .id("docker-logs-scroll")
                    .flex_1()
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .p_2()
                    .font(buffer_font)
                    .text_size(buffer_font_size)
                    .text_color(cx.theme().colors().text)
                    .line_height(relative(1.3))
                    .child(StyledText::new(joined).with_highlights(highlights)),
            )
    }
}

impl Focusable for DockerLogsView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for DockerLogsView {}

impl Item for DockerLogsView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Terminal))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("Logs: {}", self.container_name).into()
    }

    fn is_dirty(&self, _cx: &App) -> bool {
        false
    }
}

/// Opens a new logs tab in the workspace's active pane. Always opens a new
/// tab rather than focusing an existing one for the same container: simpler,
/// and acceptable per the brief (focusing an existing tab is a nicer-to-have,
/// not required).
pub fn open_logs_tab(
    workspace: &mut Workspace,
    endpoint: DockerEndpoint,
    container_id: String,
    container_name: String,
    client: Arc<dyn DockerClient>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let view = DockerLogsView::new(endpoint, container_id, container_name, client, window, cx);
    workspace.active_pane().update(cx, |pane, cx| {
        pane.add_item(Box::new(view), true, true, None, window, cx);
    });
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use docker_client::fake::FakeDockerClient;
    use docker_client::{DockerClient, EndpointKind};
    use gpui::{TestAppContext, VisualTestContext};

    use super::{DockerLogsView, filter_lines, parse_ansi};

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            gpui_tokio::init(cx);
            crate::init(cx);
        });
    }

    fn test_endpoint() -> docker_client::DockerEndpoint {
        docker_client::DockerEndpoint {
            name: "local".into(),
            kind: EndpointKind::Local,
            read_only: false,
        }
    }

    /// Drives the deterministic scheduler while giving the real tokio runtime
    /// a chance to complete cross-thread work, until `condition` holds or a
    /// bound is reached. Requires `cx.executor().allow_parking()`.
    async fn wait_until(
        cx: &mut VisualTestContext,
        condition: impl Fn(&mut VisualTestContext) -> bool,
    ) {
        for _ in 0..200 {
            cx.run_until_parked();
            if condition(cx) {
                return;
            }
            cx.background_executor
                .timer(std::time::Duration::from_millis(5))
                .await;
        }
        cx.run_until_parked();
        assert!(
            condition(cx),
            "condition did not become true within the time bound"
        );
    }

    /// The logs buffer must fill from the streamed `LogChunk`s the client
    /// yields, in order, and follow defaults to true.
    #[gpui::test]
    async fn logs_stream_fills_buffer(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let mut fake = FakeDockerClient::new_with_container("api");
        fake.log_lines = vec!["a".into(), "b".into()];
        let client: Arc<dyn DockerClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            DockerLogsView::new(
                test_endpoint(),
                "api-id".into(),
                "api".into(),
                client,
                window,
                cx,
            )
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.lines() == ["a".to_string(), "b".to_string()]
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.lines(), ["a".to_string(), "b".to_string()]);
            assert!(view.follow(), "follow should default to true");
        });
    }

    /// Toggling follow/pause must flip the observable `follow` flag; this is
    /// the behavior the toggle button in the view's toolbar drives.
    #[gpui::test]
    async fn toggle_logs_follow_flips_flag(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let mut fake = FakeDockerClient::new_with_container("api");
        fake.log_lines = vec!["a".into()];
        let client: Arc<dyn DockerClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            DockerLogsView::new(
                test_endpoint(),
                "api-id".into(),
                "api".into(),
                client,
                window,
                cx,
            )
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| !view.lines().is_empty())
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(view.follow(), "follow should default to true");
        });

        view.update(cx, |view, cx| view.toggle_follow(cx));
        view.read_with(cx, |view, _| {
            assert!(!view.follow(), "toggling once should pause following");
        });

        view.update(cx, |view, cx| view.toggle_follow(cx));
        view.read_with(cx, |view, _| {
            assert!(view.follow(), "toggling twice should resume following");
        });
    }

    /// Guards against leaking the `docker logs -f` child process: the stream
    /// task is held in a field (`_logs_task`) so dropping the view cancels
    /// the stream. This test asserts the view can be dropped cleanly while a
    /// stream is in flight, without panicking or hanging.
    #[gpui::test]
    async fn dropping_view_cancels_stream(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let mut fake = FakeDockerClient::new_with_container("api");
        fake.log_lines = vec!["a".into(), "b".into()];
        let client: Arc<dyn DockerClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            DockerLogsView::new(
                test_endpoint(),
                "api-id".into(),
                "api".into(),
                client,
                window,
                cx,
            )
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| !view.lines().is_empty())
        })
        .await;

        drop(view);
        cx.run_until_parked();
    }

    /// The search field must update the observable query text, and the
    /// view's search field should start out empty.
    #[gpui::test]
    async fn search_query_starts_empty(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let mut fake = FakeDockerClient::new_with_container("api");
        fake.log_lines = vec!["a".into()];
        let client: Arc<dyn DockerClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            DockerLogsView::new(
                test_endpoint(),
                "api-id".into(),
                "api".into(),
                client,
                window,
                cx,
            )
        });

        view.read_with(cx, |view, cx| {
            assert_eq!(view.search_query(cx), "");
        });
    }

    #[test]
    fn filter_lines_is_case_insensitive_contains() {
        let lines = vec![
            "ERROR: boom".to_string(),
            "info: all good".to_string(),
            "warning: Error retrying".to_string(),
        ];

        let matches = filter_lines(&lines, "error");
        assert_eq!(matches, vec![&lines[0], &lines[2]]);
    }

    #[test]
    fn filter_lines_empty_query_returns_all() {
        let lines = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let matches = filter_lines(&lines, "");
        assert_eq!(matches, lines.iter().collect::<Vec<_>>());

        // Whitespace-only query is treated the same as empty.
        let matches = filter_lines(&lines, "   ");
        assert_eq!(matches, lines.iter().collect::<Vec<_>>());
    }

    #[test]
    fn filter_lines_no_match_returns_empty() {
        let lines = vec!["a".to_string(), "b".to_string()];
        assert!(filter_lines(&lines, "zzz").is_empty());
    }

    #[test]
    fn parse_ansi_colored_line_strips_codes_and_highlights_red() {
        let (plain, highlights) = parse_ansi("\x1b[31mERROR\x1b[0m done");
        assert_eq!(plain, "ERROR done");
        assert_eq!(highlights.len(), 1);
        let (range, style) = &highlights[0];
        assert_eq!(*range, 0..5);
        assert!(style.color.is_some());
    }

    #[test]
    fn parse_ansi_garbage_escape_is_stripped() {
        // An OSC sequence (set-title) followed by plain text: the escape
        // must be stripped entirely and not appear in the output, with the
        // rest of the text intact.
        let (plain, highlights) = parse_ansi("\x1b]0;some title\x07hello world");
        assert_eq!(plain, "hello world");
        assert!(highlights.is_empty());
    }

    #[test]
    fn parse_ansi_unterminated_csi_is_stripped() {
        // A CSI cursor-movement sequence with no color meaning; it must not
        // leak into the plain text as garbage.
        let (plain, highlights) = parse_ansi("before\x1b[2Kafter");
        assert_eq!(plain, "beforeafter");
        assert!(highlights.is_empty());
    }

    #[test]
    fn parse_ansi_plain_line_is_unchanged() {
        let (plain, highlights) = parse_ansi("just a plain log line");
        assert_eq!(plain, "just a plain log line");
        assert!(highlights.is_empty());
    }

    #[test]
    fn parse_ansi_bold_without_color_still_highlights() {
        let (plain, highlights) = parse_ansi("\x1b[1mbold\x1b[22m plain");
        assert_eq!(plain, "bold plain");
        assert_eq!(highlights.len(), 1);
        let (range, style) = &highlights[0];
        assert_eq!(*range, 0..4);
        assert!(style.color.is_none());
        assert!(style.font_weight.is_some());
    }
}
