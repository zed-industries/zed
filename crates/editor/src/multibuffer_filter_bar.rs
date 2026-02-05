use crate::{Editor, EditorEvent, EditorStyle, FilterableMultibufferState};
use collections::HashMap;
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    Styled, Subscription, Task, TextStyle, WeakEntity, Window, relative, rems,
};
use language::{Buffer, Point};
use settings::Settings;
use std::{ops::Range, time::Duration};
use theme::ThemeSettings;
use ui::{IconButton, IconButtonShape, Tooltip, prelude::*};
use util::paths::{PathMatcher, PathStyle};
use workspace::{
    ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
    item::{ItemBufferKind, ItemHandle},
};

const FILTER_DEBOUNCE_MS: u64 = 150;

/// Toolbar bar for filtering files in multibuffer views like "Find All References".
pub struct MultibufferFilterBar {
    include_editor: Entity<Editor>,
    exclude_editor: Entity<Editor>,
    filters_enabled: bool,
    active_editor: Option<WeakEntity<Editor>>,
    filter_state: Option<Entity<FilterableMultibufferState>>,
    debounce_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl MultibufferFilterBar {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let include_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Include: src/**/*.rs", window, cx);
            editor
        });

        let exclude_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Exclude: *_test.rs, test/**", window, cx);
            editor
        });

        let subscriptions = vec![
            cx.subscribe(&include_editor, |this, _, event: &EditorEvent, cx| {
                if matches!(event, EditorEvent::BufferEdited) {
                    this.schedule_filter_update(cx);
                }
            }),
            cx.subscribe(&exclude_editor, |this, _, event: &EditorEvent, cx| {
                if matches!(event, EditorEvent::BufferEdited) {
                    this.schedule_filter_update(cx);
                }
            }),
        ];

        Self {
            include_editor,
            exclude_editor,
            filters_enabled: false,
            active_editor: None,
            filter_state: None,
            debounce_task: Task::ready(()),
            _subscriptions: subscriptions,
        }
    }

    fn schedule_filter_update(&mut self, cx: &mut Context<Self>) {
        if !self.filters_enabled {
            return;
        }

        self.debounce_task = cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(FILTER_DEBOUNCE_MS))
                .await;
            this.update(cx, |this, cx| {
                this.apply_filters(cx);
            })
            .ok();
        });
    }

    fn apply_filters(&mut self, cx: &mut Context<Self>) {
        if !self.filters_enabled {
            return;
        }

        let Some(editor) = self.active_editor.as_ref().and_then(|e| e.upgrade()) else {
            return;
        };

        let Some(filter_state) = self.filter_state.clone() else {
            return;
        };

        let include_text = self.include_editor.read(cx).text(cx);
        let exclude_text = self.exclude_editor.read(cx).text(cx);

        filter_state.update(cx, |state, _cx| {
            state.set_filter_texts(include_text.clone(), exclude_text.clone());
        });

        let path_style = editor
            .read(cx)
            .project()
            .map(|project| project.read(cx).path_style(cx))
            .unwrap_or(PathStyle::local());

        let include_matcher = if include_text.trim().is_empty() {
            None
        } else {
            parse_path_matches(&include_text, path_style).ok()
        };

        let exclude_matcher = if exclude_text.trim().is_empty() {
            None
        } else {
            parse_path_matches(&exclude_text, path_style).ok()
        };

        let filtered_locations = filter_state
            .read(cx)
            .filter_locations(include_matcher.as_ref(), exclude_matcher.as_ref());

        self.rebuild_multibuffer(&editor, filter_state, filtered_locations, cx);
    }

    fn rebuild_multibuffer(
        &self,
        editor: &Entity<Editor>,
        filter_state: Entity<FilterableMultibufferState>,
        locations: HashMap<Entity<Buffer>, Vec<Range<Point>>>,
        cx: &mut Context<Self>,
    ) {
        editor.update(cx, |editor, cx| {
            editor.rebuild_multibuffer_from_locations(
                locations,
                filter_state.read(cx).title().to_string(),
                cx,
            );
        });
    }

    fn toggle_filters_internal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.filters_enabled = !self.filters_enabled;

        if let Some(filter_state) = self.filter_state.as_ref() {
            let include_text = self.include_editor.read(cx).text(cx);
            let exclude_text = self.exclude_editor.read(cx).text(cx);
            filter_state.update(cx, |state, _cx| {
                state.set_filters_enabled(self.filters_enabled);
                state.set_filter_texts(include_text, exclude_text);
            });
        }

        if !self.filters_enabled {
            // Reset filters when disabling - show all original locations
            if let Some(filter_state) = self.filter_state.as_ref() {
                if let Some(editor) = self.active_editor.as_ref().and_then(|e| e.upgrade()) {
                    let locations = filter_state.read(cx).original_locations().clone();
                    let title = filter_state.read(cx).title().to_string();
                    editor.update(cx, |editor, cx| {
                        editor.rebuild_multibuffer_from_locations(locations, title, cx);
                    });
                }
            }
        } else {
            // Apply current filter text
            self.apply_filters(cx);
        }

        cx.emit(ToolbarItemEvent::ChangeLocation(
            self.determine_toolbar_location(cx),
        ));
        window.refresh();
        cx.notify();
    }

    fn determine_toolbar_location(&self, _cx: &App) -> ToolbarItemLocation {
        if self.filter_state.is_some() {
            if self.filters_enabled {
                ToolbarItemLocation::Secondary
            } else {
                ToolbarItemLocation::PrimaryRight
            }
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    pub(crate) fn has_filterable_editor(&self) -> bool {
        self.filter_state.is_some()
    }

    pub(crate) fn toggle_filters(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.toggle_filters_internal(window, cx);
    }

    pub(crate) fn focus_include_editor(&self, window: &mut Window, cx: &mut App) {
        let handle = self.include_editor.focus_handle(cx);
        handle.focus(window, cx);
    }

    pub(crate) fn focus_exclude_editor(&self, window: &mut Window, cx: &mut App) {
        let handle = self.exclude_editor.focus_handle(cx);
        handle.focus(window, cx);
    }

    fn sync_from_state(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(filter_state) = self.filter_state.as_ref() else {
            self.filters_enabled = false;
            return;
        };

        let include_text = filter_state.read(cx).include_text().to_string();
        let exclude_text = filter_state.read(cx).exclude_text().to_string();
        self.filters_enabled = filter_state.read(cx).filters_enabled();

        let current_include = self.include_editor.read(cx).text(cx);
        if current_include != include_text {
            self.include_editor.update(cx, |editor, cx| {
                editor.set_text(include_text, window, cx);
            });
        }

        let current_exclude = self.exclude_editor.read(cx).text(cx);
        if current_exclude != exclude_text {
            self.exclude_editor.update(cx, |editor, cx| {
                editor.set_text(exclude_text, window, cx);
            });
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for MultibufferFilterBar {}

impl ToolbarItemView for MultibufferFilterBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.active_editor = None;
        self.filter_state = None;
        self.filters_enabled = false;

        let Some(pane_item) = active_pane_item else {
            return ToolbarItemLocation::Hidden;
        };

        // Only show for multibuffer editors (not singleton buffers)
        if pane_item.buffer_kind(cx) == ItemBufferKind::Singleton {
            return ToolbarItemLocation::Hidden;
        }

        let Some(editor) = pane_item.downcast::<Editor>() else {
            return ToolbarItemLocation::Hidden;
        };

        let filter_state = editor.read(cx).filterable_state.clone();

        self.active_editor = Some(editor.downgrade());
        self.filter_state = filter_state;

        self.sync_from_state(window, cx);

        if self.filters_enabled {
            self.apply_filters(cx);
        }

        self.determine_toolbar_location(cx)
    }
}

impl Focusable for MultibufferFilterBar {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.include_editor.focus_handle(cx)
    }
}

impl Render for MultibufferFilterBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.filter_state.is_none() {
            return div().into_any_element();
        }

        if !self.filters_enabled {
            return h_flex()
                .child(
                    IconButton::new("open-filters", IconName::Filter)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.toggle_filters(window, cx);
                        }))
                        .tooltip(Tooltip::text("Show Filters")),
                )
                .into_any_element();
        }

        let theme = cx.theme();
        let border_color = theme.colors().border;

        h_flex()
            .w_full()
            .gap_2()
            .px_2()
            .py_1()
            .bg(theme.colors().toolbar_background)
            .border_b_1()
            .border_color(border_color)
            .child(
                Label::new("Filters:")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                h_flex()
                    .flex_1()
                    .gap_2()
                    .child(filter_input(
                        &self.include_editor,
                        "Include",
                        border_color,
                        cx,
                    ))
                    .child(filter_input(
                        &self.exclude_editor,
                        "Exclude",
                        border_color,
                        cx,
                    )),
            )
            .child(
                IconButton::new("close-filters", IconName::Close)
                    .shape(IconButtonShape::Square)
                    .icon_size(IconSize::Small)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toggle_filters(window, cx);
                    }))
                    .tooltip(Tooltip::text("Close Filters")),
            )
            .into_any_element()
    }
}

fn filter_input(
    editor: &Entity<Editor>,
    label: &'static str,
    border_color: gpui::Hsla,
    cx: &App,
) -> impl IntoElement {
    h_flex()
        .flex_1()
        .min_w_32()
        .h_7()
        .px_2()
        .border_1()
        .border_color(border_color)
        .rounded_md()
        .bg(cx.theme().colors().editor_background)
        .child(
            Label::new(label)
                .size(LabelSize::Small)
                .color(Color::Muted)
                .mr_1(),
        )
        .child(render_filter_editor(editor, cx))
}

fn render_filter_editor(editor: &Entity<Editor>, cx: &App) -> impl IntoElement {
    let settings = ThemeSettings::get_global(cx);
    let text_style = TextStyle {
        color: cx.theme().colors().text,
        font_family: settings.buffer_font.family.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_fallbacks: settings.buffer_font.fallbacks.clone(),
        font_size: rems(0.875).into(),
        font_weight: settings.buffer_font.weight,
        line_height: relative(1.3),
        ..TextStyle::default()
    };

    let editor_style = EditorStyle {
        background: cx.theme().colors().editor_background,
        local_player: cx.theme().players().local(),
        text: text_style,
        ..EditorStyle::default()
    };

    crate::EditorElement::new(editor, editor_style)
}

fn parse_path_matches(text: &str, path_style: PathStyle) -> anyhow::Result<PathMatcher> {
    let queries = text
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    Ok(PathMatcher::new(&queries, path_style)?)
}
