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

#ifndef GRPC_SRC_CORE_TSI_SSL_TRANSPORT_SECURITY_H
#define GRPC_SRC_CORE_TSI_SSL_TRANSPORT_SECURITY_H

#include <grpc/grpc_crl_provider.h>
#include <grpc/grpc_security_constants.h>
#include <grpc/support/port_platform.h>
#include <openssl/x509.h>

#include <memory>

#include "absl/strings/string_view.h"
#include "src/core/tsi/ssl/key_logging/ssl_key_logging.h"
#include "src/core/tsi/ssl_transport_security_utils.h"
#include "src/core/tsi/transport_security_interface.h"

// Value for the TSI_CERTIFICATE_TYPE_PEER_PROPERTY property for X509 certs.
#define TSI_X509_CERTIFICATE_TYPE "X509"

// This property is of type TSI_PEER_PROPERTY_STRING.
#define TSI_X509_SUBJECT_PEER_PROPERTY "x509_subject"
#define TSI_X509_SUBJECT_COMMON_NAME_PEER_PROPERTY "x509_subject_common_name"
#define TSI_X509_SUBJECT_ALTERNATIVE_NAME_PEER_PROPERTY \
  "x509_subject_alternative_name"
#define TSI_SSL_SESSION_REUSED_PEER_PROPERTY "ssl_session_reused"
#define TSI_X509_PEM_CERT_PROPERTY "x509_pem_cert"
#define TSI_X509_PEM_CERT_CHAIN_PROPERTY "x509_pem_cert_chain"
#define TSI_SSL_ALPN_SELECTED_PROTOCOL "ssl_alpn_selected_protocol"
#define TSI_X509_DNS_PEER_PROPERTY "x509_dns"
#define TSI_X509_URI_PEER_PROPERTY "x509_uri"
#define TSI_X509_EMAIL_PEER_PROPERTY "x509_email"
#define TSI_X509_IP_PEER_PROPERTY "x509_ip"
#define TSI_X509_VERIFIED_ROOT_CERT_SUBECT_PEER_PROPERTY \
  "x509_verified_root_cert_subject"

// --- tsi_ssl_root_certs_store object ---

// This object stores SSL root certificates. It can be shared by multiple SSL
// context.
typedef struct tsi_ssl_root_certs_store tsi_ssl_root_certs_store;

// Given a NULL-terminated string containing the PEM encoding of the root
// certificates, creates a tsi_ssl_root_certs_store object.
tsi_ssl_root_certs_store* tsi_ssl_root_certs_store_create(
    const char* pem_roots);

// Destroys the tsi_ssl_root_certs_store object.
void tsi_ssl_root_certs_store_destroy(tsi_ssl_root_certs_store* self);

// --- tsi_ssl_session_cache object ---

// Cache for SSL sessions for sessions resumption.

typedef struct tsi_ssl_session_cache tsi_ssl_session_cache;

// Create LRU cache for SSL sessions with \a capacity.
tsi_ssl_session_cache* tsi_ssl_session_cache_create_lru(size_t capacity);

// Increment reference counter of \a cache.
void tsi_ssl_session_cache_ref(tsi_ssl_session_cache* cache);

// Decrement reference counter of \a cache.
void tsi_ssl_session_cache_unref(tsi_ssl_session_cache* cache);

// --- tsi_ssl_key_logger object ---

// Experimental SSL Key logging functionality to enable decryption of
// packet captures.
static constexpr bool tsi_tls_session_key_logging_supported() {
// Supported only for open-ssl versions >= 1.1.1
#if OPENSSL_VERSION_NUMBER >= 0x10101000 && !defined(LIBRESSL_VERSION_NUMBER)
  return true;
#else
  return false;
#endif
}

// --- tsi_ssl_client_handshaker_factory object ---

// This object creates a client tsi_handshaker objects implemented in terms of
// the TLS 1.2 specification.

typedef struct tsi_ssl_client_handshaker_factory
    tsi_ssl_client_handshaker_factory;

