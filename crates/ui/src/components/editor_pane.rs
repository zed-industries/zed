use std::marker::PhantomData;

use crate::prelude::*;
use crate::{v_stack, Buffer, HighlightedText, Tab, TabBar, Toolbar};

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
            .child(Toolbar::new())
            .child(self.editor.buffer.clone())
    }
}
