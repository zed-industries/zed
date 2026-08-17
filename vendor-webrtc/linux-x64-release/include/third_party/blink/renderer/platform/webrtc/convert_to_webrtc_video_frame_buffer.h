// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_CONVERT_TO_WEBRTC_VIDEO_FRAME_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_CONVERT_TO_WEBRTC_VIDEO_FRAME_BUFFER_H_

#include "base/containers/span.h"
#include "base/memory/scoped_refptr.h"
#include "media/base/video_frame.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/webrtc/webrtc_video_frame_adapter.h"
#include "third_party/webrtc/api/video/video_frame_buffer.h"

namespace blink {

PLATFORM_EXPORT bool CanConvertToWebRtcVideoFrameBuffer(
    const media::VideoFrame* frame);

PLATFORM_EXPORT base::span<const media::VideoPixelFormat>
GetPixelFormatsMappableToWebRtcVideoFrameBuffer();

PLATFORM_EXPORT webrtc::scoped_refptr<webrtc::VideoFrameBuffer>
ConvertToWebRtcVideoFrameBuffer(
    scoped_refptr<media::VideoFrame> video_frame,
    scoped_refptr<WebRtcVideoFrameAdapter::SharedResources> shared_resources);

PLATFORM_EXPORT scoped_refptr<media::VideoFrame>
ConvertFromMappedWebRtcVideoFrameBuffer(
    webrtc::scoped_refptr<webrtc::VideoFrameBuffer> buffer,
    base::TimeDelta timestamp);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_CONVERT_TO_WEBRTC_VIDEO_FRAME_BUFFER_H_
