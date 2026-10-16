use anyhow::anyhow;
use dap::client::SessionId;
use gpui::{
    AnyElement, App, AsyncWindowContext, Bounds, Context, Entity, Focusable as _, Hsla,
    InteractiveElement, IntoElement, MouseButton, MouseDownEvent, ParentElement, Pixels, Render,
    Subscription, Task, WeakEntity, Window, canvas, div, px,
};
use itertools::Itertools;
use project::{DebuggerHoverData, DebuggerHoverVariable, Project};
use settings::Settings;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use theme_settings::ThemeSettings;
use ui::{ContextMenu, Disclosure, Tooltip, prelude::*};
use util::ResultExt;

pub(crate) fn build_debugger_hover_view(
    debugger_value: Option<DebuggerHoverData>,
    project: Option<WeakEntity<Project>>,
    cx: &mut AsyncWindowContext,
) -> Option<Entity<DebuggerHoverView>> {
    let debugger_value = debugger_value?;

    Some(cx.new(|cx| {
        let mut view = DebuggerHoverView::new(debugger_value, project);
        if view.root.has_children() {
            view.toggle_node(Vec::new(), cx);
        }
        view
    }))
}

enum DebuggerHoverChildren {
    Unsupported,
    Unloaded,
    Loading,
    Loaded(Vec<DebuggerHoverNode>),
    Failed(String),
}

struct DebuggerHoverNode {
    variable: DebuggerHoverVariable,
    is_expanded: bool,
    children: DebuggerHoverChildren,
    load_task: Option<Task<()>>,
}

impl DebuggerHoverNode {
    fn new(variable: DebuggerHoverVariable) -> Self {
        let children = if variable.has_children() {
            DebuggerHoverChildren::Unloaded
        } else {
            DebuggerHoverChildren::Unsupported
        };

        Self {
            variable,
            is_expanded: false,
            children,
            load_task: None,
        }
    }

    fn has_children(&self) -> bool {
        self.variable.has_children()
    }
}

pub(crate) struct DebuggerHoverView {
    project: Option<WeakEntity<Project>>,
    session_id: SessionId,
    root: DebuggerHoverNode,
    selected_path: Vec<usize>,
    row_bounds: Rc<RefCell<HashMap<Vec<usize>, Bounds<Pixels>>>>,
    context_menu: Option<Entity<ContextMenu>>,
    menu_position: gpui::Point<Pixels>,
    menu_subscription: Option<Subscription>,
    operation_error: Option<String>,
}

pub(crate) struct DebuggerHoverVariableColors {
    pub(crate) name: Option<Hsla>,
    pub(crate) value: Option<Hsla>,
    pub(crate) type_name: Option<Hsla>,
}

pub(crate) const DEBUGGER_HOVER_MIN_WIDTH: Pixels = px(320.0);

impl DebuggerHoverView {
    pub(crate) fn has_menu(&self) -> bool {
        self.context_menu.is_some()
    }

    pub(crate) fn context_menu(&self) -> Option<&Entity<ContextMenu>> {
        self.context_menu.as_ref()
    }

    fn new(debugger_value: DebuggerHoverData, project: Option<WeakEntity<Project>>) -> Self {
        Self {
            project,
            session_id: debugger_value.session_id,
            root: DebuggerHoverNode::new(debugger_value.root),
            selected_path: Vec::new(),
            row_bounds: Rc::new(RefCell::new(HashMap::default())),
            context_menu: None,
            menu_position: Default::default(),
            menu_subscription: None,
            operation_error: None,
        }
    }

    fn node_mut(&mut self, path: &[usize]) -> Option<&mut DebuggerHoverNode> {
        let mut node = &mut self.root;
        for index in path {
            match &mut node.children {
                DebuggerHoverChildren::Loaded(children) => node = children.get_mut(*index)?,
                DebuggerHoverChildren::Unsupported
                | DebuggerHoverChildren::Unloaded
                | DebuggerHoverChildren::Loading
                | DebuggerHoverChildren::Failed(_) => return None,
            }
        }

        Some(node)
    }

