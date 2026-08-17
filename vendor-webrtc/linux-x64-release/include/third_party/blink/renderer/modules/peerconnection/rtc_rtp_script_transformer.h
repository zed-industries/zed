// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SCRIPT_TRANSFORMER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SCRIPT_TRANSFORMER_H_

#include "base/task/sequenced_task_runner.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/unpacked_serialized_script_value.h"
#include "third_party/blink/renderer/core/streams/readable_stream.h"
#include "third_party/blink/renderer/core/streams/writable_stream.h"
#include "third_party/blink/renderer/core/workers/custom_event_message.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_encoded_underlying_sink_wrapper.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_encoded_underlying_source_wrapper.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/v8_external_memory_accounter.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_encoded_audio_stream_transformer.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_encoded_video_stream_transformer.h"

namespace blink {

class ReadableStream;
class RTCRtpScriptTransform;

// RTCRtpScriptTransformer is created and lives in the worker context.
class MODULES_EXPORT RTCRtpScriptTransformer : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  RTCRtpScriptTransformer() = default;
  ~RTCRtpScriptTransformer() override;

  explicit RTCRtpScriptTransformer(
      ScriptState* script_state,
      CustomEventMessage options,
      scoped_refptr<base::SequencedTaskRunner> transform_task_runner,
      CrossThreadWeakHandle<RTCRtpScriptTransform> transform);

  // rtc_rtp_script_transformer.idl
  ScriptValue options(ScriptState*);
  ReadableStream* readable() { return readable_; }
  WritableStream* writable() { return writable_; }
  ScriptPromise<IDLUndefined> sendKeyFrameRequest(ScriptState* script_state);

  // Never invalidates the cache because options is immutable.
  bool IsOptionsDirty() const;
  void Clear();
  void Trace(Visitor*) const override;

  // These methods are called when an
  // RTCRtpScriptTransform is assigned to an RTCRtpSender or RTCRtpReceiver.
  void SetUpAudio(WTF::CrossThreadOnceClosure disconnect_callback_source,
                  scoped_refptr<blink::RTCEncodedAudioStreamTransformer::Broker>
                      encoded_audio_transformer);
  void SetUpVideo(WTF::CrossThreadOnceClosure disconnect_callback_source,
                  scoped_refptr<blink::RTCEncodedVideoStreamTransformer::Broker>
                      encoded_video_transformer);

 private:
  size_t SizeOfExternalMemoryInBytes();

  const scoped_refptr<base::SingleThreadTaskRunner>
      rtp_transformer_task_runner_;
  const scoped_refptr<base::SequencedTaskRunner> rtp_transform_task_runner_;
  Member<UnpackedSerializedScriptValue> data_as_serialized_script_value_;
  V8ExternalMemoryAccounter serialized_data_memory_accounter_;
  Member<GCedMessagePortArray> ports_;

  Member<ReadableStream> readable_;
  Member<WritableStream> writable_;

  std::optional<CrossThreadWeakHandle<RTCRtpScriptTransform>> transform_;

  const Member<RTCEncodedUnderlyingSourceWrapper>
      rtc_encoded_underlying_source_;
  const Member<RTCEncodedUnderlyingSinkWrapper> rtc_encoded_underlying_sink_;

  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SCRIPT_TRANSFORMER_H_
