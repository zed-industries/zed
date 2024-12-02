use anyhow::{Context as _, Result};
use collections::{BTreeMap, HashMap};
use gpui::{AppContext, Context, Global, Model, ModelContext, Task};
use http_client::HttpClient;
use language::{
    language_settings::all_language_settings, Anchor, Buffer, BufferSnapshot, OffsetRangeExt,
    Point, Rope, ToOffset, ToPoint,
};
use std::{
    borrow::Cow,
    cmp,
    fmt::Write,
    mem,
    ops::Range,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

const CURSOR_MARKER: &'static str = "<|user_cursor_is_here|>";
const START_OF_FILE_MARKER: &'static str = "<|start_of_file|>";
const EDITABLE_REGION_START_MARKER: &'static str = "<|editable_region_start|>";
const EDITABLE_REGION_END_MARKER: &'static str = "<|editable_region_end|>";
const ORIGINAL_MARKER: &str = "<<<<<<< ORIGINAL\n";
const SEPARATOR_MARKER: &str = "\n=======\n";
const UPDATED_MARKER: &str = "\n>>>>>>> UPDATED";
const BUFFER_CHANGE_GROUPING_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct InlineCompletionId(usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct EventId(usize);

#[derive(Clone)]
struct ZetaGlobal(Model<Zeta>);

impl Global for ZetaGlobal {}

pub struct Zeta {
    http_client: Arc<dyn HttpClient>,
    api_url: Arc<str>,
    api_key: Arc<str>,
    model: Arc<str>,
    events: BTreeMap<EventId, Event>,
    next_inline_completion_id: InlineCompletionId,
    next_event_id: EventId,
    registered_buffers: HashMap<gpui::EntityId, RegisteredBuffer>,
}

pub struct InlineCompletion {
    id: InlineCompletionId,
    path: Arc<Path>,
    edits: Vec<(Range<Anchor>, String)>,
    snapshot: BufferSnapshot,
}

impl std::fmt::Debug for InlineCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InlineCompletion")
            .field("id", &self.id)
            .field("path", &self.path)
            .field("edits", &self.edits)
            .finish_non_exhaustive()
    }
}

impl Zeta {
    pub fn global(cx: &mut AppContext) -> Model<Self> {
        cx.try_global::<ZetaGlobal>()
            .map(|global| global.0.clone())
            .unwrap_or_else(|| {
                let model = cx.new_model(|cx| Self::production(cx));
                cx.set_global(ZetaGlobal(model.clone()));
                model
            })
    }

    pub fn production(cx: &mut ModelContext<Self>) -> Self {
        let fireworks_api_url = std::env::var("FIREWORKS_API_URL")
            .unwrap_or_else(|_| "https://api.fireworks.ai/inference/v1".to_string())
            .into();
        let fireworks_api_key = std::env::var("FIREWORKS_API_KEY")
            .expect("FIREWORKS_API_KEY must be set")
            .into();
        let fireworks_model = std::env::var("FIREWORKS_MODEL")
            .unwrap_or_else(|_| "accounts/fireworks/models/qwen2p5-coder-32b-instruct".to_string())
            .into();
        Self::new(
            fireworks_api_url,
            fireworks_api_key,
            fireworks_model,
            cx.http_client(),
        )
    }

    fn new(
        api_url: Arc<str>,
        api_key: Arc<str>,
        model: Arc<str>,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        Self {
            http_client,
            api_url,
            api_key,
            model,
            events: BTreeMap::new(),
            next_inline_completion_id: InlineCompletionId(0),
            next_event_id: EventId(0),
            registered_buffers: HashMap::default(),
        }
    }

    fn push_event(&mut self, event: Event) {
        // Coalesce edits for the same buffer when they happen one after the other.
        if let Event::BufferChange {
            old_snapshot,
            new_snapshot,
            timestamp,
        } = &event
        {
            if let Some(mut last_entry) = self.events.last_entry() {
                if let Event::BufferChange {
                    new_snapshot: last_new_snapshot,
                    timestamp: last_timestamp,
                    ..
                } = last_entry.get_mut()
                {
                    if timestamp.duration_since(*last_timestamp) <= BUFFER_CHANGE_GROUPING_INTERVAL
                        && old_snapshot.remote_id() == last_new_snapshot.remote_id()
                        && old_snapshot.version == last_new_snapshot.version
                    {
                        *last_new_snapshot = new_snapshot.clone();
                        *last_timestamp = *timestamp;
                        return;
                    }
                }
            }
        }

        let id = self.next_event_id;
        self.next_event_id.0 += 1;

        self.events.insert(id, event);
        if self.events.len() > 10 {
            self.events.pop_first();
        }
    }

    pub fn register_buffer(&mut self, buffer: &Model<Buffer>, cx: &mut ModelContext<Self>) {
        let buffer_id = buffer.entity_id();
        let weak_buffer = buffer.downgrade();

        if let std::collections::hash_map::Entry::Vacant(entry) =
            self.registered_buffers.entry(buffer_id)
        {
            let snapshot = buffer.read(cx).snapshot();

            entry.insert(RegisteredBuffer {
                snapshot,
                _subscriptions: [
                    cx.subscribe(buffer, move |this, buffer, event, cx| {
                        this.handle_buffer_event(buffer, event, cx);
                    }),
                    cx.observe_release(buffer, move |this, _buffer, _cx| {
                        this.registered_buffers.remove(&weak_buffer.entity_id());
                    }),
                ],
            });
        };
    }

