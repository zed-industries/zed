use std::{cmp, sync::Arc};

use crate::{
    editor::{self, Editor},
    fuzzy::{match_strings, StringMatch, StringMatchCandidate},
    settings::ThemeRegistry,
    workspace::Workspace,
    AppState, Settings,
};
use gpui::{
    elements::{
        Align, ChildView, ConstrainedBox, Container, Expanded, Flex, Label, ParentElement,
        UniformList, UniformListState,
    },
    keymap::{self, Binding},
    AppContext, Axis, Element, ElementBox, Entity, MutableAppContext, RenderContext, View,
    ViewContext, ViewHandle,
};
use parking_lot::Mutex;
use postage::watch;

pub struct ThemeSelector {
    settings_tx: Arc<Mutex<watch::Sender<Settings>>>,
    settings: watch::Receiver<Settings>,
    registry: Arc<ThemeRegistry>,
    matches: Vec<StringMatch>,
    query_buffer: ViewHandle<Editor>,
    list_state: UniformListState,
    selected_index: usize,
}

pub fn init(cx: &mut MutableAppContext, app_state: &Arc<AppState>) {
    cx.add_action("theme_selector:confirm", ThemeSelector::confirm);
    cx.add_action("menu:select_prev", ThemeSelector::select_prev);
    cx.add_action("menu:select_next", ThemeSelector::select_next);
    cx.add_action("theme_selector:toggle", ThemeSelector::toggle);
    cx.add_action("theme_selector:reload", ThemeSelector::reload);

    cx.add_bindings(vec![
        Binding::new("cmd-k cmd-t", "theme_selector:toggle", None).with_arg(app_state.clone()),
        Binding::new("cmd-k t", "theme_selector:reload", None).with_arg(app_state.clone()),
        Binding::new("escape", "theme_selector:toggle", Some("ThemeSelector"))
            .with_arg(app_state.clone()),
        Binding::new("enter", "theme_selector:confirm", Some("ThemeSelector")),
    ]);
}

pub enum Event {
    Dismissed,
}

impl ThemeSelector {
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
            let selector = cx.add_view(|cx| {
                Self::new(
                    app_state.settings_tx.clone(),
                    app_state.settings.clone(),
                    app_state.themes.clone(),
                    cx,
                )
            });
            cx.subscribe_to_view(&selector, Self::on_event);
            selector
        });
    }

    fn reload(_: &mut Workspace, app_state: &Arc<AppState>, cx: &mut ViewContext<Workspace>) {
        let current_theme_name = app_state.settings.borrow().theme.name.clone();
        app_state.themes.clear();
        match app_state.themes.get(&current_theme_name) {
            Ok(theme) => {
                cx.notify_all();
                app_state.settings_tx.lock().borrow_mut().theme = theme;
            }
            Err(error) => {
                log::error!("failed to load theme {}: {:?}", current_theme_name, error)
            }
        }
    }

    fn confirm(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            if let Ok(theme) = self.registry.get(&mat.string) {
                self.settings_tx.lock().borrow_mut().theme = theme;
                cx.notify_all();
                cx.emit(Event::Dismissed);
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
        _: ViewHandle<ThemeSelector>,
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
                .with_style(&settings.theme.ui.selector.label)
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
                let selector = handle.upgrade(cx).unwrap();
                let selector = selector.read(cx);
                let start = range.start;
                range.end = cmp::min(range.end, selector.matches.len());
                items.extend(
                    selector.matches[range]
                        .iter()
                        .enumerate()
                        .map(move |(i, path_match)| selector.render_match(path_match, start + i)),
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

        let container = Container::new(
            Label::new(
                theme_match.string.clone(),
                settings.ui_font_family,
                settings.ui_font_size,
            )
            .with_style(if index == self.selected_index {
                &theme.selector.active_item.label
            } else {
                &theme.selector.item.label
            })
            .with_highlights(theme_match.positions.clone())
            .boxed(),
        )
        .with_style(if index == self.selected_index {
            &theme.selector.active_item.container
        } else {
            &theme.selector.item.container
        });

        container.boxed()
    }
}

impl Entity for ThemeSelector {
    type Event = Event;
}

impl View for ThemeSelector {
    fn ui_name() -> &'static str {
        "ThemeSelector"
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
                .with_style(&settings.theme.ui.selector.container)
                .boxed(),
            )
            .with_max_width(600.0)
            .with_max_height(400.0)
            .boxed(),
        )
        .top()
        .named("theme selector")
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
