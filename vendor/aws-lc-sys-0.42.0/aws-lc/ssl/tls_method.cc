// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <assert.h>
#include <string.h>

#include <openssl/err.h>

#include "../crypto/internal.h"
#include "internal.h"


BSSL_NAMESPACE_BEGIN

static void tls_on_handshake_complete(SSL *ssl) {
  // The handshake should have released its final message.
  assert(!ssl->s3->has_message);

  // During the handshake, |hs_buf| is retained. Release if it there is no
  // excess in it. There should not be any excess because the handshake logic
  // rejects unprocessed data after each Finished message. Note this means we do
  // not allow a TLS 1.2 HelloRequest to be packed into the same record as
  // Finished. (Schannel also rejects this.)
  assert(!ssl->s3->hs_buf || ssl->s3->hs_buf->length == 0);
  if (ssl->s3->hs_buf && ssl->s3->hs_buf->length == 0) {
    ssl->s3->hs_buf.reset();
  }
}

static bool tls_set_read_state(SSL *ssl, ssl_encryption_level_t level,
                               UniquePtr<SSLAEADContext> aead_ctx,
                               Span<const uint8_t> secret_for_quic) {
  // Cipher changes are forbidden if the current epoch has leftover data.
  if (tls_has_unprocessed_handshake_data(ssl)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_EXCESS_HANDSHAKE_DATA);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_UNEXPECTED_MESSAGE);
    return false;
  }

  if (ssl->quic_method != nullptr) {
    if ((ssl->s3->hs == nullptr || !ssl->s3->hs->hints_requested) &&
        !ssl->quic_method->set_read_secret(ssl, level, aead_ctx->cipher(),
                                           secret_for_quic.data(),
                                           secret_for_quic.size())) {
      return false;
    }

    // QUIC only uses |ssl| for handshake messages, which never use early data
    // keys, so we return without installing anything. This avoids needing to
    // have two secrets active at once in 0-RTT.
    if (level == ssl_encryption_early_data) {
      return true;
    }
  }

  OPENSSL_memset(ssl->s3->read_sequence, 0, sizeof(ssl->s3->read_sequence));
  ssl->s3->aead_read_ctx = std::move(aead_ctx);
  ssl->s3->read_level = level;
  return true;
}

static bool tls_set_write_state(SSL *ssl, ssl_encryption_level_t level,
                                UniquePtr<SSLAEADContext> aead_ctx,
                                Span<const uint8_t> secret_for_quic) {
  if (!tls_flush_pending_hs_data(ssl)) {
    return false;
  }

  if (ssl->quic_method != nullptr) {
    if ((ssl->s3->hs == nullptr || !ssl->s3->hs->hints_requested) &&
        !ssl->quic_method->set_write_secret(ssl, level, aead_ctx->cipher(),
                                            secret_for_quic.data(),
                                            secret_for_quic.size())) {
      return false;
    }

    // QUIC only uses |ssl| for handshake messages, which never use early data
    // keys, so we return without installing anything. This avoids needing to
    // have two secrets active at once in 0-RTT.
    if (level == ssl_encryption_early_data) {
      return true;
    }
  }

  OPENSSL_memset(ssl->s3->write_sequence, 0, sizeof(ssl->s3->write_sequence));
  ssl->s3->aead_write_ctx = std::move(aead_ctx);
  ssl->s3->write_level = level;
  return true;
}

static const SSL_PROTOCOL_METHOD kTLSProtocolMethod = {
    false /* is_dtls */,
    tls_new,
    tls_free,
    tls_get_message,
    tls_next_message,
    tls_has_unprocessed_handshake_data,
    tls_open_handshake,
    tls_open_change_cipher_spec,
    tls_open_app_data,
    tls_write_app_data,
    tls_dispatch_alert,
    tls_init_message,
    tls_finish_message,
    tls_add_message,
    tls_add_change_cipher_spec,
    tls_flush_flight,
    tls_on_handshake_complete,
    tls_set_read_state,
    tls_set_write_state,
};

static bool ssl_noop_x509_check_client_CA_names(
    STACK_OF(CRYPTO_BUFFER) *names) {
  return true;
}

static void ssl_noop_x509_clear(CERT *cert) {}
static void ssl_noop_x509_free(CERT *cert) {}
static void ssl_noop_x509_dup(CERT *new_cert, const CERT *cert) {}
static void ssl_noop_x509_flush_cached_leaf(CERT *cert) {}
static void ssl_noop_x509_flush_cached_chain(CERT *cert) {}
static bool ssl_noop_x509_session_cache_objects(SSL_SESSION *sess) {
  return true;
}
static bool ssl_noop_x509_session_dup(SSL_SESSION *new_session,
                                      const SSL_SESSION *session) {
  return true;
}
static void ssl_noop_x509_session_clear(SSL_SESSION *session) {}
static bool ssl_noop_x509_session_verify_cert_chain(SSL_SESSION *session,
                                                    SSL_HANDSHAKE *hs,
                                                    uint8_t *out_alert) {
  return false;
}

