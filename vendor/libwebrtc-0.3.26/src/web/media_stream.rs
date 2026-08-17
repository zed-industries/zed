// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::VideoTrack;
use crate::{
    media_stream::{
        BiplanarYuv8Buffer, BiplanarYuvBuffer, PlanarYuv16BBuffer, PlanarYuv8Buffer,
        PlanarYuvBuffer, VideoFrameBuffer,
    },
    video_frame::{BiplanarYuv8Buffer, I420Buffer, SinkError, VideoFrame, VideoFrameBuffer},
    I010Buffer, I420ABuffer, I422Buffer, I444Buffer, NV12Buffer,
};
use std::sync::mpsc;
use web_sys::{WebGlRenderingContext, WebGlTexture};

#[derive(Debug)]
pub struct WebGlVideoSink {
    track: Arc<VideoTrack>,
    gl_ctx: WebGlRenderingContext,
    tex: WebGlTexture,
}

/// Create a new WebGL texture and update it inside requestAnimationFrame
impl WebGlVideoSink {
    pub fn new(
        track: Arc<VideoTrack>,
        gl_ctx: WebGlRenderingContext,
    ) -> Result<(Self, mpsc::Receiver<VideoFrame<WebGlBuffer>>), SinkError> {
        let (sender, receiver) = mpsc::channel();
        let tex = gl_ctx.create_texture()?;
        Ok((Self { track, gl_ctx, tex }, receiver))
    }
}

#[derive(Debug, Clone)]
pub struct WebGlBuffer {
    width: i32,
    height: i32,
    tex: WebGlTexture,
}

impl VideoFrameBuffer for WebGlBuffer {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }
}

/// The following types could be implemented if we want
/// to support VideoFrame with WebCodecs

#[derive(Debug)]
pub struct I420Buffer {}

#[derive(Debug)]
pub struct I420ABuffer {}

#[derive(Debug)]
pub struct I422Buffer {}

#[derive(Debug)]
pub struct I444Buffer {}

#[derive(Debug)]
pub struct I010Buffer {}

#[derive(Debug)]
pub struct NV12Buffer {}

impl VideoFrameBuffer for I420Buffer {
    fn width(&self) -> i32 {
        unimplemented!()
    }

    fn height(&self) -> i32 {
        unimplemented!()
    }
}

impl VideoFrameBuffer for I420ABuffer {
    fn width(&self) -> i32 {
        unimplemented!()
    }

    fn height(&self) -> i32 {
        unimplemented!()
    }
}

impl VideoFrameBuffer for I422Buffer {
    fn width(&self) -> i32 {
        unimplemented!()
    }

    fn height(&self) -> i32 {
        unimplemented!()
    }
}

impl VideoFrameBuffer for I444Buffer {
    fn width(&self) -> i32 {
        unimplemented!()
    }

    fn height(&self) -> i32 {
        unimplemented!()
    }
}

impl VideoFrameBuffer for I010Buffer {
    fn width(&self) -> i32 {
        unimplemented!()
    }

    fn height(&self) -> i32 {
        unimplemented!()
    }
}

impl VideoFrameBuffer for NV12Buffer {
    fn width(&self) -> i32 {
        unimplemented!()
    }

    fn height(&self) -> i32 {
        unimplemented!()
    }
}

impl PlanarYuvBuffer for I420Buffer {
    fn chroma_width(&self) -> i32 {
        unimplemented!()
    }

    fn chroma_height(&self) -> i32 {
        unimplemented!()
    }

    fn stride_y(&self) -> i32 {
        unimplemented!()
    }

    fn stride_u(&self) -> i32 {
        unimplemented!()
    }

    fn stride_v(&self) -> i32 {
        unimplemented!()
    }
}

impl PlanarYuvBuffer for I420ABuffer {
    fn chroma_width(&self) -> i32 {
        unimplemented!()
    }

    fn chroma_height(&self) -> i32 {
        unimplemented!()
    }

    fn stride_y(&self) -> i32 {
        unimplemented!()
    }

