//
//
// Copyright 2015 gRPC authors.
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

#ifndef GRPCPP_IMPL_COMPLETION_QUEUE_TAG_H
#define GRPCPP_IMPL_COMPLETION_QUEUE_TAG_H

namespace grpc {

namespace internal {
/// An interface allowing implementors to process and filter event tags.
class CompletionQueueTag {
 public:
  virtual ~CompletionQueueTag() {}

  /// FinalizeResult must be called before informing user code that the
  /// operation bound to the underlying core completion queue tag has
  /// completed. In practice, this means:
  ///
  ///   1. For the sync API - before returning from Pluck
  ///   2. For the CQ-based async API - before returning from Next
  ///   3. For the callback-based API - before invoking the user callback
  ///
  /// This is the method that translates from core-side tag/status to
  /// C++ API-observable tag/status.
  ///
  /// The return value is the status of the operation (returning status is the
  /// general behavior of this function). If this function returns false, the
  /// tag is dropped and not returned from the completion queue: this concept is
  /// for events that are observed at core but not requested by the user
  /// application (e.g., server shutdown, for server unimplemented method
  /// responses, or for cases where a server-side RPC doesn't have a completion
  /// notification registered using AsyncNotifyWhenDone)
  virtual bool FinalizeResult(void** tag, bool* status) = 0;
};
}  // namespace internal

}  // namespace grpc

#endif  // GRPCPP_IMPL_COMPLETION_QUEUE_TAG_H
