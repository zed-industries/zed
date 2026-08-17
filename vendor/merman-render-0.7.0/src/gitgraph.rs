use crate::Result;
use crate::config::{
    config_bool as cfg_bool, config_f64 as cfg_f64, config_f64_css_px, config_string as cfg_string,
};
use crate::model::{
    Bounds, GitGraphArrowLayout, GitGraphBranchLayout, GitGraphCommitLayout, GitGraphDiagramLayout,
};
use crate::text::{TextMeasurer, TextStyle};
use merman_core::diagrams::git_graph::{
    GitGraphCommitRenderModel as GitGraphCommit, GitGraphRenderModel,
};
use std::collections::HashMap;

const LAYOUT_OFFSET: f64 = 10.0;
const COMMIT_STEP: f64 = 40.0;
const DEFAULT_POS: f64 = 30.0;
const THEME_COLOR_LIMIT: usize = 8;

const COMMIT_TYPE_MERGE: i64 = 3;

#[derive(Debug, Clone, Copy)]
struct CommitPosition {
    x: f64,
    y: f64,
}

fn find_closest_parent<'a>(
    parents: &'a [String],
    dir: &str,
    commit_pos: &HashMap<&str, CommitPosition>,
) -> Option<&'a str> {
    let mut target: f64 = if dir == "BT" { f64::INFINITY } else { 0.0 };
    let mut closest: Option<&str> = None;
    for parent in parents {
        let Some(pos) = commit_pos.get(parent.as_str()) else {
            continue;
        };
        let parent_position = if dir == "TB" || dir == "BT" {
            pos.y
        } else {
            pos.x
        };
        if dir == "BT" {
            if parent_position <= target {
                closest = Some(parent.as_str());
                target = parent_position;
            }
        } else if parent_position >= target {
            closest = Some(parent.as_str());
            target = parent_position;
        }
    }
    closest
}

fn commit_axis_start_pos(dir: &str) -> f64 {
    if dir == "TB" || dir == "BT" {
        DEFAULT_POS
    } else {
        0.0
    }
}

fn branch_label_bbox_width_px(
    direction: &str,
    text: &str,
    style: &TextStyle,
    measurer: &dyn TextMeasurer,
) -> f64 {
    if direction == "TB" || direction == "BT" {
        // Mermaid measures GitGraph branch labels with `drawText(name).getBBox()` before placing
        // the background rect and, for vertical layouts, before advancing the next branch lane.
        // Chromium's bbox for these unrotated labels behaves like the centered SVG bbox path with
        // 1/64px ties-to-even quantization; including ASCII glyph overhang makes vertical roots
        // systematically too wide.
        let (left, right) = measurer.measure_svg_text_bbox_x(text, style);
        crate::text::round_to_1_64_px_ties_to_even((left + right).max(0.0))
    } else {
        // Horizontal branch labels line up with the text advance rather than ASCII-overhang bbox
        // width; upstream rects match `<text>.getComputedTextLength()`.
        crate::text::round_to_1_64_px(
            measurer
                .measure_svg_text_computed_length_px(text, style)
                .max(0.0),
        )
    }
}

fn should_reroute_arrow(
    commit_a: &GitGraphCommit,
    commit_b: &GitGraphCommit,
    p1: CommitPosition,
    p2: CommitPosition,
    all_commits: &HashMap<&str, &GitGraphCommit>,
    dir: &str,
) -> bool {
    let commit_b_is_furthest = if dir == "TB" || dir == "BT" {
        p1.x < p2.x
    } else {
        p1.y < p2.y
    };
    let branch_to_get_curve = if commit_b_is_furthest {
        commit_b.branch.as_str()
    } else {
        commit_a.branch.as_str()
    };

    all_commits.values().any(|commit_x| {
        commit_x.branch == branch_to_get_curve
            && commit_x.seq > commit_a.seq
            && commit_x.seq < commit_b.seq
    })
}

