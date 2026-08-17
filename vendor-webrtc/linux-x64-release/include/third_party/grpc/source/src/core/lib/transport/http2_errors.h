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

#ifndef GRPC_SRC_CORE_LIB_TRANSPORT_HTTP2_ERRORS_H
#define GRPC_SRC_CORE_LIB_TRANSPORT_HTTP2_ERRORS_H

// error codes for RST_STREAM from http2 draft 14 section 7
typedef enum {
  GRPC_HTTP2_NO_ERROR = 0x0,
  GRPC_HTTP2_PROTOCOL_ERROR = 0x1,
  GRPC_HTTP2_INTERNAL_ERROR = 0x2,
  GRPC_HTTP2_FLOW_CONTROL_ERROR = 0x3,
  GRPC_HTTP2_SETTINGS_TIMEOUT = 0x4,
  GRPC_HTTP2_STREAM_CLOSED = 0x5,
  GRPC_HTTP2_FRAME_SIZE_ERROR = 0x6,
  GRPC_HTTP2_REFUSED_STREAM = 0x7,
  GRPC_HTTP2_CANCEL = 0x8,
  GRPC_HTTP2_COMPRESSION_ERROR = 0x9,
  GRPC_HTTP2_CONNECT_ERROR = 0xa,
  GRPC_HTTP2_ENHANCE_YOUR_CALM = 0xb,
  GRPC_HTTP2_INADEQUATE_SECURITY = 0xc,
  // force use of a default clause
  GRPC_HTTP2__ERROR_DO_NOT_USE = -1
} grpc_http2_error_code;

#endif  // GRPC_SRC_CORE_LIB_TRANSPORT_HTTP2_ERRORS_H
