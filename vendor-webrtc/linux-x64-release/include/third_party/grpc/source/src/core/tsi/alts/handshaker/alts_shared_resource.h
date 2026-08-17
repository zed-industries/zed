//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_TSI_ALTS_HANDSHAKER_ALTS_SHARED_RESOURCE_H
#define GRPC_SRC_CORE_TSI_ALTS_HANDSHAKER_ALTS_SHARED_RESOURCE_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/sync.h>

#include "src/core/lib/iomgr/pollset_set.h"
#include "src/core/lib/surface/completion_queue.h"
#include "src/core/util/thd.h"

///
/// Main struct containing ALTS shared resources used when
/// employing the dedicated completion queue and thread.
///
typedef struct alts_shared_resource_dedicated {
  grpc_core::Thread thread;
  grpc_completion_queue* cq;
  grpc_pollset_set* interested_parties;
  grpc_cq_completion storage;
  gpr_mu mu;
  grpc_channel* channel;
} alts_shared_resource_dedicated;

// This method returns the address of alts_shared_resource_dedicated
// object shared by all TSI handshakes.
//
alts_shared_resource_dedicated* grpc_alts_get_shared_resource_dedicated(void);

///
/// This method destroys the alts_shared_resource_dedicated object
/// shared by all TSI handshakes. The application is responsible for
/// invoking the API before calling grpc_shutdown().
///
void grpc_alts_shared_resource_dedicated_shutdown();

///
/// This method initializes the alts_shared_resource_dedicated object
/// shared by all TSI handshakes. The application is responsible for
/// invoking the API after calling grpc_init();
///
void grpc_alts_shared_resource_dedicated_init();

///
/// This method populates various fields of the alts_shared_resource_dedicated
/// object shared by all TSI handshakes and start the dedicated thread.
/// The API will be invoked by the caller in a lazy manner. That is,
/// it will get invoked when ALTS TSI handshake occurs for the first time.
///
void grpc_alts_shared_resource_dedicated_start(
    const char* handshaker_service_url);

#endif  // GRPC_SRC_CORE_TSI_ALTS_HANDSHAKER_ALTS_SHARED_RESOURCE_H
