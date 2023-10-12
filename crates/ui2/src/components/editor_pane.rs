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

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        struct LeftItemsPayload {
            path: PathBuf,
            symbols: Vec<Symbol>,
        }

        v_stack()
            .w_full()
            .h_full()
            .flex_1()
            .child(TabBar::new(self.editor.tabs.clone()))
            .child(Toolbar::new(
                |_, payload| {
                    let payload = payload.downcast_ref::<LeftItemsPayload>().unwrap();

                    vec![Breadcrumb::new(payload.path.clone(), payload.symbols.clone()).into_any()]
                },
                Box::new(LeftItemsPayload {
                    path: self.editor.path.clone(),
                    symbols: self.editor.symbols.clone(),
                }),
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
