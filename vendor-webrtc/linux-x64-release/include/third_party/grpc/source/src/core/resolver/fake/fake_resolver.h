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

#ifndef GRPC_SRC_CORE_RESOLVER_FAKE_FAKE_RESOLVER_H
#define GRPC_SRC_CORE_RESOLVER_FAKE_FAKE_RESOLVER_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include <optional>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/strings/string_view.h"
#include "absl/time/time.h"
#include "src/core/resolver/resolver.h"
#include "src/core/util/notification.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/useful.h"

#define GRPC_ARG_FAKE_RESOLVER_RESPONSE_GENERATOR \
  "grpc.fake_resolver.response_generator"

namespace grpc_core {

class FakeResolver;

/// A mechanism for generating responses for the fake resolver.
/// An instance of this class is passed to the fake resolver via a channel
/// argument and used to inject and trigger custom resolutions.
// TODO(roth): I would ideally like this to be InternallyRefCounted
// instead of RefCounted, but external refs are currently needed to
// encode this in channel args.  Once channel_args are converted to C++,
// see if we can find a way to fix this.
class FakeResolverResponseGenerator final
    : public RefCounted<FakeResolverResponseGenerator> {
 public:
  static const grpc_arg_pointer_vtable kChannelArgPointerVtable;

  FakeResolverResponseGenerator();
  ~FakeResolverResponseGenerator() override;

  // Instructs the fake resolver associated with the response generator
  // instance to trigger a new resolution with the specified result. If the
  // resolver is not available yet, delays response setting until it is. This
  // can be called at most once before the resolver is available.
  // notify_when_set is an optional notification to signal when the response has
  // been set.
  void SetResponseAndNotify(Resolver::Result result,
                            Notification* notify_when_set);

  // Same as SetResponseAndNotify(), assume that async setting is fine
  void SetResponseAsync(Resolver::Result result) {
    SetResponseAndNotify(std::move(result), nullptr);
  }

  // Same as SetResponseAndNotify(), but create and wait for the notification
  void SetResponseSynchronously(Resolver::Result result) {
    Notification n;
    SetResponseAndNotify(std::move(result), &n);
    n.WaitForNotification();
  }

  // Waits up to timeout for a re-resolution request.  Returns true if a
  // re-resolution request is seen, or false if timeout occurs.  Returns
  // true immediately if there was a re-resolution request since the
  // last time this method was called.
  bool WaitForReresolutionRequest(absl::Duration timeout);

  // Wait for a resolver to be set (setting may be happening asynchronously, so
  // this may block - consider it test only).
  bool WaitForResolverSet(absl::Duration timeout);

  static absl::string_view ChannelArgName() {
    return GRPC_ARG_FAKE_RESOLVER_RESPONSE_GENERATOR;
  }

  static int ChannelArgsCompare(const FakeResolverResponseGenerator* a,
                                const FakeResolverResponseGenerator* b) {
    return QsortCompare(a, b);
  }

 private:
  friend class FakeResolver;

  // Set the corresponding FakeResolver to this generator.
  void SetFakeResolver(RefCountedPtr<FakeResolver> resolver);

  // Called by FakeResolver when re-resolution is requested.
  void ReresolutionRequested();

  // Helper function to send a result to the resolver.
  static void SendResultToResolver(RefCountedPtr<FakeResolver> resolver,
                                   Resolver::Result result,
                                   Notification* notify_when_set);

  // Mutex protecting the members below.
  Mutex mu_;
  CondVar* resolver_set_cv_ ABSL_GUARDED_BY(mu_) = nullptr;
  RefCountedPtr<FakeResolver> resolver_ ABSL_GUARDED_BY(mu_);
  // Temporarily stores the result when it gets set before the response
  // generator is seen by the FakeResolver.
  std::optional<Resolver::Result> result_ ABSL_GUARDED_BY(mu_);

  Mutex reresolution_mu_;
  CondVar* reresolution_cv_ ABSL_GUARDED_BY(reresolution_mu_) = nullptr;
  bool reresolution_requested_ ABSL_GUARDED_BY(reresolution_mu_) = false;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_RESOLVER_FAKE_FAKE_RESOLVER_H
