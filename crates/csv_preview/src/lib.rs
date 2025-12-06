use editor::Editor;
use gpui::{AppContext, Entity, EventEmitter, FocusHandle, Focusable, actions};
use log::info;
use ui::{App, Context, ParentElement, Render, Styled, Window, div, v_flex};
use workspace::{Item, Workspace};

actions!(csv, [OpenPreview]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            println!("No window yet");
            return;
        };
        CsvPreviewView::register(workspace, window, cx);
    })
    .detach()
}

pub struct CsvPreviewView {
    focus_handle: FocusHandle,
    editor: Entity<Editor>,
}
impl CsvPreviewView {
    pub fn register(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<'_, Workspace>,
    ) {
        // Register open preview action
        workspace.register_action(move |workspace, _: &OpenPreview, window, cx| {
            info!("Open preview called");
            let maybe_editor = {
                let and_then = workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx));
                let Some(editor) = and_then else {
                    info!("No editor");
                    return;
                };
                // if Self::is_csv_file(&editor, cx) {
                //     info!("Editor is csv");
                //     Some(editor)
                // } else {
                //     info!("Editor is not csv");
                //     None
                // }
                Some(editor)
            };

            let Some(editor) = maybe_editor else {
                info!("No CSV editor found");
                return;
            };

            let view = CsvPreviewView::from_editor(&editor, cx);
            info!("Created CSV View");
            workspace.active_pane().update(cx, |pane, cx| {
                // TODO: handle existing pane
                info!("Attaching CSV View");
                pane.add_item(Box::new(view.clone()), true, true, None, window, cx)
            });
            cx.notify();
        });
    }

    fn is_csv_file(editor: &Entity<Editor>, cx: &mut Context<Workspace>) -> bool {
        let buffer = editor.read(cx).buffer().read(cx);
        let Some(buffer) = buffer.as_singleton() else {
            info!("Buffer is not singletone");
            return false;
        };
        let Some(language) = buffer.read(cx).language() else {
            info!("Buffer has no language");
            return false;
        };

        info!(
            "Buffer: {:?} has language: {language:?}",
            buffer.read(cx).file().map(|f| f.path())
        );
        language.name() == "CSV".into()
    }

    fn from_editor(editor: &Entity<Editor>, cx: &mut Context<Workspace>) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            editor: editor.clone(),
        })
    }
}

impl Focusable for CsvPreviewView {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for CsvPreviewView {}

/// Icon and description as tab
impl Item for CsvPreviewView {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> ui::SharedString {
        "CSV Preview".into()
    }

    // fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {}
}

/// Main trait to render the content of the CSV preview in pane
impl Render for CsvPreviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let content = self
            .editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .map(|buffer| buffer.read(cx).text())
            .unwrap_or_else(|| "No content".to_string());

        v_flex()
            .child(div().child("CSV Preview:"))
            .child(div().child(content))
    }
}
