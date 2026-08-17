// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC


#include <openssl/hmac.h>
#include <openssl/rand.h>
#include <openssl/ssl.h>
#include <thread>

#include "internal.h"
#include "ssl_common_test.h"

#include "../crypto/test/test_util.h"

BSSL_NAMESPACE_BEGIN

// SSLVersionTest executes its test cases under all available protocol versions.
// Test cases call |Connect| to create a connection using context objects with
// the protocol version fixed to the current version under test.
class SSLVersionTest
    : public ::testing::TestWithParam<::std::tuple<VersionParam, int>> {
 protected:
  SSLVersionTest() : cert_(GetTestCertificate()), key_(GetTestKey()) {}

  void SetUp() { ResetContexts(); }

  bssl::UniquePtr<SSL_CTX> CreateContext() const {
    const SSL_METHOD *method = is_dtls() ? DTLS_method() : TLS_method();
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(method));
    if (!ctx || !SSL_CTX_set_min_proto_version(ctx.get(), version()) ||
        !SSL_CTX_set_max_proto_version(ctx.get(), version())) {
      return nullptr;
    }

    if (enable_read_ahead()) {
      SSL_CTX_set_read_ahead(ctx.get(), 1);
      SSL_CTX_set_default_read_buffer_len(ctx.get(), read_ahead_buffer_size());
    }
    return ctx;
  }

  void ResetContexts() {
    ASSERT_TRUE(cert_);
    ASSERT_TRUE(key_);
    client_ctx_ = CreateContext();
    ASSERT_TRUE(client_ctx_);
    server_ctx_ = CreateContext();
    ASSERT_TRUE(server_ctx_);
    // Set up a server cert. Client certs can be set up explicitly.
    ASSERT_TRUE(UseCertAndKey(server_ctx_.get()));
  }

  bool UseCertAndKey(SSL_CTX *ctx) const {
    return SSL_CTX_use_certificate(ctx, cert_.get()) &&
           SSL_CTX_use_PrivateKey(ctx, key_.get());
  }

  bool Connect(const ClientConfig &config = ClientConfig()) {
    bool connected = ConnectClientAndServer(
        &client_, &server_, client_ctx_.get(), server_ctx_.get(), config,
        shed_handshake_config_);
    if (connected) {
      TransferServerSSL();
    }
    return connected;
  }

  VersionParam getVersionParam() const { return std::get<0>(GetParam()); }

  void TransferServerSSL() {
    if (!getVersionParam().transfer_ssl) {
      return;
    }
    // |server_| is reset to hold the transferred SSL.
    TransferSSL(&server_, server_ctx_.get(), nullptr);
  }

  uint16_t version() const { return getVersionParam().version; }

  bool is_dtls() const {
    return getVersionParam().ssl_method == VersionParam::is_dtls;
  }

  size_t read_ahead_buffer_size() const { return std::get<1>(GetParam()); }

  bool enable_read_ahead() const { return read_ahead_buffer_size() != 0; }

  void CheckCounterInit() {
    EXPECT_EQ(SSL_CTX_sess_connect(client_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_connect(server_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_connect_good(client_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_connect_good(server_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_connect_renegotiate(client_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_connect_renegotiate(server_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_accept(client_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_accept(server_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_accept_renegotiate(client_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_accept_renegotiate(server_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_accept_good(client_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_accept_good(server_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_hits(client_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_hits(server_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_cb_hits(client_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_cb_hits(server_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_misses(client_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_misses(server_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_timeouts(client_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_timeouts(server_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_cache_full(client_ctx_.get()), 0);
    EXPECT_EQ(SSL_CTX_sess_cache_full(server_ctx_.get()), 0);
  }

  bool shed_handshake_config_ = true;
  bssl::UniquePtr<SSL> client_, server_;
  bssl::UniquePtr<SSL_CTX> server_ctx_, client_ctx_;
  bssl::UniquePtr<X509> cert_;
  bssl::UniquePtr<EVP_PKEY> key_;
};

INSTANTIATE_TEST_SUITE_P(
    WithVersion, SSLVersionTest,
    testing::Combine(::testing::ValuesIn(kAllVersions),
                     testing::Values(0, 128, 512, 8192, 65535, 131072)),
    [](const testing::TestParamInfo<::std::tuple<VersionParam, int>>
           &test_info) {
      std::string test_name =
          std::string(std::get<0>(test_info.param).name) + "_BufferSize_";
      return test_name + std::to_string(std::get<1>(test_info.param));
    });

TEST_P(SSLVersionTest, SequenceNumber) {
  CheckCounterInit();
  ASSERT_TRUE(Connect());
  // Counters should count one two-way successful connection.
  EXPECT_EQ(SSL_CTX_sess_connect(client_ctx_.get()), 1);
  EXPECT_EQ(SSL_CTX_sess_connect_good(client_ctx_.get()), 1);
  EXPECT_EQ(SSL_CTX_sess_accept(server_ctx_.get()), 1);
  EXPECT_EQ(SSL_CTX_sess_accept_good(server_ctx_.get()), 1);

  // Drain any post-handshake messages to ensure there are no unread records
  // on either end.
  ASSERT_TRUE(FlushNewSessionTickets(client_.get(), server_.get()));

  uint64_t client_read_seq = SSL_get_read_sequence(client_.get());
  uint64_t client_write_seq = SSL_get_write_sequence(client_.get());
  uint64_t server_read_seq = SSL_get_read_sequence(server_.get());
  uint64_t server_write_seq = SSL_get_write_sequence(server_.get());

  if (is_dtls()) {
    // Both client and server must be at epoch 1.
    EXPECT_EQ(EpochFromSequence(client_read_seq), 1);
    EXPECT_EQ(EpochFromSequence(client_write_seq), 1);
    EXPECT_EQ(EpochFromSequence(server_read_seq), 1);
    EXPECT_EQ(EpochFromSequence(server_write_seq), 1);

    // The next record to be written should exceed the largest received.
    EXPECT_GT(client_write_seq, server_read_seq);
    EXPECT_GT(server_write_seq, client_read_seq);
  } else {
    // The next record to be written should equal the next to be received.
    EXPECT_EQ(client_write_seq, server_read_seq);
    EXPECT_EQ(server_write_seq, client_read_seq);
  }

  // Send a record from client to server.
  uint8_t byte = 0;
  EXPECT_EQ(SSL_write(client_.get(), &byte, 1), 1);
  EXPECT_EQ(SSL_read(server_.get(), &byte, 1), 1);

  // The client write and server read sequence numbers should have
  // incremented.
  EXPECT_EQ(client_write_seq + 1, SSL_get_write_sequence(client_.get()));
  EXPECT_EQ(server_read_seq + 1, SSL_get_read_sequence(server_.get()));
}

TEST_P(SSLVersionTest, OneSidedShutdown) {
  // SSL_shutdown is a no-op in DTLS.
  if (is_dtls()) {
    return;
  }
  ASSERT_TRUE(Connect());

  // Shut down half the connection. |SSL_shutdown| will return 0 to signal only
  // one side has shut down.
  ASSERT_EQ(SSL_shutdown(client_.get()), 0);

  // Reading from the server should consume the EOF.
  uint8_t byte = 0;
  ASSERT_EQ(SSL_read(server_.get(), &byte, 1), 0);
  ASSERT_EQ(SSL_get_error(server_.get(), 0), SSL_ERROR_ZERO_RETURN);

  // However, the server may continue to write data and then shut down the
  // connection.
  byte = 42;
  ASSERT_EQ(SSL_write(server_.get(), &byte, 1), 1);
  ASSERT_EQ(SSL_read(client_.get(), &byte, 1), 1);
  ASSERT_EQ(byte, 42);

  // The server may then shutdown the connection.
  EXPECT_EQ(SSL_shutdown(server_.get()), 1);
  EXPECT_EQ(SSL_shutdown(client_.get()), 1);
}

// Test that, after calling |SSL_shutdown|, |SSL_write| fails.
TEST_P(SSLVersionTest, WriteAfterShutdown) {
  ASSERT_TRUE(Connect());

  for (SSL *ssl : {client_.get(), server_.get()}) {
    SCOPED_TRACE(SSL_is_server(ssl) ? "server" : "client");

    bssl::UniquePtr<BIO> mem(BIO_new(BIO_s_mem()));
    ASSERT_TRUE(mem);
    SSL_set0_wbio(ssl, bssl::UpRef(mem).release());

    // Shut down half the connection. |SSL_shutdown| will return 0 to signal
    // only one side has shut down.
    ASSERT_EQ(SSL_shutdown(ssl), 0);

    // |ssl| should have written an alert to the transport.
    const uint8_t *unused = nullptr;
    size_t len = 0;
    ASSERT_TRUE(BIO_mem_contents(mem.get(), &unused, &len));
    EXPECT_NE(0u, len);
    EXPECT_TRUE(BIO_reset(mem.get()));

    // Writing should fail.
    EXPECT_EQ(-1, SSL_write(ssl, "a", 1));

    // Nothing should be written to the transport.
    ASSERT_TRUE(BIO_mem_contents(mem.get(), &unused, &len));
    EXPECT_EQ(0u, len);
  }
}

// Test that, after sending a fatal alert in a failed |SSL_read|, |SSL_write|
// fails.
TEST_P(SSLVersionTest, WriteAfterReadSentFatalAlert) {
  // Decryption failures are not fatal in DTLS.
  if (is_dtls()) {
    return;
  }

  ASSERT_TRUE(Connect());

  // Save the write |BIO|s as the test will overwrite them.
  bssl::UniquePtr<BIO> client_wbio = bssl::UpRef(SSL_get_wbio(client_.get()));
  bssl::UniquePtr<BIO> server_wbio = bssl::UpRef(SSL_get_wbio(server_.get()));

  for (bool test_server : {false, true}) {
    SCOPED_TRACE(test_server ? "server" : "client");
    SSL *ssl = test_server ? server_.get() : client_.get();
    BIO *other_wbio = test_server ? client_wbio.get() : server_wbio.get();

    bssl::UniquePtr<BIO> mem(BIO_new(BIO_s_mem()));
    ASSERT_TRUE(mem);
    SSL_set0_wbio(ssl, bssl::UpRef(mem).release());

    // Read an invalid record from the peer.
    static const uint8_t kInvalidRecord[] = "invalid record";
    EXPECT_EQ(int{sizeof(kInvalidRecord)},
              BIO_write(other_wbio, kInvalidRecord, sizeof(kInvalidRecord)));
    char buf[256];
    EXPECT_EQ(-1, SSL_read(ssl, buf, sizeof(buf)));

    // |ssl| should have written an alert to the transport.
    const uint8_t *unused = nullptr;
    size_t len = 0;
    ASSERT_TRUE(BIO_mem_contents(mem.get(), &unused, &len));
    EXPECT_NE(0u, len);
    EXPECT_TRUE(BIO_reset(mem.get()));

    // Writing should fail.
    EXPECT_EQ(-1, SSL_write(ssl, "a", 1));

    // Nothing should be written to the transport.
    ASSERT_TRUE(BIO_mem_contents(mem.get(), &unused, &len));
    EXPECT_EQ(0u, len);
  }
}

static int VerifySucceed(X509_STORE_CTX *store_ctx, void *arg) { return 1; }

// Test that, after sending a fatal alert from the handshake, |SSL_write| fails.
TEST_P(SSLVersionTest, WriteAfterHandshakeSentFatalAlert) {
  for (bool test_server : {false, true}) {
    SCOPED_TRACE(test_server ? "server" : "client");

    bssl::UniquePtr<SSL> ssl(
        SSL_new(test_server ? server_ctx_.get() : client_ctx_.get()));
    ASSERT_TRUE(ssl);
    if (test_server) {
      SSL_set_accept_state(ssl.get());
    } else {
      SSL_set_connect_state(ssl.get());
    }

    std::vector<uint8_t> invalid;
    if (is_dtls()) {
      // In DTLS, invalid records are discarded. To cause the handshake to fail,
      // use a valid handshake record with invalid contents.
      invalid.push_back(SSL3_RT_HANDSHAKE);
      invalid.push_back(DTLS1_VERSION >> 8);
      invalid.push_back(DTLS1_VERSION & 0xff);
      // epoch and sequence_number
      for (int i = 0; i < 8; i++) {
        invalid.push_back(0);
      }
      // A one-byte fragment is invalid.
      invalid.push_back(0);
      invalid.push_back(1);
      // Arbitrary contents.
      invalid.push_back(0);
    } else {
      invalid = {'i', 'n', 'v', 'a', 'l', 'i', 'd'};
    }
    bssl::UniquePtr<BIO> rbio(BIO_new_mem_buf(invalid.data(), invalid.size()));
    ASSERT_TRUE(rbio);
    SSL_set0_rbio(ssl.get(), rbio.release());

    bssl::UniquePtr<BIO> mem(BIO_new(BIO_s_mem()));
    ASSERT_TRUE(mem);
    SSL_set0_wbio(ssl.get(), bssl::UpRef(mem).release());

    // The handshake should fail.
    EXPECT_EQ(-1, SSL_do_handshake(ssl.get()));
    EXPECT_EQ(SSL_ERROR_SSL, SSL_get_error(ssl.get(), -1));
    uint32_t err = ERR_get_error();

    // |ssl| should have written an alert (and, in the client's case, a
    // ClientHello) to the transport.
    const uint8_t *unused = nullptr;
    size_t len = 0;
    ASSERT_TRUE(BIO_mem_contents(mem.get(), &unused, &len));
    EXPECT_NE(0u, len);
    EXPECT_TRUE(BIO_reset(mem.get()));

    // Writing should fail, with the same error as the handshake.
    EXPECT_EQ(-1, SSL_write(ssl.get(), "a", 1));
    EXPECT_EQ(SSL_ERROR_SSL, SSL_get_error(ssl.get(), -1));
    EXPECT_EQ(err, ERR_get_error());

    // Nothing should be written to the transport.
    ASSERT_TRUE(BIO_mem_contents(mem.get(), &unused, &len));
    EXPECT_EQ(0u, len);
  }
}

TEST_P(SSLVersionTest, GetPeerCertificate) {
  ASSERT_TRUE(UseCertAndKey(client_ctx_.get()));

  // Configure both client and server to accept any certificate.
  SSL_CTX_set_verify(client_ctx_.get(),
                     SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT,
                     nullptr);
  SSL_CTX_set_cert_verify_callback(client_ctx_.get(), VerifySucceed, NULL);
  SSL_CTX_set_verify(server_ctx_.get(),
                     SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT,
                     nullptr);
  SSL_CTX_set_cert_verify_callback(server_ctx_.get(), VerifySucceed, NULL);

  ASSERT_TRUE(Connect());

  // Client and server should both see the leaf certificate.
  bssl::UniquePtr<X509> peer(SSL_get_peer_certificate(server_.get()));
  ASSERT_TRUE(peer);
  ASSERT_EQ(X509_cmp(cert_.get(), peer.get()), 0);

  peer.reset(SSL_get_peer_certificate(client_.get()));
  ASSERT_TRUE(peer);
  ASSERT_EQ(X509_cmp(cert_.get(), peer.get()), 0);

  // However, for historical reasons, the X509 chain includes the leaf on the
  // client, but does not on the server.
  EXPECT_EQ(sk_X509_num(SSL_get_peer_cert_chain(client_.get())), 1u);
  EXPECT_EQ(sk_CRYPTO_BUFFER_num(SSL_get0_peer_certificates(client_.get())),
            1u);

  EXPECT_EQ(sk_X509_num(SSL_get_peer_cert_chain(server_.get())), 0u);
  EXPECT_EQ(sk_CRYPTO_BUFFER_num(SSL_get0_peer_certificates(server_.get())),
            1u);
}

TEST_P(SSLVersionTest, NoPeerCertificate) {
  SSL_CTX_set_verify(server_ctx_.get(), SSL_VERIFY_PEER, nullptr);
  SSL_CTX_set_cert_verify_callback(server_ctx_.get(), VerifySucceed, NULL);
  SSL_CTX_set_cert_verify_callback(client_ctx_.get(), VerifySucceed, NULL);

  ASSERT_TRUE(Connect());

  // Server should not see a peer certificate.
  bssl::UniquePtr<X509> peer(SSL_get_peer_certificate(server_.get()));
  ASSERT_FALSE(peer);
  ASSERT_FALSE(SSL_get0_peer_certificates(server_.get()));
}

TEST_P(SSLVersionTest, RetainOnlySHA256OfCerts) {
  uint8_t *cert_der = NULL;
  int cert_der_len = i2d_X509(cert_.get(), &cert_der);
  ASSERT_GE(cert_der_len, 0);
  bssl::UniquePtr<uint8_t> free_cert_der(cert_der);

  uint8_t cert_sha256[SHA256_DIGEST_LENGTH];
  SHA256(cert_der, cert_der_len, cert_sha256);

  ASSERT_TRUE(UseCertAndKey(client_ctx_.get()));

  // Configure both client and server to accept any certificate, but the
  // server must retain only the SHA-256 of the peer.
  SSL_CTX_set_verify(client_ctx_.get(),
                     SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT,
                     nullptr);
  SSL_CTX_set_verify(server_ctx_.get(),
                     SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT,
                     nullptr);
  SSL_CTX_set_cert_verify_callback(client_ctx_.get(), VerifySucceed, NULL);
  SSL_CTX_set_cert_verify_callback(server_ctx_.get(), VerifySucceed, NULL);
  SSL_CTX_set_retain_only_sha256_of_client_certs(server_ctx_.get(), 1);

  ASSERT_TRUE(Connect());

  // The peer certificate has been dropped.
  bssl::UniquePtr<X509> peer(SSL_get_peer_certificate(server_.get()));
  EXPECT_FALSE(peer);

  SSL_SESSION *session = SSL_get_session(server_.get());
  EXPECT_TRUE(SSL_SESSION_has_peer_sha256(session));

  const uint8_t *peer_sha256 = nullptr;
  size_t peer_sha256_len = 0;
  SSL_SESSION_get0_peer_sha256(session, &peer_sha256, &peer_sha256_len);
  EXPECT_EQ(Bytes(cert_sha256), Bytes(peer_sha256, peer_sha256_len));
}

static int SwitchSessionIDContextSNI(SSL *ssl, int *out_alert, void *arg) {
  static const uint8_t kContext[] = {3};

  if (!SSL_set_session_id_context(ssl, kContext, sizeof(kContext))) {
    return SSL_TLSEXT_ERR_ALERT_FATAL;
  }

  return SSL_TLSEXT_ERR_OK;
}

TEST_P(SSLVersionTest, SessionIDContext) {
  static const uint8_t kContext1[] = {1};
  static const uint8_t kContext2[] = {2};

  ASSERT_TRUE(SSL_CTX_set_session_id_context(server_ctx_.get(), kContext1,
                                             sizeof(kContext1)));

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);

  bssl::UniquePtr<SSL_SESSION> session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(session);

  TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                  session.get(),
                                  true /* expect session reused */));

  // Change the session ID context.
  ASSERT_TRUE(SSL_CTX_set_session_id_context(server_ctx_.get(), kContext2,
                                             sizeof(kContext2)));

  TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                  session.get(),
                                  false /* expect session not reused */));

  // Change the session ID context back and install an SNI callback to switch
  // it.
  ASSERT_TRUE(SSL_CTX_set_session_id_context(server_ctx_.get(), kContext1,
                                             sizeof(kContext1)));

  SSL_CTX_set_tlsext_servername_callback(server_ctx_.get(),
                                         SwitchSessionIDContextSNI);

  TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                  session.get(),
                                  false /* expect session not reused */));

  // Switch the session ID context with the early callback instead.
  SSL_CTX_set_tlsext_servername_callback(server_ctx_.get(), nullptr);
  SSL_CTX_set_select_certificate_cb(
      server_ctx_.get(),
      [](const SSL_CLIENT_HELLO *client_hello) -> ssl_select_cert_result_t {
        static const uint8_t kContext[] = {3};

        if (!SSL_set_session_id_context(client_hello->ssl, kContext,
                                        sizeof(kContext))) {
          return ssl_select_cert_error;
        }

        return ssl_select_cert_success;
      });

  TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                  session.get(),
                                  false /* expect session not reused */));
}

static timeval g_current_time;

static void CurrentTimeCallback(const SSL *ssl, timeval *out_clock) {
  *out_clock = g_current_time;
}


static bssl::UniquePtr<SSL_SESSION> ExpectSessionRenewed(SSL_CTX *client_ctx,
                                                         SSL_CTX *server_ctx,
                                                         SSL_SESSION *session) {
  g_last_session = nullptr;
  SSL_CTX_sess_set_new_cb(client_ctx, SaveLastSession);

  bssl::UniquePtr<SSL> client, server;
  ClientConfig config;
  config.session = session;
  if (!ConnectClientAndServer(&client, &server, client_ctx, server_ctx,
                              config) ||
      !FlushNewSessionTickets(client.get(), server.get())) {
    fprintf(stderr, "Failed to connect client and server.\n");
    return nullptr;
  }

  if (SSL_session_reused(client.get()) != SSL_session_reused(server.get())) {
    fprintf(stderr, "Client and server were inconsistent.\n");
    return nullptr;
  }

  if (!SSL_session_reused(client.get())) {
    fprintf(stderr, "Session was not reused.\n");
    return nullptr;
  }

  SSL_CTX_sess_set_new_cb(client_ctx, nullptr);

  if (!g_last_session) {
    fprintf(stderr, "Client did not receive a renewed session.\n");
    return nullptr;
  }
  return std::move(g_last_session);
}

static const size_t kTicketKeyLen = 48;

static void ExpectTicketKeyChanged(SSL_CTX *ctx, uint8_t *inout_key,
                                   bool changed) {
  uint8_t new_key[kTicketKeyLen];
  // May return 0, 1 or 48.
  ASSERT_EQ(SSL_CTX_get_tlsext_ticket_keys(ctx, new_key, kTicketKeyLen), 1);
  if (changed) {
    ASSERT_NE(Bytes(inout_key, kTicketKeyLen), Bytes(new_key));
  } else {
    ASSERT_EQ(Bytes(inout_key, kTicketKeyLen), Bytes(new_key));
  }
  OPENSSL_memcpy(inout_key, new_key, kTicketKeyLen);
}



static int RenewTicketCallback(SSL *ssl, uint8_t *key_name, uint8_t *iv,
                               EVP_CIPHER_CTX *ctx, HMAC_CTX *hmac_ctx,
                               int encrypt) {
  static const uint8_t kZeros[16] = {0};

  if (encrypt) {
    OPENSSL_memcpy(key_name, kZeros, sizeof(kZeros));
    RAND_bytes(iv, 16);
  } else if (OPENSSL_memcmp(key_name, kZeros, 16) != 0) {
    return 0;
  }

  if (!HMAC_Init_ex(hmac_ctx, kZeros, sizeof(kZeros), EVP_sha256(), NULL) ||
      !EVP_CipherInit_ex(ctx, EVP_aes_128_cbc(), NULL, kZeros, iv, encrypt)) {
    return -1;
  }

  // Returning two from the callback in decrypt mode renews the
  // session in TLS 1.2 and below.
  return encrypt ? 1 : 2;
}

static bool GetServerTicketTime(long *out, const SSL_SESSION *session) {
  const uint8_t *ticket = nullptr;
  size_t ticket_len = 0;
  SSL_SESSION_get0_ticket(session, &ticket, &ticket_len);
  if (ticket_len < 16 + 16 + SHA256_DIGEST_LENGTH) {
    return false;
  }

  const uint8_t *ciphertext = ticket + 16 + 16;
  size_t len = ticket_len - 16 - 16 - SHA256_DIGEST_LENGTH;
  std::unique_ptr<uint8_t[]> plaintext(new uint8_t[len]);

#if defined(BORINGSSL_UNSAFE_FUZZER_MODE)
  // Fuzzer-mode tickets are unencrypted.
  OPENSSL_memcpy(plaintext.get(), ciphertext, len);
#else
  static const uint8_t kZeros[16] = {0};
  const uint8_t *iv = ticket + 16;
  bssl::ScopedEVP_CIPHER_CTX ctx;
  int len1 = 0, len2 = 0;
  if (len > INT_MAX ||
      !EVP_DecryptInit_ex(ctx.get(), EVP_aes_128_cbc(), nullptr, kZeros, iv) ||
      !EVP_DecryptUpdate(ctx.get(), plaintext.get(), &len1, ciphertext,
                         static_cast<int>(len)) ||
      !EVP_DecryptFinal_ex(ctx.get(), plaintext.get() + len1, &len2)) {
    return false;
  }

  len = static_cast<size_t>(len1 + len2);
#endif

  bssl::UniquePtr<SSL_CTX> ssl_ctx(SSL_CTX_new(TLS_method()));
  if (!ssl_ctx) {
    return false;
  }
  bssl::UniquePtr<SSL_SESSION> server_session(
      SSL_SESSION_from_bytes(plaintext.get(), len, ssl_ctx.get()));
  if (!server_session) {
    return false;
  }

  *out = SSL_SESSION_get_time(server_session.get());
  return true;
}

TEST_P(SSLVersionTest, SessionTimeout) {
  for (bool server_test : {false, true}) {
    SCOPED_TRACE(server_test);

    ASSERT_NO_FATAL_FAILURE(ResetContexts());
    SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
    SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);

    static const time_t kStartTime = 1000;
    g_current_time.tv_sec = kStartTime;

    // We are willing to use a longer lifetime for TLS 1.3 sessions as
    // resumptions still perform ECDHE.
    const time_t timeout = version() == TLS1_3_VERSION
                               ? SSL_DEFAULT_SESSION_PSK_DHE_TIMEOUT
                               : SSL_DEFAULT_SESSION_TIMEOUT;

    // Both client and server must enforce session timeouts. We configure the
    // other side with a frozen clock so it never expires tickets.
    if (server_test) {
      SSL_CTX_set_current_time_cb(client_ctx_.get(), FrozenTimeCallback);
      SSL_CTX_set_current_time_cb(server_ctx_.get(), CurrentTimeCallback);
    } else {
      SSL_CTX_set_current_time_cb(client_ctx_.get(), CurrentTimeCallback);
      SSL_CTX_set_current_time_cb(server_ctx_.get(), FrozenTimeCallback);
    }

    // Configure a ticket callback which renews tickets.
    SSL_CTX_set_tlsext_ticket_key_cb(server_ctx_.get(), RenewTicketCallback);

    bssl::UniquePtr<SSL_SESSION> session =
        CreateClientSession(client_ctx_.get(), server_ctx_.get());
    ASSERT_TRUE(session);

    // Advance the clock just behind the timeout.
    g_current_time.tv_sec += timeout - 1;

    TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                    session.get(),
                                    true /* expect session reused */));

    // Advance the clock one more second.
    g_current_time.tv_sec++;

    TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                    session.get(),
                                    false /* expect session not reused */));

    // Rewind the clock to before the session was minted.
    g_current_time.tv_sec = kStartTime - 1;

    TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                    session.get(),
                                    false /* expect session not reused */));

    // Renew the session 10 seconds before expiration.
    time_t new_start_time = kStartTime + timeout - 10;
    g_current_time.tv_sec = new_start_time;
    bssl::UniquePtr<SSL_SESSION> new_session = ExpectSessionRenewed(
        client_ctx_.get(), server_ctx_.get(), session.get());
    ASSERT_TRUE(new_session);

    // This new session is not the same object as before.
    EXPECT_NE(session.get(), new_session.get());

    // Check the sessions have timestamps measured from issuance.
    long session_time = 0;
    if (server_test) {
      ASSERT_TRUE(GetServerTicketTime(&session_time, new_session.get()));
    } else {
      session_time = SSL_SESSION_get_time(new_session.get());
    }

    ASSERT_EQ(session_time, g_current_time.tv_sec);

    if (version() == TLS1_3_VERSION) {
      // Renewal incorporates fresh key material in TLS 1.3, so we extend the
      // lifetime TLS 1.3.
      g_current_time.tv_sec = new_start_time + timeout - 1;
      TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                      new_session.get(),
                                      true /* expect session reused */));

      // The new session expires after the new timeout.
      g_current_time.tv_sec = new_start_time + timeout + 1;
      TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                      new_session.get(),
                                      false /* expect session ot reused */));

      // Renew the session until it begins just past the auth timeout.
      time_t auth_end_time = kStartTime + SSL_DEFAULT_SESSION_AUTH_TIMEOUT;
      while (new_start_time < auth_end_time - 1000) {
        // Get as close as possible to target start time.
        new_start_time =
            std::min(auth_end_time - 1000, new_start_time + timeout - 1);
        g_current_time.tv_sec = new_start_time;
        new_session = ExpectSessionRenewed(client_ctx_.get(), server_ctx_.get(),
                                           new_session.get());
        ASSERT_TRUE(new_session);
      }

      // Now the session's lifetime is bound by the auth timeout.
      g_current_time.tv_sec = auth_end_time - 1;
      TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                      new_session.get(),
                                      true /* expect session reused */));

      g_current_time.tv_sec = auth_end_time + 1;
      TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                      new_session.get(),
                                      false /* expect session ot reused */));
    } else {
      // The new session is usable just before the old expiration.
      g_current_time.tv_sec = kStartTime + timeout - 1;
      TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                      new_session.get(),
                                      true /* expect session reused */));

      // Renewal does not extend the lifetime, so it is not usable beyond the
      // old expiration.
      g_current_time.tv_sec = kStartTime + timeout + 1;
      TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                      new_session.get(),
                                      false /* expect session not reused */));
    }
  }
}



