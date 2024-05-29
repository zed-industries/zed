use anyhow::Result;
use channel::{ChannelChat, ChannelStore, MessageParams};
use client::{UserId, UserStore};
use collections::HashSet;
use editor::{AnchorRangeExt, CompletionProvider, Editor, EditorElement, EditorStyle};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AsyncWindowContext, FocusableView, FontStyle, FontWeight, HighlightStyle, IntoElement, Model,
    Render, Task, TextStyle, View, ViewContext, WeakView, WhiteSpace,
};
use language::{
    language_settings::SoftWrap, Anchor, Buffer, BufferSnapshot, CodeLabel, LanguageRegistry,
    LanguageServerId, ToOffset,
};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use project::{search::SearchQuery, Completion};
use settings::Settings;
use std::{ops::Range, sync::Arc, time::Duration};
use theme::ThemeSettings;
use ui::{prelude::*, TextSize};

use crate::panel_settings::MessageEditorSettings;

const MENTIONS_DEBOUNCE_INTERVAL: Duration = Duration::from_millis(50);

lazy_static! {
    static ref MENTIONS_SEARCH: SearchQuery =
        SearchQuery::regex("@[-_\\w]+", false, false, false, Vec::new(), Vec::new()).unwrap();
}

pub struct MessageEditor {
    pub editor: View<Editor>,
    user_store: Model<UserStore>,
    channel_chat: Option<Model<ChannelChat>>,
    mentions: Vec<UserId>,
    mentions_task: Option<Task<()>>,
    reply_to_message_id: Option<u64>,
    edit_message_id: Option<u64>,
}

struct MessageEditorCompletionProvider(WeakView<MessageEditor>);

impl CompletionProvider for MessageEditorCompletionProvider {
    fn completions(
        &self,
        buffer: &Model<Buffer>,
        buffer_position: language::Anchor,
        cx: &mut ViewContext<Editor>,
    ) -> Task<anyhow::Result<Vec<Completion>>> {
        let Some(handle) = self.0.upgrade() else {
            return Task::ready(Ok(Vec::new()));
        };
        handle.update(cx, |message_editor, cx| {
            message_editor.completions(buffer, buffer_position, cx)
        })
    }

    fn resolve_completions(
        &self,
        _buffer: Model<Buffer>,
        _completion_indices: Vec<usize>,
        _completions: Arc<RwLock<Box<[Completion]>>>,
        _cx: &mut ViewContext<Editor>,
    ) -> Task<anyhow::Result<bool>> {
        Task::ready(Ok(false))
    }

    fn apply_additional_edits_for_completion(
        &self,
        _buffer: Model<Buffer>,
        _completion: Completion,
        _push_to_history: bool,
        _cx: &mut ViewContext<Editor>,
    ) -> Task<Result<Option<language::Transaction>>> {
        Task::ready(Ok(None))
    }

    fn is_completion_trigger(
        &self,
        _buffer: &Model<Buffer>,
        _position: language::Anchor,
        text: &str,
        _trigger_in_words: bool,
        _cx: &mut ViewContext<Editor>,
    ) -> bool {
        text == "@"
    }
}

