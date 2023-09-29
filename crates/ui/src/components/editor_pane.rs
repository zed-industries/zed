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
            .child(TabBar::new(vec![
                Tab::new()
                    .title("Cargo.toml".to_string())
                    .current(false)
                    .git_status(GitStatus::Modified),
                Tab::new()
                    .title("Channels Panel".to_string())
                    .current(false),
                Tab::new()
                    .title("channels_panel.rs".to_string())
                    .current(true)
                    .git_status(GitStatus::Modified),
                Tab::new()
                    .title("workspace.rs".to_string())
                    .current(false)
                    .git_status(GitStatus::Modified),
                Tab::new()
                    .title("icon_button.rs".to_string())
                    .current(false),
                Tab::new()
                    .title("storybook.rs".to_string())
                    .current(false)
                    .git_status(GitStatus::Created),
                Tab::new().title("theme.rs".to_string()).current(false),
                Tab::new()
                    .title("theme_registry.rs".to_string())
                    .current(false),
                Tab::new()
                    .title("styleable_helpers.rs".to_string())
                    .current(false),
            ]))
            .child(Toolbar::new())
            .child(self.editor.buffer.clone())
    }
}
