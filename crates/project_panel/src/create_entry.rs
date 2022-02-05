use crate::ProjectPanel;
use editor::{Editor, EditorSettings};
use gpui::{
    action, elements::*, keymap::Binding, Axis, Entity, MutableAppContext, RenderContext, View,
    ViewContext, ViewHandle,
};
use postage::watch;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::sync::Arc;
use workspace::Settings;

action!(Dismiss);
action!(Confirm);
action!(RedeployCreateDir);
action!(RedeployCreateFile);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("escape", Dismiss, Some("CreateEntry")),
        Binding::new("enter", Confirm, Some("CreateEntry")),
        Binding::new("alt-n", RedeployCreateDir, Some("CreateEntry")),
        Binding::new("ctrl-n", RedeployCreateFile, Some("CreateEntry")),
    ]);
    cx.add_action(CreateEntry::confirm);
    cx.add_action(CreateEntry::dismiss);
    cx.add_action(CreateEntry::redeploy_as_create_dir);
    cx.add_action(CreateEntry::redeploy_as_create_file);
}

pub struct CreateEntry {
    settings: watch::Receiver<Settings>,
    query_editor: ViewHandle<Editor>,
    object: Object,
}

pub enum Event {
    Confirmed,
    Dismissed,
}

#[derive(Clone, Copy)]
pub enum Object {
    File,
    Directory,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::File => "file",
                Object::Directory => "directory",
            }
        )
    }
}

impl CreateEntry {
    pub fn new(
        settings: watch::Receiver<Settings>,
        object: Object,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::single_line(
                {
                    let settings = settings.clone();
                    Arc::new(move |_| {
                        let settings = settings.borrow();
                        EditorSettings {
                            tab_size: settings.tab_size,
                            style: settings.theme.selector.input_editor.as_editor(),
                            soft_wrap: editor::SoftWrap::None,
                        }
                    })
                },
                cx,
            )
        });
        cx.subscribe(&query_editor, Self::on_query_editor_event)
            .detach();

        Self {
            settings: settings.clone(),
            query_editor,
            object,
        }
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Confirmed);
    }

    fn dismiss(&mut self, _: &Dismiss, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed)
    }

    fn redeploy_as_create_file(&mut self, _: &RedeployCreateFile, cx: &mut ViewContext<Self>) {
        self.object = Object::File;
        cx.notify();
    }

    fn redeploy_as_create_dir(&mut self, _: &RedeployCreateDir, cx: &mut ViewContext<Self>) {
        self.object = Object::Directory;
        cx.notify();
    }

    pub(crate) fn on_event(
        project_panel: &mut ProjectPanel,
        this: ViewHandle<Self>,
        event: &Event,
        cx: &mut ViewContext<ProjectPanel>,
    ) {
        match event {
            Event::Confirmed => {
                let name = this.read(cx).query_editor.read(cx).text(cx);
                if !name.is_empty() {
                    if let Some((worktree, entry)) = project_panel.selected_entry(cx) {
                        let worktree_id = worktree.id();
                        let parent_path = if entry.is_dir() {
                            entry.path.clone()
                        } else {
                            entry
                                .path
                                .clone()
                                .parent()
                                .map_or(Path::new("").into(), |p| p.into())
                        };
                        let path = parent_path.join(Path::new(&name));
                        match this.read(cx).object {
                            Object::File => cx.emit(crate::Event::CreateFile { worktree_id, path }),
                            Object::Directory => {
                                cx.emit(crate::Event::CreateDirectory { worktree_id, path })
                            }
                        }
                        project_panel.dismiss_create_entry(cx)
                    }
                }
            }
            Event::Dismissed => project_panel.dismiss_create_entry(cx),
        }
    }

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::Blurred => cx.emit(Event::Dismissed),
            _ => {}
        }
    }
}

impl Entity for CreateEntry {
    type Event = Event;
}

impl View for CreateEntry {
    fn ui_name() -> &'static str {
        "CreateEntry"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        let theme = &self.settings.borrow().theme.selector;
        let label = format!("enter {} name", self.object);

        Align::new(
            ConstrainedBox::new(
                Container::new(
                    Flex::new(Axis::Vertical)
                        .with_child(
                            Container::new(ChildView::new(&self.query_editor).boxed())
                                .with_style(theme.input_editor.container)
                                .boxed(),
                        )
                        .with_child(
                            Container::new(Label::new(label, theme.empty.label.clone()).boxed())
                                .with_style(theme.empty.container)
                                .boxed(),
                        )
                        .boxed(),
                )
                .with_style(theme.container)
                .boxed(),
            )
            .with_max_width(500.0)
            .boxed(),
        )
        .top()
        .named("create entry")
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_editor);
    }
}
