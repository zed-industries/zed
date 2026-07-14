use agent_skills::{Skill, SkillIndex, encode_skill_share_link};
use fs::RemoveOptions;
use gpui::{App, ClipboardItem, ScrollHandle, SharedString, prelude::*};

use ui::{Divider, Tooltip, prelude::*};
use util::ResultExt as _;

use crate::pages::SkillCreatorOpenMode;
use crate::{SettingsUiFile, SettingsWindow};

/// Skills shown on the Skills page for the currently selected settings file:
/// - User file → global skills only
/// - Project file → project-local skills for that worktree only
pub(crate) fn displayed_skills(settings_window: &SettingsWindow, cx: &App) -> Vec<Skill> {
    let skill_index = cx.try_global::<SkillIndex>();

    match &settings_window.current_file {
        SettingsUiFile::User => skill_index
            .map(|idx| idx.global_skills.clone())
            .unwrap_or_default(),
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
        !settings_window
            .hidden_deleted_skill_directory_paths
            .contains(&skill.directory_path)
    })
    .collect()
}

pub(crate) fn render_skills_setup_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let skills: Vec<Skill> = displayed_skills(settings_window, cx);

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
                        let mut elements: Vec<AnyElement> =
                            vec![render_skill_row(skill, settings_window, cx)];

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
    settings_window: &SettingsWindow,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let skill_file_path = skill.skill_file_path.clone();
    let directory_path = skill.directory_path.clone();

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
        .ml(rems_from_px(-22.))
        .gap_1()
        .child({
            let share_skill_file_path = skill.skill_file_path.clone();
            let share_directory_path = skill.directory_path.clone();
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
            .on_click(cx.listener(move |_settings_window, _event, _window, cx| {
                let skill_file_path = share_skill_file_path.clone();
                let directory_path = share_directory_path.clone();
                let app_state = workspace::AppState::global(cx);
                let fs = app_state.fs.clone();
                cx.spawn(
                    async move |settings_window, cx| match fs.load(&skill_file_path).await {
                        Ok(content) => {
                            let link = encode_skill_share_link(&content);
                            settings_window
                                .update(cx, |settings_window, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new_string(link));
                                    settings_window.last_copied_skill_directory_path =
                                        Some(directory_path.clone());
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
                    },
                )
                .detach();
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
                        move |settings_window, _event, _window, cx| {
                            let directory_path = directory_path.clone();
                            if !settings_window
                                .hidden_deleted_skill_directory_paths
                                .insert(directory_path.clone())
                            {
                                return;
                            }
                            cx.notify();

                            let app_state = workspace::AppState::global(cx);
                            let fs = app_state.fs.clone();
                            cx.spawn(async move |settings_window, cx| {
                                let remove_result = fs
                                    .remove_dir(
                                        &directory_path,
                                        RemoveOptions {
                                            recursive: true,
                                            ignore_if_not_exists: true,
                                        },
                                    )
                                    .await;
                                if let Err(error) = remove_result {
                                    log::error!(
                                        "failed to delete skill directory {}: {error:#}",
                                        directory_path.display()
                                    );
                                    settings_window
                                        .update(cx, |settings_window, cx| {
                                            settings_window
                                                .hidden_deleted_skill_directory_paths
                                                .remove(&directory_path);
                                            cx.notify();
                                        })
                                        .ok();
                                }
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
