use crate::ModelProviderInfo;
use gpui::{ClickEvent, prelude::*};
use ui::{IconName, ListItem, ListItemSpacing, prelude::*};

const SPEED_COL_WIDTH: Rems = rems(3.5);
const LATENCY_COL_WIDTH: Rems = rems(3.5);
const PRICE_COL_WIDTH: Rems = rems(5.);
const CHECKMARK_COL_WIDTH: Rems = rems(1.25);

fn format_price_per_million(price_per_million: f64) -> String {
    if price_per_million < 0.01 {
        format!("{:.4}", price_per_million)
    } else if price_per_million < 1.0 {
        format!("{:.2}", price_per_million)
    } else {
        format!("{:.1}", price_per_million)
    }
}

#[derive(IntoElement)]
pub struct GenericProviderListItem {
    id: ElementId,
    provider: ModelProviderInfo,
    is_selected: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl GenericProviderListItem {
    pub fn new(id: impl Into<ElementId>, provider: ModelProviderInfo) -> Self {
        Self {
            id: id.into(),
            provider,
            is_selected: false,
            on_click: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for GenericProviderListItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let throughput = self.provider.throughput_tps.unwrap_or(0.0);
        let latency = self.provider.latency_ms.unwrap_or(0.0);

        let input_price = self
            .provider
            .input_price_per_million
            .map(format_price_per_million)
            .unwrap_or_else(|| "N/A".to_string());
        let output_price = self
            .provider
            .output_price_per_million
            .map(format_price_per_million)
            .unwrap_or_else(|| "N/A".to_string());

        ListItem::new(self.id)
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(self.is_selected)
            .when_some(self.on_click, |this, handler| this.on_click(handler))
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .child(div().w(CHECKMARK_COL_WIDTH).flex_shrink_0().when(
                        self.is_selected,
                        |this| {
                            this.child(
                                Icon::new(IconName::Check)
                                    .size(IconSize::Small)
                                    .color(Color::Accent),
                            )
                        },
                    ))
                    .child(
                        v_flex()
                            .flex_1()
                            .overflow_hidden()
                            .child(
                                Label::new(self.provider.display_name.to_string())
                                    .size(LabelSize::Small)
                                    .truncate(),
                            )
                            .when_some(self.provider.quantization.as_ref(), |this, q| {
                                this.child(
                                    Label::new(q.to_string())
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted),
                                )
                            }),
                    )
                    .child(
                        div().w(SPEED_COL_WIDTH).flex_shrink_0().child(
                            Label::new(format!("{:.0}", throughput)).size(LabelSize::XSmall),
                        ),
                    )
                    .child(
                        div()
                            .w(LATENCY_COL_WIDTH)
                            .flex_shrink_0()
                            .child(Label::new(format!("{:.0}ms", latency)).size(LabelSize::XSmall)),
                    )
                    .child(
                        v_flex()
                            .w(PRICE_COL_WIDTH)
                            .flex_shrink_0()
                            .items_end()
                            .gap_0p5()
                            .child(
                                h_flex()
                                    .gap_0p5()
                                    .child(Label::new("$").size(LabelSize::XSmall))
                                    .child(Label::new(input_price).size(LabelSize::XSmall))
                                    .child(
                                        Label::new("/M in")
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .gap_0p5()
                                    .child(Label::new("$").size(LabelSize::XSmall))
                                    .child(Label::new(output_price).size(LabelSize::XSmall))
                                    .child(
                                        Label::new("/M out")
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted),
                                    ),
                            ),
                    ),
            )
    }
}

#[derive(IntoElement)]
pub struct ProviderSelectorHeader {
    reset_button_disabled: bool,
    on_reset: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Default for ProviderSelectorHeader {
    fn default() -> Self {
        Self {
            reset_button_disabled: true,
            on_reset: None,
        }
    }
}

impl ProviderSelectorHeader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset_button(
        mut self,
        disabled: bool,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.reset_button_disabled = disabled;
        self.on_reset = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for ProviderSelectorHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .px_2()
            .pb_1()
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .items_center()
                    .child(
                        Label::new("Select Provider")
                            .size(LabelSize::Default)
                            .color(Color::Muted),
                    )
                    .when_some(self.on_reset, |this, handler| {
                        this.child(
                            Button::new("reset-to-auto", "Reset to Auto")
                                .style(ButtonStyle::Subtle)
                                .label_size(LabelSize::Small)
                                .disabled(self.reset_button_disabled)
                                .on_click(handler),
                        )
                    }),
            )
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .mt_1()
                    .child(div().w(CHECKMARK_COL_WIDTH).flex_shrink_0())
                    .child(
                        h_flex().flex_1().child(
                            Label::new("Provider")
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        ),
                    )
                    .child(
                        div().w(SPEED_COL_WIDTH).flex_shrink_0().child(
                            Label::new("tok/s")
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        ),
                    )
                    .child(
                        div().w(LATENCY_COL_WIDTH).flex_shrink_0().child(
                            Label::new("Latency")
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        ),
                    )
                    .child(
                        div().w(PRICE_COL_WIDTH).flex_shrink_0().child(
                            h_flex().justify_end().child(
                                Label::new("Price")
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            ),
                        ),
                    ),
            )
            .child(
                div()
                    .mt_1()
                    .h_px()
                    .w_full()
                    .bg(cx.theme().colors().border_variant),
            )
    }
}

#[derive(IntoElement)]
pub struct ProviderSelectorLoading;

impl RenderOnce for ProviderSelectorLoading {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().p_4().child(
            h_flex()
                .justify_center()
                .child(Label::new("Loading providers...").color(Color::Muted)),
        )
    }
}
