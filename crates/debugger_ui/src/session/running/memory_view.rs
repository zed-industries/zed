use std::{
    cell::LazyCell,
    fmt::Write,
    ops::RangeInclusive,
    sync::{Arc, LazyLock},
    time::Duration,
};

use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    Action, AppContext, DismissEvent, DragMoveEvent, Empty, Entity, FocusHandle, Focusable,
    MouseButton, Point, ScrollStrategy, ScrollWheelEvent, Stateful, Subscription, Task, TextStyle,
    UniformList, UniformListScrollHandle, WeakEntity, actions, anchored, deferred, point,
    uniform_list,
};
use notifications::status_toast::{StatusToast, ToastIcon};
use project::debugger::{MemoryCell, dap_command::DataBreakpointContext, session::Session};
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    ContextMenu, Divider, DropdownMenu, FluentBuilder, IntoElement, PopoverMenuHandle, Render,
    Scrollbar, ScrollbarState, StatefulInteractiveElement, Tooltip, prelude::*,
};
use workspace::Workspace;

use crate::{ToggleDataBreakpoint, session::running::stack_frame_list::StackFrameList};

actions!(debugger, [GoToSelectedAddress]);

pub(crate) struct MemoryView {
    workspace: WeakEntity<Workspace>,
    scroll_handle: UniformListScrollHandle,
    scroll_state: ScrollbarState,
    stack_frame_list: WeakEntity<StackFrameList>,
    focus_handle: FocusHandle,
    view_state: ViewState,
    query_editor: Entity<Editor>,
    session: Entity<Session>,
    width_picker_handle: PopoverMenuHandle<ContextMenu>,
    is_writing_memory: bool,
    open_context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
}

