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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_IOMGR_H
#define GRPC_SRC_CORE_LIB_IOMGR_IOMGR_H

#include <grpc/support/port_platform.h>
#include <stdlib.h>

#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/port.h"

/// Initializes the iomgr.
void grpc_iomgr_init();

/// Starts any background threads for iomgr.
void grpc_iomgr_start();

/// Signals the intention to shutdown the iomgr. Expects to be able to flush
/// exec_ctx.
void grpc_iomgr_shutdown();

/// Signals the intention to shutdown all the closures registered in the
/// background poller.
void grpc_iomgr_shutdown_background_closure();

// Returns true if polling engine runs in the background, false otherwise.
// Currently only 'epollbg' runs in the background.
//
bool grpc_iomgr_run_in_background();

/// Returns true if the caller is a worker thread for any background poller.
bool grpc_iomgr_is_any_background_poller_thread();

/// Returns true if the closure is registered into the background poller. Note
/// that the closure may or may not run yet when this function returns, and the
/// closure should not be blocking or long-running.
bool grpc_iomgr_add_closure_to_background_poller(grpc_closure* closure,
                                                 grpc_error_handle error);

// Exposed only for testing
size_t grpc_iomgr_count_objects_for_testing();

#endif  // GRPC_SRC_CORE_LIB_IOMGR_IOMGR_H
