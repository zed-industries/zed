// DTLS implementation written by Nagendra Modadugu (nagendra@cs.stanford.edu) for the OpenSSL project 2005.
// Copyright (c) 1999-2005 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <assert.h>
#include <string.h>

#include <openssl/err.h>

#include "../crypto/internal.h"
#include "internal.h"


using namespace bssl;

static void dtls1_on_handshake_complete(SSL *ssl) {
  // Stop the reply timer left by the last flight we sent.
  dtls1_stop_timer(ssl);
  // If the final flight had a reply, we know the peer has received it. If not,
  // we must leave the flight around for post-handshake retransmission.
  if (ssl->d1->flight_has_reply) {
    dtls_clear_outgoing_messages(ssl);
  }
}

static bool dtls1_set_read_state(SSL *ssl, ssl_encryption_level_t level,
                                 UniquePtr<SSLAEADContext> aead_ctx,
                                 Span<const uint8_t> secret_for_quic) {
  assert(secret_for_quic.empty());  // QUIC does not use DTLS.
  // Cipher changes are forbidden if the current epoch has leftover data.
  if (dtls_has_unprocessed_handshake_data(ssl)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_EXCESS_HANDSHAKE_DATA);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_UNEXPECTED_MESSAGE);
    return false;
  }

  ssl->d1->r_epoch++;
  ssl->d1->bitmap = DTLS1_BITMAP();
  OPENSSL_memset(ssl->s3->read_sequence, 0, sizeof(ssl->s3->read_sequence));

  ssl->s3->aead_read_ctx = std::move(aead_ctx);
  ssl->s3->read_level = level;
  ssl->d1->has_change_cipher_spec = false;
  return true;
}

static bool dtls1_set_write_state(SSL *ssl, ssl_encryption_level_t level,
                                  UniquePtr<SSLAEADContext> aead_ctx,
                                  Span<const uint8_t> secret_for_quic) {
  assert(secret_for_quic.empty());  // QUIC does not use DTLS.
  ssl->d1->w_epoch++;
  OPENSSL_memcpy(ssl->d1->last_write_sequence, ssl->s3->write_sequence,
                 sizeof(ssl->s3->write_sequence));
  OPENSSL_memset(ssl->s3->write_sequence, 0, sizeof(ssl->s3->write_sequence));

  ssl->d1->last_aead_write_ctx = std::move(ssl->s3->aead_write_ctx);
  ssl->s3->aead_write_ctx = std::move(aead_ctx);
  ssl->s3->write_level = level;
  return true;
}

static const SSL_PROTOCOL_METHOD kDTLSProtocolMethod = {
    true /* is_dtls */,
    dtls1_new,
    dtls1_free,
    dtls1_get_message,
    dtls1_next_message,
    dtls_has_unprocessed_handshake_data,
    dtls1_open_handshake,
    dtls1_open_change_cipher_spec,
    dtls1_open_app_data,
    dtls1_write_app_data,
    dtls1_dispatch_alert,
    dtls1_init_message,
    dtls1_finish_message,
    dtls1_add_message,
    dtls1_add_change_cipher_spec,
    dtls1_flush_flight,
    dtls1_on_handshake_complete,
    dtls1_set_read_state,
    dtls1_set_write_state,
};

const SSL_METHOD *DTLS_method(void) {
  static const SSL_METHOD kMethod = {
      0,
      &kDTLSProtocolMethod,
      &ssl_crypto_x509_method,
  };
  return &kMethod;
}

const SSL_METHOD *DTLS_with_buffers_method(void) {
  static const SSL_METHOD kMethod = {
      0,
      &kDTLSProtocolMethod,
      &ssl_noop_x509_method,
  };
  return &kMethod;
}

// Legacy version-locked methods.

const SSL_METHOD *DTLSv1_2_method(void) {
  static const SSL_METHOD kMethod = {
      DTLS1_2_VERSION,
      &kDTLSProtocolMethod,
      &ssl_crypto_x509_method,
  };
  return &kMethod;
}

const SSL_METHOD *DTLSv1_method(void) {
  static const SSL_METHOD kMethod = {
      DTLS1_VERSION,
      &kDTLSProtocolMethod,
      &ssl_crypto_x509_method,
  };
  return &kMethod;
}

// Legacy side-specific methods.

const SSL_METHOD *DTLSv1_2_server_method(void) {
  return DTLSv1_2_method();
}

const SSL_METHOD *DTLSv1_server_method(void) {
  return DTLSv1_method();
}

const SSL_METHOD *DTLSv1_2_client_method(void) {
  return DTLSv1_2_method();
}

const SSL_METHOD *DTLSv1_client_method(void) {
  return DTLSv1_method();
}

const SSL_METHOD *DTLS_server_method(void) {
  return DTLS_method();
}

const SSL_METHOD *DTLS_client_method(void) {
  return DTLS_method();
}
