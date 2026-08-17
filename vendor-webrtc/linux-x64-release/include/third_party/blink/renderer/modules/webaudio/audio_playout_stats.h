// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PLAYOUT_STATS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PLAYOUT_STATS_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webaudio/audio_context.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_track_platform.h"

namespace blink {

// https://github.com/WICG/web_audio_playout
class MODULES_EXPORT AudioPlayoutStats : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit AudioPlayoutStats(AudioContext*);

  DOMHighResTimeStamp fallbackFramesDuration(ScriptState* script_state);
  uint32_t fallbackFramesEvents(ScriptState* script_state);
  DOMHighResTimeStamp totalFramesDuration(ScriptState* script_state);
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

  WeakMember<AudioContext> context_;
  AudioFrameStatsAccumulator stats_;
  bool stats_are_from_current_task_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PLAYOUT_STATS_H_
