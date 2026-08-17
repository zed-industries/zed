// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_UNDERLYING_SINK_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_UNDERLYING_SINK_WRAPPER_H_

#include "base/gtest_prod_util.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/unguessable_token.h"
#include "third_party/blink/renderer/core/streams/underlying_sink_base.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_encoded_audio_underlying_source.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_encoded_video_underlying_source.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_encoded_audio_stream_transformer.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_encoded_video_stream_transformer.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {
class RTCEncodedAudioUnderlyingSink;
class RTCEncodedVideoUnderlyingSink;
class RTCEncodedVideoStreamTransformer;

class MODULES_EXPORT RTCEncodedUnderlyingSinkWrapper
    : public UnderlyingSinkBase {
 public:
  explicit RTCEncodedUnderlyingSinkWrapper(ScriptState* script_state);

  void CreateAudioUnderlyingSink(
      scoped_refptr<RTCEncodedAudioStreamTransformer::Broker>
          encoded_audio_transformer,
      base::UnguessableToken owner_id);
  void CreateVideoUnderlyingSink(
      scoped_refptr<RTCEncodedVideoStreamTransformer::Broker>
          encoded_video_transformer,
      base::UnguessableToken owner_id);

  // UnderlyingSinkBase
  ScriptPromise<IDLUndefined> start(ScriptState*,
                                    WritableStreamDefaultController*,
                                    ExceptionState&) override;
  ScriptPromise<IDLUndefined> write(ScriptState*,
                                    ScriptValue chunk,
                                    WritableStreamDefaultController*,
                                    ExceptionState&) override;
  ScriptPromise<IDLUndefined> close(ScriptState*, ExceptionState&) override;
  ScriptPromise<IDLUndefined> abort(ScriptState*,
                                    ScriptValue reason,
                                    ExceptionState&) override;
  void Clear();
  void Trace(Visitor*) const override;

 private:
  SEQUENCE_CHECKER(sequence_checker_);
  Member<ScriptState> script_state_;
  Member<RTCEncodedAudioUnderlyingSink> audio_to_packetizer_underlying_sink_;
  Member<RTCEncodedVideoUnderlyingSink> video_to_packetizer_underlying_sink_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_UNDERLYING_SINK_WRAPPER_H_
