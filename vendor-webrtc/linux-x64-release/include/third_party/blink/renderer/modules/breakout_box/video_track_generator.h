// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_VIDEO_TRACK_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_VIDEO_TRACK_GENERATOR_H_

#include "third_party/blink/renderer/modules/breakout_box/media_stream_video_track_underlying_sink.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_track.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class MediaStreamTrackGenerator;
class ScriptState;
class WritableStream;

class MODULES_EXPORT VideoTrackGenerator : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static VideoTrackGenerator* Create(ScriptState*, ExceptionState&);
  VideoTrackGenerator(ScriptState*, ExceptionState&);
  VideoTrackGenerator(const VideoTrackGenerator&) = delete;
  VideoTrackGenerator& operator=(const VideoTrackGenerator&) = delete;

  WritableStream* writable(ScriptState* script_state);
  bool muted();
  void setMuted(bool);
  MediaStreamTrack* track();

  void Trace(Visitor* visitor) const override;

 private:
  Member<MediaStreamTrackGenerator> wrapped_generator_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_VIDEO_TRACK_GENERATOR_H_
