// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_WEBRTC_VIDEO_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_WEBRTC_VIDEO_UTILS_H_

#include "media/base/video_codecs.h"
#include "media/base/video_color_space.h"
#include "media/base/video_transformation.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/api/video/color_space.h"
#include "third_party/webrtc/api/video/video_codec_type.h"
#include "third_party/webrtc/api/video/video_rotation.h"
#include "third_party/webrtc/api/video_codecs/sdp_video_format.h"

namespace blink {

// This file has helper methods for conversion between chromium types and
// webrtc/api/video types.

media::VideoRotation PLATFORM_EXPORT
WebRtcToMediaVideoRotation(webrtc::VideoRotation rotation);

media::VideoCodec PLATFORM_EXPORT
WebRtcToMediaVideoCodec(webrtc::VideoCodecType codec);

// Map webrtc::SdpVideoFormat to the same or closest media::VideoCodecProfile.
media::VideoCodecProfile PLATFORM_EXPORT
WebRtcVideoFormatToMediaVideoCodecProfile(const webrtc::SdpVideoFormat& format);

gfx::ColorSpace PLATFORM_EXPORT
WebRtcToGfxColorSpace(const webrtc::ColorSpace& color_space);

webrtc::ColorSpace PLATFORM_EXPORT
GfxToWebRtcColorSpace(const gfx::ColorSpace& color_space);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_WEBRTC_VIDEO_UTILS_H_
