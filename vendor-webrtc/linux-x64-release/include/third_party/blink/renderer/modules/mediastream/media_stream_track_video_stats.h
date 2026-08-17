// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_VIDEO_STATS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_VIDEO_STATS_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_track_platform.h"

namespace blink {

class MediaStreamTrackImpl;

class MODULES_EXPORT MediaStreamTrackVideoStats : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit MediaStreamTrackVideoStats(MediaStreamTrackImpl*);

  uint64_t deliveredFrames(ScriptState*);
  uint64_t discardedFrames(ScriptState*);
  uint64_t totalFrames(ScriptState*);

  ScriptObject toJSON(ScriptState*);

  void Trace(Visitor*) const override;

 private:
  void PopulateStatsCache(ScriptState*);
  void InvalidateStatsCache();

  WeakMember<MediaStreamTrackImpl> track_;
  MediaStreamTrackPlatform::VideoFrameStats stats_;
  bool stats_invalidated_ = true;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_VIDEO_STATS_H_