// Object that holds a private key / certificate chain pair in PEM format.
struct tsi_ssl_pem_key_cert_pair {
  // private_key is the NULL-terminated string containing the PEM encoding of
  // the client's private key.
  const char* private_key;

  // cert_chain is the NULL-terminated string containing the PEM encoding of
  // the client's certificate chain.
  const char* cert_chain;
};
// TO BE DEPRECATED.
// Creates a client handshaker factory.
// - pem_key_cert_pair is a pointer to the object containing client's private
//   key and certificate chain. This parameter can be NULL if the client does
//   not have such a key/cert pair.
// - pem_roots_cert is the NULL-terminated string containing the PEM encoding of
//   the server root certificates.
// - cipher_suites contains an optional list of the ciphers that the client
//   supports. The format of this string is described in:
//   https://www.openssl.org/docs/apps/ciphers.html.
//   This parameter can be set to NULL to use the default set of ciphers.
//   TODO(jboeuf): Revisit the format of this parameter.
// - alpn_protocols is an array containing the NULL terminated protocol names
//   that the handshakers created with this factory support. This parameter can
//   be NULL.
// - num_alpn_protocols is the number of alpn protocols and associated lengths
//   specified. If this parameter is 0, the other alpn parameters must be NULL.
// - factory is the address of the factory pointer to be created.

// - This method returns TSI_OK on success or TSI_INVALID_PARAMETER in the case
//   where a parameter is invalid.
tsi_result tsi_create_ssl_client_handshaker_factory(
    const tsi_ssl_pem_key_cert_pair* pem_key_cert_pair,
    const char* pem_root_certs, const char* cipher_suites,
    const char** alpn_protocols, uint16_t num_alpn_protocols,
    tsi_ssl_client_handshaker_factory** factory);

struct tsi_ssl_client_handshaker_options {
  // pem_key_cert_pair is a pointer to the object containing client's private
  // key and certificate chain. This parameter can be NULL if the client does
  // not have such a key/cert pair.
  const tsi_ssl_pem_key_cert_pair* pem_key_cert_pair;
  // pem_roots_cert is the NULL-terminated string containing the PEM encoding of
  // the client root certificates.
  const char* pem_root_certs;
  // root_store is a pointer to the ssl_root_certs_store object. If root_store
  // is not nullptr and SSL implementation permits, root_store will be used as
  // root certificates. Otherwise, pem_roots_cert will be used to load server
  // root certificates.
  const tsi_ssl_root_certs_store* root_store;
  // cipher_suites contains an optional list of the ciphers that the client
  // supports. The format of this string is described in:
  // https://www.openssl.org/docs/apps/ciphers.html.
  // This parameter can be set to NULL to use the default set of ciphers.
  // TODO(jboeuf): Revisit the format of this parameter.
  const char* cipher_suites;
  // alpn_protocols is an array containing the NULL terminated protocol names
  // that the handshakers created with this factory support. This parameter can
  // be NULL.
  const char** alpn_protocols;
  // num_alpn_protocols is the number of alpn protocols and associated lengths
  // specified. If this parameter is 0, the other alpn parameters must be
  // NULL.
  size_t num_alpn_protocols;
  // ssl_session_cache is a cache for reusable client-side sessions.
  tsi_ssl_session_cache* session_cache;
  // tsi_ssl_key_logger is an instance used to log SSL keys to a file.
  tsi::TlsSessionKeyLoggerCache::TlsSessionKeyLogger* key_logger;

  // skip server certificate verification.
  bool skip_server_certificate_verification;

  // The min and max TLS versions that will be negotiated by the handshaker.
  tsi_tls_version min_tls_version;
  tsi_tls_version max_tls_version;

  // The directory where all hashed CRL files enforced by the handshaker are
  // located. If the directory is invalid, CRL checking will fail open and just
  // log. An empty directory will not enable crl checking. Only OpenSSL version
  // >= 1.1 is supported for CRL checking. Cannot be used in conjunction with
  // `crl_provider`.
  const char* crl_directory;

