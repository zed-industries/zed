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

#ifndef GRPC_SRC_CORE_TSI_ALTS_HANDSHAKER_ALTS_TSI_HANDSHAKER_H
#define GRPC_SRC_CORE_TSI_ALTS_HANDSHAKER_ALTS_TSI_HANDSHAKER_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/pollset_set.h"
#include "src/core/lib/security/credentials/alts/grpc_alts_credentials_options.h"
#include "src/core/tsi/alts/handshaker/alts_handshaker_client.h"
#include "src/core/tsi/transport_security.h"
#include "src/core/tsi/transport_security_interface.h"
#include "src/proto/grpc/gcp/altscontext.upb.h"
#include "src/proto/grpc/gcp/handshaker.upb.h"

#define TSI_ALTS_SERVICE_ACCOUNT_PEER_PROPERTY "service_account"
#define TSI_ALTS_CERTIFICATE_TYPE "ALTS"
#define TSI_ALTS_RPC_VERSIONS "rpc_versions"
#define TSI_ALTS_CONTEXT "alts_context"

const size_t kTsiAltsNumOfPeerProperties = 5;

// Frame size negotiation extends send frame size range to
// [kTsiAltsMinFrameSize, kTsiAltsMaxFrameSize].
const size_t kTsiAltsMinFrameSize = 16 * 1024;
const size_t kTsiAltsMaxFrameSize = 1024 * 1024;

typedef struct alts_tsi_handshaker alts_tsi_handshaker;

///
/// This method creates a ALTS TSI handshaker instance.
///
///- options: ALTS credentials options containing information passed from TSI
///  caller (e.g., rpc protocol versions).
///- target_name: the name of the endpoint that the channel is connecting to,
///  and will be used for secure naming check.
///- handshaker_service_url: address of ALTS handshaker service in the format of
///  "host:port".
///- is_client: boolean value indicating if the handshaker is used at the client
///  (is_client = true) or server (is_client = false) side.
///- interested_parties: set of pollsets interested in this connection.
///- self: address of ALTS TSI handshaker instance to be returned from the
///  method.
///- user_specified_max_frame_size: Determines the maximum frame size used by
///  frame protector that is specified via user. If unspecified, the value is 0.
///
/// It returns TSI_OK on success and an error status code on failure. Note that
/// if interested_parties is nullptr, a dedicated TSI thread will be created and
/// used.
///
tsi_result alts_tsi_handshaker_create(
    const grpc_alts_credentials_options* options, const char* target_name,
    const char* handshaker_service_url, bool is_client,
    grpc_pollset_set* interested_parties, tsi_handshaker** self,
    size_t user_specified_max_frame_size);

///
/// This method creates an ALTS TSI handshaker result instance.
///
///- resp: data received from the handshaker service.
///- is_client: a boolean value indicating if the result belongs to a
///  client or not.
///- result: address of ALTS TSI handshaker result instance.
///
tsi_result alts_tsi_handshaker_result_create(grpc_gcp_HandshakerResp* resp,
                                             bool is_client,
                                             tsi_handshaker_result** result);

///
/// This method sets unused bytes of ALTS TSI handshaker result instance.
///
///- result: an ALTS TSI handshaker result instance.
///- recv_bytes: data received from the handshaker service.
///- bytes_consumed: size of data consumed by the handshaker service.
///
void alts_tsi_handshaker_result_set_unused_bytes(tsi_handshaker_result* result,
                                                 grpc_slice* recv_bytes,
                                                 size_t bytes_consumed);

///
/// This method returns a boolean value indicating if an ALTS TSI handshaker
/// has been shutdown or not.
///
bool alts_tsi_handshaker_has_shutdown(alts_tsi_handshaker* handshaker);

#endif  // GRPC_SRC_CORE_TSI_ALTS_HANDSHAKER_ALTS_TSI_HANDSHAKER_H
