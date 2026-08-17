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

#ifndef GRPC_TEST_CORE_TSI_ALTS_HANDSHAKER_ALTS_HANDSHAKER_SERVICE_API_TEST_LIB_H
#define GRPC_TEST_CORE_TSI_ALTS_HANDSHAKER_ALTS_HANDSHAKER_SERVICE_API_TEST_LIB_H

#include "src/core/tsi/alts/handshaker/transport_security_common_api.h"
#include "src/proto/grpc/gcp/handshaker.upb.h"

///
/// The first part of this file contains function signatures for de-serializing
/// ALTS handshake requests and setting/serializing ALTS handshake responses,
/// which simulate the behaviour of grpc server that runs ALTS handshaker
/// service.
///

// This method sets peer_rpc_versions for ALTS handshaker response.
bool grpc_gcp_handshaker_resp_set_peer_rpc_versions(
    grpc_gcp_HandshakerResp* resp, upb_Arena* arena, uint32_t max_major,
    uint32_t max_minor, uint32_t min_major, uint32_t min_minor);

// This method de-serializes ALTS handshaker request.
grpc_gcp_HandshakerReq* grpc_gcp_handshaker_req_decode(grpc_slice slice,
                                                       upb_Arena* arena);

// This method checks equality of two ALTS handshaker responses.
bool grpc_gcp_handshaker_resp_equals(const grpc_gcp_HandshakerResp* l_resp,
                                     const grpc_gcp_HandshakerResp* r_resp);

// This method checks equality of two handshaker response results.
bool grpc_gcp_handshaker_resp_result_equals(
    const grpc_gcp_HandshakerResult* l_result,
    const grpc_gcp_HandshakerResult* r_result);

// This method checks equality of two handshaker response statuses.
bool grpc_gcp_handshaker_resp_status_equals(
    const grpc_gcp_HandshakerStatus* l_status,
    const grpc_gcp_HandshakerStatus* r_status);

#endif  // GRPC_TEST_CORE_TSI_ALTS_HANDSHAKER_ALTS_HANDSHAKER_SERVICE_API_TEST_LIB_H
