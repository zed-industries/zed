// Copyright 2016 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_TEST_CPP_END2END_CONNECTION_ATTEMPT_INJECTOR_H
#define GRPC_TEST_CPP_END2END_CONNECTION_ATTEMPT_INJECTOR_H

#include <memory>

#include "src/core/lib/event_engine/channel_args_endpoint_config.h"
#include "src/core/lib/iomgr/tcp_client.h"
#include "src/core/util/time.h"

namespace grpc {
namespace testing {

// Allows injecting connection-establishment delays into C-core.
// Typical usage:
//
//  // At grpc_init() time.
//  ConnectionAttemptInjector::Init();
//
//  // Instantiate when injection is desired.
//  ConnectionAttemptInjector injector;
//
//  // To inject a hold for the next connection attempt for a given port.
//  auto hold = injector.AddHold(port);
//  hold.Wait();
//  // ...do stuff...
//  hold.Resume();  // Or hold.Fail() if you want to force a failure.
//
//  // Inject a fixed delay for all connection attempts.
//  injector.SetDelay(grpc_core::Duration::Seconds(10));
//
// The injection is global, so there must be only one ConnectionAttemptInjector
// object at any one time.
//
// Note: This must be "final" to avoid tsan problems in both the ctor
// and dtor related to initializing the vtable.
class ConnectionAttemptInjector final {
 private:
  // Forward declarations.
  class QueuedAttempt;

  grpc_core::Mutex mu_;

 public:
  class Hold {
   public:
    // Do not instantiate directly -- must be created via AddHold().
    Hold(ConnectionAttemptInjector* injector, int port,
         bool intercept_completion);

    // Waits for the connection attempt to start.
    // After this returns, exactly one of Resume() or Fail() must be called.
    void Wait();

    // Resumes a connection attempt.  Must be called after Wait().
    void Resume();

    // Fails a connection attempt.  Must be called after Wait().
    void Fail(grpc_error_handle error);

    // If the hold was created with intercept_completion=true, then this
    // can be called after Resume() to wait for the connection attempt
    // to complete.
    void WaitForCompletion();

    // Returns true if the connection attempt has been started.
    bool IsStarted();

   private:
    friend class ConnectionAttemptInjector;

    static void OnComplete(void* arg, grpc_error_handle error);

    ConnectionAttemptInjector* injector_;
    const int port_;
    const bool intercept_completion_;
    std::unique_ptr<QueuedAttempt> queued_attempt_
        ABSL_GUARDED_BY(&ConnectionAttemptInjector::mu_);
    grpc_core::CondVar start_cv_;
    grpc_closure on_complete_;
    grpc_closure* original_on_complete_;
    grpc_core::CondVar complete_cv_;
  };

  // Global initializer.  Replaces the iomgr TCP client vtable.
  // Must be called exactly once after grpc_init() but before any TCP
  // connections are established.
  static void Init();

  ConnectionAttemptInjector();
  ~ConnectionAttemptInjector();

  // Adds a hold for a given port.  The caller may then use Wait() on
  // the resulting Hold object to wait for the connection attempt to start.
  // If intercept_completion is true, the caller can use WaitForCompletion()
  // on the resulting Hold object.
  std::unique_ptr<Hold> AddHold(int port, bool intercept_completion = false);

  // Set a fixed delay for all RPCs.  Will be used only if there is no
  // hold for the connection attempt.
  void SetDelay(grpc_core::Duration delay);

 private:
  static grpc_tcp_client_vtable kDelayedConnectVTable;

  // Represents a queued attempt.
  // The caller must invoke either Resume() or Fail() before destroying.
  class QueuedAttempt {
   public:
    QueuedAttempt(grpc_closure* closure, grpc_endpoint** ep,
                  grpc_pollset_set* interested_parties,
                  const grpc_event_engine::experimental::EndpointConfig& config,
                  const grpc_resolved_address* addr,
                  grpc_core::Timestamp deadline);
    ~QueuedAttempt();

    // Caller must invoke this from a thread with an ExecCtx.
    void Resume();

    // Caller must invoke this from a thread with an ExecCtx.
    void Fail(grpc_error_handle error);

   private:
    grpc_closure* closure_;
    grpc_endpoint** endpoint_;
    grpc_pollset_set* interested_parties_;
    grpc_event_engine::experimental::ChannelArgsEndpointConfig config_;
    grpc_resolved_address address_;
    grpc_core::Timestamp deadline_;
  };

  // Injects a delay before continuing a connection attempt.
  class InjectedDelay {
   public:
    virtual ~InjectedDelay() = default;

    InjectedDelay(grpc_core::Duration duration, grpc_closure* closure,
                  grpc_endpoint** ep, grpc_pollset_set* interested_parties,
                  const grpc_event_engine::experimental::EndpointConfig& config,
                  const grpc_resolved_address* addr,
                  grpc_core::Timestamp deadline);

   private:
    void TimerCallback();

    QueuedAttempt attempt_;
  };

  // Invoked for every TCP connection attempt.
  void HandleConnection(
      grpc_closure* closure, grpc_endpoint** ep,
      grpc_pollset_set* interested_parties,
      const grpc_event_engine::experimental::EndpointConfig& config,
      const grpc_resolved_address* addr, grpc_core::Timestamp deadline);

  static void AttemptConnection(
      grpc_closure* closure, grpc_endpoint** ep,
      grpc_pollset_set* interested_parties,
      const grpc_event_engine::experimental::EndpointConfig& config,
      const grpc_resolved_address* addr, grpc_core::Timestamp deadline);

  // Replacement iomgr tcp_connect vtable functions that use the current
  // ConnectionAttemptInjector object.
  static int64_t TcpConnect(
      grpc_closure* closure, grpc_endpoint** ep,
      grpc_pollset_set* interested_parties,
      const grpc_event_engine::experimental::EndpointConfig& config,
      const grpc_resolved_address* addr, grpc_core::Timestamp deadline);
  static bool TcpConnectCancel(int64_t connection_handle);

  std::vector<Hold*> holds_ ABSL_GUARDED_BY(&mu_);
  std::optional<grpc_core::Duration> delay_ ABSL_GUARDED_BY(&mu_);
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_END2END_CONNECTION_ATTEMPT_INJECTOR_H
