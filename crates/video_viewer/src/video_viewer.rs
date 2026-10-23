#![cfg(any(target_os = "macos", target_os = "windows"))]

#[cfg(target_os = "macos")]
mod macos_player;
#[cfg(target_os = "windows")]
mod windows_player;

use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result};
use editor::{EditorSettings, items::entry_git_aware_label_color};
use file_icons::FileIcons;
use gpui::{
    AnyElement, App, Bounds, Context, Entity, EventEmitter, FocusHandle, Focusable, Font,
    KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ObjectFit, Pixels,
    Render, Subscription, Task, WeakEntity, Window, actions, canvas, div, px, relative,
};
use language::{DiskState, File as _};
use persistence::VideoViewerDb;
use project::{Project, ProjectEntryId, ProjectPath, git_store::GitStoreEvent};
use settings::Settings as _;
use theme_settings::ThemeSettings;
use ui::{Tooltip, prelude::*};
use util::paths::PathExt as _;
use workspace::{
    ItemId, ItemSettings, Pane, ToolbarItemLocation, Workspace, WorkspaceId, delete_unloaded_items,
    invalid_item_view::InvalidItemView,
    item::{HighlightedText, Item, ProjectItem, SerializableItem, TabContentParams},
};
use worktree::{File, Worktree};

#[cfg(target_os = "macos")]
use crate::macos_player::{Frame, Player};
#[cfg(target_os = "windows")]
use crate::windows_player::{Frame, Player};

actions!(
    video_viewer,
    [
        /// Play or pause the video.
        TogglePlayback,
        /// Skip forward five seconds.
        SeekForward,
        /// Skip back five seconds.
        SeekBackward,
        /// Mute or unmute the video.
        ToggleMute,
    ]
);

/// Containers the platform media framework plays: AVFoundation on macOS, Media Foundation on
/// Windows (which also reads Matroska, WebM, AVI and WMV).
#[cfg(target_os = "macos")]
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "m4v", "mov"];
#[cfg(target_os = "windows")]
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "m4v", "mov", "mkv", "webm", "avi", "wmv"];

const SEEK_STEP: Duration = Duration::from_secs(5);
const VOLUME_STEP: f32 = 0.1;
/// Shorter than one frame at 120 fps, so no decoded frame is skipped.
const PLAYING_POLL_INTERVAL: Duration = Duration::from_millis(8);
/// Paused, only seeks and state changes produce updates.
const PAUSED_POLL_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerState {
    Loading,
    Ready,
    Failed(String),
}

pub fn is_video_file(path: &ProjectPath) -> bool {
    path.path.extension().is_some_and(|extension| {
        VIDEO_EXTENSIONS
            .iter()
            .any(|video_extension| extension.eq_ignore_ascii_case(video_extension))
    })
}

pub struct VideoItem {
    file: Arc<File>,
    abs_path: PathBuf,
}

impl VideoItem {
    fn load(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let Some(worktree) = project.read(cx).worktree_for_id(path.worktree_id, cx) else {
            return Task::ready(Err(anyhow::anyhow!("no worktree for path {:?}", path.path)));
        };
        let fs = project.read(cx).fs().clone();
        let path = path.path.clone();
        cx.spawn(async move |cx| {
            let (abs_path, entry) = worktree.read_with(cx, |worktree, _| {
                (
                    worktree.absolutize(&path),
                    worktree.entry_for_path(&path).cloned(),
                )
            });
            let metadata = fs
                .metadata(&abs_path)
                .await?
                .with_context(|| format!("{} does not exist", abs_path.display()))?;
            let file = Arc::new(File {
                worktree,
                path,
                disk_state: DiskState::Present {
                    mtime: metadata.mtime,
                    size: metadata.len,
                },
                entry_id: entry.as_ref().map(|entry| entry.id),
                is_local: true,
                is_private: entry.is_some_and(|entry| entry.is_private),
            });
            Ok(cx.new(|_| Self { file, abs_path }))
        })
    }

    fn project_path(&self, cx: &App) -> ProjectPath {
        ProjectPath {
            worktree_id: self.file.worktree_id(cx),
            path: self.file.path().clone(),
        }
    }
}

