use anyhow::Result;
use git::repository::RepoPath;
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ObjectFit, Render, StyledImage as _, Subscription, Task, WeakEntity, Window, checkerboard, img,
};
use project::{
    Project, ProjectPath,
    git_store::{GitFileRevision, Repository, RepositoryEvent, RepositoryId},
    image_store::{ImageItem, ImageMetadata, create_gpui_image},
};
use std::sync::Arc;
use ui::prelude::*;
use util::{ResultExt as _, paths::PathStyle, size::format_file_size};
use workspace::{Item, Workspace};

const CHECKERBOARD_SQUARE_SIZE: f32 = 16.;

#[derive(Clone)]
pub(crate) enum ImageDiffSide {
    Loading,
    Absent,
    Loaded {
        image: Arc<gpui::Image>,
        metadata: ImageMetadata,
    },
    Failed(SharedString),
}

impl std::fmt::Debug for ImageDiffSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Loading => write!(f, "Loading"),
            Self::Absent => write!(f, "Absent"),
            Self::Loaded { metadata, .. } => {
                write!(f, "Loaded({}x{})", metadata.width, metadata.height)
            }
            Self::Failed(message) => write!(f, "Failed({message})"),
        }
    }
}

impl ImageDiffSide {
    fn from_bytes(bytes: Option<Vec<u8>>) -> Self {
        let Some(bytes) = bytes else {
            return Self::Absent;
        };
        match Self::decode(bytes) {
            Ok(side) => side,
            Err(error) => Self::Failed(error.to_string().into()),
        }
    }

    fn decode(bytes: Vec<u8>) -> Result<Self> {
        let metadata = ImageItem::compute_metadata_from_bytes(&bytes)?;
        let image = create_gpui_image(bytes)?;
        Ok(Self::Loaded { image, metadata })
    }
}

