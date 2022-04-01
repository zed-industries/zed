use editor::{Anchor, Editor};
use gpui::{
    elements::*, AppContext, Entity, RenderContext, Subscription, View, ViewContext, ViewHandle,
};
use language::{BufferSnapshot, OutlineItem};
use search::ProjectSearchView;
use std::borrow::Cow;
use theme::SyntaxTheme;
use workspace::{ItemHandle, Settings, ToolbarItemLocation, ToolbarItemView};

pub struct Breadcrumbs {
    editor: Option<ViewHandle<Editor>>,
    editor_subscription: Option<Subscription>,
}

impl Breadcrumbs {
    pub fn new() -> Self {
        Self {
            editor: Default::default(),
            editor_subscription: Default::default(),
        }
    }

    fn active_symbols(
        &self,
        theme: &SyntaxTheme,
        cx: &AppContext,
    ) -> Option<(BufferSnapshot, Vec<OutlineItem<Anchor>>)> {
        let editor = self.editor.as_ref()?.read(cx);
        let cursor = editor.newest_anchor_selection().head();
        let (buffer, symbols) = editor
            .buffer()
            .read(cx)
            .read(cx)
            .symbols_containing(cursor, Some(theme))?;
        if buffer.path().is_none() && symbols.is_empty() {
            None
        } else {
            Some((buffer, symbols))
        }
    }
}

impl Entity for Breadcrumbs {
    type Event = ();
}

impl View for Breadcrumbs {
    fn ui_name() -> &'static str {
        "Breadcrumbs"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();
        let (buffer, symbols) =
            if let Some((buffer, symbols)) = self.active_symbols(&theme.editor.syntax, cx) {
                (buffer, symbols)
            } else {
                return Empty::new().boxed();
            };

        let filename = if let Some(path) = buffer.path() {
            path.to_string_lossy()
        } else {
            Cow::Borrowed("untitled")
        };

        Flex::row()
            .with_child(Label::new(filename.to_string(), theme.breadcrumbs.text.clone()).boxed())
            .with_children(symbols.into_iter().flat_map(|symbol| {
                [
                    Label::new(" 〉 ".to_string(), theme.breadcrumbs.text.clone()).boxed(),
                    Text::new(symbol.text, theme.breadcrumbs.text.clone())
                        .with_highlights(symbol.highlight_ranges)
                        .boxed(),
                ]
            }))
            .contained()
            .with_style(theme.breadcrumbs.container)
            .aligned()
            .left()
            .boxed()
    }
}

impl ToolbarItemView for Breadcrumbs {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.editor_subscription = None;
        self.editor = None;
        if let Some(item) = active_pane_item {
            if let Some(editor) = item.act_as::<Editor>(cx) {
                self.editor_subscription =
                    Some(cx.subscribe(&editor, |_, _, event, cx| match event {
                        editor::Event::BufferEdited => cx.notify(),
                        editor::Event::SelectionsChanged { local } if *local => cx.notify(),
                        _ => {}
                    }));
                self.editor = Some(editor);
                if item.downcast::<ProjectSearchView>().is_some() {
                    ToolbarItemLocation::Secondary
                } else {
                    ToolbarItemLocation::PrimaryLeft
                }
            } else {
                ToolbarItemLocation::Hidden
            }
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}