    pub(crate) fn toggle_node(&mut self, path: Vec<usize>, cx: &mut Context<Self>) {
        let Some(node) = self.node_mut(&path) else {
            return;
        };

        if !node.has_children() {
            return;
        }

        node.is_expanded = !node.is_expanded;
        let should_load = node.is_expanded
            && matches!(
                node.children,
                DebuggerHoverChildren::Unloaded | DebuggerHoverChildren::Failed(_)
            );
        let variables_reference = node.variable.variables_reference;
        if should_load {
            node.children = DebuggerHoverChildren::Loading;
        }

        cx.notify();

        if should_load {
            self.load_children(path, variables_reference, cx);
        }
    }

    fn load_children(
        &mut self,
        path: Vec<usize>,
        variables_reference: u64,
        cx: &mut Context<Self>,
    ) {
        let project = self.project.clone();
        let session_id = self.session_id;
        let Some(node) = self.node_mut(&path) else {
            return;
        };

        node.load_task = Some(cx.spawn(async move |this, cx| {
            let result = match project {
                Some(project) => match project.update(cx, |project, cx| {
                    project.load_debugger_hover_children(session_id, variables_reference, cx)
                }) {
                    Ok(task) => task.await.map_err(|error| error.to_string()),
                    Err(error) => Err(error.to_string()),
                },
                None => Err(anyhow!("project is no longer available").to_string()),
            };

            this.update(cx, |this, cx| {
                let Some(node) = this.node_mut(&path) else {
                    return;
                };

                node.load_task.take();
                node.children = match result {
                    Ok(children) => DebuggerHoverChildren::Loaded(
                        children.into_iter().map(DebuggerHoverNode::new).collect(),
                    ),
                    Err(error) => DebuggerHoverChildren::Failed(error),
                };
                cx.notify();
            })
            .ok();
        }));
    }

    fn visible_paths(&self) -> Vec<Vec<usize>> {
        fn collect(node: &DebuggerHoverNode, path: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
            out.push(path.clone());

            if !node.is_expanded {
                return;
            }

            if let DebuggerHoverChildren::Loaded(children) = &node.children {
                for (index, child) in children.iter().enumerate() {
                    path.push(index);
                    collect(child, path, out);
                    path.pop();
                }
            }
        }

        let mut out = Vec::new();
        collect(&self.root, &mut Vec::new(), &mut out);
        out
    }

