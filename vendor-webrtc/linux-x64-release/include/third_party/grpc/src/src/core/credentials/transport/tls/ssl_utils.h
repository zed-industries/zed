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

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_SSL_UTILS_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_SSL_UTILS_H

#include <grpc/grpc_crl_provider.h>
#include <grpc/grpc_security.h>
#include <grpc/grpc_security_constants.h>
#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/credentials/transport/security_connector.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/tsi/ssl/key_logging/ssl_key_logging.h"
#include "src/core/tsi/ssl_transport_security.h"
#include "src/core/tsi/transport_security_interface.h"
#include "src/core/util/ref_counted_ptr.h"

// --- Util ---

// Check ALPN information returned from SSL handshakes.
grpc_error_handle grpc_ssl_check_alpn(const tsi_peer* peer);

// Check peer name information returned from SSL handshakes.
grpc_error_handle grpc_ssl_check_peer_name(absl::string_view peer_name,
                                           const tsi_peer* peer);
// Compare target_name information extracted from SSL security connectors.
int grpc_ssl_cmp_target_name(absl::string_view target_name,
                             absl::string_view other_target_name,
                             absl::string_view overridden_target_name,
                             absl::string_view other_overridden_target_name);

namespace grpc_core {
// Check the host that will be set for a call is acceptable.
absl::Status SslCheckCallHost(absl::string_view host,
                              absl::string_view target_name,
                              absl::string_view overridden_target_name,
                              grpc_auth_context* auth_context);
}  // namespace grpc_core

// Return HTTP2-compliant cipher suites that gRPC accepts by default.
const char* grpc_get_ssl_cipher_suites(void);

// Map from grpc_ssl_client_certificate_request_type to
// tsi_client_certificate_request_type.
tsi_client_certificate_request_type
grpc_get_tsi_client_certificate_request_type(
    grpc_ssl_client_certificate_request_type grpc_request_type);

// Map grpc_tls_version to tsi_tls_version.
tsi_tls_version grpc_get_tsi_tls_version(grpc_tls_version tls_version);

// Return an array of strings containing alpn protocols.
const char** grpc_fill_alpn_protocol_strings(size_t* num_alpn_protocols);

// Initialize TSI SSL server/client handshaker factory.
grpc_security_status grpc_ssl_tsi_client_handshaker_factory_init(
    tsi_ssl_pem_key_cert_pair* key_cert_pair, const char* pem_root_certs,
    bool skip_server_certificate_verification, tsi_tls_version min_tls_version,
    tsi_tls_version max_tls_version, tsi_ssl_session_cache* ssl_session_cache,
    tsi::TlsSessionKeyLoggerCache::TlsSessionKeyLogger* tls_session_key_logger,
    const char* crl_directory,
    std::shared_ptr<grpc_core::experimental::CrlProvider> crl_provider,
    tsi_ssl_client_handshaker_factory** handshaker_factory);

grpc_security_status grpc_ssl_tsi_server_handshaker_factory_init(
    tsi_ssl_pem_key_cert_pair* key_cert_pairs, size_t num_key_cert_pairs,
    const char* pem_root_certs,
    grpc_ssl_client_certificate_request_type client_certificate_request,
    tsi_tls_version min_tls_version, tsi_tls_version max_tls_version,
    tsi::TlsSessionKeyLoggerCache::TlsSessionKeyLogger* tls_session_key_logger,
    const char* crl_directory, bool send_client_ca_list,
    std::shared_ptr<grpc_core::experimental::CrlProvider> crl_provider,
    tsi_ssl_server_handshaker_factory** handshaker_factory);

// Free the memory occupied by key cert pairs.
void grpc_tsi_ssl_pem_key_cert_pairs_destroy(tsi_ssl_pem_key_cert_pair* kp,
                                             size_t num_key_cert_pairs);
// Exposed for testing only.
grpc_core::RefCountedPtr<grpc_auth_context> grpc_ssl_peer_to_auth_context(
    const tsi_peer* peer, const char* transport_security_type);
tsi_peer grpc_shallow_peer_from_ssl_auth_context(
    const grpc_auth_context* auth_context);
void grpc_shallow_peer_destruct(tsi_peer* peer);
int grpc_ssl_host_matches_name(const tsi_peer* peer,
                               absl::string_view peer_name);

// --- Default SSL Root Store. ---
namespace grpc_core {

// The class implements default SSL root store.
class DefaultSslRootStore {
 public:
  // Gets the default SSL root store. Returns nullptr if not found.
  static const tsi_ssl_root_certs_store* GetRootStore();

  // Gets the default PEM root certificate.
  static const char* GetPemRootCerts();

 protected:
  // Returns default PEM root certificates in nullptr terminated grpc_slice.
  // This function is protected instead of private, so that it can be tested.
  static grpc_slice ComputePemRootCerts();

 private:
  // Construct me not!
  DefaultSslRootStore();

  // Initialization of default SSL root store.
  static void InitRootStore();

  // One-time initialization of default SSL root store.
  static void InitRootStoreOnce();

  // SSL root store in tsi_ssl_root_certs_store object.
  static tsi_ssl_root_certs_store* default_root_store_;

  // Default PEM root certificates.
  static grpc_slice default_pem_root_certs_;
};

class PemKeyCertPair {
 public:
  PemKeyCertPair(absl::string_view private_key, absl::string_view cert_chain)
      : private_key_(private_key), cert_chain_(cert_chain) {}

  // Movable.
  PemKeyCertPair(PemKeyCertPair&& other) noexcept {
    private_key_ = std::move(other.private_key_);
    cert_chain_ = std::move(other.cert_chain_);
  }
  PemKeyCertPair& operator=(PemKeyCertPair&& other) noexcept {
    private_key_ = std::move(other.private_key_);
    cert_chain_ = std::move(other.cert_chain_);
    return *this;
  }

  // Copyable.
  PemKeyCertPair(const PemKeyCertPair& other)
      : private_key_(other.private_key()), cert_chain_(other.cert_chain()) {}
  PemKeyCertPair& operator=(const PemKeyCertPair& other) {
    private_key_ = other.private_key();
    cert_chain_ = other.cert_chain();
    return *this;
  }

  bool operator==(const PemKeyCertPair& other) const {
    return this->private_key() == other.private_key() &&
           this->cert_chain() == other.cert_chain();
  }

  const std::string& private_key() const { return private_key_; }
  const std::string& cert_chain() const { return cert_chain_; }

 private:
  std::string private_key_;
  std::string cert_chain_;
};

using PemKeyCertPairList = std::vector<PemKeyCertPair>;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_SSL_UTILS_H
