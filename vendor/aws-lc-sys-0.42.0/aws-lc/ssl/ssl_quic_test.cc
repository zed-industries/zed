// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/ssl.h>
#include "../crypto/test/test_util.h"
#include "ssl_common_test.h"

BSSL_NAMESPACE_BEGIN

template <typename T>
class UnownedSSLExData {
 public:
  UnownedSSLExData() {
    index_ = SSL_get_ex_new_index(0, nullptr, nullptr, nullptr, nullptr);
  }

  T *Get(const SSL *ssl) {
    return index_ < 0 ? nullptr
                      : static_cast<T *>(SSL_get_ex_data(ssl, index_));
  }

  bool Set(SSL *ssl, T *t) {
    return index_ >= 0 && SSL_set_ex_data(ssl, index_, t);
  }

 private:
  int index_;
};

constexpr size_t kNumQUICLevels = 4;
static_assert(ssl_encryption_initial < kNumQUICLevels,
              "kNumQUICLevels is wrong");
static_assert(ssl_encryption_early_data < kNumQUICLevels,
              "kNumQUICLevels is wrong");
static_assert(ssl_encryption_handshake < kNumQUICLevels,
              "kNumQUICLevels is wrong");
static_assert(ssl_encryption_application < kNumQUICLevels,
              "kNumQUICLevels is wrong");

static const char *LevelToString(ssl_encryption_level_t level) {
  switch (level) {
    case ssl_encryption_initial:
      return "initial";
    case ssl_encryption_early_data:
      return "early data";
    case ssl_encryption_handshake:
      return "handshake";
    case ssl_encryption_application:
      return "application";
  }
  return "<unknown>";
}

class MockQUICTransport {
 public:
  enum class Role { kClient, kServer };

  explicit MockQUICTransport(Role role) : role_(role) {
    // The caller is expected to configure initial secrets.
    levels_[ssl_encryption_initial].write_secret = {1};
    levels_[ssl_encryption_initial].read_secret = {1};
  }

  void set_peer(MockQUICTransport *peer) { peer_ = peer; }

  bool has_alert() const { return has_alert_; }
  ssl_encryption_level_t alert_level() const { return alert_level_; }
  uint8_t alert() const { return alert_; }

  bool PeerSecretsMatch(ssl_encryption_level_t level) const {
    return levels_[level].write_secret == peer_->levels_[level].read_secret &&
           levels_[level].read_secret == peer_->levels_[level].write_secret &&
           levels_[level].cipher == peer_->levels_[level].cipher;
  }

  bool HasReadSecret(ssl_encryption_level_t level) const {
    return !levels_[level].read_secret.empty();
  }

  bool HasWriteSecret(ssl_encryption_level_t level) const {
    return !levels_[level].write_secret.empty();
  }

  void AllowOutOfOrderWrites() { allow_out_of_order_writes_ = true; }

  bool SetReadSecret(ssl_encryption_level_t level, const SSL_CIPHER *cipher,
                     Span<const uint8_t> secret) {
    if (HasReadSecret(level)) {
      ADD_FAILURE() << LevelToString(level) << " read secret configured twice";
      return false;
    }

    if (role_ == Role::kClient && level == ssl_encryption_early_data) {
      ADD_FAILURE() << "Unexpected early data read secret";
      return false;
    }

    ssl_encryption_level_t ack_level =
        level == ssl_encryption_early_data ? ssl_encryption_application : level;
    if (!HasWriteSecret(ack_level)) {
      ADD_FAILURE() << LevelToString(level)
                    << " read secret configured before ACK write secret";
      return false;
    }

    if (cipher == nullptr) {
      ADD_FAILURE() << "Unexpected null cipher";
      return false;
    }

    if (level != ssl_encryption_early_data &&
        SSL_CIPHER_get_id(cipher) != levels_[level].cipher) {
      ADD_FAILURE() << "Cipher suite inconsistent";
      return false;
    }

    levels_[level].read_secret.assign(secret.begin(), secret.end());
    levels_[level].cipher = SSL_CIPHER_get_id(cipher);
    return true;
  }

  bool SetWriteSecret(ssl_encryption_level_t level, const SSL_CIPHER *cipher,
                      Span<const uint8_t> secret) {
    if (HasWriteSecret(level)) {
      ADD_FAILURE() << LevelToString(level) << " write secret configured twice";
      return false;
    }

    if (role_ == Role::kServer && level == ssl_encryption_early_data) {
      ADD_FAILURE() << "Unexpected early data write secret";
      return false;
    }

    if (cipher == nullptr) {
      ADD_FAILURE() << "Unexpected null cipher";
      return false;
    }

    levels_[level].write_secret.assign(secret.begin(), secret.end());
    levels_[level].cipher = SSL_CIPHER_get_id(cipher);
    return true;
  }

  bool WriteHandshakeData(ssl_encryption_level_t level,
                          Span<const uint8_t> data) {
    if (levels_[level].write_secret.empty()) {
      ADD_FAILURE() << LevelToString(level)
                    << " write secret not yet configured";
      return false;
    }

    // Although the levels are conceptually separate, BoringSSL finishes writing
    // data from a previous level before installing keys for the next level.
    if (!allow_out_of_order_writes_) {
      switch (level) {
        case ssl_encryption_early_data:
          ADD_FAILURE() << "unexpected handshake data at early data level";
          return false;
        case ssl_encryption_initial:
          if (!levels_[ssl_encryption_handshake].write_secret.empty()) {
            ADD_FAILURE()
                << LevelToString(level)
                << " handshake data written after handshake keys installed";
            return false;
          }
          OPENSSL_FALLTHROUGH;
        case ssl_encryption_handshake:
          if (!levels_[ssl_encryption_application].write_secret.empty()) {
            ADD_FAILURE()
                << LevelToString(level)
                << " handshake data written after application keys installed";
            return false;
          }
          OPENSSL_FALLTHROUGH;
        case ssl_encryption_application:
          break;
      }
    }

    levels_[level].write_data.insert(levels_[level].write_data.end(),
                                     data.begin(), data.end());
    return true;
  }

  bool SendAlert(ssl_encryption_level_t level, uint8_t alert_value) {
    if (has_alert_) {
      ADD_FAILURE() << "duplicate alert sent";
      return false;
    }

    if (levels_[level].write_secret.empty()) {
      ADD_FAILURE() << LevelToString(level)
                    << " write secret not yet configured";
      return false;
    }

    has_alert_ = true;
    alert_level_ = level;
    alert_ = alert_value;
    return true;
  }

  bool ReadHandshakeData(std::vector<uint8_t> *out,
                         ssl_encryption_level_t level,
                         size_t num = std::numeric_limits<size_t>::max()) {
    if (levels_[level].read_secret.empty()) {
      ADD_FAILURE() << "data read before keys configured in level " << level;
      return false;
    }
    // The peer may not have configured any keys yet.
    if (peer_->levels_[level].write_secret.empty()) {
      out->clear();
      return true;
    }
    // Check the peer computed the same key.
    if (peer_->levels_[level].write_secret != levels_[level].read_secret) {
      ADD_FAILURE() << "peer write key does not match read key in level "
                    << level;
      return false;
    }
    if (peer_->levels_[level].cipher != levels_[level].cipher) {
      ADD_FAILURE() << "peer cipher does not match in level " << level;
      return false;
    }
    std::vector<uint8_t> *peer_data = &peer_->levels_[level].write_data;
    num = std::min(num, peer_data->size());
    out->assign(peer_data->begin(), peer_data->begin() + num);
    peer_data->erase(peer_data->begin(), peer_data->begin() + num);
    return true;
  }

