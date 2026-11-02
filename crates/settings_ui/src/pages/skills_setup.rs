use agent_skills::{Skill, SkillIndex, SkillSource, encode_skill_share_link};
use anyhow::Context as _;
use fs::RemoveOptions;
use gpui::{
    App, ClipboardItem, Entity, EntityId, PromptLevel, ScrollHandle, SharedString, Task, prelude::*,
};
use project::{Project, Worktree, WorktreeId};
use std::path::PathBuf;

use ui::{Divider, Tooltip, prelude::*};
use util::ResultExt as _;
use util::paths::PathExt as _;

use crate::pages::SkillCreatorOpenMode;
use crate::{SettingsUiFile, SettingsWindow};

/// The project of the window the settings were opened from, if its global
/// skills come from a remote machine.
pub(crate) fn remote_global_skills_project(
    settings_window: &SettingsWindow,
    cx: &App,
) -> Option<Entity<Project>> {
    settings_window
        .active_project(cx)
        .filter(|project| agent::global_skills_are_remote(project.read(cx)))
}

/// A skill hidden after deletion until the skill index stops listing it.
/// Different machines can have skills at the same path, so this also
/// records the machine.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DeletedSkill {
    /// The remote project whose global skills it was in, if any.
    pub(crate) remote_project_id: Option<EntityId>,
    pub(crate) directory_path: PathBuf,
}

impl DeletedSkill {
    pub(crate) fn new(skill: &Skill, remote_project_id: Option<EntityId>) -> Self {
        Self {
            remote_project_id: remote_project_id
                .filter(|_| matches!(skill.source, SkillSource::Global)),
            directory_path: skill.directory_path.clone(),
        }
    }
}

/// Which machine, and through what, a skill's files are read, written and
/// deleted.
pub(crate) enum SkillLocation {
    /// The machine Zed runs on.
    Local,
    /// The remote machine's `~/.agents/skills`.
    RemoteGlobal(Entity<Project>),
    /// A project worktree, which may be remote.
    Project {
        project: Entity<Project>,
        worktree: Entity<Worktree>,
    },
}

impl SkillLocation {
    /// Where global skills live, given [`remote_global_skills_project`].
    pub(crate) fn global(remote_project: Option<Entity<Project>>) -> Self {
        remote_project.map_or(SkillLocation::Local, SkillLocation::RemoteGlobal)
    }
}

fn skill_location(
    skill: &Skill,
    remote_project: Option<Entity<Project>>,
    settings_window: &SettingsWindow,
    cx: &App,
) -> SkillLocation {
    let worktree_id = match &skill.source {
        SkillSource::BuiltIn => return SkillLocation::Local,
        SkillSource::Global => return SkillLocation::global(remote_project),
        SkillSource::ProjectLocal { worktree_id, .. } => worktree_id,
    };
    let worktree_id = WorktreeId::from_usize(worktree_id.0);
    // Collab projects use the host's worktree ids, which can repeat, so also check the path.
    // The file path, unlike `directory_path`, is in the worktree's path style.
    crate::all_projects(settings_window.original_window.as_ref(), cx)
        .find_map(|project| {
            let worktree = project.read(cx).worktree_for_id(worktree_id, cx)?;
            let worktree_ref = worktree.read(cx);
            worktree_ref
                .path_style()
                .starts_with(&skill.skill_file_path, worktree_ref.abs_path())
                .then_some(SkillLocation::Project { project, worktree })
        })
        .unwrap_or(SkillLocation::Local)
}

pub(crate) fn copy_skill_share_link(
    skill: &Skill,
    settings_window: &SettingsWindow,
    cx: &mut Context<SettingsWindow>,
) -> Task<()> {
    let skill_file_path = skill.skill_file_path.clone();
    let directory_path = skill.directory_path.clone();
    let remote_project = remote_global_skills_project(settings_window, cx);
    let location = skill_location(skill, remote_project, settings_window, cx);
    let fs = workspace::AppState::global(cx).fs.clone();
    cx.spawn(async move |settings_window, cx| {
        let content = match &location {
            SkillLocation::Project { project, .. } | SkillLocation::RemoteGlobal(project) => {
                project
                    .update(cx, |project, cx| {
                        project.open_local_buffer(&skill_file_path, cx)
                    })
                    .await
                    .map(|buffer| buffer.update(cx, |buffer, _cx| buffer.text()))
            }
            SkillLocation::Local => fs.load(&skill_file_path).await,
        };
        match content {
            Ok(content) => {
                let link = encode_skill_share_link(&content);
                settings_window
                    .update(cx, |settings_window, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(link));
                        settings_window.last_copied_skill_directory_path = Some(directory_path);
                        cx.notify();
                    })
                    .ok();
            }
            Err(error) => {
                log::error!(
                    "failed to read skill file {} for sharing: {error:#}",
                    skill_file_path.display()
                );
            }
        }
    })
}

