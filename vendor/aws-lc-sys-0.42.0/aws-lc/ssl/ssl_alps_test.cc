// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include "openssl/bytestring.h"
#include "openssl/ssl.h"

#include "internal.h"
#include "ssl_common_test.h"

#include <gtest/gtest.h>

BSSL_NAMESPACE_BEGIN

namespace {

// This is a basic unit-test class to verify completing handshake successfully,
// sending the correct codepoint extension and having correct application
// setting on different combination of ALPS codepoint settings. More integration
// tests on runner.go.
class AlpsNewCodepointTest : public testing::Test {
 protected:
  void SetUp() override {
    client_ctx_.reset(SSL_CTX_new(TLS_method()));
    server_ctx_ = CreateContextWithTestCertificate(TLS_method());
    ASSERT_TRUE(client_ctx_);
    ASSERT_TRUE(server_ctx_);
  }

  void SetUpApplicationSetting() {
    static const uint8_t alpn[] = {0x03, 'f', 'o', 'o'};
    static const uint8_t proto[] = {'f', 'o', 'o'};
    static const uint8_t alps[] = {0x04, 'a', 'l', 'p', 's'};
    // SSL_set_alpn_protos's return value is backwards. It returns zero on
    // success and one on failure.
    ASSERT_FALSE(SSL_set_alpn_protos(client_.get(), alpn, sizeof(alpn)));
    SSL_CTX_set_alpn_select_cb(
        server_ctx_.get(),
        [](SSL *ssl, const uint8_t **out, uint8_t *out_len, const uint8_t *in,
           unsigned in_len, void *arg) -> int {
          return SSL_select_next_proto(const_cast<uint8_t **>(out), out_len, in,
                                       in_len, alpn,
                                       sizeof(alpn)) == OPENSSL_NPN_NEGOTIATED
                     ? SSL_TLSEXT_ERR_OK
                     : SSL_TLSEXT_ERR_NOACK;
        },
        nullptr);
    ASSERT_TRUE(SSL_add_application_settings(client_.get(), proto,
                                             sizeof(proto), nullptr, 0));
    ASSERT_TRUE(SSL_add_application_settings(
        server_.get(), proto, sizeof(proto), alps, sizeof(alps)));
  }

  bssl::UniquePtr<SSL_CTX> client_ctx_;
  bssl::UniquePtr<SSL_CTX> server_ctx_;

  bssl::UniquePtr<SSL> client_;
  bssl::UniquePtr<SSL> server_;
};


TEST_F(AlpsNewCodepointTest, Enabled) {
  SetUpExpectedNewCodePoint(server_ctx_.get());

  ASSERT_TRUE(CreateClientAndServer(&client_, &server_, client_ctx_.get(),
                                    server_ctx_.get()));

  SSL_set_alps_use_new_codepoint(client_.get(), 1);
  SSL_set_alps_use_new_codepoint(server_.get(), 1);

  SetUpApplicationSetting();
  ASSERT_TRUE(CompleteHandshakes(client_.get(), server_.get()));
  ASSERT_TRUE(SSL_has_application_settings(client_.get()));
}

TEST_F(AlpsNewCodepointTest, Disabled) {
  // Both client and server disable alps new codepoint.
  SetUpExpectedOldCodePoint(server_ctx_.get());

  ASSERT_TRUE(CreateClientAndServer(&client_, &server_, client_ctx_.get(),
                                    server_ctx_.get()));

  SSL_set_alps_use_new_codepoint(client_.get(), 0);
  SSL_set_alps_use_new_codepoint(server_.get(), 0);

  SetUpApplicationSetting();
  ASSERT_TRUE(CompleteHandshakes(client_.get(), server_.get()));
  ASSERT_TRUE(SSL_has_application_settings(client_.get()));
}

TEST_F(AlpsNewCodepointTest, ClientOnly) {
  // If client set new codepoint but server doesn't set, server ignores it.
  SetUpExpectedNewCodePoint(server_ctx_.get());

  ASSERT_TRUE(CreateClientAndServer(&client_, &server_, client_ctx_.get(),
                                    server_ctx_.get()));

  SSL_set_alps_use_new_codepoint(client_.get(), 1);
  SSL_set_alps_use_new_codepoint(server_.get(), 0);

  SetUpApplicationSetting();
  ASSERT_TRUE(CompleteHandshakes(client_.get(), server_.get()));
  ASSERT_FALSE(SSL_has_application_settings(client_.get()));
}

TEST_F(AlpsNewCodepointTest, ServerOnly) {
  // If client doesn't set new codepoint, while server set.
  SetUpExpectedOldCodePoint(server_ctx_.get());

  ASSERT_TRUE(CreateClientAndServer(&client_, &server_, client_ctx_.get(),
                                    server_ctx_.get()));

  SSL_set_alps_use_new_codepoint(client_.get(), 0);
  SSL_set_alps_use_new_codepoint(server_.get(), 1);

  SetUpApplicationSetting();
  ASSERT_TRUE(CompleteHandshakes(client_.get(), server_.get()));
  ASSERT_FALSE(SSL_has_application_settings(client_.get()));
}

void MoveBIOs(SSL *dest, SSL *src) {
  BIO *rbio = SSL_get_rbio(src);
  BIO_up_ref(rbio);
  SSL_set0_rbio(dest, rbio);

  BIO *wbio = SSL_get_wbio(src);
  BIO_up_ref(wbio);
  SSL_set0_wbio(dest, wbio);

  SSL_set0_rbio(src, nullptr);
  SSL_set0_wbio(src, nullptr);
}

void VerifyHandoff(bool use_new_alps_codepoint) {
  static const uint8_t alpn[] = {0x03, 'f', 'o', 'o'};
  static const uint8_t proto[] = {'f', 'o', 'o'};
  static const uint8_t alps[] = {0x04, 'a', 'l', 'p', 's'};

  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> handshaker_ctx(
      CreateContextWithTestCertificate(TLS_method()));
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);
  ASSERT_TRUE(handshaker_ctx);

