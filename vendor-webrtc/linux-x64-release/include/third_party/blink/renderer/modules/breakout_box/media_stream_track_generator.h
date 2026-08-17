// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_MEDIA_STREAM_TRACK_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_MEDIA_STREAM_TRACK_GENERATOR_H_

#include "third_party/blink/renderer/modules/mediastream/media_stream_track_impl.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class MediaStreamAudioTrackUnderlyingSink;
class MediaStreamTrackGeneratorInit;
class MediaStreamVideoTrackUnderlyingSink;
class PushableMediaStreamVideoSource;
class ScriptState;
class WritableStream;

class MODULES_EXPORT MediaStreamTrackGenerator : public MediaStreamTrackImpl {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static MediaStreamTrackGenerator* Create(ScriptState*,
                                           const String& kind,
                                           ExceptionState&);
  static MediaStreamTrackGenerator* Create(ScriptState*,
                                           MediaStreamTrackGeneratorInit* init,
                                           ExceptionState&);
  MediaStreamTrackGenerator(ScriptState*, MediaStreamSource::StreamType);
  MediaStreamTrackGenerator(const MediaStreamTrackGenerator&) = delete;
  MediaStreamTrackGenerator& operator=(const MediaStreamTrackGenerator&) =
      delete;

  WritableStream* writable(ScriptState* script_state);

  PushableMediaStreamVideoSource* PushableVideoSource() const;

  void Trace(Visitor* visitor) const override;

 private:
  void CreateAudioStream(ScriptState* script_state);

  void CreateVideoStream(ScriptState* script_state);

  static MediaStreamComponent* MakeMediaStreamComponent(
      ScriptState* script_state,
      MediaStreamSource::StreamType type);

  Member<MediaStreamAudioTrackUnderlyingSink> audio_underlying_sink_;
  Member<MediaStreamVideoTrackUnderlyingSink> video_underlying_sink_;
  Member<WritableStream> writable_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_MEDIA_STREAM_TRACK_GENERATOR_H_
