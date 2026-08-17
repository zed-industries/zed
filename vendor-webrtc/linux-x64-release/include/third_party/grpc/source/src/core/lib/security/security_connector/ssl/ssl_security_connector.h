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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_SECURITY_CONNECTOR_SSL_SSL_SECURITY_CONNECTOR_H
#define GRPC_SRC_CORE_LIB_SECURITY_SECURITY_CONNECTOR_SSL_SSL_SECURITY_CONNECTOR_H
#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/grpc_security_constants.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include "src/core/lib/security/security_connector/security_connector.h"
#include "src/core/tsi/ssl_transport_security.h"
#include "src/core/util/ref_counted_ptr.h"

struct grpc_ssl_config {
  tsi_ssl_pem_key_cert_pair* pem_key_cert_pair;
  char* pem_root_certs;
  verify_peer_options verify_options;
  grpc_tls_version min_tls_version = grpc_tls_version::TLS1_2;
  grpc_tls_version max_tls_version = grpc_tls_version::TLS1_3;
};

// Creates an SSL channel_security_connector.
// - request_metadata_creds is the credentials object which metadata
//   will be sent with each request. This parameter can be NULL.
// - config is the SSL config to be used for the SSL channel establishment.
// - is_client should be 0 for a server or a non-0 value for a client.
// - secure_peer_name is the secure peer name that should be checked in
//   grpc_channel_security_connector_check_peer. This parameter may be NULL in
//   which case the peer name will not be checked. Note that if this parameter
//   is not NULL, then, pem_root_certs should not be NULL either.
// - sc is a pointer on the connector to be created.
// This function returns GRPC_SECURITY_OK in case of success or a
// specific error code otherwise.
//
grpc_core::RefCountedPtr<grpc_channel_security_connector>
grpc_ssl_channel_security_connector_create(
    grpc_core::RefCountedPtr<grpc_channel_credentials> channel_creds,
    grpc_core::RefCountedPtr<grpc_call_credentials> request_metadata_creds,
    const grpc_ssl_config* config, const char* target_name,
    const char* overridden_target_name,
    tsi_ssl_client_handshaker_factory* factory);

// Config for ssl servers.
struct grpc_ssl_server_config {
  tsi_ssl_pem_key_cert_pair* pem_key_cert_pairs = nullptr;
  size_t num_key_cert_pairs = 0;
  char* pem_root_certs = nullptr;
  grpc_ssl_client_certificate_request_type client_certificate_request =
      GRPC_SSL_DONT_REQUEST_CLIENT_CERTIFICATE;
  grpc_tls_version min_tls_version = grpc_tls_version::TLS1_2;
  grpc_tls_version max_tls_version = grpc_tls_version::TLS1_3;
};
// Creates an SSL server_security_connector.
// - config is the SSL config to be used for the SSL channel establishment.
// - sc is a pointer on the connector to be created.
// This function returns GRPC_SECURITY_OK in case of success or a
// specific error code otherwise.
//
grpc_core::RefCountedPtr<grpc_server_security_connector>
grpc_ssl_server_security_connector_create(
    grpc_core::RefCountedPtr<grpc_server_credentials> server_credentials);

#endif  // GRPC_SRC_CORE_LIB_SECURITY_SECURITY_CONNECTOR_SSL_SSL_SECURITY_CONNECTOR_H
