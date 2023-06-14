use editor::{scroll::autoscroll::Autoscroll, Anchor, Editor, ExcerptId};
use gpui::{
    actions,
    elements::{
        AnchorCorner, Empty, Flex, Label, MouseEventHandler, Overlay, OverlayFitMode,
        ParentElement, ScrollTarget, Stack, UniformList, UniformListState,
    },
    fonts::TextStyle,
    platform::{CursorStyle, MouseButton},
    AppContext, Element, Entity, ModelHandle, View, ViewContext, ViewHandle, WeakViewHandle,
};
use language::{Buffer, OwnedSyntaxLayerInfo, SyntaxLayerInfo};
use std::{mem, ops::Range, sync::Arc};
use theme::{Theme, ThemeSettings};
use tree_sitter::{Node, TreeCursor};
use workspace::{
    item::{Item, ItemHandle},
    ToolbarItemLocation, ToolbarItemView, Workspace,
};

actions!(debug, [OpenSyntaxTreeView]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(
        move |workspace: &mut Workspace, _: &OpenSyntaxTreeView, cx: _| {
            let active_item = workspace.active_item(cx);
            let workspace_handle = workspace.weak_handle();
            let syntax_tree_view =
                cx.add_view(|cx| SyntaxTreeView::new(workspace_handle, active_item, cx));
            workspace.add_item(Box::new(syntax_tree_view), cx);
        },
    );
}

pub struct SyntaxTreeView {
    workspace_handle: WeakViewHandle<Workspace>,
    editor: Option<EditorState>,
    mouse_y: Option<f32>,
    line_height: Option<f32>,
    list_state: UniformListState,
    selected_descendant_ix: Option<usize>,
    hovered_descendant_ix: Option<usize>,
}

pub struct SyntaxTreeToolbarItemView {
    tree_view: Option<ViewHandle<SyntaxTreeView>>,
    subscription: Option<gpui::Subscription>,
    menu_open: bool,
}

struct EditorState {
    editor: ViewHandle<Editor>,
    active_buffer: Option<BufferState>,
    _subscription: gpui::Subscription,
}

#[derive(Clone)]
struct BufferState {
    buffer: ModelHandle<Buffer>,
    excerpt_id: ExcerptId,
    active_layer: Option<OwnedSyntaxLayerInfo>,
}

impl SyntaxTreeView {
    pub fn new(
        workspace_handle: WeakViewHandle<Workspace>,
        active_item: Option<Box<dyn ItemHandle>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut this = Self {
            workspace_handle: workspace_handle.clone(),
            list_state: UniformListState::default(),
            editor: None,
            mouse_y: None,
            line_height: None,
            hovered_descendant_ix: None,
            selected_descendant_ix: None,
        };

        this.workspace_updated(active_item, cx);
        cx.observe(
            &workspace_handle.upgrade(cx).unwrap(),
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
            let did_reparse = match event {
                editor::Event::Reparsed => true,
                editor::Event::SelectionsChanged { .. } => false,
                _ => return,
            };
            this.editor_updated(did_reparse, cx);
        });

