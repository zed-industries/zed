use std::{
    cmp::Reverse, collections::hash_map::Entry, path::PathBuf, str::FromStr, sync::Arc,
    time::Duration,
};

use chrono::TimeDelta;
use client::{Client, UserStore};
use cloud_llm_client::predict_edits_v3::{DeclarationScoreComponents, PromptFormat};
use collections::HashMap;
use editor::{Editor, EditorEvent, EditorMode, ExcerptRange, MultiBuffer};
use futures::{StreamExt as _, channel::oneshot};
use gpui::{
    CursorStyle, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task, WeakEntity,
    actions, prelude::*,
};
use language::{Buffer, DiskState};
use ordered_float::OrderedFloat;
use project::{Project, WorktreeId};
use ui::{ContextMenu, ContextMenuEntry, DropdownMenu, prelude::*};
use ui_input::SingleLineInput;
use util::{ResultExt, paths::PathStyle, rel_path::RelPath};
use workspace::{Item, SplitDirection, Workspace};
use zeta2::{DEFAULT_CONTEXT_OPTIONS, PredictionDebugInfo, Zeta, ZetaOptions};

use edit_prediction_context::{DeclarationStyle, EditPredictionExcerptOptions};

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

// TODO show included diagnostics, and events

pub struct Zeta2Inspector {
    focus_handle: FocusHandle,
    project: Entity<Project>,
    last_prediction: Option<LastPrediction>,
    max_excerpt_bytes_input: Entity<SingleLineInput>,
    min_excerpt_bytes_input: Entity<SingleLineInput>,
    cursor_context_ratio_input: Entity<SingleLineInput>,
    max_prompt_bytes_input: Entity<SingleLineInput>,
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

struct LastPrediction {
    context_editor: Entity<Editor>,
    prompt_editor: Entity<Editor>,
    retrieval_time: TimeDelta,
    buffer: WeakEntity<Buffer>,
    position: language::Anchor,
    state: LastPredictionState,
    _task: Option<Task<()>>,
}

enum LastPredictionState {
    Requested,
    Success {
        inference_time: TimeDelta,
        parsing_time: TimeDelta,
        prompt_planning_time: TimeDelta,
        model_response_editor: Entity<Editor>,
    },
    Failed {
        message: String,
    },
}

impl Zeta2Inspector {
    pub fn new(
        project: &Entity<Project>,
        client: &Arc<Client>,
        user_store: &Entity<UserStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let zeta = Zeta::global(client, user_store, cx);
        let mut request_rx = zeta.update(cx, |zeta, _cx| zeta.debug_info());

        let receive_task = cx.spawn_in(window, async move |this, cx| {
            while let Some(prediction) = request_rx.next().await {
                this.update_in(cx, |this, window, cx| {
                    this.update_last_prediction(prediction, window, cx)
                })
                .ok();
            }
        });

        let mut this = Self {
            focus_handle: cx.focus_handle(),
            project: project.clone(),
            last_prediction: None,
            active_view: ActiveView::Context,
            max_excerpt_bytes_input: Self::number_input("Max Excerpt Bytes", window, cx),
            min_excerpt_bytes_input: Self::number_input("Min Excerpt Bytes", window, cx),
            cursor_context_ratio_input: Self::number_input("Cursor Context Ratio", window, cx),
            max_prompt_bytes_input: Self::number_input("Max Prompt Bytes", window, cx),
            zeta: zeta.clone(),
            _active_editor_subscription: None,
            _update_state_task: Task::ready(()),
            _receive_task: receive_task,
        };
        this.set_input_options(&zeta.read(cx).options().clone(), window, cx);
        this
    }

