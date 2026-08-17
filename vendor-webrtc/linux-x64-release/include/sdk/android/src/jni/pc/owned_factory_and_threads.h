/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_PC_OWNED_FACTORY_AND_THREADS_H_
#define SDK_ANDROID_SRC_JNI_PC_OWNED_FACTORY_AND_THREADS_H_

#include <jni.h>

#include <memory>
#include <utility>

#include "api/peer_connection_interface.h"
#include "rtc_base/thread.h"

namespace webrtc {
namespace jni {

// Helper struct for working around the fact that CreatePeerConnectionFactory()
// comes in two flavors: either entirely automagical (constructing its own
// threads and deleting them on teardown, but no external codec factory support)
// or entirely manual (requires caller to delete threads after factory
// teardown).  This struct takes ownership of its ctor's arguments to present a
// single thing for Java to hold and eventually free.
class OwnedFactoryAndThreads {
 public:
  OwnedFactoryAndThreads(
      std::unique_ptr<SocketFactory> socket_factory,
      std::unique_ptr<Thread> network_thread,
      std::unique_ptr<Thread> worker_thread,
      std::unique_ptr<Thread> signaling_thread,
      const scoped_refptr<PeerConnectionFactoryInterface>& factory);

  ~OwnedFactoryAndThreads() = default;

  PeerConnectionFactoryInterface* factory() { return factory_.get(); }
  SocketFactory* socket_factory() { return socket_factory_.get(); }
  Thread* network_thread() { return network_thread_.get(); }
  Thread* signaling_thread() { return signaling_thread_.get(); }
  Thread* worker_thread() { return worker_thread_.get(); }

 private:
  // Usually implemented by the SocketServer associated with the network thread,
  // so needs to outlive the network thread.
  const std::unique_ptr<SocketFactory> socket_factory_;
  const std::unique_ptr<Thread> network_thread_;
  const std::unique_ptr<Thread> worker_thread_;
  const std::unique_ptr<Thread> signaling_thread_;
  const scoped_refptr<PeerConnectionFactoryInterface> factory_;
};

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_OWNED_FACTORY_AND_THREADS_H_
