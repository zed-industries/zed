// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_DTLS_TRANSPORT_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_DTLS_TRANSPORT_PROXY_H_

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/webrtc/api/dtls_transport_interface.h"

// The DtlsTransportProxy class takes care of thread-jumping when
// connecting callbacks from a webrtc::DtlsTransport to a
// blink::RTCDtlsTransport object.

// Its design is modeled on the IceTransportProxy design,
// but does not use so many layers of indirection - there is
// no control, and all information is passed via callbacks on the Delegate.

// The proxy thread = the Blink main thread
// The host thread = the webrtc network thread (the one that gets callbacks)

namespace blink {

class LocalFrame;

class DtlsTransportProxy : public webrtc::DtlsTransportObserverInterface {
 public:
  class Delegate : public GarbageCollectedMixin {
   public:
    virtual ~Delegate() = default;

    // Called when the Create() function is complete. Sends current state,
    // but does not indicate a state change.
    virtual void OnStartCompleted(webrtc::DtlsTransportInformation info) = 0;
    // Called when a state change is signalled from transport.
    virtual void OnStateChange(webrtc::DtlsTransportInformation info) = 0;
    void Trace(Visitor* visitor) const override {}
  };
  // Constructs a DtlsTransportProxy.
  // The caller is responsible for keeping |dtls_transport| and |delegate|
  // alive until after the DtlsTransportProxy is deleted.
  // The DtlsTransportProxy can be safely deleted after seeing the
  // state |kClosed|, since this is the last event that can happen
  // on the transport.
  static std::unique_ptr<DtlsTransportProxy> Create(
      LocalFrame& frame,
      scoped_refptr<base::SingleThreadTaskRunner> proxy_thread,
      scoped_refptr<base::SingleThreadTaskRunner> host_thread,
      webrtc::DtlsTransportInterface* dtls_transport,
      Delegate* delegate);

 private:
  DtlsTransportProxy(LocalFrame& frame,
                     scoped_refptr<base::SingleThreadTaskRunner> proxy_thread,
                     scoped_refptr<base::SingleThreadTaskRunner> host_thread,
                     webrtc::DtlsTransportInterface* dtls_transport,
                     Delegate* delegate);
  // Implementation of webrtc::DtlsTransportObserver
  void OnStateChange(webrtc::DtlsTransportInformation info) override;
  void OnError(webrtc::RTCError error) override;

  // Internal helper for Create()
  void StartOnHostThread();

  const scoped_refptr<base::SingleThreadTaskRunner> proxy_thread_;
  const scoped_refptr<base::SingleThreadTaskRunner> host_thread_;
  raw_ptr<webrtc::DtlsTransportInterface> dtls_transport_;
  CrossThreadHandle<Delegate> delegate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_DTLS_TRANSPORT_PROXY_H_
