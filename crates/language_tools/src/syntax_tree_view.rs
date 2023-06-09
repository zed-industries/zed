use editor::{scroll::autoscroll::Autoscroll, Anchor, Editor, ExcerptId};
use gpui::{
    actions,
    elements::{Empty, Label, MouseEventHandler, UniformList, UniformListState},
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
    editor: Option<(ViewHandle<Editor>, gpui::Subscription)>,
    buffer: Option<(ModelHandle<Buffer>, usize, ExcerptId)>,
    layer: Option<OwnedSyntaxLayerInfo>,
    hover_y: Option<f32>,
    line_height: Option<f32>,
    list_state: UniformListState,
    active_descendant_ix: Option<usize>,
    highlighted_active_descendant: bool,
}

impl SyntaxTreeView {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let mut this = Self {
            list_state: UniformListState::default(),
            editor: None,
            buffer: None,
            layer: None,
            hover_y: None,
            line_height: None,
            active_descendant_ix: None,
            highlighted_active_descendant: false,
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
        if let Some((current_editor, _)) = &self.editor {
            if current_editor == &editor {
                return;
            }
            editor.update(cx, |editor, cx| {
                editor.clear_background_highlights::<Self>(cx);
            });
        }

        let subscription = cx.subscribe(&editor, |this, editor, event, cx| {
            let selection_changed = match event {
                editor::Event::Reparsed => false,
                editor::Event::SelectionsChanged { .. } => true,
                _ => return,
            };
            this.editor_updated(&editor, selection_changed, cx);
        });

        self.editor_updated(&editor, true, cx);
        self.editor = Some((editor, subscription));
    }

    fn editor_updated(
        &mut self,
        editor: &ViewHandle<Editor>,
        selection_changed: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let editor = editor.read(cx);
        if selection_changed {
            let cursor = editor.selections.last::<usize>(cx).end;
            self.buffer = editor.buffer().read(cx).point_to_buffer_offset(cursor, cx);
            self.layer = self.buffer.as_ref().and_then(|(buffer, offset, _)| {
                buffer
                    .read(cx)
                    .snapshot()
                    .syntax_layer_at(*offset)
                    .map(|l| l.to_owned())
            });
        }
        cx.notify();
    }

    fn hover_state_changed(&mut self, cx: &mut ViewContext<SyntaxTreeView>) {
        if let Some((y, line_height)) = self.hover_y.zip(self.line_height) {
            let ix = ((self.list_state.scroll_top() + y) / line_height) as usize;
            if self.active_descendant_ix != Some(ix) {
                self.active_descendant_ix = Some(ix);
                self.highlighted_active_descendant = false;
                cx.notify();
            }
        }
    }

    fn handle_click(&mut self, y: f32, cx: &mut ViewContext<SyntaxTreeView>) {
        if let Some(line_height) = self.line_height {
            let ix = ((self.list_state.scroll_top() + y) / line_height) as usize;
            if let Some(layer) = &self.layer {
                let mut cursor = layer.node().walk();
                cursor.goto_descendant(ix);
                let node = cursor.node();
                self.update_editor_with_node_range(node, cx, |editor, range, cx| {
                    editor.change_selections(Some(Autoscroll::newest()), cx, |selections| {
                        selections.select_ranges(vec![range]);
                    });
                });
            }
        }
    }

    fn update_editor_with_node_range(
        &self,
        node: tree_sitter::Node,
        cx: &mut ViewContext<Self>,
        mut f: impl FnMut(&mut Editor, Range<Anchor>, &mut ViewContext<Editor>),
    ) {
        let range = node.byte_range();
        if let Some((editor, _)) = &self.editor {
            if let Some((buffer, _, excerpt_id)) = &self.buffer {
                let buffer = &buffer.read(cx);
                let multibuffer = editor.read(cx).buffer();
                let multibuffer = multibuffer.read(cx).snapshot(cx);
                let start =
                    multibuffer.anchor_in_excerpt(*excerpt_id, buffer.anchor_before(range.start));
                let end =
                    multibuffer.anchor_in_excerpt(*excerpt_id, buffer.anchor_after(range.end));
                editor.update(cx, |editor, cx| {
                    f(editor, start..end, cx);
                });
            }
        }
    }

    fn node_is_active(&mut self, node: tree_sitter::Node, cx: &mut ViewContext<Self>) {
        if self.highlighted_active_descendant {
            return;
        }
        self.highlighted_active_descendant = true;
        self.update_editor_with_node_range(node, cx, |editor, range, cx| {
            editor.clear_background_highlights::<Self>(cx);
            editor.highlight_background::<Self>(
                vec![range],
                |theme| theme.editor.document_highlight_write_background,
                cx,
            );
        });
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
        self.line_height = Some(cx.font_cache().line_height(font_size));

        self.hover_state_changed(cx);

        if let Some(layer) = &self.layer {
            let layer = layer.clone();
            return MouseEventHandler::<Self, Self>::new(0, cx, move |_, cx| {
                UniformList::new(
                    self.list_state.clone(),
                    layer.node().descendant_count(),
                    cx,
                    move |this, range, items, cx| {
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
                                let is_hovered = Some(descendant_ix) == this.active_descendant_ix;
                                if is_hovered {
                                    this.node_is_active(node, cx);
                                }
                                items.push(
                                    Label::new(node.kind(), style.clone())
                                        .contained()
                                        .with_background_color(if is_hovered {
                                            editor_theme.active_line_background
                                        } else {
                                            Default::default()
                                        })
                                        .with_padding_left(depth as f32 * 10.0)
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
                this.hover_y = Some(y);
                this.hover_state_changed(cx);
            })
            .on_click(MouseButton::Left, move |event, this, cx| {
                let y = event.position.y() - event.region.origin_y();
                this.handle_click(y, cx);
            })
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
