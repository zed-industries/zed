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

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_ALTS_ALTS_SECURITY_CONNECTOR_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_ALTS_ALTS_SECURITY_CONNECTOR_H
#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include "src/core/credentials/transport/security_connector.h"
#include "src/core/tsi/alts/handshaker/transport_security_common_api.h"
#include "src/core/tsi/transport_security_interface.h"
#include "src/core/util/ref_counted_ptr.h"

#define GRPC_ALTS_TRANSPORT_SECURITY_TYPE "alts"

///
/// This method creates an ALTS channel security connector.
///
///- channel_creds: channel credential instance.
///- request_metadata_creds: credential object which will be sent with each
///  request. This parameter can be nullptr.
///- target_name: the name of the endpoint that the channel is connecting to.
///- sc: address of ALTS channel security connector instance to be returned from
///  the method.
///
/// It returns nullptr on failure.
///
grpc_core::RefCountedPtr<grpc_channel_security_connector>
grpc_alts_channel_security_connector_create(
    grpc_core::RefCountedPtr<grpc_channel_credentials> channel_creds,
    grpc_core::RefCountedPtr<grpc_call_credentials> request_metadata_creds,
    const char* target_name);

///
/// This method creates an ALTS server security connector.
///
///- server_creds: server credential instance.
///- sc: address of ALTS server security connector instance to be returned from
///  the method.
///
/// It returns nullptr on failure.
///
grpc_core::RefCountedPtr<grpc_server_security_connector>
grpc_alts_server_security_connector_create(
    grpc_core::RefCountedPtr<grpc_server_credentials> server_creds);

// Initializes rpc_versions.
void grpc_alts_set_rpc_protocol_versions(
    grpc_gcp_rpc_protocol_versions* rpc_versions);

namespace grpc_core {
namespace internal {

// Exposed only for testing.
RefCountedPtr<grpc_auth_context> grpc_alts_auth_context_from_tsi_peer(
    const tsi_peer* peer);

}  // namespace internal
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_ALTS_ALTS_SECURITY_CONNECTOR_H
