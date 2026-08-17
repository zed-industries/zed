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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_COMBINER_H
#define GRPC_SRC_CORE_LIB_IOMGR_COMBINER_H

#include <grpc/support/atm.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/exec_ctx.h"

namespace grpc_core {
// TODO(yashkt) : Remove this class and replace it with a class that does not
// use ExecCtx
class Combiner {
 public:
  void Run(grpc_closure* closure, grpc_error_handle error);
  // TODO(yashkt) : Remove this method
  void FinallyRun(grpc_closure* closure, grpc_error_handle error);
  // Force the next combiner execution to be offloaded
  void ForceOffload();
  Combiner* next_combiner_on_this_exec_ctx = nullptr;
  MultiProducerSingleConsumerQueue queue;
  // either:
  // a pointer to the initiating exec ctx if that is the only exec_ctx that has
  // ever queued to this combiner, or NULL. If this is non-null, it's not
  // dereferenceable (since the initiating exec_ctx may have gone out of scope)
  gpr_atm initiating_exec_ctx_or_null;
  // state is:
  // lower bit - zero if orphaned (STATE_UNORPHANED)
  // other bits - number of items queued on the lock (STATE_ELEM_COUNT_LOW_BIT)
  gpr_atm state;
  bool time_to_execute_final_list = false;
  grpc_closure_list final_list;
  // TODO(ctiller): delete this when the combiner_offload_to_event_engine
  // experiment is removed.
  grpc_closure offload;
  gpr_refcount refs;
  std::shared_ptr<grpc_event_engine::experimental::EventEngine> event_engine;
};
}  // namespace grpc_core

// Provides serialized access to some resource.
// Each action queued on a combiner is executed serially in a borrowed thread.
// The actual thread executing actions may change over time (but there will only
// ever be one at a time).

// Initialize the lock
grpc_core::Combiner* grpc_combiner_create(
    std::shared_ptr<grpc_event_engine::experimental::EventEngine> event_engine);

#ifndef NDEBUG
#define GRPC_COMBINER_DEBUG_ARGS \
  , const char *file, int line, const char *reason
#define GRPC_COMBINER_REF(combiner, reason) \
  grpc_combiner_ref((combiner), __FILE__, __LINE__, (reason))
#define GRPC_COMBINER_UNREF(combiner, reason) \
  grpc_combiner_unref((combiner), __FILE__, __LINE__, (reason))
#else
#define GRPC_COMBINER_DEBUG_ARGS
#define GRPC_COMBINER_REF(combiner, reason) grpc_combiner_ref((combiner))
#define GRPC_COMBINER_UNREF(combiner, reason) grpc_combiner_unref((combiner))
#endif

// Ref/unref the lock, for when we're sharing the lock ownership
// Prefer to use the macros above
grpc_core::Combiner* grpc_combiner_ref(
    grpc_core::Combiner* lock GRPC_COMBINER_DEBUG_ARGS);
void grpc_combiner_unref(grpc_core::Combiner* lock GRPC_COMBINER_DEBUG_ARGS);

bool grpc_combiner_continue_exec_ctx();

#endif  // GRPC_SRC_CORE_LIB_IOMGR_COMBINER_H
