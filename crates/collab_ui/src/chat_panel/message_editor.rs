use anyhow::Result;
use channel::{ChannelMembership, ChannelStore, MessageParams};
use client::{ChannelId, UserId};
use collections::{HashMap, HashSet};
use editor::{AnchorRangeExt, CompletionProvider, Editor, EditorElement, EditorStyle};
use fuzzy::StringMatchCandidate;
use gpui::{
    AsyncWindowContext, FocusableView, FontStyle, FontWeight, HighlightStyle, IntoElement, Model,
    Render, SharedString, Task, TextStyle, View, ViewContext, WeakView, WhiteSpace,
};
use language::{
    language_settings::SoftWrap, Anchor, Buffer, BufferSnapshot, CodeLabel, Completion,
    LanguageRegistry, LanguageServerId, ToOffset,
};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use project::search::SearchQuery;
use settings::Settings;
use std::{sync::Arc, time::Duration};
use theme::ThemeSettings;
use ui::{prelude::*, UiTextSize};

const MENTIONS_DEBOUNCE_INTERVAL: Duration = Duration::from_millis(50);

lazy_static! {
    static ref MENTIONS_SEARCH: SearchQuery =
        SearchQuery::regex("@[-_\\w]+", false, false, false, Vec::new(), Vec::new()).unwrap();
}

pub struct MessageEditor {
    pub editor: View<Editor>,
    channel_store: Model<ChannelStore>,
    channel_members: HashMap<String, UserId>,
    mentions: Vec<UserId>,
    mentions_task: Option<Task<()>>,
    channel_id: Option<ChannelId>,
    reply_to_message_id: Option<u64>,
}

struct MessageEditorCompletionProvider(WeakView<MessageEditor>);

impl CompletionProvider for MessageEditorCompletionProvider {
    fn completions(
        &self,
        buffer: &Model<Buffer>,
        buffer_position: language::Anchor,
        cx: &mut ViewContext<Editor>,
    ) -> Task<anyhow::Result<Vec<language::Completion>>> {
        let Some(handle) = self.0.upgrade() else {
            return Task::ready(Ok(Vec::new()));
        };
        handle.update(cx, |message_editor, cx| {
            message_editor.completions(buffer, buffer_position, cx)
        })
    }

    fn resolve_completions(
        &self,
        _completion_indices: Vec<usize>,
        _completions: Arc<RwLock<Box<[language::Completion]>>>,
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
}

impl MessageEditor {
    pub fn new(
        language_registry: Arc<LanguageRegistry>,
        channel_store: Model<ChannelStore>,
        editor: View<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let this = cx.view().downgrade();
        editor.update(cx, |editor, cx| {
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_use_autoclose(false);
            editor.set_completion_provider(Box::new(MessageEditorCompletionProvider(this)));
        });

        let buffer = editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("message editor must be singleton");

        cx.subscribe(&buffer, Self::on_buffer_event).detach();

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
            channel_store,
            channel_members: HashMap::default(),
            channel_id: None,
            mentions: Vec::new(),
            mentions_task: None,
            reply_to_message_id: None,
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

    pub fn set_channel(
        &mut self,
        channel_id: ChannelId,
        channel_name: Option<SharedString>,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            if let Some(channel_name) = channel_name {
                editor.set_placeholder_text(format!("Message #{}", channel_name), cx);
            } else {
                editor.set_placeholder_text(format!("Message Channel"), cx);
            }
        });
        self.channel_id = Some(channel_id);
        self.refresh_users(cx);
    }

