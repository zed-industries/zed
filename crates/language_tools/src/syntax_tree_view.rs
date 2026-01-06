use command_palette_hooks::CommandPaletteFilter;
use editor::{Anchor, Editor, ExcerptId, MultiBufferOffset, SelectionEffects, scroll::Autoscroll};
use gpui::{
    App, AppContext as _, Context, Div, Entity, EntityId, EventEmitter, FocusHandle, Focusable,
    Hsla, InteractiveElement, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent,
    ParentElement, Render, ScrollStrategy, SharedString, Styled, Task, UniformListScrollHandle,
    WeakEntity, Window, actions, div, rems, uniform_list,
};
use language::{Buffer, OwnedSyntaxLayer};
use std::{any::TypeId, mem, ops::Range};
use theme::ActiveTheme;
use tree_sitter::{Node, TreeCursor};
use ui::{
    ButtonCommon, ButtonLike, Clickable, Color, ContextMenu, FluentBuilder as _, IconButton,
    IconName, Label, LabelCommon, LabelSize, PopoverMenu, StyledExt, Tooltip, WithScrollbar,
    h_flex, v_flex,
};
use workspace::{
    Event as WorkspaceEvent, SplitDirection, ToolbarItemEvent, ToolbarItemLocation,
    ToolbarItemView, Workspace,
    item::{Item, ItemHandle},
};

actions!(
    dev,
    [
        /// Opens the syntax tree view for the current file.
        OpenSyntaxTreeView,
    ]
);

actions!(
    syntax_tree_view,
    [
        /// Update the syntax tree view to show the last focused file.
        UseActiveEditor
    ]
);

pub fn init(cx: &mut App) {
    let syntax_tree_actions = [TypeId::of::<UseActiveEditor>()];

    CommandPaletteFilter::update_global(cx, |this, _| {
        this.hide_action_types(&syntax_tree_actions);
    });

    cx.observe_new(move |workspace: &mut Workspace, _, _| {
        workspace.register_action(move |workspace, _: &OpenSyntaxTreeView, window, cx| {
            CommandPaletteFilter::update_global(cx, |this, _| {
                this.show_action_types(&syntax_tree_actions);
            });

            let active_item = workspace.active_item(cx);
            let workspace_handle = workspace.weak_handle();
            let syntax_tree_view = cx.new(|cx| {
                cx.on_release(move |view: &mut SyntaxTreeView, cx| {
                    if view
                        .workspace_handle
                        .read_with(cx, |workspace, cx| {
                            workspace.item_of_type::<SyntaxTreeView>(cx).is_none()
                        })
                        .unwrap_or_default()
                    {
                        CommandPaletteFilter::update_global(cx, |this, _| {
                            this.hide_action_types(&syntax_tree_actions);
                        });
                    }
                })
                .detach();

                SyntaxTreeView::new(workspace_handle, active_item, window, cx)
            });
            workspace.split_item(
                SplitDirection::Right,
                Box::new(syntax_tree_view),
                window,
                cx,
            )
        });
        workspace.register_action(|workspace, _: &UseActiveEditor, window, cx| {
            if let Some(tree_view) = workspace.item_of_type::<SyntaxTreeView>(cx) {
                tree_view.update(cx, |view, cx| {
                    view.update_active_editor(&Default::default(), window, cx)
                })
            }
        });
    })
    .detach();
}

pub struct SyntaxTreeView {
    workspace_handle: WeakEntity<Workspace>,
    editor: Option<EditorState>,
    list_scroll_handle: UniformListScrollHandle,
    /// The last active editor in the workspace. Note that this is specifically not the
    /// currently shown editor.
    last_active_editor: Option<Entity<Editor>>,
    selected_descendant_ix: Option<usize>,
    hovered_descendant_ix: Option<usize>,
    focus_handle: FocusHandle,
}

pub struct SyntaxTreeToolbarItemView {
    tree_view: Option<Entity<SyntaxTreeView>>,
    subscription: Option<gpui::Subscription>,
}

struct EditorState {
    editor: Entity<Editor>,
    active_buffer: Option<BufferState>,
    _subscription: gpui::Subscription,
}

impl EditorState {
    fn has_language(&self) -> bool {
        self.active_buffer
            .as_ref()
            .is_some_and(|buffer| buffer.active_layer.is_some())
    }
}

