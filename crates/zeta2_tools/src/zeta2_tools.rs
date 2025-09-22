use std::{
    collections::hash_map::Entry,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};

use chrono::TimeDelta;
use client::{Client, UserStore};
use collections::HashMap;
use editor::{Editor, EditorMode, ExcerptRange, MultiBuffer};
use futures::StreamExt as _;
use gpui::{
    BorderStyle, EdgesRefinement, Entity, EventEmitter, FocusHandle, Focusable, Length,
    StyleRefinement, Subscription, Task, TextStyleRefinement, UnderlineStyle, actions, prelude::*,
};
use language::{Buffer, DiskState};
use markdown::{HeadingLevelStyles, Markdown, MarkdownElement, MarkdownStyle};
use project::{Project, WorktreeId};
use ui::prelude::*;
use ui_input::SingleLineInput;
use workspace::{Item, SplitDirection, Workspace};
use zeta2::Zeta;

use edit_prediction_context::SnippetStyle;

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
                    EditPredictionTools::new(
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

pub struct EditPredictionTools {
    focus_handle: FocusHandle,
    project: Entity<Project>,
    last_prediction: Option<Result<LastPredictionState, SharedString>>,
    max_bytes_input: Entity<SingleLineInput>,
    min_bytes_input: Entity<SingleLineInput>,
    cursor_context_ratio_input: Entity<SingleLineInput>,
    active_view: ActiveView,
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
    prompt_md: Entity<Markdown>,
    model_response_md: Entity<Markdown>,
}

impl EditPredictionTools {
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
            // todo!
            // cx.subscribe_in(
            //     &input.read(cx).editor().clone(),
            //     window,
            //     |this, _, event, window, cx| {
            // if let EditorEvent::BufferEdited = event
            //     && let Some(editor) = this.last_editor.upgrade()
            // {
            //     this.update_context(&editor, window, cx);
            // }
            //     },
            // )
            // .detach();
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
                // fn number_input_value<T: FromStr + Default>(
                //     input: &Entity<SingleLineInput>,
                //     cx: &App,
                // ) -> T {
                //     input
                //         .read(cx)
                //         .editor()
                //         .read(cx)
                //         .text(cx)
                //         .parse::<T>()
                //         .unwrap_or_default()
                // }

                // let options = EditPredictionExcerptOptions {
                //     max_bytes: number_input_value(&this.max_bytes_input, cx),
                //     min_bytes: number_input_value(&this.min_bytes_input, cx),
                //     target_before_cursor_over_total_bytes: number_input_value(
                //         &this.cursor_context_ratio_input,
                //         cx,
                //     ),
                // };

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
                        prompt_md: cx.new(|cx| {
                            Markdown::new(prediction.request.prompt.into(), None, None, cx)
                        }),
                        model_response_md: cx.new(|cx| {
                            Markdown::new(prediction.request.model_response.into(), None, None, cx)
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

impl Focusable for EditPredictionTools {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for EditPredictionTools {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Zeta2 Inspector".into()
    }
}

impl EventEmitter<()> for EditPredictionTools {}

impl Render for EditPredictionTools {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .items_start()
                    .w_full()
                    .child(
                        v_flex()
                            .flex_1()
                            .p_4()
                            .gap_2()
                            .child(Headline::new("Excerpt Options").size(HeadlineSize::Small))
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(self.max_bytes_input.clone())
                                    .child(self.min_bytes_input.clone())
                                    .child(self.cursor_context_ratio_input.clone()),
                            )
                            .child(div().flex_1())
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
                                                )
                                                .selected(self.active_view == ActiveView::Context),
                                                ui::ToggleButtonSimple::new(
                                                    "Inference",
                                                    cx.listener(|this, _, _, cx| {
                                                        this.active_view = ActiveView::Inference;
                                                        cx.notify();
                                                    }),
                                                )
                                                .selected(self.active_view == ActiveView::Context),
                                            ],
                                        )
                                        .style(ui::ToggleButtonGroupStyle::Outlined),
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
            .children(self.last_prediction.as_ref().map(|result| {
                match result {
                    Ok(state) => match &self.active_view {
                        ActiveView::Context => state.context_editor.clone().into_any_element(),
                        ActiveView::Inference => h_flex()
                            .items_start()
                            .w_full()
                            .gap_2()
                            .bg(cx.theme().colors().editor_background)
                            // todo! fix layout
                            // should we use an editor instead of markdown?
                            // I don't want to use a label, because I want it to be selectable
                            // and maybe an editor would make sense later too if we make it editable
                            .child(
                                v_flex()
                                    .flex_1()
                                    .p_4()
                                    .gap_2()
                                    .child(
                                        ui::Headline::new("Prompt").size(ui::HeadlineSize::Small),
                                    )
                                    .child(MarkdownElement::new(
                                        state.prompt_md.clone(),
                                        markdown_style(window, cx),
                                    )),
                            )
                            .child(ui::vertical_divider())
                            .child(
                                v_flex()
                                    .flex_1()
                                    .p_4()
                                    .gap_2()
                                    .child(
                                        ui::Headline::new("Model Response")
                                            .size(ui::HeadlineSize::Small),
                                    )
                                    .child(MarkdownElement::new(
                                        state.model_response_md.clone(),
                                        markdown_style(window, cx),
                                    )),
                            )
                            .into_any(),
                    },
                    Err(err) => v_flex()
                        .p_4()
                        .gap_2()
                        .child(Label::new(err.clone()).buffer_font(cx))
                        .into_any(),
                }
            }))
    }
}

