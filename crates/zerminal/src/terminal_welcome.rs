use git::Clone as GitClone;
use gpui::{
    Action, App, Context, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    ParentElement, Render, Styled, WeakEntity, Window, px,
};
use menu::{SelectNext, SelectPrevious};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::{ButtonLike, Divider, DividerColor, KeyBinding, Vector, VectorName, prelude::*};
use util::ResultExt;
use workspace::{
    PathList, SerializedWorkspaceLocation, WORKSPACE_DB, Workspace, WorkspaceId,
    item::{Item, ItemEvent},
};
use zed_actions::OpenSettings;

use crate::{NewTerminalInPlace, OpenProject};

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, JsonSchema, Action)]
#[action(namespace = terminal_welcome)]
#[serde(transparent)]
pub struct OpenRecentProject {
    pub index: usize,
}

#[derive(IntoElement)]
struct SectionHeader {
    title: SharedString,
}

impl SectionHeader {
    fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl RenderOnce for SectionHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
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
            .child(Divider::horizontal().color(DividerColor::BorderVariant))
    }
}

#[derive(IntoElement)]
struct SectionButton {
    label: SharedString,
    icon: IconName,
    action: Box<dyn Action>,
    tab_index: usize,
    focus_handle: FocusHandle,
}

impl SectionButton {
    fn new(
        label: impl Into<SharedString>,
        icon: IconName,
        action: &dyn Action,
        tab_index: usize,
        focus_handle: FocusHandle,
    ) -> Self {
        Self {
            label: label.into(),
            icon,
            action: action.boxed_clone(),
            tab_index,
            focus_handle,
        }
    }
}

impl RenderOnce for SectionButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = format!("onb-button-{}", self.label);
        let action_ref: &dyn Action = &*self.action;

        ButtonLike::new(id)
            .tab_index(self.tab_index as isize)
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
                                    .size(IconSize::Small),
                            )
                            .child(Label::new(self.label)),
                    )
                    .child(
                        KeyBinding::for_action_in(action_ref, &self.focus_handle, cx)
                            .size(rems_from_px(12.)),
                    ),
            )
            .on_click(move |_, window, cx| window.dispatch_action(self.action.boxed_clone(), cx))
    }
}

struct SectionEntry {
    icon: IconName,
    title: &'static str,
    action: &'static dyn Action,
}

impl SectionEntry {
    fn render(&self, button_index: usize, focus: &FocusHandle, _cx: &App) -> impl IntoElement {
        SectionButton::new(
            self.title,
            self.icon,
            self.action,
            button_index,
            focus.clone(),
        )
    }
}

struct Section<const COLS: usize> {
    title: &'static str,
    entries: [SectionEntry; COLS],
}

impl<const COLS: usize> Section<COLS> {
    fn render(self, index_offset: usize, focus: &FocusHandle, cx: &App) -> impl IntoElement {
        v_flex()
            .min_w_full()
            .child(SectionHeader::new(self.title))
            .children(
                self.entries
                    .iter()
                    .enumerate()
                    .map(|(index, entry)| entry.render(index_offset + index, focus, cx)),
            )
    }
}

const CONTENT: (Section<3>, Section<1>) = (
    Section {
        title: "Get Started",
        entries: [
            SectionEntry {
                icon: IconName::Terminal,
                title: "New Terminal",
                action: &NewTerminalInPlace,
            },
            SectionEntry {
                icon: IconName::FolderOpen,
                title: "Open Project",
                action: &OpenProject,
            },
            SectionEntry {
                icon: IconName::CloudDownload,
                title: "Clone Repository",
                action: &GitClone,
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
        ],
    },
);

pub struct TerminalWelcomePage {
    _workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    recent_workspaces: Option<Vec<(WorkspaceId, SerializedWorkspaceLocation, PathList)>>,
}

impl TerminalWelcomePage {
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

        TerminalWelcomePage {
            _workspace: workspace,
            focus_handle,
            recent_workspaces: None,
        }
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
        cx.notify();
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_prev(cx);
        cx.notify();
    }

    fn render_recent_project_section(
        &self,
        recent_projects: Vec<impl IntoElement>,
    ) -> impl IntoElement {
        v_flex()
            .w_full()
            .child(SectionHeader::new("Recent Projects"))
            .children(recent_projects)
    }

    fn render_recent_project(
        &self,
        index: usize,
        location: &SerializedWorkspaceLocation,
        paths: &PathList,
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

        SectionButton::new(
            title,
            icon,
            &OpenRecentProject { index },
            10,
            self.focus_handle.clone(),
        )
    }
}

impl Render for TerminalWelcomePage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (first_section, second_section) = CONTENT;
        let first_section_entries = first_section.entries.len();

        let recent_projects = self
            .recent_workspaces
            .as_ref()
            .into_iter()
            .flatten()
            .take(5)
            .enumerate()
            .map(|(index, (_, loc, paths))| self.render_recent_project(index, loc, paths))
            .collect::<Vec<_>>();

        let second_section = if !recent_projects.is_empty() {
            self.render_recent_project_section(recent_projects)
                .into_any_element()
        } else {
            second_section
                .render(first_section_entries, &self.focus_handle, cx)
                .into_any_element()
        };

        h_flex()
            .key_context("TerminalWelcome")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_next))
            .size_full()
            .justify_center()
            .overflow_hidden()
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .relative()
                    .size_full()
                    .px_12()
                    .max_w(px(1100.))
                    .child(
                        v_flex()
                            .flex_1()
                            .justify_center()
                            .max_w_128()
                            .mx_auto()
                            .gap_6()
                            .overflow_x_hidden()
                            .child(
                                h_flex()
                                    .w_full()
                                    .justify_center()
                                    .mb_4()
                                    .gap_4()
                                    .child(Vector::square(VectorName::ZedLogo, rems_from_px(64.)))
                                    .child(
                                        v_flex().child(Headline::new("Welcome back to Zerminal")).child(
                                            Label::new("Your AI-powered terminal workspace")
                                                .size(LabelSize::Small)
                                                .color(Color::Muted)
                                                .italic(),
                                        ),
                                    ),
                            )
                            .child(first_section.render(Default::default(), &self.focus_handle, cx))
                            .child(second_section),
                    ),
            )
    }
}

impl EventEmitter<ItemEvent> for TerminalWelcomePage {}

impl Focusable for TerminalWelcomePage {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for TerminalWelcomePage {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Welcome".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Terminal Welcome Page Opened")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        f(*event)
    }
}