  // A provider of CRLs. If set, when doing handshakes the `CrlProvider`'s
  // `GetCrl` function will be called to find CRLs when checking certificates
  // for revocation. Cannot be used in conjunction with `crl_directory`.
  // This provider is created and owned by the user and passed in through
  // options as a shared_ptr.
  std::shared_ptr<grpc_core::experimental::CrlProvider> crl_provider;

  tsi_ssl_client_handshaker_options()
      : pem_key_cert_pair(nullptr),
        pem_root_certs(nullptr),
        root_store(nullptr),
        cipher_suites(nullptr),
        alpn_protocols(nullptr),
        num_alpn_protocols(0),
        session_cache(nullptr),
        key_logger(nullptr),
        skip_server_certificate_verification(false),
        min_tls_version(tsi_tls_version::TSI_TLS1_2),
        max_tls_version(tsi_tls_version::TSI_TLS1_3),
        crl_directory(nullptr) {}
};

// Creates a client handshaker factory.
// - options is the options used to create a factory.
// - factory is the address of the factory pointer to be created.

// - This method returns TSI_OK on success or TSI_INVALID_PARAMETER in the case
//   where a parameter is invalid.
tsi_result tsi_create_ssl_client_handshaker_factory_with_options(
    const tsi_ssl_client_handshaker_options* options,
    tsi_ssl_client_handshaker_factory** factory);

// Creates a client handshaker.
//- factory is the factory from which the handshaker will be created.
//- server_name_indication indicates the name of the server the client is
//  trying to connect to which will be relayed to the server using the SNI
//  extension.
//- network_bio_buf_size and ssl_bio_buf_size represent BIO pair buffers used in
//  SSL. The buffer size being 0 translates to 17KB in boringSSL.
//- handshaker is the address of the handshaker pointer to be created.

//- This method returns TSI_OK on success or TSI_INVALID_PARAMETER in the case
//  where a parameter is invalid.
tsi_result tsi_ssl_client_handshaker_factory_create_handshaker(
    tsi_ssl_client_handshaker_factory* factory,
    const char* server_name_indication, size_t network_bio_buf_size,
    size_t ssl_bio_buf_size, tsi_handshaker** handshaker);

// Increments reference count of the client handshaker factory.
tsi_ssl_client_handshaker_factory* tsi_ssl_client_handshaker_factory_ref(
    tsi_ssl_client_handshaker_factory* client_factory);

// Decrements reference count of the handshaker factory. Handshaker factory will
// be destroyed once no references exist.
void tsi_ssl_client_handshaker_factory_unref(
    tsi_ssl_client_handshaker_factory* factory);

// --- tsi_ssl_server_handshaker_factory object ---

// This object creates a client tsi_handshaker objects implemented in terms of
// the TLS 1.2 specification.

typedef struct tsi_ssl_server_handshaker_factory
    tsi_ssl_server_handshaker_factory;

// TO BE DEPRECATED.
// Creates a server handshaker factory.
// - pem_key_cert_pairs is an array private key / certificate chains of the
//   server.
// - num_key_cert_pairs is the number of items in the pem_key_cert_pairs array.
// - pem_root_certs is the NULL-terminated string containing the PEM encoding
//   of the client root certificates. This parameter may be NULL if the server
//   does not want the client to be authenticated with SSL.
// - cipher_suites contains an optional list of the ciphers that the server
//   supports. The format of this string is described in:
//   https://www.openssl.org/docs/apps/ciphers.html.
//   This parameter can be set to NULL to use the default set of ciphers.
//   TODO(jboeuf): Revisit the format of this parameter.
// - alpn_protocols is an array containing the NULL terminated protocol names
//   that the handshakers created with this factory support. This parameter can
//   be NULL.
// - num_alpn_protocols is the number of alpn protocols and associated lengths
//   specified. If this parameter is 0, the other alpn parameters must be NULL.
// - factory is the address of the factory pointer to be created.

