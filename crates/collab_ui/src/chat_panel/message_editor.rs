use anyhow::{Context as _, Result};
use channel::{ChannelChat, ChannelStore, MessageParams};
use client::{UserId, UserStore};
use collections::HashSet;
use editor::{AnchorRangeExt, CompletionProvider, Editor, EditorElement, EditorStyle, ExcerptId};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AsyncApp, AsyncWindowContext, Context, Entity, Focusable, FontStyle, FontWeight,
    HighlightStyle, IntoElement, Render, Task, TextStyle, WeakEntity, Window,
};
use language::{
    Anchor, Buffer, BufferSnapshot, CodeLabel, LanguageRegistry, ToOffset,
    language_settings::SoftWrap,
};
use project::{Completion, CompletionSource, search::SearchQuery};
use settings::Settings;
use std::{
    cell::RefCell,
    ops::Range,
    rc::Rc,
    sync::{Arc, LazyLock},
    time::Duration,
};
use theme::ThemeSettings;
use ui::{TextSize, prelude::*};

use crate::panel_settings::MessageEditorSettings;

const MENTIONS_DEBOUNCE_INTERVAL: Duration = Duration::from_millis(50);

static MENTIONS_SEARCH: LazyLock<SearchQuery> = LazyLock::new(|| {
    SearchQuery::regex(
        "@[-_\\w]+",
        false,
        false,
        false,
        false,
        Default::default(),
        Default::default(),
        false,
        None,
    )
    .unwrap()
});

pub struct MessageEditor {
    pub editor: Entity<Editor>,
    user_store: Entity<UserStore>,
    channel_chat: Option<Entity<ChannelChat>>,
    mentions: Vec<UserId>,
    mentions_task: Option<Task<()>>,
    reply_to_message_id: Option<u64>,
    edit_message_id: Option<u64>,
}

struct MessageEditorCompletionProvider(WeakEntity<MessageEditor>);

impl CompletionProvider for MessageEditorCompletionProvider {
    fn completions(
        &self,
        _excerpt_id: ExcerptId,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        _: editor::CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Option<Vec<Completion>>>> {
        let Some(handle) = self.0.upgrade() else {
            return Task::ready(Ok(None));
        };
        handle.update(cx, |message_editor, cx| {
            message_editor.completions(buffer, buffer_position, cx)
        })
    }

    fn resolve_completions(
        &self,
        _buffer: Entity<Buffer>,
        _completion_indices: Vec<usize>,
        _completions: Rc<RefCell<Box<[Completion]>>>,
        _cx: &mut Context<Editor>,
    ) -> Task<anyhow::Result<bool>> {
        Task::ready(Ok(false))
    }

    fn is_completion_trigger(
        &self,
        _buffer: &Entity<Buffer>,
        _position: language::Anchor,
        text: &str,
        _trigger_in_words: bool,
        _cx: &mut Context<Editor>,
    ) -> bool {
        text == "@"
    }
}

impl MessageEditor {
    pub fn new(
        language_registry: Arc<LanguageRegistry>,
        user_store: Entity<UserStore>,
        channel_chat: Option<Entity<ChannelChat>>,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let this = cx.entity().downgrade();
        editor.update(cx, |editor, cx| {
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_use_autoclose(false);
            editor.set_show_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_completion_provider(Some(Box::new(MessageEditorCompletionProvider(this))));
            editor.set_auto_replace_emoji_shortcode(
                MessageEditorSettings::get_global(cx)
                    .auto_replace_emoji_shortcode
                    .unwrap_or_default(),
            );
        });

        let buffer = editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("message editor must be singleton");

        cx.subscribe_in(&buffer, window, Self::on_buffer_event)
            .detach();
        cx.observe_global::<settings::SettingsStore>(|this, cx| {
            this.editor.update(cx, |editor, cx| {
                editor.set_auto_replace_emoji_shortcode(
                    MessageEditorSettings::get_global(cx)
                        .auto_replace_emoji_shortcode
                        .unwrap_or_default(),
                )
            })
        })
        .detach();

        let markdown = language_registry.language_for_name("Markdown");
        cx.spawn_in(window, async move |_, cx| {
            let markdown = markdown.await.context("failed to load Markdown language")?;
            buffer.update(cx, |buffer, cx| buffer.set_language(Some(markdown), cx))
        })
        .detach_and_log_err(cx);

        Self {
            editor,
            user_store,
            channel_chat,
            mentions: Vec::new(),
            mentions_task: None,
            reply_to_message_id: None,
            edit_message_id: None,
        }
    }