impl Focusable for MemoryView {
    fn focus_handle(&self, _: &ui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
#[derive(Clone, Debug)]
struct Drag {
    start_address: u64,
    end_address: u64,
}

impl Drag {
    fn contains(&self, address: u64) -> bool {
        let range = self.memory_range();
        range.contains(&address)
    }

    fn memory_range(&self) -> RangeInclusive<u64> {
        if self.start_address < self.end_address {
            self.start_address..=self.end_address
        } else {
            self.end_address..=self.start_address
        }
    }
}
#[derive(Clone, Debug)]
enum SelectedMemoryRange {
    DragUnderway(Drag),
    DragComplete(Drag),
}

impl SelectedMemoryRange {
    fn contains(&self, address: u64) -> bool {
        match self {
            SelectedMemoryRange::DragUnderway(drag) => drag.contains(address),
            SelectedMemoryRange::DragComplete(drag) => drag.contains(address),
        }
    }
    fn is_dragging(&self) -> bool {
        matches!(self, SelectedMemoryRange::DragUnderway(_))
    }
    fn drag(&self) -> &Drag {
        match self {
            SelectedMemoryRange::DragUnderway(drag) => drag,
            SelectedMemoryRange::DragComplete(drag) => drag,
        }
    }
}

#[derive(Clone)]
struct ViewState {
    /// Uppermost row index
    base_row: u64,
    /// How many cells per row do we have?
    line_width: ViewWidth,
    selection: Option<SelectedMemoryRange>,
}

impl ViewState {
    fn new(base_row: u64, line_width: ViewWidth) -> Self {
        Self {
            base_row,
            line_width,
            selection: None,
        }
    }
    fn row_count(&self) -> u64 {
        // This was picked fully arbitrarily. There's no incentive for us to care about page sizes other than the fact that it seems to be a good
        // middle ground for data size.
        const PAGE_SIZE: u64 = 4096;
        PAGE_SIZE / self.line_width.width as u64
    }
    fn schedule_scroll_down(&mut self) {
        self.base_row = self.base_row.saturating_add(1)
    }
    fn schedule_scroll_up(&mut self) {
        self.base_row = self.base_row.saturating_sub(1);
    }
}

struct ScrollbarDragging;

static HEX_BYTES_MEMOIZED: LazyLock<[SharedString; 256]> =
    LazyLock::new(|| std::array::from_fn(|byte| SharedString::from(format!("{byte:02X}"))));
static UNKNOWN_BYTE: SharedString = SharedString::new_static("??");
impl MemoryView {
    pub(crate) fn new(
        session: Entity<Session>,
        workspace: WeakEntity<Workspace>,
        stack_frame_list: WeakEntity<StackFrameList>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let view_state = ViewState::new(0, WIDTHS[4].clone());
        let scroll_handle = UniformListScrollHandle::default();

        let query_editor = cx.new(|cx| Editor::single_line(window, cx));

        let scroll_state = ScrollbarState::new(scroll_handle.clone());
        let mut this = Self {
            workspace,
            scroll_state,
            scroll_handle,
            stack_frame_list,
            focus_handle: cx.focus_handle(),
            view_state,
            query_editor,
            session,
            width_picker_handle: Default::default(),
            is_writing_memory: true,
            open_context_menu: None,
        };
        this.change_query_bar_mode(false, window, cx);
        cx.on_focus_out(&this.focus_handle, window, |this, _, window, cx| {
            this.change_query_bar_mode(false, window, cx);
            cx.notify();
        })
        .detach();
        this
    }

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        div()
            .occlude()
            .id("memory-view-vertical-scrollbar")
            .on_drag_move(cx.listener(|this, evt, _, cx| {
                let did_handle = this.handle_scroll_drag(evt);
                cx.notify();
                if did_handle {
                    cx.stop_propagation()
                }
            }))
            .on_drag(ScrollbarDragging, |_, _, _, cx| cx.new(|_| Empty))
            .on_hover(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|_, _, _, cx| {
                    cx.stop_propagation();
                }),
            )
            .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                cx.notify();
            }))
            .h_full()
            .absolute()
            .right_1()
            .top_1()
            .bottom_0()
            .w(px(12.))
            .cursor_default()
            .children(Scrollbar::vertical(self.scroll_state.clone()).map(|s| s.auto_hide(cx)))
    }

    fn render_memory(&self, cx: &mut Context<Self>) -> UniformList {
        let weak = cx.weak_entity();
        let session = self.session.clone();
        let view_state = self.view_state.clone();
        uniform_list(
            "debugger-memory-view",
            self.view_state.row_count() as usize,
            move |range, _, cx| {
                let mut line_buffer = Vec::with_capacity(view_state.line_width.width as usize);
                let memory_start =
                    (view_state.base_row + range.start as u64) * view_state.line_width.width as u64;
                let memory_end = (view_state.base_row + range.end as u64)
                    * view_state.line_width.width as u64
                    - 1;
                let mut memory = session.update(cx, |this, cx| {
                    this.read_memory(memory_start..=memory_end, cx)
                });
                let mut rows = Vec::with_capacity(range.end - range.start);
                for ix in range {
                    line_buffer.extend((&mut memory).take(view_state.line_width.width as usize));
                    rows.push(render_single_memory_view_line(
                        &line_buffer,
                        ix as u64,
                        weak.clone(),
                        cx,
                    ));
                    line_buffer.clear();
                }
                rows
            },
        )
        .track_scroll(self.scroll_handle.clone())
        .on_scroll_wheel(cx.listener(|this, evt: &ScrollWheelEvent, window, _| {
            let delta = evt.delta.pixel_delta(window.line_height());
            let scroll_handle = this.scroll_state.scroll_handle();
            let size = scroll_handle.content_size();
            let viewport = scroll_handle.viewport();
            let current_offset = scroll_handle.offset();
            let first_entry_offset_boundary = size.height / this.view_state.row_count() as f32;
            let last_entry_offset_boundary = size.height - first_entry_offset_boundary;
            if first_entry_offset_boundary + viewport.size.height > current_offset.y.abs() {
                // The topmost entry is visible, hence if we're scrolling up, we need to load extra lines.
                this.view_state.schedule_scroll_up();
            } else if last_entry_offset_boundary < current_offset.y.abs() + viewport.size.height {
                this.view_state.schedule_scroll_down();
            }
            scroll_handle.set_offset(current_offset + point(px(0.), delta.y));
        }))
    }
    fn render_query_bar(&self, cx: &Context<Self>) -> impl IntoElement {
        EditorElement::new(
            &self.query_editor,
            Self::editor_style(&self.query_editor, cx),
        )
    }
    pub(super) fn go_to_memory_reference(
        &mut self,
        memory_reference: &str,
        evaluate_name: Option<&str>,
        stack_frame_id: Option<u64>,
        cx: &mut Context<Self>,
    ) {
        use parse_int::parse;
        let Ok(as_address) = parse::<u64>(&memory_reference) else {
            return;
        };
        let access_size = evaluate_name
            .map(|typ| {
                self.session.update(cx, |this, cx| {
                    this.data_access_size(stack_frame_id, typ, cx)
                })
            })
            .unwrap_or_else(|| Task::ready(None));
        cx.spawn(async move |this, cx| {
            let access_size = access_size.await.unwrap_or(1);
            this.update(cx, |this, cx| {
                this.view_state.selection = Some(SelectedMemoryRange::DragComplete(Drag {
                    start_address: as_address,
                    end_address: as_address + access_size - 1,
                }));
                this.jump_to_address(as_address, cx);
            })
            .ok();
        })
        .detach();
    }

    fn handle_memory_drag(&mut self, evt: &DragMoveEvent<Drag>) {
        if !self
            .view_state
            .selection
            .as_ref()
            .is_some_and(|selection| selection.is_dragging())
        {
            return;
        }
        let row_count = self.view_state.row_count();
        debug_assert!(row_count > 1);
        let scroll_handle = self.scroll_state.scroll_handle();
        let viewport = scroll_handle.viewport();

        if viewport.bottom() < evt.event.position.y {
            self.view_state.schedule_scroll_down();
        } else if viewport.top() > evt.event.position.y {
            self.view_state.schedule_scroll_up();
        }
    }

    fn handle_scroll_drag(&mut self, evt: &DragMoveEvent<ScrollbarDragging>) -> bool {
        if !self.scroll_state.is_dragging() {
            return false;
        }
        let row_count = self.view_state.row_count();
        debug_assert!(row_count > 1);
        let scroll_handle = self.scroll_state.scroll_handle();
        let viewport = scroll_handle.viewport();

        if viewport.bottom() < evt.event.position.y {
            self.view_state.schedule_scroll_down();
            true
        } else if viewport.top() > evt.event.position.y {
            self.view_state.schedule_scroll_up();
            true
        } else {
            false
        }
    }

    fn editor_style(editor: &Entity<Editor>, cx: &Context<Self>) -> EditorStyle {
        let is_read_only = editor.read(cx).read_only(cx);
        let settings = ThemeSettings::get_global(cx);
        let theme = cx.theme();
        let text_style = TextStyle {
            color: if is_read_only {
                theme.colors().text_muted
            } else {
                theme.colors().text
            },
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: TextSize::Small.rems(cx).into(),
            font_weight: settings.buffer_font.weight,

            ..Default::default()
        };
        EditorStyle {
            background: theme.colors().editor_background,
            local_player: theme.players().local(),
            text: text_style,
            ..Default::default()
        }
    }

    fn render_width_picker(&self, window: &mut Window, cx: &mut Context<Self>) -> DropdownMenu {
        let weak = cx.weak_entity();
        let selected_width = self.view_state.line_width.clone();
        DropdownMenu::new(
            "memory-view-width-picker",
            selected_width.label.clone(),
            ContextMenu::build(window, cx, |mut this, window, cx| {
                for width in &WIDTHS {
                    let weak = weak.clone();
                    let width = width.clone();
                    this = this.entry(width.label.clone(), None, move |_, cx| {
                        _ = weak.update(cx, |this, _| {
                            // Convert base ix between 2 line widths to keep the shown memory address roughly the same.
                            // All widths are powers of 2, so the conversion should be lossless.
                            match this.view_state.line_width.width.cmp(&width.width) {
                                std::cmp::Ordering::Less => {
                                    // We're converting up.
                                    let shift = width.width.trailing_zeros()
                                        - this.view_state.line_width.width.trailing_zeros();
                                    this.view_state.base_row >>= shift;
                                }
                                std::cmp::Ordering::Greater => {
                                    // We're converting down.
                                    let shift = this.view_state.line_width.width.trailing_zeros()
                                        - width.width.trailing_zeros();
                                    this.view_state.base_row <<= shift;
                                }
                                _ => {}
                            }
                            this.view_state.line_width = width.clone();
                        });
                    });
                }
                if let Some(ix) = WIDTHS
                    .iter()
                    .position(|width| width.width == selected_width.width)
                {
                    for _ in 0..=ix {
                        this.select_next(&Default::default(), window, cx);
                    }
                }
                this
            }),
        )
        .handle(self.width_picker_handle.clone())
    }

    fn page_down(&mut self, _: &menu::SelectLast, _: &mut Window, cx: &mut Context<Self>) {
        self.view_state.base_row = self
            .view_state
            .base_row
            .overflowing_add(self.view_state.row_count())
            .0;
        cx.notify();
    }
    fn page_up(&mut self, _: &menu::SelectFirst, _: &mut Window, cx: &mut Context<Self>) {
        self.view_state.base_row = self
            .view_state
            .base_row
            .overflowing_sub(self.view_state.row_count())
            .0;
        cx.notify();
    }

    fn change_query_bar_mode(
        &mut self,
        is_writing_memory: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if is_writing_memory == self.is_writing_memory {
            return;
        }
        if !self.is_writing_memory {
            self.query_editor.update(cx, |this, cx| {
                this.clear(window, cx);
                this.set_placeholder_text("Write to Selected Memory Range", cx);
            });
            self.is_writing_memory = true;
            self.query_editor.focus_handle(cx).focus(window);
        } else {
            self.query_editor.update(cx, |this, cx| {
                this.clear(window, cx);
                this.set_placeholder_text("Go to Memory Address / Expression", cx);
            });
            self.is_writing_memory = false;
        }
    }

    fn toggle_data_breakpoint(
        &mut self,
        _: &crate::ToggleDataBreakpoint,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(SelectedMemoryRange::DragComplete(selection)) = self.view_state.selection.clone()
        else {
            return;
        };
        let range = selection.memory_range();
        let context = Arc::new(DataBreakpointContext::Address {
            address: range.start().to_string(),
            bytes: Some(*range.end() - *range.start()),
        });

        self.session.update(cx, |this, cx| {
            let data_breakpoint_info = this.data_breakpoint_info(context.clone(), None, cx);
            cx.spawn(async move |this, cx| {
                if let Some(info) = data_breakpoint_info.await {
                    let Some(data_id) = info.data_id.clone() else {
                        return;
                    };
                    _ = this.update(cx, |this, cx| {
                        this.create_data_breakpoint(
                            context,
                            data_id.clone(),
                            dap::DataBreakpoint {
                                data_id,
                                access_type: None,
                                condition: None,
                                hit_condition: None,
                            },
                            cx,
                        );
                    });
                }
            })
            .detach();
        })
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(SelectedMemoryRange::DragComplete(drag)) = &self.view_state.selection {
            // Go into memory writing mode.
            if !self.is_writing_memory {
                let should_return = self.session.update(cx, |session, cx| {
                    if !session
                        .capabilities()
                        .supports_write_memory_request
                        .unwrap_or_default()
                    {
                        let adapter_name = session.adapter();
                        // We cannot write memory with this adapter.
                        _ = self.workspace.update(cx, |this, cx| {
                            this.toggle_status_toast(
                                StatusToast::new(format!(
                                    "Debug Adapter `{adapter_name}` does not support writing to memory"
                                ), cx, |this, cx| {
                                    cx.spawn(async move |this, cx| {
                                        cx.background_executor().timer(Duration::from_secs(2)).await;
                                        _ = this.update(cx, |_, cx| {
                                            cx.emit(DismissEvent)
                                        });
                                    }).detach();
                                    this.icon(ToastIcon::new(IconName::XCircle).color(Color::Error))
                                }),
                                cx,
                            );
                        });
                        true
                    } else {
                        false
                    }
                });
                if should_return {
                    return;
                }

                self.change_query_bar_mode(true, window, cx);
            } else if self.query_editor.focus_handle(cx).is_focused(window) {
                let mut text = self.query_editor.read(cx).text(cx);
                if text.chars().any(|c| !c.is_ascii_hexdigit()) {
                    // Interpret this text as a string and oh-so-conveniently convert it.
                    text = text.bytes().map(|byte| format!("{:02x}", byte)).collect();
                }
                self.session.update(cx, |this, cx| {
                    let range = drag.memory_range();

                    if let Ok(as_hex) = hex::decode(text) {
                        this.write_memory(*range.start(), &as_hex, cx);
                    }
                });
                self.change_query_bar_mode(false, window, cx);
            }

            cx.notify();
            return;
        }
        // Just change the currently viewed address.
        if !self.query_editor.focus_handle(cx).is_focused(window) {
            return;
        }
        self.jump_to_query_bar_address(cx);
    }

    fn jump_to_query_bar_address(&mut self, cx: &mut Context<Self>) {
        use parse_int::parse;
        let text = self.query_editor.read(cx).text(cx);

        let Ok(as_address) = parse::<u64>(&text) else {
            return self.jump_to_expression(text, cx);
        };
        self.jump_to_address(as_address, cx);
    }

    fn jump_to_address(&mut self, address: u64, cx: &mut Context<Self>) {
        self.view_state.base_row = (address & !0xfff) / self.view_state.line_width.width as u64;
        let line_ix = (address & 0xfff) / self.view_state.line_width.width as u64;
        self.scroll_handle
            .scroll_to_item(line_ix as usize, ScrollStrategy::Center);
        cx.notify();
    }

    fn jump_to_expression(&mut self, expr: String, cx: &mut Context<Self>) {
        let Ok(selected_frame) = self
            .stack_frame_list
            .update(cx, |this, _| this.opened_stack_frame_id())
        else {
            return;
        };
        let expr = format!("?${{{expr}}}");
        let reference = self.session.update(cx, |this, cx| {
            this.memory_reference_of_expr(selected_frame, expr, cx)
        });
        cx.spawn(async move |this, cx| {
            if let Some((reference, typ)) = reference.await {
                _ = this.update(cx, |this, cx| {
                    let sizeof_expr = if typ.as_ref().is_some_and(|t| {
                        t.chars()
                            .all(|c| c.is_whitespace() || c.is_alphabetic() || c == '*')
                    }) {
                        typ.as_deref()
                    } else {
                        None
                    };
                    this.go_to_memory_reference(&reference, sizeof_expr, selected_frame, cx);
                });
            }
        })
        .detach();
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        self.view_state.selection = None;
        cx.notify();
    }

    /// Jump to memory pointed to by selected memory range.
    fn go_to_address(
        &mut self,
        _: &GoToSelectedAddress,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(SelectedMemoryRange::DragComplete(drag)) = self.view_state.selection.clone()
        else {
            return;
        };
        let range = drag.memory_range();
        let Some(memory): Option<Vec<u8>> = self.session.update(cx, |this, cx| {
            this.read_memory(range, cx).map(|cell| cell.0).collect()
        }) else {
            return;
        };
        if memory.len() > 8 {
            return;
        }
        let zeros_to_write = 8 - memory.len();
        let mut acc = String::from("0x");
        acc.extend(std::iter::repeat("00").take(zeros_to_write));
        let as_query = memory.into_iter().rev().fold(acc, |mut acc, byte| {
            _ = write!(&mut acc, "{:02x}", byte);
            acc
        });
        self.query_editor.update(cx, |this, cx| {
            this.set_text(as_query, window, cx);
        });
        self.jump_to_query_bar_address(cx);
    }

    fn deploy_memory_context_menu(
        &mut self,
        range: RangeInclusive<u64>,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let session = self.session.clone();
        let context_menu = ContextMenu::build(window, cx, |menu, _, cx| {
            let range_too_large = range.end() - range.start() > std::mem::size_of::<u64>() as u64;
            let caps = session.read(cx).capabilities();
            let supports_data_breakpoints = caps.supports_data_breakpoints.unwrap_or_default()
                && caps.supports_data_breakpoint_bytes.unwrap_or_default();
            let memory_unreadable = LazyCell::new(|| {
                session.update(cx, |this, cx| {
                    this.read_memory(range.clone(), cx)
                        .any(|cell| cell.0.is_none())
                })
            });

            let mut menu = menu.action_disabled_when(
                range_too_large || *memory_unreadable,
                "Go To Selected Address",
                GoToSelectedAddress.boxed_clone(),
            );

            if supports_data_breakpoints {
                menu = menu.action_disabled_when(
                    *memory_unreadable,
                    "Set Data Breakpoint",
                    ToggleDataBreakpoint { access_type: None }.boxed_clone(),
                );
            }
            menu.context(self.focus_handle.clone())
        });

        cx.focus_view(&context_menu, window);
        let subscription = cx.subscribe_in(
            &context_menu,
            window,
            |this, _, _: &DismissEvent, window, cx| {
                if this.open_context_menu.as_ref().is_some_and(|context_menu| {
                    context_menu.0.focus_handle(cx).contains_focused(window, cx)
                }) {
                    cx.focus_self(window);
                }
                this.open_context_menu.take();
                cx.notify();
            },
        );

        self.open_context_menu = Some((context_menu, position, subscription));
    }
}

