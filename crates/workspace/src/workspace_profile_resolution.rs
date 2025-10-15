use gpui::{App, Entity, EventEmitter, FocusHandle, Focusable, WeakEntity, actions};
use settings::SettingsStore;
use ui::{Headline, HeadlineSize, Label, prelude::*};

use crate::{Item, PathMatch, SplitDirection, Workspace};

actions!(
    dev,
    [
        /// Opens the workspace profile resolution debug view.
        OpenWorkspaceProfileResolution
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(
            |workspace, _: &OpenWorkspaceProfileResolution, window, cx| {
                let weak_workspace = cx.entity().downgrade();
                let view = cx.new(|cx| WorkspaceProfileResolutionView::new(weak_workspace, cx));
                workspace.split_item(SplitDirection::Right, Box::new(view), window, cx);
            },
        );
    })
    .detach();
}

struct WorkspaceProfileResolutionView {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
}

impl WorkspaceProfileResolutionView {
    pub fn new(workspace: WeakEntity<Workspace>, cx: &mut Context<Self>) -> Self {
        Self {
            workspace,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl EventEmitter<()> for WorkspaceProfileResolutionView {}

impl Focusable for WorkspaceProfileResolutionView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for WorkspaceProfileResolutionView {
    type Event = ();

    fn to_item_events(_: &Self::Event, _: impl FnMut(crate::item::ItemEvent)) {}

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Workspace Profile Resolution".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<crate::WorkspaceId>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>>
    where
        Self: Sized,
    {
        Some(cx.new(|cx| Self::new(self.workspace.clone(), cx)))
    }
}

impl Render for WorkspaceProfileResolutionView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let workspace_entity = self.workspace.upgrade();

        let root_paths = workspace_entity
            .as_ref()
            .map(|ws| ws.read(cx).root_paths(cx))
            .unwrap_or_default();

        let settings_store = cx.global::<SettingsStore>();
        let user_settings = settings_store.raw_user_settings();

        let workspace_profiles: Vec<(String, String, PathMatch)> = user_settings
            .map(|settings| {
                settings
                    .workspace_profiles
                    .iter()
                    .map(|(name, profile)| {
                        let match_result = workspace_entity
                            .as_ref()
                            .map(|ws| ws.read(cx).match_path(&profile.path, cx))
                            .unwrap_or(PathMatch::None);
                        (name.clone(), profile.path.clone(), match_result)
                    })
                    .collect()
            })
            .unwrap_or_default();

        let winner = workspace_entity.as_ref().and_then(|ws| {
            user_settings.and_then(|settings| {
                ws.read(cx)
                    .winning_workspace_profile(&settings.workspace_profiles, cx)
            })
        });

        // Separate matched and not matched profiles
        let (matched_profiles, not_matched_profiles): (Vec<_>, Vec<_>) = workspace_profiles
            .into_iter()
            .partition(|(_, _, match_result)| *match_result != PathMatch::None);

        v_flex()
            .id("workspace-profile-resolution-view")
            .size_full()
            .p_4()
            .overflow_scroll()
            .track_focus(&self.focus_handle)
            .child(Headline::new("Workspace Profile Resolution").size(HeadlineSize::Large))
            .child(
                h_flex()
                    .gap_2()
                    .mt_2()
                    .child(Label::new("CURRENT"))
                    .when_some(winner, |el, (name, _)| {
                        el.child(Label::new(format!("🏆 \"{}\"", name)).color(ui::Color::Selected))
                    })
                    .when(winner.is_none(), |el| el.child(Label::new("None"))),
            )
            .child(Headline::new("Workspace Root Paths").size(HeadlineSize::Medium))
            .children(
                root_paths
                    .iter()
                    .map(|path| Label::new(format!("• {}", path.display())).ml_4().mt_1()),
            )
            .when(root_paths.is_empty(), |el| {
                el.child(Label::new("No workspace roots found").ml_4().mt_1())
            })
            .when(!matched_profiles.is_empty(), |el| {
                el.child(Headline::new("Workspace Profiles Matches").size(HeadlineSize::Medium))
                    .children(matched_profiles.iter().map(|(name, path, match_result)| {
                        let is_winner = winner.map(|(w, _)| w == name.as_str()).unwrap_or(false);

                        let (match_label, match_color) = match match_result {
                            PathMatch::Exact => ("MATCH", ui::Color::Success),
                            PathMatch::Glob => ("GLOB MATCH", ui::Color::Warning),
                            PathMatch::None => unreachable!(),
                        };
                        let trophy = if is_winner { "🏆 " } else { "" };
                        let name_label_color = if is_winner {
                            ui::Color::Selected
                        } else {
                            ui::Color::Muted
                        };

                        v_flex()
                            .gap_1()
                            .ml_4()
                            .mt_2()
                            .child(
                                Label::new(format!("{}{}", trophy, match_label)).color(match_color),
                            )
                            .child(
                                Label::new(format!("\"{}\"", name))
                                    .ml_4()
                                    .color(name_label_color),
                            )
                            .child(
                                Label::new(format!("\"{}\"", path))
                                    .ml_8()
                                    .color(ui::Color::Muted),
                            )
                            .mb_2()
                    }))
            })
            .when(!not_matched_profiles.is_empty(), |el| {
                el.child(Headline::new("Workspace Profiles Not Matched").size(HeadlineSize::Medium))
                    .children(not_matched_profiles.iter().map(|(name, path, _)| {
                        v_flex()
                            .gap_1()
                            .ml_4()
                            .mt_2()
                            .child(Label::new(name.clone()))
                            .child(Label::new(format!("\"{}\"", path)).ml_4())
                    }))
            })
    }
}