fn find_lane(y1: f64, y2: f64, lanes: &mut Vec<f64>, depth: usize) -> f64 {
    let candidate = y1 + (y1 - y2).abs() / 2.0;
    if depth > 5 {
        return candidate;
    }

    let ok = lanes.iter().all(|lane| (lane - candidate).abs() >= 10.0);
    if ok {
        lanes.push(candidate);
        return candidate;
    }

    let diff = (y1 - y2).abs();
    find_lane(y1, y2 - diff / 5.0, lanes, depth + 1)
}

fn draw_arrow(
    commit_a: &GitGraphCommit,
    commit_b: &GitGraphCommit,
    all_commits: &HashMap<&str, &GitGraphCommit>,
    commit_pos: &HashMap<&str, CommitPosition>,
    branch_index: &HashMap<&str, usize>,
    lanes: &mut Vec<f64>,
    dir: &str,
) -> Option<GitGraphArrowLayout> {
    let p1 = *commit_pos.get(commit_a.id.as_str())?;
    let p2 = *commit_pos.get(commit_b.id.as_str())?;
    let arrow_needs_rerouting = should_reroute_arrow(commit_a, commit_b, p1, p2, all_commits, dir);

    let mut color_class_num = branch_index
        .get(commit_b.branch.as_str())
        .copied()
        .unwrap_or(0);
    if commit_b.commit_type == COMMIT_TYPE_MERGE
        && commit_a
            .id
            .as_str()
            .ne(commit_b.parents.first().map(|s| s.as_str()).unwrap_or(""))
    {
        color_class_num = branch_index
            .get(commit_a.branch.as_str())
            .copied()
            .unwrap_or(color_class_num);
    }

    let mut line_def: Option<String> = None;
    if arrow_needs_rerouting {
        let arc = "A 10 10, 0, 0, 0,";
        let arc2 = "A 10 10, 0, 0, 1,";
        let radius = 10.0;
        let offset = 10.0;

        let line_y = if p1.y < p2.y {
            find_lane(p1.y, p2.y, lanes, 0)
        } else {
            find_lane(p2.y, p1.y, lanes, 0)
        };
        let line_x = if p1.x < p2.x {
            find_lane(p1.x, p2.x, lanes, 0)
        } else {
            find_lane(p2.x, p1.x, lanes, 0)
        };

        if dir == "TB" {
            if p1.x < p2.x {
                line_def = Some(format!(
                    "M {} {} L {} {} {} {} {} L {} {} {} {} {} L {} {}",
                    p1.x,
                    p1.y,
                    line_x - radius,
                    p1.y,
                    arc2,
                    line_x,
                    p1.y + offset,
                    line_x,
                    p2.y - radius,
                    arc,
                    line_x + offset,
                    p2.y,
                    p2.x,
                    p2.y
                ));
            } else {
                color_class_num = branch_index
                    .get(commit_a.branch.as_str())
                    .copied()
                    .unwrap_or(0);
                line_def = Some(format!(
                    "M {} {} L {} {} {} {} {} L {} {} {} {} {} L {} {}",
                    p1.x,
                    p1.y,
                    line_x + radius,
                    p1.y,
                    arc,
                    line_x,
                    p1.y + offset,
                    line_x,
                    p2.y - radius,
                    arc2,
                    line_x - offset,
                    p2.y,
                    p2.x,
                    p2.y
                ));
            }
        } else if dir == "BT" {
            if p1.x < p2.x {
                line_def = Some(format!(
                    "M {} {} L {} {} {} {} {} L {} {} {} {} {} L {} {}",
                    p1.x,
                    p1.y,
                    line_x - radius,
                    p1.y,
                    arc,
                    line_x,
                    p1.y - offset,
                    line_x,
                    p2.y + radius,
                    arc2,
                    line_x + offset,
                    p2.y,
                    p2.x,
                    p2.y
                ));
            } else {
                color_class_num = branch_index
                    .get(commit_a.branch.as_str())
                    .copied()
                    .unwrap_or(0);
                line_def = Some(format!(
                    "M {} {} L {} {} {} {} {} L {} {} {} {} {} L {} {}",
                    p1.x,
                    p1.y,
                    line_x + radius,
                    p1.y,
                    arc2,
                    line_x,
                    p1.y - offset,
                    line_x,
                    p2.y + radius,
                    arc,
                    line_x - offset,
                    p2.y,
                    p2.x,
                    p2.y
                ));
            }
        } else if p1.y < p2.y {
            line_def = Some(format!(
                "M {} {} L {} {} {} {} {} L {} {} {} {} {} L {} {}",
                p1.x,
                p1.y,
                p1.x,
                line_y - radius,
                arc,
                p1.x + offset,
                line_y,
                p2.x - radius,
                line_y,
                arc2,
                p2.x,
                line_y + offset,
                p2.x,
                p2.y
            ));
        } else {
            color_class_num = branch_index
                .get(commit_a.branch.as_str())
                .copied()
                .unwrap_or(0);
            line_def = Some(format!(
                "M {} {} L {} {} {} {} {} L {} {} {} {} {} L {} {}",
                p1.x,
                p1.y,
                p1.x,
                line_y + radius,
                arc2,
                p1.x + offset,
                line_y,
                p2.x - radius,
                line_y,
                arc,
                p2.x,
                line_y - offset,
                p2.x,
                p2.y
            ));
        }
    } else {
        let arc = "A 20 20, 0, 0, 0,";
        let arc2 = "A 20 20, 0, 0, 1,";
        let radius = 20.0;
        let offset = 20.0;

        if dir == "TB" {
            if p1.x < p2.x {
                if commit_b.commit_type == COMMIT_TYPE_MERGE
                    && commit_a.id.as_str().ne(commit_b
                        .parents
                        .first()
                        .map(|s| s.as_str())
                        .unwrap_or(""))
                {
                    line_def = Some(format!(
                        "M {} {} L {} {} {} {} {} L {} {}",
                        p1.x,
                        p1.y,
                        p1.x,
                        p2.y - radius,
                        arc,
                        p1.x + offset,
                        p2.y,
                        p2.x,
                        p2.y
                    ));
                } else {
                    line_def = Some(format!(
                        "M {} {} L {} {} {} {} {} L {} {}",
                        p1.x,
                        p1.y,
                        p2.x - radius,
                        p1.y,
                        arc2,
                        p2.x,
                        p1.y + offset,
                        p2.x,
                        p2.y
                    ));
                }
            }

            if p1.x > p2.x {
                if commit_b.commit_type == COMMIT_TYPE_MERGE
                    && commit_a.id.as_str().ne(commit_b
                        .parents
                        .first()
                        .map(|s| s.as_str())
                        .unwrap_or(""))
                {
                    line_def = Some(format!(
                        "M {} {} L {} {} {} {} {} L {} {}",
                        p1.x,
                        p1.y,
                        p1.x,
                        p2.y - radius,
                        arc2,
                        p1.x - offset,
                        p2.y,
                        p2.x,
                        p2.y
                    ));
                } else {
                    line_def = Some(format!(
                        "M {} {} L {} {} {} {} {} L {} {}",
                        p1.x,
                        p1.y,
                        p2.x + radius,
                        p1.y,
                        arc,
                        p2.x,
                        p1.y + offset,
                        p2.x,
                        p2.y
                    ));
                }
            }

            if p1.x == p2.x {
                line_def = Some(format!("M {} {} L {} {}", p1.x, p1.y, p2.x, p2.y));
            }
        } else if dir == "BT" {
            if p1.x < p2.x {
                if commit_b.commit_type == COMMIT_TYPE_MERGE
                    && commit_a.id.as_str().ne(commit_b
                        .parents
                        .first()
                        .map(|s| s.as_str())
                        .unwrap_or(""))
                {
                    line_def = Some(format!(
                        "M {} {} L {} {} {} {} {} L {} {}",
                        p1.x,
                        p1.y,
                        p1.x,
                        p2.y + radius,
                        arc2,
                        p1.x + offset,
                        p2.y,
                        p2.x,
                        p2.y
                    ));
                } else {
                    line_def = Some(format!(
                        "M {} {} L {} {} {} {} {} L {} {}",
                        p1.x,
                        p1.y,
                        p2.x - radius,
                        p1.y,
                        arc,
                        p2.x,
                        p1.y - offset,
                        p2.x,
                        p2.y
                    ));
                }
            }

            if p1.x > p2.x {
                if commit_b.commit_type == COMMIT_TYPE_MERGE
                    && commit_a.id.as_str().ne(commit_b
                        .parents
                        .first()
                        .map(|s| s.as_str())
                        .unwrap_or(""))
                {
                    line_def = Some(format!(
                        "M {} {} L {} {} {} {} {} L {} {}",
                        p1.x,
                        p1.y,
                        p1.x,
                        p2.y + radius,
                        arc,
                        p1.x - offset,
                        p2.y,
                        p2.x,
                        p2.y
                    ));
                } else {
                    line_def = Some(format!(
                        "M {} {} L {} {} {} {} {} L {} {}",
                        p1.x,
                        p1.y,
                        p2.x - radius,
                        p1.y,
                        arc,
                        p2.x,
                        p1.y - offset,
                        p2.x,
                        p2.y
                    ));
                }
            }

            if p1.x == p2.x {
                line_def = Some(format!("M {} {} L {} {}", p1.x, p1.y, p2.x, p2.y));
            }
        } else {
            if p1.y < p2.y {
                if commit_b.commit_type == COMMIT_TYPE_MERGE
                    && commit_a.id.as_str().ne(commit_b
                        .parents
                        .first()
                        .map(|s| s.as_str())
                        .unwrap_or(""))
                {
                    line_def = Some(format!(
                        "M {} {} L {} {} {} {} {} L {} {}",
                        p1.x,
                        p1.y,
                        p2.x - radius,
                        p1.y,
                        arc2,
                        p2.x,
                        p1.y + offset,
                        p2.x,
                        p2.y
                    ));
                } else {
                    line_def = Some(format!(
                        "M {} {} L {} {} {} {} {} L {} {}",
                        p1.x,
                        p1.y,
                        p1.x,
                        p2.y - radius,
                        arc,
                        p1.x + offset,
                        p2.y,
                        p2.x,
                        p2.y
                    ));
                }
            }

            if p1.y > p2.y {
                if commit_b.commit_type == COMMIT_TYPE_MERGE
                    && commit_a.id.as_str().ne(commit_b
                        .parents
                        .first()
                        .map(|s| s.as_str())
                        .unwrap_or(""))
                {
                    line_def = Some(format!(
                        "M {} {} L {} {} {} {} {} L {} {}",
                        p1.x,
                        p1.y,
                        p2.x - radius,
                        p1.y,
                        arc,
                        p2.x,
                        p1.y - offset,
                        p2.x,
                        p2.y
                    ));
                } else {
                    line_def = Some(format!(
                        "M {} {} L {} {} {} {} {} L {} {}",
                        p1.x,
                        p1.y,
                        p1.x,
                        p2.y + radius,
                        arc2,
                        p1.x + offset,
                        p2.y,
                        p2.x,
                        p2.y
                    ));
                }
            }

            if p1.y == p2.y {
                line_def = Some(format!("M {} {} L {} {}", p1.x, p1.y, p2.x, p2.y));
            }
        }
    }

    let d = line_def?;
    Some(GitGraphArrowLayout {
        from: commit_a.id.clone(),
        to: commit_b.id.clone(),
        class_index: (color_class_num % THEME_COLOR_LIMIT) as i64,
        d,
    })
}