pub(crate) fn trash_skill(
    skill: &Skill,
    settings_window: &mut SettingsWindow,
    cx: &mut Context<SettingsWindow>,
) -> Task<()> {
    let directory_path = skill.directory_path.clone();
    let remote_project = remote_global_skills_project(settings_window, cx);
    let deleted_skill = DeletedSkill::new(
        skill,
        remote_project.as_ref().map(|project| project.entity_id()),
    );
    if !settings_window
        .hidden_deleted_skills
        .insert(deleted_skill.clone())
    {
        return Task::ready(());
    }
    cx.notify();

    let location = skill_location(skill, remote_project, settings_window, cx);
    let fs = workspace::AppState::global(cx).fs.clone();
    cx.spawn(async move |settings_window, cx| {
        let trash_result = match &location {
            SkillLocation::Project { project, .. } => {
                let trash_task = project.update(cx, |project, cx| {
                    let project_path = project
                        .project_path_for_absolute_path(&directory_path, cx)
                        .context("skill directory is outside the project's worktrees")?;
                    let entry_id = project
                        .entry_for_path(&project_path, cx)
                        .map(|entry| entry.id);
                    // Already deleted outside Zed.
                    anyhow::Ok(entry_id.and_then(|entry_id| project.trash_entry(entry_id, cx)))
                });
                match trash_task {
                    Ok(Some(trash_task)) => trash_task.await.map(|_| ()),
                    Ok(None) => Ok(()),
                    Err(error) => Err(error),
                }
            }
            SkillLocation::RemoteGlobal(project) => {
                cx.update(|cx| agent::trash_remote_global_skill(project, &directory_path, cx))
                    .await
            }
            SkillLocation::Local => fs
                .trash(
                    &directory_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .map(|_| ()),
        };
        if let Err(error) = trash_result {
            log::error!(
                "failed to move skill directory {} to trash: {error:#}",
                directory_path.display()
            );
            settings_window
                .update(cx, |settings_window, cx| {
                    settings_window.hidden_deleted_skills.remove(&deleted_skill);
                    cx.notify();
                })
                .ok();
        }
    })
}

/// Skills shown on the Skills page for the currently selected settings file:
/// - User file → global skills only
/// - Project file → project-local skills for that worktree only
pub(crate) fn displayed_skills(
    settings_window: &SettingsWindow,
    remote_project: Option<&Entity<Project>>,
    cx: &App,
) -> Vec<Skill> {
    let skill_index = cx.try_global::<SkillIndex>();

    match &settings_window.current_file {
        SettingsUiFile::User => match remote_project {
            Some(project) => skill_index
                .and_then(|index| index.remote_global_skills.get(&project.entity_id()))
                .cloned()
                .unwrap_or_default(),
            None => skill_index
                .map(|index| index.global_skills.clone())
                .unwrap_or_default(),
        },
        SettingsUiFile::Project((worktree_id, _)) => {
            let worktree_id = usize::from(*worktree_id);
            skill_index
                .and_then(|index| {
                    index
                        .project_skills
                        .iter()
                        .find(|group| group.worktree_id.0 == worktree_id)
                        .map(|group| group.skills.clone())
                })
                .unwrap_or_default()
        }
        _ => Vec::new(),
    }
    .into_iter()
    .filter(|skill| {
        let remote_project_id = remote_project.map(|project| project.entity_id());
        !settings_window
            .hidden_deleted_skills
            .contains(&DeletedSkill::new(skill, remote_project_id))
    })
    .collect()
}

pub(crate) fn render_skills_setup_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let remote_project = remote_global_skills_project(settings_window, cx);
    let skills: Vec<Skill> = displayed_skills(settings_window, remote_project.as_ref(), cx);

    v_flex()
        .id("skills-page")
        .size_full()
        .pt_2()
        .pb_16()
        .map(|this| {
            if skills.is_empty() {
                let message = match &settings_window.current_file {
                    SettingsUiFile::User => "No global skills installed.",
                    SettingsUiFile::Project(_) => "No project skills found.",
                    _ => "No skills available for this context.",
                };

                this.px_8().items_center().justify_center().child(
                    v_flex()
                        .items_center()
                        .gap_2()
                        .child(Label::new(message).color(Color::Muted))
                        .child(
                            Button::new("open-skill-creator-empty", "Create a Skill")
                                .tab_index(0_isize)
                                .style(ButtonStyle::Outlined)
                                .start_icon(
                                    Icon::new(IconName::Plus)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                )
                                .on_click(cx.listener(move |this, _event, window, cx| {
                                    this.open_skill_creator_sub_page(
                                        SkillCreatorOpenMode::Form,
                                        window,
                                        cx,
                                    );
                                })),
                        ),
                )
            } else {
                this.track_scroll(scroll_handle)
                    .overflow_y_scroll()
                    .children(skills.iter().enumerate().flat_map(|(i, skill)| {
                        let mut elements: Vec<AnyElement> = vec![render_skill_row(
                            skill,
                            remote_project.as_ref().map(|project| project.entity_id()),
                            settings_window,
                            cx,
                        )];

                        if i + 1 < skills.len() {
                            elements.push(
                                div()
                                    .px_8()
                                    .child(Divider::horizontal().flex_grow_1())
                                    .into_any_element(),
                            );
                        }

                        elements
                    }))
            }
        })
        .into_any_element()
}

