use std::{cmp, sync::Arc};

use crate::{
    editor::{self, Editor},
    settings::ThemeRegistry,
    workspace::Workspace,
    worktree::fuzzy::{match_strings, StringMatch, StringMatchCandidate},
    AppState, Settings,
};
use futures::lock::Mutex;
use gpui::{
    color::ColorF,
    elements::{
        Align, ChildView, ConstrainedBox, Container, Expanded, Flex, Label, ParentElement,
        UniformList, UniformListState,
    },
    fonts::{Properties, Weight},
    geometry::vector::vec2f,
    keymap::{self, Binding},
    AppContext, Axis, Border, Element, ElementBox, Entity, MutableAppContext, RenderContext, View,
    ViewContext, ViewHandle,
};
use postage::watch;

pub struct ThemePicker {
    settings_tx: Arc<Mutex<watch::Sender<Settings>>>,
    settings: watch::Receiver<Settings>,
    registry: Arc<ThemeRegistry>,
    matches: Vec<StringMatch>,
    query_buffer: ViewHandle<Editor>,
    list_state: UniformListState,
    selected_index: usize,
}

pub fn init(cx: &mut MutableAppContext, app_state: &Arc<AppState>) {
    cx.add_action("theme_picker:confirm", ThemePicker::confirm);
    // cx.add_action("file_finder:select", ThemePicker::select);
    cx.add_action("menu:select_prev", ThemePicker::select_prev);
    cx.add_action("menu:select_next", ThemePicker::select_next);
    cx.add_action("theme_picker:toggle", ThemePicker::toggle);

    cx.add_bindings(vec![
        Binding::new("cmd-k cmd-t", "theme_picker:toggle", None).with_arg(app_state.clone()),
        Binding::new("escape", "theme_picker:toggle", Some("ThemePicker"))
            .with_arg(app_state.clone()),
        Binding::new("enter", "theme_picker:confirm", Some("ThemePicker")),
    ]);
}

pub enum Event {
    Dismissed,
}

impl ThemePicker {
    fn new(
        settings_tx: Arc<Mutex<watch::Sender<Settings>>>,
        settings: watch::Receiver<Settings>,
        registry: Arc<ThemeRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let query_buffer = cx.add_view(|cx| Editor::single_line(settings.clone(), cx));
        cx.subscribe_to_view(&query_buffer, Self::on_query_editor_event);

        let mut this = Self {
            settings,
            settings_tx,
            registry,
            query_buffer,
            matches: Vec::new(),
            list_state: Default::default(),
            selected_index: 0,
        };
        this.update_matches(cx);
        this
    }

    fn toggle(
        workspace: &mut Workspace,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) {
        workspace.toggle_modal(cx, |cx, _| {
            let picker = cx.add_view(|cx| {
                Self::new(
                    app_state.settings_tx.clone(),
                    app_state.settings.clone(),
                    app_state.themes.clone(),
                    cx,
                )
            });
            cx.subscribe_to_view(&picker, Self::on_event);
            picker
        });
    }

