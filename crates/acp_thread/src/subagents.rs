use agent_client_protocol::{schema::MaybeUndefined, v1::schema as acp_v1, v2::schema as acp_v2};
use anyhow::Result;
use gpui::SharedString;
use std::path::PathBuf;
use util::path_list::PathList;

/// What a parent ACP session has reported about a subagent session it owns.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SubagentAssociation {
    /// The parent's label for the subagent, which can differ from the subagent's own title.
    pub title: Option<SharedString>,
    pub description: Option<SharedString>,
    /// Whether Zed may cancel the subagent's current work.
    pub can_cancel: bool,
    /// The subagent's working directory, if the parent reported one. Otherwise the
    /// subagent inherits its parent's.
    pub cwd: Option<PathBuf>,
    /// The subagent's additional directories, if the parent reported them. Otherwise the
    /// subagent inherits its parent's.
    pub additional_directories: Option<Vec<PathBuf>>,
}

impl SubagentAssociation {
    /// Omitted fields stay unchanged and `null` clears them. `state` is not stored here:
    /// it is the subagent's activity, which lives on the subagent's own thread.
    pub fn apply(&mut self, update: &acp_v2::SubagentUpdate) {
        if let Some(title) = update.title.as_opt_ref() {
            self.title = title.map(|title| title.clone().into());
        }
        if let Some(description) = update.description.as_opt_ref() {
            self.description = description.map(|description| description.clone().into());
        }
        if let Some(capabilities) = update.capabilities.as_opt_ref() {
            self.can_cancel =
                capabilities.is_some_and(|capabilities| capabilities.cancel.is_some());
        }
        if let Some(cwd) = update.cwd.as_opt_ref() {
            self.cwd = cwd.map(|cwd| cwd.0.clone());
        }
        if let Some(directories) = update.additional_directories.as_opt_ref() {
            self.additional_directories = directories.map(|directories| {
                directories
                    .iter()
                    .map(|directory| directory.0.clone())
                    .collect()
            });
        }
    }

    /// The subagent's working directories, with the cwd first. Each field the parent
    /// hasn't reported is inherited from the parent's own working directories.
    pub fn effective_work_dirs(&self, parent_work_dirs: Option<&PathList>) -> Option<PathList> {
        let mut parent_paths = parent_work_dirs
            .into_iter()
            .flat_map(|work_dirs| work_dirs.ordered_paths());
        let parent_cwd = parent_paths.next();
        let cwd = self.cwd.as_ref().or(parent_cwd)?;
        let additional_directories = match &self.additional_directories {
            Some(directories) => directories.iter().collect::<Vec<_>>(),
            None => parent_paths.collect(),
        };
        let mut paths = vec![cwd];
        for directory in additional_directories {
            if !paths.contains(&directory) {
                paths.push(directory);
            }
        }
        Some(PathList::new(&paths))
    }
}

/// Association metadata is not carried over: Zed does not use it, and v1 replaces it
/// where v2 patches it by key.
pub fn update_from_v1(update: acp_v1::SubagentUpdate) -> Result<acp_v2::SubagentUpdate> {
    let state = match update.state {
        MaybeUndefined::Undefined => MaybeUndefined::Undefined,
        MaybeUndefined::Null => MaybeUndefined::Null,
        MaybeUndefined::Value(state) => MaybeUndefined::Value(state_from_v1(state)?),
    };
    Ok(acp_v2::SubagentUpdate::new(update.session_id.0)
        .title(update.title)
        .description(update.description)
        .capabilities(update.capabilities.map_value(|capabilities| {
            acp_v2::SubagentSessionCapabilities::new()
                .cancel(
                    capabilities
                        .cancel
                        .map(|cancel| acp_v2::SessionCancelCapabilities::new().meta(cancel.meta)),
                )
                .meta(capabilities.meta)
        }))
        .state(state)
        .cwd(match update.cwd {
            MaybeUndefined::Undefined => MaybeUndefined::Undefined,
            MaybeUndefined::Null => MaybeUndefined::Null,
            MaybeUndefined::Value(cwd) => {
                absolute_path_from_v1(cwd).map_or(MaybeUndefined::Undefined, MaybeUndefined::Value)
            }
        })
        .additional_directories(match update.additional_directories {
            MaybeUndefined::Undefined => MaybeUndefined::Undefined,
            MaybeUndefined::Null => MaybeUndefined::Null,
            MaybeUndefined::Value(directories) => directories
                .into_iter()
                .map(absolute_path_from_v1)
                .collect::<Option<Vec<_>>>()
                .map_or(MaybeUndefined::Undefined, MaybeUndefined::Value),
        }))
}

/// v1 types these paths as plain paths, but the subagents RFD requires them to be absolute.
/// Like a malformed v2 field, a relative one is ignored.
fn absolute_path_from_v1(path: PathBuf) -> Option<acp_v2::AbsolutePath> {
    if path.is_absolute() {
        Some(acp_v2::AbsolutePath::new(path))
    } else {
        log::warn!("Ignoring relative subagent directory {path:?}");
        None
    }
}

