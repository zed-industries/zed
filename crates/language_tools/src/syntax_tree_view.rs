use editor::{scroll::autoscroll::Autoscroll, Anchor, Editor, ExcerptId};
use gpui::{
    actions,
    elements::{Empty, Label, MouseEventHandler, ScrollTarget, UniformList, UniformListState},
    fonts::TextStyle,
    platform::MouseButton,
    AppContext, Element, Entity, ModelHandle, View, ViewContext, ViewHandle,
};
use language::{Buffer, OwnedSyntaxLayerInfo};
use std::ops::Range;
use theme::ThemeSettings;
use workspace::{
    item::{Item, ItemHandle},
    Workspace,
};

actions!(log, [OpenSyntaxTreeView]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(
        move |workspace: &mut Workspace, _: &OpenSyntaxTreeView, cx: _| {
            let syntax_tree_view = cx.add_view(|cx| SyntaxTreeView::new(workspace, cx));
            workspace.add_item(Box::new(syntax_tree_view), cx);
        },
    );
}

pub struct SyntaxTreeView {
    editor: Option<EditorState>,
    mouse_y: Option<f32>,
    line_height: Option<f32>,
    list_state: UniformListState,
    selected_descendant_ix: Option<usize>,
    hovered_descendant_ix: Option<usize>,
}

struct EditorState {
    editor: ViewHandle<Editor>,
    active_buffer: Option<BufferState>,
    _subscription: gpui::Subscription,
}

struct BufferState {
    buffer: ModelHandle<Buffer>,
    excerpt_id: ExcerptId,
    active_layer: Option<OwnedSyntaxLayerInfo>,
}

impl SyntaxTreeView {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let mut this = Self {
            list_state: UniformListState::default(),
            editor: None,
            mouse_y: None,
            line_height: None,
            hovered_descendant_ix: None,
            selected_descendant_ix: None,
        };

        this.workspace_updated(workspace.active_item(cx), cx);
        cx.observe(
            &workspace.weak_handle().upgrade(cx).unwrap(),
            |this, workspace, cx| {
                this.workspace_updated(workspace.read(cx).active_item(cx), cx);
            },
        )
        .detach();

        this
    }

    fn workspace_updated(
        &mut self,
        active_item: Option<Box<dyn ItemHandle>>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(item) = active_item {
            if item.id() != cx.view_id() {
                if let Some(editor) = item.act_as::<Editor>(cx) {
                    self.set_editor(editor, cx);
                }
            }
        }
    }

    fn set_editor(&mut self, editor: ViewHandle<Editor>, cx: &mut ViewContext<Self>) {
        if let Some(state) = &self.editor {
            if state.editor == editor {
                return;
            }
            editor.update(cx, |editor, cx| {
                editor.clear_background_highlights::<Self>(cx)
            });
        }

        let subscription = cx.subscribe(&editor, |this, _, event, cx| {
            let reset_layer = match event {
                editor::Event::Reparsed => true,
                editor::Event::SelectionsChanged { .. } => false,
                _ => return,
            };
            this.editor_updated(reset_layer, cx);
        });

        self.editor = Some(EditorState {
            editor,
            _subscription: subscription,
            active_buffer: None,
        });
        self.editor_updated(true, cx);
    }

    fn editor_updated(&mut self, reset_layer: bool, cx: &mut ViewContext<Self>) -> Option<()> {
        // Find which excerpt the cursor is in, and the position within that excerpted buffer.
        let editor_state = self.editor.as_mut()?;
        let editor = &editor_state.editor.read(cx);
        let selection_range = editor.selections.last::<usize>(cx).range();
        let multibuffer = editor.buffer().read(cx);
        let (buffer, range, excerpt_id) = multibuffer
            .range_to_buffer_ranges(selection_range, cx)
            .pop()?;

        // If the cursor has moved into a different excerpt, retrieve a new syntax layer
        // from that buffer.
        let buffer_state = editor_state
            .active_buffer
            .get_or_insert_with(|| BufferState {
                buffer: buffer.clone(),
                excerpt_id,
                active_layer: None,
            });
        if reset_layer
            || buffer_state.buffer != buffer
            || buffer_state.excerpt_id != buffer_state.excerpt_id
        {
            buffer_state.buffer = buffer.clone();
            buffer_state.excerpt_id = excerpt_id;
            buffer_state.active_layer = None;
        }

        // Within the active layer, find the syntax node under the cursor,
        // and scroll to it.
        let layer = match &mut buffer_state.active_layer {
            Some(layer) => layer,
            None => {
                let layer = buffer.read(cx).snapshot().syntax_layer_at(0)?.to_owned();
                buffer_state.active_layer.insert(layer)
            }
        };
        let mut cursor = layer.node().walk();
        while cursor.goto_first_child_for_byte(range.start).is_some() {
            if !range.is_empty() && cursor.node().end_byte() == range.start {
                cursor.goto_next_sibling();
            }
        }

        // Ascend to the smallest ancestor that contains the range.
        loop {
            let node_range = cursor.node().byte_range();
            if node_range.start <= range.start && node_range.end >= range.end {
                break;
            }
            if !cursor.goto_parent() {
                break;
            }
        }

        let descendant_ix = cursor.descendant_index();
        self.selected_descendant_ix = Some(descendant_ix);
        self.list_state.scroll_to(ScrollTarget::Show(descendant_ix));

        cx.notify();
        Some(())
    }

    fn handle_click(&mut self, y: f32, cx: &mut ViewContext<SyntaxTreeView>) -> Option<()> {
        let line_height = self.line_height?;
        let ix = ((self.list_state.scroll_top() + y) / line_height) as usize;

        self.update_editor_with_range_for_descendant_ix(ix, cx, |editor, range, cx| {
            editor.change_selections(Some(Autoscroll::newest()), cx, |selections| {
                selections.select_ranges(vec![range]);
            });
        });
        Some(())
    }

    fn hover_state_changed(&mut self, cx: &mut ViewContext<SyntaxTreeView>) {
        if let Some((y, line_height)) = self.mouse_y.zip(self.line_height) {
            let ix = ((self.list_state.scroll_top() + y) / line_height) as usize;
            if self.hovered_descendant_ix != Some(ix) {
                self.hovered_descendant_ix = Some(ix);
                self.update_editor_with_range_for_descendant_ix(ix, cx, |editor, range, cx| {
                    editor.clear_background_highlights::<Self>(cx);
                    editor.highlight_background::<Self>(
                        vec![range],
                        |theme| theme.editor.document_highlight_write_background,
                        cx,
                    );
                });
                cx.notify();
            }
        }
    }

    fn update_editor_with_range_for_descendant_ix(
        &self,
        descendant_ix: usize,
        cx: &mut ViewContext<Self>,
        mut f: impl FnMut(&mut Editor, Range<Anchor>, &mut ViewContext<Editor>),
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
        let range = multibuffer.anchor_in_excerpt(excerpt_id, range.start)
            ..multibuffer.anchor_in_excerpt(excerpt_id, range.end);

        // Update the editor with the anchor range.
        editor_state.editor.update(cx, |editor, cx| {
            f(editor, range, cx);
        });
        Some(())
    }
}

