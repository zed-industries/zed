mod playback;

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
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Render, Subscription, Task,
    WeakEntity, Window, actions, canvas, div, px, relative,
};
use language::{DiskState, File as _};
use persistence::AudioViewerDb;
use project::{Project, ProjectEntryId, ProjectPath, git_store::GitStoreEvent};
use settings::Settings as _;
use theme_settings::ThemeSettings;
use ui::{Tooltip, prelude::*};
use util::{ResultExt as _, paths::PathExt as _, size::format_file_size};
use workspace::{
    ItemId, ItemSettings, Pane, ToolbarItemLocation, Workspace, WorkspaceId, delete_unloaded_items,
    invalid_item_view::InvalidItemView,
    item::{HighlightedText, Item, ProjectItem, SerializableItem, TabContentParams},
};
use worktree::{File, Worktree};

use crate::playback::{Playback, TrackDecoder, TrackInfo, open_track};

actions!(
    audio_viewer,
    [
        /// Play or pause the audio.
        TogglePlayback,
        /// Skip forward five seconds.
        SeekForward,
        /// Skip back five seconds.
        SeekBackward,
        /// Mute or unmute the audio.
        ToggleMute,
    ]
);

/// Extensions of formats the symphonia decoders enabled for rodio can play. Opus is left out on
/// purpose: symphonia has no production-ready Opus decoder.
const AUDIO_EXTENSIONS: &[&str] = &[
    "mp3", "wav", "wave", "flac", "ogg", "oga", "m4a", "aac", "aif", "aiff", "aifc", "caf",
];
const SEEK_STEP: Duration = Duration::from_secs(5);
const POSITION_REFRESH_INTERVAL: Duration = Duration::from_millis(100);
const VOLUME_STEP: f32 = 0.1;

pub fn is_audio_file(path: &ProjectPath) -> bool {
    path.path.extension().is_some_and(|extension| {
        AUDIO_EXTENSIONS
            .iter()
            .any(|audio_extension| extension.eq_ignore_ascii_case(audio_extension))
    })
}

pub struct AudioItem {
    file: Arc<File>,
    abs_path: PathBuf,
}

impl AudioItem {
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

    fn file_size(&self) -> Option<u64> {
        match self.file.disk_state {
            DiskState::Present { size, .. } => Some(size),
            _ => None,
        }
    }
}

impl project::ProjectItem for AudioItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        if !is_audio_file(path) {
            return None;
        }
        // Playback reads the file from disk, which remote worktrees cannot provide.
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

enum Track {
    Loading,
    Loaded {
        info: TrackInfo,
        /// The decoder opened to read the track info, kept for the first playback.
        decoder: Option<TrackDecoder>,
    },
    Failed(SharedString),
}

#[derive(Clone, Copy, PartialEq)]
enum Drag {
    Seek(f32),
    Volume,
}

pub struct AudioViewer {
    audio_item: Entity<AudioItem>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    track: Track,
    playback: Option<Playback>,
    playback_error: Option<SharedString>,
    finished: bool,
    position: Duration,
    volume: f32,
    muted: bool,
    drag: Option<Drag>,
    seek_bar_bounds: Rc<Cell<Bounds<Pixels>>>,
    volume_bar_bounds: Rc<Cell<Bounds<Pixels>>>,
    _load_track: Task<()>,
    _refresh_position: Task<()>,
    _subscriptions: Vec<Subscription>,
}

pub enum AudioViewerEvent {
    TitleChanged,
}