impl project::ProjectItem for VideoItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        if !is_video_file(path) {
            return None;
        }
        // The platform player reads the file from disk, which remote worktrees cannot provide.
        let worktree: Entity<Worktree> = project.read(cx).worktree_for_id(path.worktree_id, cx)?;
        if !worktree.read(cx).is_local() {
            return None;
        }
        Some(Self::load(project, path, cx))
    }

    fn entry_id(&self, _: &App) -> Option<ProjectEntryId> {
        self.file.entry_id
    }

    fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        Some(self.project_path(cx))
    }

    fn is_dirty(&self) -> bool {
        false
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Drag {
    Seek(f32),
    Volume,
}

/// The frame on screen. On Windows each frame is a GPUI image in the sprite atlas; the previous
/// frame stays allocated until the next one arrives, because the GPU may still be drawing it.
#[cfg(target_os = "macos")]
struct DisplayedFrame {
    current: Frame,
}

#[cfg(target_os = "windows")]
struct DisplayedFrame {
    current: Arc<gpui::RenderImage>,
    previous: Option<Arc<gpui::RenderImage>>,
}

pub struct VideoViewer {
    video_item: Entity<VideoItem>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    player: Result<Player, SharedString>,
    state: PlayerState,
    showed_first_frame: bool,
    /// The play state the controls last showed. Playback can stop on its own at the end of the
    /// video, which produces no other event, so polling compares against this to redraw.
    shown_playing: bool,
    frame: Option<DisplayedFrame>,
    position: Duration,
    volume: f32,
    muted: bool,
    drag: Option<Drag>,
    seek_bar_bounds: Rc<Cell<Bounds<Pixels>>>,
    volume_bar_bounds: Rc<Cell<Bounds<Pixels>>>,
    _poll_player: Task<()>,
    _subscriptions: Vec<Subscription>,
}

pub enum VideoViewerEvent {
    TitleChanged,
}

impl VideoViewer {
    pub fn new(
        video_item: Entity<VideoItem>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let player = Player::open(&video_item.read(cx).abs_path)
            .map_err(|error| SharedString::from(format!("{error:#}")));
        let poll_player = cx.spawn_in(window, async move |this, cx| {
            loop {
                let Ok(interval) = this.read_with(cx, |this, _| {
                    if this.is_playing() {
                        PLAYING_POLL_INTERVAL
                    } else {
                        PAUSED_POLL_INTERVAL
                    }
                }) else {
                    break;
                };
                cx.background_executor().timer(interval).await;
                if this
                    .update_in(cx, |this, window, cx| this.poll_player(window, cx))
                    .is_err()
                {
                    break;
                }
            }
        });
        let git_store = project.read(cx).git_store().clone();
        let subscriptions = vec![cx.subscribe(&git_store, |_, _, event, cx| {
            if matches!(event, GitStoreEvent::DiffBaseChanged(_)) {
                cx.emit(VideoViewerEvent::TitleChanged);
            }
        })];
        #[cfg(target_os = "windows")]
        cx.on_release_in(window, |this, window, _| {
            if let Some(frame) = this.frame.take() {
                drop_frame_images(frame, window);
            }
        })
        .detach();

        Self {
            video_item,
            project,
            focus_handle: cx.focus_handle(),
            player,
            state: PlayerState::Loading,
            showed_first_frame: false,
            shown_playing: false,
            frame: None,
            position: Duration::ZERO,
            volume: 1.0,
            muted: false,
            drag: None,
            seek_bar_bounds: Rc::default(),
            volume_bar_bounds: Rc::default(),
            _poll_player: poll_player,
            _subscriptions: subscriptions,
        }
    }

    fn poll_player(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Ok(player) = &self.player else {
            return;
        };
        let state = player.state();
        if state == PlayerState::Ready && !self.showed_first_frame {
            // A paused player decodes nothing; seeking makes it decode the first frame.
            player.seek(Duration::ZERO);
            player.set_volume(self.effective_volume());
            self.showed_first_frame = true;
        }
        let frame = player.take_frame();
        let position = player.position();
        let is_playing = player.is_playing();

        if state != self.state || is_playing != self.shown_playing {
            self.state = state;
            self.shown_playing = is_playing;
            cx.notify();
        }
        if let Some(frame) = frame {
            self.show_frame(frame, window);
            cx.notify();
        }
        if !matches!(self.drag, Some(Drag::Seek(_))) {
            let label_changed = position.as_millis() / 100 != self.position.as_millis() / 100;
            self.position = position;
            if label_changed {
                cx.notify();
            }
        }
    }

    #[cfg(target_os = "macos")]
    fn show_frame(&mut self, frame: Frame, _: &mut Window) {
        self.frame = Some(DisplayedFrame { current: frame });
    }

    #[cfg(target_os = "windows")]
    fn show_frame(&mut self, frame: Frame, window: &mut Window) {
        use util::ResultExt as _;

        let Some(buffer) = image::RgbaImage::from_raw(frame.width, frame.height, frame.bgra) else {
            log::error!("video frame has an unexpected size");
            return;
        };
        let image = Arc::new(gpui::RenderImage::new(smallvec::SmallVec::from_elem(
            image::Frame::new(buffer),
            1,
        )));
        let previous = match self.frame.take() {
            Some(DisplayedFrame { current, previous }) => {
                if let Some(older) = previous {
                    window.drop_image(older).log_err();
                }
                Some(current)
            }
            None => None,
        };
        self.frame = Some(DisplayedFrame {
            current: image,
            previous,
        });
    }

    fn duration(&self) -> Option<Duration> {
        self.player.as_ref().ok()?.duration()
    }

    fn is_playing(&self) -> bool {
        self.player.as_ref().is_ok_and(|player| player.is_playing())
    }

    fn effective_volume(&self) -> f32 {
        if self.muted { 0.0 } else { self.volume }
    }

    fn toggle_playback(&mut self, _: &TogglePlayback, _: &mut Window, cx: &mut Context<Self>) {
        let Ok(player) = &self.player else {
            return;
        };
        if self.state != PlayerState::Ready {
            return;
        }
        if player.is_playing() {
            player.pause();
        } else {
            player.play();
        }
        cx.notify();
    }

    fn seek_to(&mut self, position: Duration, cx: &mut Context<Self>) {
        let Ok(player) = &self.player else {
            return;
        };
        let position = match player.duration() {
            Some(duration) => position.min(duration),
            None => position,
        };
        player.seek(position);
        self.position = position;
        cx.notify();
    }

    fn seek_forward(&mut self, _: &SeekForward, _: &mut Window, cx: &mut Context<Self>) {
        self.seek_to(self.position + SEEK_STEP, cx);
    }

    fn seek_backward(&mut self, _: &SeekBackward, _: &mut Window, cx: &mut Context<Self>) {
        self.seek_to(self.position.saturating_sub(SEEK_STEP), cx);
    }

    fn toggle_mute(&mut self, _: &ToggleMute, _: &mut Window, cx: &mut Context<Self>) {
        self.muted = !self.muted;
        self.apply_volume(cx);
    }

    fn set_volume(&mut self, volume: f32, cx: &mut Context<Self>) {
        self.volume = volume.clamp(0.0, 1.0);
        self.muted = self.volume == 0.0;
        self.apply_volume(cx);
    }

    fn apply_volume(&mut self, cx: &mut Context<Self>) {
        if let Ok(player) = &self.player {
            player.set_volume(self.effective_volume());
        }
        cx.notify();
    }

    fn handle_key_down(&mut self, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let modifiers = &event.keystroke.modifiers;
        if modifiers.control || modifiers.platform || modifiers.alt || modifiers.shift {
            return;
        }
        let step = match event.keystroke.key.as_str() {
            "up" => VOLUME_STEP,
            "down" => -VOLUME_STEP,
            _ => return,
        };
        cx.stop_propagation();
        self.set_volume(self.volume + step, cx);
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.drag {
            Some(Drag::Seek(_)) if event.pressed_button == Some(MouseButton::Left) => {
                let fraction = fraction_at(event.position.x, self.seek_bar_bounds.get());
                self.drag = Some(Drag::Seek(fraction));
                cx.notify();
            }
            Some(Drag::Volume) if event.pressed_button == Some(MouseButton::Left) => {
                self.set_volume(
                    fraction_at(event.position.x, self.volume_bar_bounds.get()),
                    cx,
                );
            }
            Some(_) => self.finish_drag(cx),
            None => {}
        }
    }

    fn handle_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.finish_drag(cx);
    }

    fn finish_drag(&mut self, cx: &mut Context<Self>) {
        if let Some(Drag::Seek(fraction)) = self.drag.take()
            && let Some(duration) = self.duration()
        {
            self.seek_to(duration.mul_f32(fraction), cx);
        }
        cx.notify();
    }

    fn render_frame(&self) -> Option<AnyElement> {
        let frame = self.frame.as_ref()?;
        #[cfg(target_os = "macos")]
        let element = gpui::surface(frame.current.clone())
            .object_fit(ObjectFit::Contain)
            .size_full()
            .into_any_element();
        #[cfg(target_os = "windows")]
        let element = gpui::img(frame.current.clone())
            .object_fit(ObjectFit::Contain)
            .size_full()
            .into_any_element();
        Some(element)
    }

    fn render_status(&self, cx: &App) -> Option<AnyElement> {
        let error = match (&self.player, &self.state) {
            (Err(error), _) => error.clone(),
            (Ok(_), PlayerState::Failed(error)) => error.clone().into(),
            (Ok(_), PlayerState::Loading) => {
                return Some(
                    Label::new("Loading…")
                        .color(Color::Muted)
                        .into_any_element(),
                );
            }
            (Ok(player), PlayerState::Ready) => {
                return (!player.has_video()).then(|| {
                    Icon::new(IconName::AudioOn)
                        .size(IconSize::Custom(rems(3.0)))
                        .color(Color::Muted)
                        .into_any_element()
                });
            }
        };
        let abs_path = self.video_item.read(cx).abs_path.clone();
        Some(
            v_flex()
                .items_center()
                .gap_2()
                .child(Label::new(error).color(Color::Error))
                .child(
                    Button::new("open-with-system", "Open in Default App")
                        .style(ButtonStyle::Outlined)
                        .on_click(move |_, _, cx| cx.open_with_system(&abs_path)),
                )
                .into_any_element(),
        )
    }

    fn render_controls(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let duration = self.duration();
        let shown_position = match (self.drag, duration) {
            (Some(Drag::Seek(fraction)), Some(duration)) => duration.mul_f32(fraction),
            _ => self.position,
        };
        let progress = match duration {
            Some(duration) if !duration.is_zero() => {
                (shown_position.as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0)
            }
            _ => 0.0,
        };
        let is_playing = self.is_playing();
        let can_play = self.state == PlayerState::Ready;
        let volume_icon = if self.muted || self.volume == 0.0 {
            IconName::AudioOff
        } else {
            IconName::AudioOn
        };
        let time_label = |time: Option<Duration>| {
            Label::new(time.map_or_else(|| "--:--".to_string(), format_time))
                .size(LabelSize::Small)
                .color(Color::Muted)
                .buffer_font(cx)
        };

        h_flex()
            .w_full()
            .px_3()
            .py_2()
            .gap_2()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().editor_background)
            .child(
                IconButton::new(
                    "toggle-playback",
                    if is_playing {
                        IconName::PauseFilled
                    } else {
                        IconName::PlayFilled
                    },
                )
                .icon_size(IconSize::Medium)
                .disabled(!can_play)
                .tooltip(move |_, cx| {
                    Tooltip::for_action(
                        if is_playing { "Pause" } else { "Play" },
                        &TogglePlayback,
                        cx,
                    )
                })
                .on_click(cx.listener(|this, _, window, cx| {
                    this.toggle_playback(&TogglePlayback, window, cx)
                })),
            )
            .child(time_label(Some(shown_position)))
            .child(
                slider("seek-bar", progress, self.seek_bar_bounds.clone(), cx)
                    .flex_1()
                    .when(duration.is_some(), |this| {
                        this.on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, event: &MouseDownEvent, _, cx| {
                                let fraction =
                                    fraction_at(event.position.x, this.seek_bar_bounds.get());
                                this.drag = Some(Drag::Seek(fraction));
                                cx.stop_propagation();
                                cx.notify();
                            }),
                        )
                    }),
            )
            .child(time_label(duration))
            .child(
                IconButton::new("toggle-mute", volume_icon)
                    .icon_size(IconSize::Small)
                    .tooltip(|_, cx| Tooltip::for_action("Mute", &ToggleMute, cx))
                    .on_click(
                        cx.listener(|this, _, window, cx| {
                            this.toggle_mute(&ToggleMute, window, cx)
                        }),
                    ),
            )
            .child(
                slider(
                    "volume-bar",
                    self.effective_volume(),
                    self.volume_bar_bounds.clone(),
                    cx,
                )
                .w(px(80.0))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, event: &MouseDownEvent, _, cx| {
                        this.drag = Some(Drag::Volume);
                        cx.stop_propagation();
                        this.set_volume(
                            fraction_at(event.position.x, this.volume_bar_bounds.get()),
                            cx,
                        );
                    }),
                ),
            )
    }
}