 private:
  Role role_;
  MockQUICTransport *peer_ = nullptr;

  bool allow_out_of_order_writes_ = false;
  bool has_alert_ = false;
  ssl_encryption_level_t alert_level_ = ssl_encryption_initial;
  uint8_t alert_ = 0;

  struct Level {
    std::vector<uint8_t> write_data;
    std::vector<uint8_t> write_secret;
    std::vector<uint8_t> read_secret;
    uint32_t cipher = 0;
  };
  Level levels_[kNumQUICLevels];
};

class MockQUICTransportPair {
 public:
  MockQUICTransportPair()
      : client_(MockQUICTransport::Role::kClient),
        server_(MockQUICTransport::Role::kServer) {
    client_.set_peer(&server_);
    server_.set_peer(&client_);
  }

  ~MockQUICTransportPair() {
    client_.set_peer(nullptr);
    server_.set_peer(nullptr);
  }

  MockQUICTransport *client() { return &client_; }
  MockQUICTransport *server() { return &server_; }

  bool SecretsMatch(ssl_encryption_level_t level) const {
    // We only need to check |HasReadSecret| and |HasWriteSecret| on |client_|.
    // |PeerSecretsMatch| checks that |server_| is analogously configured.
    return client_.PeerSecretsMatch(level) && client_.HasWriteSecret(level) &&
           (level == ssl_encryption_early_data || client_.HasReadSecret(level));
  }

 private:
  MockQUICTransport client_;
  MockQUICTransport server_;
};

class QUICMethodTest : public testing::Test {
 protected:
  void SetUp() override {
    client_ctx_.reset(SSL_CTX_new(TLS_method()));
    server_ctx_ = CreateContextWithTestCertificate(TLS_method());
    ASSERT_TRUE(client_ctx_);
    ASSERT_TRUE(server_ctx_);

    SSL_CTX_set_min_proto_version(server_ctx_.get(), TLS1_3_VERSION);
    SSL_CTX_set_max_proto_version(server_ctx_.get(), TLS1_3_VERSION);
    SSL_CTX_set_min_proto_version(client_ctx_.get(), TLS1_3_VERSION);
    SSL_CTX_set_max_proto_version(client_ctx_.get(), TLS1_3_VERSION);

    static const uint8_t kALPNProtos[] = {0x03, 'f', 'o', 'o'};
    ASSERT_EQ(SSL_CTX_set_alpn_protos(client_ctx_.get(), kALPNProtos,
                                      sizeof(kALPNProtos)),
              0);
    SSL_CTX_set_alpn_select_cb(
        server_ctx_.get(),
        [](SSL *ssl, const uint8_t **out, uint8_t *out_len, const uint8_t *in,
           unsigned in_len, void *arg) -> int {
          return SSL_select_next_proto(
                     const_cast<uint8_t **>(out), out_len, in, in_len,
                     kALPNProtos, sizeof(kALPNProtos)) == OPENSSL_NPN_NEGOTIATED
                     ? SSL_TLSEXT_ERR_OK
                     : SSL_TLSEXT_ERR_NOACK;
        },
        nullptr);
  }

  static MockQUICTransport *TransportFromSSL(const SSL *ssl) {
    return ex_data_.Get(ssl);
  }

  static bool ProvideHandshakeData(
      SSL *ssl, size_t num = std::numeric_limits<size_t>::max()) {
    MockQUICTransport *transport = TransportFromSSL(ssl);
    ssl_encryption_level_t level = SSL_quic_read_level(ssl);
    std::vector<uint8_t> data;
    return transport->ReadHandshakeData(&data, level, num) &&
           SSL_provide_quic_data(ssl, level, data.data(), data.size());
  }

  void AllowOutOfOrderWrites() { allow_out_of_order_writes_ = true; }

  bool CreateClientAndServer() {
    client_.reset(SSL_new(client_ctx_.get()));
    server_.reset(SSL_new(server_ctx_.get()));
    if (!client_ || !server_) {
      return false;
    }

    SSL_set_connect_state(client_.get());
    SSL_set_accept_state(server_.get());

    transport_.reset(new MockQUICTransportPair);
    if (!ex_data_.Set(client_.get(), transport_->client()) ||
        !ex_data_.Set(server_.get(), transport_->server())) {
      return false;
    }
    if (allow_out_of_order_writes_) {
      transport_->client()->AllowOutOfOrderWrites();
      transport_->server()->AllowOutOfOrderWrites();
    }
    static const uint8_t client_transport_params[] = {0};
    if (!SSL_set_quic_transport_params(client_.get(), client_transport_params,
                                       sizeof(client_transport_params)) ||
        !SSL_set_quic_transport_params(server_.get(),
                                       server_transport_params_.data(),
                                       server_transport_params_.size()) ||
        !SSL_set_quic_early_data_context(
            server_.get(), server_quic_early_data_context_.data(),
            server_quic_early_data_context_.size())) {
      return false;
    }
    return true;
  }

  enum class ExpectedError {
    kNoError,
    kClientError,
    kServerError,
  };

  // CompleteHandshakesForQUIC runs |SSL_do_handshake| on |client_| and
  // |server_| until each completes once. It returns true on success and false
  // on failure.
  bool CompleteHandshakesForQUIC() {
    return RunQUICHandshakesAndExpectError(ExpectedError::kNoError);
  }

  // Runs |SSL_do_handshake| on |client_| and |server_| until each completes
  // once. If |expect_client_error| is true, it will return true only if the
  // client handshake failed. Otherwise, it returns true if both handshakes
  // succeed and false otherwise.
  bool RunQUICHandshakesAndExpectError(ExpectedError expected_error) {
    bool client_done = false, server_done = false;
    while (!client_done || !server_done) {
      if (!client_done) {
        if (!ProvideHandshakeData(client_.get())) {
          ADD_FAILURE() << "ProvideHandshakeData(client_) failed";
          return false;
        }
        int client_ret = SSL_do_handshake(client_.get());
        int client_err = SSL_get_error(client_.get(), client_ret);
        if (client_ret == 1) {
          client_done = true;
        } else if (client_ret != -1 || client_err != SSL_ERROR_WANT_READ) {
          if (expected_error == ExpectedError::kClientError) {
            return true;
          }
          ADD_FAILURE() << "Unexpected client output: " << client_ret << " "
                        << client_err;
          return false;
        }
      }

      if (!server_done) {
        if (!ProvideHandshakeData(server_.get())) {
          ADD_FAILURE() << "ProvideHandshakeData(server_) failed";
          return false;
        }
        int server_ret = SSL_do_handshake(server_.get());
        int server_err = SSL_get_error(server_.get(), server_ret);
        if (server_ret == 1) {
          server_done = true;
        } else if (server_ret != -1 || server_err != SSL_ERROR_WANT_READ) {
          if (expected_error == ExpectedError::kServerError) {
            return true;
          }
          ADD_FAILURE() << "Unexpected server output: " << server_ret << " "
                        << server_err;
          return false;
        }
      }
    }
    return expected_error == ExpectedError::kNoError;
  }

