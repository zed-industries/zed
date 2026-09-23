use std::path::PathBuf;

use agent_settings::{AgentSettings, normalize_sandbox_write_path};
use gpui::{ReadGlobal as _, ScrollHandle, prelude::*};
use http_proxy::HostPattern;
use settings::{Settings as _, SettingsStore};
use ui::{Banner, Divider, Severity, SwitchField, ToggleState, Tooltip, prelude::*};
use util::ResultExt as _;

use crate::SettingsWindow;
use crate::components::{SettingsInputField, SettingsSectionHeader};

const DOMAINS_DESCRIPTION: &str = "Each entry is an exact domain (github.com) or a leading-*. subdomain wildcard (*.npmjs.org). IP addresses and local domains are not allowed.";

const WRITE_PATHS_DESCRIPTION: &str = "Absolute or home-relative paths (e.g. ~/.cache/example). Home paths use Zed's home directory, including the Windows home on Windows. Grants cover the whole subtree, except protected Git metadata.";

pub(crate) fn render_sandbox_settings_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    // Sandbox permissions are a user-level setting; they aren't configurable
    // per-project, so always operate against the global value here.
    let permissions = AgentSettings::get_global(cx).sandbox_permissions.clone();
    let validation_error = settings_window.sandbox_host_validation_error.clone();

    // Read the list values from the raw user settings content rather than the
    // compiled `AgentSettings`. The compiled `write_paths` are lexically
    // normalized (see `compile_sandbox_permissions`), so editing or removing a
    // row by the normalized value would fail to match the literal entry stored
    // in settings.json and silently leave the permission in place.
    let (network_hosts, write_paths) = raw_sandbox_lists(cx);

    let host_rows: Vec<AnyElement> = network_hosts
        .into_iter()
        .enumerate()
        .map(|(index, host)| render_host_row(index, host, cx))
        .collect();
    let add_host_input = render_add_host_input(cx);

    let path_rows: Vec<AnyElement> = write_paths
        .into_iter()
        .enumerate()
        .map(|(index, path)| render_path_row(index, path, cx))
        .collect();
    let add_path_input = render_add_path_input(cx);

    let empty_border = cx.theme().colors().border_variant;
    let sandbox_enabled = !permissions.allow_unsandboxed;

    v_flex()
        .id("sandbox-settings-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .gap_6()
        .overflow_y_scroll()
        .track_scroll(scroll_handle)
        .child(
            SwitchField::new(
                "sandbox-enabled",
                Some("Enable Sandbox"),
                Some(
                    "Wrap agent-run terminal commands in an OS-level sandbox. When off, commands run with Zed's own permissions."
                        .into(),
                ),
                sandbox_enabled,
                move |state, _window, cx| {
                    set_sandbox_enabled(*state == ToggleState::Selected, cx);
                },
            )
            .tab_index(0),
        )
        .child({
            let docs_url =
                client::zed_urls::sandboxing_docs(Some("persistent-sandbox-permissions"), cx);
            let tooltip = format!("Opens {docs_url}");
            // Wrap in a row so the button shrinks to its content width instead
            // of stretching across the settings page.
            h_flex().child(
                Button::new("sandbox-docs-link", "Learn more about sandboxing")
                    .label_size(LabelSize::Small)
                    .color(Color::Muted)
                    .end_icon(
                        Icon::new(IconName::ArrowUpRight)
                            .color(Color::Muted)
                            .size(IconSize::XSmall),
                    )
                    .tooltip(Tooltip::text(tooltip))
                    .on_click(move |_, _, cx| cx.open_url(&docs_url)),
            )
        })
        .when(sandbox_enabled, |this| this
        .when_some(validation_error, |this, error| {
            this.child(
                Banner::new()
                    .severity(Severity::Warning)
                    .child(Label::new(error).size(LabelSize::Small))
                    .action_slot(
                        Button::new("dismiss-sandbox-host-error", "Dismiss")
                            .style(ButtonStyle::Tinted(ui::TintColor::Warning))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.sandbox_host_validation_error = None;
                                cx.notify();
                            })),
                    ),
            )
        })
        .child(
            v_flex()
                .gap_4()
                .child(SettingsSectionHeader::new("Network").no_padding(true))
                .child(
                    SwitchField::new(
                        "sandbox-allow-all-hosts",
                        Some("Allow All Domains"),
                        Some(
                            "Let sandboxed commands reach any domain over the network without prompting."
                                .into(),
                        ),
                        permissions.allow_all_hosts,
                        move |state, _window, cx| {
                            set_allow_all_hosts(*state == ToggleState::Selected, cx);
                        },
                    )
                    .tab_index(0),
                )
                .child(render_list_section(
                    "Allowed Domains",
                    DOMAINS_DESCRIPTION,
                    host_rows,
                    add_host_input,
                    empty_border,
                )),
        )

        .child(Divider::horizontal())
        .child(
            v_flex()
                .gap_4()
                .child(SettingsSectionHeader::new("File System").no_padding(true))
                .child(
                    SwitchField::new(
                        "sandbox-allow-fs-write-all",
                        Some("Allow All File System Writes"),
                        Some(
                            "Let sandboxed commands write anywhere except protected Git metadata without prompting."
                                .into(),
                        ),
                        permissions.allow_fs_write_all,
                        move |state, _window, cx| {
                            set_allow_fs_write_all(*state == ToggleState::Selected, cx);
                        },
                    )
                    .tab_index(0),
                )
                .child(render_list_section(
                    "Writable Paths",
                    WRITE_PATHS_DESCRIPTION,
                    path_rows,
                    add_path_input,
                    empty_border,
                )),
        )
        .child(Divider::horizontal())
        .child(
            v_flex()
                .gap_4()
                .child(SettingsSectionHeader::new("Escalation Prompts").no_padding(true))
                .child(
                    SwitchField::new(
                        "sandbox-warn-confusable-unicode",
                        Some("Warn About Confusable Unicode"),
                        Some(
                            "Warn when an approval prompt requests a domain or write path that contains potentially confusable Unicode characters, such as homoglyphs (i.e. two symbols that look similar, such as a Cyrillic `а`)"
                                .into(),
                        ),
                        permissions.warn_confusable_unicode,
                        move |state, _window, cx| {
                            set_warn_confusable_unicode(*state == ToggleState::Selected, cx);
                        },
                    )
                    .tab_index(0),
                )
                .child(
                    SwitchField::new(
                        "sandbox-warn-ntfs-grants",
                        Some("Warn About Windows-Drive Grants"),
                        Some(
                            "Windows only: warn when a sandbox grant targets a file on a Windows drive (accessed inside WSL via DrvFs). Such grants are enforced through a translated path and their sandbox-integrity guarantees are weaker than files on the Linux distro's own filesystem."
                                .into(),
                        ),
                        permissions.warn_ntfs_grants,
                        move |state, _window, cx| {
                            set_warn_ntfs_grants(*state == ToggleState::Selected, cx);
                        },
                    )
                    .tab_index(0),
                ),
        )
        )
        .into_any_element()
}

