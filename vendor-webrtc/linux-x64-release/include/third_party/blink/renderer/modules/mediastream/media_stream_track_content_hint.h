// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_CONTENT_HINT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_CONTENT_HINT_H_

#include "third_party/blink/renderer/modules/mediastream/media_stream_track.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class MediaStreamTrackContentHint final {
  STATIC_ONLY(MediaStreamTrackContentHint);

 public:
  static String contentHint(const MediaStreamTrack& track) {
    return track.ContentHint();
  }
  static void setContentHint(MediaStreamTrack& track, const String& hint) {
    track.SetContentHint(hint);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_CONTENT_HINT_H_
