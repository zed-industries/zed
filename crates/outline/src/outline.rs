use editor::{display_map::ToDisplayPoint, Autoscroll, Editor, EditorSettings};
use gpui::{
    action, elements::*, geometry::vector::Vector2F, keymap::Binding, Axis, Entity,
    MutableAppContext, RenderContext, View, ViewContext, ViewHandle, WeakViewHandle,
};
use language::{Outline, OutlineItem};
use postage::watch;
use std::{cmp, sync::Arc};
use text::{Bias, Point, Selection};
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
}

struct OutlineView {
    handle: WeakViewHandle<Self>,
    outline: Outline,
    matches: Vec<OutlineItem>,
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

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
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
        outline: Outline,
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
        Self {
            handle: cx.weak_handle(),
            matches: outline.0.clone(),
            outline,
            query_editor,
            list_state: Default::default(),
            settings,
        }
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
                cx.add_view(|cx| OutlineView::new(outline, workspace.settings(), cx))
            })
        }
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {}

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::Edited => {
                let query = self.query_editor.update(cx, |buffer, cx| buffer.text(cx));
            }
            _ => {}
        }
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
        let list =
            UniformList::new(
                self.list_state.clone(),
                self.matches.len(),
                move |mut range, items, cx| {
                    let cx = cx.as_ref();
                    let view = handle.upgrade(cx).unwrap();
                    let view = view.read(cx);
                    let start = range.start;
                    range.end = cmp::min(range.end, view.matches.len());
                    items.extend(view.matches[range].iter().enumerate().map(
                        move |(i, outline_match)| view.render_match(outline_match, start + i),
                    ));
                },
            );

        Container::new(list.boxed())
            .with_margin_top(6.0)
            .named("matches")
    }

    fn render_match(&self, outline_match: &OutlineItem, index: usize) -> ElementBox {
        // TODO: maintain selected index.
        let selected_index = 0;
        let settings = self.settings.borrow();
        let style = if index == selected_index {
            &settings.theme.selector.active_item
        } else {
            &settings.theme.selector.item
        };

        Label::new(outline_match.text.clone(), style.label.clone())
            .contained()
            .with_padding_left(20. * outline_match.depth as f32)
            .contained()
            .with_style(style.container)
            .boxed()
    }
}