impl Entity for SyntaxTreeView {
    type Event = ();
}

impl View for SyntaxTreeView {
    fn ui_name() -> &'static str {
        "SyntaxTreeView"
    }

    fn render(&mut self, cx: &mut gpui::ViewContext<'_, '_, Self>) -> gpui::AnyElement<Self> {
        let settings = settings::get::<ThemeSettings>(cx);
        let font_family_id = settings.buffer_font_family;
        let font_family_name = cx.font_cache().family_name(font_family_id).unwrap();
        let font_properties = Default::default();
        let font_id = cx
            .font_cache()
            .select_font(font_family_id, &font_properties)
            .unwrap();
        let font_size = settings.buffer_font_size(cx);

        let editor_theme = settings.theme.editor.clone();
        let style = TextStyle {
            color: editor_theme.text_color,
            font_family_name,
            font_family_id,
            font_id,
            font_size,
            font_properties: Default::default(),
            underline: Default::default(),
        };

        let line_height = Some(cx.font_cache().line_height(font_size));
        if line_height != self.line_height {
            self.line_height = line_height;
            self.hover_state_changed(cx);
        }

        if let Some(layer) = self
            .editor
            .as_ref()
            .and_then(|editor| editor.active_buffer.as_ref())
            .and_then(|buffer| buffer.active_layer.as_ref())
        {
            let layer = layer.clone();
            return MouseEventHandler::<Self, Self>::new(0, cx, move |state, cx| {
                let list_hovered = state.hovered();
                UniformList::new(
                    self.list_state.clone(),
                    layer.node().descendant_count(),
                    cx,
                    move |this, range, items, _| {
                        let mut cursor = layer.node().walk();
                        let mut descendant_ix = range.start as usize;
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
                                let node = cursor.node();
                                let hovered = Some(descendant_ix) == this.hovered_descendant_ix;
                                let selected = Some(descendant_ix) == this.selected_descendant_ix;
                                items.push(
                                    Label::new(node.kind(), style.clone())
                                        .contained()
                                        .with_background_color(if selected {
                                            editor_theme.selection.selection
                                        } else if hovered && list_hovered {
                                            editor_theme.active_line_background
                                        } else {
                                            Default::default()
                                        })
                                        .with_padding_left(depth as f32 * 18.0)
                                        .into_any(),
                                );
                                descendant_ix += 1;
                                if cursor.goto_first_child() {
                                    depth += 1;
                                } else {
                                    visited_children = true;
                                }
                            }
                        }
                    },
                )
            })
            .on_move(move |event, this, cx| {
                let y = event.position.y() - event.region.origin_y();
                this.mouse_y = Some(y);
                this.hover_state_changed(cx);
            })
            .on_click(MouseButton::Left, move |event, this, cx| {
                let y = event.position.y() - event.region.origin_y();
                this.handle_click(y, cx);
            })
            .contained()
            .with_background_color(editor_theme.background)
            .into_any();
        }

        Empty::new().into_any()
    }
}

impl Item for SyntaxTreeView {
    fn tab_content<V: View>(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        _: &AppContext,
    ) -> gpui::AnyElement<V> {
        Label::new("Syntax Tree", style.label.clone()).into_any()
    }
}
