// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_TRANSPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_TRANSPORT_H_

#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/workers/custom_event_message.h"
#include "third_party/blink/renderer/core/workers/dedicated_worker.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_acks.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_sent.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_transport_processor.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"
#include "third_party/webrtc/api/transport/network_control.h"

namespace blink {

// Event created on a Worker containing a newly created
// RTCRtpTransportProcessor, triggered by a call to
// RTCRtpTransport::createProcessor() on the main thread.
class MODULES_EXPORT RTCRtpTransportProcessorEvent : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit RTCRtpTransportProcessorEvent(RTCRtpTransportProcessor* processor)
      : Event(event_type_names::kRtcrtptransportprocessor,
              Bubbles::kNo,
              Cancelable::kNo),
        processor_(processor) {}

  // Impl of IDL type.
  RTCRtpTransportProcessor* processor() { return processor_; }

  void Trace(Visitor* visitor) const override {
    Event::Trace(visitor);
    visitor->Trace(processor_);
  }

 private:
  Member<RTCRtpTransportProcessor> processor_;
};

class FeedbackProvider : public WTF::ThreadSafeRefCounted<FeedbackProvider> {
 public:
  virtual ~FeedbackProvider() = default;
  virtual void SetProcessor(
      CrossThreadWeakHandle<RTCRtpTransportProcessor>
          rtp_transport_processor_handle,
      scoped_refptr<base::SequencedTaskRunner> task_runner) = 0;
  virtual void SetCustomMaxBitrateBps(uint64_t custom_max_bitrate_bps) = 0;
};

class MODULES_EXPORT RTCRtpTransport : public ScriptWrappable,
                                       public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit RTCRtpTransport(ExecutionContext* context);

  ~RTCRtpTransport() override;

  // Implements rtc_rtp_transport.idl
  void createProcessor(ScriptState*, DedicatedWorker* worker, ExceptionState&);
  void createProcessor(ScriptState*,
                       DedicatedWorker* worker,
                       const ScriptValue& options,
                       ExceptionState&);
  void createProcessor(ScriptState*,
                       DedicatedWorker* worker,
                       const ScriptValue& options,
                       HeapVector<ScriptObject> transfer,
                       ExceptionState&);

  void RegisterFeedbackProvider(scoped_refptr<FeedbackProvider> observer);

  // Receive Handle to the matching Processor created on a Worker following a
  // call to createProcessor().
  void SetProcessorHandle(
      CrossThreadWeakHandle<RTCRtpTransportProcessor> processor,
      scoped_refptr<base::SequencedTaskRunner> processor_task_runner);

  void Trace(Visitor* visitor) const override;

 private:
  // WebRTC network feedback providers which must be told of Processors. Main
  // thread only.
  Vector<scoped_refptr<FeedbackProvider>> feedback_providers_;

  // Handle to the most recently created Processor, set after it's created on
  // the worker. Main thread only.
  std::optional<CrossThreadWeakHandle<RTCRtpTransportProcessor>> processor_;
  scoped_refptr<base::SequencedTaskRunner> processor_task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_TRANSPORT_H_