/// The subagents RFD gives the v1 snapshot the same wire shape as the v2 `state_update`,
/// so converting through JSON keeps unknown states and stop reasons intact.
fn state_from_v1(state: acp_v1::StateUpdate) -> Result<acp_v2::StateUpdate> {
    Ok(serde_json::from_value(serde_json::to_value(state)?)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use util::path;

    fn v2_update(value: serde_json::Value) -> acp_v2::SubagentUpdate {
        serde_json::from_value(value).expect("v2 subagent update")
    }

    #[test]
    fn association_fields_follow_patch_semantics() {
        let mut association = SubagentAssociation::default();
        association.apply(&v2_update(json!({
            "sessionId": "child",
            "title": "Investigator",
            "description": "Looks into failures",
            "capabilities": {"cancel": {}},
        })));
        let announced = SubagentAssociation {
            title: Some("Investigator".into()),
            description: Some("Looks into failures".into()),
            can_cancel: true,
            cwd: None,
            additional_directories: None,
        };
        assert_eq!(association, announced);

        association.apply(&v2_update(json!({"sessionId": "child"})));
        assert_eq!(association, announced, "omitted fields are unchanged");

        association.apply(&v2_update(json!({
            "sessionId": "child",
            "title": null,
            "capabilities": {"cancel": null},
        })));
        assert_eq!(
            association,
            SubagentAssociation {
                title: None,
                description: Some("Looks into failures".into()),
                can_cancel: false,
                cwd: None,
                additional_directories: None,
            },
            "null clears a field, and capabilities are replaced as a whole"
        );

        association.apply(&v2_update(json!({
            "sessionId": "child",
            "capabilities": {"cancel": {}},
        })));
        association.apply(&v2_update(
            json!({"sessionId": "child", "capabilities": null}),
        ));
        assert!(!association.can_cancel, "null capabilities disable cancel");
    }

    #[test]
    fn directories_follow_patch_semantics_and_inherit_independently() {
        let parent = PathList::new(&[path!("/project"), path!("/shared")]);
        let ordered = |work_dirs: Option<PathList>| {
            work_dirs
                .expect("effective directories")
                .ordered_paths()
                .map(|path| path.to_string_lossy().into_owned())
                .collect::<Vec<_>>()
        };
        let mut association = SubagentAssociation::default();
        assert_eq!(association.effective_work_dirs(None), None);
        assert_eq!(
            ordered(association.effective_work_dirs(Some(&parent))),
            [path!("/project"), path!("/shared")],
            "unreported directories are inherited"
        );

        association.apply(&v2_update(json!({
            "sessionId": "child",
            "cwd": path!("/worktree"),
        })));
        assert_eq!(
            ordered(association.effective_work_dirs(Some(&parent))),
            [path!("/worktree"), path!("/shared")],
            "the cwd and additional directories are inherited independently"
        );

        association.apply(&v2_update(json!({
            "sessionId": "child",
            "additionalDirectories": [],
        })));
        assert_eq!(
            ordered(association.effective_work_dirs(Some(&parent))),
            [path!("/worktree")]
        );
        association.apply(&v2_update(json!({"sessionId": "child"})));
        assert_eq!(
            ordered(association.effective_work_dirs(None)),
            [path!("/worktree")],
            "omitted fields are unchanged"
        );

        association.apply(&v2_update(json!({
            "sessionId": "child",
            "cwd": null,
            "additionalDirectories": [path!("/other"), path!("/project")],
        })));
        assert_eq!(
            ordered(association.effective_work_dirs(Some(&parent))),
            [path!("/project"), path!("/other")],
            "null inherits the parent's cwd again, and directories are replaced whole"
        );
    }

    #[test]
    fn v1_directories_must_be_absolute() -> Result<()> {
        for (v1, expected) in [
            (
                json!({
                    "sessionId": "child",
                    "cwd": path!("/worktree"),
                    "additionalDirectories": [path!("/shared")],
                }),
                json!({
                    "sessionId": "child",
                    "cwd": path!("/worktree"),
                    "additionalDirectories": [path!("/shared")],
                }),
            ),
            (
                json!({"sessionId": "child", "cwd": null, "additionalDirectories": null}),
                json!({"sessionId": "child", "cwd": null, "additionalDirectories": null}),
            ),
            (
                json!({
                    "sessionId": "child",
                    "cwd": "relative",
                    "additionalDirectories": [path!("/shared"), "relative"],
                }),
                json!({"sessionId": "child"}),
            ),
        ] {
            let converted = update_from_v1(serde_json::from_value(v1.clone())?)?;
            assert_eq!(serde_json::to_value(&converted)?, expected, "{v1}");
        }
        Ok(())
    }

    #[test]
    fn v1_updates_convert_fields_and_state_snapshots() -> Result<()> {
        for (v1, expected) in [
            (
                json!({
                    "sessionId": "child",
                    "title": "Investigator",
                    "description": null,
                    "capabilities": {"cancel": {}},
                    "state": {"state": "running"},
                    "_meta": {"dropped": true},
                }),
                json!({
                    "sessionId": "child",
                    "title": "Investigator",
                    "description": null,
                    "capabilities": {"cancel": {}},
                    "state": {"state": "running"},
                }),
            ),
            (
                json!({
                    "sessionId": "child",
                    "state": {
                        "state": "idle",
                        "stopReason": "error",
                        "error": {"code": -32000, "message": "Authentication required"},
                    },
                }),
                json!({
                    "sessionId": "child",
                    "state": {
                        "state": "idle",
                        "stopReason": "error",
                        "error": {"code": -32000, "message": "Authentication required"},
                    },
                }),
            ),
            (
                json!({"sessionId": "child", "state": {"state": "unknown"}}),
                json!({"sessionId": "child", "state": {"state": "unknown"}}),
            ),
            (
                json!({"sessionId": "child", "state": {"state": "_paused", "reason": "x"}}),
                json!({"sessionId": "child", "state": {"state": "_paused", "reason": "x"}}),
            ),
            (
                json!({"sessionId": "child", "state": null, "capabilities": null}),
                json!({"sessionId": "child", "state": null, "capabilities": null}),
            ),
        ] {
            let converted = update_from_v1(serde_json::from_value(v1.clone())?)?;
            assert_eq!(serde_json::to_value(&converted)?, expected, "{v1}");
        }
        Ok(())
    }
}