impl AudioViewer {
    pub fn new(
        audio_item: Entity<AudioItem>,
        project: Entity<Project>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let path = audio_item.read(cx).abs_path.clone();
        let load_track = cx.spawn(async move |this, cx| {
            let result = cx.background_spawn(async move { open_track(&path) }).await;
            this.update(cx, |this, cx| {
                this.track = match result {
                    Ok((decoder, info)) => Track::Loaded {
                        info,
                        decoder: Some(decoder),
                    },
                    Err(error) => Track::Failed(format!("{error:#}").into()),
                };
                cx.notify();
            })
            .ok();
        });
        let git_store = project.read(cx).git_store().clone();
        let subscriptions = vec![cx.subscribe(&git_store, |_, _, event, cx| {
            if matches!(event, GitStoreEvent::DiffBaseChanged(_)) {
                cx.emit(AudioViewerEvent::TitleChanged);
            }
        })];

        Self {
            audio_item,
            project,
            focus_handle: cx.focus_handle(),
            track: Track::Loading,
            playback: None,
            playback_error: None,
            finished: false,
            position: Duration::ZERO,
            volume: 1.0,
            muted: false,
            drag: None,
            seek_bar_bounds: Rc::default(),
            volume_bar_bounds: Rc::default(),
            _load_track: load_track,
            _refresh_position: Task::ready(()),
            _subscriptions: subscriptions,
        }
    }

    fn duration(&self) -> Option<Duration> {
        match &self.track {
            Track::Loaded { info, .. } => info.duration,
            _ => None,
        }
    }

    fn is_playing(&self) -> bool {
        self.playback
            .as_ref()
            .is_some_and(|playback| !playback.is_paused() && !self.finished)
    }

    fn effective_volume(&self) -> f32 {
        if self.muted { 0.0 } else { self.volume }
    }

    fn toggle_playback(&mut self, _: &TogglePlayback, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_playing() {
            if let Some(playback) = &self.playback {
                playback.pause();
            }
            self._refresh_position = Task::ready(());
            cx.notify();
        } else {
            self.play(cx);
        }
    }

    fn play(&mut self, cx: &mut Context<Self>) {
        let Track::Loaded { decoder, .. } = &mut self.track else {
            return;
        };
        let start_at = self.position;
        match (&self.playback, decoder.take()) {
            (Some(playback), _) if !self.finished => {
                playback.play();
                self.start_position_refresh(cx);
            }
            (None, Some(decoder)) => self.start_playback(decoder, start_at, cx),
            _ => {
                // The decoder was consumed by an earlier playback that reached the end.
                let path = self.audio_item.read(cx).abs_path.clone();
                self._load_track = cx.spawn(async move |this, cx| {
                    let result = cx.background_spawn(async move { open_track(&path) }).await;
                    this.update(cx, |this, cx| match result {
                        Ok((decoder, _)) => {
                            let start_at = if this.finished {
                                Duration::ZERO
                            } else {
                                this.position
                            };
                            this.start_playback(decoder, start_at, cx);
                        }
                        Err(error) => {
                            this.playback_error = Some(format!("{error:#}").into());
                            cx.notify();
                        }
                    })
                    .ok();
                });
            }
        }
    }

    fn start_playback(
        &mut self,
        decoder: TrackDecoder,
        start_at: Duration,
        cx: &mut Context<Self>,
    ) {
        let result = match &self.playback {
            Some(playback) => {
                playback.restart(decoder);
                Ok(())
            }
            None => Playback::start(decoder, self.effective_volume()).map(|playback| {
                self.playback = Some(playback);
            }),
        };
        match result {
            Ok(()) => {
                self.finished = false;
                self.playback_error = None;
                if let Some(playback) = &self.playback
                    && start_at > Duration::ZERO
                {
                    playback.seek(start_at).log_err();
                }
                self.position = start_at;
                self.start_position_refresh(cx);
            }
            Err(error) => self.playback_error = Some(format!("{error:#}").into()),
        }
        cx.notify();
    }