    pub fn reply_to_message_id(&self) -> Option<u64> {
        self.reply_to_message_id
    }

    pub fn set_reply_to_message_id(&mut self, reply_to_message_id: u64) {
        self.reply_to_message_id = Some(reply_to_message_id);
    }

    pub fn clear_reply_to_message_id(&mut self) {
        self.reply_to_message_id = None;
    }

    pub fn edit_message_id(&self) -> Option<u64> {
        self.edit_message_id
    }

    pub fn set_edit_message_id(&mut self, edit_message_id: u64) {
        self.edit_message_id = Some(edit_message_id);
    }

    pub fn clear_edit_message_id(&mut self) {
        self.edit_message_id = None;
    }

    pub fn set_channel_chat(&mut self, chat: Entity<ChannelChat>, cx: &mut Context<Self>) {
        let channel_id = chat.read(cx).channel_id;
        self.channel_chat = Some(chat);
        let channel_name = ChannelStore::global(cx)
            .read(cx)
            .channel_for_id(channel_id)
            .map(|channel| channel.name.clone());
        self.editor.update(cx, |editor, cx| {
            if let Some(channel_name) = channel_name {
                editor.set_placeholder_text(format!("Message #{channel_name}"), cx);
            } else {
                editor.set_placeholder_text("Message Channel", cx);
            }
        });
    }

    pub fn take_message(&mut self, window: &mut Window, cx: &mut Context<Self>) -> MessageParams {
        self.editor.update(cx, |editor, cx| {
            let highlights = editor.text_highlights::<Self>(cx);
            let text = editor.text(cx);
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let mentions = if let Some((_, ranges)) = highlights {
                ranges
                    .iter()
                    .map(|range| range.to_offset(&snapshot))
                    .zip(self.mentions.iter().copied())
                    .collect()
            } else {
                Vec::new()
            };

            editor.clear(window, cx);
            self.mentions.clear();
            let reply_to_message_id = std::mem::take(&mut self.reply_to_message_id);

            MessageParams {
                text,
                mentions,
                reply_to_message_id,
            }
        })
    }

    fn on_buffer_event(
        &mut self,
        buffer: &Entity<Buffer>,
        event: &language::BufferEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let language::BufferEvent::Reparsed | language::BufferEvent::Edited = event {
            let buffer = buffer.read(cx).snapshot();
            self.mentions_task = Some(cx.spawn_in(window, async move |this, cx| {
                cx.background_executor()
                    .timer(MENTIONS_DEBOUNCE_INTERVAL)
                    .await;
                Self::find_mentions(this, buffer, cx).await;
            }));
        }
    }

