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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FRAME_GOAWAY_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FRAME_GOAWAY_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include "src/core/ext/transport/chttp2/transport/legacy_frame.h"
#include "src/core/lib/iomgr/error.h"

typedef enum {
  GRPC_CHTTP2_GOAWAY_LSI0,
  GRPC_CHTTP2_GOAWAY_LSI1,
  GRPC_CHTTP2_GOAWAY_LSI2,
  GRPC_CHTTP2_GOAWAY_LSI3,
  GRPC_CHTTP2_GOAWAY_ERR0,
  GRPC_CHTTP2_GOAWAY_ERR1,
  GRPC_CHTTP2_GOAWAY_ERR2,
  GRPC_CHTTP2_GOAWAY_ERR3,
  GRPC_CHTTP2_GOAWAY_DEBUG
} grpc_chttp2_goaway_parse_state;

struct grpc_chttp2_goaway_parser {
  grpc_chttp2_goaway_parse_state state;
  uint32_t last_stream_id;
  uint32_t error_code;
  char* debug_data;
  uint32_t debug_length;
  uint32_t debug_pos;
};
void grpc_chttp2_goaway_parser_init(grpc_chttp2_goaway_parser* p);
void grpc_chttp2_goaway_parser_destroy(grpc_chttp2_goaway_parser* p);
grpc_error_handle grpc_chttp2_goaway_parser_begin_frame(
    grpc_chttp2_goaway_parser* parser, uint32_t length, uint8_t flags);
grpc_error_handle grpc_chttp2_goaway_parser_parse(void* parser,
                                                  grpc_chttp2_transport* t,
                                                  grpc_chttp2_stream* s,
                                                  const grpc_slice& slice,
                                                  int is_last);

void grpc_chttp2_goaway_append(uint32_t last_stream_id, uint32_t error_code,
                               const grpc_slice& debug_data,
                               grpc_slice_buffer* slice_buffer);

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FRAME_GOAWAY_H