  bssl::UniquePtr<SSL_SESSION> CreateClientSessionForQUIC() {
    g_last_session = nullptr;
    SSL_CTX_sess_set_new_cb(client_ctx_.get(), SaveLastSession);
    if (!CreateClientAndServer() || !CompleteHandshakesForQUIC()) {
      return nullptr;
    }

    // The server sent NewSessionTicket messages in the handshake.
    if (!ProvideHandshakeData(client_.get()) ||
        !SSL_process_quic_post_handshake(client_.get())) {
      return nullptr;
    }

    return std::move(g_last_session);
  }

  void ExpectHandshakeSuccess() {
    EXPECT_TRUE(transport_->SecretsMatch(ssl_encryption_application));
    EXPECT_EQ(ssl_encryption_application, SSL_quic_read_level(client_.get()));
    EXPECT_EQ(ssl_encryption_application, SSL_quic_write_level(client_.get()));
    EXPECT_EQ(ssl_encryption_application, SSL_quic_read_level(server_.get()));
    EXPECT_EQ(ssl_encryption_application, SSL_quic_write_level(server_.get()));
    EXPECT_FALSE(transport_->client()->has_alert());
    EXPECT_FALSE(transport_->server()->has_alert());

    // SSL_do_handshake is now idempotent.
    EXPECT_EQ(SSL_do_handshake(client_.get()), 1);
    EXPECT_EQ(SSL_do_handshake(server_.get()), 1);
  }

  // Returns a default SSL_QUIC_METHOD. Individual methods may be overwritten by
  // the test.
  SSL_QUIC_METHOD DefaultQUICMethod() {
    return SSL_QUIC_METHOD{
        SetReadSecretCallback, SetWriteSecretCallback, AddHandshakeDataCallback,
        FlushFlightCallback,   SendAlertCallback,
    };
  }

  static int SetReadSecretCallback(SSL *ssl, ssl_encryption_level_t level,
                                   const SSL_CIPHER *cipher,
                                   const uint8_t *secret, size_t secret_len) {
    return TransportFromSSL(ssl)->SetReadSecret(
        level, cipher, MakeConstSpan(secret, secret_len));
  }

  static int SetWriteSecretCallback(SSL *ssl, ssl_encryption_level_t level,
                                    const SSL_CIPHER *cipher,
                                    const uint8_t *secret, size_t secret_len) {
    return TransportFromSSL(ssl)->SetWriteSecret(
        level, cipher, MakeConstSpan(secret, secret_len));
  }

  static int AddHandshakeDataCallback(SSL *ssl,
                                      enum ssl_encryption_level_t level,
                                      const uint8_t *data, size_t len) {
    EXPECT_EQ(level, SSL_quic_write_level(ssl));
    return TransportFromSSL(ssl)->WriteHandshakeData(level,
                                                     MakeConstSpan(data, len));
  }

  static int FlushFlightCallback(SSL *ssl) { return 1; }

  static int SendAlertCallback(SSL *ssl, ssl_encryption_level_t level,
                               uint8_t alert) {
    EXPECT_EQ(level, SSL_quic_write_level(ssl));
    return TransportFromSSL(ssl)->SendAlert(level, alert);
  }

  bssl::UniquePtr<SSL_CTX> client_ctx_;
  bssl::UniquePtr<SSL_CTX> server_ctx_;

  static UnownedSSLExData<MockQUICTransport> ex_data_;
  std::unique_ptr<MockQUICTransportPair> transport_;

  bssl::UniquePtr<SSL> client_;
  bssl::UniquePtr<SSL> server_;

  std::vector<uint8_t> server_transport_params_ = {1};
  std::vector<uint8_t> server_quic_early_data_context_ = {2};

  bool allow_out_of_order_writes_ = false;
};

UnownedSSLExData<MockQUICTransport> QUICMethodTest::ex_data_;

// Test a full handshake and resumption work.
TEST_F(QUICMethodTest, Basic) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  g_last_session = nullptr;

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_sess_set_new_cb(client_ctx_.get(), SaveLastSession);
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  ASSERT_TRUE(CreateClientAndServer());
  ASSERT_TRUE(CompleteHandshakesForQUIC());

  ExpectHandshakeSuccess();
  EXPECT_FALSE(SSL_session_reused(client_.get()));
  EXPECT_FALSE(SSL_session_reused(server_.get()));

  // The server sent NewSessionTicket messages in the handshake.
  EXPECT_FALSE(g_last_session);
  ASSERT_TRUE(ProvideHandshakeData(client_.get()));
  EXPECT_EQ(SSL_process_quic_post_handshake(client_.get()), 1);
  EXPECT_TRUE(g_last_session);

  // Create a second connection to verify resumption works.
  ASSERT_TRUE(CreateClientAndServer());
  bssl::UniquePtr<SSL_SESSION> session = std::move(g_last_session);
  SSL_set_session(client_.get(), session.get());

  ASSERT_TRUE(CompleteHandshakesForQUIC());

  ExpectHandshakeSuccess();
  EXPECT_TRUE(SSL_session_reused(client_.get()));
  EXPECT_TRUE(SSL_session_reused(server_.get()));
}

// Test that HelloRetryRequest in QUIC works.
TEST_F(QUICMethodTest, HelloRetryRequest) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  // BoringSSL predicts the most preferred ECDH group, so using different
  // preferences will trigger HelloRetryRequest.
  static const int kClientPrefs[] = {NID_X25519, NID_X9_62_prime256v1};
  ASSERT_TRUE(SSL_CTX_set1_groups(client_ctx_.get(), kClientPrefs,
                                  OPENSSL_ARRAY_SIZE(kClientPrefs)));
  static const int kServerPrefs[] = {NID_X9_62_prime256v1, NID_X25519};
  ASSERT_TRUE(SSL_CTX_set1_groups(server_ctx_.get(), kServerPrefs,
                                  OPENSSL_ARRAY_SIZE(kServerPrefs)));

  ASSERT_TRUE(CreateClientAndServer());
  ASSERT_TRUE(CompleteHandshakesForQUIC());
  ExpectHandshakeSuccess();
}

// Test that the client does not send a legacy_session_id in the ClientHello.
TEST_F(QUICMethodTest, NoLegacySessionId) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));
  // Check that the session ID length is 0 in an early callback.
  SSL_CTX_set_select_certificate_cb(
      server_ctx_.get(),
      [](const SSL_CLIENT_HELLO *client_hello) -> ssl_select_cert_result_t {
        EXPECT_EQ(client_hello->session_id_len, 0u);
        return ssl_select_cert_success;
      });

  ASSERT_TRUE(CreateClientAndServer());
  ASSERT_TRUE(CompleteHandshakesForQUIC());

  ExpectHandshakeSuccess();
}

