//
//
// Copyright 2020 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_SECURITY_CONNECTOR_INSECURE_INSECURE_SECURITY_CONNECTOR_H
#define GRPC_SRC_CORE_LIB_SECURITY_SECURITY_CONNECTOR_INSECURE_INSECURE_SECURITY_CONNECTOR_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include <utility>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/handshaker/handshaker.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/security/credentials/credentials.h"
#include "src/core/lib/security/security_connector/security_connector.h"
#include "src/core/tsi/transport_security_interface.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

extern const char kInsecureTransportSecurityType[];

// Exposed for testing purposes only.
// Create an auth context which is necessary to pass the santiy check in
// client_auth_filter that verifies if the peer's auth context is obtained
// during handshakes.
RefCountedPtr<grpc_auth_context> TestOnlyMakeInsecureAuthContext();

class InsecureChannelSecurityConnector
    : public grpc_channel_security_connector {
 public:
  InsecureChannelSecurityConnector(
      RefCountedPtr<grpc_channel_credentials> channel_creds,
      RefCountedPtr<grpc_call_credentials> request_metadata_creds)
      : grpc_channel_security_connector("", std::move(channel_creds),
                                        std::move(request_metadata_creds)) {}

  ArenaPromise<absl::Status> CheckCallHost(
      absl::string_view host, grpc_auth_context* auth_context) override;

  void add_handshakers(const ChannelArgs& args,
                       grpc_pollset_set* /* interested_parties */,
                       HandshakeManager* handshake_manager) override;

  void check_peer(tsi_peer peer, grpc_endpoint* ep, const ChannelArgs& /*args*/,
                  RefCountedPtr<grpc_auth_context>* auth_context,
                  grpc_closure* on_peer_checked) override;

  void cancel_check_peer(grpc_closure* /*on_peer_checked*/,
                         grpc_error_handle /*error*/) override {}

  int cmp(const grpc_security_connector* other_sc) const override;
};

class InsecureServerSecurityConnector : public grpc_server_security_connector {
 public:
  explicit InsecureServerSecurityConnector(
      RefCountedPtr<grpc_server_credentials> server_creds)
      : grpc_server_security_connector("" /* url_scheme */,
                                       std::move(server_creds)) {}

  void add_handshakers(const ChannelArgs& args,
                       grpc_pollset_set* /* interested_parties */,
                       HandshakeManager* handshake_manager) override;

  void check_peer(tsi_peer peer, grpc_endpoint* ep, const ChannelArgs& /*args*/,
                  RefCountedPtr<grpc_auth_context>* auth_context,
                  grpc_closure* on_peer_checked) override;

  void cancel_check_peer(grpc_closure* /*on_peer_checked*/,
                         grpc_error_handle /*error*/) override {}

  int cmp(const grpc_security_connector* other) const override;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_SECURITY_CONNECTOR_INSECURE_INSECURE_SECURITY_CONNECTOR_H
