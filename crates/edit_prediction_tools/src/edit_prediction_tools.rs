use std::{
    collections::hash_map::Entry,
    ffi::OsStr,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use collections::HashMap;
use editor::{Editor, EditorEvent, EditorMode, ExcerptRange, MultiBuffer};
use gpui::{
    Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task, WeakEntity, actions,
    prelude::*,
};
use language::{Buffer, DiskState};
use project::{Project, WorktreeId};
use text::ToPoint;
use ui::prelude::*;
use ui_input::SingleLineInput;
use workspace::{Item, SplitDirection, Workspace};

use edit_prediction_context::{
    EditPredictionContext, EditPredictionExcerptOptions, SnippetStyle, SyntaxIndex,
};

actions!(
    dev,
    [
        /// Opens the language server protocol logs viewer.
        OpenEditPredictionContext
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(move |workspace: &mut Workspace, _, _cx| {
        workspace.register_action(
            move |workspace, _: &OpenEditPredictionContext, window, cx| {
                let workspace_entity = cx.entity();
                let project = workspace.project();
                let active_editor = workspace.active_item_as::<Editor>(cx);
                workspace.split_item(
                    SplitDirection::Right,
                    Box::new(cx.new(|cx| {
                        EditPredictionTools::new(
                            &workspace_entity,
                            &project,
                            active_editor,
                            window,
                            cx,
                        )
                    })),
                    window,
                    cx,
                );
            },
        );
    })
    .detach();
}

pub struct EditPredictionTools {
    focus_handle: FocusHandle,
    project: Entity<Project>,
    last_context: Option<ContextState>,
    max_bytes_input: Entity<SingleLineInput>,
    min_bytes_input: Entity<SingleLineInput>,
    cursor_context_ratio_input: Entity<SingleLineInput>,
    // TODO move to project or provider?
    syntax_index: Entity<SyntaxIndex>,
    last_editor: WeakEntity<Editor>,
    _active_editor_subscription: Option<Subscription>,
    _edit_prediction_context_task: Task<()>,
}

struct ContextState {
    context_editor: Entity<Editor>,
    retrieval_duration: Duration,
}

impl EditPredictionTools {
    pub fn new(
        workspace: &Entity<Workspace>,
        project: &Entity<Project>,
        active_editor: Option<Entity<Editor>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe_in(workspace, window, |this, workspace, event, window, cx| {
            if let workspace::Event::ActiveItemChanged = event {
                if let Some(editor) = workspace.read(cx).active_item_as::<Editor>(cx) {
                    this._active_editor_subscription = Some(cx.subscribe_in(
                        &editor,
                        window,
                        |this, editor, event, window, cx| {
                            if let EditorEvent::SelectionsChanged { .. } = event {
                                this.update_context(editor, window, cx);
                            }
                        },
                    ));
                    this.update_context(&editor, window, cx);
                } else {
                    this._active_editor_subscription = None;
                }
            }
        })
        .detach();
        let syntax_index = cx.new(|cx| SyntaxIndex::new(project, cx));

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
                |this, _, event, window, cx| {
                    if let EditorEvent::BufferEdited = event
                        && let Some(editor) = this.last_editor.upgrade()
                    {
                        this.update_context(&editor, window, cx);
                    }
                },
            )
            .detach();
            input
        };

        let mut this = Self {
            focus_handle: cx.focus_handle(),
            project: project.clone(),
            last_context: None,
            max_bytes_input: number_input("Max Bytes", "512", window, cx),
            min_bytes_input: number_input("Min Bytes", "128", window, cx),
            cursor_context_ratio_input: number_input("Cursor Context Ratio", "0.5", window, cx),
            syntax_index,
            last_editor: WeakEntity::new_invalid(),
            _active_editor_subscription: None,
            _edit_prediction_context_task: Task::ready(()),
        };

        if let Some(editor) = active_editor {
            this.update_context(&editor, window, cx);
        }

        this
    }

    fn update_context(
        &mut self,
        editor: &Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.last_editor = editor.downgrade();

        let editor = editor.read(cx);
        let buffer = editor.buffer().clone();
        let cursor_position = editor.selections.newest_anchor().start;

        let Some(buffer) = buffer.read(cx).buffer_for_anchor(cursor_position, cx) else {
            self.last_context.take();
            return;
        };
        let current_buffer_snapshot = buffer.read(cx).snapshot();
        let cursor_position = cursor_position
            .text_anchor
            .to_point(&current_buffer_snapshot);

        let language = current_buffer_snapshot.language().cloned();
        let Some(worktree_id) = self
            .project
            .read(cx)
            .worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).id())
        else {
            log::error!("Open a worktree to use edit prediction debug view");
            self.last_context.take();
            return;
        };

        self._edit_prediction_context_task = cx.spawn_in(window, {
            let language_registry = self.project.read(cx).languages().clone();
            async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;

                let mut start_time = None;

                let Ok(task) = this.update(cx, |this, cx| {
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

                    start_time = Some(Instant::now());

                    // TODO use global zeta instead
                    EditPredictionContext::gather_context_in_background(
                        cursor_position,
                        current_buffer_snapshot,
                        options,
                        Some(this.syntax_index.clone()),
                        cx,
                    )
                }) else {
                    this.update(cx, |this, _cx| {
                        this.last_context.take();
                    })
                    .ok();
                    return;
                };

                let Some(context) = task.await else {
                    // TODO: Display message
                    this.update(cx, |this, _cx| {
                        this.last_context.take();
                    })
                    .ok();
                    return;
                };
                let retrieval_duration = start_time.unwrap().elapsed();

                let mut languages = HashMap::default();
                for snippet in context.snippets.iter() {
                    let lang_id = snippet.declaration.identifier().language_id;
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
                                let mut buffer = Buffer::local(context.excerpt_text.body, cx);
                                buffer.set_language(language, cx);
                                buffer.file_updated(excerpt_file, cx);
                                buffer
                            });

                            multibuffer.push_excerpts(
                                excerpt_buffer,
                                [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
                                cx,
                            );

                            for snippet in context.snippets {
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

                    this.last_context = Some(ContextState {
                        context_editor,
                        retrieval_duration,
                    });
                    cx.notify();
                })
                .ok();
            }
        });
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
        "Edit Prediction Context Debug View".into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::ZedPredict))
    }
}

impl EventEmitter<()> for EditPredictionTools {}

impl Render for EditPredictionTools {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                            ),
                    )
                    .child(ui::Divider::vertical())
                    .when_some(self.last_context.as_ref(), |this, last_context| {
                        this.child(
                            v_flex()
                                .p_4()
                                .gap_2()
                                .min_w(px(160.))
                                .child(Headline::new("Stats").size(HeadlineSize::Small))
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            Label::new("Time to retrieve")
                                                .color(Color::Muted)
                                                .size(LabelSize::Small),
                                        )
                                        .child(
                                            Label::new(
                                                if last_context.retrieval_duration.as_micros()
                                                    > 1000
                                                {
                                                    format!(
                                                        "{} ms",
                                                        last_context.retrieval_duration.as_millis()
                                                    )
                                                } else {
                                                    format!(
                                                        "{} µs",
                                                        last_context.retrieval_duration.as_micros()
                                                    )
                                                },
                                            )
                                            .size(LabelSize::Small),
                                        ),
                                ),
                        )
                    }),
            )
            .children(self.last_context.as_ref().map(|c| c.context_editor.clone()))
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
