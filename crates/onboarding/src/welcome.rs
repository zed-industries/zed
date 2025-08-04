use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    NoAction, ParentElement, Render, Styled, Window, actions,
};
use ui::{ButtonLike, Divider, DividerColor, KeyBinding, Vector, VectorName, prelude::*};
use workspace::{
    NewFile, Open, WorkspaceId,
    item::{Item, ItemEvent},
    with_active_or_new_workspace,
};
use zed_actions::{Extensions, OpenSettings, agent, command_palette};

use crate::{Onboarding, OpenOnboarding};

actions!(
    zed,
    [
        /// Show the Zed welcome screen
        ShowWelcome
    ]
);

const CONTENT: (Section<4>, Section<3>) = (
    Section {
        title: "Get Started",
        entries: [
            SectionEntry {
                icon: IconName::Plus,
                title: "New File",
                action: &NewFile,
            },
            SectionEntry {
                icon: IconName::FolderOpen,
                title: "Open Project",
                action: &Open,
            },
            SectionEntry {
                icon: IconName::CloudDownload,
                title: "Clone a Repo",
                // TODO: use proper action
                action: &NoAction,
            },
            SectionEntry {
                icon: IconName::ListCollapse,
                title: "Open Command Palette",
                action: &command_palette::Toggle,
            },
        ],
    },
    Section {
        title: "Configure",
        entries: [
            SectionEntry {
                icon: IconName::Settings,
                title: "Open Settings",
                action: &OpenSettings,
            },
            SectionEntry {
                icon: IconName::ZedAssistant,
                title: "View AI Settings",
                action: &agent::OpenSettings,
            },
            SectionEntry {
                icon: IconName::Blocks,
                title: "Explore Extensions",
                action: &Extensions {
                    category_filter: None,
                    id: None,
                },
            },
        ],
    },
);

struct Section<const COLS: usize> {
    title: &'static str,
    entries: [SectionEntry; COLS],
}

impl<const COLS: usize> Section<COLS> {
    fn render(
        self,
        index_offset: usize,
        focus: &FocusHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        v_flex()
            .min_w_full()
            .child(
                h_flex()
                    .px_1()
                    .mb_2()
                    .gap_2()
                    .child(
                        Label::new(self.title.to_ascii_uppercase())
                            .buffer_font(cx)
                            .color(Color::Muted)
                            .size(LabelSize::XSmall),
                    )
                    .child(Divider::horizontal().color(DividerColor::BorderVariant)),
            )
            .children(
                self.entries
                    .iter()
                    .enumerate()
                    .map(|(index, entry)| entry.render(index_offset + index, &focus, window, cx)),
            )
    }
}

struct SectionEntry {
    icon: IconName,
    title: &'static str,
    action: &'static dyn Action,
}

impl SectionEntry {
    fn render(
        &self,
        button_index: usize,
        focus: &FocusHandle,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        ButtonLike::new(("onboarding-button-id", button_index))
            .full_width()
            .size(ButtonSize::Medium)
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Icon::new(self.icon)
                                    .color(Color::Muted)
                                    .size(IconSize::XSmall),
                            )
                            .child(Label::new(self.title)),
                    )
                    .children(
                        KeyBinding::for_action_in(self.action, focus, window, cx)
                            .map(|s| s.size(rems_from_px(12.))),
                    ),
            )
            .on_click(|_, window, cx| window.dispatch_action(self.action.boxed_clone(), cx))
    }
}

pub struct WelcomePage {
    focus_handle: FocusHandle,
}

impl Render for WelcomePage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (first_section, second_entries) = CONTENT;
        let first_section_entries = first_section.entries.len();

        h_flex()
            .size_full()
            .justify_center()
            .overflow_hidden()
            .bg(cx.theme().colors().editor_background)
            .key_context("Welcome")
            .track_focus(&self.focus_handle(cx))
            .child(
                h_flex()
                    .px_12()
                    .py_40()
                    .size_full()
                    .relative()
                    .max_w(px(1100.))
                    .child(
                        div()
                            .size_full()
                            .max_w_128()
                            .mx_auto()
                            .child(
                                h_flex()
                                    .w_full()
                                    .justify_center()
                                    .gap_4()
                                    .child(Vector::square(VectorName::ZedLogo, rems(2.)))
                                    .child(
                                        div().child(Headline::new("Welcome to Zed")).child(
                                            Label::new("The editor for what's next")
                                                .size(LabelSize::Small)
                                                .color(Color::Muted)
                                                .italic(),
                                        ),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .mt_10()
                                    .gap_6()
                                    .child(first_section.render(
                                        Default::default(),
                                        &self.focus_handle,
                                        window,
                                        cx,
                                    ))
                                    .child(second_entries.render(
                                        first_section_entries,
                                        &self.focus_handle,
                                        window,
                                        cx,
                                    ))
                                    .child(
                                        h_flex()
                                            .w_full()
                                            .pt_4()
                                            .justify_center()
                                            // We call this a hack
                                            .rounded_b_xs()
                                            .border_t_1()
                                            .border_color(cx.theme().colors().border.opacity(0.6))
                                            .border_dashed()
                                            .child(
                                                    Button::new("welcome-exit", "Return to Setup")
                                                        .full_width()
                                                        .label_size(LabelSize::XSmall)
                                                        .on_click(|_, window, cx| {
                                                            window.dispatch_action(
                                                                OpenOnboarding.boxed_clone(),
                                                                cx,
                                                            );

                                                            with_active_or_new_workspace(cx, |workspace, window, cx| {
                                                                let Some((welcome_id, welcome_idx)) = workspace
                                                                    .active_pane()
                                                                    .read(cx)
                                                                    .items()
                                                                    .enumerate()
                                                                    .find_map(|(idx, item)| {
                                                                        let _ = item.downcast::<WelcomePage>()?;
                                                                        Some((item.item_id(), idx))
                                                                    })
                                                                else {
                                                                    return;
                                                                };

                                                                workspace.active_pane().update(cx, |pane, cx| {
                                                                    // Get the index here to get around the borrow checker
                                                                    let idx = pane.items().enumerate().find_map(
                                                                        |(idx, item)| {
                                                                            let _ =
                                                                                item.downcast::<Onboarding>()?;
                                                                            Some(idx)
                                                                        },
                                                                    );

                                                                    if let Some(idx) = idx {
                                                                        pane.activate_item(
                                                                            idx, true, true, window, cx,
                                                                        );
                                                                    } else {
                                                                        let item =
                                                                            Box::new(Onboarding::new(workspace, cx));
                                                                        pane.add_item(
                                                                            item,
                                                                            true,
                                                                            true,
                                                                            Some(welcome_idx),
                                                                            window,
                                                                            cx,
                                                                        );
                                                                    }

                                                                    pane.remove_item(
                                                                        welcome_id,
                                                                        false,
                                                                        false,
                                                                        window,
                                                                        cx,
                                                                    );
                                                                });
                                                            });
                                                        }),
                                                ),
                                    ),
                            ),
                    ),
            )
    }
}

impl WelcomePage {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, window, |_, _, cx| cx.notify())
                .detach();

            WelcomePage { focus_handle }
        })
    }
}

impl EventEmitter<ItemEvent> for WelcomePage {}

impl Focusable for WelcomePage {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for WelcomePage {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Welcome".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("New Welcome Page Opened")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Entity<Self>> {
        None
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
