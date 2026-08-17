// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_ICE_TRANSPORT_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_ICE_TRANSPORT_HOST_H_

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/renderer/modules/peerconnection/adapters/ice_transport_adapter.h"
#include "third_party/blink/renderer/modules/peerconnection/adapters/ice_transport_adapter_cross_thread_factory.h"

namespace blink {

class IceTransportProxy;

// This class is the host side correspondent to the IceTransportProxy. See the
// IceTransportProxy documentation for background. This class lives on the host
// thread and proxies calls between the IceTransportProxy and the
// P2PTransportChannel (which is single-threaded).
//
//     proxy thread                               host thread
// +------------------+   unique_ptr   +------------------------------+
// |                  |   =========>   |                              |
// | client <-> Proxy |                | Host <-> P2PTransportChannel |
// |                  |   <---------   |                              |
// +------------------+    WeakPtr     +------------------------------+
//
// Since the client code controls the Proxy lifetime, the Proxy has a unique_ptr
// to the Host that lives on the host thread. The unique_ptr has an
// OnTaskRunnerDeleter so that when the Proxy is destroyed a task will be queued
// to delete the Host as well (and the P2PTransportChannel with it). The Host
// needs a pointer back to the Proxy to post callbacks, and by using a WeakPtr
// any callbacks run on the proxy thread after the proxy has been deleted will
// be safely dropped.
//
// The Host can be constructed on any thread but after that point all methods
// must be called on the host thread.
class IceTransportHost final : public IceTransportAdapter::Delegate {
 public:
  IceTransportHost(scoped_refptr<base::SingleThreadTaskRunner> proxy_thread,
                   scoped_refptr<base::SingleThreadTaskRunner> host_thread,
                   base::WeakPtr<IceTransportProxy> proxy);
  ~IceTransportHost() override;

  void Initialize(
      std::unique_ptr<IceTransportAdapterCrossThreadFactory> adapter_factory);

  scoped_refptr<base::SingleThreadTaskRunner> proxy_thread() const;
  scoped_refptr<base::SingleThreadTaskRunner> host_thread() const;

 private:
  // IceTransportAdapter::Delegate overrides.
  void OnGatheringStateChanged(webrtc::IceGatheringState new_state) override;
  void OnCandidateGathered(const webrtc::Candidate& candidate) override;
  void OnStateChanged(webrtc::IceTransportState new_state) override;
  void OnSelectedCandidatePairChanged(
      const std::pair<webrtc::Candidate, webrtc::Candidate>&
          selected_candidate_pair) override;

  const scoped_refptr<base::SingleThreadTaskRunner> proxy_thread_;
  const scoped_refptr<base::SingleThreadTaskRunner> host_thread_;
  std::unique_ptr<IceTransportAdapter> transport_;
  base::WeakPtr<IceTransportProxy> proxy_;

  THREAD_CHECKER(thread_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_ICE_TRANSPORT_HOST_H_
