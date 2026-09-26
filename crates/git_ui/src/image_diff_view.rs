use anyhow::Result;
use git::repository::RepoPath;
use git::status::TreeDiffStatus;
use gpui::{
    AnyElement, App, AppContext as _, Context, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, ObjectFit, Render, StyledImage as _, Subscription, Task, WeakEntity, Window,
    checkerboard, img,
};
use language::DiskState;
use project::WorktreeId;
use project::{
    Project, ProjectPath,
    git_store::{
        GitFileRevision, Repository, RepositoryEvent, RepositoryId, StatusEntry,
        diff_buffer_list::DiffBase,
    },
    image_store::{ImageItem, ImageMetadata, create_gpui_image},
};
use std::{path::PathBuf, sync::Arc};
use ui::prelude::*;
use util::{ResultExt as _, paths::PathStyle, rel_path::RelPath, size::format_file_size};
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ImageDiffSource {
    Git(GitFileRevision),
    WorkingTree,
    Nothing,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ImageDiffInput {
    pub source: ImageDiffSource,
    pub label: SharedString,
}

impl ImageDiffInput {
    fn new(source: ImageDiffSource, label: impl Into<SharedString>) -> Self {
        Self {
            source,
            label: label.into(),
        }
    }
}

pub(crate) fn image_diff_inputs(
    diff_base: &DiffBase,
    branch_diff: Option<&TreeDiffStatus>,
) -> (ImageDiffInput, ImageDiffInput) {
    let head = || ImageDiffInput::new(ImageDiffSource::Git(GitFileRevision::Head), "HEAD");
    let index = || ImageDiffInput::new(ImageDiffSource::Git(GitFileRevision::Index), "Index");
    let working_tree = || ImageDiffInput::new(ImageDiffSource::WorkingTree, "Working Tree");
    match diff_base {
        DiffBase::Head => (head(), working_tree()),
        DiffBase::Index => (index(), working_tree()),
        DiffBase::Staged => (head(), index()),
        DiffBase::Merge { base_ref } => {
            let old = match branch_diff {
                Some(TreeDiffStatus::Added) => {
                    ImageDiffInput::new(ImageDiffSource::Nothing, base_ref.clone())
                }
                Some(TreeDiffStatus::Modified { old } | TreeDiffStatus::Deleted { old }) => {
                    ImageDiffInput::new(
                        ImageDiffSource::Git(GitFileRevision::Blob(*old)),
                        base_ref.clone(),
                    )
                }
                None => head(),
            };
            (old, working_tree())
        }
    }
}

pub(crate) struct ImageDiff {
    repository: Entity<Repository>,
    project: Entity<Project>,
    repo_path: RepoPath,
    project_path: ProjectPath,
    old_input: ImageDiffInput,
    new_input: ImageDiffInput,
    old_side: ImageDiffSide,
    new_side: ImageDiffSide,
    last_status_entry: Option<StatusEntry>,
    load_task: Task<()>,
    #[cfg(test)]
    reload_count: usize,
    _subscriptions: Vec<Subscription>,
}

impl ImageDiff {
    pub(crate) fn new(
        project: Entity<Project>,
        repository: Entity<Repository>,
        repo_path: RepoPath,
        project_path: ProjectPath,
        old_input: ImageDiffInput,
        new_input: ImageDiffInput,
        cx: &mut Context<Self>,
    ) -> Self {
        let last_status_entry = repository.read(cx).status_for_path(&repo_path);
        let subscriptions = vec![
            cx.subscribe(
                &repository,
                |this, repository, event: &RepositoryEvent, cx| {
                    let is_head_change = match event {
                        RepositoryEvent::HeadChanged => true,
                        RepositoryEvent::StatusesChanged => false,
                        _ => return,
                    };
                    let status_entry = repository.read(cx).status_for_path(&this.repo_path);
                    // Statuses change whenever any path in the repository changes, so only reload
                    // when this image's own entry does.
                    if is_head_change || status_entry != this.last_status_entry {
                        this.last_status_entry = status_entry;
                        this.reload(cx);
                    }
                },
            ),
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
            repository,
            project,
            repo_path,
            project_path,
            old_input,
            new_input,
            old_side: ImageDiffSide::Loading,
            new_side: ImageDiffSide::Loading,
            last_status_entry,
            load_task: Task::ready(()),
            #[cfg(test)]
            reload_count: 0,
            _subscriptions: subscriptions,
        };
        this.reload(cx);
        this
    }

    pub(crate) fn set_inputs(
        &mut self,
        old_input: ImageDiffInput,
        new_input: ImageDiffInput,
        cx: &mut Context<Self>,
    ) {
        if self.old_input != old_input || self.new_input != new_input {
            self.old_input = old_input;
            self.new_input = new_input;
            self.reload(cx);
        }
    }

    fn reload(&mut self, cx: &mut Context<Self>) {
        #[cfg(test)]
        {
            self.reload_count += 1;
        }
        let old_side = self.load_side(self.old_input.source, cx);
        let new_side = self.load_side(self.new_input.source, cx);
        self.load_task = cx.spawn(async move |this, cx| {
            let old_side = old_side.await;
            let new_side = new_side.await;
            this.update(cx, |this, cx| {
                this.old_side = old_side;
                this.new_side = new_side;
                cx.notify();
            })
            .log_err();
        });
    }

    fn load_side(&self, source: ImageDiffSource, cx: &mut Context<Self>) -> Task<ImageDiffSide> {
        match source {
            ImageDiffSource::Nothing => Task::ready(ImageDiffSide::Absent),
            ImageDiffSource::Git(revision) => {
                let repo_path = self.repo_path.clone();
                let bytes = self.repository.update(cx, |repository, cx| {
                    repository.load_file_bytes(revision, repo_path, cx)
                });
                cx.background_spawn(async move {
                    match bytes.await {
                        Ok(bytes) => ImageDiffSide::from_bytes(bytes),
                        Err(error) => ImageDiffSide::Failed(error.to_string().into()),
                    }
                })
            }
            ImageDiffSource::WorkingTree => {
                let project = self.project.read(cx);
                if !project.is_local() {
                    return Task::ready(ImageDiffSide::Failed(
                        "Image diffs are not yet supported in remote projects".into(),
                    ));
                }
                let fs = project.fs().clone();
                let Some(abs_path) = project.absolute_path(&self.project_path, cx) else {
                    return Task::ready(ImageDiffSide::Absent);
                };
                cx.background_spawn(async move {
                    // A missing working tree file means the image was deleted.
                    if !fs.is_file(&abs_path).await {
                        return ImageDiffSide::Absent;
                    }
                    match fs.load_bytes(&abs_path).await {
                        Ok(bytes) => ImageDiffSide::from_bytes(Some(bytes)),
                        Err(error) => ImageDiffSide::Failed(error.to_string().into()),
                    }
                })
            }
        }
    }
}

#[cfg(test)]
impl ImageDiff {
    pub(crate) fn describe_sides(&self) -> (String, String) {
        (
            format!("{:?}", self.old_side),
            format!("{:?}", self.new_side),
        )
    }

    pub(crate) fn reload_count(&self) -> usize {
        self.reload_count
    }
}

impl ImageDiff {
    fn render_pane(&self, pane: ImageDiffPane, cx: &App) -> AnyElement {
        let old = || {
            render_side(
                "old-image",
                self.old_input.label.clone(),
                &self.old_side,
                Color::Deleted,
                cx,
            )
        };
        let new = || {
            render_side(
                "new-image",
                self.new_input.label.clone(),
                &self.new_side,
                Color::Created,
                cx,
            )
        };
        match pane {
            ImageDiffPane::Both => h_flex()
                .size_full()
                .gap_2()
                .child(old())
                .child(new())
                .into_any_element(),
            ImageDiffPane::Old => old().into_any_element(),
            ImageDiffPane::New => new().into_any_element(),
        }
    }
}

impl Render for ImageDiff {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_pane(ImageDiffPane::Both, cx)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ImageDiffPane {
    Both,
    Old,
    New,
}

pub(crate) struct ImageDiffPaneView {
    image_diff: Entity<ImageDiff>,
    pane: ImageDiffPane,
    _observation: Subscription,
}

impl ImageDiffPaneView {
    pub(crate) fn new(
        image_diff: Entity<ImageDiff>,
        pane: ImageDiffPane,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            _observation: cx.observe(&image_diff, |_, _, cx| cx.notify()),
            image_diff,
            pane,
        }
    }
}

impl Render for ImageDiffPaneView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.image_diff.read(cx).render_pane(self.pane, cx)
    }
}

