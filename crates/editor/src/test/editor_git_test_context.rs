use std::ops::{Deref, DerefMut};

use gpui::ModelHandle;
use language::Buffer;
use settings::Settings;

use crate::MultiBuffer;

use super::{build_editor, editor_test_context::EditorTestContext};

pub struct EditorGitTestContext<'a> {
    pub cx: EditorTestContext<'a>,
    pub buffer: ModelHandle<Buffer>,
}

impl<'a> EditorGitTestContext<'a> {
    pub async fn new(cx: &'a mut gpui::TestAppContext) -> EditorGitTestContext<'a> {
        let (window_id, buffer, editor) = cx.update(|cx| {
            cx.set_global(Settings::test(cx));
            crate::init(cx);

            let buffer = cx.add_model(|cx| Buffer::new(0, "", cx));
            let multibuffer = cx.add_model(|cx| MultiBuffer::singleton(buffer.clone(), cx));

            let (window_id, editor) =
                cx.add_window(Default::default(), |cx| build_editor(multibuffer, cx));

            editor.update(cx, |_, cx| cx.focus_self());

            (window_id, buffer, editor)
        });

        Self {
            cx: EditorTestContext {
                cx,
                window_id,
                editor,
            },
            buffer,
        }
    }
}

impl<'a> Deref for EditorGitTestContext<'a> {
    type Target = EditorTestContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl<'a> DerefMut for EditorGitTestContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}
