// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_DTLS_TRANSPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_DTLS_TRANSPORT_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/peerconnection/adapters/dtls_transport_proxy.h"
#include "third_party/webrtc/api/dtls_transport_interface.h"
#include "third_party/webrtc/api/scoped_refptr.h"

namespace blink {

class DtlsTransportProxy;
class DOMArrayBuffer;
class RTCIceTransport;
class V8RTCDtlsTransportState;

enum class RTCDtlsTransportState {
  kNew,
  kChecking,
  kConnected,
  kCompleted,
  kDisconnected,
  kFailed,
  kClosed
};

// Blink bindings for the RTCDtlsTransport JavaScript object.
//
class MODULES_EXPORT RTCDtlsTransport final
    : public EventTarget,
      public ExecutionContextClient,
      public DtlsTransportProxy::Delegate {
  DEFINE_WRAPPERTYPEINFO();

 public:
  RTCDtlsTransport(
      ExecutionContext* context,
      webrtc::scoped_refptr<webrtc::DtlsTransportInterface> native_context,
      RTCIceTransport* ice_transport);
  ~RTCDtlsTransport() override;

  // rtc_dtls_transport.idl
  RTCIceTransport* iceTransport() const;
  V8RTCDtlsTransportState state() const;
  const HeapVector<Member<DOMArrayBuffer>>& getRemoteCertificates() const;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(statechange, kStatechange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)

  // DtlsTransportProxy::Delegate
  void OnStartCompleted(webrtc::DtlsTransportInformation info) override;
  void OnStateChange(webrtc::DtlsTransportInformation info) override;

  // EventTarget overrides.
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;
  // For garbage collection.
  void Trace(Visitor* visitor) const override;
  // Others
  void ChangeState(webrtc::DtlsTransportInformation info);
  webrtc::DtlsTransportInterface* native_transport();
  void Close();

 private:
  webrtc::DtlsTransportInformation current_state_;
  HeapVector<Member<DOMArrayBuffer>> remote_certificates_;
  webrtc::scoped_refptr<webrtc::DtlsTransportInterface> native_transport_;
  std::unique_ptr<DtlsTransportProxy> proxy_;
  Member<RTCIceTransport> ice_transport_;
  bool closed_from_owner_ = false;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_DTLS_TRANSPORT_H_
