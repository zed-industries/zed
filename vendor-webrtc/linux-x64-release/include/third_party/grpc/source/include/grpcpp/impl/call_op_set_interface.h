//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPCPP_IMPL_CALL_OP_SET_INTERFACE_H
#define GRPCPP_IMPL_CALL_OP_SET_INTERFACE_H

// IWYU pragma: private

#include <grpcpp/impl/completion_queue_tag.h>

namespace grpc {
namespace internal {

class Call;

/// An abstract collection of call ops, used to generate the
/// grpc_call_op structure to pass down to the lower layers,
/// and as it is-a CompletionQueueTag, also massages the final
/// completion into the correct form for consumption in the C++
/// API.
class CallOpSetInterface : public CompletionQueueTag {
 public:
  /// Fills in grpc_op, starting from ops[*nops] and moving
  /// upwards.
  virtual void FillOps(internal::Call* call) = 0;

  /// Get the tag to be used at the core completion queue. Generally, the
  /// value of core_cq_tag will be "this". However, it can be overridden if we
  /// want core to process the tag differently (e.g., as a core callback)
  virtual void* core_cq_tag() = 0;

  // This will be called while interceptors are run if the RPC is a hijacked
  // RPC. This should set hijacking state for each of the ops.
  virtual void SetHijackingState() = 0;

  // Should be called after interceptors are done running
  virtual void ContinueFillOpsAfterInterception() = 0;

  // Should be called after interceptors are done running on the finalize result
  // path
  virtual void ContinueFinalizeResultAfterInterception() = 0;
};
}  // namespace internal
}  // namespace grpc

#endif  // GRPCPP_IMPL_CALL_OP_SET_INTERFACE_H