    fn handle_buffer_event(
        &mut self,
        buffer: Model<Buffer>,
        event: &language::BufferEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            language::BufferEvent::Edited => {
                self.report_changes_for_buffer(&buffer, cx);
            }
            _ => {}
        }
    }

    pub fn request_inline_completion(
        &mut self,
        buffer: &Model<Buffer>,
        position: language::Anchor,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<InlineCompletion>>> {
        let snapshot = self.report_changes_for_buffer(buffer, cx);
        let point = position.to_point(&snapshot);
        let offset = point.to_offset(&snapshot);
        let excerpt_range = excerpt_range_for_position(point, &snapshot);

        let path = snapshot
            .file()
            .map(|f| f.path().clone())
            .unwrap_or_else(|| Arc::from(Path::new("untitled")));

        let id = self.next_inline_completion_id;
        self.next_inline_completion_id.0 += 1;

        let mut events = String::new();
        for event in self.events.values() {
            if !events.is_empty() {
                events.push('\n');
                events.push('\n');
            }
            events.push_str(&event.to_prompt());
        }

        let prompt = include_str!("./complete_prompt.md")
            .replace("<events>", &events)
            .replace(
                "<excerpt>",
                &prompt_for_excerpt(&snapshot, &excerpt_range, offset),
            );

        log::debug!("requesting completion: {}", prompt);

        let api_url = self.api_url.clone();
        let api_key = self.api_key.clone();
        let model = self.model.clone();

        let http_client = self.http_client.clone();

        cx.spawn(|_this, _cx| async move {
            let start = std::time::Instant::now();
            let mut response = open_ai::complete_text(
                http_client.as_ref(),
                &api_url,
                &api_key,
                &prompt,
                &model,
                Some(1024),
                0.,
            )
            .await?;

            log::debug!(
                "prompt_tokens: {:?}, completion_tokens: {:?}, elapsed: {:?}",
                response.usage.prompt_tokens,
                response.usage.completion_tokens,
                start.elapsed()
            );
            let choice = response.choices.pop().context("invalid response")?;

            log::debug!("completion response: {}", choice.text);

            let content = choice.text.replace(CURSOR_MARKER, "");

            let mut new_text = content.as_str();

            let codefence_start = new_text
                .find(EDITABLE_REGION_START_MARKER)
                .context("could not find start marker")?;
            new_text = &new_text[codefence_start..];

            let newline_ix = new_text.find('\n').context("could not find newline")?;
            new_text = &new_text[newline_ix + 1..];

            let codefence_end = new_text
                .rfind(&format!("\n{EDITABLE_REGION_END_MARKER}"))
                .context("could not find end marker")?;
            new_text = &new_text[..codefence_end];
            log::debug!("sanitized completion response: {}", new_text);

            let old_text = snapshot
                .text_for_range(excerpt_range.clone())
                .collect::<String>();
            let diff = similar::TextDiff::from_words(old_text.as_str(), new_text);

            let mut edits: Vec<(Range<usize>, String)> = Vec::new();
            let mut old_start = excerpt_range.start;
            for change in diff.iter_all_changes() {
                let value = change.value();
                match change.tag() {
                    similar::ChangeTag::Equal => {
                        old_start += value.len();
                    }
                    similar::ChangeTag::Delete => {
                        let old_end = old_start + value.len();
                        if let Some((last_old_range, _)) = edits.last_mut() {
                            if last_old_range.end == old_start {
                                last_old_range.end = old_end;
                            } else {
                                edits.push((old_start..old_end, String::new()));
                            }
                        } else {
                            edits.push((old_start..old_end, String::new()));
                        }

                        old_start = old_end;
                    }
                    similar::ChangeTag::Insert => {
                        if let Some((last_old_range, last_new_text)) = edits.last_mut() {
                            if last_old_range.end == old_start {
                                last_new_text.push_str(value);
                            } else {
                                edits.push((old_start..old_start, value.into()));
                            }
                        } else {
                            edits.push((old_start..old_start, value.into()));
                        }
                    }
                }
            }

            if edits.is_empty() {
                Ok(None)
            } else {
                let edits = edits
                    .into_iter()
                    .map(|(old_range, new_text)| {
                        (
                            snapshot.anchor_after(old_range.start)
                                ..snapshot.anchor_before(old_range.end),
                            new_text,
                        )
                    })
                    .collect();
                Ok(Some(InlineCompletion {
                    id,
                    path,
                    edits,
                    snapshot,
                }))
            }
        })
    }

    pub fn accept_inline_completion(
        &mut self,
        _completion: &InlineCompletion,
        cx: &mut ModelContext<Self>,
    ) {
        cx.notify();
    }

    pub fn reject_inline_completion(
        &mut self,
        completion: InlineCompletion,
        cx: &mut ModelContext<Self>,
    ) {
        // self.push_event(Event::InlineCompletionRejected(completion));
        cx.notify();
    }

    fn report_changes_for_buffer(
        &mut self,
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> BufferSnapshot {
        self.register_buffer(buffer, cx);

        let registered_buffer = self
            .registered_buffers
            .get_mut(&buffer.entity_id())
            .unwrap();
        let new_snapshot = buffer.read(cx).snapshot();

        if new_snapshot.version != registered_buffer.snapshot.version {
            let old_snapshot = mem::replace(&mut registered_buffer.snapshot, new_snapshot.clone());
            self.push_event(Event::BufferChange {
                old_snapshot,
                new_snapshot: new_snapshot.clone(),
                timestamp: Instant::now(),
            });
        }

        new_snapshot
    }
}

fn prompt_for_excerpt(
    snapshot: &BufferSnapshot,
    excerpt_range: &Range<usize>,
    offset: usize,
) -> String {
    let mut prompt_excerpt = String::new();
    writeln!(
        prompt_excerpt,
        "```{}",
        snapshot
            .file()
            .map_or(Cow::Borrowed("untitled"), |file| file
                .path()
                .to_string_lossy())
    )
    .unwrap();

    if excerpt_range.start == 0 {
        writeln!(prompt_excerpt, "{START_OF_FILE_MARKER}").unwrap();
    }

    let point_range = excerpt_range.to_point(snapshot);
    if point_range.start.row > 0 && !snapshot.is_line_blank(point_range.start.row - 1) {
        let extra_context_line_range = Point::new(point_range.start.row - 1, 0)..point_range.start;
        for chunk in snapshot.text_for_range(extra_context_line_range) {
            prompt_excerpt.push_str(chunk);
        }
    }
    writeln!(prompt_excerpt, "{EDITABLE_REGION_START_MARKER}").unwrap();
    for chunk in snapshot.text_for_range(excerpt_range.start..offset) {
        prompt_excerpt.push_str(chunk);
    }
    prompt_excerpt.push_str(CURSOR_MARKER);
    for chunk in snapshot.text_for_range(offset..excerpt_range.end) {
        prompt_excerpt.push_str(chunk);
    }
    write!(prompt_excerpt, "\n{EDITABLE_REGION_END_MARKER}").unwrap();

    if point_range.end.row < snapshot.max_point().row
        && !snapshot.is_line_blank(point_range.end.row + 1)
    {
        let extra_context_line_range = point_range.end
            ..Point::new(
                point_range.end.row + 1,
                snapshot.line_len(point_range.end.row + 1),
            );
        for chunk in snapshot.text_for_range(extra_context_line_range) {
            prompt_excerpt.push_str(chunk);
        }
    }

    write!(prompt_excerpt, "\n```").unwrap();
    prompt_excerpt
}

fn excerpt_range_for_position(point: Point, snapshot: &BufferSnapshot) -> Range<usize> {
    const CONTEXT_LINES: u32 = 8;

    let mut context_lines_before = CONTEXT_LINES;
    let mut context_lines_after = CONTEXT_LINES;
    if point.row < CONTEXT_LINES {
        context_lines_after += CONTEXT_LINES - point.row;
    } else if point.row + CONTEXT_LINES > snapshot.max_point().row {
        context_lines_before += (point.row + CONTEXT_LINES) - snapshot.max_point().row;
    }

    let mut excerpt_start_row = point.row.saturating_sub(context_lines_before);
    let mut excerpt_end_row = cmp::min(point.row + context_lines_after, snapshot.max_point().row);

    // Skip blank lines at the start.
    while excerpt_start_row < excerpt_end_row && snapshot.is_line_blank(excerpt_start_row) {
        excerpt_start_row += 1;
    }

    // Skip blank lines at the end.
    while excerpt_end_row > excerpt_start_row && snapshot.is_line_blank(excerpt_end_row) {
        excerpt_end_row -= 1;
    }

    let excerpt_start = Point::new(excerpt_start_row, 0);
    let excerpt_end = Point::new(excerpt_end_row, snapshot.line_len(excerpt_end_row));
    excerpt_start.to_offset(snapshot)..excerpt_end.to_offset(snapshot)
}

struct RegisteredBuffer {
    snapshot: BufferSnapshot,
    _subscriptions: [gpui::Subscription; 2],
}

enum Event {
    BufferChange {
        old_snapshot: BufferSnapshot,
        new_snapshot: BufferSnapshot,
        timestamp: Instant,
    },
    InlineCompletionRejected(InlineCompletion),
}

impl Event {
    fn to_prompt(&self) -> String {
        match self {
            Event::BufferChange {
                old_snapshot,
                new_snapshot,
                ..
            } => {
                let mut prompt = String::new();

                let old_path = old_snapshot
                    .file()
                    .map(|f| f.path().as_ref())
                    .unwrap_or(Path::new("untitled"));
                let new_path = new_snapshot
                    .file()
                    .map(|f| f.path().as_ref())
                    .unwrap_or(Path::new("untitled"));
                if old_path != new_path {
                    writeln!(prompt, "User renamed {:?} to {:?}\n", old_path, new_path).unwrap();
                }

                let mut edits = new_snapshot
                    .edits_since::<Point>(&old_snapshot.version)
                    .peekable();

                if edits.peek().is_some() {
                    writeln!(prompt, "User edited {:?}:", new_path).unwrap();
                }

                write!(
                    prompt,
                    "```diff\n{}\n```",
                    similar::TextDiff::from_lines(&old_snapshot.text(), &new_snapshot.text())
                        .unified_diff()
                )
                .unwrap();

                // while let Some(edit) = edits.next() {
                //     let mut old_start = edit.old.start.row;
                //     let mut old_end = edit.old.end.row;
                //     let mut new_start = edit.new.start.row;
                //     let mut new_end = edit.new.end.row;

                //     old_start = old_start.saturating_sub(2);
                //     old_end = cmp::min(old_end + 2, old_snapshot.max_point().row + 1);

                //     // Peek at further edits and merge if they overlap
                //     while let Some(next_edit) = edits.peek() {
                //         if next_edit.old.start.row <= old_end {
                //             old_end = cmp::min(
                //                 next_edit.old.end.row + 2,
                //                 old_snapshot.max_point().row + 1,
                //             );
                //             new_end = next_edit.new.end.row;
                //             edits.next();
                //         } else {
                //             break;
                //         }
                //     }

                //     new_start = new_start.saturating_sub(2);
                //     new_end = cmp::min(new_end + 2, new_snapshot.max_point().row + 1);

                //     // Report the merged edit
                //     let edit = format_edit(
                //         &old_snapshot
                //             .text_for_range(
                //                 Point::new(old_start, 0)
                //                     ..Point::new(old_end, old_snapshot.line_len(old_end)),
                //             )
                //             .collect::<String>(),
                //         &new_snapshot
                //             .text_for_range(
                //                 Point::new(new_start, 0)
                //                     ..Point::new(new_end, new_snapshot.line_len(new_end)),
                //             )
                //             .collect::<String>(),
                //     );
                //     write!(prompt, "\n{}", edit).unwrap();
                // }

                prompt
            }
            Event::InlineCompletionRejected(completion) => {
                let mut edits = String::new();
                for (old_range, new_text) in &completion.edits {
                    if !edits.is_empty() {
                        edits.push('\n');
                    }

                    edits.push_str(&format_edit(
                        &completion
                            .snapshot
                            .text_for_range(old_range.clone())
                            .collect::<String>(),
                        new_text,
                    ));
                }

                format!(
                    "User rejected these edits you suggested for file {:?}:\n{}",
                    completion.path, edits
                )
            }
        }
    }
}

fn format_edit(old_text: &str, new_text: &str) -> String {
    format!(
        "{}{}{}{}{}",
        ORIGINAL_MARKER, old_text, SEPARATOR_MARKER, new_text, UPDATED_MARKER
    )
}

pub struct ZetaInlineCompletionProvider {
    zeta: Model<Zeta>,
    current_completion: Option<InlineCompletion>,
    pending_refresh: Task<()>,
}

impl ZetaInlineCompletionProvider {
    pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);

    pub fn new(zeta: Model<Zeta>) -> Self {
        Self {
            zeta,
            current_completion: None,
            pending_refresh: Task::ready(()),
        }
    }
}

