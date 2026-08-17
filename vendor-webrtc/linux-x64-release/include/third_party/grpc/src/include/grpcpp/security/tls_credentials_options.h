//
//
// Copyright 2019 gRPC authors.
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

#ifndef GRPCPP_SECURITY_TLS_CREDENTIALS_OPTIONS_H
#define GRPCPP_SECURITY_TLS_CREDENTIALS_OPTIONS_H

#include <grpc/grpc_security.h>
#include <grpc/grpc_security_constants.h>
#include <grpc/status.h>
#include <grpcpp/security/tls_certificate_provider.h>
#include <grpcpp/security/tls_certificate_verifier.h>
#include <grpcpp/security/tls_crl_provider.h>
#include <grpcpp/support/config.h>

#include <memory>
#include <vector>

namespace grpc {
namespace experimental {

// Base class of configurable options specified by users to configure their
// certain security features supported in TLS. It is used for experimental
// purposes for now and it is subject to change.
class TlsCredentialsOptions {
 public:
  // Constructor for base class TlsCredentialsOptions.
  //
  // @param certificate_provider the provider which fetches TLS credentials that
  // will be used in the TLS handshake
  TlsCredentialsOptions();
  ~TlsCredentialsOptions();

  // Copy constructor does a deep copy of the underlying pointer. No assignment
  // permitted
  TlsCredentialsOptions(const TlsCredentialsOptions& other);
  TlsCredentialsOptions& operator=(const TlsCredentialsOptions& other) = delete;

  // ---- Setters for member fields ----
  // Sets the certificate provider used to store root certs and identity certs.
  void set_certificate_provider(
      std::shared_ptr<CertificateProviderInterface> certificate_provider);
  // Watches the updates of root certificates with name |root_cert_name|.
  // If used in TLS credentials, setting this field is optional for both the
  // client side and the server side.
  // If this is not set on the client side, we will use the root certificates
  // stored in the default system location, since client side must provide root
  // certificates in TLS(no matter single-side TLS or mutual TLS).
  // If this is not set on the server side, we will not watch any root
  // certificate updates, and assume no root certificates needed for the server
  // (in the one-side TLS scenario, the server is not required to provide root
  // certs). We don't support default root certs on server side.
  void watch_root_certs();
  // Sets the name of root certificates being watched, if |watch_root_certs| is
  // called. If not set, an empty string will be used as the name.
  //
  // @param root_cert_name the name of root certs being set.
  void set_root_cert_name(const std::string& root_cert_name);
  // Watches the updates of identity key-cert pairs with name
  // |identity_cert_name|. If used in TLS credentials, it is required to be set
  // on the server side, and optional for the client side(in the one-side
  // TLS scenario, the client is not required to provide identity certs).
  void watch_identity_key_cert_pairs();
  // Sets the name of identity key-cert pairs being watched, if
  // |watch_identity_key_cert_pairs| is called. If not set, an empty string will
  // be used as the name.
  //
  // @param identity_cert_name the name of identity key-cert pairs being set.
  void set_identity_cert_name(const std::string& identity_cert_name);
  // Sets the Tls session key logging configuration. If not set, tls
  // session key logging is disabled. Note that this should be used only for
  // debugging purposes. It should never be used in a production environment
  // due to security concerns.
  //
  // @param tls_session_key_log_file_path: Path where tls session keys would
  // be logged.
  void set_tls_session_key_log_file_path(
      const std::string& tls_session_key_log_file_path);
  // Sets the certificate verifier used to perform post-handshake peer identity
  // checks.
  void set_certificate_verifier(
      std::shared_ptr<CertificateVerifier> certificate_verifier);
  // Sets the options of whether to check the hostname of the peer on a per-call
  // basis. This is usually used in a combination with virtual hosting at the
  // client side, where each individual call on a channel can have a different
  // host associated with it.
  // This check is intended to verify that the host specified for the individual
  // call is covered by the cert that the peer presented.
  // We will perform such checks by default. This should be disabled if
  // verifiers other than the host name verifier is used.
  // Deprecated: This function will be removed in the 1.66 release. This will be
  // replaced by and handled within the custom verifier settings.
  void set_check_call_host(bool check_call_host);

