//! Editor-toolbar Build button for Plan Mode markdown files.
//!
//! When the active editor item is a file under `.zed/plans/` with an `.md`
//! extension, this toolbar shows a `Build` split-button on the right side of
//! the editor's primary toolbar. Clicking the button asks the active agent
//! thread (in the workspace's `AgentPanel`) to switch out of Plan Mode and
//! implement the plan; the chevron opens a model picker so the user can
//! choose which model performs the build.

use crate::{AgentPanel, conversation_view::ThreadView};
use editor::Editor;
use gpui::{
    App, Context, EventEmitter, IntoElement, ParentElement, Render, SharedString, Subscription,
    WeakEntity, Window, px,
};
use language_model::{LanguageModel, LanguageModelRegistry};
use settings::SettingsStore;
use std::path::PathBuf;
use std::sync::Arc;
use ui::{
    ButtonLike, ButtonStyle, Color, ContextMenu, Icon, IconButton, IconName, IconSize, Label,
    LabelSize, PopoverMenu, SplitButton, SplitButtonStyle, TintColor, Tooltip, prelude::*,
};
use workspace::{ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace};

pub struct PlanFileToolbar {
    /// Absolute path of the plan file currently shown in the active editor, if
    /// any. `None` when the toolbar is hidden.
    active_plan: Option<PathBuf>,
    workspace: WeakEntity<Workspace>,
    _settings_subscription: Subscription,
}

impl PlanFileToolbar {
    pub fn new(workspace: WeakEntity<Workspace>, cx: &mut Context<Self>) -> Self {
        Self {
            active_plan: None,
            workspace,
            _settings_subscription: cx
                .observe_global::<SettingsStore>(|this, cx| this.update_location(cx)),
        }
    }

    fn update_location(&mut self, cx: &mut Context<Self>) {
        let location = self.location();
        cx.emit(ToolbarItemEvent::ChangeLocation(location));
    }