    fn start_position_refresh(&mut self, cx: &mut Context<Self>) {
        self._refresh_position = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(POSITION_REFRESH_INTERVAL)
                    .await;
                let keep_refreshing = this
                    .update(cx, |this, cx| {
                        let Some(playback) = &this.playback else {
                            return false;
                        };
                        if playback.is_finished() {
                            this.finished = true;
                            this.position = this.duration().unwrap_or(this.position);
                        } else if !matches!(this.drag, Some(Drag::Seek(_))) {
                            this.position = playback.position();
                        }
                        cx.notify();
                        !this.finished && !playback.is_paused()
                    })
                    .unwrap_or(false);
                if !keep_refreshing {
                    break;
                }
            }
        });
    }

    fn seek_to(&mut self, position: Duration, cx: &mut Context<Self>) {
        let position = match self.duration() {
            Some(duration) => position.min(duration),
            None => position,
        };
        self.position = position;
        if self.finished {
            // Playing again restarts the track from the chosen position.
            self.finished = false;
            self.playback = None;
            self._refresh_position = Task::ready(());
        } else if let Some(playback) = &self.playback {
            playback.seek(position).log_err();
        }
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
        if let Some(playback) = &self.playback {
            playback.set_volume(self.effective_volume());
        }
        cx.notify();
    }

    fn handle_key_volume(&mut self, up: bool, cx: &mut Context<Self>) {
        let step = if up { VOLUME_STEP } else { -VOLUME_STEP };
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
        let can_play = matches!(self.track, Track::Loaded { .. });
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
            .max_w(px(640.0))
            .gap_2()
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
                        this.set_volume(
                            fraction_at(event.position.x, this.volume_bar_bounds.get()),
                            cx,
                        );
                    }),
                ),
            )
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

fn channel_description(channels: u16) -> String {
    match channels {
        1 => "Mono".to_string(),
        2 => "Stereo".to_string(),
        channels => format!("{channels} channels"),
    }
}

impl EventEmitter<AudioViewerEvent> for AudioViewer {}

impl Focusable for AudioViewer {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AudioViewer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let audio_item = self.audio_item.read(cx);
        let file_name = audio_item.file.file_name(cx).to_string();
        let extension = audio_item
            .abs_path
            .extension()
            .map(|extension| extension.to_string_lossy().to_uppercase());
        let mut details: Vec<String> = extension.into_iter().collect();
        if let Track::Loaded { info, .. } = &self.track {
            details.push(format!("{:.1} kHz", info.sample_rate as f32 / 1000.0));
            details.push(channel_description(info.channels));
        }
        if let Some(size) = audio_item.file_size() {
            details.push(format_file_size(size, false));
        }
        let error = match &self.track {
            Track::Failed(error) => Some(error.clone()),
            Track::Loaded { .. } => self.playback_error.clone(),
            Track::Loading => None,
        };
        let abs_path = audio_item.abs_path.clone();
        let status = match error {
            Some(error) => Some(
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
            ),
            None if matches!(self.track, Track::Loading) => Some(
                Label::new("Loading…")
                    .color(Color::Muted)
                    .into_any_element(),
            ),
            None => None,
        };

        v_flex()
            .key_context("AudioViewer")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::toggle_playback))
            .on_action(cx.listener(Self::seek_forward))
            .on_action(cx.listener(Self::seek_backward))
            .on_action(cx.listener(Self::toggle_mute))
            .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, _, cx| {
                let modifiers = &event.keystroke.modifiers;
                if modifiers.control || modifiers.platform || modifiers.alt || modifiers.shift {
                    return;
                }
                match event.keystroke.key.as_str() {
                    "up" => this.handle_key_volume(true, cx),
                    "down" => this.handle_key_volume(false, cx),
                    _ => return,
                }
                cx.stop_propagation();
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.focus_handle.focus(window, cx)),
            )
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .size_full()
            .items_center()
            .justify_center()
            .gap_3()
            .p_8()
            .bg(cx.theme().colors().editor_background)
            .child(
                div()
                    .p_4()
                    .rounded_lg()
                    .bg(cx.theme().colors().surface_background)
                    .child(
                        Icon::new(IconName::AudioOn)
                            .size(IconSize::Custom(rems(3.0)))
                            .color(Color::Muted),
                    ),
            )
            .child(Label::new(file_name).size(LabelSize::Large))
            .child(
                Label::new(details.join(" · "))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(self.render_controls(cx))
            .children(status)
    }
}