TEST_P(SSLVersionTest, DefaultTicketKeyInitialization) {
  // Do not make static and const. See t/P118709392.
  // It can trigger https://gcc.gnu.org/bugzilla/show_bug.cgi?id=95189 leading
  // to transient errors.
  uint8_t kZeroKey[kTicketKeyLen] = {0};
  uint8_t ticket_key[kTicketKeyLen];
  ASSERT_EQ(1, SSL_CTX_get_tlsext_ticket_keys(server_ctx_.get(), ticket_key,
                                              kTicketKeyLen));
  ASSERT_NE(0, OPENSSL_memcmp(ticket_key, kZeroKey, kTicketKeyLen));
}

TEST_P(SSLVersionTest, DefaultTicketKeyRotation) {
  static const time_t kStartTime = 1001;
  g_current_time.tv_sec = kStartTime;

  // We use session reuse as a proxy for ticket decryption success, hence
  // disable session timeouts.
  SSL_CTX_set_timeout(server_ctx_.get(), std::numeric_limits<uint32_t>::max());
  SSL_CTX_set_session_psk_dhe_timeout(server_ctx_.get(),
                                      std::numeric_limits<uint32_t>::max());

  SSL_CTX_set_current_time_cb(client_ctx_.get(), FrozenTimeCallback);
  SSL_CTX_set_current_time_cb(server_ctx_.get(), CurrentTimeCallback);

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_OFF);

  // Initialize ticket_key with the current key and check that it was
  // initialized to something, not all zeros.
  uint8_t ticket_key[kTicketKeyLen] = {0};
  TRACED_CALL(ExpectTicketKeyChanged(server_ctx_.get(), ticket_key,
                                     true /* changed */));

  // Verify ticket resumption actually works.
  bssl::UniquePtr<SSL> client, server;
  bssl::UniquePtr<SSL_SESSION> session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(session);
  TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                  session.get(), true /* reused */));

  // Advance time to just before key rotation.
  g_current_time.tv_sec += SSL_DEFAULT_TICKET_KEY_ROTATION_INTERVAL - 1;
  TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                  session.get(), true /* reused */));
  TRACED_CALL(ExpectTicketKeyChanged(server_ctx_.get(), ticket_key,
                                     false /* NOT changed */));

  // Force key rotation.
  g_current_time.tv_sec += 1;
  bssl::UniquePtr<SSL_SESSION> new_session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  TRACED_CALL(ExpectTicketKeyChanged(server_ctx_.get(), ticket_key,
                                     true /* changed */));

  // Resumption with both old and new ticket should work.
  TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                  session.get(), true /* reused */));
  TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                  new_session.get(), true /* reused */));
  TRACED_CALL(ExpectTicketKeyChanged(server_ctx_.get(), ticket_key,
                                     false /* NOT changed */));

  // Force key rotation again. Resumption with the old ticket now fails.
  g_current_time.tv_sec += SSL_DEFAULT_TICKET_KEY_ROTATION_INTERVAL;
  TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                  session.get(), false /* NOT reused */));
  TRACED_CALL(ExpectTicketKeyChanged(server_ctx_.get(), ticket_key,
                                     true /* changed */));

  // But resumption with the newer session still works.
  TRACED_CALL(ExpectSessionReused(client_ctx_.get(), server_ctx_.get(),
                                  new_session.get(), true /* reused */));
}