fn render_skill_row(
    skill: &Skill,
    remote_project_id: Option<EntityId>,
    settings_window: &SettingsWindow,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let deleted_skill = DeletedSkill::new(skill, remote_project_id);
    let skill_file_path = skill.skill_file_path.clone();
    let directory_path = skill.directory_path.clone();
    let skill_name = skill.name.clone();

    let (skill_scope, shared_scope) = match &skill.source {
        SkillSource::ProjectLocal { .. } => ("project", "used in this project"),
        _ if remote_project_id.is_some() => ("global", "on the remote machine"),
        _ => ("global", "on this machine"),
    };

    let delete_skill = skill.clone();

    let share_copied = settings_window.last_copied_skill_directory_path.as_deref()
        == Some(skill.directory_path.as_path());
    let warning_message = skill.load_warnings.first().map(|warning| warning.message());

    let (share_icon, share_icon_color) = if share_copied {
        (IconName::Check, Color::Success)
    } else {
        (IconName::Link, Color::Muted)
    };

    let group = format!("group-{}", skill.name);

    let title = h_flex()
        .ml(rems_from_px(-22.0_f32))
        .gap_1()
        .child({
            let share_skill = skill.clone();
            IconButton::new(
                SharedString::from(format!("share-{}", skill.name)),
                share_icon,
            )
            .tab_index(0_isize)
            .shape(ui::IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .icon_color(share_icon_color)
            .tooltip(Tooltip::text("Copy Share Link"))
            .visible_on_hover(&group)
            .on_click(cx.listener(move |settings_window, _event, _window, cx| {
                copy_skill_share_link(&share_skill, settings_window, cx).detach();
            }))
        })
        .child(Label::new(skill.name.clone()))
        .when_some(warning_message, |this, warning_message| {
            this.child(
                h_flex()
                    .id(SharedString::from(format!("warning-{}", skill.name)))
                    .child(
                        Icon::new(IconName::Warning)
                            .size(IconSize::XSmall)
                            .color(Color::Warning),
                    )
                    .tooltip(Tooltip::text(warning_message)),
            )
        });

    h_flex()
        .group(group)
        .w_full()
        .justify_between()
        .py_3()
        .px_8()
        .gap_4()
        .child(
            v_flex().gap_0p5().min_w_0().flex_1().child(title).child(
                Label::new(skill.description.clone())
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .line_clamp(5),
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
                        move |settings_window, _event, window, cx| {
                            let directory_path = directory_path.clone();
                            if settings_window
                                .hidden_deleted_skills
                                .contains(&deleted_skill)
                            {
                                return;
                            }

                            let prompt_message =
                                format!("Delete the {skill_scope} skill \"{skill_name}\"?");
                            let prompt_detail = format!(
                                "This will move {} to the trash. This skill is shared with other \
                                 agent tools {shared_scope}, so it will no longer be available to \
                                 them either.",
                                directory_path.compact().display(),
                            );
                            let answer = window.prompt(
                                PromptLevel::Info,
                                &prompt_message,
                                Some(&prompt_detail),
                                &["Delete", "Cancel"],
                                cx,
                            );

                            let delete_skill = delete_skill.clone();
                            cx.spawn(async move |settings_window, cx| {
                                if answer.await != Ok(0) {
                                    return;
                                }
                                let Ok(trash_task) =
                                    settings_window.update(cx, |settings_window, cx| {
                                        trash_skill(&delete_skill, settings_window, cx)
                                    })
                                else {
                                    return;
                                };
                                trash_task.await;
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