    fn set_input_options(
        &mut self,
        options: &ZetaOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.max_excerpt_bytes_input.update(cx, |input, cx| {
            input.set_text(options.context.excerpt.max_bytes.to_string(), window, cx);
        });
        self.min_excerpt_bytes_input.update(cx, |input, cx| {
            input.set_text(options.context.excerpt.min_bytes.to_string(), window, cx);
        });
        self.cursor_context_ratio_input.update(cx, |input, cx| {
            input.set_text(
                format!(
                    "{:.2}",
                    options
                        .context
                        .excerpt
                        .target_before_cursor_over_total_bytes
                ),
                window,
                cx,
            );
        });
        self.max_prompt_bytes_input.update(cx, |input, cx| {
            input.set_text(options.max_prompt_bytes.to_string(), window, cx);
        });
        cx.notify();
    }

    fn set_options(&mut self, options: ZetaOptions, cx: &mut Context<Self>) {
        self.zeta.update(cx, |this, _cx| this.set_options(options));

        const THROTTLE_TIME: Duration = Duration::from_millis(100);

        if let Some(prediction) = self.last_prediction.as_mut() {
            if let Some(buffer) = prediction.buffer.upgrade() {
                let position = prediction.position;
                let zeta = self.zeta.clone();
                let project = self.project.clone();
                prediction._task = Some(cx.spawn(async move |_this, cx| {
                    cx.background_executor().timer(THROTTLE_TIME).await;
                    if let Some(task) = zeta
                        .update(cx, |zeta, cx| {
                            zeta.refresh_prediction(&project, &buffer, position, cx)
                        })
                        .ok()
                    {
                        task.await.log_err();
                    }
                }));
                prediction.state = LastPredictionState::Requested;
            } else {
                self.last_prediction.take();
            }
        }

        cx.notify();
    }

