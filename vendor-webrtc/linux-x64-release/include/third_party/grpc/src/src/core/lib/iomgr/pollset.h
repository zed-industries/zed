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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_POLLSET_H
#define GRPC_SRC_CORE_LIB_IOMGR_POLLSET_H

#include <grpc/support/port_platform.h>
#include <grpc/support/sync.h>
#include <grpc/support/time.h>

#include "src/core/lib/iomgr/exec_ctx.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"

// A grpc_pollset is a set of file descriptors that a higher level item is
// interested in. For example:
//  - a server will typically keep a pollset containing all connected channels,
//    so that it can find new calls to service
//  - a completion queue might keep a pollset with an entry for each transport
//    that is servicing a call that it's tracking

typedef struct grpc_pollset_worker grpc_pollset_worker;

typedef struct grpc_pollset_vtable {
  void (*global_init)(void);
  void (*global_shutdown)(void);
  void (*init)(grpc_pollset* pollset, gpr_mu** mu);
  void (*shutdown)(grpc_pollset* pollset, grpc_closure* closure);
  void (*destroy)(grpc_pollset* pollset);
  grpc_error_handle (*work)(grpc_pollset* pollset, grpc_pollset_worker** worker,
                            grpc_core::Timestamp deadline);
  grpc_error_handle (*kick)(grpc_pollset* pollset,
                            grpc_pollset_worker* specific_worker);
  size_t (*pollset_size)(void);
} grpc_pollset_vtable;

void grpc_set_pollset_vtable(grpc_pollset_vtable* vtable);

void grpc_pollset_global_init(void);
void grpc_pollset_global_shutdown(void);

size_t grpc_pollset_size(void);
// Initialize a pollset: assumes *pollset contains all zeros
void grpc_pollset_init(grpc_pollset* pollset, gpr_mu** mu);
// Begin shutting down the pollset, and call closure when done.
// pollset's mutex must be held
void grpc_pollset_shutdown(grpc_pollset* pollset, grpc_closure* closure);
void grpc_pollset_destroy(grpc_pollset* pollset);

// Do some work on a pollset.
// May involve invoking asynchronous callbacks, or actually polling file
// descriptors.
// Requires pollset's mutex locked.
// May unlock its mutex during its execution.

// worker is a (platform-specific) handle that can be used to wake up
// from grpc_pollset_work before any events are received and before the timeout
// has expired. It is both initialized and destroyed by grpc_pollset_work.
// Initialization of worker is guaranteed to occur BEFORE the
// pollset's mutex is released for the first time by grpc_pollset_work
// and it is guaranteed that it will not be released by grpc_pollset_work
// AFTER worker has been destroyed.

// It's legal for worker to be NULL: in that case, this specific thread can not
// be directly woken with a kick, but maybe be indirectly (with a kick against
// the pollset as a whole).

// Tries not to block past deadline.
// May call grpc_closure_list_run on grpc_closure_list, without holding the
// pollset
// lock
grpc_error_handle grpc_pollset_work(grpc_pollset* pollset,
                                    grpc_pollset_worker** worker,
                                    grpc_core::Timestamp deadline);

// Break one polling thread out of polling work for this pollset.
// If specific_worker is non-NULL, then kick that worker.
grpc_error_handle grpc_pollset_kick(grpc_pollset* pollset,
                                    grpc_pollset_worker* specific_worker);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_POLLSET_H
