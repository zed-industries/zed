use crate::track::RemoteVideoTrack;
use anyhow::Result;
use futures::StreamExt as _;
use gpui::{
    Context as _, Empty, EventEmitter, IntoElement, Model, ModelContext, Render, Task,
    VisualContext as _, Window,
};

pub struct RemoteVideoTrackView {
    track: RemoteVideoTrack,
    latest_frame: Option<crate::RemoteVideoFrame>,
    #[cfg(not(target_os = "macos"))]
    current_rendered_frame: Option<crate::RemoteVideoFrame>,
    #[cfg(not(target_os = "macos"))]
    previous_rendered_frame: Option<crate::RemoteVideoFrame>,
    _maintain_frame: Task<Result<()>>,
}

#[derive(Debug)]
pub enum RemoteVideoTrackViewEvent {
    Close,
}

impl RemoteVideoTrackView {
    pub fn new(track: RemoteVideoTrack, window: &mut Window, cx: &mut ModelContext<Self>) -> Self {
        cx.focus_handle();
        let frames = super::play_remote_video_track(&track);

        Self {
            track,
            latest_frame: None,
            _maintain_frame: cx.spawn_in(window, |this, mut cx| async move {
                futures::pin_mut!(frames);
                while let Some(frame) = frames.next().await {
                    this.update(&mut cx, |this, cx| {
                        this.latest_frame = Some(frame);
                        cx.notify();
                    })?;
                }
                this.update(&mut cx, |_this, cx| {
                    #[cfg(not(target_os = "macos"))]
                    {
                        use util::ResultExt as _;
                        if let Some(frame) = _this.previous_rendered_frame.take() {
                            cx.window_context().drop_image(frame).log_err();
                        }
                        // TODO(mgsloan): This might leak the last image of the screenshare if
                        // render is called after the screenshare ends.
                        if let Some(frame) = _this.current_rendered_frame.take() {
                            cx.window_context().drop_image(frame).log_err();
                        }
                    }
                    cx.emit(RemoteVideoTrackViewEvent::Close)
                })?;
                Ok(())
            }),
            #[cfg(not(target_os = "macos"))]
            current_rendered_frame: None,
            #[cfg(not(target_os = "macos"))]
            previous_rendered_frame: None,
        }
    }

    pub fn clone(&self, window: &mut Window, cx: &mut ModelContext<Self>) -> Model<Self> {
        cx.new_model(|cx| Self::new(self.track.clone(), window, cx))
    }
}

impl EventEmitter<RemoteVideoTrackViewEvent> for RemoteVideoTrackView {}

impl Render for RemoteVideoTrackView {
    fn render(&mut self, _window: &mut Window, _cx: &mut ModelContext<Self>) -> impl IntoElement {
        #[cfg(target_os = "macos")]
        if let Some(latest_frame) = &self.latest_frame {
            use gpui::Styled as _;
            return gpui::surface(latest_frame.clone())
                .size_full()
                .into_any_element();
        }

        #[cfg(not(target_os = "macos"))]
        if let Some(latest_frame) = &self.latest_frame {
            use gpui::Styled as _;
            if let Some(current_rendered_frame) = self.current_rendered_frame.take() {
                if let Some(frame) = self.previous_rendered_frame.take() {
                    // Only drop the frame if it's not also the current frame.
                    if frame.id != current_rendered_frame.id {
                        use util::ResultExt as _;
                        _cx.window_context().drop_image(frame).log_err();
                    }
                }
                self.previous_rendered_frame = Some(current_rendered_frame)
            }
            self.current_rendered_frame = Some(latest_frame.clone());
            return gpui::img(latest_frame.clone())
                .size_full()
                .into_any_element();
        }

        Empty.into_any_element()
    }
}
