use std::time::Duration;

use acp_thread::MentionUri;
use gpui::{
    Animation, AnimationExt, AnyView, IntoElement, SharedString, Window, pulsating_between,
};
use settings::Settings;
use theme::ThemeSettings;
use ui::{ButtonLike, ElevationIndex, TintColor, Tooltip, prelude::*};
use workspace::Workspace;

use crate::acp::AcpThreadView;

#[derive(IntoElement)]
pub struct MentionCrease {
    id: ElementId,
    mention: MentionUri,
    icon: Option<SharedString>,
    label: Option<SharedString>,
    is_toggled: bool,
    is_loading: bool,
    tooltip: Option<SharedString>,
    layer: Option<ElevationIndex>,
    image_preview: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
}

impl MentionCrease {
    pub fn new(id: impl Into<ElementId>, mention: MentionUri) -> Self {
        Self {
            id: id.into(),
            mention,
            icon: None,
            label: None,
            is_toggled: false,
            is_loading: false,
            tooltip: None,
            image_preview: None,
            layer: None,
        }
    }

    pub fn is_toggled(mut self, is_toggled: bool) -> Self {
        self.is_toggled = is_toggled;
        self
    }

    pub fn is_loading(mut self, is_loading: bool) -> Self {
        self.is_loading = is_loading;
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn layer(mut self, layer: ElevationIndex) -> Self {
        self.layer = Some(layer);
        self
    }

    pub fn image_preview(
        mut self,
        builder: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        self.image_preview = Some(Box::new(builder));
        self
    }
}

impl RenderOnce for MentionCrease {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let font_size = settings.agent_buffer_font_size(cx);
        let buffer_font = settings.buffer_font.clone();

        let button_height = DefiniteLength::Absolute(AbsoluteLength::Pixels(
            px(window.line_height().into()) - px(1.),
        ));

        let mention = self.mention.clone();
        let icon = self.icon.clone().unwrap_or_else(|| mention.icon_path(cx));
        let label = self.label.clone().unwrap_or_else(|| mention.name().into());
        let tooltip = self.tooltip.clone().or_else(|| match &mention {
            MentionUri::File { .. } => Some("Open File".into()),
            MentionUri::Directory { .. } => Some("Reveal in Project Panel".into()),
            MentionUri::Symbol { .. } => Some("Show Symbol location".into()),
            MentionUri::Selection { .. } => Some("Show Selection location".into()),
            MentionUri::Thread { .. } | MentionUri::TextThread { .. } => Some("Open Thread".into()),
            MentionUri::Rule { .. } => Some("Open Rule".into()),
            MentionUri::Fetch { .. } => Some("Open Link".into()),
            MentionUri::PastedImage => Some("Open Image".into()),
            MentionUri::TerminalSelection { .. } => Some("Show Terminal Selection".into()),
            MentionUri::Diagnostics { .. } => None,
        });
        let image_preview = self.image_preview;
        let button = ButtonLike::new(self.id.clone())
            .style(ButtonStyle::Outlined)
            .size(ButtonSize::Compact)
            .height(button_height)
            .selected_style(ButtonStyle::Tinted(TintColor::Accent))
            .toggle_state(self.is_toggled)
            .child(
                h_flex()
                    .pb_px()
                    .gap_1()
                    .font(buffer_font)
                    .text_size(font_size)
                    .child(
                        Icon::from_path(icon.clone())
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(label.clone())
                    .map(|this| {
                        if self.is_loading {
                            this.with_animation(
                                "loading-context-crease",
                                Animation::new(Duration::from_secs(2))
                                    .repeat()
                                    .with_easing(pulsating_between(0.4, 0.8)),
                                |label, delta| label.opacity(delta),
                            )
                            .into_any()
                        } else {
                            this.into_any()
                        }
                    }),
            )
            .when_some(self.layer, |b, layer| b.layer(layer))
            .when_else(
                image_preview.is_some(),
                |b| b.hoverable_tooltip(image_preview.expect("checked preview presence")),
                |b| b.when_some(tooltip, |b, t| b.tooltip(Tooltip::text(t))),
            )
            .on_click(move |_event, window, cx| {
                if let Some(workspace) = window.root::<Workspace>().flatten() {
                    AcpThreadView::open_mention(&mention, &workspace.downgrade(), window, cx);
                }
            });

        button
    }
}
