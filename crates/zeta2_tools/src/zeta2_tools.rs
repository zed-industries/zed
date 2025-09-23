use std::{
    collections::hash_map::Entry,
    ffi::OsStr,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use chrono::TimeDelta;
use client::{Client, UserStore};
use collections::HashMap;
use editor::{
    Editor, EditorElement, EditorEvent, EditorMode, EditorStyle, ExcerptRange, MultiBuffer,
};
use futures::StreamExt as _;
use gpui::{
    BorderStyle, EdgesRefinement, Entity, EventEmitter, FocusHandle, Focusable, Length,
    StyleRefinement, Subscription, Task, TextStyleRefinement, UnderlineStyle, actions, prelude::*,
};
use language::{Buffer, DiskState};
use markdown::{HeadingLevelStyles, MarkdownStyle};
use project::{Project, WorktreeId};
use ui::prelude::*;
use ui_input::SingleLineInput;
use util::ResultExt;
use workspace::{Item, SplitDirection, Workspace};
use zeta2::Zeta;

use edit_prediction_context::{EditPredictionExcerptOptions, SnippetStyle};

actions!(
    dev,
    [
        /// Opens the language server protocol logs viewer.
        OpenZeta2Inspector
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(move |workspace: &mut Workspace, _, _cx| {
        workspace.register_action(move |workspace, _: &OpenZeta2Inspector, window, cx| {
            let project = workspace.project();
            workspace.split_item(
                SplitDirection::Right,
                Box::new(cx.new(|cx| {
                    Zeta2Inspector::new(
                        &project,
                        workspace.client(),
                        workspace.user_store(),
                        window,
                        cx,
                    )
                })),
                window,
                cx,
            );
        });
    })
    .detach();
}

pub struct Zeta2Inspector {
    focus_handle: FocusHandle,
    project: Entity<Project>,
    last_prediction: Option<Result<LastPredictionState, SharedString>>,
    max_bytes_input: Entity<SingleLineInput>,
    min_bytes_input: Entity<SingleLineInput>,
    cursor_context_ratio_input: Entity<SingleLineInput>,
    active_view: ActiveView,
    zeta: Entity<Zeta>,
    _active_editor_subscription: Option<Subscription>,
    _update_state_task: Task<()>,
    _receive_task: Task<()>,
}

#[derive(PartialEq)]
enum ActiveView {
    Context,
    Inference,
}

struct LastPredictionState {
    context_editor: Entity<Editor>,
    retrieval_time: TimeDelta,
    prompt_planning_time: TimeDelta,
    inference_time: TimeDelta,
    parsing_time: TimeDelta,
    prompt_editor: Entity<Editor>,
    model_response_editor: Entity<Editor>,
}

impl Zeta2Inspector {
    pub fn new(
        project: &Entity<Project>,
        client: &Arc<Client>,
        user_store: &Entity<UserStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let number_input = |label: &'static str,
                            value: &'static str,
                            window: &mut Window,
                            cx: &mut Context<Self>|
         -> Entity<SingleLineInput> {
            let input = cx.new(|cx| {
                let input = SingleLineInput::new(window, cx, "")
                    .label(label)
                    .label_min_width(px(64.));
                input.set_text(value, window, cx);
                input
            });

            cx.subscribe_in(
                &input.read(cx).editor().clone(),
                window,
                |this, _, event, _window, cx| {
                    let EditorEvent::BufferEdited = event else {
                        return;
                    };

                    fn number_input_value<T: FromStr + Default>(
                        input: &Entity<SingleLineInput>,
                        cx: &App,
                    ) -> T {
                        input
                            .read(cx)
                            .editor()
                            .read(cx)
                            .text(cx)
                            .parse::<T>()
                            .unwrap_or_default()
                    }

                    let options = EditPredictionExcerptOptions {
                        max_bytes: number_input_value(&this.max_bytes_input, cx),
                        min_bytes: number_input_value(&this.min_bytes_input, cx),
                        target_before_cursor_over_total_bytes: number_input_value(
                            &this.cursor_context_ratio_input,
                            cx,
                        ),
                    };

                    // todo! undo on drop
                    this.zeta.update(cx, |zeta, cx| {
                        zeta.set_options(options);
                        // todo! replay request
                    });
                },
            )
            .detach();
            input
        };

        let zeta = Zeta::global(client, user_store, cx);
        let mut request_rx = zeta.update(cx, |zeta, _cx| zeta.debug_info());
        let receive_task = cx.spawn_in(window, async move |this, cx| {
            while let Some(prediction_result) = request_rx.next().await {
                this.update_in(cx, |this, window, cx| match prediction_result {
                    Ok(prediction) => {
                        this.update_last_prediction(prediction, window, cx);
                    }
                    Err(err) => {
                        this.last_prediction = Some(Err(err.into()));
                        cx.notify();
                    }
                })
                .ok();
            }
        });

        Self {
            focus_handle: cx.focus_handle(),
            project: project.clone(),
            last_prediction: None,
            active_view: ActiveView::Context,
            max_bytes_input: number_input("Max Bytes", "512", window, cx),
            min_bytes_input: number_input("Min Bytes", "128", window, cx),
            cursor_context_ratio_input: number_input("Cursor Context Ratio", "0.5", window, cx),
            zeta,
            _active_editor_subscription: None,
            _update_state_task: Task::ready(()),
            _receive_task: receive_task,
        }
    }

    fn update_last_prediction(
        &mut self,
        prediction: zeta2::PredictionDebugInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(worktree_id) = self
            .project
            .read(cx)
            .worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).id())
        else {
            log::error!("Open a worktree to use edit prediction debug view");
            self.last_prediction.take();
            return;
        };

        self._update_state_task = cx.spawn_in(window, {
            let language_registry = self.project.read(cx).languages().clone();
            async move |this, cx| {
                let mut languages = HashMap::default();
                for lang_id in prediction
                    .context
                    .snippets
                    .iter()
                    .map(|snippet| snippet.declaration.identifier().language_id)
                    .chain(prediction.context.excerpt_text.language_id)
                {
                    if let Entry::Vacant(entry) = languages.entry(lang_id) {
                        // Most snippets are gonna be the same language,
                        // so we think it's fine to do this sequentially for now
                        entry.insert(language_registry.language_for_id(lang_id).await.ok());
                    }
                }

                let markdown_language = language_registry
                    .language_for_name("Markdown")
                    .await
                    .log_err();

                this.update_in(cx, |this, window, cx| {
                    let context_editor = cx.new(|cx| {
                        let multibuffer = cx.new(|cx| {
                            let mut multibuffer = MultiBuffer::new(language::Capability::ReadOnly);
                            let excerpt_file = Arc::new(ExcerptMetadataFile {
                                title: PathBuf::from("Cursor Excerpt").into(),
                                worktree_id,
                            });

                            let excerpt_buffer = cx.new(|cx| {
                                let mut buffer =
                                    Buffer::local(prediction.context.excerpt_text.body, cx);
                                if let Some(language) = prediction
                                    .context
                                    .excerpt_text
                                    .language_id
                                    .as_ref()
                                    .and_then(|id| languages.get(id))
                                {
                                    buffer.set_language(language.clone(), cx);
                                }
                                buffer.file_updated(excerpt_file, cx);
                                buffer
                            });

                            multibuffer.push_excerpts(
                                excerpt_buffer,
                                [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
                                cx,
                            );

                            for snippet in &prediction.context.snippets {
                                let path = this
                                    .project
                                    .read(cx)
                                    .path_for_entry(snippet.declaration.project_entry_id(), cx);

                                let snippet_file = Arc::new(ExcerptMetadataFile {
                                    title: PathBuf::from(format!(
                                        "{} (Score density: {})",
                                        path.map(|p| p.path.to_string_lossy().to_string())
                                            .unwrap_or_else(|| "".to_string()),
                                        snippet.score_density(SnippetStyle::Declaration)
                                    ))
                                    .into(),
                                    worktree_id,
                                });

                                let excerpt_buffer = cx.new(|cx| {
                                    let mut buffer =
                                        Buffer::local(snippet.declaration.item_text().0, cx);
                                    buffer.file_updated(snippet_file, cx);
                                    if let Some(language) =
                                        languages.get(&snippet.declaration.identifier().language_id)
                                    {
                                        buffer.set_language(language.clone(), cx);
                                    }
                                    buffer
                                });

                                multibuffer.push_excerpts(
                                    excerpt_buffer,
                                    [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
                                    cx,
                                );
                            }

                            multibuffer
                        });

                        Editor::new(EditorMode::full(), multibuffer, None, window, cx)
                    });

                    this.last_prediction = Some(Ok(LastPredictionState {
                        context_editor,
                        prompt_editor: cx.new(|cx| {
                            let buffer = cx.new(|cx| {
                                let mut buffer = Buffer::local(prediction.request.prompt, cx);
                                buffer.set_language(markdown_language.clone(), cx);
                                buffer
                            });
                            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
                            let mut editor =
                                Editor::new(EditorMode::full(), buffer, None, window, cx);
                            editor.set_read_only(true);
                            editor.set_show_line_numbers(false, cx);
                            editor.set_show_gutter(false, cx);
                            editor.set_show_scrollbars(false, cx);
                            editor
                        }),
                        model_response_editor: cx.new(|cx| {
                            let buffer = cx.new(|cx| {
                                let mut buffer =
                                    Buffer::local(prediction.request.model_response, cx);
                                buffer.set_language(markdown_language, cx);
                                buffer
                            });
                            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
                            let mut editor =
                                Editor::new(EditorMode::full(), buffer, None, window, cx);
                            editor.set_read_only(true);
                            editor.set_show_line_numbers(false, cx);
                            editor.set_show_gutter(false, cx);
                            editor.set_show_scrollbars(false, cx);
                            editor
                        }),
                        retrieval_time: prediction.retrieval_time,
                        prompt_planning_time: prediction.request.prompt_planning_time,
                        inference_time: prediction.request.inference_time,
                        parsing_time: prediction.request.parsing_time,
                    }));
                    cx.notify();
                })
                .ok();
            }
        });
    }

    fn render_duration(name: &'static str, time: chrono::TimeDelta) -> Div {
        h_flex()
            .gap_1()
            .child(Label::new(name).color(Color::Muted).size(LabelSize::Small))
            .child(
                Label::new(if time.num_microseconds().unwrap_or(0) > 1000 {
                    format!("{} ms", time.num_milliseconds())
                } else {
                    format!("{} µs", time.num_microseconds().unwrap_or(0))
                })
                .size(LabelSize::Small),
            )
    }
}

impl Focusable for Zeta2Inspector {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for Zeta2Inspector {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Zeta2 Inspector".into()
    }
}

impl EventEmitter<()> for Zeta2Inspector {}

impl Render for Zeta2Inspector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.last_prediction.as_ref() {
            None => v_flex()
                .size_full()
                .justify_center()
                .items_center()
                .child(Label::new("No predictions requested yet").size(LabelSize::Large))
                .into_any(),
            Some(Ok(state)) => match &self.active_view {
                ActiveView::Context => state.context_editor.clone().into_any_element(),
                ActiveView::Inference => h_flex()
                    .items_start()
                    .w_full()
                    .flex_1()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        v_flex()
                            .flex_1()
                            .gap_2()
                            .p_4()
                            .h_full()
                            .child(ui::Headline::new("Prompt").size(ui::HeadlineSize::XSmall))
                            .child(state.prompt_editor.clone()),
                    )
                    .child(ui::vertical_divider())
                    .child(
                        v_flex()
                            .flex_1()
                            .gap_2()
                            .h_full()
                            .p_4()
                            .child(
                                ui::Headline::new("Model Response").size(ui::HeadlineSize::XSmall),
                            )
                            .child(state.model_response_editor.clone()),
                    )
                    .into_any(),
            },

            Some(Err(err)) => v_flex()
                .p_4()
                .gap_2()
                .child(Label::new(err.clone()).buffer_font(cx))
                .into_any(),
        };

        v_flex()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .w_full()
                    .child(
                        v_flex()
                            .flex_1()
                            .p_4()
                            .h_full()
                            .justify_between()
                            .child(
                                v_flex()
                                    .gap_2()
                                    .child(
                                        Headline::new("Excerpt Options").size(HeadlineSize::Small),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(self.max_bytes_input.clone())
                                            .child(self.min_bytes_input.clone())
                                            .child(self.cursor_context_ratio_input.clone()),
                                    ),
                            )
                            .when(
                                self.last_prediction.as_ref().is_some_and(|r| r.is_ok()),
                                |this| {
                                    this.child(
                                        ui::ToggleButtonGroup::single_row(
                                            "prediction",
                                            [
                                                ui::ToggleButtonSimple::new(
                                                    "Context",
                                                    cx.listener(|this, _, _, cx| {
                                                        this.active_view = ActiveView::Context;
                                                        cx.notify();
                                                    }),
                                                ),
                                                ui::ToggleButtonSimple::new(
                                                    "Inference",
                                                    cx.listener(|this, _, _, cx| {
                                                        this.active_view = ActiveView::Inference;
                                                        cx.notify();
                                                    }),
                                                ),
                                            ],
                                        )
                                        .style(ui::ToggleButtonGroupStyle::Outlined)
                                        .selected_index(
                                            if self.active_view == ActiveView::Context {
                                                0
                                            } else {
                                                1
                                            },
                                        ),
                                    )
                                },
                            ),
                    )
                    .child(ui::vertical_divider())
                    .when_some(
                        self.last_prediction.as_ref().and_then(|r| r.as_ref().ok()),
                        |this, last_prediction| {
                            this.child(
                                v_flex()
                                    .p_4()
                                    .gap_2()
                                    .min_w(px(160.))
                                    .child(Headline::new("Stats").size(HeadlineSize::Small))
                                    .child(Self::render_duration(
                                        "Context retrieval",
                                        last_prediction.retrieval_time,
                                    ))
                                    .child(Self::render_duration(
                                        "Prompt planning",
                                        last_prediction.prompt_planning_time,
                                    ))
                                    .child(Self::render_duration(
                                        "Inference",
                                        last_prediction.inference_time,
                                    ))
                                    .child(Self::render_duration(
                                        "Parsing",
                                        last_prediction.parsing_time,
                                    )),
                            )
                        },
                    ),
            )
            .child(content)
    }
}

// Using same approach as commit view

struct ExcerptMetadataFile {
    title: Arc<Path>,
    worktree_id: WorktreeId,
}

impl language::File for ExcerptMetadataFile {
    fn as_local(&self) -> Option<&dyn language::LocalFile> {
        None
    }

    fn disk_state(&self) -> DiskState {
        DiskState::New
    }

    fn path(&self) -> &Arc<Path> {
        &self.title
    }

    fn full_path(&self, _: &App) -> PathBuf {
        self.title.as_ref().into()
    }

    fn file_name<'a>(&'a self, _: &'a App) -> &'a OsStr {
        self.title.file_name().unwrap()
    }

    fn worktree_id(&self, _: &App) -> WorktreeId {
        self.worktree_id
    }

    fn to_proto(&self, _: &App) -> language::proto::File {
        unimplemented!()
    }

    fn is_private(&self) -> bool {
        false
    }
}
