// use editor::EditorSettings;
use ui::{ButtonCommon, Clickable, Context, Render, Tooltip, Window, prelude::*};
use workspace::{ItemHandle, StatusItemView};
use zed_actions::presentation_mode_selector;

pub struct PresentationModeSelectorButton;

impl PresentationModeSelectorButton {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for PresentationModeSelectorButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let button = div();

        //     let Some(active_presentation_mode) = cx.try_global::<DisabledPresentationModeState>()
        //     else {
        //         return button.w_0().invisible();
        //     };

        //     let tooltip_title = format!(
        //         "Presentation Mode: {}",
        //         active_presentation_mode.presentation_mode.name
        //     );

        //     button.child(
        //         IconButton::new("presentation-mode-button", IconName::MagnifyingGlass)
        //             .icon_size(IconSize::Small)
        //             .tooltip(move |window, cx| {
        //                 Tooltip::for_action(
        //                     tooltip_title.clone(),
        //                     &presentation_mode_selector::Toggle::default(),
        //                     window,
        //                     cx,
        //                 )
        //             })
        //             .on_click(cx.listener(|_this, _, window, cx| {
        //                 window.dispatch_action(
        //                     Box::new(presentation_mode_selector::Toggle::default()),
        //                     cx,
        //                 );
        //             })),
        //     )

        button.w_0().invisible()
    }
}

impl StatusItemView for PresentationModeSelectorButton {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