#[derive(Clone)]
struct BufferState {
    buffer: Entity<Buffer>,
    excerpt_id: ExcerptId,
    active_layer: Option<OwnedSyntaxLayer>,
}

impl SyntaxTreeView {
    pub fn new(
        workspace_handle: WeakEntity<Workspace>,
        active_item: Option<Box<dyn ItemHandle>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self {
            workspace_handle: workspace_handle.clone(),
            list_scroll_handle: UniformListScrollHandle::new(),
            editor: None,
            last_active_editor: None,
            hovered_descendant_ix: None,
            selected_descendant_ix: None,
            focus_handle: cx.focus_handle(),
        };

        this.handle_item_updated(active_item, window, cx);

        cx.subscribe_in(
            &workspace_handle.upgrade().unwrap(),
            window,
            move |this, workspace, event, window, cx| match event {
                WorkspaceEvent::ItemAdded { .. } | WorkspaceEvent::ActiveItemChanged => {
                    this.handle_item_updated(workspace.read(cx).active_item(cx), window, cx)
                }
                WorkspaceEvent::ItemRemoved { item_id } => {
                    this.handle_item_removed(item_id, window, cx);
                }
                _ => {}
            },
        )
        .detach();

        this
    }

    fn handle_item_updated(
        &mut self,
        active_item: Option<Box<dyn ItemHandle>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(editor) = active_item
            .filter(|item| item.item_id() != cx.entity_id())
            .and_then(|item| item.act_as::<Editor>(cx))
        else {
            return;
        };

        if let Some(editor_state) = self.editor.as_ref().filter(|state| state.has_language()) {
            self.last_active_editor = (editor_state.editor != editor).then_some(editor);
        } else {
            self.set_editor(editor, window, cx);
        }
    }

    fn handle_item_removed(
        &mut self,
        item_id: &EntityId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self
            .editor
            .as_ref()
            .is_some_and(|state| state.editor.entity_id() == *item_id)
        {
            self.editor = None;
            // Try activating the last active editor if there is one
            self.update_active_editor(&Default::default(), window, cx);
            cx.notify();
        }
    }

    fn update_active_editor(
        &mut self,
        _: &UseActiveEditor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(editor) = self.last_active_editor.take() else {
            return;
        };
        self.set_editor(editor, window, cx);
    }

    fn set_editor(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(state) = &self.editor {
            if state.editor == editor {
                return;
            }
            editor.update(cx, |editor, cx| {
                editor.clear_background_highlights::<Self>(cx)
            });
        }

        let subscription = cx.subscribe_in(&editor, window, |this, _, event, window, cx| {
            let did_reparse = match event {
                editor::EditorEvent::Reparsed(_) => true,
                editor::EditorEvent::SelectionsChanged { .. } => false,
                _ => return,
            };
            this.editor_updated(did_reparse, window, cx);
        });

        self.editor = Some(EditorState {
            editor,
            _subscription: subscription,
            active_buffer: None,
        });
        self.editor_updated(true, window, cx);
    }

