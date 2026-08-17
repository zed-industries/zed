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

#ifndef GRPC_SRC_CORE_LIB_TRANSPORT_STATUS_CONVERSION_H
#define GRPC_SRC_CORE_LIB_TRANSPORT_STATUS_CONVERSION_H

#include <grpc/status.h>
#include <grpc/support/port_platform.h>

#include "src/core/ext/transport/chttp2/transport/http2_status.h"
#include "src/core/util/time.h"

// Conversion of grpc status codes to http2 error codes (for RST_STREAM)
grpc_core::http2::Http2ErrorCode grpc_status_to_http2_error(
    grpc_status_code status);
grpc_status_code grpc_http2_error_to_grpc_status(
    grpc_core::http2::Http2ErrorCode error, grpc_core::Timestamp deadline);

// Conversion of HTTP status codes (:status) to grpc status codes
grpc_status_code grpc_http2_status_to_grpc_status(int status);
int grpc_status_to_http2_status(grpc_status_code status);

#endif  // GRPC_SRC_CORE_LIB_TRANSPORT_STATUS_CONVERSION_H