pub struct ImageDiffView {
    repository_id: RepositoryId,
    repo_path: RepoPath,
    image_diff: Entity<ImageDiff>,
    focus_handle: FocusHandle,
    _image_diff_observation: Subscription,
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
            let view = cx.new(|cx| {
                let repository_id = repository.read(cx).id;
                let (old_input, new_input) = image_diff_inputs(&DiffBase::Head, None);
                let image_diff = cx.new(|cx| {
                    ImageDiff::new(
                        project,
                        repository,
                        repo_path.clone(),
                        project_path,
                        old_input,
                        new_input,
                        cx,
                    )
                });
                Self {
                    repository_id,
                    repo_path,
                    _image_diff_observation: cx.observe(&image_diff, |_, _, cx| cx.notify()),
                    image_diff,
                    focus_handle: cx.focus_handle(),
                }
            });
            workspace.add_item_to_active_pane(Box::new(view.clone()), None, true, window, cx);
            view
        });
        Task::ready(Ok(view))
    }

    fn matches(&self, repository: &Entity<Repository>, repo_path: &RepoPath, cx: &App) -> bool {
        self.repository_id == repository.read(cx).id && &self.repo_path == repo_path
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
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .p_2()
            .bg(cx.theme().colors().editor_background)
            .child(self.image_diff.clone())
    }
}