    fn editor_updated(
        &mut self,
        did_reparse: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        // Find which excerpt the cursor is in, and the position within that excerpted buffer.
        let editor_state = self.editor.as_mut()?;
        let snapshot = editor_state
            .editor
            .update(cx, |editor, cx| editor.snapshot(window, cx));
        let (buffer, range, excerpt_id) = editor_state.editor.update(cx, |editor, cx| {
            let selection_range = editor
                .selections
                .last::<MultiBufferOffset>(&editor.display_snapshot(cx))
                .range();
            let multi_buffer = editor.buffer().read(cx);
            let (buffer, range, excerpt_id) = snapshot
                .buffer_snapshot()
                .range_to_buffer_ranges(selection_range)
                .pop()?;
            let buffer = multi_buffer.buffer(buffer.remote_id()).unwrap();
            Some((buffer, range, excerpt_id))
        })?;

        // If the cursor has moved into a different excerpt, retrieve a new syntax layer
        // from that buffer.
        let buffer_state = editor_state
            .active_buffer
            .get_or_insert_with(|| BufferState {
                buffer: buffer.clone(),
                excerpt_id,
                active_layer: None,
            });
        let mut prev_layer = None;
        if did_reparse {
            prev_layer = buffer_state.active_layer.take();
        }
        if buffer_state.buffer != buffer || buffer_state.excerpt_id != excerpt_id {
            buffer_state.buffer = buffer.clone();
            buffer_state.excerpt_id = excerpt_id;
            buffer_state.active_layer = None;
        }

        let layer = match &mut buffer_state.active_layer {
            Some(layer) => layer,
            None => {
                let snapshot = buffer.read(cx).snapshot();
                let layer = if let Some(prev_layer) = prev_layer {
                    let prev_range = prev_layer.node().byte_range();
                    snapshot
                        .syntax_layers()
                        .filter(|layer| layer.language == &prev_layer.language)
                        .min_by_key(|layer| {
                            let range = layer.node().byte_range();
                            ((range.start as i64) - (prev_range.start as i64)).abs()
                                + ((range.end as i64) - (prev_range.end as i64)).abs()
                        })?
                } else {
                    snapshot.syntax_layers().next()?
                };
                buffer_state.active_layer.insert(layer.to_owned())
            }
        };

        // Within the active layer, find the syntax node under the cursor,
        // and scroll to it.
        let mut cursor = layer.node().walk();
        while cursor.goto_first_child_for_byte(range.start.0).is_some() {
            if !range.is_empty() && cursor.node().end_byte() == range.start.0 {
                cursor.goto_next_sibling();
            }
        }

        // Ascend to the smallest ancestor that contains the range.
        loop {
            let node_range = cursor.node().byte_range();
            if node_range.start <= range.start.0 && node_range.end >= range.end.0 {
                break;
            }
            if !cursor.goto_parent() {
                break;
            }
        }

        let descendant_ix = cursor.descendant_index();
        self.selected_descendant_ix = Some(descendant_ix);
        self.list_scroll_handle
            .scroll_to_item(descendant_ix, ScrollStrategy::Center);

        cx.notify();
        Some(())
    }

    fn update_editor_with_range_for_descendant_ix(
        &self,
        descendant_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
        mut f: impl FnMut(&mut Editor, Range<Anchor>, &mut Window, &mut Context<Editor>),
    ) -> Option<()> {
        let editor_state = self.editor.as_ref()?;
        let buffer_state = editor_state.active_buffer.as_ref()?;
        let layer = buffer_state.active_layer.as_ref()?;

        // Find the node.
        let mut cursor = layer.node().walk();
        cursor.goto_descendant(descendant_ix);
        let node = cursor.node();
        let range = node.byte_range();

        // Build a text anchor range.
        let buffer = buffer_state.buffer.read(cx);
        let range = buffer.anchor_before(range.start)..buffer.anchor_after(range.end);

        // Build a multibuffer anchor range.
        let multibuffer = editor_state.editor.read(cx).buffer();
        let multibuffer = multibuffer.read(cx).snapshot(cx);
        let excerpt_id = buffer_state.excerpt_id;
        let range = multibuffer.anchor_range_in_excerpt(excerpt_id, range)?;

        // Update the editor with the anchor range.
        editor_state.editor.update(cx, |editor, cx| {
            f(editor, range, window, cx);
        });
        Some(())
    }

    fn render_node(cursor: &TreeCursor, depth: u32, selected: bool, cx: &App) -> Div {
        let colors = cx.theme().colors();
        let mut row = h_flex();
        if let Some(field_name) = cursor.field_name() {
            row = row.children([Label::new(field_name).color(Color::Info), Label::new(": ")]);
        }

        let node = cursor.node();
        row.child(if node.is_named() {
            Label::new(node.kind()).color(Color::Default)
        } else {
            Label::new(format!("\"{}\"", node.kind())).color(Color::Created)
        })
        .child(
            div()
                .child(Label::new(format_node_range(node)).color(Color::Muted))
                .pl_1(),
        )
        .text_bg(if selected {
            colors.element_selected
        } else {
            Hsla::default()
        })
        .pl(rems(depth as f32))
        .hover(|style| style.bg(colors.element_hover))
    }