#[cfg(target_os = "windows")]
fn drop_frame_images(frame: DisplayedFrame, window: &mut Window) {
    use util::ResultExt as _;

    window.drop_image(frame.current).log_err();
    if let Some(previous) = frame.previous {
        window.drop_image(previous).log_err();
    }
}

fn slider(
    id: &'static str,
    fraction: f32,
    bounds: Rc<Cell<Bounds<Pixels>>>,
    cx: &App,
) -> gpui::Stateful<gpui::Div> {
    let colors = cx.theme().colors();
    div()
        .id(id)
        .h(px(16.0))
        .flex()
        .items_center()
        .cursor_pointer()
        .child(
            div()
                .relative()
                .w_full()
                .h(px(4.0))
                .rounded_full()
                .bg(colors.border)
                .child(
                    div()
                        .absolute()
                        .left_0()
                        .top_0()
                        .h_full()
                        .w(relative(fraction))
                        .rounded_full()
                        .bg(colors.text_accent),
                )
                .child(
                    canvas(
                        move |track_bounds, _, _| bounds.set(track_bounds),
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .size_full(),
                ),
        )
}

fn fraction_at(x: Pixels, bounds: Bounds<Pixels>) -> f32 {
    if bounds.size.width <= Pixels::ZERO {
        return 0.0;
    }
    ((x - bounds.origin.x) / bounds.size.width).clamp(0.0, 1.0)
}

fn format_time(time: Duration) -> String {
    let seconds = time.as_secs();
    let (hours, minutes, seconds) = (seconds / 3600, seconds / 60 % 60, seconds % 60);
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

impl EventEmitter<VideoViewerEvent> for VideoViewer {}

impl Focusable for VideoViewer {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for VideoViewer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("VideoViewer")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::toggle_playback))
            .on_action(cx.listener(Self::seek_forward))
            .on_action(cx.listener(Self::seek_backward))
            .on_action(cx.listener(Self::toggle_mute))
            .on_key_down(cx.listener(Self::handle_key_down))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.focus_handle.focus(window, cx)),
            )
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                div()
                    .id("video")
                    .relative()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .bg(gpui::black())
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toggle_playback(&TogglePlayback, window, cx)
                    }))
                    .children(self.render_frame())
                    .children(self.render_status(cx).map(|status| {
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(status)
                    })),
            )
            .child(self.render_controls(cx))
    }
}