static int SwitchContext(SSL *ssl, int *out_alert, void *arg) {
  SSL_CTX *ctx = reinterpret_cast<SSL_CTX *>(arg);
  SSL_set_SSL_CTX(ssl, ctx);
  return SSL_TLSEXT_ERR_OK;
}

TEST_P(SSLVersionTest, SNICallback) {
  bssl::UniquePtr<X509> cert2 = GetECDSATestCertificate();
  ASSERT_TRUE(cert2);
  bssl::UniquePtr<EVP_PKEY> key2 = GetECDSATestKey();
  ASSERT_TRUE(key2);

  // Test that switching the |SSL_CTX| at the SNI callback behaves correctly.
  static const uint16_t kECDSAWithSHA256 = SSL_SIGN_ECDSA_SECP256R1_SHA256;

  static const uint8_t kSCTList[] = {0, 6, 0, 4, 5, 6, 7, 8};
  static const uint8_t kOCSPResponse[] = {1, 2, 3, 4};

  bssl::UniquePtr<SSL_CTX> server_ctx2 = CreateContext();
  ASSERT_TRUE(server_ctx2);
  ASSERT_TRUE(SSL_CTX_use_certificate(server_ctx2.get(), cert2.get()));
  ASSERT_TRUE(SSL_CTX_use_PrivateKey(server_ctx2.get(), key2.get()));
  ASSERT_TRUE(SSL_CTX_set_signed_cert_timestamp_list(
      server_ctx2.get(), kSCTList, sizeof(kSCTList)));
  ASSERT_TRUE(SSL_CTX_set_ocsp_response(server_ctx2.get(), kOCSPResponse,
                                        sizeof(kOCSPResponse)));
  // Historically signing preferences would be lost in some cases with the
  // SNI callback, which triggers the TLS 1.2 SHA-1 default. To ensure
  // this doesn't happen when |version| is TLS 1.2, configure the private
  // key to only sign SHA-256.
  ASSERT_TRUE(SSL_CTX_set_signing_algorithm_prefs(server_ctx2.get(),
                                                  &kECDSAWithSHA256, 1));

  SSL_CTX_set_tlsext_servername_callback(server_ctx_.get(), SwitchContext);
  SSL_CTX_set_tlsext_servername_arg(server_ctx_.get(), server_ctx2.get());

  SSL_CTX_enable_signed_cert_timestamps(client_ctx_.get());
  SSL_CTX_enable_ocsp_stapling(client_ctx_.get());

  ASSERT_TRUE(Connect());

  // The client should have received |cert2|.
  bssl::UniquePtr<X509> peer(SSL_get_peer_certificate(client_.get()));
  ASSERT_TRUE(peer);
  EXPECT_EQ(X509_cmp(peer.get(), cert2.get()), 0);

  // The client should have received |server_ctx2|'s SCT list.
  const uint8_t *data = nullptr;
  size_t len = 0;
  SSL_get0_signed_cert_timestamp_list(client_.get(), &data, &len);
  EXPECT_EQ(Bytes(kSCTList), Bytes(data, len));

  // The client should have received |server_ctx2|'s OCSP response.
  SSL_get0_ocsp_response(client_.get(), &data, &len);
  EXPECT_EQ(Bytes(kOCSPResponse), Bytes(data, len));
}


TEST_P(SSLVersionTest, Version) {
  ASSERT_TRUE(Connect());

  EXPECT_EQ(SSL_version(client_.get()), version());
  EXPECT_EQ(SSL_version(server_.get()), version());

  // Test the version name is reported as expected.
  const char *version_name = GetVersionName(version());
  EXPECT_EQ(strcmp(version_name, SSL_get_version(client_.get())), 0);
  EXPECT_EQ(strcmp(version_name, SSL_get_version(server_.get())), 0);

  // Test SSL_SESSION reports the same name.
  const char *client_name =
      SSL_SESSION_get_version(SSL_get_session(client_.get()));
  const char *server_name =
      SSL_SESSION_get_version(SSL_get_session(server_.get()));
  EXPECT_EQ(strcmp(version_name, client_name), 0);
  EXPECT_EQ(strcmp(version_name, server_name), 0);

  // Client/server version equality asserted above, assert equality for cipher
  // here.
  ASSERT_TRUE(SSL_get_current_cipher(client_.get()));
  ASSERT_TRUE(SSL_get_current_cipher(server_.get()));
  EXPECT_EQ(SSL_get_current_cipher(client_.get())->id,
            SSL_get_current_cipher(server_.get())->id);
  const uint16_t version = SSL_version(client_.get());
  if (version == TLS1_2_VERSION || version == TLS1_3_VERSION) {
    const char *version_str = SSL_get_version(client_.get());
    EXPECT_STREQ(version_str,
                 SSL_CIPHER_get_version(SSL_get_current_cipher(client_.get())));
  } else if (version == DTLS1_2_VERSION) {  // ciphers don't differentiate D/TLS
    EXPECT_STREQ("TLSv1.2",
                 SSL_CIPHER_get_version(SSL_get_current_cipher(client_.get())));
  } else {
    EXPECT_STREQ("TLSv1/SSLv3",
                 SSL_CIPHER_get_version(SSL_get_current_cipher(client_.get())));
  }
}

// Tests that that |SSL_get_pending_cipher| is available during the ALPN
// selection callback.
TEST_P(SSLVersionTest, ALPNCipherAvailable) {
  ASSERT_TRUE(UseCertAndKey(client_ctx_.get()));

  static const uint8_t kALPNProtos[] = {0x03, 'f', 'o', 'o'};
  ASSERT_EQ(SSL_CTX_set_alpn_protos(client_ctx_.get(), kALPNProtos,
                                    sizeof(kALPNProtos)),
            0);

  // The ALPN callback does not fail the handshake on error, so have the
  // callback write a boolean.
  std::pair<uint16_t, bool> callback_state(version(), false);
  SSL_CTX_set_alpn_select_cb(
      server_ctx_.get(),
      [](SSL *ssl, const uint8_t **out, uint8_t *out_len, const uint8_t *in,
         unsigned in_len, void *arg) -> int {
        auto state = reinterpret_cast<std::pair<uint16_t, bool> *>(arg);
        if (SSL_get_pending_cipher(ssl) != nullptr &&
            SSL_version(ssl) == state->first) {
          state->second = true;
        }
        return SSL_TLSEXT_ERR_NOACK;
      },
      &callback_state);

  ASSERT_TRUE(Connect());

  ASSERT_TRUE(callback_state.second);
}