    pub fn refresh_users(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(channel_id) = self.channel_id {
            let members = self.channel_store.update(cx, |store, cx| {
                store.get_channel_member_details(channel_id, cx)
            });
            cx.spawn(|this, mut cx| async move {
                let members = members.await?;
                this.update(&mut cx, |this, cx| this.set_members(members, cx))?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }
    }

    pub fn set_members(&mut self, members: Vec<ChannelMembership>, _: &mut ViewContext<Self>) {
        self.channel_members.clear();
        self.channel_members.extend(
            members
                .into_iter()
                .map(|member| (member.user.github_login.clone(), member.user.id)),
        );
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
            return None;
        }) else {
            return Task::ready(Ok(vec![]));
        };

        let start_offset = end_offset - query.len();
        let start_anchor = buffer.read(cx).anchor_before(start_offset);

        let mut names = HashSet::default();
        for (github_login, _) in self.channel_members.iter() {
            names.insert(github_login.clone());
        }
        if let Some(channel_id) = self.channel_id {
            for participant in self.channel_store.read(cx).channel_participants(channel_id) {
                names.insert(participant.github_login.clone());
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
        cx.spawn(|_, cx| async move {
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                true,
                10,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            Ok(matches
                .into_iter()
                .map(|mat| Completion {
                    old_range: start_anchor..end_anchor,
                    new_text: mat.string.clone(),
                    label: CodeLabel {
                        filter_range: 1..mat.string.len() + 1,
                        text: format!("@{}", mat.string),
                        runs: Vec::new(),
                    },
                    documentation: None,
                    server_id: LanguageServerId(0), // TODO: Make this optional or something?
                    lsp_completion: Default::default(), // TODO: Make this optional or something?
                })
                .collect())
        })
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
                    if let Some(username) = text.strip_prefix("@") {
                        if let Some(user_id) = this.channel_members.get(username) {
                            let start = multi_buffer.anchor_after(range.start);
                            let end = multi_buffer.anchor_after(range.end);

                            mentioned_user_ids.push(*user_id);
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
            font_features: settings.ui_font.features,
            font_size: UiTextSize::Small.rems().into(),
            font_weight: FontWeight::NORMAL,
            font_style: FontStyle::Normal,
            line_height: relative(1.3).into(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use client::{Client, User, UserStore};
    use clock::FakeSystemClock;
    use gpui::TestAppContext;
    use language::{Language, LanguageConfig};
    use rpc::proto;
    use settings::SettingsStore;
    use util::{http::FakeHttpClient, test::marked_text_ranges};

    #[gpui::test]
    async fn test_message_editor(cx: &mut TestAppContext) {
        let language_registry = init_test(cx);

        let (editor, cx) = cx.add_window_view(|cx| {
            MessageEditor::new(
                language_registry,
                ChannelStore::global(cx),
                cx.new_view(|cx| Editor::auto_height(4, cx)),
                cx,
            )
        });
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            editor.set_members(
                vec![
                    ChannelMembership {
                        user: Arc::new(User {
                            github_login: "a-b".into(),
                            id: 101,
                            avatar_uri: "avatar_a-b".into(),
                        }),
                        kind: proto::channel_member::Kind::Member,
                        role: proto::ChannelRole::Member,
                    },
                    ChannelMembership {
                        user: Arc::new(User {
                            github_login: "C_D".into(),
                            id: 102,
                            avatar_uri: "avatar_C_D".into(),
                        }),
                        kind: proto::channel_member::Kind::Member,
                        role: proto::ChannelRole::Member,
                    },
                ],
                cx,
            );

            editor.editor.update(cx, |editor, cx| {
                editor.set_text("Hello, @a-b! Have you met @C_D?", cx)
            });
        });

        cx.executor().advance_clock(MENTIONS_DEBOUNCE_INTERVAL);

        editor.update(cx, |editor, cx| {
            let (text, ranges) = marked_text_ranges("Hello, «@a-b»! Have you met «@C_D»?", false);
            assert_eq!(
                editor.take_message(cx),
                MessageParams {
                    text,
                    mentions: vec![(ranges[0].clone(), 101), (ranges[1].clone(), 102)],
                    reply_to_message_id: None
                }
            );
        });
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<LanguageRegistry> {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);

            let clock = Arc::new(FakeSystemClock::default());
            let http = FakeHttpClient::with_404_response();
            let client = Client::new(clock, http.clone(), cx);
            let user_store = cx.new_model(|cx| UserStore::new(client.clone(), cx));
            theme::init(theme::LoadThemes::JustBase, cx);
            language::init(cx);
            editor::init(cx);
            client::init(&client, cx);
            channel::init(&client, user_store, cx);
        });

        let language_registry = Arc::new(LanguageRegistry::test());
        language_registry.add(Arc::new(Language::new(
            LanguageConfig {
                name: "Markdown".into(),
                ..Default::default()
            },
            Some(tree_sitter_markdown::language()),
        )));
        language_registry
    }
}