fn render_list_section(
    title: &'static str,
    description: &'static str,
    rows: Vec<AnyElement>,
    add_input: AnyElement,
    empty_border: gpui::Hsla,
) -> impl IntoElement {
    let is_empty = rows.is_empty();

    v_flex()
        .gap_0p5()
        .child(Label::new(title))
        .child(
            Label::new(description)
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
        .child(
            v_flex()
                .mt_2()
                .w_full()
                .gap_1p5()
                .when(is_empty, |this| {
                    this.child(render_empty_state(empty_border))
                })
                .when(!is_empty, |this| {
                    this.child(v_flex().gap_1p5().children(rows))
                })
                .child(add_input),
        )
}

fn render_empty_state(border_color: gpui::Hsla) -> AnyElement {
    h_flex()
        .p_2()
        .rounded_md()
        .border_1()
        .border_dashed()
        .border_color(border_color)
        .child(
            Label::new("Nothing configured")
                .size(LabelSize::Small)
                .color(Color::Disabled),
        )
        .into_any_element()
}

fn render_host_row(index: usize, host: String, cx: &mut Context<SettingsWindow>) -> AnyElement {
    let host_for_delete = host.clone();
    let host_for_update = host.clone();
    let settings_window = cx.entity().downgrade();

    SettingsInputField::new(format!("sandbox-host-{}", index))
        .with_initial_text(host)
        .tab_index(0)
        .with_buffer_font()
        .color(Color::Default)
        .action_slot(
            IconButton::new(format!("sandbox-host-delete-{}", index), IconName::Trash)
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted)
                .tooltip(Tooltip::text("Remove Domain"))
                .on_click(cx.listener(move |_, _, _, cx| {
                    remove_network_host(host_for_delete.clone(), cx);
                })),
        )
        .on_confirm(move |new_host, _window, cx| {
            let Some(new_host) = new_host else {
                return;
            };
            let new_host = new_host.trim().to_string();
            if new_host.is_empty() || new_host == host_for_update {
                return;
            }
            let result = canonicalize_host(&new_host);
            settings_window
                .update(cx, |this, cx| {
                    match result {
                        Ok(canonical) => {
                            this.sandbox_host_validation_error = None;
                            update_network_host(host_for_update.clone(), canonical, cx);
                        }
                        Err(error) => {
                            this.sandbox_host_validation_error = Some(error);
                        }
                    }
                    cx.notify();
                })
                .log_err();
        })
        .into_any_element()
}

