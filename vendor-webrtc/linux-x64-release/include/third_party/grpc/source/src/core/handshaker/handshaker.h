//
//
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
//
//

#ifndef GRPC_SRC_CORE_HANDSHAKER_HANDSHAKER_H
#define GRPC_SRC_CORE_HANDSHAKER_HANDSHAKER_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <memory>

#include "absl/base/thread_annotations.h"
#include "absl/container/inlined_vector.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/tcp_server.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"

namespace grpc_core {

/// Handshakers are used to perform initial handshakes on a connection
/// before the client sends the initial request.  Some examples of what
/// a handshaker can be used for includes support for HTTP CONNECT on
/// the client side and various types of security initialization.
///
/// In general, handshakers should be used via a handshake manager.

/// Arguments passed through handshakers and back to the caller.
///
/// For handshakers, all members are input/output parameters; for
/// example, a handshaker may read from or write to \a endpoint and
/// then later replace it with a wrapped endpoint.  Similarly, a
/// handshaker may modify \a args.
///
/// A handshaker takes ownership of the members when this struct is
/// passed to DoHandshake().  It passes ownership back to the caller
/// when it invokes on_handshake_done.
struct HandshakerArgs {
  OrphanablePtr<grpc_endpoint> endpoint;
  ChannelArgs args;
  // Any bytes read from the endpoint that are not consumed by the
  // handshaker must be passed back via this buffer.
  SliceBuffer read_buffer;
  // A handshaker may set this to true before invoking on_handshake_done
  // to indicate that subsequent handshakers should be skipped.
  bool exit_early = false;
  // EventEngine to use for async work.
  // (This is just a convenience to avoid digging it out of args.)
  grpc_event_engine::experimental::EventEngine* event_engine = nullptr;
  // Deadline associated with the handshake.
  // TODO(anramach): Move this out of handshake args after EventEngine
  // is the default.
  Timestamp deadline;
  // TODO(roth): Make this go away somehow as part of the EventEngine
  // migration?
  grpc_tcp_server_acceptor* acceptor = nullptr;
};

///
/// Handshaker
///

class Handshaker : public RefCounted<Handshaker> {
 public:
  ~Handshaker() override = default;
  virtual absl::string_view name() const = 0;
  virtual void DoHandshake(
      HandshakerArgs* args,
      absl::AnyInvocable<void(absl::Status)> on_handshake_done) = 0;
  virtual void Shutdown(absl::Status error) = 0;

 protected:
  // Helper function to safely invoke on_handshake_done asynchronously.
  //
  // Note that on_handshake_done may complete in another thread as soon
  // as this method returns, so the handshaker object may be destroyed
  // by the callback unless the caller of this method is holding its own
  // ref to the handshaker.
  static void InvokeOnHandshakeDone(
      HandshakerArgs* args,
      absl::AnyInvocable<void(absl::Status)> on_handshake_done,
      absl::Status status);
};

//
// HandshakeManager
//

class HandshakeManager : public RefCounted<HandshakeManager> {
 public:
  HandshakeManager();

  /// Adds a handshaker to the handshake manager.
  /// Takes ownership of \a handshaker.
  void Add(RefCountedPtr<Handshaker> handshaker) ABSL_LOCKS_EXCLUDED(mu_);

  /// Invokes handshakers in the order they were added.
  /// Takes ownership of \a endpoint, and then passes that ownership to
  /// the \a on_handshake_done callback.
  /// Does NOT take ownership of \a channel_args.  Instead, makes a copy before
  /// invoking the first handshaker.
  /// \a acceptor will be nullptr for client-side handshakers.
  ///
  /// When done, invokes \a on_handshake_done with a HandshakerArgs
  /// object as its argument.  If the callback is invoked with error !=
  /// absl::OkStatus(), then handshaking failed and the handshaker has done
  /// the necessary clean-up.  Otherwise, the callback takes ownership of
  /// the arguments.
  void DoHandshake(OrphanablePtr<grpc_endpoint> endpoint,
                   const ChannelArgs& channel_args, Timestamp deadline,
                   grpc_tcp_server_acceptor* acceptor,
                   absl::AnyInvocable<void(absl::StatusOr<HandshakerArgs*>)>
                       on_handshake_done) ABSL_LOCKS_EXCLUDED(mu_);

  /// Shuts down the handshake manager (e.g., to clean up when the operation is
  /// aborted in the middle).
  void Shutdown(absl::Status error) ABSL_LOCKS_EXCLUDED(mu_);

 private:
  // A function used as the handshaker-done callback when chaining
  // handshakers together.
  void CallNextHandshakerLocked(absl::Status error)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  static const size_t kHandshakerListInlineSize = 2;

  Mutex mu_;
  bool is_shutdown_ ABSL_GUARDED_BY(mu_) = false;
  // The index of the handshaker to invoke next and closure to invoke it.
  size_t index_ ABSL_GUARDED_BY(mu_) = 0;
  // An array of handshakers added via Add().
  absl::InlinedVector<RefCountedPtr<Handshaker>, kHandshakerListInlineSize>
      handshakers_ ABSL_GUARDED_BY(mu_);
  // Handshaker args.
  HandshakerArgs args_ ABSL_GUARDED_BY(mu_);
  // The final callback to invoke after the last handshaker.
  absl::AnyInvocable<void(absl::StatusOr<HandshakerArgs*>)> on_handshake_done_
      ABSL_GUARDED_BY(mu_);
  // Deadline timer across all handshakers.
  grpc_event_engine::experimental::EventEngine::TaskHandle
      deadline_timer_handle_ ABSL_GUARDED_BY(mu_);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_HANDSHAKER_HANDSHAKER_H
