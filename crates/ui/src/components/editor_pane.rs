use std::marker::PhantomData;
use std::path::PathBuf;

use crate::prelude::*;
use crate::{v_stack, Breadcrumb, Buffer, HighlightedText, Icon, IconButton, Tab, TabBar, Toolbar};

pub struct Editor {
    pub tabs: Vec<Tab>,
    pub path: PathBuf,
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
        struct LeftItemsPayload {
            path: PathBuf,
        }

        v_stack()
            .w_full()
            .h_full()
            .flex_1()
            .child(TabBar::new(self.editor.tabs.clone()))
            .child(Toolbar::new(
                |_, payload| {
                    let payload = payload.downcast_ref::<LeftItemsPayload>().unwrap();

                    vec![Breadcrumb::new(payload.path.clone(), vec![]).into_any()]
                },
                Box::new(LeftItemsPayload {
                    path: self.editor.path.clone(),
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
