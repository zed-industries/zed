use std::{collections::BTreeMap, collections::BTreeSet, ops::Range, path::PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ActivityKind {
    Execute,
    Read,
    Search,
    Edit,
    Delete,
    Move,
    Fetch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ActivityStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Canceled,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ToolActivity {
    pub id: String,
    pub kind: ActivityKind,
    pub status: ActivityStatus,
    pub paths: Vec<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ToolCallGroup {
    pub first_id: String,
    pub entries: Range<usize>,
    pub kind: ActivityKind,
    calls: BTreeMap<String, ToolActivity>,
    call_order: Vec<String>,
}

impl ToolCallGroup {
    pub fn has_failure(&self) -> bool {
        self.calls
            .values()
            .any(|call| call.status == ActivityStatus::Failed)
    }

    pub fn is_running(&self) -> bool {
        self.calls
            .values()
            .any(|call| call.status == ActivityStatus::Running)
    }

    pub fn running_command(&self) -> Option<(&str, usize)> {
        if self.kind != ActivityKind::Execute {
            return None;
        }
        self.call_order
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, id)| {
                self.calls
                    .get(id)
                    .filter(|call| call.status == ActivityStatus::Running)
                    .map(|_| (id.as_str(), index + 1))
            })
    }

    pub fn label(&self) -> String {
        let count = self.calls.len();
        let all_completed = self
            .calls
            .values()
            .all(|call| call.status == ActivityStatus::Completed);
        let running = self.is_running();
        let (verb, noun, count) = match self.kind {
            ActivityKind::Execute => (
                if all_completed {
                    "Ran"
                } else if running {
                    "Running"
                } else {
                    ""
                },
                "command",
                count,
            ),
            ActivityKind::Read => {
                let known_paths = self.calls.values().all(|call| !call.paths.is_empty());
                let file_count = self
                    .calls
                    .values()
                    .flat_map(|call| &call.paths)
                    .collect::<BTreeSet<_>>()
                    .len();
                (
                    if all_completed {
                        if known_paths { "Read" } else { "Completed" }
                    } else if running {
                        if known_paths { "Reading" } else { "Running" }
                    } else {
                        ""
                    },
                    if known_paths {
                        "file"
                    } else {
                        "read operation"
                    },
                    if known_paths { file_count } else { count },
                )
            }
            ActivityKind::Search => (
                if all_completed {
                    "Completed"
                } else if running {
                    "Running"
                } else {
                    ""
                },
                "search",
                count,
            ),
            ActivityKind::Edit => (
                if all_completed {
                    "Completed"
                } else if running {
                    "Applying"
                } else {
                    ""
                },
                "edit operation",
                count,
            ),
            ActivityKind::Delete => (
                if all_completed {
                    "Completed"
                } else if running {
                    "Running"
                } else {
                    ""
                },
                "delete operation",
                count,
            ),
            ActivityKind::Move => (
                if all_completed {
                    "Completed"
                } else if running {
                    "Running"
                } else {
                    ""
                },
                "move operation",
                count,
            ),
            ActivityKind::Fetch => (
                if all_completed {
                    "Completed"
                } else if running {
                    "Running"
                } else {
                    ""
                },
                "fetch",
                count,
            ),
        };
        let plural = if count == 1 {
            ""
        } else if matches!(self.kind, ActivityKind::Search | ActivityKind::Fetch) {
            "es"
        } else {
            "s"
        };
        let mut label = format!("{verb} {count} {noun}{plural}")
            .trim_start()
            .to_string();
        if let Some((_, ordinal)) = self.running_command() {
            label = format!("Running Command {ordinal}");
            let running_count = self
                .calls
                .values()
                .filter(|call| call.status == ActivityStatus::Running)
                .count();
            if running_count > 1 {
                label.push_str(&format!(" (+{} active)", running_count - 1));
            }
        }
        let statuses = [
            (ActivityStatus::Pending, "pending"),
            (ActivityStatus::Failed, "failed"),
            (ActivityStatus::Canceled, "canceled"),
            (ActivityStatus::Rejected, "rejected"),
        ]
        .into_iter()
        .filter_map(|(status, name)| {
            let count = self
                .calls
                .values()
                .filter(|call| call.status == status)
                .count();
            (count > 0).then(|| format!("{count} {name}"))
        })
        .collect::<Vec<_>>();
        if !statuses.is_empty() {
            label.push_str(&format!(" ({})", statuses.join(", ")));
        }
        label
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub(super) struct ToolCallGroups {
    pub groups: Vec<ToolCallGroup>,
    by_entry: Vec<Option<usize>>,
}

impl ToolCallGroups {
    pub fn new(entries: impl IntoIterator<Item = Option<ToolActivity>>) -> Self {
        let mut result = Self::default();
        let mut previous_kind = None;
        for (index, activity) in entries.into_iter().enumerate() {
            let Some(activity) = activity else {
                previous_kind = None;
                result.by_entry.push(None);
                continue;
            };
            if previous_kind != Some(activity.kind) {
                result.groups.push(ToolCallGroup {
                    first_id: activity.id.clone(),
                    entries: index..index + 1,
                    kind: activity.kind,
                    calls: BTreeMap::new(),
                    call_order: Vec::new(),
                });
            }
            previous_kind = Some(activity.kind);
            if let Some(group) = result.groups.last_mut() {
                group.entries.end = index + 1;
                // ACP updates replace a call's state; they are not new executions.
                if !group.calls.contains_key(&activity.id) {
                    group.call_order.push(activity.id.clone());
                }
                group.calls.insert(activity.id.clone(), activity);
            }
            result.by_entry.push(result.groups.len().checked_sub(1));
        }
        result
    }

    pub fn for_entry(&self, index: usize) -> Option<&ToolCallGroup> {
        self.groups.get((*self.by_entry.get(index)?)?)
    }
}

pub(super) fn compact_preview(source: &str) -> String {
    let line = source
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with("```"))
        .unwrap_or("Tool call");
    let mut characters = line.chars();
    let mut preview: String = characters.by_ref().take(100).collect();
    if characters.next().is_some()
        || source
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.trim().starts_with("```"))
            .take(2)
            .count()
            > 1
    {
        preview.push('…');
    }
    preview
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call(id: &str, kind: ActivityKind) -> Option<ToolActivity> {
        Some(ToolActivity {
            id: id.into(),
            kind,
            status: ActivityStatus::Completed,
            paths: vec![],
        })
    }

    #[test]
    fn commands_are_counted_by_id_not_updates_or_shell_syntax() {
        let mut updated = call("bash-1", ActivityKind::Execute).unwrap();
        updated.status = ActivityStatus::Running;
        let groups = ToolCallGroups::new([
            call("bash-1", ActivityKind::Execute),
            Some(updated),
            call("bash-2", ActivityKind::Execute),
            call("bash-3", ActivityKind::Execute),
        ]);
        assert_eq!(groups.groups.len(), 1);
        assert_eq!(groups.for_entry(3).unwrap().label(), "Running Command 1");
        assert_eq!(groups.for_entry(0).unwrap().first_id, "bash-1");
    }

    #[test]
    fn running_command_ordinal_tracks_chronology_not_ids_or_queued_calls() {
        let mut running = call("a", ActivityKind::Execute).unwrap();
        running.status = ActivityStatus::Running;
        let mut pending = call("queued", ActivityKind::Execute).unwrap();
        pending.status = ActivityStatus::Pending;
        let groups = ToolCallGroups::new([
            call("z", ActivityKind::Execute),
            Some(running.clone()),
            Some(running),
            Some(pending),
        ]);
        let group = &groups.groups[0];
        assert_eq!(group.running_command(), Some(("a", 2)));
        assert_eq!(group.label(), "Running Command 2 (1 pending)");
    }

    #[test]
    fn concurrent_commands_keep_individual_timers_and_report_other_active_calls() {
        let mut first = call("z", ActivityKind::Execute).unwrap();
        first.status = ActivityStatus::Running;
        let mut second = call("a", ActivityKind::Execute).unwrap();
        second.status = ActivityStatus::Running;
        let groups = ToolCallGroups::new([Some(first), Some(second)]);
        assert_eq!(groups.groups[0].running_command(), Some(("a", 2)));
        assert_eq!(groups.groups[0].label(), "Running Command 2 (+1 active)");
    }

    #[test]
    fn mixed_tools_and_message_boundaries_preserve_order() {
        let groups = ToolCallGroups::new([
            call("1", ActivityKind::Execute),
            call("2", ActivityKind::Read),
            call("3", ActivityKind::Execute),
            None,
            call("4", ActivityKind::Execute),
        ]);
        assert_eq!(
            groups
                .groups
                .iter()
                .map(|group| group.entries.clone())
                .collect::<Vec<_>>(),
            [0..1, 1..2, 2..3, 4..5]
        );
        assert!(groups.for_entry(3).is_none());
        assert!(groups.for_entry(100).is_none());
    }

    #[test]
    fn read_counts_use_unique_known_paths_and_fall_back_for_unknown_paths() {
        let mut first = call("1", ActivityKind::Read).unwrap();
        first.paths = vec!["/a".into(), "/b".into()];
        let mut second = call("2", ActivityKind::Read).unwrap();
        second.paths = vec!["/b".into()];
        let groups = ToolCallGroups::new([Some(first.clone()), Some(second.clone())]);
        assert_eq!(groups.groups[0].label(), "Read 2 files");
        second.paths.clear();
        let groups = ToolCallGroups::new([Some(first), Some(second)]);
        assert_eq!(groups.groups[0].label(), "Completed 2 read operations");
    }

    #[test]
    fn failed_canceled_and_pending_commands_are_not_labeled_as_successes() {
        for (status, expected) in [
            (ActivityStatus::Failed, "1 command (1 failed)"),
            (ActivityStatus::Canceled, "1 command (1 canceled)"),
            (ActivityStatus::Rejected, "1 command (1 rejected)"),
            (ActivityStatus::Pending, "1 command (1 pending)"),
        ] {
            let mut activity = call("1", ActivityKind::Execute).unwrap();
            activity.status = status;
            let groups = ToolCallGroups::new([Some(activity)]);
            assert_eq!(groups.groups[0].label(), expected);
        }
    }

    #[test]
    fn stable_group_identity_survives_growth_and_removal() {
        let before = ToolCallGroups::new([None, call("a", ActivityKind::Execute)]);
        let after = ToolCallGroups::new([
            call("a", ActivityKind::Execute),
            call("b", ActivityKind::Execute),
        ]);
        assert_eq!(
            before.for_entry(1).unwrap().first_id,
            after.for_entry(0).unwrap().first_id
        );
        assert_eq!(after.for_entry(1).unwrap().label(), "Ran 2 commands");
        assert_eq!(ToolCallGroups::new([]), ToolCallGroups::default());
    }

    #[test]
    fn non_terminal_kinds_stay_separate() {
        let groups = ToolCallGroups::new([
            call("1", ActivityKind::Search),
            call("2", ActivityKind::Search),
            call("3", ActivityKind::Edit),
            call("4", ActivityKind::Delete),
            call("5", ActivityKind::Move),
            call("6", ActivityKind::Fetch),
        ]);
        assert_eq!(groups.groups.len(), 5);
        assert_eq!(groups.groups[0].label(), "Completed 2 searches");
    }

    #[test]
    fn previews_bound_long_unicode_commands_and_heredocs() {
        assert_eq!(
            compact_preview("```\ncat <<'EOF'\nhello\nEOF\n```"),
            "cat <<'EOF'…"
        );
        assert_eq!(compact_preview(&"é".repeat(150)).chars().count(), 101);
        assert_eq!(compact_preview("```\nprintf ok\n```"), "printf ok");
        assert_eq!(compact_preview("\n```\n```"), "Tool call");
    }
}