    fn stride_u(&self) -> i32 {
        unimplemented!()
    }

    fn stride_v(&self) -> i32 {
        unimplemented!()
    }
}

impl PlanarYuvBuffer for I422Buffer {
    fn chroma_width(&self) -> i32 {
        unimplemented!()
    }

    fn chroma_height(&self) -> i32 {
        unimplemented!()
    }

    fn stride_y(&self) -> i32 {
        unimplemented!()
    }

    fn stride_u(&self) -> i32 {
        unimplemented!()
    }

    fn stride_v(&self) -> i32 {
        unimplemented!()
    }
}

impl PlanarYuvBuffer for I444Buffer {
    fn chroma_width(&self) -> i32 {
        unimplemented!()
    }

    fn chroma_height(&self) -> i32 {
        unimplemented!()
    }

    fn stride_y(&self) -> i32 {
        unimplemented!()
    }

    fn stride_u(&self) -> i32 {
        unimplemented!()
    }

    fn stride_v(&self) -> i32 {
        unimplemented!()
    }
}

impl PlanarYuvBuffer for I010Buffer {
    fn chroma_width(&self) -> i32 {
        unimplemented!()
    }

    fn chroma_height(&self) -> i32 {
        unimplemented!()
    }

    fn stride_y(&self) -> i32 {
        unimplemented!()
    }

    fn stride_u(&self) -> i32 {
        unimplemented!()
    }

    fn stride_v(&self) -> i32 {
        unimplemented!()
    }
}

impl PlanarYuvBuffer for NV12Buffer {
    fn chroma_width(&self) -> i32 {
        unimplemented!()
    }

    fn chroma_height(&self) -> i32 {
        unimplemented!()
    }

    fn stride_y(&self) -> i32 {
        unimplemented!()
    }

    fn stride_u(&self) -> i32 {
        unimplemented!()
    }

    fn stride_v(&self) -> i32 {
        unimplemented!()
    }
}

impl PlanarYuv8Buffer for I420Buffer {
    fn data_y(&self) -> &[u8] {
        unimplemented!()
    }

    fn data_u(&self) -> &[u8] {
        unimplemented!()
    }

    fn data_v(&self) -> &[u8] {
        unimplemented!()
    }
}

impl PlanarYuv8Buffer for I420ABuffer {
    fn data_y(&self) -> &[u8] {
        unimplemented!()
    }

    fn data_u(&self) -> &[u8] {
        unimplemented!()
    }

    fn data_v(&self) -> &[u8] {
        unimplemented!()
    }
}

impl PlanarYuv8Buffer for I422Buffer {
    fn data_y(&self) -> &[u8] {
        unimplemented!()
    }

    fn data_u(&self) -> &[u8] {
        unimplemented!()
    }

    fn data_v(&self) -> &[u8] {
        unimplemented!()
    }
}

impl PlanarYuv8Buffer for I444Buffer {
    fn data_y(&self) -> &[u8] {
        unimplemented!()
    }

    fn data_u(&self) -> &[u8] {
        unimplemented!()
    }

    fn data_v(&self) -> &[u8] {
        unimplemented!()
    }
}

impl PlanarYuv16BBuffer for I010Buffer {
    fn data_y(&self) -> &[u16] {
        unimplemented!()
    }

    fn data_u(&self) -> &[u16] {
        unimplemented!()
    }

    fn data_v(&self) -> &[u16] {
        unimplemented!()
    }
}

impl BiplanarYuvBuffer for NV12Buffer {
    fn chroma_width(&self) -> i32 {
        unimplemented!()
    }

    fn chroma_height(&self) -> i32 {
        unimplemented!()
    }

    fn stride_y(&self) -> i32 {
        unimplemented!()
    }

    fn stride_uv(&self) -> i32 {
        unimplemented!()
    }
}

impl BiplanarYuv8Buffer for NV12Buffer {
    fn data_y(&self) -> &[u8] {
        unimplemented!()
    }

    fn data_uv(&self) -> &[u8] {
        unimplemented!()
    }
}
