use editor::{Anchor, AnchorRangeExt, Editor, EditorSettings};
use fuzzy::StringMatch;
use gpui::{
    action,
    elements::*,
    keymap::{
        self,
        menu::{SelectNext, SelectPrev},
        Binding,
    },
    AppContext, Axis, Entity, MutableAppContext, RenderContext, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use language::Outline;
use ordered_float::OrderedFloat;
use postage::watch;
use std::{
    cmp::{self, Reverse},
    sync::Arc,
};
use workspace::{Settings, Workspace};

action!(Toggle);
action!(Confirm);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("cmd-shift-O", Toggle, Some("Editor")),
        Binding::new("escape", Toggle, Some("OutlineView")),
        Binding::new("enter", Confirm, Some("OutlineView")),
    ]);
    cx.add_action(OutlineView::toggle);
    cx.add_action(OutlineView::confirm);
    cx.add_action(OutlineView::select_prev);
    cx.add_action(OutlineView::select_next);
}

struct OutlineView {
    handle: WeakViewHandle<Self>,
    editor: ViewHandle<Editor>,
    outline: Outline<Anchor>,
    selected_match_index: usize,
    matches: Vec<StringMatch>,
    query_editor: ViewHandle<Editor>,
    list_state: UniformListState,
    settings: watch::Receiver<Settings>,
}

impl Entity for OutlineView {
    type Event = ();
}

impl View for OutlineView {
    fn ui_name() -> &'static str {
        "OutlineView"
    }

    fn keymap_context(&self, _: &AppContext) -> keymap::Context {
        let mut cx = Self::default_keymap_context();
        cx.set.insert("menu".into());
        cx
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        let settings = self.settings.borrow();

        Align::new(
            ConstrainedBox::new(
                Container::new(
                    Flex::new(Axis::Vertical)
                        .with_child(
                            Container::new(ChildView::new(self.query_editor.id()).boxed())
                                .with_style(settings.theme.selector.input_editor.container)
                                .boxed(),
                        )
                        .with_child(Flexible::new(1.0, false, self.render_matches()).boxed())
                        .boxed(),
                )
                .with_style(settings.theme.selector.container)
                .boxed(),
            )
            .with_max_width(500.0)
            .with_max_height(420.0)
            .boxed(),
        )
        .top()
        .named("outline view")
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_editor);
    }
}

impl OutlineView {
    fn new(
        outline: Outline<Anchor>,
        editor: ViewHandle<Editor>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::single_line(
                {
                    let settings = settings.clone();
                    Arc::new(move |_| {
                        let settings = settings.borrow();
                        EditorSettings {
                            style: settings.theme.selector.input_editor.as_editor(),
                            tab_size: settings.tab_size,
                            soft_wrap: editor::SoftWrap::None,
                        }
                    })
                },
                cx,
            )
        });
        cx.subscribe(&query_editor, Self::on_query_editor_event)
            .detach();
        let mut this = Self {
            handle: cx.weak_handle(),
            editor,
            matches: Default::default(),
            selected_match_index: 0,
            outline,
            query_editor,
            list_state: Default::default(),
            settings,
        };
        this.update_matches(cx);
        this
    }

    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        let editor = workspace
            .active_item(cx)
            .unwrap()
            .to_any()
            .downcast::<Editor>()
            .unwrap();
        let buffer = editor.read(cx).buffer().read(cx).read(cx).outline();
        if let Some(outline) = buffer {
            workspace.toggle_modal(cx, |cx, workspace| {
                cx.add_view(|cx| OutlineView::new(outline, editor, workspace.settings(), cx))
            })
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if self.selected_match_index > 0 {
            self.selected_match_index -= 1;
            self.list_state.scroll_to(self.selected_match_index);
            cx.notify();
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if self.selected_match_index + 1 < self.matches.len() {
            self.selected_match_index += 1;
            self.list_state.scroll_to(self.selected_match_index);
            cx.notify();
        }
    }

    fn confirm(&mut self, _: &Confirm, _: &mut ViewContext<Self>) {}

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::Edited => self.update_matches(cx),
            _ => {}
        }
    }

    fn update_matches(&mut self, cx: &mut ViewContext<Self>) {
        let query = self.query_editor.update(cx, |buffer, cx| buffer.text(cx));
        if query.is_empty() {
            self.matches = self
                .outline
                .items
                .iter()
                .enumerate()
                .map(|(index, _)| StringMatch {
                    candidate_index: index,
                    score: Default::default(),
                    positions: Default::default(),
                    string: Default::default(),
                })
                .collect();

            let editor = self.editor.read(cx);
            let buffer = editor.buffer().read(cx).read(cx);
            let cursor_offset = editor.newest_selection::<usize>(&buffer).head();
            self.selected_match_index = self
                .outline
                .items
                .iter()
                .enumerate()
                .map(|(ix, item)| {
                    let range = item.range.to_offset(&buffer);
                    let distance_to_closest_endpoint = cmp::min(
                        (range.start as isize - cursor_offset as isize).abs() as usize,
                        (range.end as isize - cursor_offset as isize).abs() as usize,
                    );
                    let depth = if range.contains(&cursor_offset) {
                        Some(item.depth)
                    } else {
                        None
                    };
                    (ix, depth, distance_to_closest_endpoint)
                })
                .max_by_key(|(_, depth, distance)| (*depth, Reverse(*distance)))
                .unwrap()
                .0;
        } else {
            self.matches = self.outline.search(&query, cx);
            self.selected_match_index = self
                .matches
                .iter()
                .enumerate()
                .max_by_key(|(_, m)| OrderedFloat(m.score))
                .map(|(ix, _)| ix)
                .unwrap_or(0);
        }

        self.list_state.scroll_to(self.selected_match_index);
        cx.notify();
    }

    fn render_matches(&self) -> ElementBox {
        if self.matches.is_empty() {
            let settings = self.settings.borrow();
            return Container::new(
                Label::new(
                    "No matches".into(),
                    settings.theme.selector.empty.label.clone(),
                )
                .boxed(),
            )
            .with_style(settings.theme.selector.empty.container)
            .named("empty matches");
        }

        let handle = self.handle.clone();
        let list = UniformList::new(
            self.list_state.clone(),
            self.matches.len(),
            move |mut range, items, cx| {
                let cx = cx.as_ref();
                let view = handle.upgrade(cx).unwrap();
                let view = view.read(cx);
                let start = range.start;
                range.end = cmp::min(range.end, view.matches.len());
                items.extend(
                    view.matches[range]
                        .iter()
                        .enumerate()
                        .map(move |(ix, m)| view.render_match(m, start + ix)),
                );
            },
        );

        Container::new(list.boxed())
            .with_margin_top(6.0)
            .named("matches")
    }

    fn render_match(&self, string_match: &StringMatch, index: usize) -> ElementBox {
        let settings = self.settings.borrow();
        let style = if index == self.selected_match_index {
            &settings.theme.selector.active_item
        } else {
            &settings.theme.selector.item
        };
        let outline_match = &self.outline.items[string_match.candidate_index];

        Label::new(outline_match.text.clone(), style.label.clone())
            .with_highlights(string_match.positions.clone())
            .contained()
            .with_padding_left(20. * outline_match.depth as f32)
            .contained()
            .with_style(style.container)
            .boxed()
    }
}
