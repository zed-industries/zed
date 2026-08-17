// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_VIDEO_UNDERLYING_SINK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_VIDEO_UNDERLYING_SINK_H_

#include "base/functional/callback.h"
#include "third_party/blink/renderer/core/streams/underlying_sink_base.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_encoded_video_stream_transformer.h"
#include "third_party/webrtc/api/frame_transformer_interface.h"

namespace blink {

class ExceptionState;
class RTCEncodedVideoStreamTransformer;

class MODULES_EXPORT RTCEncodedVideoUnderlyingSink final
    : public UnderlyingSinkBase {
 public:
  RTCEncodedVideoUnderlyingSink(
      ScriptState*,
      scoped_refptr<blink::RTCEncodedVideoStreamTransformer::Broker>,
      bool detach_frame_data_on_write);
  RTCEncodedVideoUnderlyingSink(
      ScriptState* script_state,
      scoped_refptr<blink::RTCEncodedVideoStreamTransformer::Broker>
          transformer_broker,
      bool detach_frame_data_on_write,
      bool enable_frame_restrictions,
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
  void ResetTransformerCallback();
  void Trace(Visitor*) const override;

 private:
  scoped_refptr<blink::RTCEncodedVideoStreamTransformer::Broker>
      transformer_broker_;
  const bool detach_frame_data_on_write_;
  const bool enable_frame_restrictions_;
  base::UnguessableToken owner_id_;
  int64_t last_received_frame_counter_ = std::numeric_limits<uint64_t>::min();
  THREAD_CHECKER(thread_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_VIDEO_UNDERLYING_SINK_H_
