// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_AUDIO_STATS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_AUDIO_STATS_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_track_platform.h"

namespace blink {

class MediaStreamTrackImpl;

class MODULES_EXPORT MediaStreamTrackAudioStats : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit MediaStreamTrackAudioStats(MediaStreamTrackImpl*);

  uint64_t deliveredFrames(ScriptState* script_state);
  DOMHighResTimeStamp deliveredFramesDuration(ScriptState* script_state);
  uint64_t totalFrames(ScriptState* script_state);
  DOMHighResTimeStamp totalFramesDuration(ScriptState* script_state);
  DOMHighResTimeStamp latency(ScriptState* script_state);
  DOMHighResTimeStamp averageLatency(ScriptState* script_state);
  DOMHighResTimeStamp minimumLatency(ScriptState* script_state);
  DOMHighResTimeStamp maximumLatency(ScriptState* script_state);
  void resetLatency(ScriptState* script_state);

  ScriptObject toJSON(ScriptState* script_state);

  void Trace(Visitor*) const override;

 private:
  // Updates the stats if they have not yet been updated during the current
  // task.
  void MaybeUpdateStats(ScriptState* script_state);
  void OnMicrotask();

  WeakMember<MediaStreamTrackImpl> track_;
  MediaStreamTrackPlatform::AudioFrameStats stats_;
  bool stats_are_from_current_task_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_AUDIO_STATS_H_
