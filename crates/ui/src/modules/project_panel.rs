use std::marker::PhantomData;

use gpui2::elements::div;
use gpui2::elements::div::ScrollState;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::prelude::{InteractionState, ToggleState};
use crate::{details, input, label, list_item, theme, IconAsset, LabelColor};

#[derive(Element)]
pub struct ProjectPanel<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
}

pub fn project_panel<V: 'static>(scroll_state: ScrollState) -> ProjectPanel<V> {
    ProjectPanel {
        view_type: PhantomData,
        scroll_state,
    }
}

impl<V: 'static> ProjectPanel<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .w_56()
            .h_full()
            .flex()
            .flex_col()
            .fill(theme.middle.base.default.background)
            .child(
                div()
                    .w_56()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .child(details("This is a long string that should wrap when it keeps going for a long time.").meta_text("6 h ago)"))
                    .child(
                        div().flex().flex_col().children(
                            std::iter::repeat_with(|| {
                                vec![
                                    list_item(label("sqlez").color(LabelColor::Modified))
                                        .left_icon(IconAsset::FolderOpen.into())
                                        .indent_level(0)
                                        .set_toggle(ToggleState::NotToggled),
                                    list_item(label("storybook").color(LabelColor::Modified))
                                        .left_icon(IconAsset::FolderOpen.into())
                                        .indent_level(0)
                                        .set_toggle(ToggleState::Toggled),
                                    list_item(label("docs").color(LabelColor::Default))
                                        .left_icon(IconAsset::Folder.into())
                                        .indent_level(1)
                                        .set_toggle(ToggleState::Toggled),
                                    list_item(label("src").color(LabelColor::Modified))
                                        .left_icon(IconAsset::FolderOpen.into())
                                        .indent_level(2)
                                        .set_toggle(ToggleState::Toggled),
                                    list_item(label("ui").color(LabelColor::Modified))
                                        .left_icon(IconAsset::FolderOpen.into())
                                        .indent_level(3)
                                        .set_toggle(ToggleState::Toggled),
                                    list_item(label("component").color(LabelColor::Created))
                                        .left_icon(IconAsset::FolderOpen.into())
                                        .indent_level(4)
                                        .set_toggle(ToggleState::Toggled),
                                    list_item(label("facepile.rs").color(LabelColor::Default))
                                        .left_icon(IconAsset::File.into())
                                        .indent_level(5),
                                    list_item(label("follow_group.rs").color(LabelColor::Default))
                                        .left_icon(IconAsset::File.into())
                                        .indent_level(5),
                                    list_item(label("list_item.rs").color(LabelColor::Created))
                                        .left_icon(IconAsset::File.into())
                                        .indent_level(5),
                                    list_item(label("tab.rs").color(LabelColor::Default))
                                        .left_icon(IconAsset::File.into())
                                        .indent_level(5),
                                ]
                            })
                            .take(10)
                            .flatten(),
                        ),
                    ),
            )
            .child(
                input("Find something...")
                    .value("buffe".to_string())
                    .state(InteractionState::Focused),
            )
    }
}
