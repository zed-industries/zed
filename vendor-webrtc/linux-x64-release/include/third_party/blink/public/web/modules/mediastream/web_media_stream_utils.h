// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_UTILS_H_

#include "media/capture/video_capture_types.h"
#include "third_party/blink/public/platform/media/video_capture.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_sink.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_source.h"

namespace blink {

class WebMediaStreamTrack;

// See documentation of MediaStreamVideoTrack::CreateVideoTrack().
BLINK_MODULES_EXPORT WebMediaStreamTrack CreateWebMediaStreamVideoTrack(
    MediaStreamVideoSource* source,
    MediaStreamVideoSource::ConstraintsOnceCallback callback,
    bool enabled);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_UTILS_H_