    fn compute_items(
        &mut self,
        layer: &OwnedSyntaxLayer,
        range: Range<usize>,
        cx: &Context<Self>,
    ) -> Vec<Div> {
        let mut items = Vec::new();
        let mut cursor = layer.node().walk();
        let mut descendant_ix = range.start;
        cursor.goto_descendant(descendant_ix);
        let mut depth = cursor.depth();
        let mut visited_children = false;
        while descendant_ix < range.end {
            if visited_children {
                if cursor.goto_next_sibling() {
                    visited_children = false;
                } else if cursor.goto_parent() {
                    depth -= 1;
                } else {
                    break;
                }
            } else {
                items.push(
                    Self::render_node(
                        &cursor,
                        depth,
                        Some(descendant_ix) == self.selected_descendant_ix,
                        cx,
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |tree_view, _: &MouseDownEvent, window, cx| {
                            tree_view.update_editor_with_range_for_descendant_ix(
                                descendant_ix,
                                window,
                                cx,
                                |editor, mut range, window, cx| {
                                    // Put the cursor at the beginning of the node.
                                    mem::swap(&mut range.start, &mut range.end);

                                    editor.change_selections(
                                        SelectionEffects::scroll(Autoscroll::newest()),
                                        window,
                                        cx,
                                        |selections| {
                                            selections.select_ranges(vec![range]);
                                        },
                                    );
                                },
                            );
                        }),
                    )
                    .on_mouse_move(cx.listener(
                        move |tree_view, _: &MouseMoveEvent, window, cx| {
                            if tree_view.hovered_descendant_ix != Some(descendant_ix) {
                                tree_view.hovered_descendant_ix = Some(descendant_ix);
                                tree_view.update_editor_with_range_for_descendant_ix(
                                    descendant_ix,
                                    window,
                                    cx,
                                    |editor, range, _, cx| {
                                        editor.clear_background_highlights::<Self>(cx);
                                        editor.highlight_background::<Self>(
                                            &[range],
                                            |_, theme| {
                                                theme
                                                    .colors()
                                                    .editor_document_highlight_write_background
                                            },
                                            cx,
                                        );
                                    },
                                );
                                cx.notify();
                            }
                        },
                    )),
                );
                descendant_ix += 1;
                if cursor.goto_first_child() {
                    depth += 1;
                } else {
                    visited_children = true;
                }
            }
        }
        items
    }
}

impl Render for SyntaxTreeView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .bg(cx.theme().colors().editor_background)
            .map(|this| {
                let editor_state = self.editor.as_ref();

                if let Some(layer) = editor_state
                    .and_then(|editor| editor.active_buffer.as_ref())
                    .and_then(|buffer| buffer.active_layer.as_ref())
                {
                    let layer = layer.clone();
                    this.child(
                        uniform_list(
                            "SyntaxTreeView",
                            layer.node().descendant_count(),
                            cx.processor(move |this, range: Range<usize>, _, cx| {
                                this.compute_items(&layer, range, cx)
                            }),
                        )
                        .size_full()
                        .track_scroll(&self.list_scroll_handle)
                        .text_bg(cx.theme().colors().background)
                        .into_any_element(),
                    )
                    .vertical_scrollbar_for(&self.list_scroll_handle, window, cx)
                    .into_any_element()
                } else {
                    let inner_content = v_flex()
                        .items_center()
                        .text_center()
                        .gap_2()
                        .max_w_3_5()
                        .map(|this| {
                            if editor_state.is_some_and(|state| !state.has_language()) {
                                this.child(Label::new("Current editor has no associated language"))
                                    .child(
                                        Label::new(concat!(
                                            "Try assigning a language or",
                                            "switching to a different buffer"
                                        ))
                                        .size(LabelSize::Small),
                                    )
                            } else {
                                this.child(Label::new("Not attached to an editor")).child(
                                    Label::new("Focus an editor to show a new tree view")
                                        .size(LabelSize::Small),
                                )
                            }
                        });

                    this.h_flex()
                        .size_full()
                        .justify_center()
                        .child(inner_content)
                        .into_any_element()
                }
            })
    }
}

impl EventEmitter<()> for SyntaxTreeView {}

impl Focusable for SyntaxTreeView {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for SyntaxTreeView {
    type Event = ();

