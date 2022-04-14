use editor::Editor;
use gpui::{
    elements::{
        ChildView, Flex, FlexItem, Label, ParentElement, ScrollTarget, UniformList,
        UniformListState,
    },
    keymap, AppContext, Axis, Element, ElementBox, Entity, MutableAppContext, RenderContext, Task,
    View, ViewContext, ViewHandle, WeakViewHandle,
};
use settings::Settings;
use std::cmp;
use workspace::menu::{Cancel, Confirm, SelectFirst, SelectLast, SelectNext, SelectPrev};

pub struct Picker<D: PickerDelegate> {
    delegate: WeakViewHandle<D>,
    query_editor: ViewHandle<Editor>,
    list_state: UniformListState,
}

pub trait PickerDelegate: View {
    fn match_count(&self) -> usize;
    fn selected_index(&self) -> usize;
    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Self>);
    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Self>) -> Task<()>;
    fn confirm(&mut self, cx: &mut ViewContext<Self>);
    fn dismiss(&mut self, cx: &mut ViewContext<Self>);
    fn render_match(&self, ix: usize, selected: bool, cx: &AppContext) -> ElementBox;
}

impl<D: PickerDelegate> Entity for Picker<D> {
    type Event = ();
}

impl<D: PickerDelegate> View for Picker<D> {
    fn ui_name() -> &'static str {
        "Picker"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> gpui::ElementBox {
        let settings = cx.global::<Settings>();
        Flex::new(Axis::Vertical)
            .with_child(
                ChildView::new(&self.query_editor)
                    .contained()
                    .with_style(settings.theme.selector.input_editor.container)
                    .boxed(),
            )
            .with_child(
                FlexItem::new(self.render_matches(cx))
                    .flex(1., false)
                    .boxed(),
            )
            .contained()
            .with_style(settings.theme.selector.container)
            .constrained()
            .with_max_width(500.0)
            .with_max_height(420.0)
            .aligned()
            .top()
            .named("picker")
    }

    fn keymap_context(&self, _: &AppContext) -> keymap::Context {
        let mut cx = Self::default_keymap_context();
        cx.set.insert("menu".into());
        cx
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_editor);
    }
}

impl<D: PickerDelegate> Picker<D> {
    pub fn init(cx: &mut MutableAppContext) {
        cx.add_action(Self::select_first);
        cx.add_action(Self::select_last);
        cx.add_action(Self::select_next);
        cx.add_action(Self::select_prev);
        cx.add_action(Self::confirm);
        cx.add_action(Self::cancel);
    }

    pub fn new(delegate: WeakViewHandle<D>, cx: &mut ViewContext<Self>) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::single_line(Some(|theme| theme.selector.input_editor.clone()), cx)
        });
        cx.subscribe(&query_editor, Self::on_query_editor_event)
            .detach();

        Self {
            delegate,
            query_editor,
            list_state: Default::default(),
        }
    }

    fn render_matches(&self, cx: &AppContext) -> ElementBox {
        let delegate = self.delegate.clone();
        let match_count = if let Some(delegate) = delegate.upgrade(cx) {
            delegate.read(cx).match_count()
        } else {
            0
        };

        if match_count == 0 {
            let settings = cx.global::<Settings>();
            return Label::new(
                "No matches".into(),
                settings.theme.selector.empty.label.clone(),
            )
            .contained()
            .with_style(settings.theme.selector.empty.container)
            .named("empty matches");
        }

        UniformList::new(
            self.list_state.clone(),
            match_count,
            move |mut range, items, cx| {
                let cx = cx.as_ref();
                let delegate = delegate.upgrade(cx).unwrap();
                let delegate = delegate.read(cx);
                let selected_ix = delegate.selected_index();
                range.end = cmp::min(range.end, delegate.match_count());
                items.extend(range.map(move |ix| delegate.render_match(ix, ix == selected_ix, cx)));
            },
        )
        .contained()
        .with_margin_top(6.0)
        .named("matches")
    }

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(delegate) = self.delegate.upgrade(cx) {
            match event {
                editor::Event::BufferEdited { .. } => {
                    let query = self.query_editor.read(cx).text(cx);
                    let update = delegate.update(cx, |d, cx| d.update_matches(query, cx));
                    cx.spawn(|this, mut cx| async move {
                        update.await;
                        this.update(&mut cx, |_, cx| cx.notify());
                    })
                    .detach();
                }
                editor::Event::Blurred => delegate.update(cx, |delegate, cx| {
                    delegate.dismiss(cx);
                }),
                _ => {}
            }
        }
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        if let Some(delegate) = self.delegate.upgrade(cx) {
            let index = 0;
            delegate.update(cx, |delegate, cx| delegate.set_selected_index(0, cx));
            self.list_state.scroll_to(ScrollTarget::Show(index));
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        if let Some(delegate) = self.delegate.upgrade(cx) {
            let index = delegate.update(cx, |delegate, cx| {
                let match_count = delegate.match_count();
                let index = if match_count > 0 { match_count - 1 } else { 0 };
                delegate.set_selected_index(index, cx);
                index
            });
            self.list_state.scroll_to(ScrollTarget::Show(index));
            cx.notify();
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(delegate) = self.delegate.upgrade(cx) {
            let index = delegate.update(cx, |delegate, cx| {
                let mut selected_index = delegate.selected_index();
                if selected_index + 1 < delegate.match_count() {
                    selected_index += 1;
                    delegate.set_selected_index(selected_index, cx);
                }
                selected_index
            });
            self.list_state.scroll_to(ScrollTarget::Show(index));
            cx.notify();
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(delegate) = self.delegate.upgrade(cx) {
            let index = delegate.update(cx, |delegate, cx| {
                let mut selected_index = delegate.selected_index();
                if selected_index > 0 {
                    selected_index -= 1;
                    delegate.set_selected_index(selected_index, cx);
                }
                selected_index
            });
            self.list_state.scroll_to(ScrollTarget::Show(index));
            cx.notify();
        }
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(delegate) = self.delegate.upgrade(cx) {
            delegate.update(cx, |delegate, cx| delegate.confirm(cx));
        }
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        if let Some(delegate) = self.delegate.upgrade(cx) {
            delegate.update(cx, |delegate, cx| delegate.dismiss(cx));
        }
    }
}