TEST_P(SSLVersionTest, SSLClearSessionResumption) {
  // Skip this for TLS 1.3. TLS 1.3's ticket mechanism is incompatible with this
  // API pattern.
  if (version() == TLS1_3_VERSION) {
    return;
  }

  shed_handshake_config_ = false;
  ASSERT_TRUE(Connect());

  EXPECT_FALSE(SSL_session_reused(client_.get()));
  EXPECT_FALSE(SSL_session_reused(server_.get()));

  // Reset everything.
  ASSERT_TRUE(SSL_clear(client_.get()));
  ASSERT_TRUE(SSL_clear(server_.get()));

  // Attempt to connect a second time.
  ASSERT_TRUE(CompleteHandshakes(client_.get(), server_.get()));

  // |SSL_clear| should implicitly offer the previous session to the server.
  EXPECT_TRUE(SSL_session_reused(client_.get()));
  EXPECT_TRUE(SSL_session_reused(server_.get()));
}

TEST_P(SSLVersionTest, SSLClearFailsWithShedding) {
  shed_handshake_config_ = false;
  ASSERT_TRUE(Connect());
  ASSERT_TRUE(CompleteHandshakes(client_.get(), server_.get()));

  // Reset everything.
  ASSERT_TRUE(SSL_clear(client_.get()));
  ASSERT_TRUE(SSL_clear(server_.get()));

  // Now enable shedding, and connect a second time.
  shed_handshake_config_ = true;
  ASSERT_TRUE(Connect());
  ASSERT_TRUE(CompleteHandshakes(client_.get(), server_.get()));

  // |SSL_clear| should now fail.
  ASSERT_FALSE(SSL_clear(client_.get()));
  ASSERT_FALSE(SSL_clear(server_.get()));
}

TEST_P(SSLVersionTest, SSLClientCiphers) {
  // Client ciphers ARE NOT SERIALIZED, so skip tests that rely on transfer or
  // serialization of |ssl| and accompanying objects under test.
  if (getVersionParam().transfer_ssl) {
    return;
  }

  EXPECT_FALSE(SSL_get_client_ciphers(client_.get()));
  EXPECT_FALSE(SSL_get_client_ciphers(server_.get()));

  shed_handshake_config_ = false;
  ASSERT_TRUE(Connect());

  // The client should still have no view of the server's preferences, but the
  // server should have seen at least one cipher from the client.
  EXPECT_FALSE(SSL_get_client_ciphers(client_.get()));
  EXPECT_GT(sk_SSL_CIPHER_num(SSL_get_client_ciphers(server_.get())),
            (size_t)0);

  // With config shedding disabled, clearing |server| shouldn't error and
  // should reset server's client ciphers
  ASSERT_TRUE(SSL_clear(server_.get()));
  EXPECT_FALSE(SSL_get_client_ciphers(server_.get()));

  shed_handshake_config_ = true;
  ASSERT_TRUE(Connect());

  // These should be unaffected by config shedding
  EXPECT_FALSE(SSL_get_client_ciphers(client_.get()));
  EXPECT_GT(sk_SSL_CIPHER_num(SSL_get_client_ciphers(server_.get())),
            (size_t)0);
}



TEST_P(SSLVersionTest, AutoChain) {
  cert_ = GetChainTestCertificate();
  ASSERT_TRUE(cert_);
  key_ = GetChainTestKey();
  ASSERT_TRUE(key_);
  bssl::UniquePtr<X509> intermediate = GetChainTestIntermediate();
  ASSERT_TRUE(intermediate);

  ASSERT_TRUE(UseCertAndKey(client_ctx_.get()));
  ASSERT_TRUE(UseCertAndKey(server_ctx_.get()));

  // Configure both client and server to accept any certificate. Add
  // |intermediate| to the cert store.
  ASSERT_TRUE(X509_STORE_add_cert(SSL_CTX_get_cert_store(client_ctx_.get()),
                                  intermediate.get()));
  ASSERT_TRUE(X509_STORE_add_cert(SSL_CTX_get_cert_store(server_ctx_.get()),
                                  intermediate.get()));
  SSL_CTX_set_verify(client_ctx_.get(),
                     SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT,
                     nullptr);
  SSL_CTX_set_verify(server_ctx_.get(),
                     SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT,
                     nullptr);
  SSL_CTX_set_cert_verify_callback(client_ctx_.get(), VerifySucceed, NULL);
  SSL_CTX_set_cert_verify_callback(server_ctx_.get(), VerifySucceed, NULL);

  // By default, the client and server should each only send the leaf.
  ASSERT_TRUE(Connect());

  EXPECT_TRUE(
      ChainsEqual(SSL_get_peer_full_cert_chain(client_.get()), {cert_.get()}));
  EXPECT_TRUE(
      ChainsEqual(SSL_get_peer_full_cert_chain(server_.get()), {cert_.get()}));
  // This test overrides the verification logic  with VerifySucceed to always
  // succeed without actually verifying anything and setting the verified chain
  // on success
  EXPECT_EQ(SSL_get0_verified_chain(server_.get()), nullptr);
  EXPECT_EQ(SSL_get0_verified_chain(client_.get()), nullptr);

  // If auto-chaining is enabled, then the intermediate is sent.
  SSL_CTX_clear_mode(client_ctx_.get(), SSL_MODE_NO_AUTO_CHAIN);
  SSL_CTX_clear_mode(server_ctx_.get(), SSL_MODE_NO_AUTO_CHAIN);
  ASSERT_TRUE(Connect());

  EXPECT_TRUE(ChainsEqual(SSL_get_peer_full_cert_chain(client_.get()),
                          {cert_.get(), intermediate.get()}));
  EXPECT_TRUE(ChainsEqual(SSL_get_peer_full_cert_chain(server_.get()),
                          {cert_.get(), intermediate.get()}));

  // Auto-chaining does not override explicitly-configured intermediates.
  ASSERT_TRUE(SSL_CTX_add1_chain_cert(client_ctx_.get(), cert_.get()));
  ASSERT_TRUE(SSL_CTX_add1_chain_cert(server_ctx_.get(), cert_.get()));
  ASSERT_TRUE(Connect());

  EXPECT_TRUE(ChainsEqual(SSL_get_peer_full_cert_chain(client_.get()),
                          {cert_.get(), cert_.get()}));

  EXPECT_TRUE(ChainsEqual(SSL_get_peer_full_cert_chain(server_.get()),
                          {cert_.get(), cert_.get()}));
}


// These tests test multi-threaded behavior. They are intended to run with
// ThreadSanitizer.
#if defined(OPENSSL_THREADS)
TEST_P(SSLVersionTest, SessionCacheThreads) {
  SSL_CTX_set_options(server_ctx_.get(), SSL_OP_NO_TICKET);
  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);

  if (version() == TLS1_3_VERSION) {
    // Our TLS 1.3 implementation does not support stateful resumption.
    ASSERT_FALSE(CreateClientSession(client_ctx_.get(), server_ctx_.get()));
    return;
  }

  // Establish two client sessions to test with.
  bssl::UniquePtr<SSL_SESSION> session1 =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(session1);
  bssl::UniquePtr<SSL_SESSION> session2 =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(session2);

  auto connect_with_session = [&](SSL_SESSION *session) {
    ClientConfig config;
    config.session = session;
    UniquePtr<SSL> client, server;
    ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                       server_ctx_.get(), config));
  };

  // Resume sessions in parallel with establishing new ones.
  {
    std::vector<std::thread> threads;
    threads.emplace_back([&] { connect_with_session(nullptr); });
    threads.emplace_back([&] { connect_with_session(nullptr); });
    threads.emplace_back([&] { connect_with_session(session1.get()); });
    threads.emplace_back([&] { connect_with_session(session1.get()); });
    threads.emplace_back([&] { connect_with_session(session2.get()); });
    threads.emplace_back([&] { connect_with_session(session2.get()); });
    for (auto &thread : threads) {
      thread.join();
    }
  }

  // Hit the maximum session cache size across multiple threads, to test the
  // size enforcement logic.
  size_t over_limit = 2;
  size_t limit = SSL_CTX_sess_number(server_ctx_.get()) + over_limit;
  SSL_CTX_sess_set_cache_size(server_ctx_.get(), limit);
  {
    std::vector<std::thread> threads;
    for (int i = 0; i < 4; i++) {
      threads.emplace_back([&]() {
        connect_with_session(nullptr);
        EXPECT_LE(SSL_CTX_sess_number(server_ctx_.get()), limit);
      });
    }
    for (auto &thread : threads) {
      thread.join();
    }
    EXPECT_EQ(SSL_CTX_sess_number(server_ctx_.get()), limit);
    // We go over the cache limit by |over_limit|.
    EXPECT_EQ(SSL_CTX_sess_cache_full(server_ctx_.get()), (int)over_limit);
  }

  // Reset the session cache, this time with a mock clock.
  ASSERT_NO_FATAL_FAILURE(ResetContexts());
  SSL_CTX_set_options(server_ctx_.get(), SSL_OP_NO_TICKET);
  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_current_time_cb(server_ctx_.get(), CurrentTimeCallback);

  // Make some sessions at an arbitrary start time. Then expire them.
  g_current_time.tv_sec = 1000;
  bssl::UniquePtr<SSL_SESSION> expired_session1 =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(expired_session1);
  bssl::UniquePtr<SSL_SESSION> expired_session2 =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(expired_session2);
  g_current_time.tv_sec += 100 * SSL_DEFAULT_SESSION_TIMEOUT;

  session1 = CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(session1);

  // Every 256 connections, we flush stale sessions from the session cache. Test
  // this logic is correctly synchronized with other connection attempts.
  static const int kNumConnections = 256;
  {
    std::vector<std::thread> threads;
    threads.emplace_back([&] {
      for (int i = 0; i < kNumConnections; i++) {
        connect_with_session(nullptr);
      }
    });
    threads.emplace_back([&] {
      for (int i = 0; i < kNumConnections; i++) {
        connect_with_session(nullptr);
      }
    });
    threads.emplace_back([&] {
      // Never connect with |expired_session2|. The session cache eagerly
      // removes expired sessions when it sees them. Leaving |expired_session2|
      // untouched ensures it is instead cleared by periodic flushing.
      for (int i = 0; i < kNumConnections; i++) {
        connect_with_session(expired_session1.get());
      }
    });
    threads.emplace_back([&] {
      for (int i = 0; i < kNumConnections; i++) {
        connect_with_session(session1.get());
      }
    });
    for (auto &thread : threads) {
      thread.join();
    }
  }
}

TEST_P(SSLVersionTest, SessionTicketThreads) {
  for (bool renew_ticket : {false, true}) {
    SCOPED_TRACE(renew_ticket);
    ASSERT_NO_FATAL_FAILURE(ResetContexts());
    SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
    SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);
    if (renew_ticket) {
      SSL_CTX_set_tlsext_ticket_key_cb(server_ctx_.get(), RenewTicketCallback);
    }

    // Establish two client sessions to test with.
    bssl::UniquePtr<SSL_SESSION> session1 =
        CreateClientSession(client_ctx_.get(), server_ctx_.get());
    ASSERT_TRUE(session1);
    bssl::UniquePtr<SSL_SESSION> session2 =
        CreateClientSession(client_ctx_.get(), server_ctx_.get());
    ASSERT_TRUE(session2);

    auto connect_with_session = [&](SSL_SESSION *session) {
      ClientConfig config;
      config.session = session;
      UniquePtr<SSL> client, server;
      ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                         server_ctx_.get(), config));
    };

    // Resume sessions in parallel with establishing new ones.
    {
      std::vector<std::thread> threads;
      threads.emplace_back([&] { connect_with_session(nullptr); });
      threads.emplace_back([&] { connect_with_session(nullptr); });
      threads.emplace_back([&] { connect_with_session(session1.get()); });
      threads.emplace_back([&] { connect_with_session(session1.get()); });
      threads.emplace_back([&] { connect_with_session(session2.get()); });
      threads.emplace_back([&] { connect_with_session(session2.get()); });
      for (auto &thread : threads) {
        thread.join();
      }
    }
  }
}
#endif  // OPENSSL_THREADS


