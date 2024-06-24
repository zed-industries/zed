use crate::track::RemoteVideoTrack;
use anyhow::Result;
use futures::StreamExt as _;
use gpui::{
    div, img, EventEmitter, IntoElement, ParentElement, Render, ScreenCaptureFrame, Styled, Task,
    View, ViewContext, VisualContext as _,
};

pub struct RemoteVideoTrackView {
    track: RemoteVideoTrack,
    frame: Option<ScreenCaptureFrame>,
    _maintain_frame: Task<Result<()>>,
}

pub enum RemoteVideoTrackViewEvent {
    Close,
}

impl RemoteVideoTrackView {
    pub fn new(track: RemoteVideoTrack, cx: &mut ViewContext<Self>) -> Self {
        cx.focus_handle();
        let frames = super::play_remote_video_track(&track);

        Self {
            track,
            frame: None,
            _maintain_frame: cx.spawn(|this, mut cx| async move {
                futures::pin_mut!(frames);
                while let Some(frame) = frames.next().await {
                    this.update(&mut cx, |this, cx| {
                        this.frame = Some(frame);
                        cx.notify();
                    })?;
                }
                this.update(&mut cx, |_, cx| cx.emit(RemoteVideoTrackViewEvent::Close))?;
                Ok(())
            }),
        }
    }

    pub fn clone(&self, cx: &mut ViewContext<Self>) -> View<Self> {
        cx.new_view(|cx| Self::new(self.track.clone(), cx))
    }
}

impl EventEmitter<RemoteVideoTrackViewEvent> for RemoteVideoTrackView {}

impl Render for RemoteVideoTrackView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .children(self.frame.as_ref().map(|frame| img(frame.0.clone())))
    }
}
