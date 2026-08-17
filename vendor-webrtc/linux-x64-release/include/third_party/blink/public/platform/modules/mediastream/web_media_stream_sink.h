// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_SINK_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_SINK_H_

#include <optional>

#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_source.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_track.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

// WebMediaStreamSink is the base interface for WebMediaStreamAudioSink and
// WebMediaStreamVideoSink. It allows an implementation to receive notifications
// about state changes on a WebMediaStreamSource object or such an
// object underlying a WebMediaStreamTrack.
class BLINK_PLATFORM_EXPORT WebMediaStreamSink {
 public:
  virtual void OnReadyStateChanged(WebMediaStreamSource::ReadyState state) {}
  virtual void OnEnabledChanged(bool enabled) {}
  virtual void OnContentHintChanged(
      WebMediaStreamTrack::ContentHintType content_hint) {}

  // OnVideoConstraintsChanged is called when constraints set on the source
  // MediaStreamVideoTrack change. Never called in case the sink isn't connected
  // to a video track.
  virtual void OnVideoConstraintsChanged(std::optional<double> min_fps,
                                         std::optional<double> max_fps) {}

 protected:
  virtual ~WebMediaStreamSink() {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_SINK_H_