impl MessageEditor {
    pub fn new(
        language_registry: Arc<LanguageRegistry>,
        user_store: Model<UserStore>,
        channel_chat: Option<Model<ChannelChat>>,
        editor: View<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let this = cx.view().downgrade();
        editor.update(cx, |editor, cx| {
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_use_autoclose(false);
            editor.set_show_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_completion_provider(Box::new(MessageEditorCompletionProvider(this)));
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

        cx.subscribe(&buffer, Self::on_buffer_event).detach();
        cx.observe_global::<settings::SettingsStore>(|view, cx| {
            view.editor.update(cx, |editor, cx| {
                editor.set_auto_replace_emoji_shortcode(
                    MessageEditorSettings::get_global(cx)
                        .auto_replace_emoji_shortcode
                        .unwrap_or_default(),
                )
            })
        })
        .detach();

        let markdown = language_registry.language_for_name("Markdown");
        cx.spawn(|_, mut cx| async move {
            let markdown = markdown.await?;
            buffer.update(&mut cx, |buffer, cx| {
                buffer.set_language(Some(markdown), cx)
            })
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

    pub fn set_channel_chat(&mut self, chat: Model<ChannelChat>, cx: &mut ViewContext<Self>) {
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

    pub fn take_message(&mut self, cx: &mut ViewContext<Self>) -> MessageParams {
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

            editor.clear(cx);
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
        buffer: Model<Buffer>,
        event: &language::Event,
        cx: &mut ViewContext<Self>,
    ) {
        if let language::Event::Reparsed | language::Event::Edited = event {
            let buffer = buffer.read(cx).snapshot();
            self.mentions_task = Some(cx.spawn(|this, cx| async move {
                cx.background_executor()
                    .timer(MENTIONS_DEBOUNCE_INTERVAL)
                    .await;
                Self::find_mentions(this, buffer, cx).await;
            }));
        }
    }

    fn completions(
        &mut self,
        buffer: &Model<Buffer>,
        end_anchor: Anchor,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<Vec<Completion>>> {
        if let Some((start_anchor, query, candidates)) =
            self.collect_mention_candidates(buffer, end_anchor, cx)
        {
            if !candidates.is_empty() {
                return cx.spawn(|_, cx| async move {
                    Ok(Self::resolve_completions_for_candidates(
                        &cx,
                        query.as_str(),
                        &candidates,
                        start_anchor..end_anchor,
                        Self::completion_for_mention,
                    )
                    .await)
                });
            }
        }

        if let Some((start_anchor, query, candidates)) =
            self.collect_emoji_candidates(buffer, end_anchor, cx)
        {
            if !candidates.is_empty() {
                return cx.spawn(|_, cx| async move {
                    Ok(Self::resolve_completions_for_candidates(
                        &cx,
                        query.as_str(),
                        candidates,
                        start_anchor..end_anchor,
                        Self::completion_for_emoji,
                    )
                    .await)
                });
            }
        }

        Task::ready(Ok(vec![]))
    }

    async fn resolve_completions_for_candidates(
        cx: &AsyncWindowContext,
        query: &str,
        candidates: &[StringMatchCandidate],
        range: Range<Anchor>,
        completion_fn: impl Fn(&StringMatch) -> (String, CodeLabel),
    ) -> Vec<Completion> {
        let matches = fuzzy::match_strings(
            &candidates,
            &query,
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
                    old_range: range.clone(),
                    new_text,
                    label,
                    documentation: None,
                    server_id: LanguageServerId(0), // TODO: Make this optional or something?
                    lsp_completion: Default::default(), // TODO: Make this optional or something?
                    confirm: None,
                    show_new_completions_on_confirm: false,
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
        buffer: &Model<Buffer>,
        end_anchor: Anchor,
        cx: &mut ViewContext<Self>,
    ) -> Option<(Anchor, String, Vec<StringMatchCandidate>)> {
        let end_offset = end_anchor.to_offset(buffer.read(cx));

        let Some(query) = buffer.update(cx, |buffer, _| {
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
        }) else {
            return None;
        };

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
            .map(|user| StringMatchCandidate {
                id: 0,
                string: user.clone(),
                char_bag: user.chars().collect(),
            })
            .collect::<Vec<_>>();

        Some((start_anchor, query, candidates))
    }

    fn collect_emoji_candidates(
        &mut self,
        buffer: &Model<Buffer>,
        end_anchor: Anchor,
        cx: &mut ViewContext<Self>,
    ) -> Option<(Anchor, String, &'static [StringMatchCandidate])> {
        lazy_static! {
            static ref EMOJI_FUZZY_MATCH_CANDIDATES: Vec<StringMatchCandidate> = {
                let emojis = emojis::iter()
                    .flat_map(|s| s.shortcodes())
                    .map(|emoji| StringMatchCandidate {
                        id: 0,
                        string: emoji.to_string(),
                        char_bag: emoji.chars().collect(),
                    })
                    .collect::<Vec<_>>();
                emojis
            };
        }

        let end_offset = end_anchor.to_offset(buffer.read(cx));

        let Some(query) = buffer.update(cx, |buffer, _| {
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
        }) else {
            return None;
        };

        let start_offset = end_offset - query.len() - 1;
        let start_anchor = buffer.read(cx).anchor_before(start_offset);

        Some((start_anchor, query, &EMOJI_FUZZY_MATCH_CANDIDATES))
    }

    async fn find_mentions(
        this: WeakView<MessageEditor>,
        buffer: BufferSnapshot,
        mut cx: AsyncWindowContext,
    ) {
        let (buffer, ranges) = cx
            .background_executor()
            .spawn(async move {
                let ranges = MENTIONS_SEARCH.search(&buffer, None).await;
                (buffer, ranges)
            })
            .await;

        this.update(&mut cx, |this, cx| {
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

    pub(crate) fn focus_handle(&self, cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.editor.read(cx).focus_handle(cx)
    }
}

impl Render for MessageEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_size: TextSize::Small.rems(cx).into(),
            font_weight: settings.ui_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
        };

        div()
            .w_full()
            .px_2()
            .py_1()
            .bg(cx.theme().colors().editor_background)
            .rounded_md()
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
