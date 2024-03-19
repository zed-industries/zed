use std::sync::Arc;

use editor::Editor;
use gpui::{AppContext, Model, WeakModel};
use task::TaskSource;
use ui::Context;
use workspace::Workspace;

pub struct BufferSource {
    workspace: WeakModel<Workspace>,
}

impl BufferSource {
    pub fn new(workspace: WeakModel<Workspace>, cx: &mut AppContext) -> Model<Box<dyn TaskSource>> {
        cx.new_model(|_| Box::new(Self { workspace }) as Box<_>)
    }
}
impl TaskSource for BufferSource {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn tasks_for_path(
        &mut self,
        path: Option<&std::path::Path>,
        cx: &mut gpui::ModelContext<Box<dyn TaskSource>>,
    ) -> Vec<Arc<dyn task::Task>> {
        let Some(active_editor) = self
            .workspace
            .update(cx, |this, cx| this.active_item_as::<Editor>(cx))
            .ok()
            .flatten()
        else {
            return vec![];
        };
        active_editor
            .model
            .update(cx, |this, cx| {
                let (_, buffer, range) = this.active_excerpt(cx)?;
                let language = buffer.update(cx, |this, _| this.language_at(range.start))?;
                Some(
                    language
                        .context_provider()?
                        .as_source()?
                        .update(cx, |source, cx| source.tasks_for_path(path, cx)),
                )
            })
            .unwrap_or_default()
    }
}