// Mostly copied from agent-ui
fn markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let colors = cx.theme().colors();

    let buffer_font_size = TextSize::Small.rems(cx);
    let mut text_style = window.text_style();
    let line_height = buffer_font_size * 1.75;

    let font_size = TextSize::Small.rems(cx);

    let text_color = colors.text;

    text_style.refine(&TextStyleRefinement {
        font_size: Some(font_size.into()),
        line_height: Some(line_height.into()),
        color: Some(text_color),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style: text_style.clone(),
        syntax: cx.theme().syntax().clone(),
        selection_background_color: colors.element_selection_background,
        code_block_overflow_x_scroll: true,
        table_overflow_x_scroll: true,
        heading_level_styles: Some(HeadingLevelStyles {
            h1: Some(TextStyleRefinement {
                font_size: Some(rems(1.15).into()),
                ..Default::default()
            }),
            h2: Some(TextStyleRefinement {
                font_size: Some(rems(1.1).into()),
                ..Default::default()
            }),
            h3: Some(TextStyleRefinement {
                font_size: Some(rems(1.05).into()),
                ..Default::default()
            }),
            h4: Some(TextStyleRefinement {
                font_size: Some(rems(1.).into()),
                ..Default::default()
            }),
            h5: Some(TextStyleRefinement {
                font_size: Some(rems(0.95).into()),
                ..Default::default()
            }),
            h6: Some(TextStyleRefinement {
                font_size: Some(rems(0.875).into()),
                ..Default::default()
            }),
        }),
        code_block: StyleRefinement {
            padding: EdgesRefinement {
                top: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                left: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                right: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                bottom: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
            },
            margin: EdgesRefinement {
                top: Some(Length::Definite(Pixels(8.).into())),
                left: Some(Length::Definite(Pixels(0.).into())),
                right: Some(Length::Definite(Pixels(0.).into())),
                bottom: Some(Length::Definite(Pixels(12.).into())),
            },
            border_style: Some(BorderStyle::Solid),
            border_widths: EdgesRefinement {
                top: Some(AbsoluteLength::Pixels(Pixels(1.))),
                left: Some(AbsoluteLength::Pixels(Pixels(1.))),
                right: Some(AbsoluteLength::Pixels(Pixels(1.))),
                bottom: Some(AbsoluteLength::Pixels(Pixels(1.))),
            },
            border_color: Some(colors.border_variant),
            background: Some(colors.editor_background.into()),
            text: Some(TextStyleRefinement {
                font_size: Some(buffer_font_size.into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        inline_code: TextStyleRefinement {
            font_size: Some(buffer_font_size.into()),
            background_color: Some(colors.editor_foreground.opacity(0.08)),
            ..Default::default()
        },
        link: TextStyleRefinement {
            background_color: Some(colors.editor_foreground.opacity(0.025)),
            underline: Some(UnderlineStyle {
                color: Some(colors.text_accent.opacity(0.5)),
                thickness: px(1.),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
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