// - This method returns TSI_OK on success or TSI_INVALID_PARAMETER in the case
//   where a parameter is invalid.
tsi_result tsi_create_ssl_server_handshaker_factory(
    const tsi_ssl_pem_key_cert_pair* pem_key_cert_pairs,
    size_t num_key_cert_pairs, const char* pem_client_root_certs,
    int force_client_auth, const char* cipher_suites,
    const char** alpn_protocols, uint16_t num_alpn_protocols,
    tsi_ssl_server_handshaker_factory** factory);

// TO BE DEPRECATED.
// Same as tsi_create_ssl_server_handshaker_factory method except uses
// tsi_client_certificate_request_type to support more ways to handle client
// certificate authentication.
// - client_certificate_request, if set to non-zero will force the client to
//   authenticate with an SSL cert. Note that this option is ignored if
//   pem_client_root_certs is NULL or pem_client_roots_certs_size is 0
tsi_result tsi_create_ssl_server_handshaker_factory_ex(
    const tsi_ssl_pem_key_cert_pair* pem_key_cert_pairs,
    size_t num_key_cert_pairs, const char* pem_client_root_certs,
    tsi_client_certificate_request_type client_certificate_request,
    const char* cipher_suites, const char** alpn_protocols,
    uint16_t num_alpn_protocols, tsi_ssl_server_handshaker_factory** factory);

struct tsi_ssl_server_handshaker_options {
  // pem_key_cert_pairs is an array private key / certificate chains of the
  // server.
  const tsi_ssl_pem_key_cert_pair* pem_key_cert_pairs;
  // num_key_cert_pairs is the number of items in the pem_key_cert_pairs
  // array.
  size_t num_key_cert_pairs;
  // pem_root_certs is the NULL-terminated string containing the PEM encoding
  // of the server root certificates. This parameter may be NULL if the server
  // does not want the client to be authenticated with SSL.
  const char* pem_client_root_certs;
  // client_certificate_request, if set to non-zero will force the client to
  // authenticate with an SSL cert. Note that this option is ignored if
  // pem_client_root_certs is NULL or pem_client_roots_certs_size is 0.
  tsi_client_certificate_request_type client_certificate_request;
  // cipher_suites contains an optional list of the ciphers that the server
  // supports. The format of this string is described in:
  // https://www.openssl.org/docs/apps/ciphers.html.
  // This parameter can be set to NULL to use the default set of ciphers.
  // TODO(jboeuf): Revisit the format of this parameter.
  const char* cipher_suites;
  // alpn_protocols is an array containing the NULL terminated protocol names
  // that the handshakers created with this factory support. This parameter can
  // be NULL.
  const char** alpn_protocols;
  // num_alpn_protocols is the number of alpn protocols and associated lengths
  // specified. If this parameter is 0, the other alpn parameters must be
  // NULL.
  uint16_t num_alpn_protocols;
  // session_ticket_key is optional key for encrypting session keys. If
  // parameter is not specified it must be NULL.
  const char* session_ticket_key;
  // session_ticket_key_size is a size of session ticket encryption key.
  size_t session_ticket_key_size;
  // The min and max TLS versions that will be negotiated by the handshaker.
  tsi_tls_version min_tls_version;
  tsi_tls_version max_tls_version;
  // tsi_ssl_key_logger is an instance used to log SSL keys to a file.
  tsi::TlsSessionKeyLoggerCache::TlsSessionKeyLogger* key_logger;

  // The directory where all hashed CRL files are cached in the x.509 store and
  // enforced by the handshaker are located. If the directory is invalid, CRL
  // checking will fail open and just log. An empty directory will not enable
  // crl checking. Only OpenSSL version > 1.1 is supported for CRL checking
  const char* crl_directory;

  // A provider of CRLs. If set, when doing handshakes the `CrlProvider`'s
  // `GetCrl` function will be called to find CRLs when checking certificates
  // for revocation. Cannot be used in conjunction with `crl_directory`.
  // This provider is created and owned by the user and passed in through
  // options as a shared_ptr.
  std::shared_ptr<grpc_core::experimental::CrlProvider> crl_provider;

