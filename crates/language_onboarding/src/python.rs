use db::kvp::Dismissable;
use editor::Editor;
use gpui::{Context, EventEmitter, Subscription};
use ui::{Banner, FluentBuilder as _, prelude::*};
use workspace::{ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace};

pub struct BasedPyrightBanner {
    dismissed: bool,
    have_basedpyright: bool,
    _subscriptions: [Subscription; 1],
}

impl Dismissable for BasedPyrightBanner {
    const KEY: &str = "basedpyright-banner";
}

impl BasedPyrightBanner {
    pub fn new(workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        let subscription = cx.subscribe(workspace.project(), |this, _, event, _| {
            if let project::Event::LanguageServerAdded(_, name, _) = event
                && name == "basedpyright"
            {
                this.have_basedpyright = true;
            }
        });
        let dismissed = Self::dismissed();
        Self {
            dismissed,
            have_basedpyright: false,
            _subscriptions: [subscription],
        }
    }

    fn onboarding_banner_enabled(&self) -> bool {
        !self.dismissed && self.have_basedpyright
    }
}

impl EventEmitter<ToolbarItemEvent> for BasedPyrightBanner {}

impl Render for BasedPyrightBanner {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("basedpyright-banner")
            .when(self.onboarding_banner_enabled(), |el| {
                el.child(
                    Banner::new()
                        .child(
                            v_flex()
                                .gap_0p5()
                                .child(Label::new("Basedpyright is now the only default language server for Python").mt_0p5())
                                .child(Label::new("We have disabled PyRight and pylsp by default. They can be re-enabled in your settings.").size(LabelSize::Small).color(Color::Muted))
                        )
                        .action_slot(
                            h_flex()
                                .gap_0p5()
                                .child(
                                    Button::new("learn-more", "Learn More")
                                        .icon(IconName::ArrowUpRight)
                                        .label_size(LabelSize::Small)
                                        .icon_size(IconSize::XSmall)
                                        .icon_color(Color::Muted)
                                        .on_click(|_, _, cx| {
                                            cx.open_url("https://zed.dev/docs/languages/python")
                                        }),
                                )
                                .child(IconButton::new("dismiss", IconName::Close).icon_size(IconSize::Small).on_click(
                                    cx.listener(|this, _, _, cx| {
                                        this.dismissed = true;
                                        Self::set_dismissed(true, cx);
                                        cx.notify();
                                    }),
                                ))
                        )
                        .into_any_element(),
                )
            })
    }
}

impl ToolbarItemView for BasedPyrightBanner {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::ItemHandle>,
        _window: &mut ui::Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        if !self.onboarding_banner_enabled() {
            return ToolbarItemLocation::Hidden;
        }
        if let Some(item) = active_pane_item
            && let Some(editor) = item.act_as::<Editor>(cx)
            && let Some(path) = editor.update(cx, |editor, cx| editor.target_file_abs_path(cx))
            && let Some(file_name) = path.file_name()
            && file_name.as_encoded_bytes().ends_with(".py".as_bytes())
        {
            return ToolbarItemLocation::Secondary;
        }

        ToolbarItemLocation::Hidden
    }
}