    fn location(&self) -> ToolbarItemLocation {
        if self.active_plan.is_some() {
            ToolbarItemLocation::PrimaryRight
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    /// Returns the absolute path of a plan file (i.e. an `.md` file under a
    /// worktree's `.zed/plans/` directory) if the given item is an editor on
    /// such a file. Returns `None` otherwise.
    fn plan_path_for_item(item: &dyn ItemHandle, cx: &App) -> Option<PathBuf> {
        let editor = item.act_as::<Editor>(cx)?;
        if !editor.read(cx).mode().is_full() {
            return None;
        }
        let buffer = editor.read(cx).buffer().read(cx).as_singleton()?;
        let buffer = buffer.read(cx);
        let file = buffer.file()?;
        let abs_path = file.as_local()?.abs_path(cx);
        if !is_plan_file_path(&abs_path) {
            return None;
        }
        Some(abs_path)
    }

    fn build_with_model(
        &mut self,
        model: Option<Arc<dyn LanguageModel>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(plan_path) = self.active_plan.clone() else {
            return;
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let Some(thread_view) = workspace
            .read(cx)
            .panel::<AgentPanel>(cx)
            .and_then(|panel| panel.read(cx).active_thread_view(cx))
        else {
            // Surface a friendly message via a workspace toast in the future;
            // for now we no-op so the toolbar click doesn't crash.
            log::warn!("PlanFileToolbar: cannot Build — agent panel has no active thread");
            return;
        };
        thread_view.update(cx, |view: &mut ThreadView, cx| {
            view.build_plan(plan_path, model, window, cx);
        });
    }
}

/// Returns true for absolute paths that look like a plan file maintained by
/// Plan Mode: an `.md` file whose path contains a `.zed/plans/` segment.
fn is_plan_file_path(path: &std::path::Path) -> bool {
    let Some(ext) = path.extension() else {
        return false;
    };
    if !ext.eq_ignore_ascii_case("md") {
        return false;
    }
    let mut components = path.components();
    while let Some(component) = components.next() {
        if component.as_os_str() == ".zed" {
            // The very next component must be `plans`.
            if let Some(next) = components.next()
                && next.as_os_str() == "plans"
            {
                return true;
            }
        }
    }
    false
}

impl EventEmitter<ToolbarItemEvent> for PlanFileToolbar {}

impl ToolbarItemView for PlanFileToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.active_plan = active_pane_item.and_then(|item| Self::plan_path_for_item(item, cx));
        self.location()
    }

    fn pane_focus_update(
        &mut self,
        _pane_focused: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

impl Render for PlanFileToolbar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.active_plan.is_none() {
            return gpui::Empty.into_any_element();
        }

        let current_model_name: Option<SharedString> = LanguageModelRegistry::read_global(cx)
            .default_model()
            .map(|m| m.model.name().0);

        let weak_self = cx.weak_entity();

        let left = ButtonLike::new("plan-toolbar-build")
            .style(ButtonStyle::Tinted(TintColor::Accent))
            .child(Icon::new(IconName::PlayFilled).size(IconSize::Small))
            .child(Label::new("Build").size(LabelSize::Small))
            .tooltip({
                let model_name = current_model_name.clone();
                move |_window, cx| {
                    let body = match model_name.as_ref() {
                        Some(name) => format!(
                            "Switch the active thread to Write mode and implement this plan with {name}"
                        ),
                        None => "Switch the active thread to Write mode and implement this plan"
                            .to_string(),
                    };
                    Tooltip::simple(body, cx)
                }
            })
            .on_click({
                let weak_self = weak_self.clone();
                move |_, window, cx| {
                    weak_self
                        .update(cx, |this, cx| this.build_with_model(None, window, cx))
                        .ok();
                }
            });

        let menu_weak = weak_self.clone();
        let right = PopoverMenu::new("plan-toolbar-build-model")
            .trigger(
                IconButton::new("plan-toolbar-build-model-trigger", IconName::ChevronDown)
                    .icon_size(IconSize::XSmall)
                    .icon_color(Color::Muted),
            )
            .anchor(gpui::Anchor::BottomRight)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
            .menu(move |window, cx| {
                let weak_self = menu_weak.clone();
                Some(ContextMenu::build(
                    window,
                    cx,
                    move |mut menu, _window, cx| {
                        let models: Vec<Arc<dyn LanguageModel>> =
                            LanguageModelRegistry::read_global(cx)
                                .available_models(cx)
                                .collect();
                        if models.is_empty() {
                            menu = menu.label("No models configured");
                        } else {
                            menu = menu.header("Build with model");
                            for model in models {
                                let label =
                                    format!("{} · {}", model.provider_name().0, model.name().0);
                                let weak_self = weak_self.clone();
                                let model = model.clone();
                                menu = menu.entry(label, None, move |window, cx| {
                                    let model = model.clone();
                                    weak_self
                                        .update(cx, |this, cx| {
                                            this.build_with_model(Some(model), window, cx);
                                        })
                                        .ok();
                                });
                            }
                        }
                        menu
                    },
                ))
            });

        SplitButton::new(left, right.into_any_element())
            .style(SplitButtonStyle::Filled)
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::is_plan_file_path;
    use std::path::Path;

    #[test]
    fn matches_md_under_dot_zed_plans() {
        assert!(is_plan_file_path(Path::new(
            "/work/zed/.zed/plans/add-auth.md"
        )));
        assert!(is_plan_file_path(Path::new(
            "C:\\work\\zed\\.zed\\plans\\add-auth.md"
        )));
    }

    #[test]
    fn rejects_md_outside_plans_dir() {
        assert!(!is_plan_file_path(Path::new("/work/zed/.zed/settings.md")));
        assert!(!is_plan_file_path(Path::new("/work/zed/README.md")));
        assert!(!is_plan_file_path(Path::new("/work/zed/docs/plans/foo.md")));
    }

    #[test]
    fn rejects_non_md_under_plans() {
        assert!(!is_plan_file_path(Path::new(
            "/work/zed/.zed/plans/notes.txt"
        )));
        assert!(!is_plan_file_path(Path::new("/work/zed/.zed/plans/README")));
    }
}