TEST_P(SSLVersionTest, SSLWriteRetry) {
  if (is_dtls()) {
    return;
  }

  for (bool enable_partial_write : {false, true}) {
    SCOPED_TRACE(enable_partial_write);

    // Connect a client and server.
    ASSERT_TRUE(Connect());

    if (enable_partial_write) {
      SSL_set_mode(client_.get(), SSL_MODE_ENABLE_PARTIAL_WRITE);
    }

    // Write without reading until the buffer is full and we have an unfinished
    // write. Keep a count so we may reread it again later. "hello!" will be
    // written in two chunks, "hello" and "!".
    char data[] = "hello!";
    static const int kChunkLen = 5;  // The length of "hello".
    unsigned count = 0;
    for (;;) {
      int ret = SSL_write(client_.get(), data, kChunkLen);
      if (ret <= 0) {
        ASSERT_EQ(SSL_get_error(client_.get(), ret), SSL_ERROR_WANT_WRITE);
        break;
      }
      ASSERT_EQ(ret, 5);
      count++;
    }

    // Retrying with the same parameters is legal.
    ASSERT_EQ(
        SSL_get_error(client_.get(), SSL_write(client_.get(), data, kChunkLen)),
        SSL_ERROR_WANT_WRITE);

    // Retrying with the same buffer but shorter length is not legal.
    ASSERT_EQ(SSL_get_error(client_.get(),
                            SSL_write(client_.get(), data, kChunkLen - 1)),
              SSL_ERROR_SSL);
    ASSERT_TRUE(ExpectSingleError(ERR_LIB_SSL, SSL_R_BAD_WRITE_RETRY));

    // Retrying with a different buffer pointer is not legal.
    char data2[] = "hello";
    ASSERT_EQ(SSL_get_error(client_.get(),
                            SSL_write(client_.get(), data2, kChunkLen)),
              SSL_ERROR_SSL);
    ASSERT_TRUE(ExpectSingleError(ERR_LIB_SSL, SSL_R_BAD_WRITE_RETRY));

    // With |SSL_MODE_ACCEPT_MOVING_WRITE_BUFFER|, the buffer may move.
    SSL_set_mode(client_.get(), SSL_MODE_ACCEPT_MOVING_WRITE_BUFFER);
    ASSERT_EQ(SSL_get_error(client_.get(),
                            SSL_write(client_.get(), data2, kChunkLen)),
              SSL_ERROR_WANT_WRITE);

    // |SSL_MODE_ACCEPT_MOVING_WRITE_BUFFER| does not disable length checks.
    ASSERT_EQ(SSL_get_error(client_.get(),
                            SSL_write(client_.get(), data2, kChunkLen - 1)),
              SSL_ERROR_SSL);
    ASSERT_TRUE(ExpectSingleError(ERR_LIB_SSL, SSL_R_BAD_WRITE_RETRY));

    // Retrying with a larger buffer is legal.
    ASSERT_EQ(SSL_get_error(client_.get(),
                            SSL_write(client_.get(), data, kChunkLen + 1)),
              SSL_ERROR_WANT_WRITE);

    // Drain the buffer.
    char buf[20];
    for (unsigned i = 0; i < count; i++) {
      ASSERT_EQ(SSL_read(server_.get(), buf, sizeof(buf)), kChunkLen);
      ASSERT_EQ(OPENSSL_memcmp(buf, "hello", kChunkLen), 0);
    }

    // Now that there is space, a retry with a larger buffer should flush the
    // pending record, skip over that many bytes of input (on assumption they
    // are the same), and write the remainder. If SSL_MODE_ENABLE_PARTIAL_WRITE
    // is set, this will complete in two steps.
    char data_longer[] = "_____!!!!!";
    if (enable_partial_write) {
      ASSERT_EQ(SSL_write(client_.get(), data_longer, 2 * kChunkLen),
                kChunkLen);
      ASSERT_EQ(SSL_write(client_.get(), data_longer + kChunkLen, kChunkLen),
                kChunkLen);
    } else {
      ASSERT_EQ(SSL_write(client_.get(), data_longer, 2 * kChunkLen),
                2 * kChunkLen);
    }

    // Check the last write was correct. The data will be spread over two
    // records, so SSL_read returns twice.
    ASSERT_EQ(SSL_read(server_.get(), buf, sizeof(buf)), kChunkLen);
    ASSERT_EQ(OPENSSL_memcmp(buf, "hello", kChunkLen), 0);
    ASSERT_EQ(SSL_read(server_.get(), buf, sizeof(buf)), kChunkLen);
    ASSERT_EQ(OPENSSL_memcmp(buf, "!!!!!", kChunkLen), 0);

    // Fill the transport buffer again. This time only leave room for one
    // record.
    count = 0;
    for (;;) {
      int ret = SSL_write(client_.get(), data, kChunkLen);
      if (ret <= 0) {
        ASSERT_EQ(SSL_get_error(client_.get(), ret), SSL_ERROR_WANT_WRITE);
        break;
      }
      ASSERT_EQ(ret, 5);
      count++;
    }
    ASSERT_EQ(SSL_read(server_.get(), buf, sizeof(buf)), kChunkLen);
    ASSERT_EQ(OPENSSL_memcmp(buf, "hello", kChunkLen), 0);
    count--;

    // Retry the last write, with a longer input. The first half is the most
    // recently failed write, from filling the buffer. |SSL_write| should write
    // that to the transport, and then attempt to write the second half.
    int ret = SSL_write(client_.get(), data_longer, 2 * kChunkLen);
    if (enable_partial_write) {
      // If partial writes are allowed, the write will succeed partially.
      ASSERT_EQ(ret, kChunkLen);

      // Check the first half and make room for another record.
      ASSERT_EQ(SSL_read(server_.get(), buf, sizeof(buf)), kChunkLen);
      ASSERT_EQ(OPENSSL_memcmp(buf, "hello", kChunkLen), 0);
      count--;

      // Finish writing the input.
      ASSERT_EQ(SSL_write(client_.get(), data_longer + kChunkLen, kChunkLen),
                kChunkLen);
    } else {
      if (enable_read_ahead()) {
        // The client and server are sharing a BIO_pair which by default only
        // allows 17 * 1024 bytes to be buffered in the shared BIO. This test
        // relies on the buffer being full here. But if the client is reading
        // ahead it is pulling data out of the BIO_pair's buffer and into it's
        // own SSLBuffer freeing up space for the write above
        ASSERT_EQ(ret, 2 * kChunkLen);
      } else {
        // Otherwise, although the first half made it to the transport, the
        // second half is blocked.
        ASSERT_EQ(ret, -1);
        ASSERT_EQ(SSL_get_error(client_.get(), -1), SSL_ERROR_WANT_WRITE);
        // Check the first half and make room for another record.
        ASSERT_EQ(SSL_read(server_.get(), buf, sizeof(buf)), kChunkLen);
        ASSERT_EQ(OPENSSL_memcmp(buf, "hello", kChunkLen), 0);
        count--;

        // Retrying with fewer bytes than previously attempted is an error. If
        // the input length is less than the number of bytes successfully
        // written, the check happens at a different point, with a different
        // error.
        //
        // TODO(davidben): Should these cases use the same error?
        ASSERT_EQ(
            SSL_get_error(client_.get(),
                          SSL_write(client_.get(), data_longer, kChunkLen - 1)),
            SSL_ERROR_SSL);
        ASSERT_TRUE(ExpectSingleError(ERR_LIB_SSL, SSL_R_BAD_LENGTH));

        // Complete the write with the correct retry.
        ASSERT_EQ(SSL_write(client_.get(), data_longer, 2 * kChunkLen),
                  2 * kChunkLen);
      }
    }

    // Drain the input and ensure everything was written correctly.
    for (unsigned i = 0; i < count; i++) {
      ASSERT_EQ(SSL_read(server_.get(), buf, sizeof(buf)), kChunkLen);
      ASSERT_EQ(OPENSSL_memcmp(buf, "hello", kChunkLen), 0);
    }

    // The final write is spread over two records.
    ASSERT_EQ(SSL_read(server_.get(), buf, sizeof(buf)), kChunkLen);
    ASSERT_EQ(OPENSSL_memcmp(buf, "hello", kChunkLen), 0);
    ASSERT_EQ(SSL_read(server_.get(), buf, sizeof(buf)), kChunkLen);
    ASSERT_EQ(OPENSSL_memcmp(buf, "!!!!!", kChunkLen), 0);
  }
}

TEST_P(SSLVersionTest, RecordCallback) {
  for (bool test_server : {true, false}) {
    SCOPED_TRACE(test_server);
    ASSERT_NO_FATAL_FAILURE(ResetContexts());

    bool read_seen = false;
    bool write_seen = false;
    auto cb = [&](int is_write, int cb_version, int cb_type, const void *buf,
                  size_t len, SSL *ssl) {
      if (cb_type != SSL3_RT_HEADER) {
        return;
      }

      // The callback does not report a version for records.
      EXPECT_EQ(0, cb_version);

      if (is_write) {
        write_seen = true;
      } else {
        read_seen = true;
      }

      // Sanity-check that the record header is plausible.
      CBS cbs;
      CBS_init(&cbs, reinterpret_cast<const uint8_t *>(buf), len);
      uint8_t type = 0;
      uint16_t record_version = 0, length = 0;
      ASSERT_TRUE(CBS_get_u8(&cbs, &type));
      ASSERT_TRUE(CBS_get_u16(&cbs, &record_version));
      EXPECT_EQ(record_version & 0xff00, version() & 0xff00);
      if (is_dtls()) {
        uint16_t epoch = 0;
        ASSERT_TRUE(CBS_get_u16(&cbs, &epoch));
        EXPECT_TRUE(epoch == 0 || epoch == 1) << "Invalid epoch: " << epoch;
        ASSERT_TRUE(CBS_skip(&cbs, 6));
      }
      ASSERT_TRUE(CBS_get_u16(&cbs, &length));
      EXPECT_EQ(0u, CBS_len(&cbs));
    };
    using CallbackType = decltype(cb);
    SSL_CTX *ctx = test_server ? server_ctx_.get() : client_ctx_.get();
    SSL_CTX_set_msg_callback(
        ctx, [](int is_write, int cb_version, int cb_type, const void *buf,
                size_t len, SSL *ssl, void *arg) {
          CallbackType *cb_ptr = reinterpret_cast<CallbackType *>(arg);
          (*cb_ptr)(is_write, cb_version, cb_type, buf, len, ssl);
        });
    SSL_CTX_set_msg_callback_arg(ctx, &cb);

    ASSERT_TRUE(Connect());

    EXPECT_TRUE(read_seen);
    EXPECT_TRUE(write_seen);
  }
}

TEST_P(SSLVersionTest, GetServerName) {
  ClientConfig config;
  config.servername = "host1";

  SSL_CTX_set_tlsext_servername_callback(
      server_ctx_.get(), [](SSL *ssl, int *out_alert, void *arg) -> int {
        // During the handshake, |SSL_get_servername| must match |config|.
        ClientConfig *config_p = reinterpret_cast<ClientConfig *>(arg);
        EXPECT_STREQ(config_p->servername.c_str(),
                     SSL_get_servername(ssl, TLSEXT_NAMETYPE_host_name));
        return SSL_TLSEXT_ERR_OK;
      });
  SSL_CTX_set_tlsext_servername_arg(server_ctx_.get(), &config);

  ASSERT_TRUE(Connect(config));
  // After the handshake, it must also be available.
  EXPECT_STREQ(config.servername.c_str(),
               SSL_get_servername(server_.get(), TLSEXT_NAMETYPE_host_name));

  // Establish a session under host1.
  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);
  bssl::UniquePtr<SSL_SESSION> session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get(), config);

  // If the client resumes a session with a different name, |SSL_get_servername|
  // must return the new name.
  ASSERT_TRUE(session);
  config.session = session.get();
  config.servername = "host2";
  ASSERT_TRUE(Connect(config));
  EXPECT_STREQ(config.servername.c_str(),
               SSL_get_servername(server_.get(), TLSEXT_NAMETYPE_host_name));
}

TEST_P(SSLVersionTest, GetState) {
  ClientConfig config;
  ASSERT_TRUE(Connect(config));
  int server_state = SSL_get_state(server_.get());
  EXPECT_EQ(server_state, TLS_ST_OK);
  EXPECT_EQ(server_state, SSL_ST_OK);
  int client_state = SSL_state(client_.get());
  EXPECT_EQ(client_state, TLS_ST_OK);
  EXPECT_EQ(client_state, SSL_ST_OK);
}

// Test that session cache mode bits are honored in the client session callback.
TEST_P(SSLVersionTest, ClientSessionCacheMode) {
  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_OFF);
  EXPECT_FALSE(CreateClientSession(client_ctx_.get(), server_ctx_.get()));

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_CLIENT);
  EXPECT_TRUE(CreateClientSession(client_ctx_.get(), server_ctx_.get()));

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_SERVER);
  EXPECT_FALSE(CreateClientSession(client_ctx_.get(), server_ctx_.get()));
}