// Test that, even in a 1-RTT handshake, the server installs keys at the right
// time. Half-RTT keys are available early, but 1-RTT read keys are deferred.
TEST_F(QUICMethodTest, HalfRTTKeys) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));
  ASSERT_TRUE(CreateClientAndServer());

  // The client sends ClientHello.
  ASSERT_EQ(SSL_do_handshake(client_.get()), -1);
  ASSERT_EQ(SSL_ERROR_WANT_READ, SSL_get_error(client_.get(), -1));

  // The server reads ClientHello and sends ServerHello..Finished.
  ASSERT_TRUE(ProvideHandshakeData(server_.get()));
  ASSERT_EQ(SSL_do_handshake(server_.get()), -1);
  ASSERT_EQ(SSL_ERROR_WANT_READ, SSL_get_error(server_.get(), -1));

  // At this point, the server has half-RTT write keys, but it cannot access
  // 1-RTT read keys until client Finished.
  EXPECT_TRUE(transport_->server()->HasWriteSecret(ssl_encryption_application));
  EXPECT_FALSE(transport_->server()->HasReadSecret(ssl_encryption_application));

  // Finish up the client and server handshakes.
  ASSERT_TRUE(CompleteHandshakesForQUIC());

  // Both sides can now exchange 1-RTT data.
  ExpectHandshakeSuccess();
}

TEST_F(QUICMethodTest, ZeroRTTAccept) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_early_data_enabled(client_ctx_.get(), 1);
  SSL_CTX_set_early_data_enabled(server_ctx_.get(), 1);
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  bssl::UniquePtr<SSL_SESSION> session = CreateClientSessionForQUIC();
  ASSERT_TRUE(session);

  ASSERT_TRUE(CreateClientAndServer());
  SSL_set_session(client_.get(), session.get());

  EXPECT_FALSE(SSL_get_client_ciphers(client_.get()));
  EXPECT_FALSE(SSL_get_client_ciphers(server_.get()));

  // The client handshake should return immediately into the early data state.
  ASSERT_EQ(SSL_do_handshake(client_.get()), 1);
  EXPECT_TRUE(SSL_in_early_data(client_.get()));
  // The transport should have keys for sending 0-RTT data.
  EXPECT_TRUE(transport_->client()->HasWriteSecret(ssl_encryption_early_data));

  // The server will consume the ClientHello and also enter the early data
  // state.
  ASSERT_TRUE(ProvideHandshakeData(server_.get()));
  ASSERT_EQ(SSL_do_handshake(server_.get()), 1);
  EXPECT_TRUE(SSL_in_early_data(server_.get()));
  EXPECT_TRUE(transport_->SecretsMatch(ssl_encryption_early_data));
  // At this point, the server has half-RTT write keys, but it cannot access
  // 1-RTT read keys until client Finished.
  EXPECT_TRUE(transport_->server()->HasWriteSecret(ssl_encryption_application));
  EXPECT_FALSE(transport_->server()->HasReadSecret(ssl_encryption_application));
  // The client should still have no view of the server's preferences, but the
  // server should have seen at least one cipher from the client.
  EXPECT_FALSE(SSL_get_client_ciphers(client_.get()));
  EXPECT_GT(sk_SSL_CIPHER_num(SSL_get_client_ciphers(server_.get())),
            (size_t)0);

  // Finish up the client and server handshakes.
  ASSERT_TRUE(CompleteHandshakesForQUIC());

  // Both sides can now exchange 1-RTT data.
  ExpectHandshakeSuccess();
  EXPECT_TRUE(SSL_session_reused(client_.get()));
  EXPECT_TRUE(SSL_session_reused(server_.get()));
  EXPECT_FALSE(SSL_in_early_data(client_.get()));
  EXPECT_FALSE(SSL_in_early_data(server_.get()));
  EXPECT_TRUE(SSL_early_data_accepted(client_.get()));
  EXPECT_TRUE(SSL_early_data_accepted(server_.get()));

  // Finish handling post-handshake messages after the first 0-RTT resumption.
  EXPECT_TRUE(ProvideHandshakeData(client_.get()));
  EXPECT_TRUE(SSL_process_quic_post_handshake(client_.get()));

  // Perform a second 0-RTT resumption attempt, and confirm that 0-RTT is
  // accepted again.
  ASSERT_TRUE(CreateClientAndServer());
  SSL_set_session(client_.get(), g_last_session.get());

  // The client handshake should return immediately into the early data state.
  ASSERT_EQ(SSL_do_handshake(client_.get()), 1);
  EXPECT_TRUE(SSL_in_early_data(client_.get()));
  // The transport should have keys for sending 0-RTT data.
  EXPECT_TRUE(transport_->client()->HasWriteSecret(ssl_encryption_early_data));

  // The server will consume the ClientHello and also enter the early data
  // state.
  ASSERT_TRUE(ProvideHandshakeData(server_.get()));
  ASSERT_EQ(SSL_do_handshake(server_.get()), 1);
  EXPECT_TRUE(SSL_in_early_data(server_.get()));
  EXPECT_TRUE(transport_->SecretsMatch(ssl_encryption_early_data));
  // At this point, the server has half-RTT write keys, but it cannot access
  // 1-RTT read keys until client Finished.
  EXPECT_TRUE(transport_->server()->HasWriteSecret(ssl_encryption_application));
  EXPECT_FALSE(transport_->server()->HasReadSecret(ssl_encryption_application));

  // Finish up the client and server handshakes.
  ASSERT_TRUE(CompleteHandshakesForQUIC());

  // Both sides can now exchange 1-RTT data.
  ExpectHandshakeSuccess();
  EXPECT_TRUE(SSL_session_reused(client_.get()));
  EXPECT_TRUE(SSL_session_reused(server_.get()));
  EXPECT_FALSE(SSL_in_early_data(client_.get()));
  EXPECT_FALSE(SSL_in_early_data(server_.get()));
  EXPECT_TRUE(SSL_early_data_accepted(client_.get()));
  EXPECT_TRUE(SSL_early_data_accepted(server_.get()));
  EXPECT_EQ(SSL_get_early_data_reason(client_.get()), ssl_early_data_accepted);
  EXPECT_EQ(SSL_get_early_data_reason(server_.get()), ssl_early_data_accepted);
}