    fn confirm(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            if let Ok(theme) = self.registry.get(&mat.string) {
                let settings_tx = self.settings_tx.clone();
                cx.spawn(|this, mut cx| async move {
                    let mut settings_tx = settings_tx.lock().await;
                    this.update(&mut cx, |_, cx| {
                        settings_tx.borrow_mut().theme = theme;
                        cx.notify_all();
                        cx.emit(Event::Dismissed);
                    })
                })
                .detach();
            }
        }
    }

    fn select_prev(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
        self.list_state.scroll_to(self.selected_index);
        cx.notify();
    }

    fn select_next(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        if self.selected_index + 1 < self.matches.len() {
            self.selected_index += 1;
        }
        self.list_state.scroll_to(self.selected_index);
        cx.notify();
    }

    // fn select(&mut self, selected_index: &usize, cx: &mut ViewContext<Self>) {
    //     self.selected_index = *selected_index;
    //     self.confirm(&(), cx);
    // }

    fn update_matches(&mut self, cx: &mut ViewContext<Self>) {
        let background = cx.background().clone();
        let candidates = self
            .registry
            .list()
            .map(|name| StringMatchCandidate {
                char_bag: name.as_str().into(),
                string: name,
            })
            .collect::<Vec<_>>();
        let query = self.query_buffer.update(cx, |buffer, cx| buffer.text(cx));

        self.matches = if query.is_empty() {
            candidates
                .into_iter()
                .map(|candidate| StringMatch {
                    string: candidate.string,
                    positions: Vec::new(),
                    score: 0.0,
                })
                .collect()
        } else {
            smol::block_on(match_strings(
                &candidates,
                &query,
                false,
                100,
                &Default::default(),
                background,
            ))
        };
    }

    fn on_event(
        workspace: &mut Workspace,
        _: ViewHandle<ThemePicker>,
        event: &Event,
        cx: &mut ViewContext<Workspace>,
    ) {
        match event {
            Event::Dismissed => {
                workspace.dismiss_modal(cx);
            }
        }
    }

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::Edited => self.update_matches(cx),
            editor::Event::Blurred => cx.emit(Event::Dismissed),
            _ => {}
        }
    }

    fn render_matches(&self, cx: &RenderContext<Self>) -> ElementBox {
        if self.matches.is_empty() {
            let settings = self.settings.borrow();
            return Container::new(
                Label::new(
                    "No matches".into(),
                    settings.ui_font_family,
                    settings.ui_font_size,
                )
                .with_default_color(settings.theme.editor.default_text.0)
                .boxed(),
            )
            .with_margin_top(6.0)
            .named("empty matches");
        }

        let handle = cx.handle();
        let list = UniformList::new(
            self.list_state.clone(),
            self.matches.len(),
            move |mut range, items, cx| {
                let cx = cx.as_ref();
                let picker = handle.upgrade(cx).unwrap();
                let picker = picker.read(cx);
                let start = range.start;
                range.end = cmp::min(range.end, picker.matches.len());
                items.extend(
                    picker.matches[range]
                        .iter()
                        .enumerate()
                        .map(move |(i, path_match)| picker.render_match(path_match, start + i)),
                );
            },
        );

        Container::new(list.boxed())
            .with_margin_top(6.0)
            .named("matches")
    }

    fn render_match(&self, theme_match: &StringMatch, index: usize) -> ElementBox {
        let settings = self.settings.borrow();
        let theme = &settings.theme.ui;
        let bold = *Properties::new().weight(Weight::BOLD);

        let mut container = Container::new(
            Label::new(
                theme_match.string.clone(),
                settings.ui_font_family,
                settings.ui_font_size,
            )
            .with_default_color(theme.modal_match_text.0)
            .with_highlights(
                theme.modal_match_text_highlight.0,
                bold,
                theme_match.positions.clone(),
            )
            .boxed(),
        )
        .with_uniform_padding(6.0)
        .with_background_color(if index == self.selected_index {
            theme.modal_match_background_active.0
        } else {
            theme.modal_match_background.0
        });

        if index == self.selected_index || index < self.matches.len() - 1 {
            container = container.with_border(Border::bottom(1.0, theme.modal_match_border));
        }

        container.boxed()
    }
}

impl Entity for ThemePicker {
    type Event = Event;
}

impl View for ThemePicker {
    fn ui_name() -> &'static str {
        "ThemePicker"
    }

    fn render(&self, cx: &RenderContext<Self>) -> ElementBox {
        let settings = self.settings.borrow();

        Align::new(
            ConstrainedBox::new(
                Container::new(
                    Flex::new(Axis::Vertical)
                        .with_child(ChildView::new(self.query_buffer.id()).boxed())
                        .with_child(Expanded::new(1.0, self.render_matches(cx)).boxed())
                        .boxed(),
                )
                .with_margin_top(12.0)
                .with_uniform_padding(6.0)
                .with_corner_radius(6.0)
                .with_background_color(settings.theme.ui.modal_background)
                .with_shadow(vec2f(0., 4.), 12., ColorF::new(0.0, 0.0, 0.0, 0.5).to_u8())
                .boxed(),
            )
            .with_max_width(600.0)
            .with_max_height(400.0)
            .boxed(),
        )
        .top()
        .named("theme picker")
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_buffer);
    }

    fn keymap_context(&self, _: &AppContext) -> keymap::Context {
        let mut cx = Self::default_keymap_context();
        cx.set.insert("menu".into());
        cx
    }
}