pub struct ImageDiffView {
    repository: Entity<Repository>,
    repository_id: RepositoryId,
    repo_path: RepoPath,
    project: Entity<Project>,
    project_path: ProjectPath,
    old_side: ImageDiffSide,
    new_side: ImageDiffSide,
    focus_handle: FocusHandle,
    load_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl ImageDiffView {
    pub fn open_or_focus(
        repo_path: RepoPath,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow::anyhow!("workspace was dropped")));
        };

        let existing = workspace
            .read(cx)
            .items_of_type::<ImageDiffView>(cx)
            .find(|item| item.read(cx).matches(&repository, &repo_path, cx));
        if let Some(existing) = existing {
            workspace.update(cx, |workspace, cx| {
                workspace.activate_item(&existing, true, true, window, cx);
            });
            return Task::ready(Ok(existing));
        }

        let Some(project_path) = repository
            .read(cx)
            .repo_path_to_project_path(&repo_path, cx)
        else {
            return Task::ready(Err(anyhow::anyhow!(
                "could not resolve repository path {repo_path:?}"
            )));
        };

        let view = workspace.update(cx, |workspace, cx| {
            let project = workspace.project().clone();
            let view = cx.new(|cx| Self::new(project, repository, repo_path, project_path, cx));
            workspace.add_item_to_active_pane(Box::new(view.clone()), None, true, window, cx);
            view
        });
        Task::ready(Ok(view))
    }

    fn new(
        project: Entity<Project>,
        repository: Entity<Repository>,
        repo_path: RepoPath,
        project_path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.subscribe(&repository, |this, _, event: &RepositoryEvent, cx| {
                if matches!(
                    event,
                    RepositoryEvent::StatusesChanged | RepositoryEvent::HeadChanged
                ) {
                    this.reload(cx);
                }
            }),
            // Rewriting an already modified image does not change its git status, so watch the
            // worktree entry as well to pick up new working tree contents.
            cx.subscribe(&project, |this, _, event: &project::Event, cx| {
                if let project::Event::WorktreeUpdatedEntries(worktree_id, changes) = event
                    && *worktree_id == this.project_path.worktree_id
                    && changes
                        .iter()
                        .any(|(path, _, _)| *path == this.project_path.path)
                {
                    this.reload(cx);
                }
            }),
        ];

        let mut this = Self {
            repository_id: repository.read(cx).id,
            repository,
            repo_path,
            project,
            project_path,
            old_side: ImageDiffSide::Loading,
            new_side: ImageDiffSide::Loading,
            focus_handle: cx.focus_handle(),
            load_task: Task::ready(()),
            _subscriptions: subscriptions,
        };
        this.reload(cx);
        this
    }

    fn matches(&self, repository: &Entity<Repository>, repo_path: &RepoPath, cx: &App) -> bool {
        self.repository_id == repository.read(cx).id && &self.repo_path == repo_path
    }

    fn reload(&mut self, cx: &mut Context<Self>) {
        let repo_path = self.repo_path.clone();
        let old_bytes = self.repository.update(cx, |repository, cx| {
            repository.load_file_bytes(GitFileRevision::Head, repo_path, cx)
        });
        let project = self.project.read(cx);
        let is_local = project.is_local();
        let fs = project.fs().clone();
        let abs_path = project.absolute_path(&self.project_path, cx);

        self.load_task = cx.spawn(async move |this, cx| {
            let old_side = match old_bytes.await {
                Ok(bytes) => ImageDiffSide::from_bytes(bytes),
                Err(error) => ImageDiffSide::Failed(error.to_string().into()),
            };
            let new_side = if !is_local {
                ImageDiffSide::Failed("Image diffs are not yet supported in remote projects".into())
            } else {
                match abs_path {
                    // A missing working tree file means the image was deleted.
                    Some(abs_path) if fs.is_file(&abs_path).await => {
                        match fs.load_bytes(&abs_path).await {
                            Ok(bytes) => ImageDiffSide::from_bytes(Some(bytes)),
                            Err(error) => ImageDiffSide::Failed(error.to_string().into()),
                        }
                    }
                    _ => ImageDiffSide::Absent,
                }
            };
            this.update(cx, |this, cx| {
                this.old_side = old_side;
                this.new_side = new_side;
                cx.notify();
            })
            .log_err();
        });
    }

    fn file_name(&self) -> String {
        self.repo_path
            .file_name()
            .map(|name| name.to_string())
            .unwrap_or_else(|| {
                self.repo_path
                    .as_ref()
                    .display(PathStyle::local())
                    .into_owned()
            })
    }
}

impl EventEmitter<()> for ImageDiffView {}

impl Focusable for ImageDiffView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ImageDiffView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Image).color(Color::Muted))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("{} (Diff)", self.file_name()).into()
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        Some(
            self.repo_path
                .as_ref()
                .display(PathStyle::local())
                .into_owned()
                .into(),
        )
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Image Diff View Opened")
    }
}

impl Render for ImageDiffView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .gap_2()
            .p_2()
            .bg(cx.theme().colors().editor_background)
            .child(render_side(
                "old-image",
                "HEAD",
                &self.old_side,
                Color::Deleted,
                cx,
            ))
            .child(render_side(
                "new-image",
                "Working Tree",
                &self.new_side,
                Color::Created,
                cx,
            ))
    }
}

fn render_side(
    id: &'static str,
    label: &'static str,
    side: &ImageDiffSide,
    accent: Color,
    cx: &App,
) -> impl IntoElement {
    let colors = cx.theme().colors();
    let caption = match side {
        ImageDiffSide::Loaded { metadata, .. } => format!(
            "{label} · {}×{} · {}",
            metadata.width,
            metadata.height,
            format_file_size(metadata.file_size, true)
        ),
        _ => label.to_string(),
    };
    let body = match side {
        ImageDiffSide::Loading => Label::new("Loading…")
            .color(Color::Muted)
            .into_any_element(),
        ImageDiffSide::Absent => Label::new("No image")
            .color(Color::Muted)
            .into_any_element(),
        ImageDiffSide::Failed(message) => Label::new(message.clone())
            .color(Color::Error)
            .into_any_element(),
        ImageDiffSide::Loaded { image, .. } => div()
            .size_full()
            .bg(checkerboard(
                colors.panel_background,
                CHECKERBOARD_SQUARE_SIZE,
            ))
            .child(
                img(image.clone())
                    .size_full()
                    .object_fit(ObjectFit::Contain),
            )
            .into_any_element(),
    };

    v_flex()
        .id(id)
        .flex_1()
        .h_full()
        .min_w_0()
        .gap_1()
        .child(Label::new(caption).size(LabelSize::Small).color(accent))
        .child(
            div()
                .flex_1()
                .w_full()
                .min_h_0()
                .flex()
                .items_center()
                .justify_center()
                .border_1()
                .border_color(colors.border)
                .child(body),
        )
}

