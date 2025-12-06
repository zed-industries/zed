use crate::{
    NewFile, Open, PathList, SerializedWorkspaceLocation, WORKSPACE_DB, Workspace, WorkspaceId,
    item::{Item, ItemEvent},
};
use git::Clone as GitClone;
use gpui::WeakEntity;
use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    ParentElement, Render, Styled, Task, Window, actions,
};
use menu::{SelectNext, SelectPrevious};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::{ButtonLike, Divider, DividerColor, KeyBinding, Vector, VectorName, prelude::*};
use util::ResultExt;
use zed_actions::{Extensions, OpenOnboarding, OpenSettings, agent, command_palette};

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, JsonSchema, Action)]
#[action(namespace = welcome)]
#[serde(transparent)]
pub struct OpenRecentProject {
    pub index: usize,
}

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
                title: "Clone Repository",
                action: &GitClone,
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
        ButtonLike::new(("onboarding-button-id", button_index))
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

pub struct WelcomePage {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    fallback_to_recent_projects: bool,
    recent_workspaces: Option<Vec<(WorkspaceId, SerializedWorkspaceLocation, PathList)>>,
}

impl WelcomePage {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        fallback_to_recent_projects: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, window, |_, _, cx| cx.notify())
            .detach();

        if fallback_to_recent_projects {
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
        }

        WelcomePage {
            workspace,
            focus_handle,
            fallback_to_recent_projects,
            recent_workspaces: None,
        }
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next();
        cx.notify();
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_prev();
        cx.notify();
    }

    fn open_recent_project(
        &mut self,
        action: &OpenRecentProject,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(recent_workspaces) = &self.recent_workspaces {
            if let Some((_workspace_id, location, paths)) = recent_workspaces.get(action.index) {
                let paths = paths.clone();
                let location = location.clone();
                let is_local = matches!(location, SerializedWorkspaceLocation::Local);
                let workspace = self.workspace.clone();

                if is_local {
                    let paths = paths.paths().to_vec();
                    cx.spawn_in(window, async move |_, cx| {
                        let _ = workspace.update_in(cx, |workspace, window, cx| {
                            workspace
                                .open_workspace_for_paths(true, paths, window, cx)
                                .detach();
                        });
                    })
                    .detach();
                } else {
                    use zed_actions::OpenRecent;
                    window.dispatch_action(OpenRecent::default().boxed_clone(), cx);
                }
            }
        }
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
                            use zed_actions::OpenRecent;
                            window.dispatch_action(OpenRecent::default().boxed_clone(), cx);
                        }),
                ),
            )
    }

    fn render_recent_project(
        &self,
        index: usize,
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

        ButtonLike::new(("recent-project", i64::from(workspace_id) as u64))
            .full_width()
            .size(ButtonSize::Medium)
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Icon::new(icon).color(Color::Muted).size(IconSize::XSmall))
                            .child(Label::new(title)),
                    )
                    .child(
                        KeyBinding::for_action_in(
                            &OpenRecentProject { index },
                            &self.focus_handle,
                            cx,
                        )
                        .size(rems_from_px(12.)),
                    ),
            )
            .on_click(move |_, window, cx| {
                window.dispatch_action(OpenRecentProject { index }.boxed_clone(), cx);
            })
    }
}