impl Item for AudioViewer {
    type Event = AudioViewerEvent;

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        match event {
            AudioViewerEvent::TitleChanged => {
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
        f(self.audio_item.entity_id(), self.audio_item.read(cx))
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        let abs_path = &self.audio_item.read(cx).abs_path;
        Some(abs_path.compact().to_string_lossy().into_owned().into())
    }

    fn tab_content(&self, params: TabContentParams, _: &Window, cx: &App) -> AnyElement {
        let project_path = self.audio_item.read(cx).project_path(cx);
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
        self.audio_item
            .read(cx)
            .file
            .file_name(cx)
            .to_string()
            .into()
    }

    fn tab_icon(&self, _: &Window, cx: &App) -> Option<Icon> {
        let path = &self.audio_item.read(cx).abs_path;
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
        let audio_item = self.audio_item.read(cx);
        let mut path = audio_item.file.path().to_rel_path_buf();
        if project.visible_worktrees(cx).count() > 1
            && let Some(worktree) =
                project.worktree_for_id(audio_item.project_path(cx).worktree_id, cx)
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
        let audio_item = self.audio_item.clone();
        let project = self.project.clone();
        Task::ready(Some(
            cx.new(|cx| Self::new(audio_item, project, window, cx)),
        ))
    }

    fn buffer_kind(&self, _: &App) -> workspace::item::ItemBufferKind {
        workspace::item::ItemBufferKind::Singleton
    }
}

impl ProjectItem for AudioViewer {
    type Item = AudioItem;

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

impl SerializableItem for AudioViewer {
    fn serialized_item_kind() -> &'static str {
        "AudioViewer"
    }

    fn deserialize(
        project: Entity<Project>,
        _: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let db = AudioViewerDb::global(cx);
        window.spawn(cx, async move |cx| {
            let audio_path = db
                .get_audio_path(item_id, workspace_id)?
                .context("No audio path found")?;
            let (worktree, relative_path) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(audio_path, false, cx)
                })
                .await
                .context("Path not found")?;
            let project_path = ProjectPath {
                worktree_id: worktree.update(cx, |worktree, _| worktree.id()),
                path: relative_path,
            };
            let audio_item = cx
                .update(|_, cx| AudioItem::load(&project, &project_path, cx))?
                .await?;
            cx.update(|window, cx| cx.new(|cx| Self::new(audio_item, project, window, cx)))
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let db = AudioViewerDb::global(cx);
        delete_unloaded_items(alive_items, workspace_id, "audio_viewers", &db, cx)
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        _: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let audio_path = self.audio_item.read(cx).abs_path.clone();
        let db = AudioViewerDb::global(cx);
        Some(cx.background_spawn(async move {
            db.save_audio_path(item_id, workspace_id, audio_path).await
        }))
    }

    fn should_serialize(&self, _: &Self::Event) -> bool {
        false
    }
}

pub fn init(cx: &mut App) {
    workspace::register_project_item::<AudioViewer>(cx);
    workspace::register_serializable_item::<AudioViewer>(cx);
}

mod persistence {
    use std::path::PathBuf;

    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    pub struct AudioViewerDb(ThreadSafeConnection);

    impl Domain for AudioViewerDb {
        const NAME: &str = stringify!(AudioViewerDb);

        const MIGRATIONS: &[&str] = &[sql!(
                CREATE TABLE audio_viewers (
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,

                    audio_path BLOB,

                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
        )];
    }

    db::static_connection!(AudioViewerDb, [WorkspaceDb]);

    impl AudioViewerDb {
        query! {
            pub async fn save_audio_path(
                item_id: ItemId,
                workspace_id: WorkspaceId,
                audio_path: PathBuf
            ) -> Result<()> {
                INSERT OR REPLACE INTO audio_viewers(item_id, workspace_id, audio_path)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_audio_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
                SELECT audio_path
                FROM audio_viewers
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use fs::RealFs;
    use gpui::{TestAppContext, VisualTestContext};
    use settings::SettingsStore;
    use util::rel_path::rel_path;
    use workspace::item::ItemHandle;
    use worktree::WorktreeId;

    use super::*;
    use crate::playback::tests::write_wav;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            cx.set_global(db::AppDatabase::test_new());
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            init(cx);
        });
    }

