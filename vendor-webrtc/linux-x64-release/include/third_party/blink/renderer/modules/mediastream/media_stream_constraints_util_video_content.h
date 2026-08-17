// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_CONSTRAINTS_UTIL_VIDEO_CONTENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_CONSTRAINTS_UTIL_VIDEO_CONTENT_H_

#include "third_party/blink/public/common/mediastream/media_stream_request.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class MediaConstraints;
class VideoCaptureSettings;

MODULES_EXPORT extern const int kMinScreenCastDimension;
MODULES_EXPORT extern const int kMaxScreenCastDimension;
MODULES_EXPORT extern const int kDefaultScreenCastWidth;
MODULES_EXPORT extern const int kDefaultScreenCastHeight;

MODULES_EXPORT extern const double kMaxScreenCastFrameRate;
MODULES_EXPORT extern const double kDefaultScreenCastFrameRate;

// This function performs source, source-settings and track-settings selection
// for content video capture based on the given |constraints|.
VideoCaptureSettings MODULES_EXPORT
SelectSettingsVideoContentCapture(const MediaConstraints& constraints,
                                  mojom::MediaStreamType stream_type,
                                  int screen_width,
                                  int screen_height);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_CONSTRAINTS_UTIL_VIDEO_CONTENT_H_
