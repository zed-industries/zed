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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_LEGACY_FRAME_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_LEGACY_FRAME_H

#include <grpc/support/port_platform.h>

// defined in internal.h
typedef struct grpc_chttp2_stream grpc_chttp2_stream;
typedef struct grpc_chttp2_transport grpc_chttp2_transport;

#define GRPC_CHTTP2_FRAME_DATA 0
#define GRPC_CHTTP2_FRAME_HEADER 1
#define GRPC_CHTTP2_FRAME_CONTINUATION 9
#define GRPC_CHTTP2_FRAME_RST_STREAM 3
#define GRPC_CHTTP2_FRAME_SETTINGS 4
#define GRPC_CHTTP2_FRAME_PING 6
#define GRPC_CHTTP2_FRAME_GOAWAY 7
#define GRPC_CHTTP2_FRAME_WINDOW_UPDATE 8
#define GRPC_CHTTP2_FRAME_SECURITY 200

#define GRPC_CHTTP2_DATA_FLAG_END_STREAM 1
#define GRPC_CHTTP2_FLAG_ACK 1
#define GRPC_CHTTP2_DATA_FLAG_END_HEADERS 4
#define GRPC_CHTTP2_DATA_FLAG_PADDED 8
#define GRPC_CHTTP2_FLAG_HAS_PRIORITY 0x20

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_LEGACY_FRAME_H