  if (!use_new_alps_codepoint) {
    SetUpExpectedOldCodePoint(server_ctx.get());
  } else {
    SetUpExpectedNewCodePoint(server_ctx.get());
  }

  SSL_CTX_set_session_cache_mode(client_ctx.get(), SSL_SESS_CACHE_CLIENT);
  SSL_CTX_sess_set_new_cb(client_ctx.get(), SaveLastSession);
  SSL_CTX_set_handoff_mode(server_ctx.get(), true);
  uint8_t keys[48];
  SSL_CTX_get_tlsext_ticket_keys(server_ctx.get(), &keys, sizeof(keys));
  SSL_CTX_set_tlsext_ticket_keys(handshaker_ctx.get(), &keys, sizeof(keys));

  for (bool early_data : {false, true}) {
    SCOPED_TRACE(early_data);
    for (bool is_resume : {false, true}) {
      SCOPED_TRACE(is_resume);
      bssl::UniquePtr<SSL> client, server;
      ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                        server_ctx.get()));
      SSL_set_early_data_enabled(client.get(), early_data);

      // Set up client ALPS settings.
      SSL_set_alps_use_new_codepoint(client.get(), use_new_alps_codepoint);
      ASSERT_TRUE(SSL_set_alpn_protos(client.get(), alpn, sizeof(alpn)) == 0);
      ASSERT_TRUE(SSL_add_application_settings(client.get(), proto,
                                               sizeof(proto), nullptr, 0));
      if (is_resume) {
        ASSERT_TRUE(g_last_session);
        SSL_set_session(client.get(), g_last_session.get());
        if (early_data) {
          EXPECT_GT(g_last_session->ticket_max_early_data, 0u);
        }
      }


      int client_ret = SSL_do_handshake(client.get());
      int client_err = SSL_get_error(client.get(), client_ret);

      uint8_t byte_written = 0;
      if (early_data && is_resume) {
        ASSERT_EQ(client_err, 0);
        EXPECT_TRUE(SSL_in_early_data(client.get()));
        // Attempt to write early data.
        byte_written = 43;
        EXPECT_EQ(SSL_write(client.get(), &byte_written, 1), 1);
      } else {
        ASSERT_EQ(client_err, SSL_ERROR_WANT_READ);
      }

      int server_ret = SSL_do_handshake(server.get());
      int server_err = SSL_get_error(server.get(), server_ret);
      ASSERT_EQ(server_err, SSL_ERROR_HANDOFF);