fn render_add_host_input(cx: &mut Context<SettingsWindow>) -> AnyElement {
    let settings_window = cx.entity().downgrade();

    SettingsInputField::new("sandbox-host-new")
        .with_placeholder("Add domain (e.g. github.com or *.npmjs.org)…")
        .tab_index(0)
        .with_buffer_font()
        .display_clear_button()
        .display_confirm_button()
        .clear_on_confirm()
        .on_confirm(move |host, _window, cx| {
            let Some(host) = host else {
                return;
            };
            let host = host.trim().to_string();
            if host.is_empty() {
                return;
            }
            let result = canonicalize_host(&host);
            settings_window
                .update(cx, |this, cx| {
                    match result {
                        Ok(canonical) => {
                            this.sandbox_host_validation_error = None;
                            add_network_host(canonical, cx);
                        }
                        Err(error) => {
                            this.sandbox_host_validation_error = Some(error);
                        }
                    }
                    cx.notify();
                })
                .log_err();
        })
        .into_any_element()
}

fn render_path_row(index: usize, path: PathBuf, cx: &mut Context<SettingsWindow>) -> AnyElement {
    let path_for_delete = path.clone();
    let path_for_update = path.clone();
    let settings_window = cx.entity().downgrade();

    SettingsInputField::new(format!("sandbox-path-{}", index))
        .with_initial_text(path.to_string_lossy().into_owned())
        .tab_index(0)
        .with_buffer_font()
        .color(Color::Default)
        .action_slot(
            IconButton::new(format!("sandbox-path-delete-{}", index), IconName::Trash)
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted)
                .tooltip(Tooltip::text("Remove Path"))
                .on_click(cx.listener(move |_, _, _, cx| {
                    remove_write_path(path_for_delete.clone(), cx);
                })),
        )
        .on_confirm(move |new_path, _window, cx| {
            let Some(new_path) = new_path else {
                return;
            };
            let new_path = new_path.trim();
            if new_path.is_empty() {
                return;
            }
            let new_path = PathBuf::from(new_path);
            if new_path == path_for_update {
                return;
            }
            update_write_path(path_for_update.clone(), new_path, cx);
            settings_window.update(cx, |_, cx| cx.notify()).log_err();
        })
        .into_any_element()
}

fn render_add_path_input(cx: &mut Context<SettingsWindow>) -> AnyElement {
    let settings_window = cx.entity().downgrade();

    SettingsInputField::new("sandbox-path-new")
        .with_placeholder("Add a path (e.g. ~/.cache/example)…")
        .tab_index(0)
        .with_buffer_font()
        .display_clear_button()
        .display_confirm_button()
        .clear_on_confirm()
        .on_confirm(move |path, _window, cx| {
            let Some(path) = path else {
                return;
            };
            let path = path.trim();
            if path.is_empty() {
                return;
            }
            add_write_path(PathBuf::from(path), cx);
            settings_window.update(cx, |_, cx| cx.notify()).log_err();
        })
        .into_any_element()
}

