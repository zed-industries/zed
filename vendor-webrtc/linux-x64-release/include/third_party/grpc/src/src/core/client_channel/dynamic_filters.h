//
// Copyright 2020 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_DYNAMIC_FILTERS_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_DYNAMIC_FILTERS_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>

#include <utility>
#include <vector>

#include "src/core/filter/blackboard.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/channel_stack.h"
#include "src/core/lib/iomgr/call_combiner.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/polling_entity.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/time.h"
#include "src/core/util/time_precise.h"

namespace grpc_core {

class DynamicFilters final : public RefCounted<DynamicFilters> {
 public:
  // Implements the interface of RefCounted<>.
  class Call {
   public:
    struct Args {
      RefCountedPtr<DynamicFilters> channel_stack;
      grpc_polling_entity* pollent;
      gpr_cycle_counter start_time;
      Timestamp deadline;
      Arena* arena;
      CallCombiner* call_combiner;
    };

    Call(Args args, grpc_error_handle* error);

    // Continues processing a transport stream op batch.
    void StartTransportStreamOpBatch(grpc_transport_stream_op_batch* batch);

    // Sets the 'then_schedule_closure' argument for call stack destruction.
    // Must be called once per call.
    void SetAfterCallStackDestroy(grpc_closure* closure);

    // Interface of RefCounted<>.
    GRPC_MUST_USE_RESULT RefCountedPtr<Call> Ref();
    GRPC_MUST_USE_RESULT RefCountedPtr<Call> Ref(const DebugLocation& location,
                                                 const char* reason);
    // When refcount drops to 0, destroys itself and the associated call stack,
    // but does NOT free the memory because it's in the call arena.
    void Unref();
    void Unref(const DebugLocation& location, const char* reason);

   private:
    // Allow RefCountedPtr<> to access IncrementRefCount().
    template <typename T>
    friend class RefCountedPtr;

    // Interface of RefCounted<>.
    void IncrementRefCount();
    void IncrementRefCount(const DebugLocation& location, const char* reason);

    static void Destroy(void* arg, grpc_error_handle error);

    RefCountedPtr<DynamicFilters> channel_stack_;
    grpc_closure* after_call_stack_destroy_ = nullptr;
  };

  static RefCountedPtr<DynamicFilters> Create(
      const ChannelArgs& args, std::vector<const grpc_channel_filter*> filters,
      const Blackboard* old_blackboard, Blackboard* new_blackboard);

  explicit DynamicFilters(RefCountedPtr<grpc_channel_stack> channel_stack)
      : channel_stack_(std::move(channel_stack)) {}

  RefCountedPtr<Call> CreateCall(Call::Args args, grpc_error_handle* error);

  grpc_channel_stack* channel_stack() const { return channel_stack_.get(); }

 private:
  RefCountedPtr<grpc_channel_stack> channel_stack_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_DYNAMIC_FILTERS_H
