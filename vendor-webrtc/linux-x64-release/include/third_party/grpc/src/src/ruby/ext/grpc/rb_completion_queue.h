/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#ifndef GRPC_RB_COMPLETION_QUEUE_H_
#define GRPC_RB_COMPLETION_QUEUE_H_

#include <ruby/ruby.h>

#include <grpc/grpc.h>

void grpc_rb_completion_queue_destroy(grpc_completion_queue* cq);

/**
 * Makes the implementation of CompletionQueue#pluck available in other files
 *
 * This avoids having code that holds the GIL repeated at multiple sites.
 *
 * unblock_func is invoked with the provided argument to unblock the CQ
 * operation in the event of process termination (e.g. a signal), but
 * unblock_func may be NULL in which case it's unused.
 */
grpc_event rb_completion_queue_pluck(grpc_completion_queue* queue, void* tag,
                                     gpr_timespec deadline,
                                     void (*unblock_func)(void* param),
                                     void* unblock_func_arg);

#endif /* GRPC_RB_COMPLETION_QUEUE_H_ */
