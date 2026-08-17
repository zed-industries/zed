//
//
// Copyright 2020 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_EV_APPLE_H
#define GRPC_SRC_CORE_LIB_IOMGR_EV_APPLE_H

#include <grpc/support/port_platform.h>

#ifdef GRPC_APPLE_EV

#include <CoreFoundation/CoreFoundation.h>

#include "src/core/lib/iomgr/pollset.h"
#include "src/core/lib/iomgr/pollset_set.h"

void grpc_apple_register_read_stream(CFReadStreamRef read_stream,
                                     dispatch_queue_t dispatch_queue);

void grpc_apple_register_write_stream(CFWriteStreamRef write_stream,
                                      dispatch_queue_t dispatch_queue);

extern grpc_pollset_vtable grpc_apple_pollset_vtable;

extern grpc_pollset_set_vtable grpc_apple_pollset_set_vtable;

#endif

#endif  // GRPC_SRC_CORE_LIB_IOMGR_EV_APPLE_H
