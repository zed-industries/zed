use std::marker::PhantomData;
use std::path::PathBuf;
use std::str::FromStr;

use crate::prelude::*;
use crate::{v_stack, Breadcrumb, Buffer, HighlightedText, Icon, IconButton, Tab, TabBar, Toolbar};

pub struct Editor {
    pub tabs: Vec<Tab>,
    pub path: Vec<String>,
    pub symbols: Vec<HighlightedText>,
    pub buffer: Buffer,
}

#[derive(Element)]
pub struct EditorPane<V: 'static> {
    view_type: PhantomData<V>,
    editor: Editor,
    // toolbar: Toolbar,
    // buffer: Buffer<V>,
}

impl<V: 'static> EditorPane<V> {
    pub fn new(editor: Editor) -> Self {
        Self {
            view_type: PhantomData,
            editor,
            // toolbar,
            // buffer,
        }
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        v_stack()
            .w_full()
            .h_full()
            .flex_1()
            .child(TabBar::new(self.editor.tabs.clone()))
            .child(Toolbar::new(
                |_, _| {
                    vec![Breadcrumb::new(
                        PathBuf::from_str("crates/ui/src/components/toolbar.rs").unwrap(),
                    )
                    .into_any()]
                },
                Box::new(()),
                |_, _| {
                    vec![
                        IconButton::new(Icon::InlayHint).into_any(),
                        IconButton::new(Icon::MagnifyingGlass).into_any(),
                        IconButton::new(Icon::MagicWand).into_any(),
                    ]
                },
                Box::new(()),
            ))
            .child(self.editor.buffer.clone())
    }
}