impl Render for WelcomePage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (first_section, second_section) = CONTENT;
        let first_section_entries = first_section.entries.len();
        let last_index = first_section_entries + second_section.entries.len();

        let recent_projects = self
            .recent_workspaces
            .as_ref()
            .into_iter()
            .flatten()
            .take(5)
            .enumerate()
            .map(|(index, (id, loc, paths))| self.render_recent_project(index, *id, loc, paths, cx))
            .collect::<Vec<_>>();

        let second_section_element =
            if self.fallback_to_recent_projects && !recent_projects.is_empty() {
                self.render_recent_project_section(recent_projects, cx)
                    .into_any_element()
            } else {
                second_section
                    .render(first_section_entries, &self.focus_handle, cx)
                    .into_any_element()
            };

        h_flex()
            .size_full()
            .justify_center()
            .overflow_hidden()
            .bg(cx.theme().colors().editor_background)
            .key_context("Welcome")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::open_recent_project))
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
                                        cx,
                                    ))
                                    .child(second_section_element)
                                    .when(!self.fallback_to_recent_projects, |this| {
                                        this.child(
                                            h_flex()
                                                .w_full()
                                                .pt_4()
                                                .justify_center()
                                                // We call this a hack
                                                .rounded_b_xs()
                                                .border_t_1()
                                                .border_color(
                                                    cx.theme().colors().border.opacity(0.6),
                                                )
                                                .border_dashed()
                                                .child(
                                                    Button::new("welcome-exit", "Return to Setup")
                                                        .tab_index(last_index as isize)
                                                        .full_width()
                                                        .label_size(LabelSize::XSmall)
                                                        .on_click(|_, window, cx| {
                                                            window.dispatch_action(
                                                                OpenOnboarding.boxed_clone(),
                                                                cx,
                                                            );
                                                        }),
                                                ),
                                        )
                                    }),
                            ),
                    ),
            )
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

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(crate::item::ItemEvent)) {
        f(*event)
    }
}

impl crate::SerializableItem for WelcomePage {
    fn serialized_item_kind() -> &'static str {
        "WelcomePage"
    }

    fn cleanup(
        workspace_id: crate::WorkspaceId,
        alive_items: Vec<crate::ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<()>> {
        crate::delete_unloaded_items(
            alive_items,
            workspace_id,
            "welcome_pages",
            &persistence::WELCOME_PAGES,
            cx,
        )
    }

    fn deserialize(
        _project: Entity<project::Project>,
        workspace: gpui::WeakEntity<Workspace>,
        workspace_id: crate::WorkspaceId,
        item_id: crate::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<Entity<Self>>> {
        if persistence::WELCOME_PAGES
            .get_welcome_page(item_id, workspace_id)
            .ok()
            .is_some_and(|is_open| is_open)
        {
            Task::ready(Ok(
                cx.new(|cx| WelcomePage::new(workspace, false, window, cx))
            ))
        } else {
            Task::ready(Err(anyhow::anyhow!("No welcome page to deserialize")))
        }
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: crate::ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<gpui::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        Some(cx.background_spawn(async move {
            persistence::WELCOME_PAGES
                .save_welcome_page(item_id, workspace_id, true)
                .await
        }))
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        event == &ItemEvent::UpdateTab
    }
}

mod persistence {
    use crate::WorkspaceDb;
    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };

    pub struct WelcomePagesDb(ThreadSafeConnection);

    impl Domain for WelcomePagesDb {
        const NAME: &str = stringify!(WelcomePagesDb);

        const MIGRATIONS: &[&str] = (&[sql!(
                    CREATE TABLE welcome_pages (
                        workspace_id INTEGER,
                        item_id INTEGER UNIQUE,
                        is_open INTEGER DEFAULT FALSE,

                        PRIMARY KEY(workspace_id, item_id),
                        FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                        ON DELETE CASCADE
                    ) STRICT;
        )]);
    }

    db::static_connection!(WELCOME_PAGES, WelcomePagesDb, [WorkspaceDb]);

    impl WelcomePagesDb {
        query! {
            pub async fn save_welcome_page(
                item_id: crate::ItemId,
                workspace_id: crate::WorkspaceId,
                is_open: bool
            ) -> Result<()> {
                INSERT OR REPLACE INTO welcome_pages(item_id, workspace_id, is_open)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_welcome_page(
                item_id: crate::ItemId,
                workspace_id: crate::WorkspaceId
            ) -> Result<bool> {
                SELECT is_open
                FROM welcome_pages
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