#[cfg(test)]
pub(crate) fn png_bytes(width: u32, height: u32) -> Vec<u8> {
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbaImage::new(width, height)
        .write_to(&mut bytes, image::ImageFormat::Png)
        .expect("encode png");
    bytes.into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::Path;
    use util::path;
    use workspace::MultiWorkspace;

    #[test]
    fn test_side_from_bytes() {
        assert!(matches!(
            ImageDiffSide::from_bytes(None),
            ImageDiffSide::Absent
        ));

        match ImageDiffSide::from_bytes(Some(png_bytes(3, 2))) {
            ImageDiffSide::Loaded { metadata, .. } => {
                assert_eq!((metadata.width, metadata.height), (3, 2));
            }
            other => panic!("expected Loaded, got {other:?}"),
        }

        assert!(matches!(
            ImageDiffSide::from_bytes(Some(b"not an image".to_vec())),
            ImageDiffSide::Failed(_)
        ));
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    async fn open_view(
        head_image: Option<Vec<u8>>,
        cx: &mut TestAppContext,
    ) -> (
        Entity<ImageDiffView>,
        Entity<Workspace>,
        Entity<Repository>,
        &mut gpui::VisualTestContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({ ".git": {} }))
            .await;
        fs.insert_file(path!("/project/logo.png"), png_bytes(4, 4))
            .await;
        if let Some(head_image) = head_image {
            fs.with_git_state(Path::new(path!("/project/.git")), true, |state| {
                state
                    .head_contents
                    .insert(RepoPath::new("logo.png").unwrap(), head_image);
            })
            .unwrap();
        }

        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        cx.run_until_parked();

        let repository =
            project.read_with(cx, |project, cx| project.active_repository(cx).unwrap());
        let view = cx
            .update(|window, cx| {
                ImageDiffView::open_or_focus(
                    RepoPath::new("logo.png").unwrap(),
                    repository.clone(),
                    workspace.downgrade(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap();
        cx.run_until_parked();
        (view, workspace, repository, cx)
    }

    #[gpui::test]
    async fn test_image_diff_view_loads_head_and_working_tree(cx: &mut TestAppContext) {
        let (view, workspace, repository, cx) = open_view(Some(png_bytes(2, 2)), cx).await;

        view.read_with(cx, |view, _| {
            assert_eq!(format!("{:?}", view.old_side), "Loaded(2x2)");
            assert_eq!(format!("{:?}", view.new_side), "Loaded(4x4)");
        });

        let again = cx
            .update(|window, cx| {
                ImageDiffView::open_or_focus(
                    RepoPath::new("logo.png").unwrap(),
                    repository.clone(),
                    workspace.downgrade(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(again.entity_id(), view.entity_id());
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.items_of_type::<ImageDiffView>(cx).count(), 1);
        });
    }

    #[gpui::test]
    async fn test_image_diff_view_added_file_has_absent_old_side(cx: &mut TestAppContext) {
        let (view, _workspace, _repository, cx) = open_view(None, cx).await;

        view.read_with(cx, |view, _| {
            assert_eq!(format!("{:?}", view.old_side), "Absent");
            assert_eq!(format!("{:?}", view.new_side), "Loaded(4x4)");
        });
    }
}
