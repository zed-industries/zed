use anyhow::{Context as _, Result};
use db::kvp::KeyValueStore;
use gpui::{
    App, AsyncWindowContext, Context, DismissEvent, EventEmitter, FocusHandle, Focusable,
    PromptButton, PromptLevel, Render, WeakEntity, Window,
};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use ui::{
    ActiveTheme, Color, CommonAnimationExt, Icon, IconName, IconSize, IntoElement, Label,
    LabelCommon, LabelSize, ParentElement, Styled, StyledExt, div, h_flex, v_flex,
};
use util::ResultExt;
use util::command::new_command;
use workspace::{ModalView, MultiWorkspace};

const DONT_ASK_AGAIN_KEY: &str = "move_to_applications_dont_ask_again";
static PROMPTED_THIS_SESSION: AtomicBool = AtomicBool::new(false);

pub fn init(cx: &mut App) {
    let kvp = KeyValueStore::global(cx);
    if matches!(kvp.read_kvp(DONT_ASK_AGAIN_KEY), Ok(Some(value)) if value == "true") {
        return;
    }

    let Some(request) = MoveToApplicationsRequest::new(cx).log_err().flatten() else {
        return;
    };

    cx.observe_new(move |_workspace: &mut MultiWorkspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        if PROMPTED_THIS_SESSION.swap(true, Ordering::AcqRel) {
            return;
        }

        let request = request.clone();
        cx.spawn_in(window, async move |workspace, cx| {
            request.prompt(workspace, cx).await.log_err();
        })
        .detach();
    })
    .detach();
}

#[derive(Clone)]
struct MoveToApplicationsRequest {
    app_path: PathBuf,
}

impl MoveToApplicationsRequest {
    fn new(cx: &App) -> Result<Option<Self>> {
        let app_path = match cx.app_path() {
            Ok(app_path) => app_path,
            Err(_) => return Ok(None),
        };

        if !should_offer_to_move(&app_path) {
            return Ok(None);
        }

        Ok(Some(Self { app_path }))
    }

    async fn prompt(
        self,
        workspace: WeakEntity<MultiWorkspace>,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let response = cx
            .prompt(
                PromptLevel::Info,
                "Move Zed to Applications?",
                Some(
                    "Zed is running from a temporary location. Move it to Applications to finish installing it.",
                ),
                &[
                    PromptButton::ok("Yes"),
                    PromptButton::cancel("No"),
                    PromptButton::new("Don't ask me again"),
                ],
            )
            .await?;

        match response {
            0 => {
                workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace
                            .toggle_modal(window, cx, |_window, cx| InstallingZedModal::new(cx));
                    })
                    .ok();
                if let Err(error) = move_to_applications(&self.app_path, cx).await {
                    workspace
                        .update_in(cx, |workspace, _window, cx| {
                            if let Some(modal) = workspace.active_modal::<InstallingZedModal>(cx) {
                                modal.update(cx, |modal, cx| modal.finished(cx));
                            }
                        })
                        .ok();
                    cx.prompt(
                        PromptLevel::Critical,
                        "Failed to move Zed to Applications",
                        Some(&error.to_string()),
                        &["Ok"],
                    )
                    .await
                    .log_err();
                }
            }
            2 => {
                let kvp = cx.update(|_window, cx| KeyValueStore::global(cx))?;
                kvp.write_kvp(DONT_ASK_AGAIN_KEY.to_string(), "true".to_string())
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}

pub struct InstallingZedModal {
    focus_handle: FocusHandle,
    finished: bool,
}

impl InstallingZedModal {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            finished: false,
        }
    }

    fn finished(&mut self, cx: &mut Context<Self>) {
        self.finished = true;
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for InstallingZedModal {}

impl ModalView for InstallingZedModal {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        workspace::DismissDecision::Dismiss(self.finished)
    }

    fn fade_out_background(&self) -> bool {
        true
    }
}

impl Focusable for InstallingZedModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for InstallingZedModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .elevation_3(cx)
            .w_80()
            .overflow_hidden()
            .child(
                div()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(theme.colors().border_variant)
                    .child(Label::new("Installing Zed…")),
            )
            .child(
                h_flex()
                    .w_full()
                    .gap_3()
                    .px_4()
                    .py_3()
                    .bg(theme.colors().editor_background)
                    .child(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::Medium)
                            .color(Color::Accent)
                            .with_rotate_animation(3),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("Moving Zed to Applications"))
                            .child(
                                Label::new("Zed will reopen when installation is complete.")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    ),
            )
    }
}

fn should_offer_to_move(app_path: &Path) -> bool {
    app_path.starts_with(Path::new("/Volumes"))
        || app_path.to_string_lossy().contains("/AppTranslocation/")
}

async fn move_to_applications(app_path: &Path, cx: &mut AsyncWindowContext) -> Result<()> {
    let destination_path = install_destination(app_path).await?;
    restart_into(destination_path, cx)
}

async fn install_destination(app_path: &Path) -> Result<PathBuf> {
    let app_name = app_path
        .file_name()
        .context("invalid app path: missing app bundle name")?;

    let system_destination = Path::new("/Applications").join(app_name);
    if system_destination.exists() {
        copy_app_bundle(app_path, &system_destination)
            .await
            .with_context(|| {
                format!(
                    "failed to replace existing app at {}",
                    system_destination.display()
                )
            })?;
        return Ok(system_destination);
    }

    if let Some(user_destination) = user_applications_directory().map(|path| path.join(app_name))
        && user_destination.exists()
    {
        copy_app_bundle(app_path, &user_destination)
            .await
            .with_context(|| {
                format!(
                    "failed to replace existing app at {}",
                    user_destination.display()
                )
            })?;
        return Ok(user_destination);
    }

    match copy_app_bundle(app_path, &system_destination).await {
        Ok(()) => Ok(system_destination),
        Err(system_error) => {
            let user_applications_directory = user_applications_directory()
                .context("could not determine a writable Applications directory")?;
            smol::fs::create_dir_all(&user_applications_directory)
                .await
                .with_context(|| {
                    format!("failed to create {}", user_applications_directory.display())
                })?;
            let user_destination = user_applications_directory.join(app_name);
            copy_app_bundle(app_path, &user_destination)
                .await
                .with_context(|| {
                    format!(
                        "failed to copy app to {} after system Applications copy failed: {system_error:#}",
                        user_destination.display()
                    )
                })?;
            Ok(user_destination)
        }
    }
}

async fn copy_app_bundle(source: &Path, destination: &Path) -> Result<()> {
    let parent = destination
        .parent()
        .context("invalid destination path: missing parent directory")?;
    smol::fs::create_dir_all(parent)
        .await
        .with_context(|| format!("failed to create {}", parent.display()))?;

    let mut source_with_contents: OsString = source.into();
    source_with_contents.push("/");
    let mut destination_with_contents: OsString = destination.into();
    destination_with_contents.push("/");

    let mut command = new_command("rsync");
    command
        .args(["-a", "--delete"])
        .arg(&source_with_contents)
        .arg(&destination_with_contents);
    let output = command
        .output()
        .await
        .with_context(|| format!("failed to run rsync for {}", source.display()))?;

    anyhow::ensure!(
        output.status.success(),
        "failed to copy app bundle: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(())
}

fn restart_into(app_path: PathBuf, cx: &mut AsyncWindowContext) -> Result<()> {
    cx.update(|_window, cx| {
        cx.set_restart_path(app_path);
        cx.restart();
    })?;
    Ok(())
}

fn user_applications_directory() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join("Applications"))
}