    fn to_item_events(_: &Self::Event, _: impl FnMut(workspace::item::ItemEvent)) {}

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Syntax Tree".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        Task::ready(Some(cx.new(|cx| {
            let mut clone = Self::new(self.workspace_handle.clone(), None, window, cx);
            if let Some(editor) = &self.editor {
                clone.set_editor(editor.editor.clone(), window, cx)
            }
            clone
        })))
    }
}

impl Default for SyntaxTreeToolbarItemView {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntaxTreeToolbarItemView {
    pub fn new() -> Self {
        Self {
            tree_view: None,
            subscription: None,
        }
    }

    fn render_menu(&mut self, cx: &mut Context<Self>) -> Option<PopoverMenu<ContextMenu>> {
        let tree_view = self.tree_view.as_ref()?;
        let tree_view = tree_view.read(cx);

        let editor_state = tree_view.editor.as_ref()?;
        let buffer_state = editor_state.active_buffer.as_ref()?;
        let active_layer = buffer_state.active_layer.clone()?;
        let active_buffer = buffer_state.buffer.read(cx).snapshot();

        let view = cx.weak_entity();
        Some(
            PopoverMenu::new("Syntax Tree")
                .trigger(Self::render_header(&active_layer))
                .menu(move |window, cx| {
                    ContextMenu::build(window, cx, |mut menu, _, _| {
                        for (layer_ix, layer) in active_buffer.syntax_layers().enumerate() {
                            let view = view.clone();
                            menu = menu.entry(
                                format!(
                                    "{} {}",
                                    layer.language.name(),
                                    format_node_range(layer.node())
                                ),
                                None,
                                move |window, cx| {
                                    view.update(cx, |view, cx| {
                                        view.select_layer(layer_ix, window, cx);
                                    })
                                    .ok();
                                },
                            );
                        }
                        menu
                    })
                    .into()
                }),
        )
    }

    fn select_layer(
        &mut self,
        layer_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let tree_view = self.tree_view.as_ref()?;
        tree_view.update(cx, |view, cx| {
            let editor_state = view.editor.as_mut()?;
            let buffer_state = editor_state.active_buffer.as_mut()?;
            let snapshot = buffer_state.buffer.read(cx).snapshot();
            let layer = snapshot.syntax_layers().nth(layer_ix)?;
            buffer_state.active_layer = Some(layer.to_owned());
            view.selected_descendant_ix = None;
            cx.notify();
            view.focus_handle.focus(window, cx);
            Some(())
        })
    }

    fn render_header(active_layer: &OwnedSyntaxLayer) -> ButtonLike {
        ButtonLike::new("syntax tree header")
            .child(Label::new(active_layer.language.name()))
            .child(Label::new(format_node_range(active_layer.node())))
    }

    fn render_update_button(&mut self, cx: &mut Context<Self>) -> Option<IconButton> {
        self.tree_view.as_ref().and_then(|view| {
            view.update(cx, |view, cx| {
                view.last_active_editor.as_ref().map(|editor| {
                    IconButton::new("syntax-view-update", IconName::RotateCw)
                        .tooltip({
                            let active_tab_name = editor.read_with(cx, |editor, cx| {
                                editor.tab_content_text(Default::default(), cx)
                            });

                            Tooltip::text(format!("Update view to '{active_tab_name}'"))
                        })
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.update_active_editor(&Default::default(), window, cx);
                        }))
                })
            })
        })
    }
}

fn format_node_range(node: Node) -> String {
    let start = node.start_position();
    let end = node.end_position();
    format!(
        "[{}:{} - {}:{}]",
        start.row + 1,
        start.column + 1,
        end.row + 1,
        end.column + 1,
    )
}

impl Render for SyntaxTreeToolbarItemView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_1()
            .children(self.render_menu(cx))
            .children(self.render_update_button(cx))
    }
}

impl EventEmitter<ToolbarItemEvent> for SyntaxTreeToolbarItemView {}

impl ToolbarItemView for SyntaxTreeToolbarItemView {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        if let Some(item) = active_pane_item
            && let Some(view) = item.downcast::<SyntaxTreeView>()
        {
            self.tree_view = Some(view.clone());
            self.subscription = Some(cx.observe_in(&view, window, |_, _, _, cx| cx.notify()));
            return ToolbarItemLocation::PrimaryLeft;
        }
        self.tree_view = None;
        self.subscription = None;
        ToolbarItemLocation::Hidden
    }
}
