// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SCRIPT_TRANSFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SCRIPT_TRANSFORM_H_

#include <optional>

#include "base/thread_annotations.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/workers/dedicated_worker.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_script_transformer.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_encoded_audio_stream_transformer.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_encoded_video_stream_transformer.h"

namespace blink {

class ExceptionState;
class ScriptState;
class RTCRtpReceiver;

class MODULES_EXPORT RTCRtpScriptTransform : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum class SendKeyFrameRequestResult {
    kSuccess,
    kNoReceiver,
    kNoVideo,
    kTrackEnded,
    kInvalidState
  };

  static RTCRtpScriptTransform* Create(ScriptState*,
                                       DedicatedWorker* worker,
                                       ExceptionState&);
  static RTCRtpScriptTransform* Create(ScriptState*,
                                       DedicatedWorker* worker,
                                       const ScriptValue& message,
                                       ExceptionState&);
  static RTCRtpScriptTransform* Create(ScriptState*,
                                       DedicatedWorker* worker,
                                       const ScriptValue& message,
                                       HeapVector<ScriptObject> transfer,
                                       ExceptionState&);

  RTCRtpScriptTransform() = default;
  ~RTCRtpScriptTransform() override = default;

  // Called after the RTCTransformEvent is fired.
  void SetRtpTransformer(
      CrossThreadWeakHandle<RTCRtpScriptTransformer>,
      scoped_refptr<base::SingleThreadTaskRunner> transformer_task_runner);

  // Called when this transform is assigned to an RTCRtpSender or
  // RTCRtpReceiver.
  void CreateAudioUnderlyingSourceAndSink(
      WTF::CrossThreadOnceClosure disconnect_callback_source,
      scoped_refptr<blink::RTCEncodedAudioStreamTransformer::Broker>
          encoded_audio_transformer);
  void CreateVideoUnderlyingSourceAndSink(
      WTF::CrossThreadOnceClosure disconnect_callback_source,
      scoped_refptr<blink::RTCEncodedVideoStreamTransformer::Broker>
          encoded_video_transformer);

  void Attach() {
    DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
    is_attached_ = true;
  }
  void AttachToReceiver(RTCRtpReceiver*);
  bool IsAttached() {
    DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
    return is_attached_;
  }
  void Detach();

  void CreateVideoUnderlyingSink(
      scoped_refptr<blink::RTCEncodedVideoStreamTransformer::Broker>
          encoded_video_transformer);
  void CreateAudioUnderlyingSink(
      scoped_refptr<blink::RTCEncodedAudioStreamTransformer::Broker>
          encoded_audio_transformer);

  void SendKeyFrameRequestToReceiver(
      CrossThreadFunction<void(const SendKeyFrameRequestResult)> callback);

  RTCRtpScriptTransform::SendKeyFrameRequestResult
  HandleSendKeyFrameRequestResults();

  void Trace(Visitor*) const override;

 private:
  // These methods post a task to the worker to set up an
  // RTCRtpScriptTransformer. They are called when the RTCRtpScriptTransformer
  // is assigned to a sender or receiver.
  void SetUpAudioRtpTransformer(
      WTF::CrossThreadOnceClosure disconnect_callback_source,
      scoped_refptr<blink::RTCEncodedAudioStreamTransformer::Broker>);
  void SetUpVideoRtpTransformer(
      WTF::CrossThreadOnceClosure disconnect_callback_source,
      scoped_refptr<blink::RTCEncodedVideoStreamTransformer::Broker>);

  SEQUENCE_CHECKER(sequence_checker_);

  std::optional<CrossThreadWeakHandle<RTCRtpScriptTransformer>> rtp_transformer_
      GUARDED_BY_CONTEXT(sequence_checker_);
  scoped_refptr<base::SingleThreadTaskRunner> rtp_transformer_task_runner_
      GUARDED_BY_CONTEXT(sequence_checker_);

  // These fields are used to store the callbacks only if the
  // RTCRtpScriptTransformer has not been created/set yet. The callbacks will be
  // invoked once the RTCRtpScriptTransformer becomes available.
  WTF::CrossThreadOnceClosure disconnect_callback_source_
      GUARDED_BY_CONTEXT(sequence_checker_);

  scoped_refptr<blink::RTCEncodedAudioStreamTransformer::Broker>
      encoded_audio_transformer_ GUARDED_BY_CONTEXT(sequence_checker_);
  scoped_refptr<blink::RTCEncodedVideoStreamTransformer::Broker>
      encoded_video_transformer_ GUARDED_BY_CONTEXT(sequence_checker_);

  bool is_attached_ GUARDED_BY_CONTEXT(sequence_checker_) = false;
  WeakMember<RTCRtpReceiver> receiver_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SCRIPT_TRANSFORM_H_