TEST_F(QUICMethodTest, ZeroRTTRejectMismatchedParameters) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_early_data_enabled(client_ctx_.get(), 1);
  SSL_CTX_set_early_data_enabled(server_ctx_.get(), 1);
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));


  bssl::UniquePtr<SSL_SESSION> session = CreateClientSessionForQUIC();
  ASSERT_TRUE(session);

  ASSERT_TRUE(CreateClientAndServer());
  static const uint8_t new_context[] = {4};
  ASSERT_TRUE(SSL_set_quic_early_data_context(server_.get(), new_context,
                                              sizeof(new_context)));
  SSL_set_session(client_.get(), session.get());

  // The client handshake should return immediately into the early data
  // state.
  ASSERT_EQ(SSL_do_handshake(client_.get()), 1);
  EXPECT_TRUE(SSL_in_early_data(client_.get()));
  // The transport should have keys for sending 0-RTT data.
  EXPECT_TRUE(transport_->client()->HasWriteSecret(ssl_encryption_early_data));

  // The server will consume the ClientHello, but it will not accept 0-RTT.
  ASSERT_TRUE(ProvideHandshakeData(server_.get()));
  ASSERT_EQ(SSL_do_handshake(server_.get()), -1);
  ASSERT_EQ(SSL_ERROR_WANT_READ, SSL_get_error(server_.get(), -1));
  EXPECT_FALSE(SSL_in_early_data(server_.get()));
  EXPECT_FALSE(transport_->server()->HasReadSecret(ssl_encryption_early_data));

  // The client consumes the server response and signals 0-RTT rejection.
  for (;;) {
    ASSERT_TRUE(ProvideHandshakeData(client_.get()));
    ASSERT_EQ(-1, SSL_do_handshake(client_.get()));
    int err = SSL_get_error(client_.get(), -1);
    if (err == SSL_ERROR_EARLY_DATA_REJECTED) {
      break;
    }
    ASSERT_EQ(SSL_ERROR_WANT_READ, err);
  }

  // As in TLS over TCP, 0-RTT rejection is sticky.
  ASSERT_EQ(-1, SSL_do_handshake(client_.get()));
  ASSERT_EQ(SSL_ERROR_EARLY_DATA_REJECTED, SSL_get_error(client_.get(), -1));

  // Finish up the client and server handshakes.
  SSL_reset_early_data_reject(client_.get());
  ASSERT_TRUE(CompleteHandshakesForQUIC());

  // Both sides can now exchange 1-RTT data.
  ExpectHandshakeSuccess();
  EXPECT_TRUE(SSL_session_reused(client_.get()));
  EXPECT_TRUE(SSL_session_reused(server_.get()));
  EXPECT_FALSE(SSL_in_early_data(client_.get()));
  EXPECT_FALSE(SSL_in_early_data(server_.get()));
  EXPECT_FALSE(SSL_early_data_accepted(client_.get()));
  EXPECT_FALSE(SSL_early_data_accepted(server_.get()));
}

TEST_F(QUICMethodTest, NoZeroRTTTicketWithoutEarlyDataContext) {
  server_quic_early_data_context_ = {};
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_early_data_enabled(client_ctx_.get(), 1);
  SSL_CTX_set_early_data_enabled(server_ctx_.get(), 1);
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  bssl::UniquePtr<SSL_SESSION> session = CreateClientSessionForQUIC();
  ASSERT_TRUE(session);
  EXPECT_FALSE(SSL_SESSION_early_data_capable(session.get()));
}

TEST_F(QUICMethodTest, ZeroRTTReject) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_early_data_enabled(client_ctx_.get(), 1);
  SSL_CTX_set_early_data_enabled(server_ctx_.get(), 1);
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  bssl::UniquePtr<SSL_SESSION> session = CreateClientSessionForQUIC();
  ASSERT_TRUE(session);

  for (bool reject_hrr : {false, true}) {
    SCOPED_TRACE(reject_hrr);

    ASSERT_TRUE(CreateClientAndServer());
    if (reject_hrr) {
      // Configure the server to prefer P-256, which will reject 0-RTT via
      // HelloRetryRequest.
      int p256 = NID_X9_62_prime256v1;
      ASSERT_TRUE(SSL_set1_groups(server_.get(), &p256, 1));
    } else {
      // Disable 0-RTT on the server, so it will reject it.
      SSL_set_early_data_enabled(server_.get(), 0);
    }
    SSL_set_session(client_.get(), session.get());

    // The client handshake should return immediately into the early data state.
    ASSERT_EQ(SSL_do_handshake(client_.get()), 1);
    EXPECT_TRUE(SSL_in_early_data(client_.get()));
    // The transport should have keys for sending 0-RTT data.
    EXPECT_TRUE(
        transport_->client()->HasWriteSecret(ssl_encryption_early_data));

    // The server will consume the ClientHello, but it will not accept 0-RTT.
    ASSERT_TRUE(ProvideHandshakeData(server_.get()));
    ASSERT_EQ(SSL_do_handshake(server_.get()), -1);
    ASSERT_EQ(SSL_ERROR_WANT_READ, SSL_get_error(server_.get(), -1));
    EXPECT_FALSE(SSL_in_early_data(server_.get()));
    EXPECT_FALSE(
        transport_->server()->HasReadSecret(ssl_encryption_early_data));

    // The client consumes the server response and signals 0-RTT rejection.
    for (;;) {
      ASSERT_TRUE(ProvideHandshakeData(client_.get()));
      ASSERT_EQ(-1, SSL_do_handshake(client_.get()));
      int err = SSL_get_error(client_.get(), -1);
      if (err == SSL_ERROR_EARLY_DATA_REJECTED) {
        break;
      }
      ASSERT_EQ(SSL_ERROR_WANT_READ, err);
    }

    // As in TLS over TCP, 0-RTT rejection is sticky.
    ASSERT_EQ(-1, SSL_do_handshake(client_.get()));
    ASSERT_EQ(SSL_ERROR_EARLY_DATA_REJECTED, SSL_get_error(client_.get(), -1));

    // Finish up the client and server handshakes.
    SSL_reset_early_data_reject(client_.get());
    ASSERT_TRUE(CompleteHandshakesForQUIC());

    // Both sides can now exchange 1-RTT data.
    ExpectHandshakeSuccess();
    EXPECT_TRUE(SSL_session_reused(client_.get()));
    EXPECT_TRUE(SSL_session_reused(server_.get()));
    EXPECT_FALSE(SSL_in_early_data(client_.get()));
    EXPECT_FALSE(SSL_in_early_data(server_.get()));
    EXPECT_FALSE(SSL_early_data_accepted(client_.get()));
    EXPECT_FALSE(SSL_early_data_accepted(server_.get()));
  }
}

TEST_F(QUICMethodTest, NoZeroRTTKeysBeforeReverify) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_early_data_enabled(client_ctx_.get(), 1);
  SSL_CTX_set_reverify_on_resume(client_ctx_.get(), 1);
  SSL_CTX_set_early_data_enabled(server_ctx_.get(), 1);
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  bssl::UniquePtr<SSL_SESSION> session = CreateClientSessionForQUIC();
  ASSERT_TRUE(session);

  ASSERT_TRUE(CreateClientAndServer());
  SSL_set_session(client_.get(), session.get());

  // Configure the certificate (re)verification to never complete. The client
  // handshake should pause.
  SSL_set_custom_verify(
      client_.get(), SSL_VERIFY_PEER,
      [](SSL *ssl, uint8_t *out_alert) -> ssl_verify_result_t {
        return ssl_verify_retry;
      });
  ASSERT_EQ(SSL_do_handshake(client_.get()), -1);
  ASSERT_EQ(SSL_get_error(client_.get(), -1),
            SSL_ERROR_WANT_CERTIFICATE_VERIFY);

  // The early data keys have not yet been released.
  EXPECT_FALSE(transport_->client()->HasWriteSecret(ssl_encryption_early_data));

  // After the verification completes, the handshake progresses to the 0-RTT
  // point and releases keys.
  SSL_set_custom_verify(
      client_.get(), SSL_VERIFY_PEER,
      [](SSL *ssl, uint8_t *out_alert) -> ssl_verify_result_t {
        return ssl_verify_ok;
      });
  ASSERT_EQ(SSL_do_handshake(client_.get()), 1);
  EXPECT_TRUE(SSL_in_early_data(client_.get()));
  EXPECT_TRUE(transport_->client()->HasWriteSecret(ssl_encryption_early_data));
}

