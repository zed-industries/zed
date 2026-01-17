use std::{ops::RangeInclusive, path::PathBuf, time::Duration};

use acp_thread::MentionUri;
use agent_client_protocol as acp;
use editor::{Editor, SelectionEffects, scroll::Autoscroll};
use gpui::{
    Animation, AnimationExt, AnyView, Context, IntoElement, WeakEntity, Window, pulsating_between,
};
use prompt_store::PromptId;
use rope::Point;
use settings::Settings;
use theme::ThemeSettings;
use ui::{ButtonLike, TintColor, prelude::*};
use workspace::{OpenOptions, Workspace};

#[derive(IntoElement)]
pub struct MentionCrease {
    id: ElementId,
    icon: SharedString,
    label: SharedString,
    mention_uri: Option<MentionUri>,
    workspace: Option<WeakEntity<Workspace>>,
    is_toggled: bool,
    is_loading: bool,
    image_preview: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
}

impl MentionCrease {
    pub fn new(
        id: impl Into<ElementId>,
        icon: impl Into<SharedString>,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            label: label.into(),
            mention_uri: None,
            workspace: None,
            is_toggled: false,
            is_loading: false,
            image_preview: None,
        }
    }

    pub fn mention_uri(mut self, mention_uri: Option<MentionUri>) -> Self {
        self.mention_uri = mention_uri;
        self
    }

    pub fn workspace(mut self, workspace: Option<WeakEntity<Workspace>>) -> Self {
        self.workspace = workspace;
        self
    }

    pub fn is_toggled(mut self, is_toggled: bool) -> Self {
        self.is_toggled = is_toggled;
        self
    }

    pub fn is_loading(mut self, is_loading: bool) -> Self {
        self.is_loading = is_loading;
        self
    }

    pub fn image_preview(
        mut self,
        builder: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        self.image_preview = Some(Box::new(builder));
        self
    }
}

impl RenderOnce for MentionCrease {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let font_size = settings.agent_buffer_font_size(cx);
        let buffer_font = settings.buffer_font.clone();

        let button_height = DefiniteLength::Absolute(AbsoluteLength::Pixels(
            px(window.line_height().into()) - px(1.),
        ));

        ButtonLike::new(self.id)
            .style(ButtonStyle::Outlined)
            .size(ButtonSize::Compact)
            .height(button_height)
            .selected_style(ButtonStyle::Tinted(TintColor::Accent))
            .toggle_state(self.is_toggled)
            .when_some(self.image_preview, |this, image_preview| {
                this.hoverable_tooltip(image_preview)
            })
            .when(
                self.mention_uri.is_some() && self.workspace.is_some(),
                |this| {
                    let mention_uri = self.mention_uri.clone().unwrap();
                    let workspace = self.workspace.clone().unwrap();
                    this.on_click(move |_event, window, cx| {
                        open_mention_uri(mention_uri.clone(), &workspace, window, cx);
                    })
                },
            )
            .child(
                h_flex()
                    .pb_px()
                    .gap_1()
                    .font(buffer_font)
                    .text_size(font_size)
                    .child(
                        Icon::from_path(self.icon.clone())
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(self.label.clone())
                    .map(|this| {
                        if self.is_loading {
                            this.with_animation(
                                "loading-context-crease",
                                Animation::new(Duration::from_secs(2))
                                    .repeat()
                                    .with_easing(pulsating_between(0.4, 0.8)),
                                |label, delta| label.opacity(delta),
                            )
                            .into_any()
                        } else {
                            this.into_any()
                        }
                    }),
            )
    }
}

