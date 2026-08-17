// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_SCTP_TRANSPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_SCTP_TRANSPORT_H_

#include <memory>

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/peerconnection/adapters/sctp_transport_proxy.h"
#include "third_party/webrtc/api/scoped_refptr.h"
#include "third_party/webrtc/api/sctp_transport_interface.h"

namespace blink {

class SctpTransportProxy;
class RTCDtlsTransport;
class V8RTCSctpTransportState;

enum class RTCSctpTransportState { kChecking, kConnected, kClosed };

// Blink bindings for the RTCSctpTransport JavaScript object.
class MODULES_EXPORT RTCSctpTransport final
    : public EventTarget,
      public ExecutionContextClient,
      public SctpTransportProxy::Delegate {
  DEFINE_WRAPPERTYPEINFO();

 public:
  RTCSctpTransport(
      ExecutionContext* context,
      webrtc::scoped_refptr<webrtc::SctpTransportInterface> native_transport);
  // Constructor with explicit thread injection, used for testing.
  RTCSctpTransport(
      ExecutionContext* context,
      webrtc::scoped_refptr<webrtc::SctpTransportInterface> native_transport,
      scoped_refptr<base::SingleThreadTaskRunner> main_thread,
      scoped_refptr<base::SingleThreadTaskRunner> worker_thread);
  ~RTCSctpTransport() override;

  // rtc_sctp_transport.idl
  RTCDtlsTransport* transport() const;
  V8RTCSctpTransportState state() const;
  double maxMessageSize() const;
  std::optional<int16_t> maxChannels() const;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(statechange, kStatechange)

  // SctpTransportProxy::Delegate
  void OnStartCompleted(webrtc::SctpTransportInformation info) override;
  void OnStateChange(webrtc::SctpTransportInformation info) override;

  // EventTarget overrides.
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;
  // Others
  void ChangeState(webrtc::SctpTransportInformation info);
  void SetTransport(RTCDtlsTransport*);
  webrtc::scoped_refptr<webrtc::SctpTransportInterface> native_transport();
  // Called from owning RtcPeerConnection when it is closed.
  void Close();
  // For garbage collection.
  void Trace(Visitor* visitor) const override;

 private:
  webrtc::SctpTransportInformation current_state_;
  webrtc::scoped_refptr<webrtc::SctpTransportInterface> native_transport_;
  std::unique_ptr<SctpTransportProxy> proxy_;
  Member<RTCDtlsTransport> dtls_transport_;
  bool start_completed_ = false;
  bool closed_from_owner_ = false;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_SCTP_TRANSPORT_H_