/// The literal host and write-path lists as stored in user settings.json. These
/// are the exact strings/paths that edits and removals must match against.
fn raw_sandbox_lists(cx: &App) -> (Vec<String>, Vec<PathBuf>) {
    let store = SettingsStore::global(cx);
    let permissions = store
        .raw_user_settings()
        .and_then(|user| user.content.agent.as_ref())
        .and_then(|agent| agent.sandbox_permissions.as_ref());

    let network_hosts = permissions
        .and_then(|permissions| permissions.network_hosts.as_ref())
        .map(|hosts| hosts.0.clone())
        .unwrap_or_default();
    // Display and match on the requested path of each entry (the literal a
    // hand-edit types); Zed-written "allow always" grants also carry a resolved
    // canonical, but the settings row still keys off the requested path.
    let write_paths = permissions
        .and_then(|permissions| permissions.write_paths.as_ref())
        .map(|paths| {
            paths
                .0
                .iter()
                .map(|entry| entry.requested.clone())
                .collect()
        })
        .unwrap_or_default();

    (network_hosts, write_paths)
}

/// Validate and canonicalize a user-provided domain, returning either the
/// canonical form to persist or a domain-friendly error to surface.
fn canonicalize_host(host: &str) -> Result<String, String> {
    use http_proxy::HostPatternError;

    HostPattern::parse(host)
        .map(|pattern| pattern.to_string())
        .map_err(|error| match error {
            HostPatternError::Empty => "Domain cannot be empty.".to_string(),
            HostPatternError::IpLiteral(_) => {
                "IP addresses and local domains aren't allowed; enter a domain like github.com."
                    .to_string()
            }
            HostPatternError::InvalidWildcard(_) => {
                "Wildcards are only allowed as a leading label, e.g. *.github.com.".to_string()
            }
            HostPatternError::Invalid { .. } => {
                "Not a valid domain. Use a domain like github.com or *.npmjs.org.".to_string()
            }
        })
}

fn update_sandbox_permissions(
    cx: &mut App,
    update: impl 'static + Send + FnOnce(&mut settings::SandboxPermissionsContent),
) {
    SettingsStore::global(cx).update_settings_file(<dyn fs::Fs>::global(cx), move |settings, _| {
        update(
            settings
                .agent
                .get_or_insert_default()
                .sandbox_permissions
                .get_or_insert_default(),
        );
    });
}

fn set_sandbox_enabled(value: bool, cx: &mut App) {
    // The UI presents an "enabled" switch, but the stored setting is the
    // inverse (`allow_unsandboxed`).
    update_sandbox_permissions(cx, move |permissions| {
        permissions.allow_unsandboxed = Some(!value);
    });
}

fn set_allow_all_hosts(value: bool, cx: &mut App) {
    update_sandbox_permissions(cx, move |permissions| {
        permissions.allow_all_hosts = Some(value);
    });
}

fn set_allow_fs_write_all(value: bool, cx: &mut App) {
    update_sandbox_permissions(cx, move |permissions| {
        permissions.allow_fs_write_all = Some(value);
    });
}

fn set_warn_confusable_unicode(value: bool, cx: &mut App) {
    update_sandbox_permissions(cx, move |permissions| {
        permissions.warn_confusable_unicode = Some(value);
    });
}

fn set_warn_ntfs_grants(value: bool, cx: &mut App) {
    update_sandbox_permissions(cx, move |permissions| {
        permissions.warn_ntfs_grants = Some(value);
    });
}

fn add_network_host(host: String, cx: &mut App) {
    update_sandbox_permissions(cx, move |permissions| {
        let hosts = &mut permissions.network_hosts.get_or_insert_default().0;
        if !hosts.contains(&host) {
            hosts.push(host);
        }
    });
}

fn update_network_host(old_host: String, new_host: String, cx: &mut App) {
    update_sandbox_permissions(cx, move |permissions| {
        let hosts = &mut permissions.network_hosts.get_or_insert_default().0;
        if hosts.contains(&new_host) {
            return;
        }
        if let Some(entry) = hosts.iter_mut().find(|host| **host == old_host) {
            *entry = new_host;
        }
    });
}