pub fn layout_gitgraph_diagram(
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    measurer: &dyn TextMeasurer,
) -> Result<GitGraphDiagramLayout> {
    let model: GitGraphRenderModel = crate::json::from_value_ref(semantic)?;
    layout_gitgraph_diagram_typed(&model, effective_config, measurer)
}

pub fn layout_gitgraph_diagram_typed(
    model: &GitGraphRenderModel,
    effective_config: &serde_json::Value,
    measurer: &dyn TextMeasurer,
) -> Result<GitGraphDiagramLayout> {
    let _ = model.diagram_type.as_str();

    let direction = if model.direction.trim().is_empty() {
        "LR".to_string()
    } else {
        model.direction.trim().to_string()
    };

    let rotate_commit_label =
        cfg_bool(effective_config, &["gitGraph", "rotateCommitLabel"]).unwrap_or(true);
    let show_commit_label =
        cfg_bool(effective_config, &["gitGraph", "showCommitLabel"]).unwrap_or(true);
    let show_branches = cfg_bool(effective_config, &["gitGraph", "showBranches"]).unwrap_or(true);
    let diagram_padding = cfg_f64(effective_config, &["gitGraph", "diagramPadding"])
        .unwrap_or(8.0)
        .max(0.0);
    let parallel_commits =
        cfg_bool(effective_config, &["gitGraph", "parallelCommits"]).unwrap_or(false);

    // Upstream gitGraph uses SVG `getBBox()` probes for branch label widths while the
    // `drawText(...)` nodes inherit Mermaid's global font config.
    let font_family = cfg_string(effective_config, &["fontFamily"])
        .or_else(|| cfg_string(effective_config, &["themeVariables", "fontFamily"]))
        .map(|s| s.trim().trim_end_matches(';').trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "\"trebuchet ms\", verdana, arial, sans-serif".to_string());
    let font_size = config_f64_css_px(effective_config, &["themeVariables", "fontSize"])
        .unwrap_or(16.0)
        .max(1.0);

    let label_style = TextStyle {
        font_family: Some(font_family),
        font_size,
        font_weight: None,
    };

    let mut branches: Vec<GitGraphBranchLayout> = Vec::new();
    let mut branch_pos: HashMap<&str, f64> = HashMap::new();
    let mut branch_index: HashMap<&str, usize> = HashMap::new();
    let mut pos = 0.0;
    for (i, b) in model.branches.iter().enumerate() {
        let metrics = measurer.measure(&b.name, &label_style);
        let bbox_w = branch_label_bbox_width_px(&direction, &b.name, &label_style, measurer);
        branch_pos.insert(b.name.as_str(), pos);
        branch_index.insert(b.name.as_str(), i);

        branches.push(GitGraphBranchLayout {
            name: b.name.clone(),
            index: i as i64,
            pos,
            bbox_width: bbox_w.max(0.0),
            bbox_height: metrics.height.max(0.0),
        });

        pos += 50.0
            + if rotate_commit_label { 40.0 } else { 0.0 }
            + if direction == "TB" || direction == "BT" {
                bbox_w.max(0.0) / 2.0
            } else {
                0.0
            };
    }

    let commits_by_id: HashMap<&str, &GitGraphCommit> =
        model.commits.iter().map(|c| (c.id.as_str(), c)).collect();

    let mut commit_order: Vec<&GitGraphCommit> = model.commits.iter().collect();
    commit_order.sort_by_key(|c| c.seq);

    let mut sorted_keys: Vec<&str> = commit_order.iter().map(|c| c.id.as_str()).collect();
    let mirror_parallel_bt_axis = direction == "BT" && parallel_commits;
    if direction == "BT" && !mirror_parallel_bt_axis {
        sorted_keys.reverse();
    }

    let mut commit_pos: HashMap<&str, CommitPosition> = HashMap::new();
    let mut commits: Vec<GitGraphCommitLayout> = Vec::new();
    let mut max_pos: f64 = 0.0;
    let mut cur_pos = commit_axis_start_pos(&direction);

    for &id in &sorted_keys {
        let Some(commit) = commits_by_id.get(id).copied() else {
            continue;
        };

        if parallel_commits {
            if !commit.parents.is_empty() {
                if let Some(closest_parent) =
                    find_closest_parent(&commit.parents, &direction, &commit_pos)
                {
                    if let Some(parent_position) = commit_pos.get(closest_parent) {
                        if mirror_parallel_bt_axis {
                            cur_pos = parent_position.y + COMMIT_STEP + LAYOUT_OFFSET;
                        } else if direction == "TB" {
                            cur_pos = parent_position.y + COMMIT_STEP;
                        } else if direction == "BT" {
                            let current_position = commit_pos
                                .get(commit.id.as_str())
                                .copied()
                                .unwrap_or(CommitPosition { x: 0.0, y: 0.0 });
                            cur_pos = current_position.y - COMMIT_STEP;
                        } else {
                            cur_pos = parent_position.x + COMMIT_STEP;
                        }
                    }
                }
            } else {
                cur_pos = commit_axis_start_pos(&direction);
            }
        }

        let pos_with_offset = if direction == "BT" && parallel_commits {
            cur_pos
        } else {
            cur_pos + LAYOUT_OFFSET
        };
        let Some(branch_lane) = branch_pos.get(commit.branch.as_str()).copied() else {
            return Err(crate::Error::InvalidModel {
                message: format!("unknown branch for commit {}: {}", commit.id, commit.branch),
            });
        };

        let (x, y) = if direction == "TB" || direction == "BT" {
            (branch_lane, pos_with_offset)
        } else {
            (pos_with_offset, branch_lane)
        };
        commit_pos.insert(commit.id.as_str(), CommitPosition { x, y });

        commits.push(GitGraphCommitLayout {
            id: commit.id.clone(),
            message: commit.message.clone(),
            seq: commit.seq,
            commit_type: commit.commit_type,
            custom_type: commit.custom_type,
            custom_id: commit.custom_id,
            tags: commit.tags.clone(),
            parents: commit.parents.clone(),
            branch: commit.branch.clone(),
            pos: cur_pos,
            pos_with_offset,
            x,
            y,
        });

        cur_pos += COMMIT_STEP + LAYOUT_OFFSET;
        max_pos = max_pos.max(cur_pos);
    }

    if mirror_parallel_bt_axis && !commits.is_empty() {
        // Mermaid lays out `parallelCommits` in sequence order, then mirrors the commit axis for
        // bottom-to-top rendering. Doing the mirror after parent placement keeps branch timelines
        // compact instead of treating the reversed parse order as a new linear timeline.
        let mirror_axis = max_pos - DEFAULT_POS;
        max_pos -= 2.0 * LAYOUT_OFFSET;

        for commit in &mut commits {
            let y = mirror_axis - commit.y;
            commit.pos = y;
            commit.pos_with_offset = y;
            commit.y = y;
        }

        for position in commit_pos.values_mut() {
            position.y = mirror_axis - position.y;
        }
    }

    let mut lanes: Vec<f64> = if show_branches {
        branches.iter().map(|b| b.pos).collect()
    } else {
        Vec::new()
    };

    let mut arrows: Vec<GitGraphArrowLayout> = Vec::new();
    // Mermaid draws arrows by iterating insertion order of the commits map. The DB inserts commits
    // in sequence order, so iterate by `seq` regardless of direction.
    for commit_b in commit_order {
        for parent in &commit_b.parents {
            let Some(commit_a) = commits_by_id.get(parent.as_str()).copied() else {
                continue;
            };
            if let Some(a) = draw_arrow(
                commit_a,
                commit_b,
                &commits_by_id,
                &commit_pos,
                &branch_index,
                &mut lanes,
                &direction,
            ) {
                arrows.push(a);
            }
        }
    }

    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for b in &branches {
        if direction == "TB" || direction == "BT" {
            min_x = min_x.min(b.pos);
            max_x = max_x.max(b.pos);
            min_y = min_y.min(DEFAULT_POS.min(max_pos));
            max_y = max_y.max(DEFAULT_POS.max(max_pos));
        } else {
            min_y = min_y.min(b.pos);
            max_y = max_y.max(b.pos);
            min_x = min_x.min(0.0);
            max_x = max_x.max(max_pos);
            let label_left =
                -b.bbox_width - 4.0 - if rotate_commit_label { 30.0 } else { 0.0 } - 19.0;
            min_x = min_x.min(label_left);
        }
    }

    for c in &commits {
        let r = if c.custom_type.unwrap_or(c.commit_type) == COMMIT_TYPE_MERGE {
            9.0
        } else {
            10.0
        };
        min_x = min_x.min(c.x - r);
        min_y = min_y.min(c.y - r);
        max_x = max_x.max(c.x + r);
        max_y = max_y.max(c.y + r);
    }

    let bounds = if min_x.is_finite() && min_y.is_finite() && max_x.is_finite() && max_y.is_finite()
    {
        Some(Bounds {
            min_x: min_x - diagram_padding,
            min_y: min_y - diagram_padding,
            max_x: max_x + diagram_padding,
            max_y: max_y + diagram_padding,
        })
    } else {
        None
    };

    Ok(GitGraphDiagramLayout {
        bounds,
        direction,
        rotate_commit_label,
        show_branches,
        show_commit_label,
        parallel_commits,
        diagram_padding,
        max_pos,
        branches,
        commits,
        arrows,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::VendoredFontMetricsTextMeasurer;
    use merman_core::diagrams::git_graph::{
        GitGraphBranchRenderModel, GitGraphCommitRenderModel, GitGraphRenderModel,
    };
    use serde_json::json;

    fn commit(id: &str, seq: i64, parents: &[&str], branch: &str) -> GitGraphCommitRenderModel {
        GitGraphCommitRenderModel {
            id: id.to_string(),
            message: id.to_string(),
            seq,
            commit_type: 0,
            tags: Vec::new(),
            parents: parents.iter().map(|p| (*p).to_string()).collect(),
            branch: branch.to_string(),
            custom_type: None,
            custom_id: Some(true),
        }
    }

    #[test]
    fn font_size_ignores_top_level_font_size() {
        let cfg = json!({
            "fontSize": 22,
            "themeVariables": {
                "fontFamily": "\"courier new\", courier, monospace;",
            },
        });

        assert_eq!(
            config_f64_css_px(&cfg, &["themeVariables", "fontSize"])
                .unwrap_or(16.0)
                .max(1.0),
            16.0
        );
    }

    #[test]
    fn font_size_honors_theme_variable_font_size() {
        let cfg = json!({
            "fontSize": 10,
            "themeVariables": {
                "fontSize": "24px",
            },
        });

        assert_eq!(
            config_f64_css_px(&cfg, &["themeVariables", "fontSize"])
                .unwrap_or(16.0)
                .max(1.0),
            24.0
        );
    }

    #[test]
    fn vertical_branch_label_widths_use_centered_bbox_ties_to_even() {
        let measurer = VendoredFontMetricsTextMeasurer::default();
        let style = TextStyle {
            font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
            font_size: 16.0,
            font_weight: None,
        };

        assert_eq!(
            branch_label_bbox_width_px("TB", "main", &style, &measurer),
            35.0
        );
        assert_eq!(
            branch_label_bbox_width_px("TB", "branch1", &style, &measurer),
            57.34375
        );
        assert_eq!(
            branch_label_bbox_width_px("TB", "branch4", &style, &measurer),
            57.34375
        );
        assert_eq!(
            branch_label_bbox_width_px("LR", "branch4", &style, &measurer),
            57.359375
        );
    }

    #[test]
    fn parallel_lr_unconnected_branches_restart_commit_axis() {
        let model = GitGraphRenderModel {
            diagram_type: "gitGraph".to_string(),
            branches: ["main", "dev", "v2", "feat"]
                .into_iter()
                .map(|name| GitGraphBranchRenderModel {
                    name: name.to_string(),
                })
                .collect(),
            commits: vec![
                commit("1-abcdefg", 0, &[], "feat"),
                commit("2-abcdefg", 1, &["1-abcdefg"], "feat"),
                commit("3-abcdefg", 2, &[], "main"),
                commit("4-abcdefg", 3, &[], "dev"),
                commit("5-abcdefg", 4, &[], "v2"),
                commit("6-abcdefg", 5, &["3-abcdefg"], "main"),
            ],
            current_branch: "main".to_string(),
            direction: "LR".to_string(),
            acc_title: None,
            acc_descr: None,
            warnings: Vec::new(),
        };
        let cfg = json!({ "gitGraph": { "parallelCommits": true } });
        let measurer = VendoredFontMetricsTextMeasurer::default();
        let layout = layout_gitgraph_diagram_typed(&model, &cfg, &measurer).unwrap();

        let x_by_id = layout
            .commits
            .iter()
            .map(|c| (c.id.as_str(), c.x))
            .collect::<HashMap<_, _>>();

        assert_eq!(x_by_id["1-abcdefg"], 10.0);
        assert_eq!(x_by_id["2-abcdefg"], 60.0);
        assert_eq!(x_by_id["3-abcdefg"], 10.0);
        assert_eq!(x_by_id["4-abcdefg"], 10.0);
        assert_eq!(x_by_id["5-abcdefg"], 10.0);
        assert_eq!(x_by_id["6-abcdefg"], 60.0);
        assert_eq!(layout.max_pos, 100.0);
    }

    #[test]
    fn parallel_bt_commits_use_mirrored_compact_axis() {
        let model = GitGraphRenderModel {
            diagram_type: "gitGraph".to_string(),
            branches: ["main", "develop", "feature"]
                .into_iter()
                .map(|name| GitGraphBranchRenderModel {
                    name: name.to_string(),
                })
                .collect(),
            commits: vec![
                commit("1-abcdefg", 0, &[], "main"),
                commit("2-abcdefg", 1, &["1-abcdefg"], "main"),
                commit("3-abcdefg", 2, &["2-abcdefg"], "develop"),
                commit("4-abcdefg", 3, &["3-abcdefg"], "develop"),
                commit("5-abcdefg", 4, &["2-abcdefg"], "feature"),
                commit("6-abcdefg", 5, &["5-abcdefg"], "feature"),
                commit("7-abcdefg", 6, &["2-abcdefg"], "main"),
                commit("8-abcdefg", 7, &["7-abcdefg"], "main"),
            ],
            current_branch: "main".to_string(),
            direction: "BT".to_string(),
            acc_title: None,
            acc_descr: None,
            warnings: Vec::new(),
        };
        let cfg = json!({ "gitGraph": { "parallelCommits": true } });
        let measurer = VendoredFontMetricsTextMeasurer::default();
        let layout = layout_gitgraph_diagram_typed(&model, &cfg, &measurer).unwrap();

        let y_by_id = layout
            .commits
            .iter()
            .map(|c| (c.id.as_str(), c.y))
            .collect::<HashMap<_, _>>();

        assert_eq!(y_by_id["1-abcdefg"], 170.0);
        assert_eq!(y_by_id["2-abcdefg"], 120.0);
        assert_eq!(y_by_id["3-abcdefg"], 70.0);
        assert_eq!(y_by_id["4-abcdefg"], 20.0);
        assert_eq!(y_by_id["5-abcdefg"], 70.0);
        assert_eq!(y_by_id["6-abcdefg"], 20.0);
        assert_eq!(y_by_id["7-abcdefg"], 70.0);
        assert_eq!(y_by_id["8-abcdefg"], 20.0);
        assert_eq!(layout.max_pos, 210.0);
    }
}