// Test only releasing data to QUIC one byte at a time on request, to maximize
// state machine pauses. Additionally, test that existing asynchronous callbacks
// still work.
TEST_F(QUICMethodTest, Async) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));
  ASSERT_TRUE(CreateClientAndServer());

  // Install an asynchronous certificate callback.
  bool cert_cb_ok = false;
  SSL_set_cert_cb(
      server_.get(),
      [](SSL *, void *arg) -> int {
        return *static_cast<bool *>(arg) ? 1 : -1;
      },
      &cert_cb_ok);

  for (;;) {
    int client_ret = SSL_do_handshake(client_.get());
    if (client_ret != 1) {
      ASSERT_EQ(client_ret, -1);
      ASSERT_EQ(SSL_get_error(client_.get(), client_ret), SSL_ERROR_WANT_READ);
      ASSERT_TRUE(ProvideHandshakeData(client_.get(), 1));
    }

    int server_ret = SSL_do_handshake(server_.get());
    if (server_ret != 1) {
      ASSERT_EQ(server_ret, -1);
      int ssl_err = SSL_get_error(server_.get(), server_ret);
      switch (ssl_err) {
        case SSL_ERROR_WANT_READ:
          ASSERT_TRUE(ProvideHandshakeData(server_.get(), 1));
          break;
        case SSL_ERROR_WANT_X509_LOOKUP:
          ASSERT_FALSE(cert_cb_ok);
          cert_cb_ok = true;
          break;
        default:
          FAIL() << "Unexpected SSL_get_error result: " << ssl_err;
      }
    }

    if (client_ret == 1 && server_ret == 1) {
      break;
    }
  }

  ExpectHandshakeSuccess();
}

// Test buffering write data until explicit flushes.
TEST_F(QUICMethodTest, Buffered) {
  AllowOutOfOrderWrites();

  struct BufferedFlight {
    std::vector<uint8_t> data[kNumQUICLevels];
  };
  static UnownedSSLExData<BufferedFlight> buffered_flights;

  auto add_handshake_data = [](SSL *ssl, enum ssl_encryption_level_t level,
                               const uint8_t *data, size_t len) -> int {
    BufferedFlight *flight = buffered_flights.Get(ssl);
    flight->data[level].insert(flight->data[level].end(), data, data + len);
    return 1;
  };

  auto flush_flight = [](SSL *ssl) -> int {
    BufferedFlight *flight = buffered_flights.Get(ssl);
    for (size_t level = 0; level < kNumQUICLevels; level++) {
      if (!flight->data[level].empty()) {
        if (!TransportFromSSL(ssl)->WriteHandshakeData(
                static_cast<ssl_encryption_level_t>(level),
                flight->data[level])) {
          return 0;
        }
        flight->data[level].clear();
      }
    }
    return 1;
  };

  SSL_QUIC_METHOD quic_method = DefaultQUICMethod();
  quic_method.add_handshake_data = add_handshake_data;
  quic_method.flush_flight = flush_flight;

  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));
  ASSERT_TRUE(CreateClientAndServer());

  BufferedFlight client_flight, server_flight;
  ASSERT_TRUE(buffered_flights.Set(client_.get(), &client_flight));
  ASSERT_TRUE(buffered_flights.Set(server_.get(), &server_flight));

  ASSERT_TRUE(CompleteHandshakesForQUIC());

  ExpectHandshakeSuccess();
}

// Test that excess data at one level is rejected. That is, if a single
// |SSL_provide_quic_data| call included both ServerHello and
// EncryptedExtensions in a single chunk, BoringSSL notices and rejects this on
// key change.
TEST_F(QUICMethodTest, ExcessProvidedData) {
  AllowOutOfOrderWrites();

  auto add_handshake_data = [](SSL *ssl, enum ssl_encryption_level_t level,
                               const uint8_t *data, size_t len) -> int {
    // Switch everything to the initial level.
    return TransportFromSSL(ssl)->WriteHandshakeData(ssl_encryption_initial,
                                                     MakeConstSpan(data, len));
  };

  SSL_QUIC_METHOD quic_method = DefaultQUICMethod();
  quic_method.add_handshake_data = add_handshake_data;

  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));
  ASSERT_TRUE(CreateClientAndServer());

  // Send the ClientHello and ServerHello through Finished.
  ASSERT_EQ(SSL_do_handshake(client_.get()), -1);
  ASSERT_EQ(SSL_get_error(client_.get(), -1), SSL_ERROR_WANT_READ);
  ASSERT_TRUE(ProvideHandshakeData(server_.get()));
  ASSERT_EQ(SSL_do_handshake(server_.get()), -1);
  ASSERT_EQ(SSL_get_error(server_.get(), -1), SSL_ERROR_WANT_READ);

  // The client is still waiting for the ServerHello at initial
  // encryption.
  ASSERT_EQ(ssl_encryption_initial, SSL_quic_read_level(client_.get()));

  // |add_handshake_data| incorrectly wrote everything at the initial level, so
  // this queues up ServerHello through Finished in one chunk.
  ASSERT_TRUE(ProvideHandshakeData(client_.get()));

  // The client reads ServerHello successfully, but then rejects the buffered
  // EncryptedExtensions on key change.
  ASSERT_EQ(SSL_do_handshake(client_.get()), -1);
  ASSERT_EQ(SSL_get_error(client_.get(), -1), SSL_ERROR_SSL);
  EXPECT_TRUE(
      ErrorEquals(ERR_get_error(), ERR_LIB_SSL, SSL_R_EXCESS_HANDSHAKE_DATA));

  // The client sends an alert in response to this. The alert is sent at
  // handshake level because we install write secrets before read secrets and
  // the error is discovered when installing the read secret. (How to send
  // alerts on protocol syntax errors near key changes is ambiguous in general.)
  ASSERT_TRUE(transport_->client()->has_alert());
  EXPECT_EQ(transport_->client()->alert_level(), ssl_encryption_handshake);
  EXPECT_EQ(transport_->client()->alert(), SSL_AD_UNEXPECTED_MESSAGE);

  // Sanity-check handshake secrets. The error is discovered while setting the
  // read secret, so only the write secret has been installed.
  EXPECT_TRUE(transport_->client()->HasWriteSecret(ssl_encryption_handshake));
  EXPECT_FALSE(transport_->client()->HasReadSecret(ssl_encryption_handshake));
}

