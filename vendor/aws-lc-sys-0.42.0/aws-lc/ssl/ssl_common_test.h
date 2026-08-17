// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//
// Created by Smith, Justin on 6/13/25.
//

#ifndef SSL_TEST_H
#define SSL_TEST_H
#include "internal.h"
#include "openssl/base.h"

#include <gtest/gtest.h>

BSSL_NAMESPACE_BEGIN

#define TRACED_CALL(code)                     \
  do {                                        \
    SCOPED_TRACE("<- called from here");      \
    code;                                     \
    if (::testing::Test::HasFatalFailure()) { \
      return;                                 \
    }                                         \
  } while (false)

extern UniquePtr<SSL_SESSION> g_last_session;

bool GetClientHello(SSL *ssl, std::vector<uint8_t> *out);

struct VersionParam {
  uint16_t version;
  enum { is_tls, is_dtls } ssl_method;
  // This field is used to generate custom test name suffixes
  // based on the test parameters.
  const char name[20];
  // SSL transfer: the sever SSL is encoded into bytes, and then decoded to
  // another SSL. After transfer, the encoded SSL is freed. The decoded one is
  // used to exchange data. This flag is to replay existing tests with the
  // transferred SSL. If false, the tests use the original server SSL. If true,
  // the tests are replayed with the transferred server SSL. Note: SSL transfer
  // works only with either TLS 1.2 or TLS 1.3 after handshake finished and all
  // post-handshake messages have been flushed.
  bool transfer_ssl;
};

struct SSLTestParam {
  // SSL transfer: the sever SSL is encoded into bytes, and then decoded to
  // another SSL. After transfer, the encoded SSL is freed. The decoded one is
  // used to exchange data. This flag is to replay existing tests with the
  // transferred SSL. If false, the tests use the original server SSL. If true,
  // the tests are replayed with the transferred server SSL. Note: SSL transfer
  // works only with either TLS 1.2 or TLS 1.3 after handshake finished and all
  // post-handshake messages have been flushed.
  bool transfer_ssl;
};

// If true, after handshake finished, the test uses the transferred SSL.
static const bool TRANSFER_SSL = true;

static const SSLTestParam kSSLTestParams[] = {
    {!TRANSFER_SSL},
    {TRANSFER_SSL},
};

static const VersionParam kAllVersions[] = {
    {TLS1_VERSION, VersionParam::is_tls, "TLS1", !TRANSFER_SSL},
    {TLS1_1_VERSION, VersionParam::is_tls, "TLS1_1", !TRANSFER_SSL},
    {TLS1_2_VERSION, VersionParam::is_tls, "TLS1_2", !TRANSFER_SSL},
    {TLS1_3_VERSION, VersionParam::is_tls, "TLS1_3", !TRANSFER_SSL},
    {DTLS1_VERSION, VersionParam::is_dtls, "DTLS1", !TRANSFER_SSL},
    {DTLS1_2_VERSION, VersionParam::is_dtls, "DTLS1_2", !TRANSFER_SSL},
    {TLS1_2_VERSION, VersionParam::is_tls, "TLS1_2_SSL_TRANSFER", TRANSFER_SSL},
    {TLS1_3_VERSION, VersionParam::is_tls, "TLS1_3_SSL_TRANSFER", TRANSFER_SSL},
};

uint16_t EpochFromSequence(uint64_t seq);


struct ClientConfig {
  SSL_SESSION *session = nullptr;
  std::string servername;
  std::string verify_hostname;
  unsigned hostflags = 0;
  bool early_data = false;
};

void ExpectSessionReused(SSL_CTX *client_ctx, SSL_CTX *server_ctx,
                         SSL_SESSION *session, bool want_reused);

UniquePtr<SSL_SESSION> CreateClientSession(
    SSL_CTX *client_ctx, SSL_CTX *server_ctx,
    const ClientConfig &config = ClientConfig());

void TransferSSL(bssl::UniquePtr<SSL> *in, SSL_CTX *in_ctx,
                 bssl::UniquePtr<SSL> *out, bool free_in=true);

bool ConnectClientAndServer(UniquePtr<SSL> *out_client,
                            UniquePtr<SSL> *out_server, SSL_CTX *client_ctx,
                            SSL_CTX *server_ctx,
                            const ClientConfig &config = ClientConfig(),
                            bool shed_handshake_config = true);

bool FlushNewSessionTickets(SSL *client, SSL *server);

UniquePtr<CRYPTO_BUFFER> GetChainTestCertificateBuffer();
UniquePtr<CRYPTO_BUFFER> GetChainTestIntermediateBuffer();
UniquePtr<X509> GetECDSATestCertificate();
UniquePtr<EVP_PKEY> GetECDSATestKey();
UniquePtr<CRYPTO_BUFFER> GetChainTestCertificateBuffer();
UniquePtr<X509> GetChainTestCertificate();
UniquePtr<EVP_PKEY> GetChainTestKey();
UniquePtr<X509> GetChainTestIntermediate();

UniquePtr<X509> GetLeafSecret();
UniquePtr<X509> GetLeafPublic();
UniquePtr<EVP_PKEY> GetLeafKey();
UniquePtr<X509> GetLeafRoot();

UniquePtr<X509> GetED25519TestCertificate();
UniquePtr<EVP_PKEY> GetED25519TestKey();

// Test fixtures sourced from draft-ietf-lamps-dilithium-certificates examples
// (https://github.com/lamps-wg/dilithium-certificates/tree/main/examples).
UniquePtr<X509> GetMLDSA44TestCertificate();
UniquePtr<EVP_PKEY> GetMLDSA44TestKey();
UniquePtr<X509> GetMLDSA65TestCertificate();
UniquePtr<EVP_PKEY> GetMLDSA65TestKey();
UniquePtr<X509> GetMLDSA87TestCertificate();
UniquePtr<EVP_PKEY> GetMLDSA87TestKey();

bool ExpectSingleError(int lib, int reason);

UniquePtr<X509> X509FromBuffer(UniquePtr<CRYPTO_BUFFER> buffer);

const char *GetVersionName(uint16_t version);

void FrozenTimeCallback(const SSL *ssl, timeval *out_clock);

bool ChainsEqual(STACK_OF(X509) *chain, const std::vector<X509 *> &expected);

int SaveLastSession(SSL *ssl, SSL_SESSION *session);

class SSLTest : public testing::TestWithParam<SSLTestParam> {};

UniquePtr<SSL_CTX> CreateContextWithTestCertificate(const SSL_METHOD *method);

void SetUpExpectedNewCodePoint(SSL_CTX *ctx);

UniquePtr<X509> GetTestCertificate();

UniquePtr<EVP_PKEY> GetTestKey();

UniquePtr<EVP_PKEY> KeyFromPEM(const char *pem);

bool CreateClientAndServer(bssl::UniquePtr<SSL> *out_client,
                           bssl::UniquePtr<SSL> *out_server,
                           SSL_CTX *client_ctx, SSL_CTX *server_ctx);

bool CompleteHandshakes(SSL *client, SSL *server);

void SetUpExpectedOldCodePoint(SSL_CTX *ctx);

UniquePtr<EVP_PKEY> KeyFromPEM(const char *pem);

BSSL_NAMESPACE_END

#endif  // SSL_TEST_H