    fn number_input(
        label: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<SingleLineInput> {
        let input = cx.new(|cx| {
            SingleLineInput::new(window, cx, "")
                .label(label)
                .label_min_width(px(64.))
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

                let mut context_options = DEFAULT_CONTEXT_OPTIONS.clone();
                context_options.excerpt = EditPredictionExcerptOptions {
                    max_bytes: number_input_value(&this.max_excerpt_bytes_input, cx),
                    min_bytes: number_input_value(&this.min_excerpt_bytes_input, cx),
                    target_before_cursor_over_total_bytes: number_input_value(
                        &this.cursor_context_ratio_input,
                        cx,
                    ),
                };

                let zeta_options = this.zeta.read(cx).options();
                this.set_options(
                    ZetaOptions {
                        context: context_options,
                        max_prompt_bytes: number_input_value(&this.max_prompt_bytes_input, cx),
                        max_diagnostic_bytes: zeta_options.max_diagnostic_bytes,
                        prompt_format: zeta_options.prompt_format,
                        file_indexing_parallelism: zeta_options.file_indexing_parallelism,
                    },
                    cx,
                );
            },
        )
        .detach();
        input
    }

    fn update_last_prediction(
        &mut self,
        prediction: zeta2::PredictionDebugInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let project = self.project.read(cx);
        let path_style = project.path_style(cx);
        let Some(worktree_id) = project
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
                    .declarations
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
                        let mut excerpt_score_components = HashMap::default();

                        let multibuffer = cx.new(|cx| {
                            let mut multibuffer = MultiBuffer::new(language::Capability::ReadOnly);
                            let excerpt_file = Arc::new(ExcerptMetadataFile {
                                title: RelPath::unix("Cursor Excerpt").unwrap().into(),
                                path_style,
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

                            let mut declarations = prediction.context.declarations.clone();
                            declarations.sort_unstable_by_key(|declaration| {
                                Reverse(OrderedFloat(
                                    declaration.score(DeclarationStyle::Declaration),
                                ))
                            });

                            for snippet in &declarations {
                                let path = this
                                    .project
                                    .read(cx)
                                    .path_for_entry(snippet.declaration.project_entry_id(), cx);

                                let snippet_file = Arc::new(ExcerptMetadataFile {
                                    title: RelPath::unix(&format!(
                                        "{} (Score: {})",
                                        path.map(|p| p.path.display(path_style).to_string())
                                            .unwrap_or_else(|| "".to_string()),
                                        snippet.score(DeclarationStyle::Declaration)
                                    ))
                                    .unwrap()
                                    .into(),
                                    path_style,
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

                                let excerpt_ids = multibuffer.push_excerpts(
                                    excerpt_buffer,
                                    [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
                                    cx,
                                );
                                let excerpt_id = excerpt_ids.first().unwrap();

                                excerpt_score_components
                                    .insert(*excerpt_id, snippet.components.clone());
                            }

                            multibuffer
                        });

                        let mut editor =
                            Editor::new(EditorMode::full(), multibuffer, None, window, cx);
                        editor.register_addon(ZetaContextAddon {
                            excerpt_score_components,
                        });
                        editor
                    });

                    let PredictionDebugInfo {
                        response_rx,
                        position,
                        buffer,
                        retrieval_time,
                        local_prompt,
                        ..
                    } = prediction;

                    let task = cx.spawn_in(window, {
                        let markdown_language = markdown_language.clone();
                        async move |this, cx| {
                            let response = response_rx.await;

                            this.update_in(cx, |this, window, cx| {
                                if let Some(prediction) = this.last_prediction.as_mut() {
                                    prediction.state = match response {
                                        Ok(Ok(response)) => {
                                            prediction.prompt_editor.update(
                                                cx,
                                                |prompt_editor, cx| {
                                                    prompt_editor.set_text(
                                                        response.prompt,
                                                        window,
                                                        cx,
                                                    );
                                                },
                                            );

                                            LastPredictionState::Success {
                                                prompt_planning_time: response.prompt_planning_time,
                                                inference_time: response.inference_time,
                                                parsing_time: response.parsing_time,
                                                model_response_editor: cx.new(|cx| {
                                                    let buffer = cx.new(|cx| {
                                                        let mut buffer = Buffer::local(
                                                            response.model_response,
                                                            cx,
                                                        );
                                                        buffer.set_language(markdown_language, cx);
                                                        buffer
                                                    });
                                                    let buffer = cx.new(|cx| {
                                                        MultiBuffer::singleton(buffer, cx)
                                                    });
                                                    let mut editor = Editor::new(
                                                        EditorMode::full(),
                                                        buffer,
                                                        None,
                                                        window,
                                                        cx,
                                                    );
                                                    editor.set_read_only(true);
                                                    editor.set_show_line_numbers(false, cx);
                                                    editor.set_show_gutter(false, cx);
                                                    editor.set_show_scrollbars(false, cx);
                                                    editor
                                                }),
                                            }
                                        }
                                        Ok(Err(err)) => {
                                            LastPredictionState::Failed { message: err }
                                        }
                                        Err(oneshot::Canceled) => LastPredictionState::Failed {
                                            message: "Canceled".to_string(),
                                        },
                                    };
                                }
                            })
                            .ok();
                        }
                    });

                    this.last_prediction = Some(LastPrediction {
                        context_editor,
                        prompt_editor: cx.new(|cx| {
                            let buffer = cx.new(|cx| {
                                let mut buffer =
                                    Buffer::local(local_prompt.unwrap_or_else(|err| err), cx);
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
                        retrieval_time,
                        buffer,
                        position,
                        state: LastPredictionState::Requested,
                        _task: Some(task),
                    });
                    cx.notify();
                })
                .ok();
            }
        });
    }

    fn render_options(&self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .child(Headline::new("Options").size(HeadlineSize::Small))
                    .justify_between()
                    .child(
                        ui::Button::new("reset-options", "Reset")
                            .disabled(self.zeta.read(cx).options() == &zeta2::DEFAULT_OPTIONS)
                            .style(ButtonStyle::Outlined)
                            .size(ButtonSize::Large)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.set_input_options(&zeta2::DEFAULT_OPTIONS, window, cx);
                            })),
                    ),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_end()
                            .child(self.max_excerpt_bytes_input.clone())
                            .child(self.min_excerpt_bytes_input.clone())
                            .child(self.cursor_context_ratio_input.clone()),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .items_end()
                            .child(self.max_prompt_bytes_input.clone())
                            .child(self.render_prompt_format_dropdown(window, cx)),
                    ),
            )
    }

    fn render_prompt_format_dropdown(&self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        let active_format = self.zeta.read(cx).options().prompt_format;
        let this = cx.weak_entity();

        v_flex()
            .gap_1p5()
            .child(
                Label::new("Prompt Format")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                DropdownMenu::new(
                    "ep-prompt-format",
                    active_format.to_string(),
                    ContextMenu::build(window, cx, move |mut menu, _window, _cx| {
                        for prompt_format in PromptFormat::iter() {
                            menu = menu.item(
                                ContextMenuEntry::new(prompt_format.to_string())
                                    .toggleable(IconPosition::End, active_format == prompt_format)
                                    .handler({
                                        let this = this.clone();
                                        move |_window, cx| {
                                            this.update(cx, |this, cx| {
                                                let current_options =
                                                    this.zeta.read(cx).options().clone();
                                                let options = ZetaOptions {
                                                    prompt_format,
                                                    ..current_options
                                                };
                                                this.set_options(options, cx);
                                            })
                                            .ok();
                                        }
                                    }),
                            )
                        }
                        menu
                    }),
                )
                .style(ui::DropdownStyle::Outlined),
            )
    }

    fn render_tabs(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if self.last_prediction.is_none() {
            return None;
        };

        Some(
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
            .selected_index(if self.active_view == ActiveView::Context {
                0
            } else {
                1
            })
            .into_any_element(),
        )
    }

    fn render_stats(&self) -> Option<Div> {
        let Some(prediction) = self.last_prediction.as_ref() else {
            return None;
        };

        let (prompt_planning_time, inference_time, parsing_time) = match &prediction.state {
            LastPredictionState::Success {
                inference_time,
                parsing_time,
                prompt_planning_time,
                ..
            } => (
                Some(*prompt_planning_time),
                Some(*inference_time),
                Some(*parsing_time),
            ),
            LastPredictionState::Requested | LastPredictionState::Failed { .. } => {
                (None, None, None)
            }
        };

        Some(
            v_flex()
                .p_4()
                .gap_2()
                .min_w(px(160.))
                .child(Headline::new("Stats").size(HeadlineSize::Small))
                .child(Self::render_duration(
                    "Context retrieval",
                    Some(prediction.retrieval_time),
                ))
                .child(Self::render_duration(
                    "Prompt planning",
                    prompt_planning_time,
                ))
                .child(Self::render_duration("Inference", inference_time))
                .child(Self::render_duration("Parsing", parsing_time)),
        )
    }

    fn render_duration(name: &'static str, time: Option<chrono::TimeDelta>) -> Div {
        h_flex()
            .gap_1()
            .child(Label::new(name).color(Color::Muted).size(LabelSize::Small))
            .child(match time {
                Some(time) => Label::new(if time.num_microseconds().unwrap_or(0) >= 1000 {
                    format!("{} ms", time.num_milliseconds())
                } else {
                    format!("{} µs", time.num_microseconds().unwrap_or(0))
                })
                .size(LabelSize::Small),
                None => Label::new("...").size(LabelSize::Small),
            })
    }

    fn render_content(&self, cx: &mut Context<Self>) -> AnyElement {
        match self.last_prediction.as_ref() {
            None => v_flex()
                .size_full()
                .justify_center()
                .items_center()
                .child(Label::new("No prediction").size(LabelSize::Large))
                .into_any(),
            Some(prediction) => self.render_last_prediction(prediction, cx).into_any(),
        }
    }

    fn render_last_prediction(&self, prediction: &LastPrediction, cx: &mut Context<Self>) -> Div {
        match &self.active_view {
            ActiveView::Context => div().size_full().child(prediction.context_editor.clone()),
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
                        .child(
                            h_flex()
                                .justify_between()
                                .child(ui::Headline::new("Prompt").size(ui::HeadlineSize::XSmall))
                                .child(match prediction.state {
                                    LastPredictionState::Requested
                                    | LastPredictionState::Failed { .. } => ui::Chip::new("Local")
                                        .bg_color(cx.theme().status().warning_background)
                                        .label_color(Color::Success),
                                    LastPredictionState::Success { .. } => ui::Chip::new("Cloud")
                                        .bg_color(cx.theme().status().success_background)
                                        .label_color(Color::Success),
                                }),
                        )
                        .child(prediction.prompt_editor.clone()),
                )
                .child(ui::vertical_divider())
                .child(
                    v_flex()
                        .flex_1()
                        .gap_2()
                        .h_full()
                        .p_4()
                        .child(ui::Headline::new("Model Response").size(ui::HeadlineSize::XSmall))
                        .child(match &prediction.state {
                            LastPredictionState::Success {
                                model_response_editor,
                                ..
                            } => model_response_editor.clone().into_any_element(),
                            LastPredictionState::Requested => v_flex()
                                .p_4()
                                .gap_2()
                                .child(Label::new("Loading...").buffer_font(cx))
                                .into_any(),
                            LastPredictionState::Failed { message } => v_flex()
                                .p_4()
                                .gap_2()
                                .child(Label::new(message.clone()).buffer_font(cx))
                                .into_any(),
                        }),
                ),
        }
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                            .child(self.render_options(window, cx))
                            .gap_4()
                            .children(self.render_tabs(cx)),
                    )
                    .child(ui::vertical_divider())
                    .children(self.render_stats()),
            )
            .child(self.render_content(cx))
    }
}