    fn variable_menu(
        &mut self,
        variable: DebuggerHoverVariable,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let view = cx.entity().downgrade();
        let menu = ContextMenu::build(window, cx, |menu, _, _| {
            menu.key_context("menu DebuggerHoverMenu")
                .entry("Copy Value", None, move |_, cx| {
                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(variable.value.clone()));
                })
                .when_some(variable.evaluate_name, |menu, expression| {
                    let watch_expression = expression.clone();
                    menu.entry("Copy as Expression", None, move |_, cx| {
                        cx.write_to_clipboard(gpui::ClipboardItem::new_string(expression.clone()));
                    })
                    .entry("Add to Watch", None, move |_, cx| {
                        view.update(cx, |view, cx| view.add_watch(watch_expression.clone(), cx))
                            .log_err();
                    })
                })
        });
        menu.update(cx, |menu, cx| menu.select_toggled_or_first(window, cx));
        let previous_focus = window.focused(cx);
        self.menu_subscription = Some(cx.subscribe_in(
            &menu,
            window,
            move |view, menu, _: &gpui::DismissEvent, window, cx| {
                if menu.focus_handle(cx).contains_focused(window, cx)
                    && let Some(focus) = &previous_focus
                {
                    window.focus(focus, cx);
                }
                view.context_menu = None;
                cx.notify();
            },
        ));
        self.context_menu = Some(menu.clone());
        self.menu_position = window.mouse_position();
        let focus = menu.focus_handle(cx);
        window.on_next_frame(move |window, _| {
            window.on_next_frame(move |window, cx| window.focus(&focus, cx));
        });
        cx.notify();
        menu
    }

    fn add_watch(&mut self, expression: String, cx: &mut Context<Self>) {
        let session_id = self.session_id;
        let task = self.project.as_ref().and_then(|project| {
            project
                .update(cx, |project, cx| {
                    let (session, frame) = project.active_debug_session(cx)?;
                    if session.read(cx).session_id() != session_id {
                        return None;
                    }
                    Some(session.update(cx, |session, cx| {
                        session.add_watcher(expression.into(), frame.stack_frame_id, cx)
                    }))
                })
                .ok()
                .flatten()
        });
        let Some(task) = task else {
            self.operation_error = Some("The debug session is no longer paused here".into());
            cx.notify();
            return;
        };
        self.operation_error = None;
        cx.spawn(async move |view, cx| {
            let result = task.await;
            view.update(cx, |view, cx| {
                view.operation_error = result
                    .err()
                    .map(|error| format!("Could not add watch: {error}"));
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }

    fn select_path(&mut self, path: Vec<usize>, cx: &mut Context<Self>) {
        self.selected_path = path;
        cx.notify();
    }

    pub(crate) fn select_next(&mut self, cx: &mut Context<Self>) {
        let visible_paths = self.visible_paths();
        let Some(index) = visible_paths
            .iter()
            .position(|path| path == &self.selected_path)
        else {
            self.selected_path = Vec::new();
            cx.notify();
            return;
        };

        if let Some(path) = visible_paths.get(index + 1) {
            self.selected_path = path.clone();
            cx.notify();
        }
    }

    pub(crate) fn select_previous(&mut self, cx: &mut Context<Self>) {
        let visible_paths = self.visible_paths();
        let Some(index) = visible_paths
            .iter()
            .position(|path| path == &self.selected_path)
        else {
            self.selected_path = Vec::new();
            cx.notify();
            return;
        };

        if let Some(path) = index
            .checked_sub(1)
            .and_then(|previous_index| visible_paths.get(previous_index))
        {
            self.selected_path = path.clone();
            cx.notify();
        }
    }

    pub(crate) fn expand_selected(&mut self, cx: &mut Context<Self>) {
        let path = self.selected_path.clone();
        let Some(node) = self.node_mut(&path) else {
            return;
        };

        if !node.has_children() {
            return;
        }

        if !node.is_expanded {
            self.toggle_node(path, cx);
            return;
        }

        if let DebuggerHoverChildren::Loaded(children) = &node.children
            && !children.is_empty()
        {
            let mut child_path = path;
            child_path.push(0);
            self.selected_path = child_path;
            cx.notify();
        }
    }

    pub(crate) fn collapse_selected(&mut self, cx: &mut Context<Self>) {
        let path = self.selected_path.clone();
        let Some(node) = self.node_mut(&path) else {
            return;
        };

        if node.is_expanded {
            self.toggle_node(path, cx);
            return;
        }

        if !self.selected_path.is_empty() {
            self.selected_path.pop();
            cx.notify();
        }
    }

    pub(crate) fn selected_row_bounds(&self) -> Option<Bounds<Pixels>> {
        self.row_bounds.borrow().get(&self.selected_path).copied()
    }

    fn render_entries(
        &self,
        node: &DebuggerHoverNode,
        depth: usize,
        path: &[usize],
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let mut entries = vec![self.render_node(node, depth, path, cx)];

        if node.is_expanded {
            match &node.children {
                DebuggerHoverChildren::Loaded(children) => {
                    for (index, child) in children.iter().enumerate() {
                        let mut child_path = path.to_vec();
                        child_path.push(index);
                        entries.extend(self.render_entries(child, depth + 1, &child_path, cx));
                    }
                }
                DebuggerHoverChildren::Loading => {
                    entries.push(self.render_status_row("Loading…", depth + 1, cx));
                }
                DebuggerHoverChildren::Failed(error) => {
                    entries.push(self.render_status_row(error, depth + 1, cx));
                }
                DebuggerHoverChildren::Unsupported | DebuggerHoverChildren::Unloaded => {}
            }
        }

        entries
    }

    fn render_status_row(&self, message: &str, depth: usize, cx: &mut Context<Self>) -> AnyElement {
        div()
            .w_full()
            .min_h_5()
            .pl(px(depth as f32 * 14.0 + 18.0))
            .pr_1()
            .flex()
            .items_center()
            .text_ui_sm(cx)
            .font_buffer(cx)
            .child(
                Label::new(message.to_string())
                    .single_line()
                    .truncate()
                    .color(Color::Muted),
            )
            .into_any_element()
    }

    fn render_node(
        &self,
        node: &DebuggerHoverNode,
        depth: usize,
        path: &[usize],
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let path = path.to_vec();
        let row_selector = debugger_hover_row_selector(&path);
        let toggle_selector = debugger_hover_toggle_selector(&path);
        let value_container_selector = format!("{row_selector}-value-container");
        let is_expandable = node.has_children();
        let is_selected = self.selected_path == path;
        let variable_colors = debugger_hover_variable_colors(cx);
        let row_bounds = self.row_bounds.clone();
        let row_bounds_path = path.clone();
        let variable = node.variable.clone();

        if depth == 0 && !is_expandable {
            let scalar = div()
                .debug_selector(|| row_selector.clone())
                .w_full()
                .min_w_0()
                .text_ui_sm(cx)
                .font_buffer(cx)
                .whitespace_normal()
                .child(
                    div()
                        .debug_selector(move || value_container_selector.clone())
                        .w_full()
                        .child(variable.value.clone()),
                );
            return scalar
                .on_mouse_down(
                    MouseButton::Right,
                    cx.listener(move |view, _, window, cx| {
                        window.prevent_default();
                        cx.stop_propagation();
                        view.variable_menu(variable.clone(), window, cx);
                    }),
                )
                .into_any_element();
        }

        let row = div()
            .min_w(gpui::relative(1.))
            .flex_none()
            .debug_selector(|| row_selector.clone())
            .rounded_sm()
            .child(
                canvas(
                    move |bounds, _window, _cx| {
                        row_bounds
                            .borrow_mut()
                            .insert(row_bounds_path.clone(), bounds);
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            .when(!is_expandable, |this| {
                this.on_mouse_down(
                    MouseButton::Left,
                    cx.listener({
                        let path = path.clone();
                        move |this, _: &MouseDownEvent, _window, cx| {
                            this.select_path(path.clone(), cx);
                        }
                    }),
                )
            })
            .when(is_expandable, |this| {
                this.cursor_pointer().on_mouse_down(
                    MouseButton::Left,
                    cx.listener({
                        let path = path.clone();
                        move |this, _: &MouseDownEvent, window, cx| {
                            window.prevent_default();
                            this.select_path(path.clone(), cx);
                            this.toggle_node(path.clone(), cx)
                        }
                    }),
                )
            })
            .child(
                h_flex()
                    .min_w(gpui::relative(1.))
                    .min_h_5()
                    .items_center()
                    .gap_1()
                    .rounded_sm()
                    .px_1()
                    .when(is_selected, |this| {
                        this.bg(cx.theme().colors().ghost_element_selected)
                    })
                    .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                    .pl(px(depth as f32 * 14.0))
                    .child(if is_expandable {
                        div()
                            .debug_selector(|| toggle_selector.clone())
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .child(
                                Disclosure::new(toggle_selector.clone(), node.is_expanded)
                                    .on_click(cx.listener({
                                        let path = path.clone();
                                        move |this, _, window, cx| {
                                            window.prevent_default();
                                            this.toggle_node(path.clone(), cx)
                                        }
                                    })),
                            )
                            .into_any_element()
                    } else {
                        div().w_4().flex_none().into_any_element()
                    })
                    .child(
                        h_flex()
                            .flex_none()
                            .gap_0p5()
                            .text_ui_sm(cx)
                            .font_buffer(cx)
                            .child(
                                div()
                                    .id(format!("{row_selector}-name"))
                                    .flex_none()
                                    .tooltip(Tooltip::text(node.variable.name.clone()))
                                    .child(
                                        Label::new(node.variable.name.clone())
                                            .single_line()
                                            .when_some(variable_colors.name, |this, color| {
                                                this.color(Color::from(color))
                                            }),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .debug_selector(move || value_container_selector.clone())
                                    .flex_none()
                                    .gap_0p5()
                                    .child(Label::new("=").single_line().color(Color::Muted))
                                    .child(
                                        div()
                                            .id(format!("{row_selector}-value"))
                                            .flex_none()
                                            .tooltip(Tooltip::text(node.variable.value.clone()))
                                            .child(
                                                Label::new(node.variable.value.clone())
                                                    .single_line()
                                                    .color(Color::Muted)
                                                    .when_some(
                                                        variable_colors.value,
                                                        |this, color| {
                                                            this.color(Color::from(color))
                                                        },
                                                    ),
                                            ),
                                    ),
                            )
                            .children(
                                debugger_hover_type_suffix(
                                    depth,
                                    node.variable.type_name.as_deref(),
                                )
                                .map(|type_name| {
                                    div().flex_none().child(
                                        Label::new(type_name)
                                            .single_line()
                                            .color(Color::Muted)
                                            .when_some(variable_colors.type_name, |this, color| {
                                                this.color(Color::from(color))
                                            }),
                                    )
                                }),
                            ),
                    ),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |view, _, window, cx| {
                    window.prevent_default();
                    cx.stop_propagation();
                    view.variable_menu(variable.clone(), window, cx);
                }),
            )
            .into_any_element();
        row
    }
}

pub(crate) fn debugger_hover_variable_colors(cx: &App) -> DebuggerHoverVariableColors {
    let syntax = cx.theme().syntax();
    let colors = cx.theme().colors();
    let syntax_color_for = |name| syntax.style_for_name(name).and_then(|style| style.color);

    DebuggerHoverVariableColors {
        name: syntax_color_for("variable").or(Some(colors.text)),
        value: syntax_color_for("variable.special").or(Some(colors.text_accent)),
        type_name: syntax_color_for("type").or(Some(colors.text_muted)),
    }
}

pub(crate) fn debugger_hover_type_suffix(depth: usize, type_name: Option<&str>) -> Option<String> {
    if depth > 0 {
        return None;
    }

    type_name
        .filter(|type_name| !type_name.is_empty())
        .map(|type_name| format!(": {type_name}"))
}

impl DebuggerHoverView {
    pub(crate) fn content_width(&self, window: &Window, cx: &App) -> Option<Pixels> {
        self.root.has_children().then(|| {
            let mut width = Pixels::ZERO;
            let mut nodes = vec![(&self.root, 0)];
            let font = ThemeSettings::get_global(cx).buffer_font.clone();
            let font_size = ui::TextSize::Small.rems(cx).to_pixels(window.rem_size());
            while let Some((node, depth)) = nodes.pop() {
                let text = ui::utils::replace_control_characters(&format!(
                    "{} = {}{}",
                    node.variable.name,
                    node.variable.value,
                    debugger_hover_type_suffix(depth, node.variable.type_name.as_deref())
                        .unwrap_or_default(),
                ))
                .into_owned();
                let line = window.text_system().shape_line(
                    text.clone().into(),
                    font_size,
                    &[gpui::TextRun {
                        len: text.len(),
                        font: font.clone(),
                        ..Default::default()
                    }],
                    None,
                );
                width = width.max(line.width + px(depth as f32 * 14.) + window.rem_size() * 3.);
                if node.is_expanded
                    && let DebuggerHoverChildren::Loaded(children) = &node.children
                {
                    nodes.extend(children.iter().map(|child| (child, depth + 1)));
                }
            }
            width
        })
    }
}

impl Render for DebuggerHoverView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.row_bounds.borrow_mut().clear();
        let rows = self.render_entries(&self.root, 0, &[], cx);
        v_flex()
            .id("debugger-hover-view")
            .w_full()
            .flex_none()
            .gap_0()
            .children(rows)
            .children(self.context_menu.as_ref().map(|menu| {
                gpui::deferred(
                    gpui::anchored()
                        .position(self.menu_position)
                        .snap_to_window_with_margin(px(8.))
                        .child(menu.clone()),
                )
                // The editor draws hover popovers at priority 2.
                .with_priority(3)
            }))
            .when_some(self.operation_error.as_ref(), |this, error| {
                this.child(self.render_status_row(error, 0, cx))
            })
    }
}

fn debugger_hover_row_selector(path: &[usize]) -> String {
    if path.is_empty() {
        "debugger-hover-node-root".to_string()
    } else {
        format!(
            "debugger-hover-node-{}",
            path.iter().map(|index| index.to_string()).join("-")
        )
    }
}

fn debugger_hover_toggle_selector(path: &[usize]) -> String {
    if path.is_empty() {
        "debugger-hover-toggle-root".to_string()
    } else {
        format!(
            "debugger-hover-toggle-{}",
            path.iter().map(|index| index.to_string()).join("-")
        )
    }
}
