#![allow(missing_docs)]

use std::sync::Arc;

use crate::{prelude::*, Checkbox};

/// A [`Checkbox`] that has a [`Label`].
#[derive(IntoElement)]
pub struct CheckboxWithLabel {
    id: ElementId,
    label: Label,
    checked: Selection,
    on_click: Arc<dyn Fn(&Selection, &mut WindowContext) + 'static>,
}

impl CheckboxWithLabel {
    pub fn new(
        id: impl Into<ElementId>,
        label: Label,
        checked: Selection,
        on_click: impl Fn(&Selection, &mut WindowContext) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label,
            checked,
            on_click: Arc::new(on_click),
        }
    }
}

impl RenderOnce for CheckboxWithLabel {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .gap(Spacing::Large.rems(cx))
            .child(Checkbox::new(self.id.clone(), self.checked).on_click({
                let on_click = self.on_click.clone();
                move |checked, cx| {
                    (on_click)(checked, cx);
                }
            }))
            .child(
                div()
                    .id(SharedString::from(format!("{}-label", self.id)))
                    .on_click(move |_event, cx| {
                        (self.on_click)(&self.checked.inverse(), cx);
                    })
                    .child(self.label),
            )
    }
}