fn remove_network_host(host: String, cx: &mut App) {
    update_sandbox_permissions(cx, move |permissions| {
        if let Some(hosts) = permissions.network_hosts.as_mut() {
            hosts.0.retain(|entry| *entry != host);
        }
    });
}

fn insert_write_path_subtree(
    paths: &mut Vec<settings::GrantedWritePathContent>,
    path: PathBuf,
    normalized_path: PathBuf,
) {
    let normalize_entry = |entry: &settings::GrantedWritePathContent| match &entry.resolved {
        Some(resolved) => util::paths::normalize_lexically(resolved).map_err(anyhow::Error::from),
        None => normalize_sandbox_write_path(&entry.requested),
    };
    if paths.iter().any(|entry| {
        normalize_entry(entry).is_ok_and(|existing| normalized_path.starts_with(existing))
    }) {
        return;
    }
    paths.retain(|entry| {
        !normalize_entry(entry).is_ok_and(|existing| existing.starts_with(&normalized_path))
    });
    // Keep home-relative spelling portable, and do not turn an ordinary relative
    // path into a home-relative entry on the next settings load
    let requested = if path.starts_with("~") || normalized_path.starts_with("~") {
        path
    } else {
        normalized_path
    };
    paths.push(settings::GrantedWritePathContent {
        requested,
        resolved: None,
        on_windows_fs: false,
    });
}

fn add_write_path(path: PathBuf, cx: &mut App) {
    let Ok(normalized_path) = normalize_sandbox_write_path(&path) else {
        return;
    };
    update_sandbox_permissions(cx, move |permissions| {
        let paths = &mut permissions.write_paths.get_or_insert_default().0;
        insert_write_path_subtree(paths, path, normalized_path);
    });
}

fn update_write_path(old_path: PathBuf, new_path: PathBuf, cx: &mut App) {
    let Ok(normalized_path) = normalize_sandbox_write_path(&new_path) else {
        return;
    };
    update_sandbox_permissions(cx, move |permissions| {
        if let Some(paths) = permissions.write_paths.as_mut() {
            paths.0.retain(|entry| entry.requested != old_path);
            insert_write_path_subtree(&mut paths.0, new_path, normalized_path);
        }
    });
}