      ScopedCBB cbb;
      Array<uint8_t> handoff;
      SSL_CLIENT_HELLO hello;
      ASSERT_TRUE(CBB_init(cbb.get(), 256));
      ASSERT_TRUE(SSL_serialize_handoff(server.get(), cbb.get(), &hello));
      ASSERT_TRUE(CBBFinishArray(cbb.get(), &handoff));

      bssl::UniquePtr<SSL> handshaker(SSL_new(handshaker_ctx.get()));
      ASSERT_TRUE(handshaker);
      // Note split handshakes determines 0-RTT support, for both the current
      // handshake and newly-issued tickets, entirely by |handshaker|. There is
      // no need to call |SSL_set_early_data_enabled| on |server|.
      SSL_set_early_data_enabled(handshaker.get(), 1);

      // Set up handshaker ALPS settings.
      SSL_set_alps_use_new_codepoint(handshaker.get(), use_new_alps_codepoint);
      SSL_CTX_set_alpn_select_cb(
          handshaker_ctx.get(),
          [](SSL *ssl, const uint8_t **out, uint8_t *out_len, const uint8_t *in,
             unsigned in_len, void *arg) -> int {
            return SSL_select_next_proto(const_cast<uint8_t **>(out), out_len,
                                         in, in_len, alpn,
                                         sizeof(alpn)) == OPENSSL_NPN_NEGOTIATED
                       ? SSL_TLSEXT_ERR_OK
                       : SSL_TLSEXT_ERR_NOACK;
          },
          nullptr);
      ASSERT_TRUE(SSL_add_application_settings(
          handshaker.get(), proto, sizeof(proto), alps, sizeof(alps)));

      ASSERT_TRUE(SSL_apply_handoff(handshaker.get(), handoff));

      MoveBIOs(handshaker.get(), server.get());

      int handshake_ret = SSL_do_handshake(handshaker.get());
      int handshake_err = SSL_get_error(handshaker.get(), handshake_ret);
      ASSERT_EQ(handshake_err, SSL_ERROR_HANDBACK);

      // Double-check that additional calls to |SSL_do_handshake| continue
      // to get |SSL_ERROR_HANDBACK|.
      handshake_ret = SSL_do_handshake(handshaker.get());
      handshake_err = SSL_get_error(handshaker.get(), handshake_ret);
      ASSERT_EQ(handshake_err, SSL_ERROR_HANDBACK);

      ScopedCBB cbb_handback;
      Array<uint8_t> handback;
      ASSERT_TRUE(CBB_init(cbb_handback.get(), 1024));
      ASSERT_TRUE(SSL_serialize_handback(handshaker.get(), cbb_handback.get()));
      ASSERT_TRUE(CBBFinishArray(cbb_handback.get(), &handback));

      bssl::UniquePtr<SSL> server2(SSL_new(server_ctx.get()));
      ASSERT_TRUE(server2);
      ASSERT_TRUE(SSL_apply_handback(server2.get(), handback));

      MoveBIOs(server2.get(), handshaker.get());
      ASSERT_TRUE(CompleteHandshakes(client.get(), server2.get()));
      EXPECT_EQ(is_resume, SSL_session_reused(client.get()));
      // Verify application settings.
      ASSERT_TRUE(SSL_has_application_settings(client.get()));

      if (early_data && is_resume) {
        // In this case, one byte of early data has already been written above.
        EXPECT_TRUE(SSL_early_data_accepted(client.get()));
      } else {
        byte_written = 42;
        EXPECT_EQ(SSL_write(client.get(), &byte_written, 1), 1);
      }
      uint8_t byte = 0;
      EXPECT_EQ(SSL_read(server2.get(), &byte, 1), 1);
      EXPECT_EQ(byte_written, byte);

      byte = 44;
      EXPECT_EQ(SSL_write(server2.get(), &byte, 1), 1);
      EXPECT_EQ(SSL_read(client.get(), &byte, 1), 1);
      EXPECT_EQ(44, byte);
    }
  }
}

TEST(SSLTest, Handoff) {
  for (bool use_new_alps_codepoint : {false, true}) {
    SCOPED_TRACE(use_new_alps_codepoint);
    VerifyHandoff(use_new_alps_codepoint);
  }
}

}  // namespace
BSSL_NAMESPACE_END
