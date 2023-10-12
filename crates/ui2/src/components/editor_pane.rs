use std::path::PathBuf;

use crate::prelude::*;
use crate::{v_stack, Breadcrumb, Buffer, Icon, IconButton, Symbol, Tab, TabBar, Toolbar};

pub struct Editor<S: 'static + Send + Sync + Clone> {
    pub tabs: Vec<Tab<S>>,
    pub path: PathBuf,
    pub symbols: Vec<Symbol>,
    pub buffer: Buffer<S>,
}

#[derive(Element)]
pub struct EditorPane<S: 'static + Send + Sync + Clone> {
    editor: Editor<S>,
}

impl<S: 'static + Send + Sync + Clone> EditorPane<S> {
    pub fn new(editor: Editor<S>) -> Self {
        Self { editor }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        v_stack()
            .w_full()
            .h_full()
            .flex_1()
            .child(TabBar::new(self.editor.tabs.clone()))
            .child(
                Toolbar::new()
                    .left_item(Breadcrumb::new(
                        self.editor.path.clone(),
                        self.editor.symbols.clone(),
                    ))
                    .right_items(vec![
                        IconButton::new(Icon::InlayHint).into_any(),
                        IconButton::new(Icon::MagnifyingGlass).into_any(),
                        IconButton::new(Icon::MagicWand).into_any(),
                    ]),
            )
            .child(self.editor.buffer.clone())
    }
}