// Test that |SSL_provide_quic_data| will reject data at the wrong level.
TEST_F(QUICMethodTest, ProvideWrongLevel) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));
  ASSERT_TRUE(CreateClientAndServer());

  // Send the ClientHello and ServerHello through Finished.
  ASSERT_EQ(SSL_do_handshake(client_.get()), -1);
  ASSERT_EQ(SSL_get_error(client_.get(), -1), SSL_ERROR_WANT_READ);
  ASSERT_TRUE(ProvideHandshakeData(server_.get()));
  ASSERT_EQ(SSL_do_handshake(server_.get()), -1);
  ASSERT_EQ(SSL_get_error(server_.get(), -1), SSL_ERROR_WANT_READ);

  // The client is still waiting for the ServerHello at initial
  // encryption.
  ASSERT_EQ(ssl_encryption_initial, SSL_quic_read_level(client_.get()));

  // Data cannot be provided at the next level.
  std::vector<uint8_t> data;
  ASSERT_TRUE(
      transport_->client()->ReadHandshakeData(&data, ssl_encryption_initial));
  ASSERT_FALSE(SSL_provide_quic_data(client_.get(), ssl_encryption_handshake,
                                     data.data(), data.size()));
  ERR_clear_error();

  // Progress to EncryptedExtensions.
  ASSERT_TRUE(SSL_provide_quic_data(client_.get(), ssl_encryption_initial,
                                    data.data(), data.size()));
  ASSERT_EQ(SSL_do_handshake(client_.get()), -1);
  ASSERT_EQ(SSL_get_error(client_.get(), -1), SSL_ERROR_WANT_READ);
  ASSERT_EQ(ssl_encryption_handshake, SSL_quic_read_level(client_.get()));

  // Data cannot be provided at the previous level.
  ASSERT_TRUE(
      transport_->client()->ReadHandshakeData(&data, ssl_encryption_handshake));
  ASSERT_FALSE(SSL_provide_quic_data(client_.get(), ssl_encryption_initial,
                                     data.data(), data.size()));
}

TEST_F(QUICMethodTest, TooMuchData) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));
  ASSERT_TRUE(CreateClientAndServer());

  size_t limit =
      SSL_quic_max_handshake_flight_len(client_.get(), ssl_encryption_initial);
  uint8_t b = 0;
  for (size_t i = 0; i < limit; i++) {
    ASSERT_TRUE(
        SSL_provide_quic_data(client_.get(), ssl_encryption_initial, &b, 1));
  }

  EXPECT_FALSE(
      SSL_provide_quic_data(client_.get(), ssl_encryption_initial, &b, 1));
}

// Provide invalid post-handshake data.
TEST_F(QUICMethodTest, BadPostHandshake) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  g_last_session = nullptr;

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_sess_set_new_cb(client_ctx_.get(), SaveLastSession);
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));
  ASSERT_TRUE(CreateClientAndServer());
  ASSERT_TRUE(CompleteHandshakesForQUIC());

  EXPECT_EQ(SSL_do_handshake(client_.get()), 1);
  EXPECT_EQ(SSL_do_handshake(server_.get()), 1);
  EXPECT_TRUE(transport_->SecretsMatch(ssl_encryption_application));
  EXPECT_FALSE(transport_->client()->has_alert());
  EXPECT_FALSE(transport_->server()->has_alert());

  // Junk sent as part of post-handshake data should cause an error.
  uint8_t kJunk[] = {0x17, 0x0, 0x0, 0x4, 0xB, 0xE, 0xE, 0xF};
  ASSERT_TRUE(SSL_provide_quic_data(client_.get(), ssl_encryption_application,
                                    kJunk, sizeof(kJunk)));
  EXPECT_EQ(SSL_process_quic_post_handshake(client_.get()), 0);
}


static void ExpectReceivedTransportParamsEqual(const SSL *ssl,
                                               Span<const uint8_t> expected) {
  const uint8_t *received = nullptr;
  size_t received_len = 0;
  SSL_get_peer_quic_transport_params(ssl, &received, &received_len);
  ASSERT_EQ(received_len, expected.size());
  EXPECT_EQ(Bytes(received, received_len), Bytes(expected));
}

TEST_F(QUICMethodTest, SetTransportParameters) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  ASSERT_TRUE(CreateClientAndServer());
  uint8_t kClientParams[] = {1, 2, 3, 4};
  uint8_t kServerParams[] = {5, 6, 7};
  ASSERT_TRUE(SSL_set_quic_transport_params(client_.get(), kClientParams,
                                            sizeof(kClientParams)));
  ASSERT_TRUE(SSL_set_quic_transport_params(server_.get(), kServerParams,
                                            sizeof(kServerParams)));

  ASSERT_TRUE(CompleteHandshakesForQUIC());
  ExpectReceivedTransportParamsEqual(client_.get(), kServerParams);
  ExpectReceivedTransportParamsEqual(server_.get(), kClientParams);
}

TEST_F(QUICMethodTest, SetTransportParamsInCallback) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  ASSERT_TRUE(CreateClientAndServer());
  uint8_t kClientParams[] = {1, 2, 3, 4};
  static uint8_t kServerParams[] = {5, 6, 7};
  ASSERT_TRUE(SSL_set_quic_transport_params(client_.get(), kClientParams,
                                            sizeof(kClientParams)));
  SSL_CTX_set_tlsext_servername_callback(
      server_ctx_.get(), [](SSL *ssl, int *out_alert, void *arg) -> int {
        EXPECT_TRUE(SSL_set_quic_transport_params(ssl, kServerParams,
                                                  sizeof(kServerParams)));
        return SSL_TLSEXT_ERR_OK;
      });

  ASSERT_TRUE(CompleteHandshakesForQUIC());
  ExpectReceivedTransportParamsEqual(client_.get(), kServerParams);
  ExpectReceivedTransportParamsEqual(server_.get(), kClientParams);
}

TEST_F(QUICMethodTest, ForbidCrossProtocolResumptionClient) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  g_last_session = nullptr;

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_sess_set_new_cb(client_ctx_.get(), SaveLastSession);
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  ASSERT_TRUE(CreateClientAndServer());
  ASSERT_TRUE(CompleteHandshakesForQUIC());

  ExpectHandshakeSuccess();
  EXPECT_FALSE(SSL_session_reused(client_.get()));
  EXPECT_FALSE(SSL_session_reused(server_.get()));

  // The server sent NewSessionTicket messages in the handshake.
  EXPECT_FALSE(g_last_session);
  ASSERT_TRUE(ProvideHandshakeData(client_.get()));
  EXPECT_EQ(SSL_process_quic_post_handshake(client_.get()), 1);
  ASSERT_TRUE(g_last_session);

  // Pretend that g_last_session came from a TLS-over-TCP connection.
  g_last_session->is_quic = false;

  // Create a second connection and verify that resumption does not occur with
  // a session from a non-QUIC connection. This tests that the client does not
  // offer over QUIC a session believed to be received over TCP. The server
  // believes this is a QUIC session, so if the client offered the session, the
  // server would have resumed it.
  ASSERT_TRUE(CreateClientAndServer());
  bssl::UniquePtr<SSL_SESSION> session = std::move(g_last_session);
  SSL_set_session(client_.get(), session.get());

  ASSERT_TRUE(CompleteHandshakesForQUIC());
  ExpectHandshakeSuccess();
  EXPECT_FALSE(SSL_session_reused(client_.get()));
  EXPECT_FALSE(SSL_session_reused(server_.get()));
}

