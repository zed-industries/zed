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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FRAME_DATA_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FRAME_DATA_H

// Parser for GRPC streams embedded in DATA frames

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include "absl/status/status.h"
#include "src/core/ext/transport/chttp2/transport/legacy_frame.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/telemetry/call_tracer.h"

// start processing a new data frame
absl::Status grpc_chttp2_data_parser_begin_frame(uint8_t flags,
                                                 uint32_t stream_id,
                                                 grpc_chttp2_stream* s);

// handle a slice of a data frame - is_last indicates the last slice of a
// frame
grpc_error_handle grpc_chttp2_data_parser_parse(void* parser,
                                                grpc_chttp2_transport* t,
                                                grpc_chttp2_stream* s,
                                                const grpc_slice& slice,
                                                int is_last);

void grpc_chttp2_encode_data(uint32_t id, grpc_slice_buffer* inbuf,
                             uint32_t write_bytes, int is_eof,
                             grpc_core::CallTracerInterface* call_tracer,
                             grpc_slice_buffer* outbuf);

grpc_core::Poll<grpc_error_handle> grpc_deframe_unprocessed_incoming_frames(
    grpc_chttp2_stream* s, int64_t* min_progress_size,
    grpc_core::SliceBuffer* stream_out, uint32_t* message_flags);

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FRAME_DATA_H