// Test that all versions survive tiny write buffers. In particular, TLS 1.3
// NewSessionTickets are written post-handshake. Servers that block
// |SSL_do_handshake| on writing them will deadlock if clients are not draining
// the buffer. Test that we do not do this.
TEST_P(SSLVersionTest, SmallBuffer) {
  // DTLS is a datagram protocol and requires packet-sized buffers.
  if (is_dtls()) {
    return;
  }

  // Test both flushing NewSessionTickets with a zero-sized write and
  // non-zero-sized write.
  for (bool use_zero_write : {false, true}) {
    SCOPED_TRACE(use_zero_write);

    g_last_session = nullptr;
    SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
    SSL_CTX_sess_set_new_cb(client_ctx_.get(), SaveLastSession);

    bssl::UniquePtr<SSL> client(SSL_new(client_ctx_.get())),
        server(SSL_new(server_ctx_.get()));
    ASSERT_TRUE(client);
    ASSERT_TRUE(server);
    SSL_set_connect_state(client.get());
    SSL_set_accept_state(server.get());

    // Use a tiny buffer.
    BIO *bio1 = nullptr, *bio2 = nullptr;
    ASSERT_TRUE(BIO_new_bio_pair(&bio1, 1, &bio2, 1));

    // SSL_set_bio takes ownership.
    SSL_set_bio(client.get(), bio1, bio1);
    SSL_set_bio(server.get(), bio2, bio2);

    ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));
    if (version() >= TLS1_3_VERSION) {
      // The post-handshake ticket should not have been processed yet.
      EXPECT_FALSE(g_last_session);
    }

    if (use_zero_write) {
      ASSERT_TRUE(FlushNewSessionTickets(client.get(), server.get()));
      EXPECT_TRUE(g_last_session);
    }

    // Send some data from server to client. If |use_zero_write| is false, this
    // will also flush the NewSessionTickets.
    static const char kMessage[] = "hello world";
    char buf[sizeof(kMessage)];
    for (;;) {
      int server_ret = SSL_write(server.get(), kMessage, sizeof(kMessage));
      int server_err = SSL_get_error(server.get(), server_ret);
      int client_ret = SSL_read(client.get(), buf, sizeof(buf));
      int client_err = SSL_get_error(client.get(), client_ret);

      // The server will write a single record, so every iteration should see
      // |SSL_ERROR_WANT_WRITE| and |SSL_ERROR_WANT_READ|, until the final
      // iteration, where both will complete.
      if (server_ret > 0) {
        EXPECT_EQ(server_ret, static_cast<int>(sizeof(kMessage)));
        EXPECT_EQ(client_ret, static_cast<int>(sizeof(kMessage)));
        EXPECT_EQ(Bytes(buf), Bytes(kMessage));
        break;
      }

      ASSERT_EQ(server_ret, -1);
      ASSERT_EQ(server_err, SSL_ERROR_WANT_WRITE);
      ASSERT_EQ(client_ret, -1);
      ASSERT_EQ(client_err, SSL_ERROR_WANT_READ);
    }

    // The NewSessionTickets should have been flushed and processed.
    EXPECT_TRUE(g_last_session);
  }
}


TEST_P(SSLVersionTest, SessionVersion) {
  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);

  bssl::UniquePtr<SSL_SESSION> session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(session);
  EXPECT_EQ(version(), SSL_SESSION_get_protocol_version(session.get()));

  // Sessions in TLS 1.3 and later should be single-use.
  EXPECT_EQ(version() == TLS1_3_VERSION,
            !!SSL_SESSION_should_be_single_use(session.get()));

  // Making fake sessions for testing works.
  session.reset(SSL_SESSION_new(client_ctx_.get()));
  ASSERT_TRUE(session);
  ASSERT_TRUE(SSL_SESSION_set_protocol_version(session.get(), version()));
  EXPECT_EQ(version(), SSL_SESSION_get_protocol_version(session.get()));
}

TEST_P(SSLVersionTest, SSLPending) {
  UniquePtr<SSL> ssl(SSL_new(client_ctx_.get()));
  ASSERT_TRUE(ssl);
  EXPECT_EQ(0, SSL_pending(ssl.get()));

  ASSERT_TRUE(Connect());
  EXPECT_EQ(0, SSL_pending(client_.get()));
  EXPECT_EQ(0, SSL_has_pending(client_.get()));

  ASSERT_EQ(5, SSL_write(server_.get(), "hello", 5));
  ASSERT_EQ(5, SSL_write(server_.get(), "world", 5));
  EXPECT_EQ(0, SSL_pending(client_.get()));
  EXPECT_EQ(0, SSL_has_pending(client_.get()));

  char buf[10];
  ASSERT_EQ(1, SSL_peek(client_.get(), buf, 1));
  EXPECT_EQ(5, SSL_pending(client_.get()));
  EXPECT_EQ(1, SSL_has_pending(client_.get()));

  ASSERT_EQ(1, SSL_read(client_.get(), buf, 1));
  EXPECT_EQ(4, SSL_pending(client_.get()));
  EXPECT_EQ(1, SSL_has_pending(client_.get()));

  ASSERT_EQ(4, SSL_read(client_.get(), buf, 10));
  EXPECT_EQ(0, SSL_pending(client_.get()));
  if (is_dtls()) {
    // In DTLS, the two records would have been read as a single datagram and
    // buffered inside |client_|. Thus, |SSL_has_pending| should return true.
    //
    // This test is slightly unrealistic. It relies on |ConnectClientAndServer|
    // using a |BIO| pair, which does not preserve datagram boundaries. Reading
    // 1 byte, then 4 bytes, from the first record also relies on
    // https://crbug.com/boringssl/65. But it does test the codepaths. When
    // fixing either of these bugs, this test may need to be redone.
    EXPECT_EQ(1, SSL_has_pending(client_.get()));
  } else {
    // In TLS if read ahead is enabled the two records would also have been read
    // in a single call.
    EXPECT_EQ(enable_read_ahead(), SSL_has_pending(client_.get()));
  }

  ASSERT_EQ(2, SSL_read(client_.get(), buf, 2));
  EXPECT_EQ(3, SSL_pending(client_.get()));
  EXPECT_EQ(1, SSL_has_pending(client_.get()));
}

// Identical test to the |SSLPending| test suite above, but with
// |SSL_(read/peek/write)_ex| operations instead.
TEST_P(SSLVersionTest, SSLPendingEx) {
  UniquePtr<SSL> ssl(SSL_new(client_ctx_.get()));
  ASSERT_TRUE(ssl);
  EXPECT_EQ(0, SSL_pending(ssl.get()));

  ASSERT_TRUE(Connect());
  EXPECT_EQ(0, SSL_pending(client_.get()));
  EXPECT_EQ(0, SSL_has_pending(client_.get()));

  size_t buf_len = 0;
  ASSERT_EQ(1, SSL_write_ex(server_.get(), "hello", 5, &buf_len));
  ASSERT_EQ(buf_len, (size_t)5);
  ASSERT_EQ(1, SSL_write_ex(server_.get(), "world", 5, &buf_len));
  ASSERT_EQ(buf_len, (size_t)5);

  EXPECT_EQ(0, SSL_pending(client_.get()));
  EXPECT_EQ(0, SSL_has_pending(client_.get()));

  char buf[10];
  ASSERT_EQ(1, SSL_peek_ex(client_.get(), buf, 1, &buf_len));
  ASSERT_EQ(buf_len, (size_t)1);
  EXPECT_EQ(5, SSL_pending(client_.get()));
  EXPECT_EQ(1, SSL_has_pending(client_.get()));

  ASSERT_EQ(1, SSL_read_ex(client_.get(), buf, 1, &buf_len));
  ASSERT_EQ(buf_len, (size_t)1);
  EXPECT_EQ(4, SSL_pending(client_.get()));
  EXPECT_EQ(1, SSL_has_pending(client_.get()));

  ASSERT_EQ(1, SSL_read_ex(client_.get(), buf, 10, &buf_len));
  ASSERT_EQ(buf_len, (size_t)4);
  EXPECT_EQ(0, SSL_pending(client_.get()));
  if (is_dtls()) {
    EXPECT_EQ(1, SSL_has_pending(client_.get()));
  } else {
    // In TLS if read ahead is enabled the two records would also have been read
    // in a single call.
    EXPECT_EQ(enable_read_ahead(), SSL_has_pending(client_.get()));
  }

  ASSERT_EQ(1, SSL_read_ex(client_.get(), buf, 2, &buf_len));
  ASSERT_EQ(buf_len, (size_t)2);
  EXPECT_EQ(3, SSL_pending(client_.get()));
  EXPECT_EQ(1, SSL_has_pending(client_.get()));

  // 0-sized IO with valid inputs should succeed but not read/write nor effect
  // buffer state. However, NULL |read_bytes|/|written| pointer should fail.
  const int client_pending = SSL_pending(client_.get());
  ASSERT_EQ(1, SSL_read_ex(client_.get(), (void *)"", 0, &buf_len));
  ASSERT_EQ(0UL, buf_len);
  ASSERT_EQ(client_pending, SSL_pending(client_.get()));
  ASSERT_EQ(1, SSL_write_ex(client_.get(), (void *)"", 0, &buf_len));
  ASSERT_EQ(0UL, buf_len);
  ASSERT_EQ(client_pending, SSL_pending(client_.get()));
  ASSERT_EQ(0, SSL_read_ex(client_.get(), (void *)"", 0, nullptr));
  ASSERT_EQ(0, SSL_write_ex(client_.get(), (void *)"", 0, nullptr));
}

TEST_P(SSLVersionTest, ReadAhead) {
  ASSERT_TRUE(Connect());
  size_t buf_len = 0;
  std::string test_string = "Hello, world!";
  for (char &i : test_string) {
    ASSERT_EQ(1, SSL_write_ex(server_.get(), &i, 1, &buf_len));
  }

  char buf[13];
  size_t starting_size = BIO_pending(client_.get()->rbio.get());

  ASSERT_NE(0UL, starting_size);
  ASSERT_EQ(1, SSL_read_ex(client_.get(), buf, 1, &buf_len));
  if (enable_read_ahead() || is_dtls()) {
    // Even though we didn't request the full string (13 * record overhead)
    // everything should be read from the BIO
    if (read_ahead_buffer_size() > starting_size || is_dtls()) {
      ASSERT_EQ(0UL, BIO_pending(client_.get()->rbio.get()));
    } else {
      // Depending on TLS version each record will be a different size and a
      // variable but non-zero amount of data will remain
      ASSERT_NE(0UL, BIO_pending(client_.get()->rbio.get()));
    }
  } else {
    // Only the requested 1 byte + TLS record overhead should have been read,
    // the remaining 12 letters each in its own record should be in the BIO
    // not the SSLBuffer
    ASSERT_NE(0UL, BIO_pending(client_.get()->rbio.get()));
  }
}


TEST_P(SSLVersionTest, VerifyBeforeCertRequest) {
  // Configure the server to request client certificates.
  SSL_CTX_set_custom_verify(
      server_ctx_.get(), SSL_VERIFY_PEER,
      [](SSL *ssl, uint8_t *out_alert) { return ssl_verify_ok; });

  // Configure the client to reject the server certificate.
  SSL_CTX_set_custom_verify(
      client_ctx_.get(), SSL_VERIFY_PEER,
      [](SSL *ssl, uint8_t *out_alert) { return ssl_verify_invalid; });

  // cert_cb should not be called. Verification should fail first.
  SSL_CTX_set_cert_cb(
      client_ctx_.get(),
      [](SSL *ssl, void *arg) {
        ADD_FAILURE() << "cert_cb unexpectedly called";
        return 0;
      },
      nullptr);

  bssl::UniquePtr<SSL> client, server;
  EXPECT_FALSE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                      server_ctx_.get()));
}

// Test that ticket-based sessions on the client get fake session IDs.
TEST_P(SSLVersionTest, FakeIDsForTickets) {
  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);

  bssl::UniquePtr<SSL_SESSION> session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(session);

  EXPECT_TRUE(SSL_SESSION_has_ticket(session.get()));
  unsigned session_id_length = 0;
  SSL_SESSION_get_id(session.get(), &session_id_length);
  EXPECT_NE(session_id_length, 0u);
}


#if defined(OPENSSL_THREADS)
// Functions which access properties on the negotiated session are thread-safe
// where needed. Prior to TLS 1.3, clients resuming sessions and servers
// performing stateful resumption will share an underlying SSL_SESSION object,
// potentially across threads.
TEST_P(SSLVersionTest, SessionPropertiesThreads) {
  if (version() == TLS1_3_VERSION) {
    // Our TLS 1.3 implementation does not support stateful resumption.
    ASSERT_FALSE(CreateClientSession(client_ctx_.get(), server_ctx_.get()));
    return;
  }

  SSL_CTX_set_options(server_ctx_.get(), SSL_OP_NO_TICKET);
  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);

  ASSERT_TRUE(UseCertAndKey(client_ctx_.get()));
  ASSERT_TRUE(UseCertAndKey(server_ctx_.get()));

  // Configure mutual authentication, so we have more session state.
  SSL_CTX_set_custom_verify(
      client_ctx_.get(), SSL_VERIFY_PEER,
      [](SSL *ssl, uint8_t *out_alert) { return ssl_verify_ok; });
  SSL_CTX_set_custom_verify(
      server_ctx_.get(), SSL_VERIFY_PEER,
      [](SSL *ssl, uint8_t *out_alert) { return ssl_verify_ok; });

  // Establish a client session to test with.
  bssl::UniquePtr<SSL_SESSION> session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(session);

  // Resume with it twice.
  UniquePtr<SSL> ssls[4];
  ClientConfig config;
  config.session = session.get();
  ASSERT_TRUE(ConnectClientAndServer(&ssls[0], &ssls[1], client_ctx_.get(),
                                     server_ctx_.get(), config));
  ASSERT_TRUE(ConnectClientAndServer(&ssls[2], &ssls[3], client_ctx_.get(),
                                     server_ctx_.get(), config));

  // Read properties in parallel.
  auto read_properties = [](const SSL *ssl) {
    EXPECT_TRUE(SSL_get_peer_cert_chain(ssl));
    bssl::UniquePtr<X509> peer(SSL_get_peer_certificate(ssl));
    EXPECT_TRUE(peer);
    STACK_OF(X509) *verified_chain = SSL_get0_verified_chain(ssl);
    // This test sets a custom verifier callback which doesn't actually do any
    // verification
    EXPECT_FALSE(verified_chain);
    EXPECT_TRUE(SSL_get_current_cipher(ssl));
    EXPECT_TRUE(SSL_get_group_id(ssl));
  };

  std::vector<std::thread> threads;
  for (const auto &ssl_ptr : ssls) {
    const SSL *ssl = ssl_ptr.get();
    threads.emplace_back([=] { read_properties(ssl); });
  }
  for (auto &thread : threads) {
    thread.join();
  }
  // Session has been resumed twice.
  EXPECT_EQ(SSL_CTX_sess_hits(server_ctx_.get()), 2);
  EXPECT_EQ(SSL_CTX_sess_hits(client_ctx_.get()), 2);
}
#endif  // OPENSSL_THREADS


