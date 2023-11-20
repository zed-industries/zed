use gpui::{AnyElement, Div, RenderOnce, Stateful};
use smallvec::SmallVec;

use crate::{h_stack, prelude::*, v_stack, Button, Icon, IconButton, Label};

#[derive(RenderOnce)]
pub struct Modal<V: 'static> {
    id: ElementId,
    title: Option<SharedString>,
    primary_action: Option<Button<V>>,
    secondary_action: Option<Button<V>>,
    children: SmallVec<[AnyElement<V>; 2]>,
}

impl<V: 'static> Component<V> for Modal<V> {
    type Rendered = Stateful<V, Div<V>>;

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> Self::Rendered {
        let _view: &mut V = view;
        v_stack()
            .id(self.id.clone())
            .w_96()
            // .rounded_xl()
            .bg(cx.theme().colors().background)
            .border()
            .border_color(cx.theme().colors().border)
            .shadow_2xl()
            .child(
                h_stack()
                    .justify_between()
                    .p_1()
                    .border_b()
                    .border_color(cx.theme().colors().border)
                    .child(div().children(self.title.clone().map(|t| Label::new(t))))
                    .child(IconButton::new("close", Icon::Close)),
            )
            .child(v_stack().p_1().children(self.children))
            .when(
                self.primary_action.is_some() || self.secondary_action.is_some(),
                |this| {
                    this.child(
                        h_stack()
                            .border_t()
                            .border_color(cx.theme().colors().border)
                            .p_1()
                            .justify_end()
                            .children(self.secondary_action)
                            .children(self.primary_action),
                    )
                },
            )
    }
}

impl<V: 'static> Modal<V> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            primary_action: None,
            secondary_action: None,
            children: SmallVec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn primary_action(mut self, action: Button<V>) -> Self {
        self.primary_action = Some(action);
        self
    }

    pub fn secondary_action(mut self, action: Button<V>) -> Self {
        self.secondary_action = Some(action);
        self
    }
}

impl<V: 'static> ParentElement<V> for Modal<V> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        &mut self.children
    }
}
