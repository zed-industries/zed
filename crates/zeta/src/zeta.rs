mod fuzzy;

use anyhow::{anyhow, Context as _, Result};
use collections::{BTreeMap, HashMap};
use gpui::{AppContext, Context, Global, Model, ModelContext, Task};
use http_client::HttpClient;
use language::{Anchor, Buffer, BufferSnapshot, Point, ToOffset, ToPoint};
use std::{borrow::Cow, cmp, fmt::Write, mem, ops::Range, path::Path, sync::Arc};

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
    event_ids_by_inline_completion_id: HashMap<InlineCompletionId, EventId>,
    next_inline_completion_id: InlineCompletionId,
    next_event_id: EventId,
    registered_buffers: HashMap<gpui::EntityId, RegisteredBuffer>,
}

#[derive(Debug)]
pub struct InlineCompletion {
    id: InlineCompletionId,
    range: Range<Anchor>,
    new_text: Arc<str>,
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
            event_ids_by_inline_completion_id: HashMap::default(),
            next_inline_completion_id: InlineCompletionId(0),
            next_event_id: EventId(0),
            registered_buffers: HashMap::default(),
        }
    }

    fn push_event(&mut self, event: Event) {
        // Filter out an edit event that occurs when an inline completion is accepted
        if let Event::Edit {
            old_text,
            new_text,
            path,
        } = &event
        {
            if let Some(last_entry) = self.events.last_entry() {
                if let Event::InlineCompletion {
                    old_text: old_text_completion,
                    new_text: new_text_completion,
                    path: path_completion,
                    accepted: Some(true),
                    ..
                } = last_entry.get()
                {
                    if path == path_completion
                        && old_text_completion == old_text
                        && new_text_completion == new_text
                    {
                        return;
                    }
                }
            }
        }

        let id = self.next_event_id;
        self.next_event_id.0 += 1;

        if let Event::InlineCompletion {
            id: inline_completion_id,
            ..
        } = &event
        {
            self.event_ids_by_inline_completion_id
                .insert(*inline_completion_id, id);
        }
        self.events.insert(id, event);

        if self.events.len() > 100 {
            if let Some((
                _,
                Event::InlineCompletion {
                    id: inline_completion_id,
                    ..
                },
            )) = self.events.pop_first()
            {
                self.event_ids_by_inline_completion_id
                    .remove(&inline_completion_id);
            }
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
                        if let Some(path) = this
                            .registered_buffers
                            .get(&weak_buffer.entity_id())
                            .and_then(|rb| rb.snapshot.file())
                            .map(|f| f.path().to_owned())
                        {
                            this.push_event(Event::Close {
                                path: Arc::from(path),
                            });
                        }
                        this.registered_buffers.remove(&weak_buffer.entity_id());
                    }),
                ],
            });

            let path = buffer.read(cx).snapshot().file().map(|f| f.path().clone());
            self.push_event(Event::Open {
                path: path.unwrap_or_else(|| Arc::from(Path::new("untitled"))),
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
            language::BufferEvent::Edited if buffer.read(cx).file().is_some() => {
                self.report_changes_for_buffer(&buffer, cx);
            }
            language::BufferEvent::Saved => {
                if let Some(file) = buffer.read(cx).file() {
                    self.push_event(Event::Save {
                        path: file.path().clone(),
                    });
                }
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
        let path = snapshot
            .file()
            .map(|f| f.path().clone())
            .unwrap_or_else(|| Arc::from(Path::new("untitled")));

        let id = self.next_inline_completion_id;
        self.next_inline_completion_id.0 += 1;

        let mut events = String::new();
        // let mut first_edit_event = None;
        // let mut last_edit_event = None;
        for event in self.events.values() {
            // match event {
            //     Event::Edit { .. } => {
            //         if first_edit_event.is_none() {
            //             first_edit_event = Some(event);
            //         }
            //         last_edit_event = Some(event);
            //         continue;
            //     }
            //     _ => {
            //         if let Some((
            //             Event::Edit {
            //                 old_text: first_old_text,
            //                 ..
            //             },
            //             Event::Edit {
            //                 path,
            //                 new_text: last_new_text,
            //                 ..
            //             },
            //         )) = first_edit_event.take().zip(last_edit_event.take())
            //         {
            //             let coalesced_edit = Event::Edit {
            //                 old_text: first_old_text.clone(),
            //                 new_text: last_new_text.clone(),
            //                 path: path.clone(),
            //             };
            //             events.push_str(&coalesced_edit.to_prompt());
            //             events.push('\n');
            //             events.push('\n');
            //         }
            //     }
            // }
            events.push_str(&event.to_prompt());
            events.push('\n');
            events.push('\n');
        }

        // if let Some((
        //     Event::Edit {
        //         old_text: first_old_text,
        //         ..
        //     },
        //     Event::Edit {
        //         path,
        //         new_text: last_new_text,
        //         ..
        //     },
        // )) = first_edit_event.take().zip(last_edit_event.take())
        // {
        //     let coalesced_edit = Event::Edit {
        //         old_text: first_old_text.clone(),
        //         new_text: last_new_text.clone(),
        //         path: path.clone(),
        //     };
        //     events.push_str(&coalesced_edit.to_prompt());
        //     events.push('\n');
        //     events.push('\n');
        // }

        let excerpt = inline_completion_excerpt(&snapshot, &position);
        let prompt = include_str!("./complete_prompt.md")
            .replace("<events>", &events)
            .replace("<excerpt>", &excerpt);
        log::debug!("requesting completion: {}", prompt);

        let api_url = self.api_url.clone();
        let api_key = self.api_key.clone();
        let request = open_ai::Request {
            model: self.model.to_string(),
            messages: vec![open_ai::RequestMessage::User { content: prompt }],
            stream: false,
            max_tokens: None,
            stop: Vec::new(),
            temperature: 0.0,
            tool_choice: None,
            tools: Vec::new(),
        };
        let http_client = self.http_client.clone();

        cx.spawn(|this, mut cx| async move {
            let mut response =
                open_ai::complete(http_client.as_ref(), &api_url, &api_key, request).await?;
            let choice = response.choices.pop().context("invalid response")?;
            let mut content = match choice.message {
                open_ai::RequestMessage::Assistant { content, .. } => {
                    content.context("empty response from the assistant")?
                }
                open_ai::RequestMessage::User { content } => content,
                open_ai::RequestMessage::System { content } => content,
                open_ai::RequestMessage::Tool { .. } => return Err(anyhow!("unexpected tool use")),
            };
            log::debug!("completion response: {}", content);

            content = content.replace(CURSOR_MARKER, "");
            log::debug!("sanitized completion response: {}", content);

            if let (Some(orig_start), Some(sep), Some(upd_end)) = (
                content.find(ORIGINAL_MARKER),
                content.find(SEPARATOR_MARKER),
                content.find(UPDATED_MARKER),
            ) {
                let old_start = orig_start + ORIGINAL_MARKER.len();
                let new_start = sep + SEPARATOR_MARKER.len();

                let old_text: Arc<str> = content[old_start..sep + 1].into();
                let new_text: Arc<str> = content[new_start..upd_end + 1].into();
                let range = fuzzy::search(&snapshot, &old_text);

                this.update(&mut cx, |this, _cx| {
                    this.push_event(Event::InlineCompletion {
                        id,
                        path,
                        old_text,
                        new_text: new_text.clone(),
                        accepted: None,
                    })
                })?;

                Ok(Some(InlineCompletion {
                    id,
                    range,
                    new_text,
                }))
            } else {
                this.update(&mut cx, |this, _cx| {
                    this.push_event(Event::NoInlineCompletion { id })
                })?;

                Ok(None)
            }
        })
    }

    pub fn accept_inline_completion(
        &mut self,
        completion: &InlineCompletion,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(&event_id) = self.event_ids_by_inline_completion_id.get(&completion.id) {
            if let Some(Event::InlineCompletion { accepted, .. }) = self.events.get_mut(&event_id) {
                *accepted = Some(true);
                cx.notify();
            }
        }
    }

    pub fn reject_inline_completion(
        &mut self,
        completion: InlineCompletion,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(&event_id) = self.event_ids_by_inline_completion_id.get(&completion.id) {
            if let Some(Event::InlineCompletion { accepted, .. }) = self.events.get_mut(&event_id) {
                *accepted = Some(false);
                cx.notify();
            }
        }
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

        if new_snapshot.version == registered_buffer.snapshot.version {
            new_snapshot
        } else {
            let old_snapshot = mem::replace(&mut registered_buffer.snapshot, new_snapshot.clone());

            let old_path = old_snapshot.file().map(|f| f.path().clone());
            let new_path = new_snapshot.file().map(|f| f.path().clone());
            if old_path != new_path {
                self.push_event(Event::Rename { old_path, new_path });
            }

            let mut edits = new_snapshot
                .edits_since::<Point>(&old_snapshot.version)
                .peekable();
            while let Some(edit) = edits.next() {
                let mut old_start = edit.old.start.row;
                let mut old_end = edit.old.end.row;
                let mut new_start = edit.new.start.row;
                let mut new_end = edit.new.end.row;

                old_start = old_start.saturating_sub(2);
                old_end = cmp::max(old_end + 2, old_snapshot.max_point().row + 1);

                // Peek at further edits and merge if they overlap
                while let Some(next_edit) = edits.peek() {
                    if next_edit.old.start.row <= old_end {
                        old_end =
                            cmp::max(next_edit.old.end.row + 2, old_snapshot.max_point().row + 1);
                        new_end = next_edit.new.end.row;
                        edits.next();
                    } else {
                        break;
                    }
                }

                new_start = new_start.saturating_sub(2);
                new_end = cmp::max(new_end + 2, new_snapshot.max_point().row + 1);

                // Report the merged edit
                self.push_event(Event::Edit {
                    path: new_snapshot.file().map_or_else(
                        || Arc::from(Path::new("untitled")),
                        |file| file.path().clone(),
                    ),
                    old_text: old_snapshot
                        .text_for_range(
                            Point::new(old_start, 0)
                                ..Point::new(old_end, old_snapshot.line_len(old_end)),
                        )
                        .collect::<String>()
                        .into(),
                    new_text: new_snapshot
                        .text_for_range(
                            Point::new(new_start, 0)
                                ..Point::new(new_end, new_snapshot.line_len(new_end)),
                        )
                        .collect::<String>()
                        .into(),
                });
            }
            drop(edits);
            new_snapshot
        }
    }
}

const CURSOR_MARKER: &'static str = "<|user_cursor_is_here|>";
const ORIGINAL_MARKER: &str = "<<<<<<< ORIGINAL\n";
const SEPARATOR_MARKER: &str = "\n=======\n";
const UPDATED_MARKER: &str = "\n>>>>>>> UPDATED";

struct RegisteredBuffer {
    snapshot: BufferSnapshot,
    _subscriptions: [gpui::Subscription; 2],
}

enum Event {
    Open {
        path: Arc<Path>,
    },
    Save {
        path: Arc<Path>,
    },
    Rename {
        old_path: Option<Arc<Path>>,
        new_path: Option<Arc<Path>>,
    },
    Close {
        path: Arc<Path>,
    },
    Edit {
        path: Arc<Path>,
        old_text: Arc<str>,
        new_text: Arc<str>,
    },
    InlineCompletion {
        id: InlineCompletionId,
        path: Arc<Path>,
        old_text: Arc<str>,
        new_text: Arc<str>,
        accepted: Option<bool>,
    },
    NoInlineCompletion {
        id: InlineCompletionId,
    },
}

impl Event {
    fn to_prompt(&self) -> String {
        match self {
            Event::Open { path } => format!("User opened file: {:?}", path),
            Event::Save { path } => format!("User saved file: {:?}", path),
            Event::Rename { old_path, new_path } => format!(
                "User renamed file: {:?} -> {:?}",
                old_path.as_deref().unwrap_or(Path::new("untitled")),
                new_path.as_deref().unwrap_or(Path::new("untitled"))
            ),
            Event::Close { path } => format!("User closed file: {:?}", path),
            Event::Edit {
                path,
                old_text,
                new_text,
            } => {
                format!(
                    "User edited file: {:?}\n\n{}",
                    path,
                    format_edit(old_text, new_text)
                )
            }
            Event::InlineCompletion {
                old_text,
                new_text,
                accepted,
                ..
            } => {
                let acceptance = accepted
                    .map(|a| {
                        let action = if a { "accepted" } else { "rejected" };
                        format!(". User {}", action)
                    })
                    .unwrap_or_default();

                format!(
                    "Completion response{}.\n{}",
                    acceptance,
                    format_edit(old_text, new_text)
                )
            }
            Event::NoInlineCompletion { .. } => "<|DONE|>".into(),
        }
    }
}

fn inline_completion_excerpt(snapshot: &BufferSnapshot, position: &Anchor) -> String {
    let position = position.to_point(snapshot);

    let start = Point::new(position.row.saturating_sub(50), 0);
    let end = cmp::min(Point::new(position.row + 50, 0), snapshot.max_point());

    let mut content = String::new();
    writeln!(
        content,
        "```{}",
        snapshot
            .file()
            .map_or(Cow::Borrowed("untitled"), |file| file
                .path()
                .to_string_lossy())
    )
    .unwrap();

    for chunk in snapshot.text_for_range(start..position) {
        content.push_str(chunk);
    }
    content.push_str(CURSOR_MARKER);
    for chunk in snapshot.text_for_range(position..end) {
        content.push_str(chunk);
    }
    content.push_str("\n```");
    content
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
    pub fn new(zeta: Model<Zeta>) -> Self {
        Self {
            zeta,
            current_completion: None,
            pending_refresh: Task::ready(()),
        }
    }
}

impl editor::InlineCompletionProvider for ZetaInlineCompletionProvider {
    fn name() -> &'static str {
        "Zeta"
    }

    fn is_enabled(
        &self,
        _buffer: &Model<Buffer>,
        _cursor_position: language::Anchor,
        _cx: &AppContext,
    ) -> bool {
        true
    }

    fn refresh(
        &mut self,
        buffer: Model<Buffer>,
        position: language::Anchor,
        _debounce: bool,
        cx: &mut ModelContext<Self>,
    ) {
        let completion = self.zeta.update(cx, |zeta, cx| {
            zeta.request_inline_completion(&buffer, position, cx)
        });
        self.pending_refresh = cx.spawn(|this, mut cx| async move {
            let completion = completion.await.ok().and_then(|r| r);
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
        _direction: editor::Direction,
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
        buffer: &Model<Buffer>,
        _cursor_position: language::Anchor,
        cx: &'a AppContext,
    ) -> Option<editor::CompletionProposal> {
        let completion = self.current_completion.as_ref()?;

        let snapshot = buffer.read(cx).snapshot();
        let old_text = snapshot
            .text_for_range(completion.range.clone())
            .collect::<String>();

        let diff = similar::TextDiff::from_words(old_text.as_str(), completion.new_text.as_ref());
        let remapper = similar::utils::TextDiffRemapper::from_text_diff(
            &diff,
            old_text.as_str(),
            completion.new_text.as_ref(),
        );
        let changes = diff.ops().iter().flat_map(move |x| remapper.iter_slices(x));

        let mut inlays = Vec::new();
        let mut ix = completion.range.start.to_offset(&snapshot);

        for (tag, value) in changes {
            match tag {
                similar::ChangeTag::Equal => {
                    ix += value.len();
                }
                similar::ChangeTag::Delete => {
                    ix += value.len();
                }
                similar::ChangeTag::Insert => {
                    inlays.push(editor::InlayProposal::Suggestion(
                        snapshot.anchor_after(ix),
                        language::Rope::from(value),
                    ));
                }
            }
        }

        println!("text={:?}", &completion.new_text);
        Some(editor::CompletionProposal {
            inlays,
            text: language::Rope::from(completion.new_text.as_ref()),
            delete_range: Some(completion.range.clone()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use indoc::indoc;
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
            indoc! {"
                use std::cmp::Ord;

                pub fn quicksort<T: Ord>(arr: &mut [T]) {
                    let len = arr.len();
                    if len <= 1 {
                        return;
                    }

                    let pivot_index = partition(arr);
                    quicksort(&mut arr[0..pivot_index]);
                    quicksort(&mut arr[pivot_index + 1..]);
                }
            "},
            vec!["Ensure that the quicksort function is implememented correctly"],
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
            indoc! {"
                use std::cmp::Ord;

                pub fn quicksort<T: Ord>(arr: &mut [T]) {
                    let len = arr.len();
                    if len <= 1 {
                        return;
                    }

                    let pivot = partition(arr);
            "},
            vec!["Ensure that the 'pivot' assignment statement is valid"],
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
                fn main() {
                    thread::sleep(Duration::from_secs(1));<|user_cursor_is_here|>
                }
            "},
            indoc! {"
                use std::thread;
                use std::time::Duration;

                fn main() {
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
            indoc! {"
                fn main() {
                    let dir = \"/tmp\";
                    let glob_pattern = format!(\"{}/**/*.rs\", dir);
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
            indoc! {"
                fn main() {
                    let dir = \"/tmp\";
                    let glob_pattern = format!(\"{}/**/*.rs\", dir);
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
            indoc! {"
                fn main() {
                    let dir = \"/tmp\";
                    let glob_pattern = format!(\"{}/**/*.rs\", dir);
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
            indoc! {"
                fn main() {
                    let args = std::env::args();
                    let root_directory = args.nth(1).unwrap_or(\"/tmp\");
                    let glob_pattern = format!(\"{}/{}\", root_directory, \"**/*.rs\");
                }
            "},
            vec!["Ensure that `root_directory` is using the first command line argument"],
            cx,
        )
        .await;
    }

    async fn assert_open_edit_complete_full(
        filename: &str,
        initial: &str,
        edited: &str,
        expected: &str,
        assertions: &[&str],
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        let zeta = zeta(cx);

        let buffer = open_buffer(filename, initial, &zeta, cx);
        let cursor_start = edited
            .find(CURSOR_MARKER)
            .expect(&format!("{CURSOR_MARKER} not found"));
        let edited = edited.replace(CURSOR_MARKER, "");
        edit(&buffer, &edited, cx);
        autocomplete(&buffer, cursor_start, &zeta, cx).await;
        let autocompleted = buffer.read_with(cx, |buffer, _| buffer.text());
        assert_autocompleted(autocompleted, expected, assertions, zeta, cx).await;
    }

    async fn assert_open_edit_complete_incremental(
        filename: &str,
        initial: &str,
        edited: &str,
        expected: &str,
        assertions: &[&str],
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        let zeta = zeta(cx);

        let buffer = open_buffer(filename, initial, &zeta, cx);
        let cursor_start = edited
            .find(CURSOR_MARKER)
            .expect(&format!("{CURSOR_MARKER} not found"));
        let edited = edited.replace(CURSOR_MARKER, "");
        character_wise_edit(&buffer, &edited, cx);
        autocomplete(&buffer, cursor_start, &zeta, cx).await;
        let autocompleted = buffer.read_with(cx, |buffer, _| buffer.text());
        assert_autocompleted(autocompleted, expected, assertions, zeta, cx).await;
    }

    async fn assert_open_edit_complete(
        filename: &str,
        initial: &str,
        edited: &str,
        expected: &str,
        mut assertions: Vec<&str>,
        cx: &mut TestAppContext,
    ) {
        assertions.insert(0, "Must be similar to the expected output");
        assert_open_edit_complete_full(filename, initial, edited, expected, &assertions, cx).await;
        // assert_open_edit_complete_incremental(filename, initial, edited, expected, &assertions, cx)
        //     .await;
    }

    async fn assert_autocompleted(
        autocompleted: String,
        expected: &str,
        assertions: &[&str],
        zeta: Model<Zeta>,
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
            .replace("<expected>", expected)
            .replace("<assertions>", &assertion_text);

        log::debug!("grading prompt: {}", prompt);
        let (api_url, api_key, http_client, request) = zeta.read_with(cx, |zeta, _cx| {
            (
                zeta.api_url.clone(),
                zeta.api_key.clone(),
                zeta.http_client.clone(),
                open_ai::Request {
                    model: zeta.model.to_string(),
                    messages: vec![open_ai::RequestMessage::User { content: prompt }],
                    stream: false,
                    max_tokens: None,
                    stop: Vec::new(),
                    temperature: 0.0,
                    tool_choice: None,
                    tools: Vec::new(),
                },
            )
        });
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
            .parse::<f64>()
            .with_context(|| format!("failed to parse response into a f64: {:?}", content))
            .unwrap();
        assert!(
            score >= 0.8,
            "score was {}\n----- actual: ------\n{}\n----- expected: ------\n{}",
            score,
            autocompleted,
            expected
        );
    }

    fn zeta(cx: &mut TestAppContext) -> Model<Zeta> {
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
                    Arc::from("http://localhost:11434"),
                    Arc::from(""),
                    Arc::from("qwen2.5-coder:32b"),
                ),
            };
            Zeta::new(api_url, api_key, model, Arc::new(ReqwestClient::new()))
        })
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
                buffer.edit([(completion.range, completion.new_text)], None, cx);
            });
        }
    }

    fn open_buffer(
        path: impl AsRef<Path>,
        text: &str,
        zeta: &Model<Zeta>,
        cx: &mut TestAppContext,
    ) -> Model<Buffer> {
        let buffer = cx.new_model(|cx| Buffer::local(text, cx));
        buffer.update(cx, |buffer, cx| {
            buffer.file_updated(Arc::new(TestFile(path.as_ref().into())), cx)
        });
        zeta.update(cx, |zeta, cx| zeta.register_buffer(&buffer, cx));
        buffer
    }

    struct TestFile(Arc<Path>);

    impl language::File for TestFile {
        fn as_local(&self) -> Option<&dyn language::LocalFile> {
            None
        }

        fn mtime(&self) -> Option<std::time::SystemTime> {
            None
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

        fn is_deleted(&self) -> bool {
            false
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
