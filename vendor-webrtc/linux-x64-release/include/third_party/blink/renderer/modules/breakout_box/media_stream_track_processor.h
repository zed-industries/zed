// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_MEDIA_STREAM_TRACK_PROCESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_MEDIA_STREAM_TRACK_PROCESSOR_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;
class MediaStreamVideoTrackUnderlyingSource;
class MediaStreamAudioTrackUnderlyingSource;
class MediaStreamTrack;
class MediaStreamTrackProcessorInit;
class ReadableStream;
class ScriptState;

class MODULES_EXPORT MediaStreamTrackProcessor : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static MediaStreamTrackProcessor* Create(ScriptState*,
                                           MediaStreamTrackProcessorInit*,
                                           ExceptionState&);
  static MediaStreamTrackProcessor* Create(ScriptState*,
                                           MediaStreamTrack*,
                                           uint16_t buffer_size,
                                           ExceptionState&);
  static MediaStreamTrackProcessor* Create(ScriptState*,
                                           MediaStreamTrack*,
                                           ExceptionState&);
  MediaStreamTrackProcessor(ScriptState*,
                            MediaStreamTrack*,
                            uint16_t buffer_size);
  MediaStreamTrackProcessor(const MediaStreamTrackProcessor&) = delete;
  MediaStreamTrackProcessor& operator=(const MediaStreamTrackProcessor&) =
      delete;

  // MediaStreamTrackProcessor interface
  ReadableStream* readable(ScriptState* script_state);

  // Closes |audio_underlying_source_| and |video_underlying_source_| if they
  // exist.
  void CloseSources();

  MediaStreamTrack* InputTrack() { return input_track_.Get(); }

  void Trace(Visitor* visitor) const override;

 private:
  void CreateVideoSourceStream(ScriptState* script_state);
  void CreateAudioSourceStream(ScriptState* script_state);

  class UnderlyingSourceCloser;

  Member<MediaStreamTrack> input_track_;
  Member<MediaStreamVideoTrackUnderlyingSource> video_underlying_source_;
  Member<MediaStreamAudioTrackUnderlyingSource> audio_underlying_source_;
  Member<ReadableStream> source_stream_;
  Member<UnderlyingSourceCloser> source_closer_;
  uint16_t buffer_size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_MEDIA_STREAM_TRACK_PROCESSOR_H_
