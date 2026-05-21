use agent_skills::{Skill, SkillIndex};
use fs::RemoveOptions;
use gpui::{Action as _, ScrollHandle, SharedString, prelude::*};

use ui::{Divider, Tooltip, prelude::*};
use util::ResultExt as _;

use crate::{SettingsUiFile, SettingsWindow};

pub(crate) fn render_skills_setup_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let skill_index = cx.try_global::<SkillIndex>();

    // Pick skills that match the current settings file tab:
    // - User tab → global skills only
    // - Project tab → project-local skills for that worktree only
    let skills: Vec<Skill> = match &settings_window.current_file {
        SettingsUiFile::User => skill_index
            .map(|idx| idx.global_skills.clone())
            .unwrap_or_default(),
        SettingsUiFile::Project((worktree_id, _)) => {
            let wt_id = usize::from(*worktree_id);
            skill_index
                .and_then(|idx| {
                    idx.project_skills
                        .iter()
                        .find(|g| g.worktree_id.0 == wt_id)
                        .map(|g| g.skills.clone())
                })
                .unwrap_or_default()
        }
        _ => Vec::new(),
    };

    v_flex()
        .id("skills-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .map(|this| {
            if skills.is_empty() {
                let message = match &settings_window.current_file {
                    SettingsUiFile::User => "No global skills installed.",
                    SettingsUiFile::Project(_) => "No project skills found.",
                    _ => "No skills available for this context.",
                };
                let original_window = settings_window.original_window;
                this.items_center().justify_center().child(
                    v_flex()
                        .items_center()
                        .gap_2()
                        .child(Label::new(message).color(Color::Muted))
                        .child(
                            Button::new("open-skill-creator", "Create a Skill")
                                .tab_index(0_isize)
                                .style(ButtonStyle::Outlined)
                                .end_icon(
                                    Icon::new(IconName::ArrowUpRight)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                )
                                .on_click(cx.listener(move |_this, _event, window, cx| {
                                    let Some(original_window) = original_window else {
                                        return;
                                    };
                                    original_window
                                        .update(cx, |_workspace, original_window, cx| {
                                            original_window.dispatch_action(
                                                zed_actions::assistant::OpenSkillCreator
                                                    .boxed_clone(),
                                                cx,
                                            );
                                        })
                                        .log_err();
                                    window.remove_window();
                                })),
                        ),
                )
            } else {
                this.track_scroll(scroll_handle)
                    .overflow_y_scroll()
                    .children(skills.iter().enumerate().flat_map(|(i, skill)| {
                        let mut elements: Vec<AnyElement> = vec![render_skill_row(skill, cx)];
                        if i + 1 < skills.len() {
                            elements.push(Divider::horizontal().into_any_element());
                        }
                        elements
                    }))
            }
        })
        .into_any_element()
}

fn render_skill_row(skill: &Skill, cx: &mut Context<SettingsWindow>) -> AnyElement {
    let skill_file_path = skill.skill_file_path.clone();
    let directory_path = skill.directory_path.clone();

    h_flex()
        .w_full()
        .justify_between()
        .py_2p5()
        .gap_4()
        .child(
            v_flex()
                .gap_0p5()
                .min_w_0()
                .flex_1()
                .child(Label::new(skill.name.clone()))
                .child(
                    Label::new(skill.description.clone())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
        .child(
            h_flex()
                .gap_2()
                .child(
                    IconButton::new(
                        SharedString::from(format!("delete-{}", skill.name)),
                        IconName::Trash,
                    )
                    .tab_index(0_isize)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Delete Skill"))
                    .on_click(cx.listener(
                        move |_this, _event, _window, cx| {
                            let directory_path = directory_path.clone();
                            let app_state = workspace::AppState::global(cx);
                            let fs = app_state.fs.clone();
                            cx.spawn(async move |_this, _cx| {
                                fs.remove_dir(
                                    &directory_path,
                                    RemoveOptions {
                                        recursive: true,
                                        ignore_if_not_exists: true,
                                    },
                                )
                                .await
                                .log_err();
                            })
                            .detach();
                        },
                    )),
                )
                .child(
                    Button::new(SharedString::from(format!("open-{}", skill.name)), "Open")
                        .tab_index(0_isize)
                        .style(ButtonStyle::OutlinedGhost)
                        .size(ButtonSize::Medium)
                        .end_icon(
                            Icon::new(IconName::ArrowUpRight)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                        .on_click(cx.listener(move |settings_window, _event, window, cx| {
                            let skill_file_path = skill_file_path.clone();
                            let Some(original_window) = settings_window.original_window else {
                                return;
                            };
                            original_window
                                .update(cx, |multi_workspace, original_window, cx| {
                                    let workspace = multi_workspace.workspace().clone();
                                    workspace.update(cx, |workspace, cx| {
                                        workspace
                                            .open_abs_path(
                                                skill_file_path,
                                                Default::default(),
                                                original_window,
                                                cx,
                                            )
                                            .detach_and_log_err(cx);
                                    });
                                })
                                .log_err();
                            window.remove_window();
                        })),
                ),
        )
        .into_any_element()
}