fn render_side(
    id: &'static str,
    label: SharedString,
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

/// Images cannot be loaded as text buffers, so their diff sections use an empty placeholder
/// buffer backed by this file.
pub(crate) struct ImageDiffFile {
    pub path: Arc<RelPath>,
    pub worktree_id: WorktreeId,
    pub is_deleted: bool,
}

impl language::File for ImageDiffFile {
    fn as_local(&self) -> Option<&dyn language::LocalFile> {
        None
    }

    fn disk_state(&self) -> DiskState {
        DiskState::Historic {
            was_deleted: self.is_deleted,
        }
    }

    fn path_style(&self, _: &App) -> PathStyle {
        PathStyle::local()
    }

    fn path(&self) -> &Arc<RelPath> {
        &self.path
    }

    fn full_path(&self, _: &App) -> PathBuf {
        self.path.as_std_path().to_path_buf()
    }

    fn file_name<'a>(&'a self, _: &'a App) -> &'a str {
        self.path.file_name().unwrap_or_default()
    }

    fn worktree_id(&self, _: &App) -> WorktreeId {
        self.worktree_id
    }

    fn to_proto(&self, _cx: &App) -> language::proto::File {
        language::proto::File {
            worktree_id: self.worktree_id.to_proto(),
            entry_id: None,
            path: self.path.as_unix_str().to_owned(),
            mtime: None,
            is_deleted: self.is_deleted,
            is_historic: true,
        }
    }

    fn is_private(&self) -> bool {
        false
    }

    fn can_open(&self) -> bool {
        !self.is_deleted
    }
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
    fn test_image_diff_inputs_follow_diff_base() {
        let sources = |diff_base: &DiffBase, branch_diff: Option<&TreeDiffStatus>| {
            let (old, new) = image_diff_inputs(diff_base, branch_diff);
            (old.source, new.source)
        };
        let head = ImageDiffSource::Git(GitFileRevision::Head);
        let index = ImageDiffSource::Git(GitFileRevision::Index);

        assert_eq!(
            sources(&DiffBase::Head, None),
            (head, ImageDiffSource::WorkingTree)
        );
        assert_eq!(
            sources(&DiffBase::Index, None),
            (index, ImageDiffSource::WorkingTree)
        );
        assert_eq!(sources(&DiffBase::Staged, None), (head, index));

        let merge = DiffBase::Merge {
            base_ref: "main".into(),
        };
        let oid = git::Oid::from_bytes(&[7; 20]).unwrap();
        assert_eq!(
            sources(&merge, Some(&TreeDiffStatus::Modified { old: oid })),
            (
                ImageDiffSource::Git(GitFileRevision::Blob(oid)),
                ImageDiffSource::WorkingTree
            )
        );
        assert_eq!(
            sources(&merge, Some(&TreeDiffStatus::Added)),
            (ImageDiffSource::Nothing, ImageDiffSource::WorkingTree)
        );
        assert_eq!(sources(&merge, None), (head, ImageDiffSource::WorkingTree));
    }

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

        view.read_with(cx, |view, cx| {
            let image_diff = view.image_diff.read(cx);
            assert_eq!(format!("{:?}", image_diff.old_side), "Loaded(2x2)");
            assert_eq!(format!("{:?}", image_diff.new_side), "Loaded(4x4)");
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

        view.read_with(cx, |view, cx| {
            let image_diff = view.image_diff.read(cx);
            assert_eq!(format!("{:?}", image_diff.old_side), "Absent");
            assert_eq!(format!("{:?}", image_diff.new_side), "Loaded(4x4)");
        });
    }
}