// Using same approach as commit view

struct ExcerptMetadataFile {
    title: Arc<RelPath>,
    worktree_id: WorktreeId,
    path_style: PathStyle,
}

impl language::File for ExcerptMetadataFile {
    fn as_local(&self) -> Option<&dyn language::LocalFile> {
        None
    }

    fn disk_state(&self) -> DiskState {
        DiskState::New
    }

    fn path(&self) -> &Arc<RelPath> {
        &self.title
    }

    fn full_path(&self, _: &App) -> PathBuf {
        self.title.as_std_path().to_path_buf()
    }

    fn file_name<'a>(&'a self, _: &'a App) -> &'a str {
        self.title.file_name().unwrap()
    }

    fn path_style(&self, _: &App) -> PathStyle {
        self.path_style
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

struct ZetaContextAddon {
    excerpt_score_components: HashMap<editor::ExcerptId, DeclarationScoreComponents>,
}

impl editor::Addon for ZetaContextAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn render_buffer_header_controls(
        &self,
        excerpt_info: &multi_buffer::ExcerptInfo,
        _window: &Window,
        _cx: &App,
    ) -> Option<AnyElement> {
        let score_components = self.excerpt_score_components.get(&excerpt_info.id)?.clone();

        Some(
            div()
                .id(excerpt_info.id.to_proto() as usize)
                .child(ui::Icon::new(IconName::Info))
                .cursor(CursorStyle::PointingHand)
                .tooltip(move |_, cx| {
                    cx.new(|_| ScoreComponentsTooltip::new(&score_components))
                        .into()
                })
                .into_any(),
        )
    }
}

struct ScoreComponentsTooltip {
    text: SharedString,
}

impl ScoreComponentsTooltip {
    fn new(components: &DeclarationScoreComponents) -> Self {
        Self {
            text: format!("{:#?}", components).into(),
        }
    }
}

impl Render for ScoreComponentsTooltip {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().pl_2().pt_2p5().child(
            div()
                .elevation_2(cx)
                .py_1()
                .px_2()
                .child(ui::Label::new(self.text.clone()).buffer_font(cx)),
        )
    }
}
