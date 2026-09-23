use std::time::{Duration, Instant};

use gpui::{
    AnyElement, App, ClipboardItem, Context, ElementId, Entity, IntoElement, ParentElement,
    RenderOnce, Styled, Window,
};

use crate::{Tooltip, prelude::*};

const COPIED_STATE_DURATION: Duration = Duration::from_secs(2);

struct CopyButtonState {
    copied_at: Option<Instant>,
}

impl CopyButtonState {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { copied_at: None }
    }

    fn is_copied(&self) -> bool {
        self.copied_at
            .map(|t| t.elapsed() < COPIED_STATE_DURATION)
            .unwrap_or(false)
    }

    fn mark_copied(&mut self) {
        self.copied_at = Some(Instant::now());
    }
}

#[derive(IntoElement, RegisterComponent)]
pub struct CopyButton {
    id: ElementId,
    message: SharedString,
    icon_size: IconSize,
    disabled: bool,
    tooltip_label: SharedString,
    visible_on_hover: Option<SharedString>,
    custom_on_click: Option<Box<dyn Fn(&mut Window, &mut App) -> bool + 'static>>,
}

impl CopyButton {
    pub fn new(id: impl Into<ElementId>, message: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            message: message.into(),
            icon_size: IconSize::Small,
            disabled: false,
            tooltip_label: "Copy".into(),
            visible_on_hover: None,
            custom_on_click: None,
        }
    }

    pub fn new_with_action(
        id: impl Into<ElementId>,
        action: impl Fn(&mut Window, &mut App) -> bool + 'static,
    ) -> Self {
        Self::new(id, String::new()).custom_on_click_with_result(action)
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = icon_size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn tooltip_label(mut self, tooltip_label: impl Into<SharedString>) -> Self {
        self.tooltip_label = tooltip_label.into();
        self
    }

    pub fn visible_on_hover(mut self, visible_on_hover: impl Into<SharedString>) -> Self {
        self.visible_on_hover = Some(visible_on_hover.into());
        self
    }

    pub fn custom_on_click(
        self,
        custom_on_click: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.custom_on_click_with_result(move |window, cx| {
            custom_on_click(window, cx);
            true
        })
    }

    pub fn custom_on_click_with_result(
        mut self,
        custom_on_click: impl Fn(&mut Window, &mut App) -> bool + 'static,
    ) -> Self {
        self.custom_on_click = Some(Box::new(custom_on_click));
        self
    }
}

impl RenderOnce for CopyButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = self.id.clone();
        let message = self.message;
        let custom_on_click = self.custom_on_click;
        let visible_on_hover = self.visible_on_hover;

        let state: Entity<CopyButtonState> =
            window.use_keyed_state(id.clone(), cx, CopyButtonState::new);
        let is_copied = state.read(cx).is_copied();

        let (icon, color, tooltip) = if is_copied {
            (IconName::Check, Color::Success, "Copied!".into())
        } else {
            (IconName::Copy, Color::Muted, self.tooltip_label)
        };

        let button = IconButton::new(id, icon)
            .icon_color(color)
            .icon_size(self.icon_size)
            .disabled(self.disabled)
            .tooltip(Tooltip::text(tooltip))
            .on_click(move |_, window, cx| {
                let copied = if let Some(custom_on_click) = custom_on_click.as_ref() {
                    (custom_on_click)(window, cx)
                } else {
                    cx.stop_propagation();
                    cx.write_to_clipboard(ClipboardItem::new_string(message.to_string()));
                    true
                };

                if copied {
                    state.update(cx, |state, _cx| {
                        state.mark_copied();
                    });

                    let state_id = state.entity_id();
                    cx.spawn(async move |cx| {
                        cx.background_executor().timer(COPIED_STATE_DURATION).await;
                        cx.update(|cx| {
                            cx.notify(state_id);
                        })
                    })
                    .detach();
                }
            });

        if let Some(visible_on_hover) = visible_on_hover {
            button.visible_on_hover(visible_on_hover)
        } else {
            button
        }
    }
}

impl Component for CopyButton {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn description() -> &'static str {
        "An icon button that encapsulates the logic to copy a string into the clipboard."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
        let label_text = "Here's an example label";

        let examples = vec![
            single_example(
                "Default",
                h_flex()
                    .gap_1()
                    .child(Label::new(label_text).size(LabelSize::Small))
                    .child(CopyButton::new("preview-default", label_text))
                    .into_any_element(),
            ),
            single_example(
                "Multiple Icon Sizes",
                h_flex()
                    .gap_1()
                    .child(Label::new(label_text).size(LabelSize::Small))
                    .child(
                        CopyButton::new("preview-xsmall", label_text).icon_size(IconSize::XSmall),
                    )
                    .child(
                        CopyButton::new("preview-medium", label_text).icon_size(IconSize::Medium),
                    )
                    .child(
                        CopyButton::new("preview-xlarge", label_text).icon_size(IconSize::XLarge),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Custom Tooltip Label",
                h_flex()
                    .gap_1()
                    .child(Label::new(label_text).size(LabelSize::Small))
                    .child(
                        CopyButton::new("preview-tooltip", label_text)
                            .tooltip_label("Custom tooltip label"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Visible On Hover",
                h_flex()
                    .group("container")
                    .gap_1()
                    .child(Label::new(label_text).size(LabelSize::Small))
                    .child(
                        CopyButton::new("preview-hover", label_text).visible_on_hover("container"),
                    )
                    .into_any_element(),
            ),
        ];

        example_group(examples).vertical().into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Render, TestAppContext};
    use std::{cell::Cell, rc::Rc};

    struct TestCopyButton {
        should_copy: Rc<Cell<bool>>,
    }

    impl Render for TestCopyButton {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let should_copy = self.should_copy.clone();
            div().child(CopyButton::new_with_action("test-copy", move |_, _| {
                should_copy.get()
            }))
        }
    }

    #[gpui::test]
    async fn test_custom_copy_action_only_shows_success_when_it_copies(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
        let should_copy = Rc::new(Cell::new(false));
        let (_view, cx) = cx.add_window_view({
            let should_copy = should_copy.clone();
            move |_, _| TestCopyButton { should_copy }
        });

        let button = cx.debug_bounds("ICON-Copy").unwrap();
        cx.simulate_click(button.center(), gpui::Modifiers::default());
        cx.run_until_parked();
        assert!(cx.debug_bounds("ICON-Check").is_none());

        should_copy.set(true);
        cx.simulate_click(button.center(), gpui::Modifiers::default());
        cx.run_until_parked();
        assert!(cx.debug_bounds("ICON-Check").is_some());
    }
}