    /// Opens `file_name` from `directory` in a new workspace window. The player decodes files
    /// from disk, so the project uses the real file system.
    async fn open_in_workspace<'a>(
        directory: &Path,
        file_name: &str,
        cx: &'a mut TestAppContext,
    ) -> (Result<Box<dyn ItemHandle>>, &'a mut VisualTestContext) {
        cx.executor().allow_parking();
        let project = Project::test(RealFs::new(None, cx.executor()), [directory], cx).await;
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
    async fn test_audio_files_open_in_the_audio_viewer(cx: &mut TestAppContext) {
        init_test(cx);
        let directory = tempfile::tempdir().expect("temp dir should be created");
        write_wav(&directory.path().join("Song.WAV"), 8000, 16000);

        let (item, cx) = open_in_workspace(directory.path(), "Song.WAV", cx).await;
        let audio_viewer = item
            .expect("audio file should open")
            .downcast::<AudioViewer>()
            .expect("audio file should open in the audio viewer");

        audio_viewer.read_with(cx, |audio_viewer, cx| {
            assert_eq!(audio_viewer.tab_content_text(0, cx), "Song.WAV");
            let Track::Loaded { info, .. } = &audio_viewer.track else {
                panic!("the track should be loaded");
            };
            assert_eq!(
                *info,
                TrackInfo {
                    duration: Some(Duration::from_secs(2)),
                    sample_rate: 8000,
                    channels: 1,
                }
            );
        });
    }

    #[gpui::test]
    async fn test_undecodable_audio_file_shows_the_error(cx: &mut TestAppContext) {
        init_test(cx);
        let directory = tempfile::tempdir().expect("temp dir should be created");
        std::fs::write(directory.path().join("broken.mp3"), b"this is not audio")
            .expect("test file should be written");

        let (item, cx) = open_in_workspace(directory.path(), "broken.mp3", cx).await;
        let audio_viewer = item
            .expect("audio file should open")
            .downcast::<AudioViewer>()
            .expect("audio file should open in the audio viewer");

        audio_viewer.read_with(cx, |audio_viewer, _| {
            let Track::Failed(error) = &audio_viewer.track else {
                panic!("decoding should fail");
            };
            assert!(
                error.starts_with("This audio file cannot be decoded"),
                "unexpected error: {error}"
            );
        });
    }

    #[gpui::test]
    async fn test_seeking_stays_within_the_track(cx: &mut TestAppContext) {
        init_test(cx);
        let directory = tempfile::tempdir().expect("temp dir should be created");
        write_wav(&directory.path().join("song.wav"), 8000, 16000);

        let (item, cx) = open_in_workspace(directory.path(), "song.wav", cx).await;
        let audio_viewer = item
            .expect("audio file should open")
            .downcast::<AudioViewer>()
            .expect("audio file should open in the audio viewer");

        // Seeking before the first playback needs no audio output device, which CI machines lack.
        audio_viewer.update_in(cx, |audio_viewer, window, cx| {
            audio_viewer.seek_forward(&SeekForward, window, cx);
            assert_eq!(audio_viewer.position, Duration::from_secs(2));
            audio_viewer.seek_backward(&SeekBackward, window, cx);
            assert_eq!(audio_viewer.position, Duration::ZERO);
        });
    }

    #[test]
    fn test_is_audio_file() {
        let project_path = |path| ProjectPath {
            worktree_id: WorktreeId::from_usize(0),
            path: rel_path(path).into(),
        };
        assert!(is_audio_file(&project_path("song.mp3")));
        assert!(is_audio_file(&project_path("Song.FLAC")));
        assert!(!is_audio_file(&project_path("voice.opus")));
        assert!(!is_audio_file(&project_path("notes.txt")));
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