TEST_P(SSLVersionTest, SimpleVerifiedChain) {
  ASSERT_TRUE(UseCertAndKey(server_ctx_.get()));
  ASSERT_TRUE(X509_STORE_add_cert(SSL_CTX_get_cert_store(client_ctx_.get()),
                                  cert_.get()));
  SSL_CTX_set_verify(client_ctx_.get(),
                     SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT,
                     nullptr);
  X509_VERIFY_PARAM_set_flags(SSL_CTX_get0_param(client_ctx_.get()),
                              X509_V_FLAG_NO_CHECK_TIME);


  UniquePtr<SSL> client_ssl, server_ssl;
  ClientConfig config;
  ASSERT_TRUE(ConnectClientAndServer(
      &client_ssl, &server_ssl, client_ctx_.get(), server_ctx_.get(), config));

  STACK_OF(X509) *client_chain = SSL_get_peer_full_cert_chain(client_ssl.get());
  STACK_OF(X509) *verified_client_chain =
      SSL_get0_verified_chain(client_ssl.get());
  EXPECT_TRUE(verified_client_chain);

  STACK_OF(X509) *verified_server_chain =
      SSL_get0_verified_chain(server_ssl.get());
  // The client didn't send a certificate so the server shouldn't have anything
  EXPECT_FALSE(verified_server_chain);

  // UseCertAndKey sets a single cert that is directly trusted, it is the only
  // one sent, and only one needed for verification
  EXPECT_EQ(sk_X509_num(client_chain), 1UL);
  EXPECT_EQ(X509_cmp(sk_X509_value(client_chain, 0), cert_.get()), 0);

  EXPECT_EQ(sk_X509_num(verified_client_chain), 1UL);
  EXPECT_EQ(X509_cmp(sk_X509_value(verified_client_chain, 0), cert_.get()), 0);
}

TEST_P(SSLVersionTest, VerifiedChain) {
  ASSERT_TRUE(X509_STORE_add_cert(SSL_CTX_get_cert_store(client_ctx_.get()),
                                  cert_.get()));
  SSL_CTX_set_verify(client_ctx_.get(),
                     SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT,
                     nullptr);
  X509_VERIFY_PARAM_set_flags(SSL_CTX_get0_param(client_ctx_.get()),
                              X509_V_FLAG_NO_CHECK_TIME);

  // UseCertAndKey sets the leaf cert the server will use and ensures the client
  // trusts the server's cert
  ASSERT_TRUE(UseCertAndKey(server_ctx_.get()));

  // Add two extra certs to the chain
  bssl::UniquePtr<STACK_OF(X509)> chain(sk_X509_new_null());
  bssl::UniquePtr<X509> cert1 = GetECDSATestCertificate();
  ASSERT_TRUE(sk_X509_push(chain.get(), cert1.get()));
  X509_up_ref(cert1.get());

  bssl::UniquePtr<X509> cert2 = GetTestCertificate();
  ASSERT_TRUE(sk_X509_push(chain.get(), cert2.get()));
  X509_up_ref(cert2.get());

  SSL_CTX_set1_chain(server_ctx_.get(), chain.get());

  UniquePtr<SSL> client_ssl, server_ssl;
  ClientConfig config;
  ASSERT_TRUE(ConnectClientAndServer(
      &client_ssl, &server_ssl, client_ctx_.get(), server_ctx_.get(), config));

  // The client didn't send a certificate so the server shouldn't have anything
  STACK_OF(X509) *verified_client_chain =
      SSL_get0_verified_chain(server_ssl.get());
  EXPECT_FALSE(verified_client_chain);
  STACK_OF(X509) *client_chain = SSL_get_peer_full_cert_chain(server_ssl.get());
  EXPECT_FALSE(client_chain);

  // The server sent a chain that the client can verify, the client directly
  // trusts the server's certificate
  STACK_OF(X509) *verified_server_chain =
      SSL_get0_verified_chain(client_ssl.get());
  EXPECT_EQ(sk_X509_num(verified_server_chain), 1UL);
  EXPECT_EQ(X509_cmp(sk_X509_value(verified_server_chain, 0), cert_.get()), 0);

  // The server sent two extra certs that are unneeded for verification,
  // but it is included in the unverified chain
  STACK_OF(X509) *server_chain = SSL_get_peer_full_cert_chain(client_ssl.get());
  EXPECT_EQ(sk_X509_num(server_chain), 3UL);
  EXPECT_EQ(X509_cmp(sk_X509_value(server_chain, 0), cert_.get()), 0);
  EXPECT_EQ(X509_cmp(sk_X509_value(server_chain, 1), cert1.get()), 0);
  EXPECT_EQ(X509_cmp(sk_X509_value(server_chain, 2), cert2.get()), 0);
}

TEST_P(SSLVersionTest, FailedHandshakeVerifiedChain) {
  SSL_CTX_set_verify(client_ctx_.get(),
                     SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT,
                     nullptr);
  X509_VERIFY_PARAM_set_flags(SSL_CTX_get0_param(client_ctx_.get()),
                              X509_V_FLAG_NO_CHECK_TIME);

  ASSERT_TRUE(UseCertAndKey(server_ctx_.get()));
  UniquePtr<SSL> client_ssl, server_ssl;

  ASSERT_TRUE(CreateClientAndServer(&client_ssl, &server_ssl, client_ctx_.get(),
                                    server_ctx_.get()));
  ASSERT_FALSE(CompleteHandshakes(client_ssl.get(), server_ssl.get()));
  EXPECT_NE(SSL_get_verify_result(client_ssl.get()), X509_V_OK);

  STACK_OF(X509) *client_chain = SSL_get_peer_full_cert_chain(client_ssl.get());
  ASSERT_TRUE(client_chain);
  EXPECT_EQ(sk_X509_num(client_chain), 1UL);
  EXPECT_EQ(X509_cmp(sk_X509_value(client_chain, 0), cert_.get()), 0);


  // For a failed handshake SSL_get0_verified_chain will return null
  STACK_OF(X509) *verified_client_chain =
      SSL_get0_verified_chain(client_ssl.get());
  EXPECT_FALSE(verified_client_chain);
}


TEST_P(SSLVersionTest, SessionMissCache) {
  if (version() == TLS1_3_VERSION) {
    // Our TLS 1.3 implementation does not support stateful resumption.
    ASSERT_FALSE(CreateClientSession(client_ctx_.get(), server_ctx_.get()));
    return;
  }

  SSL_CTX_set_options(server_ctx_.get(), SSL_OP_NO_TICKET);
  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_current_time_cb(server_ctx_.get(), CurrentTimeCallback);

  ClientConfig config;
  bssl::UniquePtr<SSL> client, server;
  // Make some sessions at an arbitrary start time. Then expire them.
  g_current_time.tv_sec = 1000;
  bssl::UniquePtr<SSL_SESSION> expired_session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(expired_session);
  g_current_time.tv_sec += 100 * SSL_DEFAULT_SESSION_TIMEOUT;

  static const int kNumConnections = 2;
  config.session = expired_session.get();
  EXPECT_TRUE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                     server_ctx_.get(), config));
  EXPECT_TRUE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                     server_ctx_.get(), config));

  // First connection is not an |sess_miss|, but failed to connect successfully.
  // Subsequent connections will all be both timeouts and misses.
  EXPECT_EQ(SSL_CTX_sess_misses(server_ctx_.get()), kNumConnections - 1);
  EXPECT_EQ(SSL_CTX_sess_timeouts(server_ctx_.get()), kNumConnections);
  // Check that |sess_hits| is not incorrectly incremented on either end.
  EXPECT_EQ(SSL_CTX_sess_hits(client_ctx_.get()), 0);
  EXPECT_EQ(SSL_CTX_sess_hits(server_ctx_.get()), 0);
}

// Callback function to force an external session cache counter update.
// This intentionally always returns a value, to verify that the counter is
// updated as intended.
// Allocating any memory within the callback function will cause the address
// sanitizers to fail, so we manage the memory externally.
static bssl::UniquePtr<SSL_SESSION> ssl_session;
static SSL_SESSION *get_session(SSL *ssl, const unsigned char *id, int idlen,
                                int *do_copy) {
  return ssl_session.release();
}

TEST_P(SSLVersionTest, SessionExternalCacheHit) {
  if (version() == TLS1_3_VERSION) {
    // Our TLS 1.3 implementation does not support stateful resumption.
    ASSERT_FALSE(CreateClientSession(client_ctx_.get(), server_ctx_.get()));
    return;
  }

  SSL_CTX_set_options(server_ctx_.get(), SSL_OP_NO_TICKET);
  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(
      server_ctx_.get(), SSL_SESS_CACHE_BOTH | SSL_SESS_CACHE_NO_INTERNAL);
  SSL_CTX_sess_set_get_cb(server_ctx_.get(), get_session);

  ClientConfig config;
  bssl::UniquePtr<SSL> client, server;
  bssl::UniquePtr<SSL_SESSION> session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());

  static const int kNumConnections = 2;
  config.session = session.get();
  for (int i = 0; i < kNumConnections; i++) {
    ssl_session.reset(session.get());
    EXPECT_TRUE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                       server_ctx_.get(), config));
  }
  EXPECT_EQ(SSL_CTX_sess_cb_hits(server_ctx_.get()), kNumConnections);
}


TEST_P(SSLVersionTest, DoubleSSLError) {
  // Connect the inner SSL connections.
  ASSERT_TRUE(Connect());

  // Make a pair of |BIO|s which wrap |client_| and |server_|.
  UniquePtr<BIO_METHOD> bio_method(BIO_meth_new(0, nullptr));
  ASSERT_TRUE(bio_method);
  ASSERT_TRUE(BIO_meth_set_read(
      bio_method.get(), [](BIO *bio, char *out, int len) -> int {
        SSL *ssl = static_cast<SSL *>(BIO_get_data(bio));
        int ret = SSL_read(ssl, out, len);
        int ssl_ret = SSL_get_error(ssl, ret);
        if (ssl_ret == SSL_ERROR_WANT_READ) {
          BIO_set_retry_read(bio);
        }
        return ret;
      }));
  ASSERT_TRUE(BIO_meth_set_write(
      bio_method.get(), [](BIO *bio, const char *in, int len) -> int {
        SSL *ssl = static_cast<SSL *>(BIO_get_data(bio));
        int ret = SSL_write(ssl, in, len);
        int ssl_ret = SSL_get_error(ssl, ret);
        if (ssl_ret == SSL_ERROR_WANT_WRITE) {
          BIO_set_retry_write(bio);
        }
        return ret;
      }));
  ASSERT_TRUE(BIO_meth_set_ctrl(
      bio_method.get(), [](BIO *bio, int cmd, long larg, void *parg) -> long {
        // |SSL| objects require |BIO_flush| support.
        if (cmd == BIO_CTRL_FLUSH) {
          return 1;
        }
        return 0;
      }));

  UniquePtr<BIO> client_bio(BIO_new(bio_method.get()));
  ASSERT_TRUE(client_bio);
  BIO_set_data(client_bio.get(), client_.get());
  BIO_set_init(client_bio.get(), 1);

  UniquePtr<BIO> server_bio(BIO_new(bio_method.get()));
  ASSERT_TRUE(server_bio);
  BIO_set_data(server_bio.get(), server_.get());
  BIO_set_init(server_bio.get(), 1);

  // Wrap the inner connections in another layer of SSL.
  UniquePtr<SSL> client_outer(SSL_new(client_ctx_.get()));
  ASSERT_TRUE(client_outer);
  SSL_set_connect_state(client_outer.get());
  SSL_set_bio(client_outer.get(), client_bio.get(), client_bio.get());
  client_bio.release();  // |SSL_set_bio| takes ownership.

  UniquePtr<SSL> server_outer(SSL_new(server_ctx_.get()));
  ASSERT_TRUE(server_outer);
  SSL_set_accept_state(server_outer.get());
  SSL_set_bio(server_outer.get(), server_bio.get(), server_bio.get());
  server_bio.release();  // |SSL_set_bio| takes ownership.

  // Configure |client_outer| to reject the server certificate.
  SSL_set_custom_verify(
      client_outer.get(), SSL_VERIFY_PEER,
      [](SSL *ssl, uint8_t *out_alert) -> ssl_verify_result_t {
        return ssl_verify_invalid;
      });

  for (;;) {
    int client_ret = SSL_do_handshake(client_outer.get());
    int client_err = SSL_get_error(client_outer.get(), client_ret);
    if (client_err != SSL_ERROR_WANT_READ &&
        client_err != SSL_ERROR_WANT_WRITE) {
      // The client handshake should terminate on a certificate verification
      // error.
      EXPECT_EQ(SSL_ERROR_SSL, client_err);
      EXPECT_TRUE(ErrorEquals(ERR_peek_error(), ERR_LIB_SSL,
                              SSL_R_CERTIFICATE_VERIFY_FAILED));
      break;
    }

    // Run the server handshake and continue.
    int server_ret = SSL_do_handshake(server_outer.get());
    int server_err = SSL_get_error(server_outer.get(), server_ret);
    ASSERT_TRUE(server_err == SSL_ERROR_NONE ||
                server_err == SSL_ERROR_WANT_READ ||
                server_err == SSL_ERROR_WANT_WRITE);
  }
}