    fn completions(
        &mut self,
        buffer: &Entity<Buffer>,
        end_anchor: Anchor,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Vec<Completion>>>> {
        if let Some((start_anchor, query, candidates)) =
            self.collect_mention_candidates(buffer, end_anchor, cx)
        {
            if !candidates.is_empty() {
                return cx.spawn(async move |_, cx| {
                    Ok(Some(
                        Self::resolve_completions_for_candidates(
                            &cx,
                            query.as_str(),
                            &candidates,
                            start_anchor..end_anchor,
                            Self::completion_for_mention,
                        )
                        .await,
                    ))
                });
            }
        }

        if let Some((start_anchor, query, candidates)) =
            self.collect_emoji_candidates(buffer, end_anchor, cx)
        {
            if !candidates.is_empty() {
                return cx.spawn(async move |_, cx| {
                    Ok(Some(
                        Self::resolve_completions_for_candidates(
                            &cx,
                            query.as_str(),
                            candidates,
                            start_anchor..end_anchor,
                            Self::completion_for_emoji,
                        )
                        .await,
                    ))
                });
            }
        }

        Task::ready(Ok(Some(Vec::new())))
    }

    async fn resolve_completions_for_candidates(
        cx: &AsyncApp,
        query: &str,
        candidates: &[StringMatchCandidate],
        range: Range<Anchor>,
        completion_fn: impl Fn(&StringMatch) -> (String, CodeLabel),
    ) -> Vec<Completion> {
        let matches = fuzzy::match_strings(
            candidates,
            query,
            true,
            10,
            &Default::default(),
            cx.background_executor().clone(),
        )
        .await;

        matches
            .into_iter()
            .map(|mat| {
                let (new_text, label) = completion_fn(&mat);
                Completion {
                    replace_range: range.clone(),
                    new_text,
                    label,
                    icon_path: None,
                    confirm: None,
                    documentation: None,
                    insert_text_mode: None,
                    source: CompletionSource::Custom,
                }
            })
            .collect()
    }

    fn completion_for_mention(mat: &StringMatch) -> (String, CodeLabel) {
        let label = CodeLabel {
            filter_range: 1..mat.string.len() + 1,
            text: format!("@{}", mat.string),
            runs: Vec::new(),
        };
        (mat.string.clone(), label)
    }

    fn completion_for_emoji(mat: &StringMatch) -> (String, CodeLabel) {
        let emoji = emojis::get_by_shortcode(&mat.string).unwrap();
        let label = CodeLabel {
            filter_range: 1..mat.string.len() + 1,
            text: format!(":{}: {}", mat.string, emoji),
            runs: Vec::new(),
        };
        (emoji.to_string(), label)
    }

    fn collect_mention_candidates(
        &mut self,
        buffer: &Entity<Buffer>,
        end_anchor: Anchor,
        cx: &mut Context<Self>,
    ) -> Option<(Anchor, String, Vec<StringMatchCandidate>)> {
        let end_offset = end_anchor.to_offset(buffer.read(cx));

        let query = buffer.update(cx, |buffer, _| {
            let mut query = String::new();
            for ch in buffer.reversed_chars_at(end_offset).take(100) {
                if ch == '@' {
                    return Some(query.chars().rev().collect::<String>());
                }
                if ch.is_whitespace() || !ch.is_ascii() {
                    break;
                }
                query.push(ch);
            }
            None
        })?;

        let start_offset = end_offset - query.len();
        let start_anchor = buffer.read(cx).anchor_before(start_offset);

        let mut names = HashSet::default();
        if let Some(chat) = self.channel_chat.as_ref() {
            let chat = chat.read(cx);
            for participant in ChannelStore::global(cx)
                .read(cx)
                .channel_participants(chat.channel_id)
            {
                names.insert(participant.github_login.clone());
            }
            for message in chat
                .messages_in_range(chat.message_count().saturating_sub(100)..chat.message_count())
            {
                names.insert(message.sender.github_login.clone());
            }
        }

        let candidates = names
            .into_iter()
            .map(|user| StringMatchCandidate::new(0, &user))
            .collect::<Vec<_>>();

        Some((start_anchor, query, candidates))
    }

    fn collect_emoji_candidates(
        &mut self,
        buffer: &Entity<Buffer>,
        end_anchor: Anchor,
        cx: &mut Context<Self>,
    ) -> Option<(Anchor, String, &'static [StringMatchCandidate])> {
        static EMOJI_FUZZY_MATCH_CANDIDATES: LazyLock<Vec<StringMatchCandidate>> =
            LazyLock::new(|| {
                let emojis = emojis::iter()
                    .flat_map(|s| s.shortcodes())
                    .map(|emoji| StringMatchCandidate::new(0, emoji))
                    .collect::<Vec<_>>();
                emojis
            });

        let end_offset = end_anchor.to_offset(buffer.read(cx));

        let query = buffer.update(cx, |buffer, _| {
            let mut query = String::new();
            for ch in buffer.reversed_chars_at(end_offset).take(100) {
                if ch == ':' {
                    let next_char = buffer
                        .reversed_chars_at(end_offset - query.len() - 1)
                        .next();
                    // Ensure we are at the start of the message or that the previous character is a whitespace
                    if next_char.is_none() || next_char.unwrap().is_whitespace() {
                        return Some(query.chars().rev().collect::<String>());
                    }

                    // If the previous character is not a whitespace, we are in the middle of a word
                    // and we only want to complete the shortcode if the word is made up of other emojis
                    let mut containing_word = String::new();
                    for ch in buffer
                        .reversed_chars_at(end_offset - query.len() - 1)
                        .take(100)
                    {
                        if ch.is_whitespace() {
                            break;
                        }
                        containing_word.push(ch);
                    }
                    let containing_word = containing_word.chars().rev().collect::<String>();
                    if util::word_consists_of_emojis(containing_word.as_str()) {
                        return Some(query.chars().rev().collect::<String>());
                    }
                    break;
                }
                if ch.is_whitespace() || !ch.is_ascii() {
                    break;
                }
                query.push(ch);
            }
            None
        })?;

        let start_offset = end_offset - query.len() - 1;
        let start_anchor = buffer.read(cx).anchor_before(start_offset);

        Some((start_anchor, query, &EMOJI_FUZZY_MATCH_CANDIDATES))
    }

    async fn find_mentions(
        this: WeakEntity<MessageEditor>,
        buffer: BufferSnapshot,
        cx: &mut AsyncWindowContext,
    ) {
        let (buffer, ranges) = cx
            .background_spawn(async move {
                let ranges = MENTIONS_SEARCH.search(&buffer, None).await;
                (buffer, ranges)
            })
            .await;

        this.update(cx, |this, cx| {
            let mut anchor_ranges = Vec::new();
            let mut mentioned_user_ids = Vec::new();
            let mut text = String::new();

            this.editor.update(cx, |editor, cx| {
                let multi_buffer = editor.buffer().read(cx).snapshot(cx);
                for range in ranges {
                    text.clear();
                    text.extend(buffer.text_for_range(range.clone()));
                    if let Some(username) = text.strip_prefix('@') {
                        if let Some(user) = this
                            .user_store
                            .read(cx)
                            .cached_user_by_github_login(username)
                        {
                            let start = multi_buffer.anchor_after(range.start);
                            let end = multi_buffer.anchor_after(range.end);

                            mentioned_user_ids.push(user.id);
                            anchor_ranges.push(start..end);
                        }
                    }
                }

                editor.clear_highlights::<Self>(cx);
                editor.highlight_text::<Self>(
                    anchor_ranges,
                    HighlightStyle {
                        font_weight: Some(FontWeight::BOLD),
                        ..Default::default()
                    },
                    cx,
                )
            });

            this.mentions = mentioned_user_ids;
            this.mentions_task.take();
        })
        .ok();
    }

    pub(crate) fn focus_handle(&self, cx: &gpui::App) -> gpui::FocusHandle {
        self.editor.read(cx).focus_handle(cx)
    }
}

impl Render for MessageEditor {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_fallbacks: settings.ui_font.fallbacks.clone(),
            font_size: TextSize::Small.rems(cx).into(),
            font_weight: settings.ui_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            ..Default::default()
        };

        div()
            .w_full()
            .px_2()
            .py_1()
            .bg(cx.theme().colors().editor_background)
            .rounded_sm()
            .child(EditorElement::new(
                &self.editor,
                EditorStyle {
                    local_player: cx.theme().players().local(),
                    text: text_style,
                    ..Default::default()
                },
            ))
    }
}
