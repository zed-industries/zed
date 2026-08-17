// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_VIDEO_RECEIVER_SINK_OPTIMIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_VIDEO_RECEIVER_SINK_OPTIMIZER_H_

#include "third_party/blink/renderer/core/streams/underlying_sink_base.h"
#include "third_party/blink/renderer/core/streams/writable_stream_transferring_optimizer.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_encoded_video_underlying_sink.h"

namespace blink {

class UnderlyingSinkBase;
class ScriptState;
class RTCEncodedVideoUnderlyingSink;

class MODULES_EXPORT RtcEncodedVideoReceiverSinkOptimizer
    : public WritableStreamTransferringOptimizer {
 public:
  using UnderlyingSinkSetter =
      WTF::CrossThreadOnceFunction<void(RTCEncodedVideoUnderlyingSink*)>;
  RtcEncodedVideoReceiverSinkOptimizer(
      UnderlyingSinkSetter,
      scoped_refptr<blink::RTCEncodedVideoStreamTransformer::Broker>
          transformer);
  UnderlyingSinkBase* PerformInProcessOptimization(
      ScriptState* script_state) override;

 private:
  UnderlyingSinkSetter set_underlying_sink_;
  scoped_refptr<blink::RTCEncodedVideoStreamTransformer::Broker> transformer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_VIDEO_RECEIVER_SINK_OPTIMIZER_H_
