// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_SCTP_TRANSPORT_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_SCTP_TRANSPORT_PROXY_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/webrtc/api/sctp_transport_interface.h"

// The SctpTransportProxy class takes care of thread-jumping when
// connecting callbacks from a webrtc::SctpTransport to a
// blink::RTCSctpTransport object.

// Its design is modeled on the IceTransportProxy design,
// but does not use so many layers of indirection - there is
// no control, and all information is passed via callbacks on the Delegate.

// The proxy thread = the Blink main thread
// The host thread = the webrtc signalling thread (the one that gets callbacks)

namespace blink {

class LocalFrame;

class SctpTransportProxy : public webrtc::SctpTransportObserverInterface {
 public:
  // Delegate class for actions caused by the Proxy, but executed on the
  // main thread. The Delegate must remain alive until it has observed
  // the kClosed state, which is the last thing that can happen.
  class Delegate : public blink::GarbageCollectedMixin {
   public:
    virtual ~Delegate() = default;

    // Called when the Create() function is complete. Sends current state,
    // but does not indicate a state change.
    virtual void OnStartCompleted(webrtc::SctpTransportInformation info) = 0;
    // Called when a state change is signalled from transport.
    virtual void OnStateChange(webrtc::SctpTransportInformation info) = 0;
    void Trace(Visitor* visitor) const override {}
  };

  // Constructs a SctpTransportProxy.  The caller is responsible for keeping
  // |sctp_transport| alive until after the SctpTransportProxy is deleted.
  // The SctpTransportProxy takes a <CrossThreadPersistent> reference to the
  // Delegate, preventing it from being garbage collected until the |kClosed|
  // state is reached.
  // The SctpTransportProxy can only be safely deleted after seeing the state
  // |kClosed|, since this is the last event that can happen on the transport.
  static std::unique_ptr<SctpTransportProxy> Create(
      LocalFrame& frame,
      scoped_refptr<base::SingleThreadTaskRunner> proxy_thread,
      scoped_refptr<base::SingleThreadTaskRunner> host_thread,
      webrtc::scoped_refptr<webrtc::SctpTransportInterface> sctp_transport,
      Delegate* delegate);

  ~SctpTransportProxy() override {}

 private:
  SctpTransportProxy(
      LocalFrame& frame,
      scoped_refptr<base::SingleThreadTaskRunner> proxy_thread,
      scoped_refptr<base::SingleThreadTaskRunner> host_thread,
      webrtc::scoped_refptr<webrtc::SctpTransportInterface> sctp_transport,
      Delegate* delegate);
  // Implementation of webrtc::SctpTransportObserver
  void OnStateChange(webrtc::SctpTransportInformation info) override;

  // Internal helper for Create()
  void StartOnHostThread();

  const scoped_refptr<base::SingleThreadTaskRunner> proxy_thread_;
  const scoped_refptr<base::SingleThreadTaskRunner> host_thread_;
  const webrtc::scoped_refptr<webrtc::SctpTransportInterface> sctp_transport_;
  UnwrappingCrossThreadHandle<Delegate> delegate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_SCTP_TRANSPORT_PROXY_H_