  // Deprecated in favor of set_crl_provider. The
  // crl provider interface provides a significantly more flexible approach to
  // using CRLs. See gRFC A69 for details.
  // If set, gRPC will read all hashed x.509 CRL files in the directory and
  // enforce the CRL files on all TLS handshakes. Only supported for OpenSSL
  // version > 1.1.
  // Deprecated: This function will be removed in the 1.66 release. Use the
  // set_crl_provider function instead.
  void set_crl_directory(const std::string& path);

  void set_crl_provider(std::shared_ptr<CrlProvider> crl_provider);

  // Sets the minimum TLS version that will be negotiated during the TLS
  // handshake. If not set, the underlying SSL library will use TLS v1.2.
  // @param tls_version: The minimum TLS version.
  void set_min_tls_version(grpc_tls_version tls_version);
  // Sets the maximum TLS version that will be negotiated during the TLS
  // handshake. If not set, the underlying SSL library will use TLS v1.3.
  // @param tls_version: The maximum TLS version.
  void set_max_tls_version(grpc_tls_version tls_version);

  // ----- Getters for member fields ----
  // Returns a deep copy of the internal c options. The caller takes ownership
  // of the returned pointer. This function shall be used only internally.
  grpc_tls_credentials_options* c_credentials_options() const;

 protected:
  // Returns the internal c options. The caller does not take ownership of the
  // returned pointer.
  grpc_tls_credentials_options* mutable_c_credentials_options() {
    return c_credentials_options_;
  }

 private:
  std::shared_ptr<CertificateProviderInterface> certificate_provider_;
  std::shared_ptr<CertificateVerifier> certificate_verifier_;
  grpc_tls_credentials_options* c_credentials_options_ = nullptr;
};

// Contains configurable options on the client side.
// Client side doesn't need to always use certificate provider. When the
// certificate provider is not set, we will use the root certificates stored
// in the system default locations, and assume client won't provide any
// identity certificates(single side TLS).
// It is used for experimental purposes for now and it is subject to change.
class TlsChannelCredentialsOptions final : public TlsCredentialsOptions {
 public:
  // Sets the decision of whether to do a crypto check on the server certs.
  // The default is true.
  void set_verify_server_certs(bool verify_server_certs);

 private:
};

// Contains configurable options on the server side.
// It is used for experimental purposes for now and it is subject to change.
class TlsServerCredentialsOptions final : public TlsCredentialsOptions {
 public:
  // Server side is required to use a provider, because server always needs to
  // use identity certs.
  explicit TlsServerCredentialsOptions(
      std::shared_ptr<CertificateProviderInterface> certificate_provider)
      : TlsCredentialsOptions() {
    set_certificate_provider(certificate_provider);
  }

  // Sets option to request the certificates from the client.
  // The default is GRPC_SSL_DONT_REQUEST_CLIENT_CERTIFICATE.
  void set_cert_request_type(
      grpc_ssl_client_certificate_request_type cert_request_type);

  // Sets whether or not a TLS server should send a list of CA names in the
  // ServerHello. This list of CA names is read from the server's trust bundle,
  // so that the client can use this list as a hint to know which certificate it
  // should send to the server.
  //
  // By default, this option is turned off.
  //
  // WARNING: This API is extremely dangerous and should not be used. If the
  // server's trust bundle is too large, then the TLS server will be unable to
  // form a ServerHello, and hence will be unusable.
  // Deprecated: This function will be removed in the 1.66 release.
  void set_send_client_ca_list(bool send_client_ca_list);

 private:
};

}  // namespace experimental
}  // namespace grpc

#endif  // GRPCPP_SECURITY_TLS_CREDENTIALS_OPTIONS_H
