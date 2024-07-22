use std::ops::Range;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use collections::HashMap;
use editor::{Anchor, Editor, RangeToAnchorExt};
use gpui::{
    prelude::*, AppContext, EntityId, Global, Model, ModelContext, Subscription, Task, View,
    ViewContext, WeakView,
};
use language::{Language, Point};
use multi_buffer::MultiBufferRow;
use project::Fs;
use settings::{Settings, SettingsStore};

use crate::kernels::kernel_specifications;
use crate::session::SessionEvent;
use crate::{JupyterSettings, KernelSpecification, Session};

pub enum SessionSupport {
    ActiveSession(View<Session>),
    Inactive(Box<KernelSpecification>),
    RequiresSetup(Arc<str>),
    Unsupported,
}

struct GlobalReplStore(Model<ReplStore>);

impl Global for GlobalReplStore {}

pub struct ReplStore {
    fs: Arc<dyn Fs>,
    enabled: bool,
    sessions: HashMap<EntityId, View<Session>>,
    kernel_specifications: Vec<KernelSpecification>,
    _subscriptions: Vec<Subscription>,
}

impl ReplStore {
    pub(crate) fn init(fs: Arc<dyn Fs>, cx: &mut AppContext) {
        let store = cx.new_model(move |cx| Self::new(fs, cx));

        cx.set_global(GlobalReplStore(store))
    }

    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalReplStore>().0.clone()
    }

    pub fn new(fs: Arc<dyn Fs>, cx: &mut ModelContext<Self>) -> Self {
        let subscriptions = vec![cx.observe_global::<SettingsStore>(move |this, cx| {
            this.set_enabled(JupyterSettings::enabled(cx), cx);
        })];

        Self {
            fs,
            enabled: JupyterSettings::enabled(cx),
            sessions: HashMap::default(),
            kernel_specifications: Vec::new(),
            _subscriptions: subscriptions,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn kernel_specifications(&self) -> impl Iterator<Item = &KernelSpecification> {
        self.kernel_specifications.iter()
    }

    pub fn sessions(&self) -> impl Iterator<Item = &View<Session>> {
        self.sessions.values()
    }

    fn set_enabled(&mut self, enabled: bool, cx: &mut ModelContext<Self>) {
        if self.enabled != enabled {
            self.enabled = enabled;
            cx.notify();
        }
    }

    pub fn snippet(
        &self,
        editor: WeakView<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> Option<(String, Arc<Language>, Range<Anchor>)> {
        let editor = editor.upgrade()?;
        let editor = editor.read(cx);

        let buffer = editor.buffer().read(cx).snapshot(cx);

        let selection = editor.selections.newest::<usize>(cx);
        let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);

        let range = if selection.is_empty() {
            let cursor = selection.head();

            let cursor_row = multi_buffer_snapshot.offset_to_point(cursor).row;
            let start_offset = multi_buffer_snapshot.point_to_offset(Point::new(cursor_row, 0));

            let end_point = Point::new(
                cursor_row,
                multi_buffer_snapshot.line_len(MultiBufferRow(cursor_row)),
            );
            let end_offset = start_offset.saturating_add(end_point.column as usize);

            // Create a range from the start to the end of the line
            start_offset..end_offset
        } else {
            selection.range()
        };

        let anchor_range = range.to_anchors(&multi_buffer_snapshot);

        let selected_text = buffer
            .text_for_range(anchor_range.clone())
            .collect::<String>();

        let start_language = buffer.language_at(anchor_range.start)?;
        let end_language = buffer.language_at(anchor_range.end)?;
        if start_language != end_language {
            return None;
        }

        Some((selected_text, start_language.clone(), anchor_range))
    }

    pub fn language(
        &self,
        editor: WeakView<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> Option<Arc<Language>> {
        let editor = editor.upgrade()?;
        let selection = editor.read(cx).selections.newest::<usize>(cx);
        let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
        buffer.language_at(selection.head()).cloned()
    }

    pub fn refresh_kernelspecs(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let kernel_specifications = kernel_specifications(self.fs.clone());
        cx.spawn(|this, mut cx| async move {
            let kernel_specifications = kernel_specifications.await?;

            this.update(&mut cx, |this, cx| {
                this.kernel_specifications = kernel_specifications;
                cx.notify();
            })
        })
    }

    pub fn kernelspec(
        &self,
        language: &Language,
        cx: &mut ViewContext<Self>,
    ) -> Option<KernelSpecification> {
        let settings = JupyterSettings::get_global(cx);
        let language_name = language.code_fence_block_name();
        let selected_kernel = settings.kernel_selections.get(language_name.as_ref());

        self.kernel_specifications
            .iter()
            .find(|runtime_specification| {
                if let Some(selected) = selected_kernel {
                    // Top priority is the selected kernel
                    runtime_specification.name.to_lowercase() == selected.to_lowercase()
                } else {
                    // Otherwise, we'll try to find a kernel that matches the language
                    runtime_specification.kernelspec.language.to_lowercase()
                        == language_name.to_lowercase()
                }
            })
            .cloned()
    }

    pub fn run(&mut self, editor: WeakView<Editor>, cx: &mut ViewContext<Self>) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let (selected_text, language, anchor_range) = match self.snippet(editor.clone(), cx) {
            Some(snippet) => snippet,
            None => return Ok(()),
        };

        let entity_id = editor.entity_id();

        let kernel_specification = self
            .kernelspec(&language, cx)
            .with_context(|| format!("No kernel found for language: {}", language.name()))?;

        let session = self.sessions.entry(entity_id).or_insert_with(|| {
            let view =
                cx.new_view(|cx| Session::new(editor, self.fs.clone(), kernel_specification, cx));
            cx.notify();

            let subscription = cx.subscribe(&view, |this, _session, event, _cx| match event {
                SessionEvent::Shutdown(shutdown_event) => {
                    this.sessions.remove(&shutdown_event.entity_id());
                }
            });

            subscription.detach();

            view
        });

        session.update(cx, |session, cx| {
            session.execute(&selected_text, anchor_range, cx);
        });

        anyhow::Ok(())
    }

    pub fn session(
        &mut self,
        editor: WeakView<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> SessionSupport {
        let entity_id = editor.entity_id();
        let session = self.sessions.get(&entity_id).cloned();

        match session {
            Some(session) => SessionSupport::ActiveSession(session),
            None => {
                let language = self.language(editor, cx);
                let language = match language {
                    Some(language) => language,
                    None => return SessionSupport::Unsupported,
                };
                let kernelspec = self.kernelspec(&language, cx);

                match kernelspec {
                    Some(kernelspec) => SessionSupport::Inactive(Box::new(kernelspec)),
                    None => match language.name().as_ref() {
                        "TypeScript" | "Python" => SessionSupport::RequiresSetup(language.name()),
                        _ => SessionSupport::Unsupported,
                    },
                }
            }
        }
    }

    pub fn clear_outputs(&mut self, editor: WeakView<Editor>, cx: &mut ViewContext<Self>) {
        let entity_id = editor.entity_id();
        if let Some(session) = self.sessions.get_mut(&entity_id) {
            session.update(cx, |session, cx| {
                session.clear_outputs(cx);
            });
            cx.notify();
        }
    }

    pub fn interrupt(&mut self, editor: WeakView<Editor>, cx: &mut ViewContext<Self>) {
        let entity_id = editor.entity_id();
        if let Some(session) = self.sessions.get_mut(&entity_id) {
            session.update(cx, |session, cx| {
                session.interrupt(cx);
            });
            cx.notify();
        }
    }

    pub fn shutdown(&self, editor: WeakView<Editor>, cx: &mut ViewContext<Self>) {
        let entity_id = editor.entity_id();
        if let Some(session) = self.sessions.get(&entity_id) {
            session.update(cx, |session, cx| {
                session.shutdown(cx);
            });
            cx.notify();
        }
    }
}