TEST_F(QUICMethodTest, ForbidCrossProtocolResumptionServer) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();

  g_last_session = nullptr;

  SSL_CTX_set_session_cache_mode(client_ctx_.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_sess_set_new_cb(client_ctx_.get(), SaveLastSession);
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  ASSERT_TRUE(CreateClientAndServer());
  ASSERT_TRUE(CompleteHandshakesForQUIC());

  ExpectHandshakeSuccess();
  EXPECT_FALSE(SSL_session_reused(client_.get()));
  EXPECT_FALSE(SSL_session_reused(server_.get()));

  // The server sent NewSessionTicket messages in the handshake.
  EXPECT_FALSE(g_last_session);
  ASSERT_TRUE(ProvideHandshakeData(client_.get()));
  EXPECT_EQ(SSL_process_quic_post_handshake(client_.get()), 1);
  ASSERT_TRUE(g_last_session);

  // Attempt a resumption with g_last_session using TLS_method.
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(client_ctx);

  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), nullptr));

  bssl::UniquePtr<SSL> client(SSL_new(client_ctx.get())),
      server(SSL_new(server_ctx_.get()));
  ASSERT_TRUE(client);
  ASSERT_TRUE(server);
  SSL_set_connect_state(client.get());
  SSL_set_accept_state(server.get());

  // The TLS-over-TCP client will refuse to resume with a quic session, so
  // mark is_quic = false to bypass the client check to test the server check.
  g_last_session->is_quic = false;
  SSL_set_session(client.get(), g_last_session.get());

  BIO *bio1 = nullptr, *bio2 = nullptr;
  ASSERT_TRUE(BIO_new_bio_pair(&bio1, 0, &bio2, 0));

  // SSL_set_bio takes ownership.
  SSL_set_bio(client.get(), bio1, bio1);
  SSL_set_bio(server.get(), bio2, bio2);
  ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

  EXPECT_FALSE(SSL_session_reused(client.get()));
  EXPECT_FALSE(SSL_session_reused(server.get()));
}

TEST_F(QUICMethodTest, ClientRejectsMissingTransportParams) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  ASSERT_TRUE(CreateClientAndServer());
  ASSERT_TRUE(SSL_set_quic_transport_params(server_.get(), nullptr, 0));
  ASSERT_TRUE(RunQUICHandshakesAndExpectError(ExpectedError::kServerError));
}

TEST_F(QUICMethodTest, ServerRejectsMissingTransportParams) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  ASSERT_TRUE(CreateClientAndServer());
  ASSERT_TRUE(SSL_set_quic_transport_params(client_.get(), nullptr, 0));
  ASSERT_TRUE(RunQUICHandshakesAndExpectError(ExpectedError::kClientError));
}

TEST_F(QUICMethodTest, QuicLegacyCodepointEnabled) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  ASSERT_TRUE(CreateClientAndServer());
  uint8_t kClientParams[] = {1, 2, 3, 4};
  uint8_t kServerParams[] = {5, 6, 7};
  SSL_set_quic_use_legacy_codepoint(client_.get(), 1);
  SSL_set_quic_use_legacy_codepoint(server_.get(), 1);
  ASSERT_TRUE(SSL_set_quic_transport_params(client_.get(), kClientParams,
                                            sizeof(kClientParams)));
  ASSERT_TRUE(SSL_set_quic_transport_params(server_.get(), kServerParams,
                                            sizeof(kServerParams)));

  ASSERT_TRUE(CompleteHandshakesForQUIC());
  ExpectReceivedTransportParamsEqual(client_.get(), kServerParams);
  ExpectReceivedTransportParamsEqual(server_.get(), kClientParams);
}

TEST_F(QUICMethodTest, QuicLegacyCodepointDisabled) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  ASSERT_TRUE(CreateClientAndServer());
  uint8_t kClientParams[] = {1, 2, 3, 4};
  uint8_t kServerParams[] = {5, 6, 7};
  SSL_set_quic_use_legacy_codepoint(client_.get(), 0);
  SSL_set_quic_use_legacy_codepoint(server_.get(), 0);
  ASSERT_TRUE(SSL_set_quic_transport_params(client_.get(), kClientParams,
                                            sizeof(kClientParams)));
  ASSERT_TRUE(SSL_set_quic_transport_params(server_.get(), kServerParams,
                                            sizeof(kServerParams)));

  ASSERT_TRUE(CompleteHandshakesForQUIC());
  ExpectReceivedTransportParamsEqual(client_.get(), kServerParams);
  ExpectReceivedTransportParamsEqual(server_.get(), kClientParams);
}

TEST_F(QUICMethodTest, QuicLegacyCodepointClientOnly) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  ASSERT_TRUE(CreateClientAndServer());
  uint8_t kClientParams[] = {1, 2, 3, 4};
  uint8_t kServerParams[] = {5, 6, 7};
  SSL_set_quic_use_legacy_codepoint(client_.get(), 1);
  SSL_set_quic_use_legacy_codepoint(server_.get(), 0);
  ASSERT_TRUE(SSL_set_quic_transport_params(client_.get(), kClientParams,
                                            sizeof(kClientParams)));
  ASSERT_TRUE(SSL_set_quic_transport_params(server_.get(), kServerParams,
                                            sizeof(kServerParams)));

  ASSERT_TRUE(RunQUICHandshakesAndExpectError(ExpectedError::kServerError));
}

TEST_F(QUICMethodTest, QuicLegacyCodepointServerOnly) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));

  ASSERT_TRUE(CreateClientAndServer());
  uint8_t kClientParams[] = {1, 2, 3, 4};
  uint8_t kServerParams[] = {5, 6, 7};
  SSL_set_quic_use_legacy_codepoint(client_.get(), 0);
  SSL_set_quic_use_legacy_codepoint(server_.get(), 1);
  ASSERT_TRUE(SSL_set_quic_transport_params(client_.get(), kClientParams,
                                            sizeof(kClientParams)));
  ASSERT_TRUE(SSL_set_quic_transport_params(server_.get(), kServerParams,
                                            sizeof(kServerParams)));

  ASSERT_TRUE(RunQUICHandshakesAndExpectError(ExpectedError::kServerError));
}

// Test that the default QUIC code point is consistent with
// |TLSEXT_TYPE_quic_transport_parameters|. This test ensures we remember to
// update the two values together.
TEST_F(QUICMethodTest, QuicCodePointDefault) {
  const SSL_QUIC_METHOD quic_method = DefaultQUICMethod();
  ASSERT_TRUE(SSL_CTX_set_quic_method(client_ctx_.get(), &quic_method));
  ASSERT_TRUE(SSL_CTX_set_quic_method(server_ctx_.get(), &quic_method));
  SSL_CTX_set_select_certificate_cb(
      server_ctx_.get(),
      [](const SSL_CLIENT_HELLO *client_hello) -> ssl_select_cert_result_t {
        const uint8_t *data = nullptr;
        size_t len = 0;
        if (!SSL_early_callback_ctx_extension_get(
                client_hello, TLSEXT_TYPE_quic_transport_parameters, &data,
                &len)) {
          ADD_FAILURE() << "Could not find quic_transport_parameters extension";
          return ssl_select_cert_error;
        }
        return ssl_select_cert_success;
      });

  ASSERT_TRUE(CreateClientAndServer());
  ASSERT_TRUE(CompleteHandshakesForQUIC());
}


BSSL_NAMESPACE_END
