// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_HOST_ADDRESS_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_HOST_ADDRESS_REQUEST_H_

#include <stdint.h>

#include <optional>

#include "base/functional/callback.h"
#include "base/threading/thread_checker.h"
#include "net/base/ip_address.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc/rtc_base/socket_address.h"

namespace blink {

class P2PSocketDispatcher;

// P2PAsyncAddressResolver performs DNS hostname resolution. It's used
// to resolve addresses of STUN and relay servers. It is created and lives on
// one of libjingle's threads.
class P2PAsyncAddressResolver
    : public ThreadSafeRefCounted<P2PAsyncAddressResolver> {
 public:
  using DoneCallback = base::OnceCallback<void(const Vector<net::IPAddress>&)>;

  P2PAsyncAddressResolver(P2PSocketDispatcher* dispatcher);
  P2PAsyncAddressResolver(const P2PAsyncAddressResolver&) = delete;
  P2PAsyncAddressResolver& operator=(const P2PAsyncAddressResolver&) = delete;

  // Start address resolve process.
  void Start(const webrtc::SocketAddress& addr,
             std::optional<int> address_family,
             DoneCallback done_callback);
  // Clients must unregister before exiting for cleanup.
  void Cancel();

 private:
  enum State {
    kStateCreated,
    kStateSent,
    kStateFinished,
  };

  friend class P2PSocketDispatcher;
  friend class ThreadSafeRefCounted<P2PAsyncAddressResolver>;
  virtual ~P2PAsyncAddressResolver();

  void OnResponse(const Vector<net::IPAddress>& address);

  // `P2PSocketDispatcher` is owned by the main thread, and must be accessed in
  // a thread-safe way. Will be reset once `Start()` is called, so it doesn't
  // prevent the dispatcher from being garbage-collected.
  CrossThreadPersistent<P2PSocketDispatcher> dispatcher_;
  THREAD_CHECKER(thread_checker_);

  // State must be accessed from delegate thread only.
  State state_;
  DoneCallback done_callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_HOST_ADDRESS_REQUEST_H_
