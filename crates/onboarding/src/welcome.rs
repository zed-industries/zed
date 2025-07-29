use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    NoAction, ParentElement, Render, Styled, Window, actions,
};
use ui::{ButtonLike, Divider, DividerColor, KeyBinding, Vector, VectorName, prelude::*};
use workspace::{
    NewFile, Open, Workspace, WorkspaceId,
    item::{Item, ItemEvent},
};
use zed_actions::{Extensions, OpenSettings, command_palette};

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
                // TODO: use proper action
                action: &NoAction,
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
            .gap_2()
            .child(
                h_flex()
                    .px_1()
                    .gap_4()
                    .child(
                        Label::new(self.title.to_ascii_uppercase())
                            .buffer_font(cx)
                            .color(Color::Muted)
                            .size(LabelSize::XSmall),
                    )
                    .child(Divider::horizontal().color(DividerColor::Border)),
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
            .child(
                h_flex()
                    .w_full()
                    .gap_1()
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
                    .children(KeyBinding::for_action_in(self.action, focus, window, cx)),
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
                                    .mt_12()
                                    .gap_8()
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
                                            .border_color(DividerColor::Border.hsla(cx))
                                            .border_dashed()
                                            .child(
                                                div().child(
                                                    Button::new("welcome-exit", "Return to Setup")
                                                        .full_width()
                                                        .label_size(LabelSize::XSmall),
                                                ),
                                            ),
                                    ),
                            ),
                    ),
            )
    }
}

impl WelcomePage {
    pub fn new(cx: &mut Context<Workspace>) -> Entity<Self> {
        let this = cx.new(|cx| WelcomePage {
            focus_handle: cx.focus_handle(),
        });

        this
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