static void ssl_noop_x509_hs_flush_cached_ca_names(SSL_HANDSHAKE *hs) {}
static bool ssl_noop_x509_ssl_new(SSL_HANDSHAKE *hs) { return true; }
static void ssl_noop_x509_ssl_config_free(SSL_CONFIG *cfg) {}
static void ssl_noop_x509_ssl_flush_cached_client_CA(SSL_CONFIG *cfg) {}
static bool ssl_noop_x509_ssl_auto_chain_if_needed(SSL_HANDSHAKE *hs) {
  return true;
}
static bool ssl_noop_x509_ssl_ctx_new(SSL_CTX *ctx) { return true; }
static void ssl_noop_x509_ssl_ctx_free(SSL_CTX *ctx) {}
static void ssl_noop_x509_ssl_ctx_flush_cached_client_CA(SSL_CTX *ctx) {}

const SSL_X509_METHOD ssl_noop_x509_method = {
  ssl_noop_x509_check_client_CA_names,
  ssl_noop_x509_clear,
  ssl_noop_x509_free,
  ssl_noop_x509_dup,
  ssl_noop_x509_flush_cached_chain,
  ssl_noop_x509_flush_cached_leaf,
  ssl_noop_x509_session_cache_objects,
  ssl_noop_x509_session_dup,
  ssl_noop_x509_session_clear,
  ssl_noop_x509_session_verify_cert_chain,
  ssl_noop_x509_hs_flush_cached_ca_names,
  ssl_noop_x509_ssl_new,
  ssl_noop_x509_ssl_config_free,
  ssl_noop_x509_ssl_flush_cached_client_CA,
  ssl_noop_x509_ssl_auto_chain_if_needed,
  ssl_noop_x509_ssl_ctx_new,
  ssl_noop_x509_ssl_ctx_free,
  ssl_noop_x509_ssl_ctx_flush_cached_client_CA,
};

BSSL_NAMESPACE_END

using namespace bssl;

const SSL_METHOD *TLS_method(void) {
  static const SSL_METHOD kMethod = {
      0,
      &kTLSProtocolMethod,
      &ssl_crypto_x509_method,
  };
  return &kMethod;
}

const SSL_METHOD *SSLv23_method(void) {
  return TLS_method();
}

const SSL_METHOD *TLS_with_buffers_method(void) {
  static const SSL_METHOD kMethod = {
      0,
      &kTLSProtocolMethod,
      &ssl_noop_x509_method,
  };
  return &kMethod;
}

// Legacy version-locked methods.

const SSL_METHOD *TLSv1_2_method(void) {
  static const SSL_METHOD kMethod = {
      TLS1_2_VERSION,
      &kTLSProtocolMethod,
      &ssl_crypto_x509_method,
  };
  return &kMethod;
}

const SSL_METHOD *TLSv1_1_method(void) {
  static const SSL_METHOD kMethod = {
      TLS1_1_VERSION,
      &kTLSProtocolMethod,
      &ssl_crypto_x509_method,
  };
  return &kMethod;
}

const SSL_METHOD *TLSv1_method(void) {
  static const SSL_METHOD kMethod = {
      TLS1_VERSION,
      &kTLSProtocolMethod,
      &ssl_crypto_x509_method,
  };
  return &kMethod;
}

// Legacy side-specific methods.

const SSL_METHOD *TLSv1_2_server_method(void) {
  return TLSv1_2_method();
}

const SSL_METHOD *TLSv1_1_server_method(void) {
  return TLSv1_1_method();
}

const SSL_METHOD *TLSv1_server_method(void) {
  return TLSv1_method();
}

const SSL_METHOD *TLSv1_2_client_method(void) {
  return TLSv1_2_method();
}

const SSL_METHOD *TLSv1_1_client_method(void) {
  return TLSv1_1_method();
}

const SSL_METHOD *TLSv1_client_method(void) {
  return TLSv1_method();
}

const SSL_METHOD *SSLv23_server_method(void) {
  return SSLv23_method();
}

const SSL_METHOD *SSLv23_client_method(void) {
  return SSLv23_method();
}

const SSL_METHOD *TLS_server_method(void) {
  return TLS_method();
}

const SSL_METHOD *TLS_client_method(void) {
  return TLS_method();
}
