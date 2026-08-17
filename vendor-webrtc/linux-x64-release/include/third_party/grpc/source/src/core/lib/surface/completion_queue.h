//
//
// Copyright 2015-2016 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_SURFACE_COMPLETION_QUEUE_H
#define GRPC_SRC_CORE_LIB_SURFACE_COMPLETION_QUEUE_H

// Internal API for completion queues

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/util/manual_constructor.h"
#include "src/core/util/mpscq.h"

typedef struct grpc_cq_completion {
  grpc_core::ManualConstructor<
      grpc_core::MultiProducerSingleConsumerQueue::Node>
      node;

  /// user supplied tag
  void* tag;
  /// done callback - called when this queue element is no longer
  /// needed by the completion queue
  void (*done)(void* done_arg, struct grpc_cq_completion* c);
  void* done_arg;
  /// next pointer; low bit is used to indicate success or not
  uintptr_t next;
} grpc_cq_completion;

#ifndef NDEBUG
void grpc_cq_internal_ref(grpc_completion_queue* cq, const char* reason,
                          const char* file, int line);
void grpc_cq_internal_unref(grpc_completion_queue* cq, const char* reason,
                            const char* file, int line);
#define GRPC_CQ_INTERNAL_REF(cq, reason) \
  grpc_cq_internal_ref(cq, reason, __FILE__, __LINE__)
#define GRPC_CQ_INTERNAL_UNREF(cq, reason) \
  grpc_cq_internal_unref(cq, reason, __FILE__, __LINE__)
#else
void grpc_cq_internal_ref(grpc_completion_queue* cq);
void grpc_cq_internal_unref(grpc_completion_queue* cq);
#define GRPC_CQ_INTERNAL_REF(cq, reason) grpc_cq_internal_ref(cq)
#define GRPC_CQ_INTERNAL_UNREF(cq, reason) grpc_cq_internal_unref(cq)
#endif

// Flag that an operation is beginning: the completion channel will not finish
// shutdown until a corresponding grpc_cq_end_* call is made.
// \a tag is currently used only in debug builds. Return true on success, and
// false if completion_queue has been shutdown.
bool grpc_cq_begin_op(grpc_completion_queue* cq, void* tag);

// Queue a GRPC_OP_COMPLETED operation; tag must correspond to the tag passed to
// grpc_cq_begin_op
void grpc_cq_end_op(grpc_completion_queue* cq, void* tag,
                    grpc_error_handle error,
                    void (*done)(void* done_arg, grpc_cq_completion* storage),
                    void* done_arg, grpc_cq_completion* storage,
                    bool internal = false);

grpc_pollset* grpc_cq_pollset(grpc_completion_queue* cq);

bool grpc_cq_can_listen(grpc_completion_queue* cq);

grpc_cq_completion_type grpc_get_cq_completion_type(grpc_completion_queue* cq);

int grpc_get_cq_poll_num(grpc_completion_queue* cq);

grpc_completion_queue* grpc_completion_queue_create_internal(
    grpc_cq_completion_type completion_type, grpc_cq_polling_type polling_type,
    grpc_completion_queue_functor* shutdown_callback);

#endif  // GRPC_SRC_CORE_LIB_SURFACE_COMPLETION_QUEUE_H
