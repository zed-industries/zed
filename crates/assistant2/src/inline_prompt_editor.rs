use gpui::{AnyElement, EventEmitter};
use ui::{prelude::*, IconButtonShape, Tooltip};

pub enum CodegenStatus {
    Idle,
    Pending,
    Done,
    Error(anyhow::Error),
}

/// This is just CodegenStatus without the anyhow::Error, which causes a lifetime issue for rendering the Cancel button.
#[derive(Copy, Clone)]
pub enum CancelButtonState {
    Idle,
    Pending,
    Done,
    Error,
}

impl Into<CancelButtonState> for &CodegenStatus {
    fn into(self) -> CancelButtonState {
        match self {
            CodegenStatus::Idle => CancelButtonState::Idle,
            CodegenStatus::Pending => CancelButtonState::Pending,
            CodegenStatus::Done => CancelButtonState::Done,
            CodegenStatus::Error(_) => CancelButtonState::Error,
        }
    }
}

#[derive(Copy, Clone)]
pub enum PromptMode {
    Generate { supports_execute: bool },
    Transform,
}

impl PromptMode {
    fn start_label(self) -> &'static str {
        match self {
            PromptMode::Generate { .. } => "Generate",
            PromptMode::Transform => "Transform",
        }
    }
    fn tooltip_interrupt(self) -> &'static str {
        match self {
            PromptMode::Generate { .. } => "Interrupt Generation",
            PromptMode::Transform => "Interrupt Transform",
        }
    }

    fn tooltip_restart(self) -> &'static str {
        match self {
            PromptMode::Generate { .. } => "Restart Generation",
            PromptMode::Transform => "Restart Transform",
        }
    }

    fn tooltip_accept(self) -> &'static str {
        match self {
            PromptMode::Generate { .. } => "Accept Generation",
            PromptMode::Transform => "Accept Transform",
        }
    }
}

pub fn render_cancel_button<T: EventEmitter<PromptEditorEvent>>(
    cancel_button_state: CancelButtonState,
    edited_since_done: bool,
    mode: PromptMode,
    cx: &mut ViewContext<T>,
) -> Vec<AnyElement> {
    match cancel_button_state {
        CancelButtonState::Idle => {
            vec![
                IconButton::new("cancel", IconName::Close)
                    .icon_color(Color::Muted)
                    .shape(IconButtonShape::Square)
                    .tooltip(|cx| Tooltip::for_action("Cancel Assist", &menu::Cancel, cx))
                    .on_click(cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::CancelRequested)))
                    .into_any_element(),
                Button::new("start", mode.start_label())
                    .icon(IconName::Return)
                    .icon_color(Color::Muted)
                    .on_click(cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::StartRequested)))
                    .into_any_element(),
            ]
        }
        CancelButtonState::Pending => vec![
            IconButton::new("cancel", IconName::Close)
                .icon_color(Color::Muted)
                .shape(IconButtonShape::Square)
                .tooltip(|cx| Tooltip::text("Cancel Assist", cx))
                .on_click(cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::CancelRequested)))
                .into_any_element(),
            IconButton::new("stop", IconName::Stop)
                .icon_color(Color::Error)
                .shape(IconButtonShape::Square)
                .tooltip(move |cx| {
                    Tooltip::with_meta(
                        mode.tooltip_interrupt(),
                        Some(&menu::Cancel),
                        "Changes won't be discarded",
                        cx,
                    )
                })
                .on_click(cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::StopRequested)))
                .into_any_element(),
        ],
        CancelButtonState::Done | CancelButtonState::Error => {
            let cancel = IconButton::new("cancel", IconName::Close)
                .icon_color(Color::Muted)
                .shape(IconButtonShape::Square)
                .tooltip(|cx| Tooltip::for_action("Cancel Assist", &menu::Cancel, cx))
                .on_click(cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::CancelRequested)))
                .into_any_element();

            let has_error = matches!(cancel_button_state, CancelButtonState::Error);
            if has_error || edited_since_done {
                vec![
                    cancel,
                    IconButton::new("restart", IconName::RotateCw)
                        .icon_color(Color::Info)
                        .shape(IconButtonShape::Square)
                        .tooltip(move |cx| {
                            Tooltip::with_meta(
                                mode.tooltip_restart(),
                                Some(&menu::Confirm),
                                "Changes will be discarded",
                                cx,
                            )
                        })
                        .on_click(cx.listener(|_, _, cx| {
                            cx.emit(PromptEditorEvent::StartRequested);
                        }))
                        .into_any_element(),
                ]
            } else {
                let mut buttons = vec![
                    cancel,
                    IconButton::new("accept", IconName::Check)
                        .icon_color(Color::Info)
                        .shape(IconButtonShape::Square)
                        .tooltip(move |cx| {
                            Tooltip::for_action(mode.tooltip_accept(), &menu::Confirm, cx)
                        })
                        .on_click(cx.listener(|_, _, cx| {
                            cx.emit(PromptEditorEvent::ConfirmRequested { execute: false });
                        }))
                        .into_any_element(),
                ];

                match mode {
                    PromptMode::Generate { supports_execute } => {
                        if supports_execute {
                            buttons.push(
                                IconButton::new("confirm", IconName::Play)
                                    .icon_color(Color::Info)
                                    .shape(IconButtonShape::Square)
                                    .tooltip(|cx| {
                                        Tooltip::for_action(
                                            "Execute Generated Command",
                                            &menu::SecondaryConfirm,
                                            cx,
                                        )
                                    })
                                    .on_click(cx.listener(|_, _, cx| {
                                        cx.emit(PromptEditorEvent::ConfirmRequested {
                                            execute: true,
                                        });
                                    }))
                                    .into_any_element(),
                            )
                        }
                    }
                    PromptMode::Transform => {}
                }

                buttons
            }
        }
    }
}

pub enum PromptEditorEvent {
    StartRequested,
    StopRequested,
    ConfirmRequested { execute: bool },
    CancelRequested,
    DismissRequested,
    Resized { height_in_lines: u8 },
}