TEST_P(SSLVersionTest, SameKeyResume) {
  uint8_t key[48];
  RAND_bytes(key, sizeof(key));

  bssl::UniquePtr<SSL_CTX> server_ctx2 = CreateContext();
  ASSERT_TRUE(server_ctx2);
  ASSERT_TRUE(UseCertAndKey(server_ctx2.get()));
  ASSERT_TRUE(
      SSL_CTX_set_tlsext_ticket_keys(server_ctx_.get(), key, sizeof(key)));
  ASSERT_TRUE(
      SSL_CTX_set_tlsext_ticket_keys(server_ctx2.get(), key, sizeof(key)));

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx2.get(), SSL_SESS_CACHE_BOTH);

  // Establish a session for |server_ctx_|.
  bssl::UniquePtr<SSL_SESSION> session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(session);
  ClientConfig config;
  config.session = session.get();

  // No hits before we resume the connection
  EXPECT_EQ(SSL_CTX_sess_hits(client_ctx_.get()), 0);
  EXPECT_EQ(SSL_CTX_sess_hits(server_ctx_.get()), 0);
  EXPECT_EQ(SSL_CTX_sess_hits(server_ctx2.get()), 0);

  // Resuming with |server_ctx_| again works.
  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                     server_ctx_.get(), config));
  EXPECT_TRUE(SSL_session_reused(client.get()));
  EXPECT_TRUE(SSL_session_reused(server.get()));

  // Resuming with |server_ctx2| also works.
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                     server_ctx2.get(), config));
  EXPECT_TRUE(SSL_session_reused(client.get()));
  EXPECT_TRUE(SSL_session_reused(server.get()));

  // By this point, the session has been resumed twice on the client side and
  // once for each server context.
  EXPECT_EQ(SSL_CTX_sess_hits(client_ctx_.get()), 2);
  EXPECT_EQ(SSL_CTX_sess_hits(server_ctx_.get()), 1);
  EXPECT_EQ(SSL_CTX_sess_hits(server_ctx2.get()), 1);
}

TEST_P(SSLVersionTest, DifferentKeyNoResume) {
  uint8_t key1[48], key2[48];
  RAND_bytes(key1, sizeof(key1));
  RAND_bytes(key2, sizeof(key2));

  bssl::UniquePtr<SSL_CTX> server_ctx2 = CreateContext();
  ASSERT_TRUE(server_ctx2);
  ASSERT_TRUE(UseCertAndKey(server_ctx2.get()));
  ASSERT_TRUE(
      SSL_CTX_set_tlsext_ticket_keys(server_ctx_.get(), key1, sizeof(key1)));
  ASSERT_TRUE(
      SSL_CTX_set_tlsext_ticket_keys(server_ctx2.get(), key2, sizeof(key2)));

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx2.get(), SSL_SESS_CACHE_BOTH);

  // Establish a session for |server_ctx_|.
  bssl::UniquePtr<SSL_SESSION> session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(session);
  ClientConfig config;
  config.session = session.get();

  // Resuming with |server_ctx_| again works.
  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                     server_ctx_.get(), config));
  EXPECT_TRUE(SSL_session_reused(client.get()));
  EXPECT_TRUE(SSL_session_reused(server.get()));

  // Resuming with |server_ctx2| does not work.
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                     server_ctx2.get(), config));
  EXPECT_FALSE(SSL_session_reused(client.get()));
  EXPECT_FALSE(SSL_session_reused(server.get()));
}

TEST_P(SSLVersionTest, UnrelatedServerNoResume) {
  bssl::UniquePtr<SSL_CTX> server_ctx2 = CreateContext();
  ASSERT_TRUE(server_ctx2);
  ASSERT_TRUE(UseCertAndKey(server_ctx2.get()));

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx2.get(), SSL_SESS_CACHE_BOTH);

  // Establish a session for |server_ctx_|.
  bssl::UniquePtr<SSL_SESSION> session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());
  ASSERT_TRUE(session);
  ClientConfig config;
  config.session = session.get();

  // Resuming with |server_ctx_| again works.
  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                     server_ctx_.get(), config));
  EXPECT_TRUE(SSL_session_reused(client.get()));
  EXPECT_TRUE(SSL_session_reused(server.get()));

  // Resuming with |server_ctx2| does not work.
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                     server_ctx2.get(), config));
  EXPECT_FALSE(SSL_session_reused(client.get()));
  EXPECT_FALSE(SSL_session_reused(server.get()));
}

static Span<const uint8_t> SessionIDOf(const SSL *ssl) {
  const SSL_SESSION *session = SSL_get_session(ssl);
  unsigned len = 0;
  const uint8_t *data = SSL_SESSION_get_id(session, &len);
  return MakeConstSpan(data, len);
}

TEST_P(SSLVersionTest, TicketSessionIDsMatch) {
  // This checks that the session IDs at client and server match after a ticket
  // resumption. It's unclear whether this should be true, but Envoy depends
  // on it in their tests so this will give an early signal if we break it.
  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx_.get(), SSL_SESS_CACHE_BOTH);

  bssl::UniquePtr<SSL_SESSION> session =
      CreateClientSession(client_ctx_.get(), server_ctx_.get());

  bssl::UniquePtr<SSL> client, server;
  ClientConfig config;
  config.session = session.get();
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx_.get(),
                                     server_ctx_.get(), config));
  EXPECT_TRUE(SSL_session_reused(client.get()));
  EXPECT_TRUE(SSL_session_reused(server.get()));

  EXPECT_EQ(Bytes(SessionIDOf(client.get())), Bytes(SessionIDOf(server.get())));
}

TEST_P(SSLVersionTest, PeerTmpKey) {
  if (getVersionParam().transfer_ssl) {
    // The peer's temporary key is not within the boundary of the SSL transfer
    // feature.
    GTEST_SKIP();
  }

  ASSERT_TRUE(Connect());
  for (SSL *ssl : {client_.get(), server_.get()}) {
    SCOPED_TRACE(SSL_is_server(ssl) ? "server" : "client");
    EVP_PKEY *key = nullptr;
    if (getVersionParam().version == TLS1_3_VERSION) {
      // TLS 1.3 default should be using X25519MLKEM768 as the key exchange.
      // We expect SSL_R_UNKNOWN_KEY_EXCHANGE_TYPE because there is no EVP_PKEY type
      // for hybrid keys, only individual X25519 or MLKEM768 keys.
      ERR_clear_error();
      EXPECT_FALSE(SSL_get_peer_tmp_key(ssl, &key));
      ErrorEquals(ERR_get_error(), ERR_LIB_SSL, SSL_R_UNKNOWN_KEY_EXCHANGE_TYPE);
    } else {
      EXPECT_TRUE(SSL_get_peer_tmp_key(ssl, &key));
      EXPECT_EQ(EVP_PKEY_id(key), EVP_PKEY_X25519);
      bssl::UniquePtr<EVP_PKEY> pkey(key);
    }
  }

  // Check that x25519 works.
  ASSERT_TRUE(SSL_CTX_set1_groups_list(server_ctx_.get(), "x25519"));
  ASSERT_TRUE(Connect());
  for (SSL *ssl : {client_.get(), server_.get()}) {
    SCOPED_TRACE(SSL_is_server(ssl) ? "server" : "client");
    EVP_PKEY *key = nullptr;
    EXPECT_TRUE(SSL_get_peer_tmp_key(ssl, &key));
    EXPECT_EQ(EVP_PKEY_id(key), EVP_PKEY_X25519);
    bssl::UniquePtr<EVP_PKEY> pkey(key);
  }

  // Check that EC Groups for the key exchange also work.
  ASSERT_TRUE(SSL_CTX_set1_groups_list(server_ctx_.get(), "P-384"));
  ASSERT_TRUE(Connect());
  for (SSL *ssl : {client_.get(), server_.get()}) {
    SCOPED_TRACE(SSL_is_server(ssl) ? "server" : "client");
    EVP_PKEY *key = nullptr;
    EXPECT_TRUE(SSL_get_peer_tmp_key(ssl, &key));
    EXPECT_EQ(EVP_PKEY_id(key), EVP_PKEY_EC);
    EC_KEY *ec_key = EVP_PKEY_get0_EC_KEY(key);
    EXPECT_TRUE(ec_key);
    EXPECT_EQ(EC_KEY_get0_group(ec_key), EC_group_p384());
    bssl::UniquePtr<EVP_PKEY> pkey(key);
  }
}

TEST_P(SSLVersionTest, GetFinished) {
  // Test that contents of |finished| and the peer's |finished| align.
  ASSERT_TRUE(Connect());
  for (SSL *ssl : {client_.get(), server_.get()}) {
    SCOPED_TRACE(SSL_is_server(ssl) ? "server" : "client");
    size_t finished_size = SSL_get_finished(ssl, nullptr, 0);
    EXPECT_TRUE(finished_size);
    bssl::UniquePtr<uint8_t> finished((uint8_t *)OPENSSL_malloc(finished_size));
    ASSERT_TRUE(finished);
    EXPECT_TRUE(SSL_get_finished(ssl, finished.get(), finished_size));

    size_t peer_finished_size = SSL_get_peer_finished(ssl, nullptr, 0);
    EXPECT_TRUE(peer_finished_size);
    bssl::UniquePtr<uint8_t> peer_finished(
        (uint8_t *)OPENSSL_malloc(peer_finished_size));
    ASSERT_TRUE(peer_finished);
    EXPECT_TRUE(SSL_get_finished(ssl, peer_finished.get(), peer_finished_size));

    EXPECT_EQ(Bytes(finished.get(), finished_size),
              Bytes(peer_finished.get(), peer_finished_size));
  }
}

// Test the specific scenario that was failing: buffer
// serialization/deserialization with sizes larger than uint16_t max (65535)
TEST(SSLBufferSizeFailureTest, SerDeLargeBuffer) {
  // Create a buffer with capacity larger than the old uint16_t limit of 65535
  SSLBuffer buffer;
  const size_t large_capacity = 70000;

  // Test that we can allocate a buffer larger than 65535 bytes
  EXPECT_TRUE(buffer.EnsureCap(0, large_capacity));

  // Fill the buffer with test data
  std::vector<uint8_t> test_data(large_capacity);
  for (size_t i = 0; i < large_capacity; i++) {
    test_data[i] = static_cast<uint8_t>(i % 256);
  }

  // Write data to the buffer using the correct API
  OPENSSL_memcpy(buffer.data(), test_data.data(), test_data.size());
  buffer.DidWrite(test_data.size());
  EXPECT_EQ(buffer.size(), large_capacity);

  // Test serialization (this would have failed before the fix due to uint16_t
  // overflow)
  bssl::ScopedCBB cbb;
  EXPECT_TRUE(CBB_init(cbb.get(), 0));
  EXPECT_TRUE(buffer.DoSerialization(*cbb.get()));

  uint8_t *serialized_data = nullptr;
  size_t serialized_len = 0;
  EXPECT_TRUE(CBB_finish(cbb.get(), &serialized_data, &serialized_len));
  bssl::UniquePtr<uint8_t> serialized_ptr(serialized_data);
  EXPECT_GT(serialized_len, 0u);

  // Test deserialization (this would have failed before the fix)
  SSLBuffer deserialized_buffer;
  CBS cbs;
  CBS_init(&cbs, serialized_data, serialized_len);
  EXPECT_TRUE(deserialized_buffer.DoDeserialization(cbs));

  // Verify the deserialized buffer has the correct size and capacity
  EXPECT_EQ(deserialized_buffer.size(), large_capacity);
  EXPECT_GE(deserialized_buffer.cap(), large_capacity);

  // Verify the data integrity
  EXPECT_EQ(0, OPENSSL_memcmp(deserialized_buffer.data(), test_data.data(),
                              large_capacity));
}

// Test that specifically targets the internal buffer allocation logic
TEST(SSLVersionTest, InternalBufferAllocationLimits) {
  // This test directly exercises the SSLBuffer class to ensure it properly
  // handles large buffer size requests
  SSLBuffer buffer;

  // Test various sizes around the boundary
  EXPECT_TRUE(buffer.EnsureCap(5, 65535));  // Old limit
  buffer.Clear();

  EXPECT_TRUE(buffer.EnsureCap(5, 65536));  // Just above old limit
  buffer.Clear();

  // These should fail - beyond the maximum capacity
  EXPECT_FALSE(buffer.EnsureCap(5, UINT32_MAX));
  EXPECT_FALSE(buffer.EnsureCap(5, SIZE_MAX));
}

BSSL_NAMESPACE_END
