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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_ENDPOINT_CFSTREAM_H
#define GRPC_SRC_CORE_LIB_IOMGR_ENDPOINT_CFSTREAM_H
//
// Low level TCP "bottom half" implementation, for use by transports built on
// top of a TCP connection.

// Note that this file does not (yet) include APIs for creating the socket in
// the first place.

// All calls passing slice transfer ownership of a slice refcount unless
// otherwise specified.
//

#include <grpc/support/port_platform.h>

#ifdef GRPC_CFSTREAM

#import <CoreFoundation/CoreFoundation.h>

#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/cfstream_handle.h"
#include "src/core/lib/iomgr/endpoint.h"

grpc_endpoint* grpc_cfstream_endpoint_create(CFReadStreamRef read_stream,
                                             CFWriteStreamRef write_stream,
                                             const char* peer_string,
                                             CFStreamHandle* stream_sync);

#endif  // GRPC_CFSTREAM

#endif  // GRPC_SRC_CORE_LIB_IOMGR_ENDPOINT_CFSTREAM_H
