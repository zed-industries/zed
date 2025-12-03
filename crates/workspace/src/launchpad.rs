use gpui::{
    Action, AnyElement, App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable,
    IntoElement, ParentElement, Render, Styled, WeakEntity, Window,
};
use theme::ActiveTheme;
use ui::{
    ButtonLike, ButtonSize, Clickable, Divider, DividerColor, FixedWidth, Headline, Icon, IconName,
    IconSize, KeyBinding, Label, LabelCommon, LabelSize, Vector, VectorName, h_flex, prelude::*,
    px, rems, rems_from_px, v_flex,
};
use util::ResultExt;
use zed_actions::{OpenRecent, command_palette};

use crate::{
    Item, NewFile, Open, PathList, SerializedWorkspaceLocation, WORKSPACE_DB, Workspace,
    WorkspaceId,
};

const GET_STARTED: Section<4> = Section {
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
            title: "Clone Repository",
            action: &git::Clone,
        },
        SectionEntry {
            icon: IconName::ListCollapse,
            title: "Open Command Palette",
            action: &command_palette::Toggle,
        },
    ],
};

struct Section<const COLS: usize> {
    title: &'static str,
    entries: [SectionEntry; COLS],
}

impl<const COLS: usize> Section<COLS> {
    fn render(self, index_offset: usize, focus: &FocusHandle, cx: &App) -> impl IntoElement {
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
                    .map(|(index, entry)| entry.render(index_offset + index, focus, cx)),
            )
    }
}

struct SectionEntry {
    icon: IconName,
    title: &'static str,
    action: &'static dyn Action,
}

impl SectionEntry {
    fn render(&self, button_index: usize, focus: &FocusHandle, cx: &App) -> impl IntoElement {
        ButtonLike::new(("launchpad-button", button_index))
            .tab_index(button_index as isize)
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
                    .child(
                        KeyBinding::for_action_in(self.action, focus, cx).size(rems_from_px(12.)),
                    ),
            )
            .on_click(|_, window, cx| window.dispatch_action(self.action.boxed_clone(), cx))
    }
}

pub struct LaunchpadPage {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    recent_workspaces: Option<Vec<(WorkspaceId, SerializedWorkspaceLocation, PathList)>>,
}

impl LaunchpadPage {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, window, |_, _, cx| cx.notify())
            .detach();

        cx.spawn_in(window, async move |this: WeakEntity<Self>, cx| {
            let workspaces = WORKSPACE_DB
                .recent_workspaces_on_disk()
                .await
                .log_err()
                .unwrap_or_default();

            this.update(cx, |this, cx| {
                this.recent_workspaces = Some(workspaces);
                cx.notify();
            })
            .ok();
        })
        .detach();

        Self {
            workspace,
            focus_handle,
            recent_workspaces: None,
        }
    }

    fn render_header(&self, cx: &mut Context<Self>) -> AnyElement {
        h_flex()
            .w_full()
            .justify_center()
            .gap_4()
            .child(Vector::square(VectorName::ZedLogo, rems(2.)))
            .child(
                v_flex().child(Headline::new("Welcome to Zed")).child(
                    Label::new(
                        self.workspace
                            .update(cx, |workspace, _| workspace.app_state().languages.clone())
                            .ok()
                            .map(|_| "The editor for what's next")
                            .unwrap_or("The editor for what's next"),
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .italic(),
                ),
            )
            .into_any_element()
    }

    fn render_recent_project_section(
        &self,
        recent_projects: Vec<impl IntoElement>,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .w_full()
            .child(
                h_flex()
                    .px_1()
                    .mb_2()
                    .gap_2()
                    .child(
                        Label::new("RECENT PROJECTS")
                            .buffer_font(cx)
                            .color(Color::Muted)
                            .size(LabelSize::XSmall),
                    )
                    .child(Divider::horizontal().color(DividerColor::BorderVariant)),
            )
            .child(
                v_flex().children(recent_projects).child(
                    ButtonLike::new("show-more")
                        .full_width()
                        .size(ButtonSize::Medium)
                        .child(
                            h_flex()
                                .w_full()
                                .justify_center()
                                .child(Label::new("Show more...").color(Color::Muted)),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(OpenRecent::default().boxed_clone(), cx);
                        }),
                ),
            )
    }

    fn render_recent_project(
        &self,
        workspace_id: WorkspaceId,
        location: &SerializedWorkspaceLocation,
        paths: &PathList,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let (icon, title) = match location {
            SerializedWorkspaceLocation::Local => {
                let path = paths.paths().first().map(|p| p.as_path());
                let name = path
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Untitled".to_string());
                (IconName::Folder, name)
            }
            SerializedWorkspaceLocation::Remote(_) => {
                (IconName::Server, "Remote Project".to_string())
            }
        };

        let paths = paths.clone();
        let is_local = matches!(location, SerializedWorkspaceLocation::Local);
        let workspace = self.workspace.clone();

        ButtonLike::new(("recent-project", i64::from(workspace_id) as u64))
            .full_width()
            .size(ButtonSize::Medium)
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .child(Icon::new(icon).color(Color::Muted).size(IconSize::XSmall))
                    .child(Label::new(title)),
            )
            .on_click(cx.listener(move |_, _, window, cx| {
                if is_local {
                    let paths = paths.paths().to_vec();
                    let workspace = workspace.clone();
                    cx.spawn_in(window, async move |_, cx| {
                        let _ = workspace.update_in(cx, |workspace, window, cx| {
                            workspace.open_workspace_for_paths(true, paths, window, cx).detach();
                        });
                    })
                    .detach();
                } else {
                    window.dispatch_action(OpenRecent::default().boxed_clone(), cx);
                }
            }))
    }
}

impl Render for LaunchpadPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bg_color = cx.theme().colors().editor_background;
        let header = self.render_header(cx);

        let recent_projects = self
            .recent_workspaces
            .as_ref()
            .into_iter()
            .flatten()
            .take(5)
            .map(|(id, loc, paths)| self.render_recent_project(*id, loc, paths, cx))
            .collect::<Vec<_>>();

        h_flex()
            .size_full()
            .justify_center()
            .bg(bg_color)
            .track_focus(&self.focus_handle(cx))
            .child(
            h_flex()
                .px_12()
                .py_40()
                .size_full()
                .relative()
                .max_w(px(1100.))
                .child(
                    div().size_full().max_w_128().mx_auto().child(header).child(
                        v_flex()
                            .mt_10()
                            .gap_6()
                            .child(GET_STARTED.render(0, &self.focus_handle, cx))
                            .child(self.render_recent_project_section(recent_projects, cx)),
                    ),
                ),
        )
    }
}

impl EventEmitter<DismissEvent> for LaunchpadPage {}

impl Focusable for LaunchpadPage {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for LaunchpadPage {
    type Event = DismissEvent;

    fn tab_content(
        &self,
        params: crate::item::TabContentParams,
        _window: &Window,
        _cx: &App,
    ) -> AnyElement {
        ui::Label::new("Launchpad")
            .color(params.text_color())
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> gpui::SharedString {
        "Launchpad".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Launchpad Page Opened")
    }
}