fn open_mention_uri(
    mention_uri: MentionUri,
    workspace: &WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace) = workspace.upgrade() else {
        return;
    };

    workspace.update(cx, |workspace, cx| match mention_uri {
        MentionUri::File { abs_path } => {
            open_file(workspace, abs_path, window, cx);
        }
        MentionUri::Symbol {
            abs_path,
            line_range,
            ..
        }
        | MentionUri::Selection {
            abs_path: Some(abs_path),
            line_range,
        } => {
            open_file_at_line(workspace, abs_path, line_range, window, cx);
        }
        MentionUri::Directory { abs_path } => {
            reveal_in_project_panel(workspace, abs_path, cx);
        }
        MentionUri::Thread { id, name } => {
            open_thread(workspace, id, name, window, cx);
        }
        MentionUri::TextThread { .. } => {}
        MentionUri::Rule { id, .. } => {
            open_rule(workspace, id, window, cx);
        }
        MentionUri::Fetch { url } => {
            cx.open_url(url.as_str());
        }
        MentionUri::PastedImage | MentionUri::Selection { abs_path: None, .. } => {}
    });
}

fn open_file(
    workspace: &mut Workspace,
    abs_path: PathBuf,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let project = workspace.project();

    if let Some(project_path) =
        project.update(cx, |project, cx| project.find_project_path(&abs_path, cx))
    {
        workspace
            .open_path(project_path, None, true, window, cx)
            .detach_and_log_err(cx);
    } else if abs_path.exists() {
        workspace
            .open_abs_path(
                abs_path,
                OpenOptions {
                    focus: Some(true),
                    ..Default::default()
                },
                window,
                cx,
            )
            .detach_and_log_err(cx);
    }
}

fn open_file_at_line(
    workspace: &mut Workspace,
    abs_path: PathBuf,
    line_range: RangeInclusive<u32>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let project = workspace.project();

    if let Some(project_path) =
        project.update(cx, |project, cx| project.find_project_path(&abs_path, cx))
    {
        let item = workspace.open_path(project_path, None, true, window, cx);
        window
            .spawn(cx, async move |cx| {
                let Some(editor) = item.await?.downcast::<Editor>() else {
                    return Ok(());
                };
                editor
                    .update_in(cx, |editor, window, cx| {
                        let range =
                            Point::new(*line_range.start(), 0)..Point::new(*line_range.start(), 0);
                        editor.change_selections(
                            SelectionEffects::scroll(Autoscroll::center()),
                            window,
                            cx,
                            |selections| selections.select_ranges(vec![range]),
                        );
                    })
                    .ok();
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
    } else if abs_path.exists() {
        workspace
            .open_abs_path(
                abs_path,
                OpenOptions {
                    focus: Some(true),
                    ..Default::default()
                },
                window,
                cx,
            )
            .detach_and_log_err(cx);
    }
}

fn reveal_in_project_panel(
    workspace: &mut Workspace,
    abs_path: PathBuf,
    cx: &mut Context<Workspace>,
) {
    let project = workspace.project();
    let Some(entry_id) = project.update(cx, |project, cx| {
        let path = project.find_project_path(&abs_path, cx)?;
        project.entry_for_path(&path, cx).map(|entry| entry.id)
    }) else {
        return;
    };

    project.update(cx, |_, cx| {
        cx.emit(project::Event::RevealInProjectPanel(entry_id));
    });
}

fn open_thread(
    workspace: &mut Workspace,
    id: acp::SessionId,
    name: String,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    use crate::AgentPanel;
    use acp_thread::AgentSessionInfo;

    let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
        return;
    };

    panel.update(cx, |panel, cx| {
        panel.load_agent_thread(
            AgentSessionInfo {
                session_id: id,
                cwd: None,
                title: Some(name.into()),
                updated_at: None,
                meta: None,
            },
            window,
            cx,
        )
    });
}

fn open_rule(
    _workspace: &mut Workspace,
    id: PromptId,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    use zed_actions::assistant::OpenRulesLibrary;

    let PromptId::User { uuid } = id else {
        return;
    };

    window.dispatch_action(
        Box::new(OpenRulesLibrary {
            prompt_to_select: Some(uuid.0),
        }),
        cx,
    );
}
