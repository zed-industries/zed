use aho_corasick::AhoCorasickBuilder;
use editor::{Editor, EditorSettings};
use gpui::{
    action, elements::*, keymap::Binding, Entity, MutableAppContext, RenderContext, View,
    ViewContext, ViewHandle,
};
use postage::watch;
use std::sync::Arc;
use workspace::{ItemViewHandle, Settings, Toolbar, Workspace};

action!(Deploy);
action!(Cancel);
action!(ToggleMode, SearchMode);

#[derive(Clone, Copy)]
pub enum SearchMode {
    WholeWord,
    CaseSensitive,
    Regex,
}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("cmd-f", Deploy, Some("Editor && mode == full")),
        Binding::new("escape", Cancel, Some("FindBar")),
    ]);
    cx.add_action(FindBar::deploy);
    cx.add_action(FindBar::cancel);
    cx.add_action(FindBar::toggle_mode);
}

struct FindBar {
    settings: watch::Receiver<Settings>,
    query_editor: ViewHandle<Editor>,
    active_editor: Option<ViewHandle<Editor>>,
    case_sensitive_mode: bool,
    whole_word_mode: bool,
    regex_mode: bool,
}

impl Entity for FindBar {
    type Event = ();
}

impl View for FindBar {
    fn ui_name() -> &'static str {
        "FindBar"
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_editor);
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &self.settings.borrow().theme.find;
        Flex::row()
            .with_child(
                ChildView::new(&self.query_editor)
                    .contained()
                    .with_style(theme.editor.input.container)
                    .constrained()
                    .with_max_width(theme.editor.max_width)
                    .boxed(),
            )
            .with_child(
                Flex::row()
                    .with_child(self.render_mode_button("Aa", SearchMode::CaseSensitive, theme, cx))
                    .with_child(self.render_mode_button("|ab|", SearchMode::WholeWord, theme, cx))
                    .with_child(self.render_mode_button(".*", SearchMode::Regex, theme, cx))
                    .contained()
                    .with_style(theme.mode_button_group)
                    .boxed(),
            )
            .contained()
            .with_style(theme.container)
            .boxed()
    }
}

impl Toolbar for FindBar {
    fn active_item_changed(
        &mut self,
        item: Option<Box<dyn ItemViewHandle>>,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        self.active_editor = item.and_then(|item| item.act_as::<Editor>(cx));
        self.active_editor.is_some()
    }
}

impl FindBar {
    fn new(settings: watch::Receiver<Settings>, cx: &mut ViewContext<Self>) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::single_line(
                {
                    let settings = settings.clone();
                    Arc::new(move |_| {
                        let settings = settings.borrow();
                        EditorSettings {
                            style: settings.theme.find.editor.input.as_editor(),
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
            query_editor,
            active_editor: None,
            case_sensitive_mode: false,
            whole_word_mode: false,
            regex_mode: false,
            settings,
        }
    }

    fn render_mode_button(
        &self,
        icon: &str,
        mode: SearchMode,
        theme: &theme::Find,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let is_active = self.is_mode_enabled(mode);
        MouseEventHandler::new::<Self, _, _, _>(mode as usize, cx, |state, _| {
            let style = match (is_active, state.hovered) {
                (false, false) => &theme.mode_button,
                (false, true) => &theme.hovered_mode_button,
                (true, false) => &theme.active_mode_button,
                (true, true) => &theme.active_hovered_mode_button,
            };
            Label::new(icon.to_string(), style.text.clone())
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .on_click(move |cx| cx.dispatch_action(ToggleMode(mode)))
        .boxed()
    }

    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        let settings = workspace.settings();
        workspace.active_pane().update(cx, |pane, cx| {
            pane.show_toolbar(cx, |cx| FindBar::new(settings, cx));
            if let Some(toolbar) = pane.active_toolbar() {
                cx.focus(toolbar);
            }
        });
    }

    fn cancel(workspace: &mut Workspace, _: &Cancel, cx: &mut ViewContext<Workspace>) {
        workspace
            .active_pane()
            .update(cx, |pane, cx| pane.hide_toolbar(cx));
    }

    fn is_mode_enabled(&self, mode: SearchMode) -> bool {
        match mode {
            SearchMode::WholeWord => self.whole_word_mode,
            SearchMode::CaseSensitive => self.case_sensitive_mode,
            SearchMode::Regex => self.regex_mode,
        }
    }

    fn toggle_mode(&mut self, ToggleMode(mode): &ToggleMode, cx: &mut ViewContext<Self>) {
        eprintln!("TOGGLE MODE");
        let value = match mode {
            SearchMode::WholeWord => &mut self.whole_word_mode,
            SearchMode::CaseSensitive => &mut self.case_sensitive_mode,
            SearchMode::Regex => &mut self.regex_mode,
        };
        *value = !*value;
        cx.notify();
    }

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        _: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(editor) = &self.active_editor {
            let search = self.query_editor.read(cx).text(cx);
            let theme = &self.settings.borrow().theme.find;
            editor.update(cx, |editor, cx| {
                if search.is_empty() {
                    editor.clear_highlighted_ranges::<Self>(cx);
                    return;
                }

                let search = AhoCorasickBuilder::new()
                    .auto_configure(&[&search])
                    .ascii_case_insensitive(!self.case_sensitive_mode)
                    .build(&[&search]);
                let buffer = editor.buffer().read(cx).snapshot(cx);
                let ranges = search
                    .stream_find_iter(buffer.bytes_in_range(0..buffer.len()))
                    .map(|mat| {
                        let mat = mat.unwrap();
                        buffer.anchor_after(mat.start())..buffer.anchor_before(mat.end())
                    })
                    .collect();
                editor.highlight_ranges::<Self>(ranges, theme.match_background, cx);
            });
        }
    }
}
