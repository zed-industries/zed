//
//
// Copyright 2016 gRPC authors.
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
#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_SSL_SSL_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_SSL_SSL_CREDENTIALS_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/grpc_security_constants.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include "absl/log/check.h"
#include "src/core/credentials/transport/security_connector.h"
#include "src/core/credentials/transport/ssl/ssl_security_connector.h"
#include "src/core/credentials/transport/transport_credentials.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/tsi/ssl_transport_security.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/useful.h"

class grpc_ssl_credentials : public grpc_channel_credentials {
 public:
  grpc_ssl_credentials(const char* pem_root_certs,
                       grpc_ssl_pem_key_cert_pair* pem_key_cert_pair,
                       const grpc_ssl_verify_peer_options* verify_options);

  ~grpc_ssl_credentials() override;

  grpc_core::RefCountedPtr<grpc_channel_security_connector>
  create_security_connector(
      grpc_core::RefCountedPtr<grpc_call_credentials> call_creds,
      const char* target, grpc_core::ChannelArgs* args) override;

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

  // TODO(mattstev): Plumb to wrapped languages. Until then, setting the TLS
  // version should be done for testing purposes only.
  void set_min_tls_version(grpc_tls_version min_tls_version);
  void set_max_tls_version(grpc_tls_version max_tls_version);

 private:
  int cmp_impl(const grpc_channel_credentials* other) const override {
    // TODO(yashykt): Check if we can do something better here
    return grpc_core::QsortCompare(
        static_cast<const grpc_channel_credentials*>(this), other);
  }

  void build_config(const char* pem_root_certs,
                    grpc_ssl_pem_key_cert_pair* pem_key_cert_pair,
                    const grpc_ssl_verify_peer_options* verify_options);

  // InitializeClientHandshakerFactory constructs a client handshaker factory
  // that is stored on this credentials object. This handshaker factory will be
  // used when creating handshakers using these credentials except in the case
  // that there is a session cache. If a session cache is used, a new handshaker
  // factory will be created and used that contains that session cache.
  grpc_security_status InitializeClientHandshakerFactory(
      const grpc_ssl_config* config, const char* pem_root_certs,
      const tsi_ssl_root_certs_store* root_store,
      tsi_ssl_session_cache* ssl_session_cache,
      tsi_ssl_client_handshaker_factory** handshaker_factory);

  grpc_ssl_config config_;
  tsi_ssl_client_handshaker_factory* client_handshaker_factory_ = nullptr;
  const tsi_ssl_root_certs_store* root_store_ = nullptr;
  grpc_security_status client_handshaker_initialization_status_;
};

struct grpc_ssl_server_certificate_config {
  grpc_ssl_pem_key_cert_pair* pem_key_cert_pairs = nullptr;
  size_t num_key_cert_pairs = 0;
  char* pem_root_certs = nullptr;
};

struct grpc_ssl_server_certificate_config_fetcher {
  grpc_ssl_server_certificate_config_callback cb = nullptr;
  void* user_data;
};

class grpc_ssl_server_credentials final : public grpc_server_credentials {
 public:
  explicit grpc_ssl_server_credentials(
      const grpc_ssl_server_credentials_options& options);
  ~grpc_ssl_server_credentials() override;

  grpc_core::RefCountedPtr<grpc_server_security_connector>
  create_security_connector(const grpc_core::ChannelArgs& /* args */) override;

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

  bool has_cert_config_fetcher() const {
    return certificate_config_fetcher_.cb != nullptr;
  }

  grpc_ssl_certificate_config_reload_status FetchCertConfig(
      grpc_ssl_server_certificate_config** config) {
    DCHECK(has_cert_config_fetcher());
    return certificate_config_fetcher_.cb(certificate_config_fetcher_.user_data,
                                          config);
  }

  // TODO(mattstev): Plumb to wrapped languages. Until then, setting the TLS
  // version should be done for testing purposes only.
  void set_min_tls_version(grpc_tls_version min_tls_version);
  void set_max_tls_version(grpc_tls_version max_tls_version);

  const grpc_ssl_server_config& config() const { return config_; }

 private:
  void build_config(
      const char* pem_root_certs,
      grpc_ssl_pem_key_cert_pair* pem_key_cert_pairs, size_t num_key_cert_pairs,
      grpc_ssl_client_certificate_request_type client_certificate_request);

  grpc_ssl_server_config config_;
  grpc_ssl_server_certificate_config_fetcher certificate_config_fetcher_;
};

tsi_ssl_pem_key_cert_pair* grpc_convert_grpc_to_tsi_cert_pairs(
    const grpc_ssl_pem_key_cert_pair* pem_key_cert_pairs,
    size_t num_key_cert_pairs);

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_SSL_SSL_CREDENTIALS_H
