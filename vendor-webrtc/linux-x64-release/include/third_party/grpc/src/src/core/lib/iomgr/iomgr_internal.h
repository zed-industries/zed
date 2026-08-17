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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_IOMGR_INTERNAL_H
#define GRPC_SRC_CORE_LIB_IOMGR_IOMGR_INTERNAL_H

#include <grpc/support/port_platform.h>
#include <stdbool.h>

#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"

typedef struct grpc_iomgr_object {
  char* name;
  struct grpc_iomgr_object* next;
  struct grpc_iomgr_object* prev;
} grpc_iomgr_object;

typedef struct grpc_iomgr_platform_vtable {
  void (*init)(void);
  void (*flush)(void);
  void (*shutdown)(void);
  void (*shutdown_background_closure)(void);
  bool (*is_any_background_poller_thread)(void);
  bool (*add_closure_to_background_poller)(grpc_closure* closure,
                                           grpc_error_handle error);
} grpc_iomgr_platform_vtable;

void grpc_iomgr_register_object(grpc_iomgr_object* obj, const char* name);
void grpc_iomgr_unregister_object(grpc_iomgr_object* obj);

bool grpc_have_determined_iomgr_platform();

void grpc_set_iomgr_platform_vtable(grpc_iomgr_platform_vtable* vtable);

void grpc_set_default_iomgr_platform();

void grpc_iomgr_platform_init(void);
/// flush any globally queued work from iomgr
void grpc_iomgr_platform_flush(void);
/// tear down all platform specific global iomgr structures
void grpc_iomgr_platform_shutdown(void);

/// shut down all the closures registered in the background poller
void grpc_iomgr_platform_shutdown_background_closure(void);

/// return true if the caller is a worker thread for any background poller
bool grpc_iomgr_platform_is_any_background_poller_thread(void);

/// Return true if the closure is registered into the background poller. Note
/// that the closure may or may not run yet when this function returns, and the
/// closure should not be blocking or long-running.
bool grpc_iomgr_platform_add_closure_to_background_poller(
    grpc_closure* closure, grpc_error_handle error);

bool grpc_iomgr_abort_on_leaks(void);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_IOMGR_INTERNAL_H