        self.editor = Some(EditorState {
            editor,
            _subscription: subscription,
            active_buffer: None,
        });
        self.editor_updated(true, cx);
    }

    fn editor_updated(&mut self, did_reparse: bool, cx: &mut ViewContext<Self>) -> Option<()> {
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
        let mut prev_layer = None;
        if did_reparse {
            prev_layer = buffer_state.active_layer.take();
        }
        if buffer_state.buffer != buffer || buffer_state.excerpt_id != buffer_state.excerpt_id {
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

        self.update_editor_with_range_for_descendant_ix(ix, cx, |editor, mut range, cx| {
            // Put the cursor at the beginning of the node.
            mem::swap(&mut range.start, &mut range.end);

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

    fn render_node(
        cursor: &TreeCursor,
        depth: u32,
        selected: bool,
        hovered: bool,
        list_hovered: bool,
        style: &TextStyle,
        editor_theme: &theme::Editor,
        cx: &AppContext,
    ) -> gpui::AnyElement<SyntaxTreeView> {
        let node = cursor.node();
        let mut range_style = style.clone();
        let em_width = style.em_width(cx.font_cache());
        let gutter_padding = (em_width * editor_theme.gutter_padding_factor).round();

        range_style.color = editor_theme.line_number;

        let mut anonymous_node_style = style.clone();
        let string_color = editor_theme
            .syntax
            .highlights
            .iter()
            .find_map(|(name, style)| (name == "string").then(|| style.color)?);
        let property_color = editor_theme
            .syntax
            .highlights
            .iter()
            .find_map(|(name, style)| (name == "property").then(|| style.color)?);
        if let Some(color) = string_color {
            anonymous_node_style.color = color;
        }

        let mut row = Flex::row();
        if let Some(field_name) = cursor.field_name() {
            let mut field_style = style.clone();
            if let Some(color) = property_color {
                field_style.color = color;
            }

            row.add_children([
                Label::new(field_name, field_style),
                Label::new(": ", style.clone()),
            ]);
        }

        return row
            .with_child(
                if node.is_named() {
                    Label::new(node.kind(), style.clone())
                } else {
                    Label::new(format!("\"{}\"", node.kind()), anonymous_node_style)
                }
                .contained()
                .with_margin_right(em_width),
            )
            .with_child(Label::new(format_node_range(node), range_style))
            .contained()
            .with_background_color(if selected {
                editor_theme.selection.selection
            } else if hovered && list_hovered {
                editor_theme.active_line_background
            } else {
                Default::default()
            })
            .with_padding_left(gutter_padding + depth as f32 * 18.0)
            .into_any();
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

        let line_height = cx.font_cache().line_height(font_size);
        if Some(line_height) != self.line_height {
            self.line_height = Some(line_height);
            self.hover_state_changed(cx);
        }

        if let Some(layer) = self
            .editor
            .as_ref()
            .and_then(|editor| editor.active_buffer.as_ref())
            .and_then(|buffer| buffer.active_layer.as_ref())
        {
            let layer = layer.clone();
            let theme = editor_theme.clone();
            return MouseEventHandler::<Self, Self>::new(0, cx, move |state, cx| {
                let list_hovered = state.hovered();
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
                                items.push(Self::render_node(
                                    &cursor,
                                    depth,
                                    Some(descendant_ix) == this.selected_descendant_ix,
                                    Some(descendant_ix) == this.hovered_descendant_ix,
                                    list_hovered,
                                    &style,
                                    &theme,
                                    cx,
                                ));
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

    fn clone_on_split(
        &self,
        _workspace_id: workspace::WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        let mut clone = Self::new(self.workspace_handle.clone(), None, cx);
        if let Some(editor) = &self.editor {
            clone.set_editor(editor.editor.clone(), cx)
        }
        Some(clone)
    }
}

impl SyntaxTreeToolbarItemView {
    pub fn new() -> Self {
        Self {
            menu_open: false,
            tree_view: None,
            subscription: None,
        }
    }

    fn render_menu(
        &mut self,
        cx: &mut ViewContext<'_, '_, Self>,
    ) -> Option<gpui::AnyElement<Self>> {
        let theme = theme::current(cx).clone();
        let tree_view = self.tree_view.as_ref()?;
        let tree_view = tree_view.read(cx);

        let editor_state = tree_view.editor.as_ref()?;
        let buffer_state = editor_state.active_buffer.as_ref()?;
        let active_layer = buffer_state.active_layer.clone()?;
        let active_buffer = buffer_state.buffer.read(cx).snapshot();

        enum Menu {}

        Some(
            Stack::new()
                .with_child(Self::render_header(&theme, &active_layer, cx))
                .with_children(self.menu_open.then(|| {
                    Overlay::new(
                        MouseEventHandler::<Menu, _>::new(0, cx, move |_, cx| {
                            Flex::column()
                                .with_children(active_buffer.syntax_layers().enumerate().map(
                                    |(ix, layer)| {
                                        Self::render_menu_item(&theme, &active_layer, layer, ix, cx)
                                    },
                                ))
                                .contained()
                                .with_style(theme.toolbar_dropdown_menu.container)
                                .constrained()
                                .with_width(400.)
                                .with_height(400.)
                        })
                        .on_down_out(MouseButton::Left, |_, this, cx| {
                            this.menu_open = false;
                            cx.notify()
                        }),
                    )
                    .with_hoverable(true)
                    .with_fit_mode(OverlayFitMode::SwitchAnchor)
                    .with_anchor_corner(AnchorCorner::TopLeft)
                    .with_z_index(999)
                    .aligned()
                    .bottom()
                    .left()
                }))
                .aligned()
                .left()
                .clipped()
                .into_any(),
        )
    }

    fn toggle_menu(&mut self, cx: &mut ViewContext<Self>) {
        self.menu_open = !self.menu_open;
        cx.notify();
    }

    fn select_layer(&mut self, layer_ix: usize, cx: &mut ViewContext<Self>) -> Option<()> {
        let tree_view = self.tree_view.as_ref()?;
        tree_view.update(cx, |view, cx| {
            let editor_state = view.editor.as_mut()?;
            let buffer_state = editor_state.active_buffer.as_mut()?;
            let snapshot = buffer_state.buffer.read(cx).snapshot();
            let layer = snapshot.syntax_layers().nth(layer_ix)?;
            buffer_state.active_layer = Some(layer.to_owned());
            view.selected_descendant_ix = None;
            self.menu_open = false;
            cx.notify();
            Some(())
        })
    }

    fn render_header(
        theme: &Arc<Theme>,
        active_layer: &OwnedSyntaxLayerInfo,
        cx: &mut ViewContext<Self>,
    ) -> impl Element<Self> {
        enum ToggleMenu {}
        MouseEventHandler::<ToggleMenu, Self>::new(0, cx, move |state, _| {
            let style = theme.toolbar_dropdown_menu.header.style_for(state, false);
            Flex::row()
                .with_child(
                    Label::new(active_layer.language.name().to_string(), style.text.clone())
                        .contained()
                        .with_margin_right(style.secondary_text_spacing),
                )
                .with_child(Label::new(
                    format_node_range(active_layer.node()),
                    style
                        .secondary_text
                        .clone()
                        .unwrap_or_else(|| style.text.clone()),
                ))
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, view, cx| {
            view.toggle_menu(cx);
        })
    }

    fn render_menu_item(
        theme: &Arc<Theme>,
        active_layer: &OwnedSyntaxLayerInfo,
        layer: SyntaxLayerInfo,
        layer_ix: usize,
        cx: &mut ViewContext<Self>,
    ) -> impl Element<Self> {
        enum ActivateLayer {}
        MouseEventHandler::<ActivateLayer, _>::new(layer_ix, cx, move |state, _| {
            let is_selected = layer.node() == active_layer.node();
            let style = theme
                .toolbar_dropdown_menu
                .item
                .style_for(state, is_selected);
            Flex::row()
                .with_child(
                    Label::new(layer.language.name().to_string(), style.text.clone())
                        .contained()
                        .with_margin_right(style.secondary_text_spacing),
                )
                .with_child(Label::new(
                    format_node_range(layer.node()),
                    style
                        .secondary_text
                        .clone()
                        .unwrap_or_else(|| style.text.clone()),
                ))
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, view, cx| {
            view.select_layer(layer_ix, cx);
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

impl Entity for SyntaxTreeToolbarItemView {
    type Event = ();
}

impl View for SyntaxTreeToolbarItemView {
    fn ui_name() -> &'static str {
        "SyntaxTreeToolbarItemView"
    }

    fn render(&mut self, cx: &mut ViewContext<'_, '_, Self>) -> gpui::AnyElement<Self> {
        self.render_menu(cx)
            .unwrap_or_else(|| Empty::new().into_any())
    }
}

impl ToolbarItemView for SyntaxTreeToolbarItemView {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> workspace::ToolbarItemLocation {
        self.menu_open = false;
        if let Some(item) = active_pane_item {
            if let Some(view) = item.downcast::<SyntaxTreeView>() {
                self.tree_view = Some(view.clone());
                self.subscription = Some(cx.observe(&view, |_, _, cx| cx.notify()));
                return ToolbarItemLocation::PrimaryLeft {
                    flex: Some((1., false)),
                };
            }
        }
        self.tree_view = None;
        self.subscription = None;
        ToolbarItemLocation::Hidden
    }
}
