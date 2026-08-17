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

#ifndef GRPC_SRC_CORE_TSI_ALTS_HANDSHAKER_ALTS_TSI_UTILS_H
#define GRPC_SRC_CORE_TSI_ALTS_HANDSHAKER_ALTS_TSI_UTILS_H

#include <grpc/byte_buffer.h>
#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include "src/core/tsi/transport_security_interface.h"
#include "src/proto/grpc/gcp/handshaker.upb.h"

///
/// This method converts grpc_status_code code to the corresponding tsi_result
/// code.
///
///- code: grpc_status_code code.
///
/// It returns the converted tsi_result code.
///
tsi_result alts_tsi_utils_convert_to_tsi_result(grpc_status_code code);

///
/// This method deserializes a handshaker response returned from ALTS handshaker
/// service.
///
///- bytes_received: data returned from ALTS handshaker service.
///- arena: upb arena.
///
/// It returns a deserialized handshaker response on success and nullptr on
/// failure.
///
grpc_gcp_HandshakerResp* alts_tsi_utils_deserialize_response(
    grpc_byte_buffer* resp_buffer, upb_Arena* arena);

#endif  // GRPC_SRC_CORE_TSI_ALTS_HANDSHAKER_ALTS_TSI_UTILS_H