#[derive(Clone)]
struct ViewWidth {
    width: u8,
    label: SharedString,
}

impl ViewWidth {
    const fn new(width: u8, label: &'static str) -> Self {
        Self {
            width,
            label: SharedString::new_static(label),
        }
    }
}

static WIDTHS: [ViewWidth; 7] = [
    ViewWidth::new(1, "1 byte"),
    ViewWidth::new(2, "2 bytes"),
    ViewWidth::new(4, "4 bytes"),
    ViewWidth::new(8, "8 bytes"),
    ViewWidth::new(16, "16 bytes"),
    ViewWidth::new(32, "32 bytes"),
    ViewWidth::new(64, "64 bytes"),
];

fn render_single_memory_view_line(
    memory: &[MemoryCell],
    ix: u64,
    weak: gpui::WeakEntity<MemoryView>,
    cx: &mut App,
) -> AnyElement {
    let Ok(view_state) = weak.update(cx, |this, _| this.view_state.clone()) else {
        return div().into_any();
    };
    let base_address = (view_state.base_row + ix) * view_state.line_width.width as u64;

    h_flex()
        .id((
            "memory-view-row-full",
            ix * view_state.line_width.width as u64,
        ))
        .size_full()
        .gap_x_2()
        .child(
            div()
                .child(
                    Label::new(format!("{:016X}", base_address))
                        .buffer_font(cx)
                        .size(ui::LabelSize::Small)
                        .color(Color::Muted),
                )
                .px_1()
                .border_r_1()
                .border_color(Color::Muted.color(cx)),
        )
        .child(
            h_flex()
                .id((
                    "memory-view-row-raw-memory",
                    ix * view_state.line_width.width as u64,
                ))
                .px_1()
                .children(memory.iter().enumerate().map(|(cell_ix, cell)| {
                    let weak = weak.clone();
                    div()
                        .id(("memory-view-row-raw-memory-cell", cell_ix as u64))
                        .px_0p5()
                        .when_some(view_state.selection.as_ref(), |this, selection| {
                            this.when(selection.contains(base_address + cell_ix as u64), |this| {
                                let weak = weak.clone();

                                this.bg(Color::Selected.color(cx).opacity(0.2)).when(
                                    !selection.is_dragging(),
                                    |this| {
                                        let selection = selection.drag().memory_range();
                                        this.on_mouse_down(
                                            MouseButton::Right,
                                            move |click, window, cx| {
                                                _ = weak.update(cx, |this, cx| {
                                                    this.deploy_memory_context_menu(
                                                        selection.clone(),
                                                        click.position,
                                                        window,
                                                        cx,
                                                    )
                                                });
                                                cx.stop_propagation();
                                            },
                                        )
                                    },
                                )
                            })
                        })
                        .child(
                            Label::new(
                                cell.0
                                    .map(|val| HEX_BYTES_MEMOIZED[val as usize].clone())
                                    .unwrap_or_else(|| UNKNOWN_BYTE.clone()),
                            )
                            .buffer_font(cx)
                            .when(cell.0.is_none(), |this| this.color(Color::Muted))
                            .size(ui::LabelSize::Small),
                        )
                        .on_drag(
                            Drag {
                                start_address: base_address + cell_ix as u64,
                                end_address: base_address + cell_ix as u64,
                            },
                            {
                                let weak = weak.clone();
                                move |drag, _, _, cx| {
                                    _ = weak.update(cx, |this, _| {
                                        this.view_state.selection =
                                            Some(SelectedMemoryRange::DragUnderway(drag.clone()));
                                    });

                                    cx.new(|_| Empty)
                                }
                            },
                        )
                        .on_drop({
                            let weak = weak.clone();
                            move |drag: &Drag, _, cx| {
                                _ = weak.update(cx, |this, _| {
                                    this.view_state.selection =
                                        Some(SelectedMemoryRange::DragComplete(Drag {
                                            start_address: drag.start_address,
                                            end_address: base_address + cell_ix as u64,
                                        }));
                                });
                            }
                        })
                        .drag_over(move |style, drag: &Drag, _, cx| {
                            _ = weak.update(cx, |this, _| {
                                this.view_state.selection =
                                    Some(SelectedMemoryRange::DragUnderway(Drag {
                                        start_address: drag.start_address,
                                        end_address: base_address + cell_ix as u64,
                                    }));
                            });

                            style
                        })
                })),
        )
        .child(
            h_flex()
                .id((
                    "memory-view-row-ascii-memory",
                    ix * view_state.line_width.width as u64,
                ))
                .h_full()
                .px_1()
                .mr_4()
                // .gap_x_1p5()
                .border_x_1()
                .border_color(Color::Muted.color(cx))
                .children(memory.iter().enumerate().map(|(ix, cell)| {
                    let as_character = char::from(cell.0.unwrap_or(0));
                    let as_visible = if as_character.is_ascii_graphic() {
                        as_character
                    } else {
                        '·'
                    };
                    div()
                        .px_0p5()
                        .when_some(view_state.selection.as_ref(), |this, selection| {
                            this.when(selection.contains(base_address + ix as u64), |this| {
                                this.bg(Color::Selected.color(cx).opacity(0.2))
                            })
                        })
                        .child(
                            Label::new(format!("{as_visible}"))
                                .buffer_font(cx)
                                .when(cell.0.is_none(), |this| this.color(Color::Muted))
                                .size(ui::LabelSize::Small),
                        )
                })),
        )
        .into_any()
}