impl inline_completion::InlineCompletionProvider for ZetaInlineCompletionProvider {
    fn name() -> &'static str {
        "Zeta"
    }

    fn is_enabled(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &AppContext,
    ) -> bool {
        let buffer = buffer.read(cx);
        let file = buffer.file();
        let language = buffer.language_at(cursor_position);
        let settings = all_language_settings(file, cx);
        settings.inline_completions_enabled(language.as_ref(), file.map(|f| f.path().as_ref()), cx)
    }

    fn refresh(
        &mut self,
        buffer: Model<Buffer>,
        position: language::Anchor,
        debounce: bool,
        cx: &mut ModelContext<Self>,
    ) {
        self.pending_refresh = cx.spawn(|this, mut cx| async move {
            if debounce {
                cx.background_executor().timer(Self::DEBOUNCE_TIMEOUT).await;
            }

            let completion_request = this.update(&mut cx, |this, cx| {
                this.zeta.update(cx, |zeta, cx| {
                    zeta.request_inline_completion(&buffer, position, cx)
                })
            });

            let mut completion = None;
            if let Ok(completion_request) = completion_request {
                completion = completion_request.await.ok().flatten();
            }

            this.update(&mut cx, |this, cx| {
                this.current_completion = completion;
                cx.notify();
            })
            .ok();
        });
    }

    fn cycle(
        &mut self,
        _buffer: Model<Buffer>,
        _cursor_position: language::Anchor,
        _direction: inline_completion::Direction,
        _cx: &mut ModelContext<Self>,
    ) {
        // todo!()
    }

    fn accept(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(completion) = self.current_completion.take() {
            self.zeta.update(cx, |zeta, cx| {
                zeta.accept_inline_completion(&completion, cx)
            });
        }
    }

    fn discard(
        &mut self,
        _should_report_inline_completion_event: bool,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(completion) = self.current_completion.take() {
            self.zeta
                .update(cx, |zeta, cx| zeta.reject_inline_completion(completion, cx));
        }
    }

    fn active_completion_text<'a>(
        &'a self,
        _buffer: &Model<Buffer>,
        _cursor_position: language::Anchor,
        _cx: &'a AppContext,
    ) -> Option<inline_completion::CompletionProposal> {
        let completion = self.current_completion.as_ref()?;
        Some(inline_completion::CompletionProposal {
            edits: completion
                .edits
                .iter()
                .map(|(old_range, new_text)| inline_completion::CompletionEdit {
                    range: old_range.clone(),
                    text: new_text.as_str().into(),
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use super::*;
    use gpui::TestAppContext;
    use indoc::indoc;
    use language::{Language, LanguageConfig};
    use reqwest_client::ReqwestClient;

    #[gpui::test]
    async fn test_quicksort_1(cx: &mut TestAppContext) {
        assert_open_edit_complete(
            "quicksort.rs",
            indoc! {"
                use std::cmp::Ord;

                pub fn quicksort<T: Ord>(arr: &mut [T]) {
                    let len = arr.len();
                    if len <= 1 {
                        return;
                    }

                    let pivot_index = partition(arr);
                }
            "},
            indoc! {"
                use std::cmp::Ord;

                pub fn quicksort<T: Ord>(arr: &mut [T]) {
                    let len = arr.len();
                    if len <= 1 {
                        return;
                    }

                    let pivot_index = partition(arr);
                    <|user_cursor_is_here|>
                }
            "},
            vec!["Ensure that the quicksort function recurses to the left and to the right of the pivot"],
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_quicksort_2(cx: &mut TestAppContext) {
        assert_open_edit_complete(
            "quicksort.rs",
            indoc! {"
                use std::cmp::Ord;

                pub fn quicksort<T: Ord>(arr: &mut [T]) {
                    let len = arr.len();
                    if len <= 1 {
                        return;
                    }

                    let p
            "},
            indoc! {"
                use std::cmp::Ord;

                pub fn quicksort<T: Ord>(arr: &mut [T]) {
                    let len = arr.len();
                    if len <= 1 {
                        return;
                    }

                    let pivot = partit<|user_cursor_is_here|>
            "},
            vec!["Ensure that it calls a function called `partition` and assign its to `pivot`"],
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_import_statement_rust(cx: &mut TestAppContext) {
        assert_open_edit_complete(
            "main.rs",
            indoc! {"
                fn main() {
                }
            "},
            indoc! {"
                fn main() {<|user_cursor_is_here|>
                    thread::sleep(Duration::from_secs(1));
                }
            "},
            vec!["Ensure that there are the Rust `use` statements importing `std::thread` and `std::time::Duration`, like `use std::thread;` at the start of the file"],
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_rename(cx: &mut TestAppContext) {
        assert_open_edit_complete(
            "main.rs",
            indoc! {"
                fn main() {
                    let root_directory = \"/tmp\";
                    let glob_pattern = format!(\"{}/**/*.rs\", root_directory);
                }
            "},
            indoc! {"
                fn main() {
                    let dir<|user_cursor_is_here|> = \"/tmp\";
                    let glob_pattern = format!(\"{}/**/*.rs\", root_directory);
                }
            "},
            vec!["Ensure that the Actual test output does not contain the `root_directory` variable anymore and that it has been renamed into dir everywhere"],
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_replace(cx: &mut TestAppContext) {
        assert_open_edit_complete(
            "main.rs",
            indoc! {"
                fn main() {
                    let glob_pattern = format!(\"{}/**/*.rs\", \"/tmp\");
                }
            "},
            indoc! {"
                fn main() {
                    let dir = \"/tmp\";<|user_cursor_is_here|>
                    let glob_pattern = format!(\"{}/**/*.rs\", \"/tmp\");
                }
            "},
            vec!["Ensure that the Actual test output replaced the string `\"/tmp\"` with the variable `dir` in the call to `format!`"],
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_extract(cx: &mut TestAppContext) {
        assert_open_edit_complete(
            "main.rs",
            indoc! {"
                fn main() {
                    let glob_pattern = format!(\"{}/**/*.rs\", \"/tmp\");
                }
            "},
            indoc! {"
                fn main() {
                    let dir = \"<|user_cursor_is_here|>
                    let glob_pattern = format!(\"{}/**/*.rs\", \"/tmp\");
                }
            "},
            vec!["Ensure that the Actual test output assigns the string `\"/tmp\"` to the variable `dir``"],
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_command_line_args(cx: &mut TestAppContext) {
        assert_open_edit_complete(
            "main.rs",
            indoc! {"
                fn main() {
                    let root_directory = \"/tmp\";
                    let glob_pattern = format!(\"{}/{}\", root_directory, \"**/*.rs\");
                }
            "},
            indoc! {"
                fn main() {
                    let args = std::env::args();
                    let <|user_cursor_is_here|>
                    let root_directory = \"/tmp\";
                    let glob_pattern = format!(\"{}/{}\", root_directory, \"**/*.rs\");
                }
            "},
            vec!["Ensure that `root_directory` is using the first command line argument"],
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_element_to_vec(cx: &mut TestAppContext) {
        assert_open_edit_complete(
            "main.rs",
            indoc! {"
                fn main() {
                    let word = \"hello\";
                    for ch in word.chars() {
                        dbg!(ch);
                    }
                }
            "},
            indoc! {"
                fn main() {
                    let words = vec![<|user_cursor_is_here|>\"hello\";
                    for ch in word.chars() {
                        dbg!(ch);
                    }
                }
            "},
            vec![
                "Ensure that `words` assignment is valid",
                "Ensure a nested loop is created",
                "It's okay if the vec! contains more than one element",
            ],
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_auto_indent(cx: &mut TestAppContext) {
        assert_open_edit_complete(
            "voice/conn.go",
            indoc! {"
				func (c *connImpl) HandleVoiceServerUpdate(update botgateway.EventVoiceServerUpdate) {
					c.stateMu.Lock()
					defer c.stateMu.Unlock()
					if update.GuildID != c.state.GuildID || update.Endpoint == nil {
						return
					}

					c.state.Token = update.Token
					c.state.Endpoint = *update.Endpoint

					ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
					defer cancel()
					if err := c.gateway.Open(ctx, c.state); err != nil {
						c.config.Logger.Error(\"error opening voice gateway\", slog.Any(\"err\", err))
					}
				}
            "},
            indoc! {"
				func (c *connImpl) HandleVoiceServerUpdate(update botgateway.EventVoiceServerUpdate) {
					c.stateMu.Lock()
					defer c.stateMu.Unlock()
					if update.GuildID != c.state.GuildID || update.Endpoint == nil {
						return
					}

					c.state.Token = update.Token
					c.state.Endpoint = *update.Endpoint

					go fun<|user_cursor_is_here|>
					ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
					defer cancel()
					if err := c.gateway.Open(ctx, c.state); err != nil {
						c.config.Logger.Error(\"error opening voice gateway\", slog.Any(\"err\", err))
					}
				}
            "},
            vec![
                "Ensure the `gateway.Open()` call is done inside an anonymous goroutine",
                "Ensure that the context timeout with 5 seconds is also happening inside the anonymous goroutine",
            ],
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_pub_fields(cx: &mut TestAppContext) {
        assert_open_edit_complete(
            "main.rs",
            indoc! {"
                struct Canvas {
                    pixels: Vec<u8>,
                    stride: u8,
                    size: Size<u8>,
                    format: Format
                }
            "},
            indoc! {"
                struct Canvas {
                    pub <|user_cursor_is_here|>pixels: Vec<u8>,
                    stride: u8,
                    size: Size<u8>,
                    format: Format
                }
            "},
            vec![
                "Ensure that `pixels` is public",
                "Ensure that `stride` is public",
                "Ensure that `size` is public",
                "Ensure that `format` is public",
            ],
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_private_fields(cx: &mut TestAppContext) {
        assert_open_edit_complete(
            "main.rs",
            indoc! {"
                struct Canvas {
                    pub pixels: Vec<u8>,
                    pub stride: u8,
                    pub size: Size<u8>,
                    pub format: Format
                }
            "},
            indoc! {"
                struct Canvas {
                    <|user_cursor_is_here|>pixels: Vec<u8>,
                    pub stride: u8,
                    pub size: Size<u8>,
                    pub format: Format
                }
            "},
            vec![
                "Ensure that `pixels` does not have a visibility modifier",
                "Ensure that `stride` does not have a visibility modifier",
                "Ensure that `size` does not have a visibility modifier",
                "Ensure that `format` does not have a visibility modifier",
            ],
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_keeping_user_edit(cx: &mut TestAppContext) {
        assert_open_edit_complete(
            "main.rs",
            indoc! {"
                struct Element {
                    height: usize,
                    width: usize,
                    layout: Layout,
                    bg_color: Color,
                    fg_color: Color
                }

                impl Element {
                    fn height(&self) -> usize {
                        self.height
                    }

                    fn width(&self) -> usize {
                        self.width
                    }
                }
            "},
            indoc! {"
                struct Element {
                    height: usize,
                    wedge<|user_cursor_is_here|>: usize,
                    layout: Layout,
                    bg_color: Color,
                    fg_color: Color
                }

                impl Element {
                    fn height(&self) -> usize {
                        self.height
                    }

                    fn width(&self) -> usize {
                        self.width
                    }
                }
            "},
            vec![
                "Ensure that `Element` has a `wedge` field",
                "Ensure that `Element` has a method that returns `self.wedge`",
            ],
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_new_cli_arg(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let zeta = Zeta::test(cx);

        let buffer = open_buffer(
            "crates/cli/src/main.rs",
            include_str!("../fixtures/new-cli-arg/initial.rs"),
            &zeta,
            cx,
        );
        let edited_1 = include_str!("../fixtures/new-cli-arg/edit1.rs");
        let cursor_start = edited_1
            .find(CURSOR_MARKER)
            .expect(&format!("{CURSOR_MARKER} not found"));
        let edited_1 = edited_1.replace(CURSOR_MARKER, "");
        edit(&buffer, &edited_1, cx);
        autocomplete(&buffer, cursor_start, &zeta, cx).await;

        let autocompleted = buffer.read_with(cx, |buffer, _| buffer.text());
        assert_autocompleted(
            autocompleted,
            &[
                "Ensure a new boolean field has been added to the `Args` struct to control whether to do an update or not",
                "Ensure the field also has an `#[arg]` attribute macro",
                "Ideally, it has the `#[arg(long)]` attribute macro",
                "Ideally, the field name is `update` (but if it's not called that, that's fine too)",
            ],
            &zeta,
            cx,
        )
        .await;

        let edited_2 = include_str!("../fixtures/new-cli-arg/edit2.rs");
        let cursor_start = edited_2
            .find(CURSOR_MARKER)
            .expect(&format!("{CURSOR_MARKER} not found"));
        let edited_2 = edited_2.replace(CURSOR_MARKER, "");
        edit(&buffer, &edited_2, cx);
        autocomplete(&buffer, cursor_start, &zeta, cx).await;

        let autocompleted = buffer.read_with(cx, |buffer, _| buffer.text());
        assert_autocompleted(
            autocompleted,
            &[
                "Ensure that the `main` function contains an if-expression checking if an update-flag in args is set",
                "It's okay if the body of that if-expression does not contain logic yet. It's fine if it only contains placeholder comments."
            ],
            &zeta,
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_interpreter(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let zeta = Zeta::test(cx);

        // token.go
        let buffer_1 = open_buffer(
            "token/token.go",
            include_str!("../fixtures/interpreter/token_initial.go"),
            &zeta,
            cx,
        );
        let buffer_1_edit_1 = include_str!("../fixtures/interpreter/token_edit1.go");
        let cursor_start = buffer_1_edit_1
            .find(CURSOR_MARKER)
            .expect(&format!("{CURSOR_MARKER} not found"));
        let buffer_1_edited_1 = buffer_1_edit_1.replace(CURSOR_MARKER, "");
        edit(&buffer_1, &buffer_1_edited_1, cx);
        autocomplete(&buffer_1, cursor_start, &zeta, cx).await;

        let autocompleted = buffer_1.read_with(cx, |buffer, _| buffer.text());
        assert_autocompleted(
            autocompleted.clone(),
            &["Ensure the new WHILE constant has a value of \"WHILE\""],
            &zeta,
            cx,
        )
        .await;

        // Accept the completion:
        edit(&buffer_1, &autocompleted, cx);

        // ast.go
        let buffer_2 = open_buffer(
            "ast/ast.go",
            include_str!("../fixtures/interpreter/ast_initial.go"),
            &zeta,
            cx,
        );
        let buffer_2_edit_1 = include_str!("../fixtures/interpreter/ast_edit1.go");
        let cursor_start = buffer_2_edit_1
            .find(CURSOR_MARKER)
            .expect(&format!("{CURSOR_MARKER} not found"));
        let buffer_2_edited_2 = buffer_2_edit_1.replace(CURSOR_MARKER, "");
        edit(&buffer_2, &buffer_2_edited_2, cx);
        autocomplete(&buffer_2, cursor_start, &zeta, cx).await;

        let autocompleted = buffer_2.read_with(cx, |buffer, _| buffer.text());
        assert_autocompleted(
            autocompleted.clone(),
            &["Ensure the new WhileStatement struct has fields for the token, condition, and body",
          "Ensure the condition field is an Expression",
          "Ensure the body is a BlockStatement",
          "Ensure the code is syntactically valid",
          "The code has defined methods on the WhileStatment struct to make it a valid AST node",
            ],
            &zeta,
            cx,
        )
        .await;

        // Accept the completion:
        edit(&buffer_2, &autocompleted, cx);

        // parser_test.go
        let buffer_3 = open_buffer(
            "parser/parser_test.go",
            include_str!("../fixtures/interpreter/parser_test_initial.go"),
            &zeta,
            cx,
        );
        let buffer_3_edit_1 = include_str!("../fixtures/interpreter/parser_test_edit1.go");
        let cursor_start = buffer_3_edit_1
            .find(CURSOR_MARKER)
            .expect(&format!("{CURSOR_MARKER} not found"));
        let buffer_3_edited_2 = buffer_3_edit_1.replace(CURSOR_MARKER, "");
        edit(&buffer_3, &buffer_3_edited_2, cx);
        autocomplete(&buffer_3, cursor_start, &zeta, cx).await;

        let autocompleted = buffer_3.read_with(cx, |buffer, _| buffer.text());
        assert_autocompleted(
            autocompleted.clone(),
            &[
                "Ensure the TestWhileStatements function has at least one valid test case in its body",
                "Ensure the code is syntactically valid",
            ],
            &zeta,
            cx,
        )
        .await;

        edit(&buffer_3, &autocompleted, cx);
    }

    async fn assert_open_edit_complete_full(
        filename: &str,
        initial: &str,
        edited: &str,
        assertions: &[&str],
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        let zeta = Zeta::test(cx);

        let buffer = open_buffer(filename, initial, &zeta, cx);
        let cursor_start = edited
            .find(CURSOR_MARKER)
            .expect(&format!("{CURSOR_MARKER} not found"));
        let edited = edited.replace(CURSOR_MARKER, "");
        edit(&buffer, &edited, cx);
        autocomplete(&buffer, cursor_start, &zeta, cx).await;
        let autocompleted = buffer.read_with(cx, |buffer, _| buffer.text());
        assert_autocompleted(autocompleted, assertions, &zeta, cx).await;
    }

    async fn assert_open_edit_complete_incremental(
        filename: &str,
        initial: &str,
        edited: &str,
        assertions: &[&str],
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        let zeta = Zeta::test(cx);

        let buffer = open_buffer(filename, initial, &zeta, cx);
        let cursor_start = edited
            .find(CURSOR_MARKER)
            .expect(&format!("{CURSOR_MARKER} not found"));
        let edited = edited.replace(CURSOR_MARKER, "");
        character_wise_edit(&buffer, &edited, cx);
        autocomplete(&buffer, cursor_start, &zeta, cx).await;
        let autocompleted = buffer.read_with(cx, |buffer, _| buffer.text());
        assert_autocompleted(autocompleted, assertions, &zeta, cx).await;
    }

    async fn assert_open_edit_complete(
        filename: &str,
        initial: &str,
        edited: &str,
        assertions: Vec<&str>,
        cx: &mut TestAppContext,
    ) {
        assert_open_edit_complete_full(filename, initial, edited, &assertions, cx).await;
        assert_open_edit_complete_incremental(filename, initial, edited, &assertions, cx).await;
    }

    async fn assert_autocompleted(
        autocompleted: String,
        assertions: &[&str],
        zeta: &Model<Zeta>,
        cx: &mut TestAppContext,
    ) {
        let mut assertion_text = String::new();
        for assertion in assertions {
            assertion_text.push_str("- ");
            assertion_text.push_str(assertion);
            assertion_text.push('\n');
        }

        let prompt = include_str!("./eval_prompt.md")
            .replace("<actual>", &autocompleted)
            .replace("<assertions>", &assertion_text);

        log::debug!("grading prompt: {}", prompt);

        let api_url = "https://api.fireworks.ai/inference/v1";
        let api_key = std::env::var("GRADING_API_KEY").expect("missing GRADING_API_KEY");
        let model = std::env::var("GRADING_MODEL")
            .unwrap_or_else(|_| "accounts/fireworks/models/qwen2p5-coder-32b-instruct".to_string());
        let request = open_ai::Request {
            model,
            messages: vec![open_ai::RequestMessage::User { content: prompt }],
            stream: false,
            max_tokens: None,
            stop: Vec::new(),
            temperature: 0.0,
            tool_choice: None,
            tools: Vec::new(),
            prediction: None,
        };

        let http_client = zeta.read_with(cx, |zeta, _cx| zeta.http_client.clone());
        let response = open_ai::complete(http_client.as_ref(), &api_url, &api_key, request)
            .await
            .unwrap();
        let choice = response.choices.first().unwrap();
        let open_ai::RequestMessage::Assistant {
            content: Some(content),
            ..
        } = &choice.message
        else {
            panic!("unexpected response: {:?}", choice.message);
        };

        log::info!("received score from LLM: {}", content);

        let score = content
            .lines()
            .last()
            .unwrap()
            .parse::<usize>()
            .with_context(|| format!("failed to parse response into a usize: {:?}", content))
            .unwrap();
        assert!(
            score >= 4,
            "score was {}\n----- actual: ------\n{}",
            score,
            autocompleted,
        );
    }

    impl Zeta {
        fn test(cx: &mut TestAppContext) -> Model<Zeta> {
            cx.new_model(|_| {
                let (api_url, api_key, model) = match std::env::var("FIREWORKS_API_KEY") {
                    Ok(api_key) => (
                        Arc::from("https://api.fireworks.ai/inference/v1"),
                        Arc::from(api_key),
                        Arc::from(std::env::var("FIREWORKS_MODEL").unwrap_or_else(|_| {
                            "accounts/fireworks/models/qwen2p5-coder-32b-instruct".to_string()
                        })),
                    ),
                    Err(_) => (
                        Arc::from("http://localhost:11434/v1"),
                        Arc::from(""),
                        Arc::from("qwen2.5-coder:32b"),
                    ),
                };
                log::debug!("using {api_url} with {model}");
                Zeta::new(api_url, api_key, model, Arc::new(ReqwestClient::new()))
            })
        }
    }

    fn edit(buffer: &Model<Buffer>, text: &str, cx: &mut TestAppContext) {
        let diff = cx
            .executor()
            .block(buffer.update(cx, |buffer, cx| buffer.diff(text.to_string(), cx)));
        buffer.update(cx, |buffer, cx| buffer.apply_diff(diff, cx));
    }

    fn character_wise_edit(buffer: &Model<Buffer>, text: &str, cx: &mut TestAppContext) {
        let diff = cx
            .executor()
            .block(buffer.update(cx, |buffer, cx| buffer.diff(text.to_string(), cx)));

        let mut delta = 0isize;
        for (old_range, new_text) in &diff.edits {
            let new_range = (old_range.start as isize + delta) as usize
                ..(old_range.end as isize + delta) as usize;

            if !new_range.is_empty() {
                buffer.update(cx, |buffer, cx| {
                    buffer.edit([(new_range.clone(), "")], None, cx)
                });
            }

            for (char_ix, ch) in new_text.char_indices() {
                buffer.update(cx, |buffer, cx| {
                    let insertion_ix = new_range.start + char_ix;
                    buffer.edit([(insertion_ix..insertion_ix, ch.to_string())], None, cx)
                });
            }

            delta += new_text.len() as isize - new_range.len() as isize;
        }
    }

    async fn autocomplete(
        buffer: &Model<Buffer>,
        position: usize,
        zeta: &Model<Zeta>,
        cx: &mut TestAppContext,
    ) {
        let position = buffer.read_with(cx, |buffer, _| buffer.anchor_after(position));
        let completion = zeta
            .update(cx, |zeta, cx| {
                zeta.request_inline_completion(buffer, position, cx)
            })
            .await
            .unwrap();
        if let Some(completion) = completion {
            buffer.update(cx, |buffer, cx| {
                buffer.edit(completion.edits.clone(), None, cx);
            });
        }
    }

    fn open_buffer(
        path: impl AsRef<Path>,
        text: &str,
        zeta: &Model<Zeta>,
        cx: &mut TestAppContext,
    ) -> Model<Buffer> {
        let language = if path.as_ref().extension() == Some(OsStr::new("rs")) {
            Some(Arc::new(rust_lang()))
        } else if path.as_ref().extension() == Some(OsStr::new("go")) {
            Some(Arc::new(go_lang()))
        } else {
            None
        };

        let buffer = cx.new_model(|cx| Buffer::local(text, cx));
        buffer.update(cx, |buffer, cx| {
            buffer.set_language(language, cx);
            buffer.file_updated(Arc::new(TestFile(path.as_ref().into())), cx)
        });
        zeta.update(cx, |zeta, cx| zeta.register_buffer(&buffer, cx));
        buffer
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_outline_query(include_str!("../../languages/src/rust/outline.scm"))
        .unwrap()
    }

    fn go_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Go".into(),
                ..Default::default()
            },
            Some(tree_sitter_go::LANGUAGE.into()),
        )
        .with_outline_query(include_str!("../../languages/src/go/outline.scm"))
        .unwrap()
    }

    struct TestFile(Arc<Path>);

    impl language::File for TestFile {
        fn as_local(&self) -> Option<&dyn language::LocalFile> {
            None
        }

        fn disk_state(&self) -> language::DiskState {
            language::DiskState::New
        }

        fn path(&self) -> &Arc<Path> {
            &self.0
        }

        fn full_path(&self, _cx: &AppContext) -> std::path::PathBuf {
            self.0.to_path_buf()
        }

        fn file_name<'a>(&'a self, _cx: &'a AppContext) -> &'a std::ffi::OsStr {
            self.0.file_name().unwrap()
        }

        fn worktree_id(&self, _cx: &AppContext) -> worktree::WorktreeId {
            unimplemented!()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn to_proto(&self, _cx: &AppContext) -> rpc::proto::File {
            unimplemented!()
        }

        fn is_private(&self) -> bool {
            unimplemented!()
        }
    }

    #[ctor::ctor]
    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }
}
