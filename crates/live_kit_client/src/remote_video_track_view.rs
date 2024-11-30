use std::sync::Arc;

use crate::track::RemoteVideoTrack;
use anyhow::Result;
use futures::StreamExt as _;
use gpui::{
    img, Empty, EventEmitter, IntoElement, Render, RenderImage, ScreenCaptureFrame, Task, View,
    ViewContext, VisualContext as _,
};
use image::Frame;
use smallvec::SmallVec;

pub struct RemoteVideoTrackView {
    track: RemoteVideoTrack,
    frame: Option<ScreenCaptureFrame>,
    _maintain_frame: Task<Result<()>>,
}

#[derive(Debug)]
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
        #[cfg(target_os = "macos")]
        if let Some(frame) = &self.frame {
            use gpui::Styled as _;
            return gpui::surface(frame.0.clone())
                .size_full()
                .into_any_element();
        }

        #[cfg(not(target_os = "macos"))]
        if let Some(frame) = &self.frame {
            return img(frame.0.clone()).into_any_element();
        }

        Empty.into_any_element()
    }
}
