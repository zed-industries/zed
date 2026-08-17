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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_TIMER_H
#define GRPC_SRC_CORE_LIB_IOMGR_TIMER_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>

#include <cstdint>

#include "src/core/lib/iomgr/exec_ctx.h"
#include "src/core/lib/iomgr/iomgr.h"
#include "src/core/lib/iomgr/port.h"

typedef struct grpc_timer {
  int64_t deadline;
  // Uninitialized if not using heap, or INVALID_HEAP_INDEX if not in heap.
  uint32_t heap_index;
  bool pending;
  struct grpc_timer* next;
  struct grpc_timer* prev;
  grpc_closure* closure;
#ifndef NDEBUG
  struct grpc_timer* hash_table_next;
#endif

  // Optional field used by custom timers
  union {
    void* custom_timer;
    grpc_event_engine::experimental::EventEngine::TaskHandle ee_task_handle;
  };
} grpc_timer;

static_assert(std::is_trivial<grpc_timer>::value,
              "grpc_timer is expected to be a trivial type");

typedef enum {
  GRPC_TIMERS_NOT_CHECKED,
  GRPC_TIMERS_CHECKED_AND_EMPTY,
  GRPC_TIMERS_FIRED,
} grpc_timer_check_result;

typedef struct grpc_timer_vtable {
  void (*init)(grpc_timer* timer, grpc_core::Timestamp, grpc_closure* closure);
  void (*cancel)(grpc_timer* timer);

  // Internal API
  grpc_timer_check_result (*check)(grpc_core::Timestamp* next);
  void (*list_init)();
  void (*list_shutdown)(void);
  void (*consume_kick)(void);
} grpc_timer_vtable;

// Initialize *timer. When expired or canceled, closure will be called with
// error set to indicate if it expired (absl::OkStatus()) or was canceled
// (absl::CancelledError()). *closure is guaranteed to be called exactly once,
// and application code should check the error to determine how it was invoked.
// The application callback is also responsible for maintaining information
// about when to free up any user-level state. Behavior is undefined for a
// deadline of grpc_core::Timestamp::InfFuture().
void grpc_timer_init(grpc_timer* timer, grpc_core::Timestamp deadline,
                     grpc_closure* closure);

// Initialize *timer without setting it. This can later be passed through
// the regular init or cancel
void grpc_timer_init_unset(grpc_timer* timer);

// Note that there is no timer destroy function. This is because the
// timer is a one-time occurrence with a guarantee that the callback will
// be called exactly once, either at expiration or cancellation. Thus, all
// the internal timer event management state is destroyed just before
// that callback is invoked. If the user has additional state associated with
// the timer, the user is responsible for determining when it is safe to
// destroy that state.

// Cancel an *timer.
// There are three cases:
// 1. We normally cancel the timer
// 2. The timer has already run
// 3. We can't cancel the timer because it is "in flight".

// In all of these cases, the cancellation is still considered successful.
// They are essentially distinguished in that the timer_cb will be run
// exactly once from either the cancellation (with error absl::CancelledError())
// or from the activation (with error absl::OkStatus()).

// Note carefully that the callback function MAY occur in the same callstack
// as grpc_timer_cancel. It's expected that most timers will be cancelled (their
// primary use is to implement deadlines), and so this code is optimized such
// that cancellation costs as little as possible. Making callbacks run inline
// matches this aim.

// Requires: cancel() must happen after init() on a given timer
void grpc_timer_cancel(grpc_timer* timer);

// iomgr internal api for dealing with timers

// Check for timers to be run, and run them.
// Return true if timer callbacks were executed.
// If next is non-null, TRY to update *next with the next running timer
// IF that timer occurs before *next current value.
// *next is never guaranteed to be updated on any given execution; however,
// with high probability at least one thread in the system will see an update
// at any time slice.
grpc_timer_check_result grpc_timer_check(grpc_core::Timestamp* next);
void grpc_timer_list_init();
void grpc_timer_list_shutdown();

// Consume a kick issued by grpc_kick_poller
void grpc_timer_consume_kick(void);

// the following must be implemented by each iomgr implementation
void grpc_kick_poller(void);

// Sets the timer implementation
void grpc_set_timer_impl(grpc_timer_vtable* vtable);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_TIMER_H
