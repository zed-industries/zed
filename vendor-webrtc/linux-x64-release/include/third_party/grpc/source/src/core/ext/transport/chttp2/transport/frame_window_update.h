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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FRAME_WINDOW_UPDATE_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FRAME_WINDOW_UPDATE_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include "src/core/ext/transport/chttp2/transport/legacy_frame.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/telemetry/call_tracer.h"

struct grpc_chttp2_window_update_parser {
  uint8_t byte;
  uint8_t is_connection_update;
  uint32_t amount;
};
grpc_slice grpc_chttp2_window_update_create(
    uint32_t id, uint32_t window_delta,
    grpc_core::CallTracerInterface* call_tracer);

grpc_error_handle grpc_chttp2_window_update_parser_begin_frame(
    grpc_chttp2_window_update_parser* parser, uint32_t length, uint8_t flags);
grpc_error_handle grpc_chttp2_window_update_parser_parse(
    void* parser, grpc_chttp2_transport* t, grpc_chttp2_stream* s,
    const grpc_slice& slice, int is_last);

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FRAME_WINDOW_UPDATE_H