impl Render for MemoryView {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        let (icon, tooltip_text) = if self.is_writing_memory {
            (IconName::Pencil, "Edit memory at a selected address")
        } else {
            (
                IconName::LocationEdit,
                "Change address of currently viewed memory",
            )
        };
        v_flex()
            .id("Memory-view")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::go_to_address))
            .p_1()
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::toggle_data_breakpoint))
            .on_action(cx.listener(Self::page_down))
            .on_action(cx.listener(Self::page_up))
            .size_full()
            .track_focus(&self.focus_handle)
            .child(
                h_flex()
                    .w_full()
                    .mb_0p5()
                    .gap_1()
                    .child(
                        h_flex()
                            .w_full()
                            .rounded_md()
                            .border_1()
                            .gap_x_2()
                            .px_2()
                            .py_0p5()
                            .mb_0p5()
                            .bg(cx.theme().colors().editor_background)
                            .when_else(
                                self.query_editor
                                    .focus_handle(cx)
                                    .contains_focused(window, cx),
                                |this| this.border_color(cx.theme().colors().border_focused),
                                |this| this.border_color(cx.theme().colors().border_transparent),
                            )
                            .child(
                                div()
                                    .id("memory-view-editor-icon")
                                    .child(Icon::new(icon).size(ui::IconSize::XSmall))
                                    .tooltip(Tooltip::text(tooltip_text)),
                            )
                            .child(self.render_query_bar(cx)),
                    )
                    .child(self.render_width_picker(window, cx)),
            )
            .child(Divider::horizontal())
            .child(
                v_flex()
                    .size_full()
                    .on_drag_move(cx.listener(|this, evt, _, _| {
                        this.handle_memory_drag(&evt);
                    }))
                    .child(self.render_memory(cx).size_full())
                    .children(self.open_context_menu.as_ref().map(|(menu, position, _)| {
                        deferred(
                            anchored()
                                .position(*position)
                                .anchor(gpui::Corner::TopLeft)
                                .child(menu.clone()),
                        )
                        .with_priority(1)
                    }))
                    .child(self.render_vertical_scrollbar(cx)),
            )
    }
}