impl Item for VideoViewer {
    type Event = VideoViewerEvent;

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        match event {
            VideoViewerEvent::TitleChanged => {
                f(workspace::item::ItemEvent::UpdateTab);
                f(workspace::item::ItemEvent::UpdateBreadcrumbs);
            }
        }
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        f(self.video_item.entity_id(), self.video_item.read(cx))
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        let abs_path = &self.video_item.read(cx).abs_path;
        Some(abs_path.compact().to_string_lossy().into_owned().into())
    }

    fn tab_content(&self, params: TabContentParams, _: &Window, cx: &App) -> AnyElement {
        let project_path = self.video_item.read(cx).project_path(cx);
        let label_color = if ItemSettings::get_global(cx).git_status {
            let project = self.project.read(cx);
            let git_status = project
                .git_store()
                .read(cx)
                .display_status_for_project_path(&project_path, cx)
                .map(|status| status.summary())
                .unwrap_or_default();
            project
                .entry_for_path(&project_path, cx)
                .map(|entry| {
                    entry_git_aware_label_color(git_status, entry.is_ignored, params.selected)
                })
                .unwrap_or_else(|| params.text_color())
        } else {
            params.text_color()
        };

        Label::new(self.tab_content_text(params.detail.unwrap_or_default(), cx))
            .single_line()
            .color(label_color)
            .when(params.preview, |this| this.italic())
            .into_any_element()
    }

    fn tab_content_text(&self, _: usize, cx: &App) -> SharedString {
        self.video_item
            .read(cx)
            .file
            .file_name(cx)
            .to_string()
            .into()
    }

    fn tab_icon(&self, _: &Window, cx: &App) -> Option<Icon> {
        let path = &self.video_item.read(cx).abs_path;
        ItemSettings::get_global(cx)
            .file_icons
            .then(|| FileIcons::get_icon(path, cx))
            .flatten()
            .map(Icon::from_path)
    }

    fn breadcrumb_location(&self, cx: &App) -> ToolbarItemLocation {
        if EditorSettings::get_global(cx).toolbar.breadcrumbs {
            ToolbarItemLocation::PrimaryLeft
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn breadcrumbs(&self, cx: &App) -> Option<(Vec<HighlightedText>, Option<Font>)> {
        let project = self.project.read(cx);
        let video_item = self.video_item.read(cx);
        let mut path = video_item.file.path().to_rel_path_buf();
        if project.visible_worktrees(cx).count() > 1
            && let Some(worktree) =
                project.worktree_for_id(video_item.project_path(cx).worktree_id, cx)
        {
            path = worktree.read(cx).root_name().join(&path);
        }
        let text = path.display(project.path_style(cx)).to_string();
        Some((
            vec![HighlightedText {
                text: text.into(),
                highlights: vec![],
            }],
            Some(ThemeSettings::get_global(cx).buffer_font.clone()),
        ))
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>> {
        let video_item = self.video_item.clone();
        let project = self.project.clone();
        Task::ready(Some(
            cx.new(|cx| Self::new(video_item, project, window, cx)),
        ))
    }

    fn buffer_kind(&self, _: &App) -> workspace::item::ItemBufferKind {
        workspace::item::ItemBufferKind::Singleton
    }
}

impl ProjectItem for VideoViewer {
    type Item = VideoItem;

    fn for_project_item(
        project: Entity<Project>,
        _: Option<&Pane>,
        item: Entity<Self::Item>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(item, project, window, cx)
    }

    fn for_broken_project_item(
        abs_path: &Path,
        is_local: bool,
        error: &anyhow::Error,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InvalidItemView> {
        Some(InvalidItemView::new(abs_path, is_local, error, window, cx))
    }
}

impl SerializableItem for VideoViewer {
    fn serialized_item_kind() -> &'static str {
        "VideoViewer"
    }

    fn deserialize(
        project: Entity<Project>,
        _: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let db = VideoViewerDb::global(cx);
        window.spawn(cx, async move |cx| {
            let video_path = db
                .get_video_path(item_id, workspace_id)?
                .context("No video path found")?;
            let (worktree, relative_path) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(video_path, false, cx)
                })
                .await
                .context("Path not found")?;
            let project_path = ProjectPath {
                worktree_id: worktree.update(cx, |worktree, _| worktree.id()),
                path: relative_path,
            };
            let video_item = cx
                .update(|_, cx| VideoItem::load(&project, &project_path, cx))?
                .await?;
            cx.update(|window, cx| cx.new(|cx| Self::new(video_item, project, window, cx)))
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let db = VideoViewerDb::global(cx);
        delete_unloaded_items(alive_items, workspace_id, "video_viewers", &db, cx)
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        _: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let video_path = self.video_item.read(cx).abs_path.clone();
        let db = VideoViewerDb::global(cx);
        Some(cx.background_spawn(async move {
            db.save_video_path(item_id, workspace_id, video_path).await
        }))
    }

    fn should_serialize(&self, _: &Self::Event) -> bool {
        false
    }
}

pub fn init(cx: &mut App) {
    workspace::register_project_item::<VideoViewer>(cx);
    workspace::register_serializable_item::<VideoViewer>(cx);
}

mod persistence {
    use std::path::PathBuf;

    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    pub struct VideoViewerDb(ThreadSafeConnection);

    impl Domain for VideoViewerDb {
        const NAME: &str = stringify!(VideoViewerDb);

        const MIGRATIONS: &[&str] = &[sql!(
                CREATE TABLE video_viewers (
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,

                    video_path BLOB,

                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
        )];
    }

    db::static_connection!(VideoViewerDb, [WorkspaceDb]);

    impl VideoViewerDb {
        query! {
            pub async fn save_video_path(
                item_id: ItemId,
                workspace_id: WorkspaceId,
                video_path: PathBuf
            ) -> Result<()> {
                INSERT OR REPLACE INTO video_viewers(item_id, workspace_id, video_path)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_video_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
                SELECT video_path
                FROM video_viewers
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use fs::{FakeFs, Fs as _};
    use gpui::{TestAppContext, VisualTestContext};
    use settings::SettingsStore;
    use util::{path, rel_path::rel_path};
    use workspace::item::ItemHandle;
    use worktree::WorktreeId;

    use super::*;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            cx.set_global(db::AppDatabase::test_new());
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            init(cx);
        });
    }

    /// Opens a file named `file_name` in a new workspace window. Its contents are no video: the
    /// platform players cannot run in tests, because AVPlayer needs the process's main thread
    /// and Media Foundation needs COM and a GPU.
    async fn open_in_workspace<'a>(
        file_name: &str,
        cx: &'a mut TestAppContext,
    ) -> (Result<Box<dyn ItemHandle>>, &'a mut VisualTestContext) {
        let fs = FakeFs::new(cx.executor());
        fs.create_dir(Path::new(path!("/root")))
            .await
            .expect("test root should be created");
        fs.insert_file(
            Path::new(path!("/root")).join(file_name),
            b"not a video".to_vec(),
        )
        .await;
        let project = Project::test(fs, [Path::new(path!("/root"))], cx).await;
        let worktree_id = cx.update(|cx| {
            project
                .read(cx)
                .worktrees(cx)
                .next()
                .expect("test project should contain a worktree")
                .read(cx)
                .id()
        });
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let item = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path(file_name)), None, true, window, cx)
            })
            .await;
        cx.run_until_parked();
        (item, cx)
    }

    #[gpui::test]
    async fn test_video_files_open_in_the_video_viewer(cx: &mut TestAppContext) {
        init_test(cx);
        let (item, cx) = open_in_workspace("Clip.MOV", cx).await;
        let video_viewer = item
            .expect("video file should open")
            .downcast::<VideoViewer>()
            .expect("video file should open in the video viewer");

        video_viewer.read_with(cx, |video_viewer, cx| {
            assert_eq!(video_viewer.tab_content_text(0, cx), "Clip.MOV");
        });
    }

    #[gpui::test]
    async fn test_play_waits_until_the_video_is_ready(cx: &mut TestAppContext) {
        init_test(cx);
        let (item, cx) = open_in_workspace("clip.mp4", cx).await;
        let video_viewer = item
            .expect("video file should open")
            .downcast::<VideoViewer>()
            .expect("video file should open in the video viewer");

        video_viewer.update_in(cx, |video_viewer, window, cx| {
            assert_ne!(video_viewer.state, PlayerState::Ready);
            video_viewer.toggle_playback(&TogglePlayback, window, cx);
            assert!(!video_viewer.is_playing());
        });
    }

    #[test]
    fn test_is_video_file() {
        let project_path = |path| ProjectPath {
            worktree_id: WorktreeId::from_usize(0),
            path: rel_path(path).into(),
        };
        assert!(is_video_file(&project_path("clip.mp4")));
        assert!(is_video_file(&project_path("Clip.MOV")));
        assert!(!is_video_file(&project_path("song.mp3")));
        assert_eq!(
            is_video_file(&project_path("film.mkv")),
            cfg!(target_os = "windows")
        );
    }

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(Duration::from_secs(0)), "0:00");
        assert_eq!(format_time(Duration::from_millis(65_900)), "1:05");
        assert_eq!(format_time(Duration::from_secs(3_723)), "1:02:03");
    }

    #[test]
    fn test_fraction_at_clamps_to_the_bar() {
        let bounds = Bounds::new(
            gpui::point(px(100.0), px(0.0)),
            gpui::size(px(200.0), px(4.0)),
        );
        assert_eq!(fraction_at(px(150.0), bounds), 0.25);
        assert_eq!(fraction_at(px(50.0), bounds), 0.0);
        assert_eq!(fraction_at(px(400.0), bounds), 1.0);
        assert_eq!(fraction_at(px(150.0), Bounds::default()), 0.0);
    }
}
