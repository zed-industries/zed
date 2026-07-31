use command_palette::humanize_action_name;
use gpui::{App, Context, KeybindingKeystroke, PENDING_INPUT_TIMEOUT, Render, Task, Window};
use settings::Settings;
use std::{rc::Rc, time::Instant};
use ui::{ButtonLike, CircularProgress, KeyBinding, Tooltip, prelude::*, text_for_keystrokes};
use util::ResultExt;
use workspace::{HideStatusItem, StatusBarSettings, StatusItemView, item::ItemHandle};

use crate::FILTERED_KEYSTROKES;

const MAX_TOOLTIP_BINDINGS: usize = 10;

/// A status bar item shown while pending keystrokes both match a key binding and are a prefix of
/// longer bindings, counting down until the shorter binding is applied.
pub struct PendingKeystrokesIndicator {
    pending: Option<PendingKeystrokes>,
    _countdown_task: Option<Task<()>>,
}

#[derive(Clone)]
struct PendingKeystrokes {
    keystrokes: Rc<[KeybindingKeystroke]>,
    started_at: Instant,
    bindings: Vec<(Rc<[KeybindingKeystroke]>, SharedString)>,
}

impl PendingKeystrokesIndicator {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe_pending_input(window, |this: &mut Self, window, cx| {
            this.update_pending(window, cx);
            cx.notify();
        })
        .detach();

        Self {
            pending: None,
            _countdown_task: None,
        }
    }

    fn update_pending(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(keystrokes) = window
            .pending_input_keystrokes()
            .filter(|_| window.pending_input_will_timeout())
        else {
            self.pending = None;
            self._countdown_task = None;
            return;
        };

        let pending_len = keystrokes.len();
        let mut bindings = window
            .possible_bindings_for_input(keystrokes)
            .iter()
            .filter_map(|binding| {
                let binding_keystrokes = binding.keystrokes();
                if binding_keystrokes.len() <= pending_len {
                    return None;
                }
                if FILTERED_KEYSTROKES.iter().any(|filtered| {
                    binding_keystrokes.len() >= filtered.len()
                        && binding_keystrokes[..filtered.len()]
                            .iter()
                            .map(|keystroke| keystroke.inner())
                            .eq(filtered.iter())
                }) {
                    return None;
                }
                let remaining = &binding_keystrokes[pending_len..];
                let remaining_text = text_for_keystrokes(
                    &remaining
                        .iter()
                        .map(|keystroke| keystroke.inner().clone())
                        .collect::<Vec<_>>(),
                    cx,
                );
                Some((
                    remaining_text,
                    remaining.to_vec(),
                    SharedString::from(humanize_action_name(binding.action().name())),
                ))
            })
            .collect::<Vec<_>>();
        bindings.sort_by(|(text_a, keys_a, action_a), (text_b, keys_b, action_b)| {
            keys_a
                .len()
                .cmp(&keys_b.len())
                .then_with(|| text_a.cmp(text_b))
                .then_with(|| action_a.cmp(action_b))
        });
        bindings.dedup_by(|(text_a, _, action_a), (text_b, _, action_b)| {
            text_a == text_b && action_a == action_b
        });

        self.pending = Some(PendingKeystrokes {
            keystrokes: keystrokes
                .iter()
                .map(|keystroke| KeybindingKeystroke::from_keystroke(keystroke.clone()))
                .collect(),
            started_at: Instant::now(),
            bindings: bindings
                .into_iter()
                .map(|(_, keystrokes, action)| (Rc::from(keystrokes), action))
                .collect(),
        });
        self._countdown_task = Some(cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(PENDING_INPUT_TIMEOUT / 30)
                    .await;
                let done = this
                    .update(cx, |this, cx| {
                        cx.notify();
                        this.pending.is_none()
                    })
                    .unwrap_or(true);
                if done {
                    break;
                }
            }
        }));
    }
}

impl Render for PendingKeystrokesIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !StatusBarSettings::get_global(cx).pending_keystrokes_indicator {
            return div().hidden().into_any_element();
        }
        let Some(pending) = self.pending.clone() else {
            return div().hidden().into_any_element();
        };

        let remaining = 1.0
            - (pending.started_at.elapsed().as_secs_f32() / PENDING_INPUT_TIMEOUT.as_secs_f32())
                .clamp(0.0, 1.0);

        ButtonLike::new("pending-keystrokes-indicator")
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        CircularProgress::new(remaining, 1.0, px(13.), cx)
                            .stroke_width(px(2.))
                            .progress_color(cx.theme().colors().text_muted),
                    )
                    .child(
                        KeyBinding::from_keystrokes(pending.keystrokes.clone(), false)
                            .size(rems_from_px(12.)),
                    ),
            )
            .tooltip(Tooltip::element(move |_, _| {
                let pending = &pending;
                v_flex()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_1()
                            .child(KeyBinding::from_keystrokes(
                                pending.keystrokes.clone(),
                                false,
                            ))
                            .child(Label::new("is waiting for more keys").color(Color::Muted)),
                    )
                    .children(pending.bindings.iter().take(MAX_TOOLTIP_BINDINGS).map(
                        |(keystrokes, action)| {
                            h_flex()
                                .gap_2()
                                .child(KeyBinding::from_keystrokes(keystrokes.clone(), false))
                                .child(Label::new(action.clone()).size(LabelSize::Small))
                        },
                    ))
                    .when(pending.bindings.len() > MAX_TOOLTIP_BINDINGS, |el| {
                        el.child(
                            Label::new(format!(
                                "…and {} more",
                                pending.bindings.len() - MAX_TOOLTIP_BINDINGS
                            ))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                        )
                    })
                    .child(
                        Label::new("Click to open the key context view")
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .into_any_element()
            }))
            .on_click(|_, window, cx| {
                if let Some(action) = cx.build_action("dev::OpenKeyContextView", None).log_err() {
                    window.dispatch_action(action, cx);
                }
            })
            .into_any_element()
    }
}

impl StatusItemView for PendingKeystrokesIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings
                .status_bar
                .get_or_insert_default()
                .pending_keystrokes_indicator = Some(false);
        }))
    }
}