  // If true, the SSL server sends a list of CA names to the client in the
  // ServerHello. This list of CA names is extracted from the server's trust
  // bundle, and the client may use this lint as a hint to decide which
  // certificate it should send to the server.
  //
  // WARNING: This is an extremely dangerous option. If the server's trust
  // bundle is sufficiently large, then setting this bit to true will result in
  // the server being unable to generate a ServerHello, and hence the server
  // will be unusable.
  bool send_client_ca_list;

  tsi_ssl_server_handshaker_options()
      : pem_key_cert_pairs(nullptr),
        num_key_cert_pairs(0),
        pem_client_root_certs(nullptr),
        client_certificate_request(TSI_DONT_REQUEST_CLIENT_CERTIFICATE),
        cipher_suites(nullptr),
        alpn_protocols(nullptr),
        num_alpn_protocols(0),
        session_ticket_key(nullptr),
        session_ticket_key_size(0),
        min_tls_version(tsi_tls_version::TSI_TLS1_2),
        max_tls_version(tsi_tls_version::TSI_TLS1_3),
        key_logger(nullptr),
        crl_directory(nullptr),
        send_client_ca_list(true) {}
};

// Creates a server handshaker factory.
// - options is the options used to create a factory.
// - factory is the address of the factory pointer to be created.

// - This method returns TSI_OK on success or TSI_INVALID_PARAMETER in the case
//   where a parameter is invalid.
tsi_result tsi_create_ssl_server_handshaker_factory_with_options(
    const tsi_ssl_server_handshaker_options* options,
    tsi_ssl_server_handshaker_factory** factory);

// Creates a server handshaker.
//- factory is the factory from which the handshaker will be created.
//- network_bio_buf_size and ssl_bio_buf_size represent BIO pair buffers used in
//  SSL. The buffer size being 0 translates to 17KB in boringSSL.
//- handshaker is the address of the handshaker pointer to be created.

//- This method returns TSI_OK on success or TSI_INVALID_PARAMETER in the case
//  where a parameter is invalid.
tsi_result tsi_ssl_server_handshaker_factory_create_handshaker(
    tsi_ssl_server_handshaker_factory* factory, size_t network_bio_buf_size,
    size_t ssl_bio_buf_size, tsi_handshaker** handshaker);

// Decrements reference count of the handshaker factory. Handshaker factory will
// be destroyed once no references exist.
void tsi_ssl_server_handshaker_factory_unref(
    tsi_ssl_server_handshaker_factory* factory);

// Util that checks that an ssl peer matches a specific name.
// Still TODO(jboeuf):
// - handle mixed case.
// - handle %encoded chars.
// - handle public suffix wildchar more strictly (e.g. *.co.uk)
int tsi_ssl_peer_matches_name(const tsi_peer* peer, absl::string_view name);

// --- Testing support. ---

// These functions and typedefs are not intended to be used outside of testing.
//

// Base type of client and server handshaker factories.
typedef struct tsi_ssl_handshaker_factory tsi_ssl_handshaker_factory;

// Function pointer to handshaker_factory destructor.
typedef void (*tsi_ssl_handshaker_factory_destructor)(
    tsi_ssl_handshaker_factory* factory);

// Virtual table for tsi_ssl_handshaker_factory.
struct tsi_ssl_handshaker_factory_vtable {
  tsi_ssl_handshaker_factory_destructor destroy;
};
// Set destructor of handshaker_factory to new_destructor, returns previous
// destructor.
const tsi_ssl_handshaker_factory_vtable* tsi_ssl_handshaker_factory_swap_vtable(
    tsi_ssl_handshaker_factory* factory,
    tsi_ssl_handshaker_factory_vtable* new_vtable);

// Exposed for testing only.
tsi_result tsi_ssl_extract_x509_subject_names_from_pem_cert(
    const char* pem_cert, tsi_peer* peer);

// Exposed for testing only.
tsi_result tsi_ssl_get_cert_chain_contents(STACK_OF(X509) * peer_chain,
                                           tsi_peer_property* property);

#endif  // GRPC_SRC_CORE_TSI_SSL_TRANSPORT_SECURITY_H
