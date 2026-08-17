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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_TIMER_HEAP_H
#define GRPC_SRC_CORE_LIB_IOMGR_TIMER_HEAP_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/timer.h"

struct grpc_timer_heap {
  grpc_timer** timers;
  uint32_t timer_count;
  uint32_t timer_capacity;
};
// return true if the new timer is the first timer in the heap
bool grpc_timer_heap_add(grpc_timer_heap* heap, grpc_timer* timer);

void grpc_timer_heap_init(grpc_timer_heap* heap);
void grpc_timer_heap_destroy(grpc_timer_heap* heap);

void grpc_timer_heap_remove(grpc_timer_heap* heap, grpc_timer* timer);
grpc_timer* grpc_timer_heap_top(grpc_timer_heap* heap);
void grpc_timer_heap_pop(grpc_timer_heap* heap);

bool grpc_timer_heap_is_empty(grpc_timer_heap* heap);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_TIMER_HEAP_H
