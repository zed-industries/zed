// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_MEDIA_STREAM_AUDIO_TRACK_UNDERLYING_SINK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_MEDIA_STREAM_AUDIO_TRACK_UNDERLYING_SINK_H_

#include "third_party/blink/renderer/core/streams/underlying_sink_base.h"
#include "third_party/blink/renderer/modules/breakout_box/pushable_media_stream_audio_source.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/audio_data.h"

namespace blink {

class WritableStreamTransferringOptimizer;

class MODULES_EXPORT MediaStreamAudioTrackUnderlyingSink
    : public UnderlyingSinkBase {
 public:
  // |source| must outlive this MediaStreamAudioTrackUnderlyingSink.
  explicit MediaStreamAudioTrackUnderlyingSink(
      scoped_refptr<PushableMediaStreamAudioSource::Broker> source_broker);

  // UnderlyingSinkBase overrides.
  ScriptPromise<IDLUndefined> start(ScriptState* script_state,
                                    WritableStreamDefaultController* controller,
                                    ExceptionState& exception_state) override;
  ScriptPromise<IDLUndefined> write(ScriptState* script_state,
                                    ScriptValue chunk,
                                    WritableStreamDefaultController* controller,
                                    ExceptionState& exception_state) override;
  ScriptPromise<IDLUndefined> abort(ScriptState* script_state,
                                    ScriptValue reason,
                                    ExceptionState& exception_state) override;
  ScriptPromise<IDLUndefined> close(ScriptState* script_state,
                                    ExceptionState& exception_state) override;

  std::unique_ptr<WritableStreamTransferringOptimizer>
  GetTransferringOptimizer();

 private:
  void Disconnect();
  const scoped_refptr<PushableMediaStreamAudioSource::Broker> source_broker_;
  bool is_connected_ = false;
  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_MEDIA_STREAM_AUDIO_TRACK_UNDERLYING_SINK_H_
