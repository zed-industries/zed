use crate::{Editor, EditorElement, EditorStyle};
use gpui::{sketch::View, Model};
use ui::ViewContext;

pub struct PaneItemProps {}
struct TabProps {}

pub trait RenderEditor {
    fn tab_view(&self, props: EditorStyle) -> View<Editor, TabProps>;
    fn render(&self, props: EditorStyle) -> EditorElement;
}

impl RenderEditor for Model<Editor> {
    fn tab_view(&self, style: EditorStyle) -> View<Editor, TabProps> {
        View::new(
            self.clone(),
            move |_, _: TabProps, cx: &mut ViewContext<Editor>| {
                cx.view().model.render(style.clone())
            },
        )
    }

    fn render(&self, style: EditorStyle) -> EditorElement {
        todo!()
        // EditorElement::new(self, style)
    }
}