fn remove_write_path(path: PathBuf, cx: &mut App) {
    update_sandbox_permissions(cx, move |permissions| {
        if let Some(paths) = permissions.write_paths.as_mut() {
            paths.0.retain(|entry| entry.requested != path);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::{FakeFs, Fs as _};
    use gpui::TestAppContext;
    use serde_json::{Value, json};
    use std::sync::Arc;

    async fn setup(write_paths: Value, cx: &mut TestAppContext) -> anyhow::Result<Arc<FakeFs>> {
        let fs = FakeFs::new(cx.executor());
        fs.create_dir(
            paths::settings_file()
                .parent()
                .expect("settings has a parent"),
        )
        .await?;
        let text = json!({ "agent": { "sandbox_permissions": { "write_paths": write_paths } } })
            .to_string();
        fs.insert_file(paths::settings_file(), text.as_bytes().to_vec())
            .await;
        cx.update(|cx| {
            let mut store = SettingsStore::test(cx);
            store.set_user_settings(&text, cx).result()?;
            cx.set_global(store);
            <dyn fs::Fs>::set_global(fs.clone(), cx);
            anyhow::Ok(())
        })?;
        Ok(fs)
    }

    async fn stored_write_paths(fs: &FakeFs) -> anyhow::Result<Value> {
        let text = fs.load(paths::settings_file()).await?;
        let settings: Value = serde_json::from_str(&text)?;
        Ok(settings["agent"]["sandbox_permissions"]["write_paths"].clone())
    }

    #[gpui::test]
    async fn test_home_path_edits_preserve_portable_json(cx: &mut TestAppContext) {
        let fs = setup(json!([]), cx).await.expect("settings setup succeeds");
        let path = PathBuf::from("~/../shared");
        cx.update(|cx| add_write_path(path.clone(), cx));
        cx.run_until_parked();
        assert_eq!(
            stored_write_paths(&fs).await.expect("valid settings"),
            json!(["~/../shared"])
        );
        assert_eq!(cx.read(raw_sandbox_lists).1, vec![path.clone()]);

        let edited_path = PathBuf::from("~/.cache/example");
        cx.update(|cx| update_write_path(path, edited_path.clone(), cx));
        cx.run_until_parked();
        assert_eq!(
            stored_write_paths(&fs).await.expect("valid settings"),
            json!(["~/.cache/example"])
        );
        assert_eq!(cx.read(raw_sandbox_lists).1, vec![edited_path.clone()]);

        cx.update(|cx| remove_write_path(edited_path, cx));
        cx.run_until_parked();
        assert_eq!(
            stored_write_paths(&fs).await.expect("valid settings"),
            json!([])
        );
        assert!(cx.read(raw_sandbox_lists).1.is_empty());
    }

    #[gpui::test]
    async fn test_relative_path_edits_do_not_introduce_home_prefix(cx: &mut TestAppContext) {
        let fs = setup(json!([]), cx).await.expect("settings setup succeeds");
        let path = PathBuf::from("relative/../~");
        cx.update(|cx| add_write_path(path.clone(), cx));
        cx.run_until_parked();
        assert_eq!(
            stored_write_paths(&fs).await.expect("valid settings"),
            json!(["relative/../~"])
        );

        cx.update(|cx| update_write_path(path, PathBuf::from("relative/../~/cache"), cx));
        cx.run_until_parked();
        assert_eq!(
            stored_write_paths(&fs).await.expect("valid settings"),
            json!(["relative/../~/cache"])
        );
    }

    #[gpui::test]
    async fn test_home_path_edits_compare_effective_paths(cx: &mut TestAppContext) {
        let home = util::paths::home_dir();
        let fs = setup(json!(["~/cache/child", home.join("other")]), cx)
            .await
            .expect("settings setup succeeds");
        cx.update(|cx| add_write_path(PathBuf::from("~/cache"), cx));
        cx.run_until_parked();
        assert_eq!(
            stored_write_paths(&fs).await.expect("valid settings"),
            json!([home.join("other"), "~/cache"])
        );

        cx.update(|cx| add_write_path(home.join("cache"), cx));
        cx.run_until_parked();
        assert_eq!(
            stored_write_paths(&fs).await.expect("valid settings"),
            json!([home.join("other"), "~/cache"])
        );

        cx.update(|cx| update_write_path(home.join("other"), PathBuf::from("~/../shared"), cx));
        cx.run_until_parked();
        assert_eq!(
            stored_write_paths(&fs).await.expect("valid settings"),
            json!(["~/cache", "~/../shared"])
        );

        cx.update(|cx| remove_write_path(PathBuf::from("~/cache"), cx));
        cx.run_until_parked();
        assert_eq!(
            stored_write_paths(&fs).await.expect("valid settings"),
            json!(["~/../shared"])
        );
    }

    #[gpui::test]
    async fn test_home_path_edits_preserve_approved_grants(cx: &mut TestAppContext) {
        let home = util::paths::home_dir();
        let approved = json!({
            "requested": home.join("link"),
            "resolved": home.join("cache"),
            "on_windows_fs": true,
        });
        let fs = setup(json!([approved]), cx)
            .await
            .expect("settings setup succeeds");
        cx.update(|cx| add_write_path(PathBuf::from("~/cache"), cx));
        cx.run_until_parked();
        assert_eq!(
            stored_write_paths(&fs).await.expect("valid settings"),
            json!([approved])
        );

        cx.update(|cx| add_write_path(PathBuf::from("~/other"), cx));
        cx.run_until_parked();
        cx.update(|cx| {
            update_write_path(PathBuf::from("~/other"), PathBuf::from("~/cache/child"), cx)
        });
        cx.run_until_parked();
        assert_eq!(
            stored_write_paths(&fs).await.expect("valid settings"),
            json!([approved])
        );

        cx.update(|cx| remove_write_path(home.join("link"), cx));
        cx.run_until_parked();
        assert_eq!(
            stored_write_paths(&fs).await.expect("valid settings"),
            json!([])
        );
    }
}
