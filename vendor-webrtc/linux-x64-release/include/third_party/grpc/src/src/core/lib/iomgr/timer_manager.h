//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_TIMER_MANAGER_H
#define GRPC_SRC_CORE_LIB_IOMGR_TIMER_MANAGER_H

#include <grpc/support/port_platform.h>
#include <stdbool.h>

// Timer Manager tries to keep only one thread waiting for the next timeout at
// all times, and thus effectively preventing the thundering herd problem.

void grpc_timer_manager_init(void);
void grpc_timer_manager_shutdown(void);

// enable/disable threading - must be called after grpc_timer_manager_init and
// before grpc_timer_manager_shutdown
void grpc_timer_manager_set_threading(bool enabled);
// enable/disable threading - must be called before first grpc init
void grpc_timer_manager_set_start_threaded(bool enabled);
// explicitly perform one tick of the timer system - for when threading is
// disabled
void grpc_timer_manager_tick(void);
// get global counter that tracks timer wakeups
uint64_t grpc_timer_manager_get_wakeups_testonly(void);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_TIMER_MANAGER_H
