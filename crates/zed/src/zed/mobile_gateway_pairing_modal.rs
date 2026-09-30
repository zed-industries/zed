use gpui::{
    App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, InteractiveElement as _,
    Render, Window, black, px, white,
};
use qrcode::{Color as QrColor, QrCode};
use std::time::Duration;
use ui::{
    ActiveTheme, Color, FluentBuilder, IntoElement, Label, LabelCommon, LabelSize, ParentElement,
    Styled, StyledExt, div, h_flex, v_flex,
};
use workspace::ModalView;

const QR_MODULE_SIZE: f32 = 5.0;
const QR_QUIET_ZONE_MODULES: f32 = 4.0;

struct QrGrid {
    width: usize,
    modules: Vec<bool>,
}

pub struct MobileGatewayPairingModal {
    focus_handle: FocusHandle,
    qr_grid: Option<QrGrid>,
    expires_in: Option<Duration>,
    bind_address: Option<String>,
    error: Option<String>,
}

impl MobileGatewayPairingModal {
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        let pairing = crate::mobile_gateway::begin_pairing(cx);
        let (qr_grid, expires_in, bind_address, error) = match pairing {
            Ok(pairing) => match QrCode::new(pairing.payload.as_bytes()) {
                Ok(qr_code) => (
                    Some(QrGrid {
                        width: qr_code.width(),
                        modules: qr_code
                            .into_colors()
                            .into_iter()
                            .map(|color| color == QrColor::Dark)
                            .collect(),
                    }),
                    Some(pairing.expires_in),
                    Some(pairing.bind_address),
                    None,
                ),
                Err(error) => {
                    log::error!("Could not generate a Mobile Gateway pairing QR code: {error}");
                    (
                        None,
                        None,
                        None,
                        Some("Zed could not generate a pairing code. Close this dialog and try again.".into()),
                    )
                }
            },
            Err(error) => (None, None, None, Some(error)),
        };

        Self {
            focus_handle: cx.focus_handle(),
            qr_grid,
            expires_in,
            bind_address,
            error,
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for MobileGatewayPairingModal {}
impl ModalView for MobileGatewayPairingModal {}

impl Focusable for MobileGatewayPairingModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MobileGatewayPairingModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let expiration = self.expires_in.map(format_expiration);

        v_flex()
            .key_context("MobileGatewayPairingModal")
            .on_action(cx.listener(Self::cancel))
            .elevation_3(cx)
            .w_96()
            .overflow_hidden()
            .child(
                div()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(theme.colors().border_variant)
                    .child(Label::new("Connect mobile device").size(LabelSize::Large)),
            )
            .child(
                v_flex()
                    .gap_4()
                    .px_4()
                    .py_3()
                    .bg(theme.colors().editor_background)
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                Label::new("Open Zed on your mobile device and scan this code to start pairing.")
                                    .size(LabelSize::Small),
                            )
                            .when_some(expiration, |this, expiration| {
                                this.child(
                                    Label::new(format!("This pairing code expires in {expiration}."))
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                            })
                            .when_some(self.bind_address.as_ref(), |this, bind_address| {
                                this.child(
                                    Label::new(format!(
                                        "Your mobile device must be able to reach {bind_address}."
                                    ))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                                )
                            }),
                    )
                    .when_some(self.qr_grid.as_ref(), |this, qr_grid| {
                        this.child(
                            h_flex()
                                .w_full()
                                .justify_center()
                                .child(render_qr_grid(qr_grid)),
                        )
                    })
                    .when_some(self.error.as_ref(), |this, error| {
                        this.child(Label::new(error.clone()).size(LabelSize::Small).color(Color::Error))
                    })
                    .when(self.qr_grid.is_some(), |this| {
                        this.child(
                            Label::new("Do not share this pairing code. Press Escape to close.")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    }),
            )
    }
}

fn render_qr_grid(qr_grid: &QrGrid) -> impl IntoElement {
    div()
        .p(px(QR_MODULE_SIZE * QR_QUIET_ZONE_MODULES))
        .bg(white())
        .child(
            v_flex()
                .gap_0()
                .children(qr_grid.modules.chunks(qr_grid.width).map(|row| {
                    h_flex().gap_0().children(row.iter().map(|module| {
                        div()
                            .size(px(QR_MODULE_SIZE))
                            .bg(if *module { black() } else { white() })
                    }))
                })),
        )
}

fn format_expiration(expires_in: Duration) -> String {
    let seconds = expires_in.as_secs();
    if seconds == 1 {
        "1 second".into()
    } else {
        format!("{seconds} seconds")
    }
}
