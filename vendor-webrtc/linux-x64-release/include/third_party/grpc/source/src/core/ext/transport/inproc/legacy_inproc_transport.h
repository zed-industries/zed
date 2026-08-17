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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_INPROC_LEGACY_INPROC_TRANSPORT_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_INPROC_LEGACY_INPROC_TRANSPORT_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include "src/core/lib/debug/trace.h"

grpc_channel* grpc_legacy_inproc_channel_create(grpc_server* server,
                                                const grpc_channel_args* args,
                                                void* reserved);

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_INPROC_LEGACY_INPROC_TRANSPORT_H
