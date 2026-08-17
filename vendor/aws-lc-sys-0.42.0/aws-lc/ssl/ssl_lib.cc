// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2007 The OpenSSL Project.  All rights reserved.
// Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
// Copyright 2005 Nokia. All rights reserved.
//
// ECC cipher suite support in OpenSSL originally developed by
// SUN MICROSYSTEMS, INC., and contributed to the OpenSSL project.
//
// The Contribution, originally written by Mika Kousa and Pasi Eronen of
// Nokia Corporation, consists of the "PSK" (Pre-Shared Key) ciphersuites
// support (see RFC 4279) to OpenSSL.
//
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <algorithm>

#include <assert.h>
#include <limits.h>
#include <stdlib.h>
#include <string.h>

#include <openssl/bytestring.h>
#include <openssl/crypto.h>
#include <openssl/err.h>
#include <openssl/lhash.h>
#include <openssl/mem.h>
#include <openssl/rand.h>

#include "../crypto/internal.h"
#include "../crypto/x509/internal.h"
#include "internal.h"

#if defined(OPENSSL_WINDOWS)
#include <sys/timeb.h>
#else
#include <sys/socket.h>
#include <sys/time.h>
#endif



BSSL_NAMESPACE_BEGIN

#define GUARD_SUSPENDED_STATE(ptr,code)                         \
  do {                                                           \
    if (ptr->is_suspended_state) {                               \
      OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED); \
      return code;                                               \
    }                                                            \
  } while (0)

// |SSL_R_UNKNOWN_PROTOCOL| is no longer emitted, but continue to define it
// to avoid downstream churn.
OPENSSL_DECLARE_ERROR_REASON(SSL, UNKNOWN_PROTOCOL)

// The following errors are no longer emitted, but are used in nginx without
// #ifdefs.
OPENSSL_DECLARE_ERROR_REASON(SSL, BLOCK_CIPHER_PAD_IS_WRONG)
OPENSSL_DECLARE_ERROR_REASON(SSL, NO_CIPHERS_SPECIFIED)

// Some error codes are special. Ensure the make_errors.go script never
// regresses this.
static_assert(SSL_R_TLSV1_ALERT_NO_RENEGOTIATION ==
                  SSL_AD_NO_RENEGOTIATION + SSL_AD_REASON_OFFSET,
              "alert reason code mismatch");

// kMaxHandshakeSize is the maximum size, in bytes, of a handshake message.
static const size_t kMaxHandshakeSize = (1u << 24) - 1;

static CRYPTO_EX_DATA_CLASS g_ex_data_class_ssl =
    CRYPTO_EX_DATA_CLASS_INIT_WITH_APP_DATA;
static CRYPTO_EX_DATA_CLASS g_ex_data_class_ssl_ctx =
    CRYPTO_EX_DATA_CLASS_INIT_WITH_APP_DATA;

bool CBBFinishArray(CBB *cbb, Array<uint8_t> *out) {
  uint8_t *ptr;
  size_t len;
  if (!CBB_finish(cbb, &ptr, &len)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }
  out->Reset(ptr, len);
  return true;
}

void ssl_reset_error_state(SSL *ssl) {
  // Functions which use |SSL_get_error| must reset I/O and error state on
  // entry.
  ssl->s3->rwstate = SSL_ERROR_NONE;
  ERR_clear_error();
  ERR_clear_system_error();
}

void ssl_set_read_error(SSL *ssl) {
  ssl->s3->read_shutdown = ssl_shutdown_error;
  ssl->s3->read_error.reset(ERR_save_state());
}

static bool check_read_error(const SSL *ssl) {
  if (ssl->s3->read_shutdown == ssl_shutdown_error) {
    ERR_restore_state(ssl->s3->read_error.get());
    return false;
  }
  return true;
}

bool ssl_can_write(const SSL *ssl) {
  return !SSL_in_init(ssl) || ssl->s3->hs->can_early_write;
}

bool ssl_can_read(const SSL *ssl) {
  return !SSL_in_init(ssl) || ssl->s3->hs->can_early_read;
}

ssl_open_record_t ssl_open_handshake(SSL *ssl, size_t *out_consumed,
                                     uint8_t *out_alert, Span<uint8_t> in) {
  *out_consumed = 0;
  if (!check_read_error(ssl)) {
    *out_alert = 0;
    return ssl_open_record_error;
  }
  auto ret = ssl->method->open_handshake(ssl, out_consumed, out_alert, in);
  if (ret == ssl_open_record_error) {
    ssl_set_read_error(ssl);
  }
  return ret;
}

ssl_open_record_t ssl_open_change_cipher_spec(SSL *ssl, size_t *out_consumed,
                                              uint8_t *out_alert,
                                              Span<uint8_t> in) {
  *out_consumed = 0;
  if (!check_read_error(ssl)) {
    *out_alert = 0;
    return ssl_open_record_error;
  }
  auto ret =
      ssl->method->open_change_cipher_spec(ssl, out_consumed, out_alert, in);
  if (ret == ssl_open_record_error) {
    ssl_set_read_error(ssl);
  }
  return ret;
}

ssl_open_record_t ssl_open_app_data(SSL *ssl, Span<uint8_t> *out,
                                    size_t *out_consumed, uint8_t *out_alert,
                                    Span<uint8_t> in) {
  *out_consumed = 0;
  if (!check_read_error(ssl)) {
    *out_alert = 0;
    return ssl_open_record_error;
  }
  auto ret = ssl->method->open_app_data(ssl, out, out_consumed, out_alert, in);
  if (ret == ssl_open_record_error) {
    ssl_set_read_error(ssl);
  }
  return ret;
}

static bool cbb_add_hex(CBB *cbb, Span<const uint8_t> in) {
  static const char hextable[] = "0123456789abcdef";
  uint8_t *out;

  if (!CBB_add_space(cbb, &out, in.size() * 2)) {
    return false;
  }

  for (uint8_t b : in) {
    *(out++) = (uint8_t)hextable[b >> 4];
    *(out++) = (uint8_t)hextable[b & 0xf];
  }

  return true;
}

bool ssl_log_secret(const SSL *ssl, const char *label,
                    Span<const uint8_t> secret) {
  if (ssl->ctx->keylog_callback == NULL) {
    return true;
  }

  ScopedCBB cbb;
  Array<uint8_t> line;
  if (!CBB_init(cbb.get(), strlen(label) + 1 + SSL3_RANDOM_SIZE * 2 + 1 +
                               secret.size() * 2 + 1) ||
      !CBB_add_bytes(cbb.get(), reinterpret_cast<const uint8_t *>(label),
                     strlen(label)) ||
      !CBB_add_u8(cbb.get(), ' ') ||
      !cbb_add_hex(cbb.get(), ssl->s3->client_random) ||
      !CBB_add_u8(cbb.get(), ' ') || !cbb_add_hex(cbb.get(), secret) ||
      !CBB_add_u8(cbb.get(), 0 /* NUL */) ||
      !CBBFinishArray(cbb.get(), &line)) {
    return false;
  }

  ssl->ctx->keylog_callback(ssl, reinterpret_cast<const char *>(line.data()));
  return true;
}

void ssl_do_info_callback(const SSL *ssl, int type, int value) {
  void (*cb)(const SSL *ssl, int type, int value) = NULL;
  if (ssl->info_callback != NULL) {
    cb = ssl->info_callback;
  } else if (ssl->ctx->info_callback != NULL) {
    cb = ssl->ctx->info_callback;
  }

  if (cb != NULL) {
    cb(ssl, type, value);
  }
}

void ssl_do_msg_callback(const SSL *ssl, int is_write, int content_type,
                         Span<const uint8_t> in) {
  if (ssl->msg_callback == NULL) {
    return;
  }

  // |version| is zero when calling for |SSL3_RT_HEADER| and |SSL2_VERSION| for
  // a V2ClientHello.
  int version;
  switch (content_type) {
    case 0:
      // V2ClientHello
      version = SSL2_VERSION;
      break;
    case SSL3_RT_HEADER:
      version = 0;
      break;
    default:
      version = SSL_version(ssl);
  }

  ssl->msg_callback(is_write, version, content_type, in.data(), in.size(),
                    const_cast<SSL *>(ssl), ssl->msg_callback_arg);
}

void ssl_get_current_time(const SSL *ssl, struct OPENSSL_timeval *out_clock) {
  // TODO(martinkr): Change callers to |ssl_ctx_get_current_time| and drop the
  // |ssl| arg from |current_time_cb| if possible.
  ssl_ctx_get_current_time(ssl->ctx.get(), out_clock);
}

void ssl_ctx_get_current_time(const SSL_CTX *ctx,
                              struct OPENSSL_timeval *out_clock) {
  if (ctx->current_time_cb != NULL) {
    // TODO(davidben): Update current_time_cb to use OPENSSL_timeval. See
    // https://crbug.com/boringssl/155.
    struct timeval clock;
    ctx->current_time_cb(nullptr /* ssl */, &clock);
    if (clock.tv_sec < 0) {
      assert(0);
      out_clock->tv_sec = 0;
      out_clock->tv_usec = 0;
    } else {
      out_clock->tv_sec = (uint64_t)clock.tv_sec;
      out_clock->tv_usec = (uint32_t)clock.tv_usec;
    }
    return;
  }

#if defined(BORINGSSL_UNSAFE_DETERMINISTIC_MODE)
  out_clock->tv_sec = 1234;
  out_clock->tv_usec = 1234;
#elif defined(OPENSSL_WINDOWS)
  struct _timeb time;
  _ftime(&time);
  if (time.time < 0) {
    assert(0);
    out_clock->tv_sec = 0;
    out_clock->tv_usec = 0;
  } else {
    out_clock->tv_sec = time.time;
    out_clock->tv_usec = time.millitm * 1000;
  }
#else
  struct timeval clock;
  gettimeofday(&clock, NULL);
  if (clock.tv_sec < 0) {
    assert(0);
    out_clock->tv_sec = 0;
    out_clock->tv_usec = 0;
  } else {
    out_clock->tv_sec = (uint64_t)clock.tv_sec;
    out_clock->tv_usec = (uint32_t)clock.tv_usec;
  }
#endif
}

void SSL_CTX_set_handoff_mode(SSL_CTX *ctx, bool on) { ctx->handoff = on; }

static bool ssl_can_renegotiate(const SSL *ssl) {
  if (ssl->server || SSL_is_dtls(ssl)) {
    return false;
  }

  if (ssl->s3->have_version && ssl_protocol_version(ssl) >= TLS1_3_VERSION) {
    return false;
  }

  // The config has already been shed.
  if (!ssl->config) {
    return false;
  }

  switch (ssl->renegotiate_mode) {
    case ssl_renegotiate_ignore:
    case ssl_renegotiate_never:
      return false;

    case ssl_renegotiate_freely:
    case ssl_renegotiate_explicit:
      return true;
    case ssl_renegotiate_once:
      return ssl->s3->total_renegotiations == 0;
  }

  assert(0);
  return false;
}

static void ssl_maybe_shed_handshake_config(SSL *ssl) {
  if (ssl->s3->hs != nullptr || ssl->config == nullptr ||
      !ssl->config->shed_handshake_config || ssl_can_renegotiate(ssl)) {
    return;
  }

  ssl->config.reset();
}

void SSL_set_handoff_mode(SSL *ssl, bool on) {
  if (!ssl->config) {
    return;
  }
  ssl->config->handoff = on;
}

bool SSL_get_traffic_secrets(const SSL *ssl,
                             Span<const uint8_t> *out_read_traffic_secret,
                             Span<const uint8_t> *out_write_traffic_secret) {
  if (SSL_version(ssl) < TLS1_3_VERSION) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_SSL_VERSION);
    return false;
  }

  if (!ssl->s3->initial_handshake_complete) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_HANDSHAKE_NOT_COMPLETE);
    return false;
  }

  *out_read_traffic_secret = Span<const uint8_t>(
      ssl->s3->read_traffic_secret, ssl->s3->read_traffic_secret_len);
  *out_write_traffic_secret = Span<const uint8_t>(
      ssl->s3->write_traffic_secret, ssl->s3->write_traffic_secret_len);

  return true;
}

void ssl_update_counter(SSL_CTX *ctx, SSL_STATS_COUNTER_TYPE &counter, bool lock) {
#if defined(OPENSSL_STATS_C11_ATOMIC)
  counter.fetch_add(1, std::memory_order_relaxed);
#else
  if (lock) {
    MutexWriteLock ctx_lock(&ctx->lock);
    counter++;
  } else {
    // Lock is already held by caller
    counter++;
  }
#endif
}

static int ssl_read_counter(const SSL_CTX *ctx, const SSL_STATS_COUNTER_TYPE &counter) {
#if defined(OPENSSL_STATS_C11_ATOMIC)
  return counter.load(std::memory_order_relaxed);
#else
  MutexReadLock lock(const_cast<CRYPTO_MUTEX *>(&ctx->lock));
  return counter;
#endif
}

void SSL_CTX_set_aes_hw_override_for_testing(SSL_CTX *ctx,
                                             bool override_value) {
  ctx->aes_hw_override = true;
  ctx->aes_hw_override_value = override_value;
}

void SSL_set_aes_hw_override_for_testing(SSL *ssl, bool override_value) {
  ssl->config->aes_hw_override = true;
  ssl->config->aes_hw_override_value = override_value;
}

BSSL_NAMESPACE_END

using namespace bssl;

int SSL_library_init(void) {
  CRYPTO_library_init();
  return 1;
}

int OPENSSL_init_ssl(uint64_t opts, const OPENSSL_INIT_SETTINGS *settings) {
  CRYPTO_library_init();
  return 1;
}

static uint32_t ssl_session_hash(const SSL_SESSION *sess) {
  return ssl_hash_session_id(
      MakeConstSpan(sess->session_id, sess->session_id_length));
}

static int ssl_session_cmp(const SSL_SESSION *a, const SSL_SESSION *b) {
  if (a->session_id_length != b->session_id_length) {
    return 1;
  }

  return OPENSSL_memcmp(a->session_id, b->session_id, a->session_id_length);
}

ssl_ctx_st::ssl_ctx_st(const SSL_METHOD *ssl_method)
    : RefCounted(CheckSubClass()),
      method(ssl_method->method),
      x509_method(ssl_method->x509_method),
      retain_only_sha256_of_client_certs(false),
      quiet_shutdown(false),
      ocsp_stapling_enabled(false),
      signed_cert_timestamps_enabled(false),
      channel_id_enabled(false),
      grease_enabled(false),
      permute_extensions(false),
      allow_unknown_alpn_protos(false),
      false_start_allowed_without_alpn(false),
      handoff(false),
      enable_early_data(false),
      enable_read_ahead(false),
      aes_hw_override(false),
      aes_hw_override_value(false),
      conf_max_version_use_default(true),
      conf_min_version_use_default(true) {
  CRYPTO_MUTEX_init(&lock);
  CRYPTO_new_ex_data(&ex_data);
}

ssl_ctx_st::~ssl_ctx_st() {
  // Free the internal session cache. Note that this calls the caller-supplied
  // remove callback, so we must do it before clearing ex_data. (See ticket
  // [openssl.org #212].)
  SSL_CTX_flush_sessions(this, 0);

  CRYPTO_free_ex_data(&g_ex_data_class_ssl_ctx, this, &ex_data);

  CRYPTO_MUTEX_cleanup(&lock);
  lh_SSL_SESSION_free(sessions);
  sk_SSL_CUSTOM_EXTENSION_pop_free(client_custom_extensions,
                                   SSL_CUSTOM_EXTENSION_free);
  sk_SSL_CUSTOM_EXTENSION_pop_free(server_custom_extensions,
                                   SSL_CUSTOM_EXTENSION_free);
  x509_method->ssl_ctx_free(this);
}

SSL_CTX *SSL_CTX_new(const SSL_METHOD *method) {
  if (method == NULL) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NULL_SSL_METHOD_PASSED);
    return nullptr;
  }

  UniquePtr<SSL_CTX> ret = MakeUnique<SSL_CTX>(method);
  if (!ret) {
    return nullptr;
  }

  ret->cert = MakeUnique<CERT>(method->x509_method);
  ret->sessions = lh_SSL_SESSION_new(ssl_session_hash, ssl_session_cmp);
  ret->client_CA.reset(sk_CRYPTO_BUFFER_new_null());
  if (ret->cert == nullptr || ret->sessions == nullptr ||
      ret->client_CA == nullptr || !ret->x509_method->ssl_ctx_new(ret.get())) {
    return nullptr;
  }

  const bool has_aes_hw = ret->aes_hw_override ? ret->aes_hw_override_value :
                                                 EVP_has_aes_hardware();
  const char *cipher_rule;
  if (has_aes_hw) {
    cipher_rule = TLS13_DEFAULT_CIPHER_LIST_AES_HW;
  } else {
    cipher_rule = TLS13_DEFAULT_CIPHER_LIST_NO_AES_HW;
  }

  if (!SSL_CTX_set_ciphersuites(ret.get(), cipher_rule) ||
      !SSL_CTX_set_strict_cipher_list(ret.get(), SSL_DEFAULT_CIPHER_LIST) ||
      // Lock the SSL_CTX to the specified version, for compatibility with
      // legacy uses of SSL_METHOD.
      !SSL_CTX_set_max_proto_version(ret.get(), method->version) ||
      !SSL_CTX_set_min_proto_version(ret.get(), method->version)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return nullptr;
  }

  // By default, use the method's default min/max version. Make sure to set
  // this after calls to |SSL_CTX_set_{min,max}_proto_version| because those
  // calls modify these values if |method->version| is not 0. We should still
  // defer to the protocol method's default min/max values in that case.
  ret->conf_max_version_use_default = true;
  ret->conf_min_version_use_default = true;
  ret->enable_read_ahead = false;

  return ret.release();
}

int SSL_CTX_up_ref(SSL_CTX *ctx) {
  ctx->UpRefInternal();
  return 1;
}

void SSL_CTX_free(SSL_CTX *ctx) {
  if (ctx != nullptr) {
    ctx->DecRefInternal();
  }
}

ssl_st::ssl_st(SSL_CTX *ctx_arg)
    : method(ctx_arg->method),
      max_send_fragment(ctx_arg->max_send_fragment),
      read_ahead_buffer_size(ctx_arg->read_ahead_buffer_size),
      msg_callback(ctx_arg->msg_callback),
      msg_callback_arg(ctx_arg->msg_callback_arg),
      security_callback(ctx_arg->security_callback),
      security_callback_ex_data(ctx_arg->security_callback_ex_data),
      ctx(UpRef(ctx_arg)),
      session_ctx(UpRef(ctx_arg)),
      options(ctx->options),
      mode(ctx->mode),
      max_cert_list(ctx->max_cert_list),
      server(false),
      quiet_shutdown(ctx->quiet_shutdown),
      enable_early_data(ctx->enable_early_data),
      enable_read_ahead(ctx->enable_read_ahead),
      is_suspended_state(false) {
  CRYPTO_new_ex_data(&ex_data);
}

ssl_st::~ssl_st() {
  CRYPTO_free_ex_data(&g_ex_data_class_ssl, this, &ex_data);
  // |config| refers to |this|, so we must release it earlier.
  config.reset();
  if (method != NULL) {
    method->ssl_free(this);
  }
}

SSL *SSL_new(SSL_CTX *ctx) {
  if (ctx == nullptr) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NULL_SSL_CTX);
    return nullptr;
  }

  UniquePtr<SSL> ssl = MakeUnique<SSL>(ctx);
  if (ssl == nullptr) {
    return nullptr;
  }

  ssl->config = MakeUnique<SSL_CONFIG>(ssl.get());
  if (ssl->config == nullptr) {
    return nullptr;
  }
  ssl->config->conf_min_version = ctx->conf_min_version;
  ssl->config->conf_max_version = ctx->conf_max_version;
  ssl->config->conf_max_version_use_default = ctx->conf_max_version_use_default;
  ssl->config->conf_min_version_use_default = ctx->conf_min_version_use_default;

  ssl->config->cert = ssl_cert_dup(ctx->cert.get());
  if (ssl->config->cert == nullptr) {
    return nullptr;
  }

  ssl->config->verify_mode = ctx->verify_mode;
  ssl->config->verify_callback = ctx->default_verify_callback;
  ssl->config->custom_verify_callback = ctx->custom_verify_callback;
  ssl->config->retain_only_sha256_of_client_certs =
      ctx->retain_only_sha256_of_client_certs;
  ssl->config->permute_extensions = ctx->permute_extensions;
  ssl->config->aes_hw_override = ctx->aes_hw_override;
  ssl->config->aes_hw_override_value = ctx->aes_hw_override_value;

  if (!ssl->config->supported_group_list.CopyFrom(ctx->supported_group_list) ||
      !ssl->config->alpn_client_proto_list.CopyFrom(
          ctx->alpn_client_proto_list) ||
      !ssl->config->verify_sigalgs.CopyFrom(ctx->verify_sigalgs)) {
    return nullptr;
  }

  if (ctx->cipher_list) {
    ssl->config->cipher_list = MakeUnique<SSLCipherPreferenceList>();
    if (!ssl->config->cipher_list ||
        !ssl->config->cipher_list->Init(*ctx->cipher_list.get())) {
      return nullptr;
    }
  }
  if (ctx->tls13_cipher_list) {
    ssl->config->tls13_cipher_list = MakeUnique<SSLCipherPreferenceList>();
    if (!ssl->config->tls13_cipher_list ||
        !ssl->config->tls13_cipher_list->Init(*ctx->tls13_cipher_list.get())) {
      return nullptr;
    }
  }

  if (ctx->psk_identity_hint) {
    ssl->config->psk_identity_hint.reset(
        OPENSSL_strdup(ctx->psk_identity_hint.get()));
    if (ssl->config->psk_identity_hint == nullptr) {
      return nullptr;
    }
  }
  ssl->config->psk_client_callback = ctx->psk_client_callback;
  ssl->config->psk_server_callback = ctx->psk_server_callback;

  ssl->config->channel_id_enabled = ctx->channel_id_enabled;
  ssl->config->channel_id_private = UpRef(ctx->channel_id_private);

  ssl->config->signed_cert_timestamps_enabled =
      ctx->signed_cert_timestamps_enabled;
  ssl->config->ocsp_stapling_enabled = ctx->ocsp_stapling_enabled;
  ssl->config->handoff = ctx->handoff;
  ssl->quic_method = ctx->quic_method;

  if (!ssl->method->ssl_new(ssl.get()) ||
      !ssl->ctx->x509_method->ssl_new(ssl->s3->hs.get())) {
    return nullptr;
  }

  return ssl.release();
}

SSL_CONFIG::SSL_CONFIG(SSL *ssl_arg)
    : ssl(ssl_arg),
      ech_grease_enabled(false),
      signed_cert_timestamps_enabled(false),
      ocsp_stapling_enabled(false),
      channel_id_enabled(false),
      enforce_rsa_key_usage(false),
      retain_only_sha256_of_client_certs(false),
      handoff(false),
      shed_handshake_config(false),
      jdk11_workaround(false),
      quic_use_legacy_codepoint(false),
      permute_extensions(false),
      conf_max_version_use_default(true),
      conf_min_version_use_default(true),
      alps_use_new_codepoint(false),
      check_client_certificate_type(true) {
  assert(ssl);
}

SSL_CONFIG::~SSL_CONFIG() {
  if (ssl->ctx != nullptr) {
    ssl->ctx->x509_method->ssl_config_free(this);
  }
}

void SSL_free(SSL *ssl) { Delete(ssl); }

void SSL_set_connect_state(SSL *ssl) {
  ssl->server = false;
  ssl->do_handshake = ssl_client_handshake;
}

void SSL_set_accept_state(SSL *ssl) {
  ssl->server = true;
  ssl->do_handshake = ssl_server_handshake;
}

void SSL_set0_rbio(SSL *ssl, BIO *rbio) { ssl->rbio.reset(rbio); }

void SSL_set0_wbio(SSL *ssl, BIO *wbio) { ssl->wbio.reset(wbio); }

void SSL_set_bio(SSL *ssl, BIO *rbio, BIO *wbio) {
  // For historical reasons, this function has many different cases in ownership
  // handling.

  // If nothing has changed, do nothing
  if (rbio == SSL_get_rbio(ssl) && wbio == SSL_get_wbio(ssl)) {
    return;
  }

  // If the two arguments are equal, one fewer reference is granted than
  // taken.
  if (rbio != NULL && rbio == wbio) {
    BIO_up_ref(rbio);
  }

  // If only the wbio is changed, adopt only one reference.
  if (rbio == SSL_get_rbio(ssl)) {
    SSL_set0_wbio(ssl, wbio);
    return;
  }

  // There is an asymmetry here for historical reasons. If only the rbio is
  // changed AND the rbio and wbio were originally different, then we only adopt
  // one reference.
  if (wbio == SSL_get_wbio(ssl) && SSL_get_rbio(ssl) != SSL_get_wbio(ssl)) {
    SSL_set0_rbio(ssl, rbio);
    return;
  }

  // Otherwise, adopt both references.
  SSL_set0_rbio(ssl, rbio);
  SSL_set0_wbio(ssl, wbio);
}

BIO *SSL_get_rbio(const SSL *ssl) { return ssl->rbio.get(); }

BIO *SSL_get_wbio(const SSL *ssl) { return ssl->wbio.get(); }

size_t SSL_quic_max_handshake_flight_len(const SSL *ssl,
                                         enum ssl_encryption_level_t level) {
  // Limits flights to 16K by default when there are no large
  // (certificate-carrying) messages.
  static const size_t kDefaultLimit = 16384;

  switch (level) {
    case ssl_encryption_initial:
      return kDefaultLimit;
    case ssl_encryption_early_data:
      // QUIC does not send EndOfEarlyData.
      return 0;
    case ssl_encryption_handshake:
      if (ssl->server) {
        // Servers may receive Certificate message if configured to request
        // client certificates.
        if (!!(ssl->config->verify_mode & SSL_VERIFY_PEER) &&
            ssl->max_cert_list > kDefaultLimit) {
          return ssl->max_cert_list;
        }
      } else {
        // Clients may receive both Certificate message and a CertificateRequest
        // message.
        if (2 * ssl->max_cert_list > kDefaultLimit) {
          return 2 * ssl->max_cert_list;
        }
      }
      return kDefaultLimit;
    case ssl_encryption_application:
      // Note there is not actually a bound on the number of NewSessionTickets
      // one may send in a row. This level may need more involved flow
      // control. See https://github.com/quicwg/base-drafts/issues/1834.
      return kDefaultLimit;
  }

  return 0;
}

enum ssl_encryption_level_t SSL_quic_read_level(const SSL *ssl) {
  return ssl->s3->read_level;
}

enum ssl_encryption_level_t SSL_quic_write_level(const SSL *ssl) {
  return ssl->s3->write_level;
}

int SSL_provide_quic_data(SSL *ssl, enum ssl_encryption_level_t level,
                          const uint8_t *data, size_t len) {
  if (ssl->quic_method == nullptr) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  if (level != ssl->s3->read_level) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_ENCRYPTION_LEVEL_RECEIVED);
    return 0;
  }

  size_t new_len = (ssl->s3->hs_buf ? ssl->s3->hs_buf->length : 0) + len;
  if (new_len < len ||
      new_len > SSL_quic_max_handshake_flight_len(ssl, level)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_EXCESSIVE_MESSAGE_SIZE);
    return 0;
  }

  return tls_append_handshake_data(ssl, MakeConstSpan(data, len));
}

int SSL_do_handshake(SSL *ssl) {
  GUARD_SUSPENDED_STATE(ssl, -1);

  ssl_reset_error_state(ssl);

  if (ssl->do_handshake == NULL) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_CONNECTION_TYPE_NOT_SET);
    return -1;
  }

  if (!SSL_in_init(ssl)) {
    return 1;
  }

  // Run the handshake.
  SSL_HANDSHAKE *hs = ssl->s3->hs.get();

  bool early_return = false;
  int ret = ssl_run_handshake(hs, &early_return);
  ssl_do_info_callback(
      ssl, ssl->server ? SSL_CB_ACCEPT_EXIT : SSL_CB_CONNECT_EXIT, ret);
  if (ret <= 0) {
    return ret;
  }

  // Destroy the handshake object if the handshake has completely finished.
  if (!early_return) {
    // On the client, persist the CA names received in the CertificateRequest
    // message so that |SSL_get_client_CA_list| can return them after the
    // handshake. This eagerly converts to X509_NAMEs since the raw
    // CRYPTO_BUFFERs in |hs->ca_names| will be destroyed with |hs|.
    if (!ssl->server && ssl->s3->hs->ca_names &&
        ssl->ctx->x509_method == &ssl_crypto_x509_method) {
      // Failure is non-fatal: the handshake has already completed, so
      // |SSL_get_client_CA_list| will simply return NULL post-handshake.
      ssl_x509_persist_peer_ca_names(ssl);
    } else if (!ssl->s3->hs->ca_names) {
      // No CertificateRequest in this handshake — clear any stale peer
      // CA names left over from a previous handshake.
      ssl->ctx->x509_method->hs_flush_cached_ca_names(ssl->s3->hs.get());
    }
    ssl->s3->hs.reset();
    ssl_maybe_shed_handshake_config(ssl);
  }

  return 1;
}

int SSL_connect(SSL *ssl) {
  GUARD_SUSPENDED_STATE(ssl, -1);

  if (ssl->do_handshake == NULL) {
    // Not properly initialized yet
    SSL_set_connect_state(ssl);
  }

  return SSL_do_handshake(ssl);
}

int SSL_accept(SSL *ssl) {
  GUARD_SUSPENDED_STATE(ssl, -1);

  if (ssl->do_handshake == NULL) {
    // Not properly initialized yet
    SSL_set_accept_state(ssl);
  }

  return SSL_do_handshake(ssl);
}

static int ssl_do_post_handshake(SSL *ssl, const SSLMessage &msg) {
  if (ssl_protocol_version(ssl) >= TLS1_3_VERSION) {
    return tls13_post_handshake(ssl, msg);
  }

  // Check for renegotiation on the server before parsing to use the correct
  // error. Renegotiation is triggered by a different message for servers.
  if (ssl->server) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NO_RENEGOTIATION);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_NO_RENEGOTIATION);
    return 0;
  }

  if (msg.type != SSL3_MT_HELLO_REQUEST || CBS_len(&msg.body) != 0) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_HELLO_REQUEST);
    return 0;
  }

  if (ssl->renegotiate_mode == ssl_renegotiate_ignore) {
    return 1;  // Ignore the HelloRequest.
  }

  ssl->s3->renegotiate_pending = true;
  if (ssl->renegotiate_mode == ssl_renegotiate_explicit) {
    return 1;  // Handle it later.
  }

  if (!SSL_renegotiate(ssl)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_NO_RENEGOTIATION);
    return 0;
  }

  return 1;
}

int SSL_process_quic_post_handshake(SSL *ssl) {
  ssl_reset_error_state(ssl);

  if (ssl->quic_method == nullptr || (SSL_in_init(ssl) != 0)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  // Replay post-handshake message errors.
  if (!check_read_error(ssl)) {
    return 0;
  }

  // Process any buffered post-handshake messages.
  SSLMessage msg;
  while (ssl->method->get_message(ssl, &msg)) {
    // Handle the post-handshake message and try again.
    if (!ssl_do_post_handshake(ssl, msg)) {
      ssl_set_read_error(ssl);
      return 0;
    }
    ssl->method->next_message(ssl);
  }

  return 1;
}

static int ssl_read_impl(SSL *ssl) {
  ssl_reset_error_state(ssl);

  if (ssl->do_handshake == NULL) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNINITIALIZED);
    return -1;
  }

  // Replay post-handshake message errors.
  if (!check_read_error(ssl)) {
    return -1;
  }

  while (ssl->s3->pending_app_data.empty()) {
    if (ssl->s3->renegotiate_pending) {
      ssl->s3->rwstate = SSL_ERROR_WANT_RENEGOTIATE;
      return -1;
    }

    // Complete the current handshake, if any. False Start will cause
    // |SSL_do_handshake| to return mid-handshake, so this may require multiple
    // iterations.
    while (!ssl_can_read(ssl)) {
      int ret = SSL_do_handshake(ssl);
      if (ret < 0) {
        return ret;
      }
      if (ret == 0) {
        OPENSSL_PUT_ERROR(SSL, SSL_R_SSL_HANDSHAKE_FAILURE);
        return -1;
      }
    }

    // Process any buffered post-handshake messages.
    SSLMessage msg;
    if (ssl->method->get_message(ssl, &msg)) {
      // If we received an interrupt in early read (EndOfEarlyData), loop again
      // for the handshake to process it.
      if (SSL_in_init(ssl)) {
        ssl->s3->hs->can_early_read = false;
        continue;
      }

      // Handle the post-handshake message and try again.
      if (!ssl_do_post_handshake(ssl, msg)) {
        ssl_set_read_error(ssl);
        return -1;
      }
      ssl->method->next_message(ssl);
      continue;  // Loop again. We may have begun a new handshake.
    }

    uint8_t alert = SSL_AD_DECODE_ERROR;
    size_t consumed = 0;
    auto ret = ssl_open_app_data(ssl, &ssl->s3->pending_app_data, &consumed,
                                 &alert, ssl->s3->read_buffer.span());
    bool retry;
    int bio_ret = ssl_handle_open_record(ssl, &retry, ret, consumed, alert);
    if (bio_ret <= 0) {
      return bio_ret;
    }
    if (!retry) {
      assert(!ssl->s3->pending_app_data.empty());
    }
  }

  return 1;
}

int SSL_read_ex(SSL *ssl, void *buf, size_t num, size_t *read_bytes) {
  GUARD_SUSPENDED_STATE(ssl, 0);

  if (num == 0 && read_bytes != nullptr) {
    *read_bytes = 0;
    return 1;
  }
  int ret = SSL_read(ssl, buf, (int)num);
  if (ret <= 0) {
    return 0;
  }
  if (read_bytes != nullptr) {
    *read_bytes = ret;
  }
  return 1;
}

int SSL_read(SSL *ssl, void *buf, int num) {
  GUARD_SUSPENDED_STATE(ssl, -1);

  int ret = SSL_peek(ssl, buf, num);
  if (ret <= 0) {
    return ret;
  }
  // TODO(davidben): In DTLS, should the rest of the record be discarded?  DTLS
  // is not a stream. See https://crbug.com/boringssl/65.
  ssl->s3->pending_app_data =
      ssl->s3->pending_app_data.subspan(static_cast<size_t>(ret));
  if (ssl->s3->pending_app_data.empty()) {
    ssl->s3->read_buffer.DiscardConsumed();
  }
  return ret;
}

int SSL_peek(SSL *ssl, void *buf, int num) {
  GUARD_SUSPENDED_STATE(ssl, -1);

  if (ssl->quic_method != nullptr) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return -1;
  }

  int ret = ssl_read_impl(ssl);
  if (ret <= 0) {
    return ret;
  }
  if (num <= 0) {
    return num;
  }
  size_t todo =
      std::min(ssl->s3->pending_app_data.size(), static_cast<size_t>(num));
  OPENSSL_memcpy(buf, ssl->s3->pending_app_data.data(), todo);
  return static_cast<int>(todo);
}

int SSL_peek_ex(SSL *ssl, void *buf, size_t num, size_t *read_bytes) {
  GUARD_SUSPENDED_STATE(ssl, 0);
  int ret = SSL_peek(ssl, buf, (int)num);
  if (ret <= 0) {
    return 0;
  }
  *read_bytes = ret;
  return 1;
}

int SSL_write(SSL *ssl, const void *buf, int num) {
  GUARD_SUSPENDED_STATE(ssl, -1);

  ssl_reset_error_state(ssl);

  if (ssl->quic_method != nullptr) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return -1;
  }

  if (ssl->do_handshake == NULL) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNINITIALIZED);
    return -1;
  }

  int ret = 0;
  size_t bytes_written = 0;
  bool needs_handshake = false;
  do {
    // If necessary, complete the handshake implicitly.
    if (!ssl_can_write(ssl)) {
      ret = SSL_do_handshake(ssl);
      if (ret < 0) {
        return ret;
      }
      if (ret == 0) {
        OPENSSL_PUT_ERROR(SSL, SSL_R_SSL_HANDSHAKE_FAILURE);
        return -1;
      }
    }

    if (num < 0) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_LENGTH);
      return -1;
    }
    ret = ssl->method->write_app_data(
        ssl, &needs_handshake, &bytes_written,
        MakeConstSpan(static_cast<const uint8_t *>(buf),
                      static_cast<size_t>(num)));
  } while (needs_handshake);
  return ret <= 0 ? ret : static_cast<int>(bytes_written);
}

int SSL_write_ex(SSL *ssl, const void *buf, size_t num, size_t *written) {
  GUARD_SUSPENDED_STATE(ssl, 0);
  if (num == 0 && written != nullptr) {
    *written = 0;
    return 1;
  }
  int ret = SSL_write(ssl, buf, (int)num);
  if (ret <= 0) {
    return 0;
  }
  if (written != nullptr) {
    *written = ret;
  }
  return 1;
}

int SSL_key_update(SSL *ssl, int request_type) {
  GUARD_SUSPENDED_STATE(ssl, 0);

  ssl_reset_error_state(ssl);

  if (ssl->do_handshake == NULL) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNINITIALIZED);
    return 0;
  }

  if (ssl->ctx->quic_method != nullptr) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  if (!ssl->s3->initial_handshake_complete) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_HANDSHAKE_NOT_COMPLETE);
    return 0;
  }

  if (ssl_protocol_version(ssl) < TLS1_3_VERSION) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_SSL_VERSION);
    return 0;
  }

  if (ssl->s3->key_update_pending == SSL_KEY_UPDATE_NONE &&
      !tls13_add_key_update(ssl, request_type)) {
    return 0;
  }

  return 1;
}

int SSL_shutdown(SSL *ssl) {
  GUARD_SUSPENDED_STATE(ssl, -1);

  ssl_reset_error_state(ssl);

  if (ssl->do_handshake == NULL) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNINITIALIZED);
    return -1;
  }

  // If we are in the middle of a handshake, silently succeed. Consumers often
  // call this function before |SSL_free|, whether the handshake succeeded or
  // not. We assume the caller has already handled failed handshakes.
  if (SSL_in_init(ssl)) {
    return 1;
  }

  if (ssl->quiet_shutdown) {
    // Do nothing if configured not to send a close_notify.
    ssl->s3->write_shutdown = ssl_shutdown_close_notify;
    ssl->s3->read_shutdown = ssl_shutdown_close_notify;
    return 1;
  }

  // This function completes in two stages. It sends a close_notify and then it
  // waits for a close_notify to come in. Perform exactly one action and return
  // whether or not it succeeds.

  if (ssl->s3->write_shutdown != ssl_shutdown_close_notify) {
    // Send a close_notify.
    if (ssl_send_alert_impl(ssl, SSL3_AL_WARNING, SSL_AD_CLOSE_NOTIFY) <= 0) {
      return -1;
    }
  } else if (ssl->s3->alert_dispatch) {
    // Finish sending the close_notify.
    if (ssl->method->dispatch_alert(ssl) <= 0) {
      return -1;
    }
  } else if (ssl->s3->read_shutdown != ssl_shutdown_close_notify) {
    if (SSL_is_dtls(ssl)) {
      // Bidirectional shutdown doesn't make sense for an unordered
      // transport. DTLS alerts also aren't delivered reliably, so we may even
      // time out because the peer never received our close_notify. Report to
      // the caller that the channel has fully shut down.
      if (ssl->s3->read_shutdown == ssl_shutdown_error) {
        ERR_restore_state(ssl->s3->read_error.get());
        return -1;
      }
      ssl->s3->read_shutdown = ssl_shutdown_close_notify;
    } else {
      // Process records until an error, close_notify, or application data.
      if (ssl_read_impl(ssl) > 0) {
        // We received some unexpected application data.
        OPENSSL_PUT_ERROR(SSL, SSL_R_APPLICATION_DATA_ON_SHUTDOWN);
        return -1;
      }
      if (ssl->s3->read_shutdown != ssl_shutdown_close_notify) {
        return -1;
      }
    }
  }

  // Return 0 for unidirectional shutdown and 1 for bidirectional shutdown.
  return ssl->s3->read_shutdown == ssl_shutdown_close_notify;
}

int SSL_send_fatal_alert(SSL *ssl, uint8_t alert) {
  if (ssl->s3->alert_dispatch) {
    if (ssl->s3->send_alert[0] != SSL3_AL_FATAL ||
        ssl->s3->send_alert[1] != alert) {
      // We are already attempting to write a different alert.
      OPENSSL_PUT_ERROR(SSL, SSL_R_PROTOCOL_IS_SHUTDOWN);
      return -1;
    }
    return ssl->method->dispatch_alert(ssl);
  }

  return ssl_send_alert_impl(ssl, SSL3_AL_FATAL, alert);
}

int SSL_set_quic_transport_params(SSL *ssl, const uint8_t *params,
                                  size_t params_len) {
  return ssl->config && ssl->config->quic_transport_params.CopyFrom(
                            MakeConstSpan(params, params_len));
}

void SSL_get_peer_quic_transport_params(const SSL *ssl,
                                        const uint8_t **out_params,
                                        size_t *out_params_len) {
  *out_params = ssl->s3->peer_quic_transport_params.data();
  *out_params_len = ssl->s3->peer_quic_transport_params.size();
}

int SSL_set_quic_early_data_context(SSL *ssl, const uint8_t *context,
                                    size_t context_len) {
  return ssl->config && ssl->config->quic_early_data_context.CopyFrom(
                            MakeConstSpan(context, context_len));
}

void SSL_CTX_set_early_data_enabled(SSL_CTX *ctx, int enabled) {
  ctx->enable_early_data = !!enabled;
}

void SSL_set_early_data_enabled(SSL *ssl, int enabled) {
  ssl->enable_early_data = !!enabled;
}

int SSL_in_early_data(const SSL *ssl) {
  if (ssl->s3->hs == NULL) {
    return 0;
  }
  return ssl->s3->hs->in_early_data;
}

int SSL_early_data_accepted(const SSL *ssl) {
  return ssl->s3->early_data_accepted;
}

void SSL_reset_early_data_reject(SSL *ssl) {
  SSL_HANDSHAKE *hs = ssl->s3->hs.get();
  if (hs == NULL || hs->wait != ssl_hs_early_data_rejected) {
    abort();
  }

  hs->wait = ssl_hs_ok;
  hs->in_early_data = false;
  hs->early_session.reset();

  // Discard any unfinished writes from the perspective of |SSL_write|'s
  // retry. The handshake will transparently flush out the pending record
  // (discarded by the server) to keep the framing correct.
  ssl->s3->pending_write = {};
}

enum ssl_early_data_reason_t SSL_get_early_data_reason(const SSL *ssl) {
  return ssl->s3->early_data_reason;
}

const char *SSL_early_data_reason_string(enum ssl_early_data_reason_t reason) {
  switch (reason) {
    case ssl_early_data_unknown:
      return "unknown";
    case ssl_early_data_disabled:
      return "disabled";
    case ssl_early_data_accepted:
      return "accepted";
    case ssl_early_data_protocol_version:
      return "protocol_version";
    case ssl_early_data_peer_declined:
      return "peer_declined";
    case ssl_early_data_no_session_offered:
      return "no_session_offered";
    case ssl_early_data_session_not_resumed:
      return "session_not_resumed";
    case ssl_early_data_unsupported_for_session:
      return "unsupported_for_session";
    case ssl_early_data_hello_retry_request:
      return "hello_retry_request";
    case ssl_early_data_alpn_mismatch:
      return "alpn_mismatch";
    case ssl_early_data_channel_id:
      return "channel_id";
    case ssl_early_data_ticket_age_skew:
      return "ticket_age_skew";
    case ssl_early_data_quic_parameter_mismatch:
      return "quic_parameter_mismatch";
    case ssl_early_data_alps_mismatch:
      return "alps_mismatch";
    case ssl_early_data_unsupported_with_custom_extension:
      return "custom_extension_not_permitted";
  }

  return nullptr;
}

static int bio_retry_reason_to_error(int reason) {
  switch (reason) {
    case BIO_RR_CONNECT:
      return SSL_ERROR_WANT_CONNECT;
    case BIO_RR_ACCEPT:
      return SSL_ERROR_WANT_ACCEPT;
    default:
      return SSL_ERROR_SYSCALL;
  }
}

int SSL_get_error(const SSL *ssl, int ret_code) {
  if (ret_code > 0) {
    return SSL_ERROR_NONE;
  }

  // Make things return SSL_ERROR_SYSCALL when doing SSL_do_handshake etc,
  // where we do encode the error
  uint32_t err = ERR_peek_error();
  if (err != 0) {
    if (ERR_GET_LIB(err) == ERR_LIB_SYS) {
      return SSL_ERROR_SYSCALL;
    }
    return SSL_ERROR_SSL;
  }

  switch (ssl->s3->rwstate) {
    case SSL_ERROR_PENDING_SESSION:
    case SSL_ERROR_PENDING_CERTIFICATE:
    case SSL_ERROR_HANDOFF:
    case SSL_ERROR_HANDBACK:
    case SSL_ERROR_WANT_X509_LOOKUP:
    case SSL_ERROR_WANT_PRIVATE_KEY_OPERATION:
    case SSL_ERROR_PENDING_TICKET:
    case SSL_ERROR_EARLY_DATA_REJECTED:
    case SSL_ERROR_WANT_CERTIFICATE_VERIFY:
    case SSL_ERROR_WANT_RENEGOTIATE:
    case SSL_ERROR_HANDSHAKE_HINTS_READY:
    case SSL_ERROR_ZERO_RETURN:
      return ssl->s3->rwstate;

    case SSL_ERROR_WANT_READ: {
      if (ssl->quic_method) {
        return SSL_ERROR_WANT_READ;
      }
      BIO *bio = SSL_get_rbio(ssl);
      if (BIO_should_read(bio)) {
        return SSL_ERROR_WANT_READ;
      }

      if (BIO_should_write(bio)) {
        // TODO(davidben): OpenSSL historically checked for writes on the read
        // BIO. Can this be removed?
        return SSL_ERROR_WANT_WRITE;
      }

      if (BIO_should_io_special(bio)) {
        return bio_retry_reason_to_error(BIO_get_retry_reason(bio));
      }

      break;
    }

    case SSL_ERROR_WANT_WRITE: {
      BIO *bio = SSL_get_wbio(ssl);
      if (BIO_should_write(bio)) {
        return SSL_ERROR_WANT_WRITE;
      }

      if (BIO_should_read(bio)) {
        // TODO(davidben): OpenSSL historically checked for reads on the write
        // BIO. Can this be removed?
        return SSL_ERROR_WANT_READ;
      }

      if (BIO_should_io_special(bio)) {
        return bio_retry_reason_to_error(BIO_get_retry_reason(bio));
      }

      break;
    }
  }

  return SSL_ERROR_SYSCALL;
}

const char *SSL_error_description(int err) {
  switch (err) {
    case SSL_ERROR_NONE:
      return "NONE";
    case SSL_ERROR_SSL:
      return "SSL";
    case SSL_ERROR_WANT_READ:
      return "WANT_READ";
    case SSL_ERROR_WANT_WRITE:
      return "WANT_WRITE";
    case SSL_ERROR_WANT_X509_LOOKUP:
      return "WANT_X509_LOOKUP";
    case SSL_ERROR_SYSCALL:
      return "SYSCALL";
    case SSL_ERROR_ZERO_RETURN:
      return "ZERO_RETURN";
    case SSL_ERROR_WANT_CONNECT:
      return "WANT_CONNECT";
    case SSL_ERROR_WANT_ACCEPT:
      return "WANT_ACCEPT";
    case SSL_ERROR_PENDING_SESSION:
      return "PENDING_SESSION";
    case SSL_ERROR_PENDING_CERTIFICATE:
      return "PENDING_CERTIFICATE";
    case SSL_ERROR_WANT_PRIVATE_KEY_OPERATION:
      return "WANT_PRIVATE_KEY_OPERATION";
    case SSL_ERROR_PENDING_TICKET:
      return "PENDING_TICKET";
    case SSL_ERROR_EARLY_DATA_REJECTED:
      return "EARLY_DATA_REJECTED";
    case SSL_ERROR_WANT_CERTIFICATE_VERIFY:
      return "WANT_CERTIFICATE_VERIFY";
    case SSL_ERROR_HANDOFF:
      return "HANDOFF";
    case SSL_ERROR_HANDBACK:
      return "HANDBACK";
    case SSL_ERROR_WANT_RENEGOTIATE:
      return "WANT_RENEGOTIATE";
    case SSL_ERROR_HANDSHAKE_HINTS_READY:
      return "HANDSHAKE_HINTS_READY";
    default:
      return nullptr;
  }
}

uint32_t SSL_CTX_set_options(SSL_CTX *ctx, uint32_t options) {
  ctx->options |= options;
  return ctx->options;
}

uint32_t SSL_CTX_clear_options(SSL_CTX *ctx, uint32_t options) {
  ctx->options &= ~options;
  return ctx->options;
}

uint32_t SSL_CTX_get_options(const SSL_CTX *ctx) { return ctx->options; }

uint32_t SSL_set_options(SSL *ssl, uint32_t options) {
  ssl->options |= options;
  return ssl->options;
}

uint32_t SSL_clear_options(SSL *ssl, uint32_t options) {
  ssl->options &= ~options;
  return ssl->options;
}

uint32_t SSL_get_options(const SSL *ssl) { return ssl->options; }

uint32_t SSL_CTX_set_mode(SSL_CTX *ctx, uint32_t mode) {
  ctx->mode |= mode;
  return ctx->mode;
}

uint32_t SSL_CTX_clear_mode(SSL_CTX *ctx, uint32_t mode) {
  ctx->mode &= ~mode;
  return ctx->mode;
}

uint32_t SSL_CTX_get_mode(const SSL_CTX *ctx) { return ctx->mode; }

uint32_t SSL_set_mode(SSL *ssl, uint32_t mode) {
  ssl->mode |= mode;
  return ssl->mode;
}

uint32_t SSL_clear_mode(SSL *ssl, uint32_t mode) {
  ssl->mode &= ~mode;
  return ssl->mode;
}

uint32_t SSL_get_mode(const SSL *ssl) { return ssl->mode; }

void SSL_CTX_set0_buffer_pool(SSL_CTX *ctx, CRYPTO_BUFFER_POOL *pool) {
  ctx->pool = pool;
}

int SSL_get_tls_unique(const SSL *ssl, uint8_t *out, size_t *out_len,
                       size_t max_out) {
  *out_len = 0;
  OPENSSL_memset(out, 0, max_out);

  // tls-unique is not defined for TLS 1.3.
  if (!ssl->s3->initial_handshake_complete ||
      ssl_protocol_version(ssl) >= TLS1_3_VERSION) {
    return 0;
  }

  // The tls-unique value is the first Finished message in the handshake, which
  // is the client's in a full handshake and the server's for a resumption. See
  // https://tools.ietf.org/html/rfc5929#section-3.1.
  const uint8_t *finished = ssl->s3->previous_client_finished;
  size_t finished_len = ssl->s3->previous_client_finished_len;
  if (ssl->session != NULL) {
    // tls-unique is broken for resumed sessions unless EMS is used.
    if (!ssl->session->extended_master_secret) {
      return 0;
    }
    finished = ssl->s3->previous_server_finished;
    finished_len = ssl->s3->previous_server_finished_len;
  }

  *out_len = finished_len;
  if (finished_len > max_out) {
    *out_len = max_out;
  }

  OPENSSL_memcpy(out, finished, *out_len);
  return 1;
}

static int set_session_id_context(CERT *cert, const uint8_t *sid_ctx,
                                  size_t sid_ctx_len) {
  if (sid_ctx_len > sizeof(cert->sid_ctx)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SSL_SESSION_ID_CONTEXT_TOO_LONG);
    return 0;
  }

  static_assert(sizeof(cert->sid_ctx) < 256, "sid_ctx too large");
  cert->sid_ctx_length = (uint8_t)sid_ctx_len;
  OPENSSL_memcpy(cert->sid_ctx, sid_ctx, sid_ctx_len);
  return 1;
}

int SSL_CTX_set_session_id_context(SSL_CTX *ctx, const uint8_t *sid_ctx,
                                   size_t sid_ctx_len) {
  return set_session_id_context(ctx->cert.get(), sid_ctx, sid_ctx_len);
}

int SSL_set_session_id_context(SSL *ssl, const uint8_t *sid_ctx,
                               size_t sid_ctx_len) {
  if (!ssl->config) {
    return 0;
  }
  return set_session_id_context(ssl->config->cert.get(), sid_ctx, sid_ctx_len);
}

const uint8_t *SSL_get0_session_id_context(const SSL *ssl, size_t *out_len) {
  if (!ssl->config) {
    assert(ssl->config);
    *out_len = 0;
    return NULL;
  }
  *out_len = ssl->config->cert->sid_ctx_length;
  return ssl->config->cert->sid_ctx;
}

int SSL_get_fd(const SSL *ssl) { return SSL_get_rfd(ssl); }

int SSL_get_rfd(const SSL *ssl) {
  int ret = -1;
  BIO *b = BIO_find_type(SSL_get_rbio(ssl), BIO_TYPE_DESCRIPTOR);
  if (b != NULL) {
    BIO_get_fd(b, &ret);
  }
  return ret;
}

int SSL_get_wfd(const SSL *ssl) {
  int ret = -1;
  BIO *b = BIO_find_type(SSL_get_wbio(ssl), BIO_TYPE_DESCRIPTOR);
  if (b != NULL) {
    BIO_get_fd(b, &ret);
  }
  return ret;
}

#if !defined(OPENSSL_NO_SOCK)
int SSL_set_fd(SSL *ssl, int fd) {
  BIO *bio = BIO_new(BIO_s_socket());
  if (bio == NULL) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_BUF_LIB);
    return 0;
  }
  BIO_set_fd(bio, fd, BIO_NOCLOSE);
  SSL_set_bio(ssl, bio, bio);
  return 1;
}

int SSL_set_wfd(SSL *ssl, int fd) {
  BIO *rbio = SSL_get_rbio(ssl);
  if (rbio == NULL || BIO_method_type(rbio) != BIO_TYPE_SOCKET ||
      BIO_get_fd(rbio, NULL) != fd) {
    BIO *bio = BIO_new(BIO_s_socket());
    if (bio == NULL) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_BUF_LIB);
      return 0;
    }
    BIO_set_fd(bio, fd, BIO_NOCLOSE);
    SSL_set0_wbio(ssl, bio);
  } else {
    // Copy the rbio over to the wbio.
    BIO_up_ref(rbio);
    SSL_set0_wbio(ssl, rbio);
  }

  return 1;
}

int SSL_set_rfd(SSL *ssl, int fd) {
  BIO *wbio = SSL_get_wbio(ssl);
  if (wbio == NULL || BIO_method_type(wbio) != BIO_TYPE_SOCKET ||
      BIO_get_fd(wbio, NULL) != fd) {
    BIO *bio = BIO_new(BIO_s_socket());
    if (bio == NULL) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_BUF_LIB);
      return 0;
    }
    BIO_set_fd(bio, fd, BIO_NOCLOSE);
    SSL_set0_rbio(ssl, bio);
  } else {
    // Copy the wbio over to the rbio.
    BIO_up_ref(wbio);
    SSL_set0_rbio(ssl, wbio);
  }
  return 1;
}
#endif  // !OPENSSL_NO_SOCK

static size_t copy_finished(void *out, size_t out_len, const uint8_t *in,
                            size_t in_len) {
  if (out_len > in_len) {
    out_len = in_len;
  }
  OPENSSL_memcpy(out, in, out_len);
  return in_len;
}

size_t SSL_get_finished(const SSL *ssl, void *buf, size_t count) {
  if (!ssl->s3->initial_handshake_complete) {
    return 0;
  }

  if (ssl->server) {
    return copy_finished(buf, count, ssl->s3->previous_server_finished,
                         ssl->s3->previous_server_finished_len);
  }

  return copy_finished(buf, count, ssl->s3->previous_client_finished,
                       ssl->s3->previous_client_finished_len);
}

size_t SSL_get_peer_finished(const SSL *ssl, void *buf, size_t count) {
  if (!ssl->s3->initial_handshake_complete) {
    return 0;
  }

  if (ssl->server) {
    return copy_finished(buf, count, ssl->s3->previous_client_finished,
                         ssl->s3->previous_client_finished_len);
  }

  return copy_finished(buf, count, ssl->s3->previous_server_finished,
                       ssl->s3->previous_server_finished_len);
}

int SSL_get_verify_mode(const SSL *ssl) {
  if (!ssl->config) {
    assert(ssl->config);
    return -1;
  }
  return ssl->config->verify_mode;
}

int SSL_get_extms_support(const SSL *ssl) {
  // TLS 1.3 does not require extended master secret and always reports as
  // supporting it.
  if (!ssl->s3->have_version) {
    return 0;
  }
  if (ssl_protocol_version(ssl) >= TLS1_3_VERSION) {
    return 1;
  }

  // If the initial handshake completed, query the established session.
  if (ssl->s3->established_session != NULL) {
    return ssl->s3->established_session->extended_master_secret;
  }

  // Otherwise, query the in-progress handshake.
  if (ssl->s3->hs != NULL) {
    return ssl->s3->hs->extended_master_secret;
  }
  assert(0);
  return 0;
}

int SSL_CTX_get_read_ahead(const SSL_CTX *ctx) {
  GUARD_PTR(ctx);
  return ctx->enable_read_ahead;
}

int SSL_get_read_ahead(const SSL *ssl) {
  GUARD_PTR(ssl);
  return ssl->enable_read_ahead;
}

int SSL_CTX_set_default_read_buffer_len(SSL_CTX *ctx, size_t len) {
  GUARD_PTR(ctx);
  // SSLBUFFER_MAX_CAPACITY(0xffff) is the maximum SSLBuffer supports reading at one time
  if (len > SSLBUFFER_MAX_CAPACITY) {
    len = SSLBUFFER_MAX_CAPACITY;
  }
  // Setting a very small read buffer won't cause issue because the SSLBuffer
  // will always read at least the amount of data specified in the TLS record
  // header
  ctx->read_ahead_buffer_size = len;
  return 1;
}

int SSL_set_default_read_buffer_len(SSL *ssl, size_t len) {
  GUARD_PTR(ssl);
  // SSLBUFFER_MAX_CAPACITY(0xffff) is the maximum SSLBuffer supports reading at one time
  if (len > SSLBUFFER_MAX_CAPACITY) {
    len = SSLBUFFER_MAX_CAPACITY;
  }
  // Setting a very small read buffer won't cause issue because the SSLBuffer
  // will always read at least the amount of data specified in the TLS record
  // header
  ssl->read_ahead_buffer_size = len;
  return 1;
}

int SSL_CTX_set_read_ahead(SSL_CTX *ctx, int yes) {
  GUARD_PTR(ctx);
  if (yes == 0) {
    ctx->enable_read_ahead = false;
    return 1;
  } else if (yes == 1) {
    ctx->enable_read_ahead = true;
    return 1;
  } else {
    return 0;
  }
}

int SSL_set_read_ahead(SSL *ssl, int yes) {
  GUARD_PTR(ssl);
  if (yes == 0) {
    ssl->enable_read_ahead = false;
    return 1;
  } else if (yes == 1) {
    ssl->enable_read_ahead = true;
    return 1;
  } else {
    return 0;
  }}

int SSL_pending(const SSL *ssl) {
  return static_cast<int>(ssl->s3->pending_app_data.size());
}

int SSL_has_pending(const SSL *ssl) {
  return SSL_pending(ssl) != 0 || !ssl->s3->read_buffer.empty();
}

int SSL_CTX_check_private_key(const SSL_CTX *ctx) {
  if (!ssl_cert_check_cert_private_keys_usage(ctx->cert.get())) {
    return 0;
  }
  return ssl_cert_check_private_key(
      ctx->cert.get(),
      ctx->cert->cert_private_keys[ctx->cert->cert_private_key_idx]
          .privatekey.get());
}

int SSL_check_private_key(const SSL *ssl) {
  if (!ssl->config ||
      !ssl_cert_check_cert_private_keys_usage(ssl->config->cert.get())) {
    return 0;
  }
  return ssl_cert_check_private_key(
      ssl->config->cert.get(),
      ssl->config->cert
          ->cert_private_keys[ssl->config->cert->cert_private_key_idx]
          .privatekey.get());
}

long SSL_get_default_timeout(const SSL *ssl) {
  return SSL_DEFAULT_SESSION_TIMEOUT;
}

int SSL_get_key_update_type(const SSL *ssl) {
  return ssl->s3->key_update_pending;
}

int SSL_renegotiate(SSL *ssl) {
  // Caller-initiated renegotiation is not supported.
  if (!ssl->s3->renegotiate_pending) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  if (!ssl_can_renegotiate(ssl)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NO_RENEGOTIATION);
    return 0;
  }

  // We should not have told the caller to release the private key.
  assert(!SSL_can_release_private_key(ssl));
  ssl_update_counter(ssl->session_ctx.get(),
                     ssl->session_ctx->stats.sess_connect_renegotiate, true);

  // Renegotiation is only supported at quiescent points in the application
  // protocol, namely in HTTPS, just before reading the HTTP response.
  // Require the record-layer be idle and avoid complexities of sending a
  // handshake record while an application_data record is being written.
  if (!ssl->s3->write_buffer.empty() ||
      ssl->s3->write_shutdown != ssl_shutdown_none) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NO_RENEGOTIATION);
    return 0;
  }

  // Begin a new handshake.
  if (ssl->s3->hs != nullptr) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return 0;
  }
  ssl->s3->hs = ssl_handshake_new(ssl);
  if (ssl->s3->hs == nullptr) {
    return 0;
  }

  ssl->s3->renegotiate_pending = false;
  ssl->s3->total_renegotiations++;
  return 1;
}

int SSL_renegotiate_pending(SSL *ssl) {
  return SSL_in_init(ssl) && ssl->s3->initial_handshake_complete;
}

int SSL_total_renegotiations(const SSL *ssl) {
  return ssl->s3->total_renegotiations;
}

size_t SSL_CTX_get_max_cert_list(const SSL_CTX *ctx) {
  return ctx->max_cert_list;
}

void SSL_CTX_set_max_cert_list(SSL_CTX *ctx, size_t max_cert_list) {
  if (max_cert_list > kMaxHandshakeSize) {
    max_cert_list = kMaxHandshakeSize;
  }
  ctx->max_cert_list = (uint32_t)max_cert_list;
}

size_t SSL_get_max_cert_list(const SSL *ssl) { return ssl->max_cert_list; }

void SSL_set_max_cert_list(SSL *ssl, size_t max_cert_list) {
  if (max_cert_list > kMaxHandshakeSize) {
    max_cert_list = kMaxHandshakeSize;
  }
  ssl->max_cert_list = (uint32_t)max_cert_list;
}

int SSL_CTX_set_max_send_fragment(SSL_CTX *ctx, size_t max_send_fragment) {
  if (max_send_fragment < MIN_SAFE_FRAGMENT_SIZE) {
    max_send_fragment = MIN_SAFE_FRAGMENT_SIZE;
  }
  if (max_send_fragment > SSL3_RT_MAX_PLAIN_LENGTH) {
    max_send_fragment = SSL3_RT_MAX_PLAIN_LENGTH;
  }
  ctx->max_send_fragment = (uint16_t)max_send_fragment;

  return 1;
}

int SSL_set_max_send_fragment(SSL *ssl, size_t max_send_fragment) {
  if (max_send_fragment < MIN_SAFE_FRAGMENT_SIZE) {
    max_send_fragment = MIN_SAFE_FRAGMENT_SIZE;
  }
  if (max_send_fragment > SSL3_RT_MAX_PLAIN_LENGTH) {
    max_send_fragment = SSL3_RT_MAX_PLAIN_LENGTH;
  }
  ssl->max_send_fragment = (uint16_t)max_send_fragment;

  return 1;
}

int SSL_set_mtu(SSL *ssl, unsigned mtu) {
  if (!SSL_is_dtls(ssl) || mtu < dtls1_min_mtu()) {
    return 0;
  }
  ssl->d1->mtu = mtu;
  return 1;
}

int SSL_get_secure_renegotiation_support(const SSL *ssl) {
  if (!ssl->s3->have_version) {
    return 0;
  }
  return ssl_protocol_version(ssl) >= TLS1_3_VERSION ||
         ssl->s3->send_connection_binding;
}

size_t SSL_CTX_sess_number(const SSL_CTX *ctx) {
  MutexReadLock lock(const_cast<CRYPTO_MUTEX *>(&ctx->lock));
  return lh_SSL_SESSION_num_items(ctx->sessions);
}

unsigned long SSL_CTX_sess_set_cache_size(SSL_CTX *ctx, unsigned long size) {
  unsigned long ret = ctx->session_cache_size;
  ctx->session_cache_size = size;
  return ret;
}

unsigned long SSL_CTX_sess_get_cache_size(const SSL_CTX *ctx) {
  return ctx->session_cache_size;
}

int SSL_CTX_set_session_cache_mode(SSL_CTX *ctx, int mode) {
  int ret = ctx->session_cache_mode;
  ctx->session_cache_mode = mode;
  return ret;
}

int SSL_CTX_get_session_cache_mode(const SSL_CTX *ctx) {
  return ctx->session_cache_mode;
}


int SSL_CTX_get_tlsext_ticket_keys(SSL_CTX *ctx, void *out, size_t len) {
  if (out == NULL) {
    return 48;
  }
  if (len != 48) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_INVALID_TICKET_KEYS_LENGTH);
    return 0;
  }

  // The default ticket keys are initialized lazily. Trigger a key
  // rotation to initialize them.
  if (!ssl_ctx_rotate_ticket_encryption_key(ctx)) {
    return 0;
  }

  uint8_t *out_bytes = reinterpret_cast<uint8_t *>(out);
  MutexReadLock lock(&ctx->lock);
  OPENSSL_memcpy(out_bytes, ctx->ticket_key_current->name, 16);
  OPENSSL_memcpy(out_bytes + 16, ctx->ticket_key_current->hmac_key, 16);
  OPENSSL_memcpy(out_bytes + 32, ctx->ticket_key_current->aes_key, 16);
  return 1;
}

int SSL_CTX_set_tlsext_ticket_keys(SSL_CTX *ctx, const void *in, size_t len) {
  if (in == NULL) {
    return 48;
  }
  if (len != 48) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_INVALID_TICKET_KEYS_LENGTH);
    return 0;
  }
  auto key = MakeUnique<TicketKey>();
  if (!key) {
    return 0;
  }
  const uint8_t *in_bytes = reinterpret_cast<const uint8_t *>(in);
  OPENSSL_memcpy(key->name, in_bytes, 16);
  OPENSSL_memcpy(key->hmac_key, in_bytes + 16, 16);
  OPENSSL_memcpy(key->aes_key, in_bytes + 32, 16);
  // Disable automatic key rotation for manually-configured keys. This is now
  // the caller's responsibility.
  key->next_rotation_tv_sec = 0;
  ctx->ticket_key_current = std::move(key);
  ctx->ticket_key_prev.reset();
  return 1;
}

int SSL_CTX_set_tlsext_ticket_key_cb(
    SSL_CTX *ctx,
    int (*callback)(SSL *ssl, uint8_t *key_name, uint8_t *iv,
                    EVP_CIPHER_CTX *ctx, HMAC_CTX *hmac_ctx, int encrypt)) {
  ctx->ticket_key_cb = callback;
  return 1;
}

static bool ssl_nids_to_group_ids(Array<uint16_t> *out_group_ids,
                                  Span<const int> nids) {
  Array<uint16_t> group_ids;
  if (!group_ids.Init(nids.size())) {
    return false;
  }

  for (size_t i = 0; i < nids.size(); i++) {
    if (!ssl_nid_to_group_id(&group_ids[i], nids[i])) {
      return false;
    }
  }

  *out_group_ids = std::move(group_ids);
  return true;
}

int SSL_CTX_set1_groups(SSL_CTX *ctx, const int *groups, size_t num_groups) {
  return ssl_nids_to_group_ids(&ctx->supported_group_list,
                               MakeConstSpan(groups, num_groups));
}

int SSL_set1_groups(SSL *ssl, const int *groups, size_t num_groups) {
  if (!ssl->config) {
    return 0;
  }
  return ssl_nids_to_group_ids(&ssl->config->supported_group_list,
                               MakeConstSpan(groups, num_groups));
}

static bool ssl_str_to_group_ids(Array<uint16_t> *out_group_ids,
                                 const char *str) {
  // Count the number of groups in the list.
  size_t count = 0;
  const char *ptr = str, *col;
  do {
    col = strchr(ptr, ':');
    count++;
    if (col) {
      ptr = col + 1;
    }
  } while (col);

  Array<uint16_t> group_ids;
  if (!group_ids.Init(count)) {
    return false;
  }

  size_t i = 0;
  ptr = str;
  do {
    col = strchr(ptr, ':');
    if (!ssl_name_to_group_id(&group_ids[i++], ptr,
                              col ? (size_t)(col - ptr) : strlen(ptr))) {
      return false;
    }
    if (col) {
      ptr = col + 1;
    }
  } while (col);

  assert(i == count);
  *out_group_ids = std::move(group_ids);
  return true;
}

int SSL_CTX_set1_groups_list(SSL_CTX *ctx, const char *groups) {
  return ssl_str_to_group_ids(&ctx->supported_group_list, groups);
}

int SSL_set1_groups_list(SSL *ssl, const char *groups) {
  if (!ssl->config) {
    return 0;
  }
  return ssl_str_to_group_ids(&ssl->config->supported_group_list, groups);
}

uint16_t SSL_get_group_id(const SSL *ssl) {
  SSL_SESSION *session = SSL_get_session(ssl);
  if (session == NULL) {
    return 0;
  }

  return session->group_id;
}

int SSL_get_negotiated_group(const SSL *ssl) {
  uint16_t group_id = SSL_get_group_id(ssl);
  if (group_id == 0) {
    return NID_undef;
  }
  return ssl_group_id_to_nid(group_id);
}

int SSL_CTX_set_tmp_dh(SSL_CTX *ctx, const DH *dh) { return 1; }

int SSL_set_tmp_dh(SSL *ssl, const DH *dh) { return 1; }

STACK_OF(SSL_CIPHER) *SSL_CTX_get_ciphers(const SSL_CTX *ctx) {
  return ctx->cipher_list->ciphers.get();
}

int SSL_CTX_cipher_in_group(const SSL_CTX *ctx, size_t i) {
  if (i >= sk_SSL_CIPHER_num(ctx->cipher_list->ciphers.get())) {
    return 0;
  }
  return ctx->cipher_list->in_group_flags[i];
}

STACK_OF(SSL_CIPHER) *SSL_get_ciphers(const SSL *ssl) {
  if (ssl == NULL) {
    return NULL;
  }

  if (ssl->config && ssl->config->cipher_list) {
    return ssl->config->cipher_list->ciphers.get();
  }

  return ssl->ctx->cipher_list->ciphers.get();
}

const char *SSL_get_cipher_list(const SSL *ssl, int n) {
  if (ssl == NULL) {
    return NULL;
  }

  STACK_OF(SSL_CIPHER) *sk = SSL_get_ciphers(ssl);
  if (sk == NULL || n < 0 || (size_t)n >= sk_SSL_CIPHER_num(sk)) {
    return NULL;
  }

  const SSL_CIPHER *c = sk_SSL_CIPHER_value(sk, n);
  if (c == NULL) {
    return NULL;
  }

  return c->name;
}

int SSL_CTX_set_cipher_list(SSL_CTX *ctx, const char *str) {
  const bool has_aes_hw = ctx->aes_hw_override ? ctx->aes_hw_override_value
                                               : EVP_has_aes_hardware();
  if (!ssl_create_cipher_list(&ctx->cipher_list, has_aes_hw, str,
                                false /* not strict */,
                                false /* don't configure TLSv1.3 ciphers */)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NO_CIPHER_MATCH);
    return 0;
  }

  return update_cipher_list(ctx->cipher_list, ctx->cipher_list, ctx->tls13_cipher_list);
}

int SSL_CTX_set_strict_cipher_list(SSL_CTX *ctx, const char *str) {
  const bool has_aes_hw = ctx->aes_hw_override ? ctx->aes_hw_override_value
                                               : EVP_has_aes_hardware();
  if (!ssl_create_cipher_list(&ctx->cipher_list, has_aes_hw, str,
                                true /* strict */,
                                false /* don't configure TLSv1.3 ciphers */)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NO_CIPHER_MATCH);
    return 0;
  }

  return update_cipher_list(ctx->cipher_list, ctx->cipher_list, ctx->tls13_cipher_list);
}

int SSL_set_cipher_list(SSL *ssl, const char *str) {
  if (!ssl->config) {
    return 0;
  }
  const bool has_aes_hw = ssl->config->aes_hw_override
                              ? ssl->config->aes_hw_override_value
                              : EVP_has_aes_hardware();
  if (!ssl_create_cipher_list(&ssl->config->cipher_list, has_aes_hw, str,
                                false /* not strict */,
                                false /* don't configure TLSv1.3 ciphers */)) {
    return 0;
  }

  UniquePtr<SSLCipherPreferenceList> &tls13_ciphers = ssl->config->tls13_cipher_list ? ssl->config->tls13_cipher_list :
                                        ssl->ctx->tls13_cipher_list;

  return update_cipher_list(ssl->config->cipher_list, ssl->config->cipher_list, tls13_ciphers);
}

int SSL_CTX_set_ciphersuites(SSL_CTX *ctx, const char *str) {
  const bool has_aes_hw = ctx->aes_hw_override ? ctx->aes_hw_override_value
                                               : EVP_has_aes_hardware();

  if (!ssl_create_cipher_list(&ctx->tls13_cipher_list, has_aes_hw, str,
                                false /* not strict */,
                                true /* only configure TLSv1.3 ciphers */)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NO_CIPHER_MATCH);
    return 0;
  }

  return update_cipher_list(ctx->cipher_list, ctx->cipher_list, ctx->tls13_cipher_list);
}

int SSL_set_ciphersuites(SSL *ssl, const char *str) {
  if (!ssl->config) {
    return 0;
  }
  const bool has_aes_hw = ssl->config->aes_hw_override
                              ? ssl->config->aes_hw_override_value
                              : EVP_has_aes_hardware();
  if (!ssl_create_cipher_list(&ssl->config->tls13_cipher_list,
                                has_aes_hw, str, false /* not strict */,
                                true /* configure TLSv1.3 ciphers */)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NO_CIPHER_MATCH);
    return 0;
  }

  UniquePtr<SSLCipherPreferenceList> &ciphers = ssl->config->cipher_list ? ssl->config->cipher_list :
                                          ssl->ctx->cipher_list;

  return update_cipher_list(ssl->config->cipher_list, ciphers, ssl->config->tls13_cipher_list);
}

int SSL_set_strict_cipher_list(SSL *ssl, const char *str) {
  if (!ssl->config) {
    return 0;
  }
  const bool has_aes_hw = ssl->config->aes_hw_override
                              ? ssl->config->aes_hw_override_value
                              : EVP_has_aes_hardware();
  if (!ssl_create_cipher_list(&ssl->config->cipher_list,
                                has_aes_hw, str, true /* strict */,
                                false /* don't configure TLSv1.3 ciphers */)) {
    return 0;
  }

  UniquePtr<SSLCipherPreferenceList> &tls13_ciphers = ssl->config->tls13_cipher_list ? ssl->config->tls13_cipher_list :
                                        ssl->ctx->tls13_cipher_list;

  return update_cipher_list(ssl->config->cipher_list, ssl->config->cipher_list, tls13_ciphers);
}

const char *SSL_get_servername(const SSL *ssl, const int type) {
  if (type != TLSEXT_NAMETYPE_host_name) {
    return NULL;
  }

  // Historically, |SSL_get_servername| was also the configuration getter
  // corresponding to |SSL_set_tlsext_host_name|.
  if (ssl->hostname != nullptr) {
    return ssl->hostname.get();
  }

  return ssl->s3->hostname.get();
}

int SSL_get_servername_type(const SSL *ssl) {
  if (SSL_get_servername(ssl, TLSEXT_NAMETYPE_host_name) == NULL) {
    return -1;
  }
  return TLSEXT_NAMETYPE_host_name;
}

void SSL_CTX_set_custom_verify(
    SSL_CTX *ctx, int mode,
    enum ssl_verify_result_t (*callback)(SSL *ssl, uint8_t *out_alert)) {
  ctx->verify_mode = mode;
  ctx->custom_verify_callback = callback;
}

void SSL_set_custom_verify(
    SSL *ssl, int mode,
    enum ssl_verify_result_t (*callback)(SSL *ssl, uint8_t *out_alert)) {
  if (!ssl->config) {
    return;
  }
  ssl->config->verify_mode = mode;
  ssl->config->custom_verify_callback = callback;
}

void SSL_CTX_enable_signed_cert_timestamps(SSL_CTX *ctx) {
  ctx->signed_cert_timestamps_enabled = true;
}

void SSL_enable_signed_cert_timestamps(SSL *ssl) {
  if (!ssl->config) {
    return;
  }
  ssl->config->signed_cert_timestamps_enabled = true;
}

void SSL_CTX_enable_ocsp_stapling(SSL_CTX *ctx) {
  ctx->ocsp_stapling_enabled = true;
}

void SSL_enable_ocsp_stapling(SSL *ssl) {
  if (!ssl->config) {
    return;
  }
  ssl->config->ocsp_stapling_enabled = true;
}

void SSL_get0_signed_cert_timestamp_list(const SSL *ssl, const uint8_t **out,
                                         size_t *out_len) {
  SSL_SESSION *session = SSL_get_session(ssl);
  if (ssl->server || !session || !session->signed_cert_timestamp_list) {
    *out_len = 0;
    *out = NULL;
    return;
  }

  *out = CRYPTO_BUFFER_data(session->signed_cert_timestamp_list.get());
  *out_len = CRYPTO_BUFFER_len(session->signed_cert_timestamp_list.get());
}

void SSL_get0_ocsp_response(const SSL *ssl, const uint8_t **out,
                            size_t *out_len) {
  SSL_SESSION *session = SSL_get_session(ssl);
  if (ssl->server || !session || !session->ocsp_response) {
    *out_len = 0;
    *out = NULL;
    return;
  }

  *out = CRYPTO_BUFFER_data(session->ocsp_response.get());
  *out_len = CRYPTO_BUFFER_len(session->ocsp_response.get());
}

int SSL_set_tlsext_host_name(SSL *ssl, const char *name) {
  ssl->hostname.reset();
  if (name == nullptr) {
    return 1;
  }

  size_t len = strlen(name);
  if (len == 0 || len > TLSEXT_MAXLEN_host_name) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SSL3_EXT_INVALID_SERVERNAME);
    return 0;
  }
  ssl->hostname.reset(OPENSSL_strdup(name));
  if (ssl->hostname == nullptr) {
    return 0;
  }
  return 1;
}

int SSL_CTX_set_tlsext_servername_callback(
    SSL_CTX *ctx, int (*callback)(SSL *ssl, int *out_alert, void *arg)) {
  ctx->servername_callback = callback;
  return 1;
}

int SSL_CTX_set_tlsext_servername_arg(SSL_CTX *ctx, void *arg) {
  ctx->servername_arg = arg;
  return 1;
}

int SSL_select_next_proto(uint8_t **out, uint8_t *out_len, const uint8_t *peer,
                          unsigned peer_len, const uint8_t *supported,
                          unsigned supported_len) {
  *out = nullptr;
  *out_len = 0;

  // Both |peer| and |supported| must be valid protocol lists, but |peer| may be
  // empty in NPN.
  auto peer_span = MakeConstSpan(peer, peer_len);
  auto supported_span = MakeConstSpan(supported, supported_len);
  if ((!peer_span.empty() && !ssl_is_valid_alpn_list(peer_span)) ||
      !ssl_is_valid_alpn_list(supported_span)) {
    return OPENSSL_NPN_NO_OVERLAP;
  }

  // For each protocol in peer preference order, see if we support it.
  CBS cbs = peer_span, proto;
  while (CBS_len(&cbs) != 0) {
    if (!CBS_get_u8_length_prefixed(&cbs, &proto) || CBS_len(&proto) == 0) {
      return OPENSSL_NPN_NO_OVERLAP;
    }

    if (ssl_alpn_list_contains_protocol(MakeConstSpan(supported, supported_len),
                                        proto)) {
      // This function is not const-correct for compatibility with existing
      // callers.
      *out = const_cast<uint8_t *>(CBS_data(&proto));
      // A u8 length prefix will fit in |uint8_t|.
      *out_len = static_cast<uint8_t>(CBS_len(&proto));
      return OPENSSL_NPN_NEGOTIATED;
    }
  }

  // There's no overlap between our protocols and the peer's list. In ALPN, the
  // caller is expected to fail the connection with no_application_protocol. In
  // NPN, the caller is expected to opportunistically select the first protocol.
  // See draft-agl-tls-nextprotoneg-04, section 6.
  cbs = supported_span;
  if (!CBS_get_u8_length_prefixed(&cbs, &proto) || CBS_len(&proto) == 0) {
    return OPENSSL_NPN_NO_OVERLAP;
  }

  // See above.
  *out = const_cast<uint8_t *>(CBS_data(&proto));
  *out_len = static_cast<uint8_t>(CBS_len(&proto));
  return OPENSSL_NPN_NO_OVERLAP;
}

void SSL_get0_next_proto_negotiated(const SSL *ssl, const uint8_t **out_data,
                                    unsigned *out_len) {
  // NPN protocols have one-byte lengths, so they must fit in |unsigned|.
  assert(ssl->s3->next_proto_negotiated.size() <= UINT_MAX);
  *out_data = ssl->s3->next_proto_negotiated.data();
  *out_len = static_cast<unsigned>(ssl->s3->next_proto_negotiated.size());
}

void SSL_CTX_set_next_protos_advertised_cb(
    SSL_CTX *ctx,
    int (*cb)(SSL *ssl, const uint8_t **out, unsigned *out_len, void *arg),
    void *arg) {
  ctx->next_protos_advertised_cb = cb;
  ctx->next_protos_advertised_cb_arg = arg;
}

void SSL_CTX_set_next_proto_select_cb(SSL_CTX *ctx,
                                      int (*cb)(SSL *ssl, uint8_t **out,
                                                uint8_t *out_len,
                                                const uint8_t *in,
                                                unsigned in_len, void *arg),
                                      void *arg) {
  ctx->next_proto_select_cb = cb;
  ctx->next_proto_select_cb_arg = arg;
}

int SSL_CTX_set_alpn_protos(SSL_CTX *ctx, const uint8_t *protos,
                            size_t protos_len) {
  // Note this function's return value is backwards.
  auto span = MakeConstSpan(protos, protos_len);
  if (!span.empty() && !ssl_is_valid_alpn_list(span)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_INVALID_ALPN_PROTOCOL_LIST);
    return 1;
  }
  return ctx->alpn_client_proto_list.CopyFrom(span) ? 0 : 1;
}

int SSL_set_alpn_protos(SSL *ssl, const uint8_t *protos, size_t protos_len) {
  // Note this function's return value is backwards.
  if (!ssl->config) {
    return 1;
  }
  auto span = MakeConstSpan(protos, protos_len);
  if (!span.empty() && !ssl_is_valid_alpn_list(span)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_INVALID_ALPN_PROTOCOL_LIST);
    return 1;
  }
  return ssl->config->alpn_client_proto_list.CopyFrom(span) ? 0 : 1;
}

void SSL_CTX_set_alpn_select_cb(SSL_CTX *ctx,
                                int (*cb)(SSL *ssl, const uint8_t **out,
                                          uint8_t *out_len, const uint8_t *in,
                                          unsigned in_len, void *arg),
                                void *arg) {
  ctx->alpn_select_cb = cb;
  ctx->alpn_select_cb_arg = arg;
}

void SSL_get0_alpn_selected(const SSL *ssl, const uint8_t **out_data,
                            unsigned *out_len) {
  Span<const uint8_t> protocol;
  if (SSL_in_early_data(ssl) && !ssl->server) {
    protocol = ssl->s3->hs->early_session->early_alpn;
  } else {
    protocol = ssl->s3->alpn_selected;
  }
  // ALPN protocols have one-byte lengths, so they must fit in |unsigned|.
  assert(protocol.size() < UINT_MAX);
  *out_data = protocol.data();
  *out_len = static_cast<unsigned>(protocol.size());
}

void SSL_CTX_set_allow_unknown_alpn_protos(SSL_CTX *ctx, int enabled) {
  ctx->allow_unknown_alpn_protos = !!enabled;
}

int SSL_add_application_settings(SSL *ssl, const uint8_t *proto,
                                 size_t proto_len, const uint8_t *settings,
                                 size_t settings_len) {
  if (!ssl->config) {
    return 0;
  }
  ALPSConfig config;
  if (!config.protocol.CopyFrom(MakeConstSpan(proto, proto_len)) ||
      !config.settings.CopyFrom(MakeConstSpan(settings, settings_len)) ||
      !ssl->config->alps_configs.Push(std::move(config))) {
    return 0;
  }
  return 1;
}

void SSL_get0_peer_application_settings(const SSL *ssl,
                                        const uint8_t **out_data,
                                        size_t *out_len) {
  const SSL_SESSION *session = SSL_get_session(ssl);
  Span<const uint8_t> settings =
      session ? session->peer_application_settings : Span<const uint8_t>();
  *out_data = settings.data();
  *out_len = settings.size();
}

int SSL_has_application_settings(const SSL *ssl) {
  const SSL_SESSION *session = SSL_get_session(ssl);
  return session && session->has_application_settings;
}

void SSL_set_alps_use_new_codepoint(SSL *ssl, int use_new) {
  if (!ssl->config) {
    return;
  }
  ssl->config->alps_use_new_codepoint = !!use_new;
}

int SSL_CTX_add_cert_compression_alg(SSL_CTX *ctx, uint16_t alg_id,
                                     ssl_cert_compression_func_t compress,
                                     ssl_cert_decompression_func_t decompress) {
  assert(compress != nullptr || decompress != nullptr);

  for (const auto &alg : ctx->cert_compression_algs) {
    if (alg.alg_id == alg_id) {
      return 0;
    }
  }

  CertCompressionAlg alg;
  alg.alg_id = alg_id;
  alg.compress = compress;
  alg.decompress = decompress;
  return ctx->cert_compression_algs.Push(alg);
}

void SSL_CTX_set_tls_channel_id_enabled(SSL_CTX *ctx, int enabled) {
  ctx->channel_id_enabled = !!enabled;
}

int SSL_CTX_enable_tls_channel_id(SSL_CTX *ctx) {
  SSL_CTX_set_tls_channel_id_enabled(ctx, 1);
  return 1;
}

void SSL_set_tls_channel_id_enabled(SSL *ssl, int enabled) {
  if (!ssl->config) {
    return;
  }
  ssl->config->channel_id_enabled = !!enabled;
}

int SSL_enable_tls_channel_id(SSL *ssl) {
  SSL_set_tls_channel_id_enabled(ssl, 1);
  return 1;
}

static int is_p256_key(EVP_PKEY *private_key) {
  const EC_KEY *ec_key = EVP_PKEY_get0_EC_KEY(private_key);
  return ec_key != NULL && EC_GROUP_get_curve_name(EC_KEY_get0_group(ec_key)) ==
                               NID_X9_62_prime256v1;
}

int SSL_CTX_set1_tls_channel_id(SSL_CTX *ctx, EVP_PKEY *private_key) {
  if (!is_p256_key(private_key)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_CHANNEL_ID_NOT_P256);
    return 0;
  }

  ctx->channel_id_private = UpRef(private_key);
  return 1;
}

int SSL_set1_tls_channel_id(SSL *ssl, EVP_PKEY *private_key) {
  if (!ssl->config) {
    return 0;
  }
  if (!is_p256_key(private_key)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_CHANNEL_ID_NOT_P256);
    return 0;
  }

  ssl->config->channel_id_private = UpRef(private_key);
  return 1;
}

size_t SSL_get_tls_channel_id(SSL *ssl, uint8_t *out, size_t max_out) {
  if (!ssl->s3->channel_id_valid) {
    return 0;
  }
  OPENSSL_memcpy(out, ssl->s3->channel_id, (max_out < 64) ? max_out : 64);
  return 64;
}

size_t SSL_get0_certificate_types(const SSL *ssl, const uint8_t **out_types) {
  Span<const uint8_t> types;
  if (!ssl->server && ssl->s3->hs != nullptr) {
    types = ssl->s3->hs->certificate_types;
  }
  *out_types = types.data();
  return types.size();
}

size_t SSL_get0_peer_verify_algorithms(const SSL *ssl,
                                       const uint16_t **out_sigalgs) {
  Span<const uint16_t> sigalgs;
  if (ssl->s3->hs != nullptr) {
    sigalgs = ssl->s3->hs->peer_sigalgs;
  }
  *out_sigalgs = sigalgs.data();
  return sigalgs.size();
}

size_t SSL_get0_peer_delegation_algorithms(const SSL *ssl,
                                           const uint16_t **out_sigalgs) {
  Span<const uint16_t> sigalgs;
  if (ssl->s3->hs != nullptr) {
    sigalgs = ssl->s3->hs->peer_delegated_credential_sigalgs;
  }
  *out_sigalgs = sigalgs.data();
  return sigalgs.size();
}

EVP_PKEY *SSL_get_privatekey(const SSL *ssl) {
  if (!ssl->config) {
    assert(ssl->config);
    return NULL;
  }
  if (ssl_cert_check_cert_private_keys_usage(ssl->config->cert.get())) {
    return ssl->config->cert
        ->cert_private_keys[ssl->config->cert->cert_private_key_idx]
        .privatekey.get();
  }

  return NULL;
}

EVP_PKEY *SSL_CTX_get0_privatekey(const SSL_CTX *ctx) {
  if (ssl_cert_check_cert_private_keys_usage(ctx->cert.get())) {
    return ctx->cert->cert_private_keys[ctx->cert->cert_private_key_idx]
        .privatekey.get();
  }

  return NULL;
}

const SSL_CIPHER *SSL_get_current_cipher(const SSL *ssl) {
  const SSL_SESSION *session = SSL_get_session(ssl);
  return session == nullptr ? nullptr : session->cipher;
}

STACK_OF(SSL_CIPHER) *SSL_get_client_ciphers(const SSL *ssl) {
  if (ssl == NULL || ssl->s3 == NULL) {
    return NULL;
  }
  return ssl->client_cipher_suites.get();
}

int SSL_session_reused(const SSL *ssl) {
  return ssl->s3->session_reused || SSL_in_early_data(ssl);
}

const COMP_METHOD *SSL_get_current_compression(SSL *ssl) { return NULL; }

const COMP_METHOD *SSL_get_current_expansion(SSL *ssl) { return NULL; }

int SSL_get_peer_tmp_key(SSL *ssl, EVP_PKEY **out_key) {
  GUARD_PTR(ssl);
  GUARD_PTR(ssl->s3);
  GUARD_PTR(out_key);

  SSL_SESSION *session = SSL_get_session(ssl);
  uint16_t nid;
  if (!session || !ssl_group_id_to_nid(&nid, session->group_id)) {
    return 0;
  }
  bssl::UniquePtr<EVP_PKEY> ret(EVP_PKEY_new());
  if (!ret) {
    return 0;
  }

  // Assign key type based on the session's key exchange |nid|.
  if (nid == EVP_PKEY_X25519) {
    if (!EVP_PKEY_set_type(ret.get(), EVP_PKEY_X25519)) {
      return 0;
    }
  } else {
    EC_KEY *key = EC_KEY_new_by_curve_name(nid);
    if (!key) {
      // We only support ECDHE for temporary keys, so fail if an unrecognized
      // key exchange is used.
      OPENSSL_PUT_ERROR(SSL, SSL_R_UNKNOWN_KEY_EXCHANGE_TYPE);
      return 0;
    }
    if (!EVP_PKEY_assign_EC_KEY(ret.get(), key)) {
      return 0;
    }
  }

  if (!EVP_PKEY_set1_tls_encodedpoint(ret.get(), ssl->s3->peer_key.data(),
                                      ssl->s3->peer_key.size())) {
    return 0;
  }
  EVP_PKEY_up_ref(ret.get());
  *out_key = ret.get();
  return 1;
}

int SSL_get_server_tmp_key(SSL *ssl, EVP_PKEY **out_key) {
  return SSL_get_peer_tmp_key(ssl, out_key);
}

void SSL_CTX_set_quiet_shutdown(SSL_CTX *ctx, int mode) {
  ctx->quiet_shutdown = (mode != 0);
}

int SSL_CTX_get_quiet_shutdown(const SSL_CTX *ctx) {
  return ctx->quiet_shutdown;
}

void SSL_set_quiet_shutdown(SSL *ssl, int mode) {
  ssl->quiet_shutdown = (mode != 0);
}

int SSL_get_quiet_shutdown(const SSL *ssl) { return ssl->quiet_shutdown; }

void SSL_set_shutdown(SSL *ssl, int mode) {
  if (mode & SSL_RECEIVED_SHUTDOWN &&
      ssl->s3->read_shutdown == ssl_shutdown_none) {
    ssl->s3->read_shutdown = ssl_shutdown_close_notify;
  }

  if (mode & SSL_SENT_SHUTDOWN &&
      ssl->s3->write_shutdown == ssl_shutdown_none) {
    ssl->s3->write_shutdown = ssl_shutdown_close_notify;
  }
}

int SSL_get_shutdown(const SSL *ssl) {
  int ret = 0;
  if (ssl->s3->read_shutdown != ssl_shutdown_none) {
    // Historically, OpenSSL set |SSL_RECEIVED_SHUTDOWN| on both close_notify
    // and fatal alert.
    ret |= SSL_RECEIVED_SHUTDOWN;
  }
  if (ssl->s3->write_shutdown == ssl_shutdown_close_notify) {
    // Historically, OpenSSL set |SSL_SENT_SHUTDOWN| on only close_notify.
    ret |= SSL_SENT_SHUTDOWN;
  }
  return ret;
}

SSL_CTX *SSL_get_SSL_CTX(const SSL *ssl) { return ssl->ctx.get(); }

SSL_CTX *SSL_set_SSL_CTX(SSL *ssl, SSL_CTX *ctx) {
  if (!ssl->config) {
    return NULL;
  }

  if (!ctx || ssl->ctx.get() == ctx) {
    return ssl->ctx.get();
  }

  // One cannot change the X.509 callbacks during a connection.
  if (ssl->ctx->x509_method != ctx->x509_method) {
    assert(0);
    return NULL;
  }

  UniquePtr<CERT> new_cert = ssl_cert_dup(ctx->cert.get());
  if (!new_cert) {
    return nullptr;
  }

  ssl->config->cert = std::move(new_cert);
  ssl->ctx = UpRef(ctx);
  ssl->enable_early_data = ssl->ctx->enable_early_data;

  return ssl->ctx.get();
}

void SSL_set_info_callback(SSL *ssl,
                           void (*cb)(const SSL *ssl, int type, int value)) {
  ssl->info_callback = cb;
}

void (*SSL_get_info_callback(const SSL *ssl))(const SSL *ssl, int type,
                                              int value) {
  return ssl->info_callback;
}

int SSL_state(const SSL *ssl) {
  return SSL_in_init(ssl) ? SSL_ST_INIT : SSL_ST_OK;
}

void SSL_set_state(SSL *ssl, int state) {}

char *SSL_get_shared_ciphers(const SSL *ssl, char *buf, int len) {
  if (len <= 0) {
    return NULL;
  }
  buf[0] = '\0';
  return buf;
}

int SSL_get_shared_sigalgs(SSL *ssl, int idx, int *psign, int *phash,
                           int *psignandhash, uint8_t *rsig, uint8_t *rhash) {
  return 0;
}

int SSL_CTX_set_quic_method(SSL_CTX *ctx, const SSL_QUIC_METHOD *quic_method) {
  if (ctx->method->is_dtls) {
    return 0;
  }
  ctx->quic_method = quic_method;
  return 1;
}

int SSL_set_quic_method(SSL *ssl, const SSL_QUIC_METHOD *quic_method) {
  if (ssl->method->is_dtls) {
    return 0;
  }
  ssl->quic_method = quic_method;
  return 1;
}

int SSL_get_ex_new_index(long argl, void *argp, CRYPTO_EX_unused *unused,
                         CRYPTO_EX_dup *dup_unused, CRYPTO_EX_free *free_func) {
  int index;
  if (!CRYPTO_get_ex_new_index(&g_ex_data_class_ssl, &index, argl, argp,
                               free_func)) {
    return -1;
  }
  return index;
}

int SSL_set_ex_data(SSL *ssl, int idx, void *data) {
  return CRYPTO_set_ex_data(&ssl->ex_data, idx, data);
}

void *SSL_get_ex_data(const SSL *ssl, int idx) {
  return CRYPTO_get_ex_data(&ssl->ex_data, idx);
}

int SSL_CTX_get_ex_new_index(long argl, void *argp, CRYPTO_EX_unused *unused,
                             CRYPTO_EX_dup *dup_unused,
                             CRYPTO_EX_free *free_func) {
  int index;
  if (!CRYPTO_get_ex_new_index(&g_ex_data_class_ssl_ctx, &index, argl, argp,
                               free_func)) {
    return -1;
  }
  return index;
}

int SSL_CTX_set_ex_data(SSL_CTX *ctx, int idx, void *data) {
  return CRYPTO_set_ex_data(&ctx->ex_data, idx, data);
}

void *SSL_CTX_get_ex_data(const SSL_CTX *ctx, int idx) {
  return CRYPTO_get_ex_data(&ctx->ex_data, idx);
}

int SSL_want(const SSL *ssl) {
  // Historically, OpenSSL did not track |SSL_ERROR_ZERO_RETURN| as an |rwstate|
  // value. We do, but map it back to |SSL_ERROR_NONE| to preserve the original
  // behavior.
  return ssl->s3->rwstate == SSL_ERROR_ZERO_RETURN ? SSL_ERROR_NONE
                                                   : ssl->s3->rwstate;
}

void SSL_CTX_set_tmp_rsa_callback(SSL_CTX *ctx,
                                  RSA *(*cb)(SSL *ssl, int is_export,
                                             int keylength)) {}

void SSL_set_tmp_rsa_callback(SSL *ssl, RSA *(*cb)(SSL *ssl, int is_export,
                                                   int keylength)) {}

void SSL_CTX_set_tmp_dh_callback(SSL_CTX *ctx,
                                 DH *(*cb)(SSL *ssl, int is_export,
                                           int keylength)) {}

void SSL_set_tmp_dh_callback(SSL *ssl, DH *(*cb)(SSL *ssl, int is_export,
                                                 int keylength)) {}

long SSL_CTX_set_dh_auto(SSL_CTX *ctx, int onoff) {
    return 0;
}

static int use_psk_identity_hint(UniquePtr<char> *out,
                                 const char *identity_hint) {
  if (identity_hint != NULL && strlen(identity_hint) > PSK_MAX_IDENTITY_LEN) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DATA_LENGTH_TOO_LONG);
    return 0;
  }

  // Clear currently configured hint, if any.
  out->reset();

  // Treat the empty hint as not supplying one. Plain PSK makes it possible to
  // send either no hint (omit ServerKeyExchange) or an empty hint, while
  // ECDHE_PSK can only spell empty hint. Having different capabilities is odd,
  // so we interpret empty and missing as identical.
  if (identity_hint != NULL && identity_hint[0] != '\0') {
    out->reset(OPENSSL_strdup(identity_hint));
    if (*out == nullptr) {
      return 0;
    }
  }

  return 1;
}

int SSL_CTX_use_psk_identity_hint(SSL_CTX *ctx, const char *identity_hint) {
  return use_psk_identity_hint(&ctx->psk_identity_hint, identity_hint);
}

int SSL_use_psk_identity_hint(SSL *ssl, const char *identity_hint) {
  if (!ssl->config) {
    return 0;
  }
  return use_psk_identity_hint(&ssl->config->psk_identity_hint, identity_hint);
}

const char *SSL_get_psk_identity_hint(const SSL *ssl) {
  if (ssl == NULL) {
    return NULL;
  }
  if (ssl->config == NULL) {
    assert(ssl->config);
    return NULL;
  }
  return ssl->config->psk_identity_hint.get();
}

const char *SSL_get_psk_identity(const SSL *ssl) {
  if (ssl == NULL) {
    return NULL;
  }
  SSL_SESSION *session = SSL_get_session(ssl);
  if (session == NULL) {
    return NULL;
  }
  return session->psk_identity.get();
}

void SSL_set_psk_client_callback(SSL *ssl, SSL_psk_client_cb_func cb) {
  if (!ssl->config) {
    return;
  }
  ssl->config->psk_client_callback = cb;
}

void SSL_CTX_set_psk_client_callback(SSL_CTX *ctx, SSL_psk_client_cb_func cb) {
  ctx->psk_client_callback = cb;
}

void SSL_set_psk_server_callback(SSL *ssl, SSL_psk_server_cb_func cb) {
  if (!ssl->config) {
    return;
  }
  ssl->config->psk_server_callback = cb;
}

void SSL_CTX_set_psk_server_callback(SSL_CTX *ctx, SSL_psk_server_cb_func cb) {
  ctx->psk_server_callback = cb;
}

void SSL_CTX_set_msg_callback(SSL_CTX *ctx,
                              void (*cb)(int write_p, int version,
                                         int content_type, const void *buf,
                                         size_t len, SSL *ssl, void *arg)) {
  ctx->msg_callback = cb;
}

void SSL_CTX_set_msg_callback_arg(SSL_CTX *ctx, void *arg) {
  ctx->msg_callback_arg = arg;
}

void SSL_set_msg_callback(SSL *ssl,
                          void (*cb)(int write_p, int version, int content_type,
                                     const void *buf, size_t len, SSL *ssl,
                                     void *arg)) {
  ssl->msg_callback = cb;
}

void SSL_set_msg_callback_arg(SSL *ssl, void *arg) {
  ssl->msg_callback_arg = arg;
}

void SSL_CTX_set_client_hello_cb(SSL_CTX *c, SSL_client_hello_cb_fn cb,
                                 void *arg) {
  c->client_hello_cb = cb;
  c->client_hello_cb_arg = arg;
}

int SSL_client_hello_isv2(SSL *s) {
  // SSLv2 not supported
  return 0;
}

int SSL_client_hello_get0_ext(SSL *s, unsigned int type, const unsigned char **out,
                              size_t *outlen) {
  GUARD_PTR(s);
  GUARD_PTR(s->s3);
  SSL_HANDSHAKE* hs = s->s3->hs.get();
  GUARD_PTR(hs);

  SSLMessage msg_unused;
  SSL_CLIENT_HELLO client_hello;
  if (!hs->GetClientHello(&msg_unused, &client_hello)) {
    return 0;
  }

  CBS cbs;
  if (!ssl_client_hello_get_extension(&client_hello, &cbs, static_cast<uint16_t>(type))) {
    return 0;  // Extension not found
  }

  if (out != nullptr) {
    *out = CBS_data(&cbs);
  }
  if (outlen != nullptr) {
    *outlen = CBS_len(&cbs);
  }
  return 1;  // Success
}

int SSL_client_hello_get1_extensions_present(SSL *s, int **out,
                                             size_t *outlen) {
  GUARD_PTR(s);
  GUARD_PTR(out);
  GUARD_PTR(outlen);
  size_t num_extensions = 0;

  // Count the number of extensions so we can allocate
  if (1 != SSL_client_hello_get_extension_order(s, nullptr, &num_extensions)) {
    return 0;
  }

  if (num_extensions == 0) {
    *out = nullptr;
    *outlen = 0;
    return 1;
  }

  // Allocate a uint16_t for each extension
  uint16_t *exts =
      static_cast<uint16_t *>(OPENSSL_zalloc(sizeof(uint16_t) * num_extensions));
  if (exts == nullptr) {
    return 0;
  }

  // Collect the type for each extension
  if (1 != SSL_client_hello_get_extension_order(s, exts, &num_extensions)) {
    OPENSSL_free(exts);
    return 0;
  }

  // Allocate the int array needed by caller.
  int *ext_types =
    static_cast<int *>(OPENSSL_zalloc(sizeof(int) * num_extensions));
  if (ext_types == nullptr) {
    OPENSSL_free(exts);
    return 0;
  }

  // Cast each uint16_t type to an int
  for (size_t i = 0; i < num_extensions; i++) {
    ext_types[i] = exts[i];
  }
  OPENSSL_free(exts);

  *out = ext_types;
  *outlen = num_extensions;

  return 1;
}

int SSL_client_hello_get_extension_order(SSL *s, uint16_t *exts, size_t *num_exts) {
  GUARD_PTR(s);
  GUARD_PTR(s->s3);
  SSL_HANDSHAKE *hs = s->s3->hs.get();
  GUARD_PTR(hs);

  SSLMessage msg_unused;
  SSL_CLIENT_HELLO client_hello;
  if (!hs->GetClientHello(&msg_unused, &client_hello)) {
    return 0;
  }

  CBS extensions;
  CBS_init(&extensions, client_hello.extensions, client_hello.extensions_len);

  size_t num_extensions = 0;
  while (CBS_len(&extensions) > 0) {
    uint16_t type = 0;
    CBS body;
    if (!CBS_get_u16(&extensions, &type) ||
        !CBS_get_u16_length_prefixed(&extensions, &body)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
      return 0;
    }
    if (exts != nullptr) {
      // num_exts is an in/out param. Return error if insufficient size.
      if (num_extensions >= *num_exts) {
        return 0;
      }
      // Store the type for each extension
      exts[num_extensions] = type;
    }
    num_extensions++;
  }
  *num_exts = num_extensions;

  return 1;
}

unsigned int SSL_client_hello_get0_legacy_version(SSL *s) {
  GUARD_PTR(s);
  GUARD_PTR(s->s3);
  SSL_HANDSHAKE *hs = s->s3->hs.get();
  GUARD_PTR(hs);

  SSLMessage msg_unused;
  SSL_CLIENT_HELLO client_hello;
  if (!hs->GetClientHello(&msg_unused, &client_hello)) {
    return 0;
  }
  return client_hello.version;
}

void SSL_CTX_set_keylog_callback(SSL_CTX *ctx,
                                 void (*cb)(const SSL *ssl, const char *line)) {
  ctx->keylog_callback = cb;
}

void (*SSL_CTX_get_keylog_callback(const SSL_CTX *ctx))(const SSL *ssl,
                                                        const char *line) {
  return ctx->keylog_callback;
}

void SSL_CTX_set_current_time_cb(SSL_CTX *ctx,
                                 void (*cb)(const SSL *ssl,
                                            struct timeval *out_clock)) {
  ctx->current_time_cb = cb;
}

int SSL_can_release_private_key(const SSL *ssl) {
  if (ssl_can_renegotiate(ssl)) {
    // If the connection can renegotiate (client only), the private key may be
    // used in a future handshake.
    return 0;
  }

  // Otherwise, this is determined by the current handshake.
  return !ssl->s3->hs || ssl->s3->hs->can_release_private_key;
}

int SSL_in_connect_init(const SSL *ssl) {
  return SSL_in_init(ssl) && !SSL_is_server(ssl);
}

int SSL_in_accept_init(const SSL *ssl) {
  return SSL_in_init(ssl) && SSL_is_server(ssl);
}

int SSL_is_init_finished(const SSL *ssl) { return !SSL_in_init(ssl); }

int SSL_in_init(const SSL *ssl) {
  // This returns false once all the handshake state has been finalized, to
  // allow callbacks and getters based on SSL_in_init to return the correct
  // values.
  SSL_HANDSHAKE *hs = ssl->s3->hs.get();
  return hs != nullptr && !hs->handshake_finalized;
}

int SSL_in_false_start(const SSL *ssl) {
  if (ssl->s3->hs == NULL) {
    return 0;
  }
  return ssl->s3->hs->in_false_start;
}

int SSL_cutthrough_complete(const SSL *ssl) { return SSL_in_false_start(ssl); }

int SSL_is_server(const SSL *ssl) { return ssl->server; }

int SSL_is_dtls(const SSL *ssl) { return ssl->method->is_dtls; }

void SSL_CTX_set_select_certificate_cb(
    SSL_CTX *ctx,
    enum ssl_select_cert_result_t (*cb)(const SSL_CLIENT_HELLO *)) {
  ctx->select_certificate_cb = cb;
}

void SSL_CTX_set_dos_protection_cb(SSL_CTX *ctx,
                                   int (*cb)(const SSL_CLIENT_HELLO *)) {
  ctx->dos_protection_cb = cb;
}

void SSL_CTX_set_reverify_on_resume(SSL_CTX *ctx, int enabled) {
  ctx->reverify_on_resume = !!enabled;
}

void SSL_set_enforce_rsa_key_usage(SSL *ssl, int enabled) {
  if (!ssl->config) {
    return;
  }
  ssl->config->enforce_rsa_key_usage = !!enabled;
}

int SSL_was_key_usage_invalid(const SSL *ssl) {
  return ssl->s3->was_key_usage_invalid;
}

void SSL_set_renegotiate_mode(SSL *ssl, enum ssl_renegotiate_mode_t mode) {
  ssl->renegotiate_mode = mode;

  // Check if |ssl_can_renegotiate| has changed and the configuration may now be
  // shed. HTTP clients may initially allow renegotiation for HTTP/1.1, and then
  // disable after the handshake once the ALPN protocol is known to be HTTP/2.
  ssl_maybe_shed_handshake_config(ssl);
}

int SSL_get_ivs(const SSL *ssl, const uint8_t **out_read_iv,
                const uint8_t **out_write_iv, size_t *out_iv_len) {
  size_t write_iv_len;
  if (!ssl->s3->aead_read_ctx->GetIV(out_read_iv, out_iv_len) ||
      !ssl->s3->aead_write_ctx->GetIV(out_write_iv, &write_iv_len) ||
      *out_iv_len != write_iv_len) {
    return 0;
  }

  return 1;
}

uint64_t SSL_get_read_sequence(const SSL *ssl) {
  // TODO(davidben): Internally represent sequence numbers as uint64_t.
  if (SSL_is_dtls(ssl)) {
    // max_seq_num already includes the epoch.
    assert(ssl->d1->r_epoch == (ssl->d1->bitmap.max_seq_num >> 48));
    return ssl->d1->bitmap.max_seq_num;
  }
  return CRYPTO_load_u64_be(ssl->s3->read_sequence);
}

uint64_t SSL_get_write_sequence(const SSL *ssl) {
  uint64_t ret = CRYPTO_load_u64_be(ssl->s3->write_sequence);
  if (SSL_is_dtls(ssl)) {
    assert((ret >> 48) == 0);
    ret |= ((uint64_t)ssl->d1->w_epoch) << 48;
  }
  return ret;
}

uint16_t SSL_get_peer_signature_algorithm(const SSL *ssl) {
  SSL_SESSION *session = SSL_get_session(ssl);
  if (session == NULL) {
    return 0;
  }

  return session->peer_signature_algorithm;
}

int SSL_get_peer_signature_nid(const SSL *ssl, int *psig_nid) {
  GUARD_PTR(psig_nid);

  uint16_t sig_alg = SSL_get_peer_signature_algorithm(ssl);
  if (sig_alg == 0) {
    return 0;
  }

  const EVP_MD *digest_type = SSL_get_signature_algorithm_digest(sig_alg);
  if (digest_type == NULL) {
    return 0;
  }

  *psig_nid = EVP_MD_nid(digest_type);
  return 1;
}

int SSL_get_peer_signature_type_nid(const SSL *ssl, int *psigtype_nid) {
  GUARD_PTR(psigtype_nid);

  uint16_t sig_alg = SSL_get_peer_signature_algorithm(ssl);
  if (sig_alg == 0) {
    return 0;
  }

  int sig_type = SSL_get_signature_algorithm_key_type(sig_alg);

  if (sig_type == EVP_PKEY_NONE) {
    return 0;
  }

  *psigtype_nid = sig_type;
  return 1;
}

size_t SSL_get_client_random(const SSL *ssl, uint8_t *out, size_t max_out) {
  if (max_out == 0) {
    return sizeof(ssl->s3->client_random);
  }
  if (max_out > sizeof(ssl->s3->client_random)) {
    max_out = sizeof(ssl->s3->client_random);
  }
  OPENSSL_memcpy(out, ssl->s3->client_random, max_out);
  return max_out;
}

size_t SSL_get_server_random(const SSL *ssl, uint8_t *out, size_t max_out) {
  if (max_out == 0) {
    return sizeof(ssl->s3->server_random);
  }
  if (max_out > sizeof(ssl->s3->server_random)) {
    max_out = sizeof(ssl->s3->server_random);
  }
  OPENSSL_memcpy(out, ssl->s3->server_random, max_out);
  return max_out;
}

const SSL_CIPHER *SSL_get_pending_cipher(const SSL *ssl) {
  SSL_HANDSHAKE *hs = ssl->s3->hs.get();
  if (hs == NULL) {
    return NULL;
  }
  return hs->new_cipher;
}

void SSL_set_retain_only_sha256_of_client_certs(SSL *ssl, int enabled) {
  if (!ssl->config) {
    return;
  }
  ssl->config->retain_only_sha256_of_client_certs = !!enabled;
}

void SSL_CTX_set_retain_only_sha256_of_client_certs(SSL_CTX *ctx, int enabled) {
  ctx->retain_only_sha256_of_client_certs = !!enabled;
}

void SSL_CTX_set_grease_enabled(SSL_CTX *ctx, int enabled) {
  ctx->grease_enabled = !!enabled;
}

void SSL_CTX_set_permute_extensions(SSL_CTX *ctx, int enabled) {
  ctx->permute_extensions = !!enabled;
}

void SSL_set_permute_extensions(SSL *ssl, int enabled) {
  if (!ssl->config) {
    return;
  }
  ssl->config->permute_extensions = !!enabled;
}

int32_t SSL_get_ticket_age_skew(const SSL *ssl) {
  return ssl->s3->ticket_age_skew;
}

void SSL_CTX_set_false_start_allowed_without_alpn(SSL_CTX *ctx, int allowed) {
  ctx->false_start_allowed_without_alpn = !!allowed;
}

int SSL_used_hello_retry_request(const SSL *ssl) {
  return ssl->s3->used_hello_retry_request;
}

void SSL_set_shed_handshake_config(SSL *ssl, int enable) {
  if (!ssl->config) {
    return;
  }
  ssl->config->shed_handshake_config = !!enable;
}

void SSL_set_jdk11_workaround(SSL *ssl, int enable) {
  if (!ssl->config) {
    return;
  }
  ssl->config->jdk11_workaround = !!enable;
}

void SSL_set_check_client_certificate_type(SSL *ssl, int enable) {
  if (!ssl->config) {
    return;
  }
  ssl->config->check_client_certificate_type = !!enable;
}

void SSL_set_quic_use_legacy_codepoint(SSL *ssl, int use_legacy) {
  if (!ssl->config) {
    return;
  }
  ssl->config->quic_use_legacy_codepoint = !!use_legacy;
}

int SSL_clear(SSL *ssl) {
  if (!ssl->config) {
    return 0;  // SSL_clear may not be used after shedding config.
  }

  ssl->client_cipher_suites.reset();
  ssl->all_client_cipher_suites.reset();

  // In OpenSSL, reusing a client |SSL| with |SSL_clear| causes the previously
  // established session to be offered the next time around. wpa_supplicant
  // depends on this behavior, so emulate it.
  UniquePtr<SSL_SESSION> session;
  if (!ssl->server && ssl->s3->established_session != NULL) {
    session = UpRef(ssl->s3->established_session);
  }

  // The ssl->d1->mtu is simultaneously configuration (preserved across
  // clear) and connection-specific state (gets reset).
  //
  // TODO(davidben): Avoid this.
  unsigned mtu = 0;
  if (ssl->d1 != NULL) {
    mtu = ssl->d1->mtu;
  }

  ssl->method->ssl_free(ssl);
  if (!ssl->method->ssl_new(ssl)) {
    return 0;
  }

  if (SSL_is_dtls(ssl) && (SSL_get_options(ssl) & SSL_OP_NO_QUERY_MTU)) {
    ssl->d1->mtu = mtu;
  }

  if (session != nullptr) {
    SSL_set_session(ssl, session.get());
  }

  return 1;
}

int SSL_CTX_sess_connect(const SSL_CTX *ctx) {
  return ssl_read_counter(ctx, ctx->stats.sess_connect);
}

int SSL_CTX_sess_connect_good(const SSL_CTX *ctx) {
  return ssl_read_counter(ctx, ctx->stats.sess_connect_good);
}

int SSL_CTX_sess_connect_renegotiate(const SSL_CTX *ctx) {
  return ssl_read_counter(ctx, ctx->stats.sess_connect_renegotiate);
}

int SSL_CTX_sess_accept(const SSL_CTX *ctx) {
  return ssl_read_counter(ctx, ctx->stats.sess_accept);
}

int SSL_CTX_sess_accept_renegotiate(const SSL_CTX *ctx) { return 0; }

int SSL_CTX_sess_accept_good(const SSL_CTX *ctx) {
  return ssl_read_counter(ctx, ctx->stats.sess_accept_good);
}

int SSL_CTX_sess_hits(const SSL_CTX *ctx) {
  return ssl_read_counter(ctx, ctx->stats.sess_hit);
}

int SSL_CTX_sess_cb_hits(const SSL_CTX *ctx) {
  return ssl_read_counter(ctx, ctx->stats.sess_cb_hit);
}

int SSL_CTX_sess_misses(const SSL_CTX *ctx) {
  return ssl_read_counter(ctx, ctx->stats.sess_miss);
}

int SSL_CTX_sess_timeouts(const SSL_CTX *ctx) {
  return ssl_read_counter(ctx, ctx->stats.sess_timeout);
}

int SSL_CTX_sess_cache_full(const SSL_CTX *ctx) {
  return ssl_read_counter(ctx, ctx->stats.sess_cache_full);
}

int SSL_num_renegotiations(const SSL *ssl) {
  return SSL_total_renegotiations(ssl);
}

int SSL_clear_num_renegotiations(const SSL *ssl) {
  int ret = SSL_total_renegotiations(ssl);
  ssl->s3->total_renegotiations = 0;
  return ret;
}

int SSL_CTX_need_tmp_RSA(const SSL_CTX *ctx) { return 0; }
int SSL_need_tmp_RSA(const SSL *ssl) { return 0; }
int SSL_CTX_set_tmp_rsa(SSL_CTX *ctx, const RSA *rsa) { return 1; }
int SSL_set_tmp_rsa(SSL *ssl, const RSA *rsa) { return 1; }
void ERR_load_SSL_strings(void) {}
void SSL_load_error_strings(void) {}
int SSL_cache_hit(SSL *ssl) { return SSL_session_reused(ssl); }

int SSL_CTX_set_tmp_ecdh(SSL_CTX *ctx, const EC_KEY *ec_key) {
  if (ec_key == NULL || EC_KEY_get0_group(ec_key) == NULL) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  int nid = EC_GROUP_get_curve_name(EC_KEY_get0_group(ec_key));
  return SSL_CTX_set1_groups(ctx, &nid, 1);
}

int SSL_set_tmp_ecdh(SSL *ssl, const EC_KEY *ec_key) {
  if (ec_key == NULL || EC_KEY_get0_group(ec_key) == NULL) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  int nid = EC_GROUP_get_curve_name(EC_KEY_get0_group(ec_key));
  return SSL_set1_groups(ssl, &nid, 1);
}

void SSL_CTX_set_ticket_aead_method(SSL_CTX *ctx,
                                    const SSL_TICKET_AEAD_METHOD *aead_method) {
  ctx->ticket_aead_method = aead_method;
}

SSL_SESSION *SSL_process_tls13_new_session_ticket(SSL *ssl, const uint8_t *buf,
                                                  size_t buf_len) {
  if (SSL_in_init(ssl) || ssl_protocol_version(ssl) != TLS1_3_VERSION ||
      ssl->server) {
    // Only TLS 1.3 clients are supported.
    OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return nullptr;
  }

  CBS cbs, body;
  CBS_init(&cbs, buf, buf_len);
  uint8_t type;
  if (!CBS_get_u8(&cbs, &type) || !CBS_get_u24_length_prefixed(&cbs, &body) ||
      CBS_len(&cbs) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return nullptr;
  }

  UniquePtr<SSL_SESSION> session = tls13_create_session_with_ticket(ssl, &body);
  if (!session) {
    // |tls13_create_session_with_ticket| puts the correct error.
    return nullptr;
  }
  return session.release();
}

int SSL_CTX_set_num_tickets(SSL_CTX *ctx, size_t num_tickets) {
  num_tickets = std::min(num_tickets, kMaxTickets);
  static_assert(kMaxTickets <= 0xff, "Too many tickets.");
  ctx->num_tickets = static_cast<uint8_t>(num_tickets);
  return 1;
}

size_t SSL_CTX_get_num_tickets(const SSL_CTX *ctx) { return ctx->num_tickets; }

int SSL_set_tlsext_status_type(SSL *ssl, int type) {
  if (!ssl->config) {
    return 0;
  }
  ssl->config->ocsp_stapling_enabled = type == TLSEXT_STATUSTYPE_ocsp;
  return 1;
}

int SSL_get_tlsext_status_type(const SSL *ssl) {
  if (ssl->server) {
    SSL_HANDSHAKE *hs = ssl->s3->hs.get();
    return hs != nullptr && hs->ocsp_stapling_requested
               ? TLSEXT_STATUSTYPE_ocsp
               : TLSEXT_STATUSTYPE_nothing;
  }

  return ssl->config != nullptr && ssl->config->ocsp_stapling_enabled
             ? TLSEXT_STATUSTYPE_ocsp
             : TLSEXT_STATUSTYPE_nothing;
}

int SSL_set_tlsext_status_ocsp_resp(SSL *ssl, uint8_t *resp, size_t resp_len) {
  if (SSL_set_ocsp_response(ssl, resp, resp_len)) {
    OPENSSL_free(resp);
    return 1;
  }
  return 0;
}

size_t SSL_get_tlsext_status_ocsp_resp(const SSL *ssl, const uint8_t **out) {
  size_t ret;
  SSL_get0_ocsp_response(ssl, out, &ret);
  return ret;
}

int SSL_CTX_set_tlsext_status_cb(SSL_CTX *ctx,
                                 int (*callback)(SSL *ssl, void *arg)) {
  ctx->legacy_ocsp_callback = callback;
  return 1;
}

int SSL_CTX_get_tlsext_status_cb(SSL_CTX *ctx,
                                 int (**callback)(SSL *, void *)) {
  *callback = ctx->legacy_ocsp_callback;
  return 1;
}

int SSL_CTX_set_tlsext_status_arg(SSL_CTX *ctx, void *arg) {
  ctx->legacy_ocsp_callback_arg = arg;
  return 1;
}

uint16_t SSL_get_curve_id(const SSL *ssl) { return SSL_get_group_id(ssl); }

const char *SSL_get_curve_name(uint16_t curve_id) {
  return SSL_get_group_name(curve_id);
}

size_t SSL_get_all_curve_names(const char **out, size_t max_out) {
  return SSL_get_all_group_names(out, max_out);
}

int SSL_CTX_set1_curves(SSL_CTX *ctx, const int *curves, size_t num_curves) {
  return SSL_CTX_set1_groups(ctx, curves, num_curves);
}

int SSL_set1_curves(SSL *ssl, const int *curves, size_t num_curves) {
  return SSL_set1_groups(ssl, curves, num_curves);
}

int SSL_CTX_set1_curves_list(SSL_CTX *ctx, const char *curves) {
  return SSL_CTX_set1_groups_list(ctx, curves);
}

int SSL_set1_curves_list(SSL *ssl, const char *curves) {
  return SSL_set1_groups_list(ssl, curves);
}

size_t SSL_client_hello_get0_ciphers(SSL *ssl, const unsigned char **out) {
  if (SSL_get_client_ciphers(ssl) == nullptr) {
      return 0;
  }
  const char * ciphers = ssl->all_client_cipher_suites.get();
  assert(ciphers != nullptr);

  if (out != nullptr) {
    *out = reinterpret_cast<const unsigned char*>(ciphers);
  }
  return ssl->all_client_cipher_suites_len;
}

OPENSSL_EXPORT int SSL_get_read_traffic_secret(
    const SSL *ssl,
    uint8_t *secret, size_t *out_len)  {
  if (SSL_in_init(ssl) || ssl_protocol_version(ssl) < TLS1_3_VERSION) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  GUARD_PTR(out_len);

  if (secret == nullptr) {
    *out_len = ssl->s3->read_traffic_secret_len;
    return 1;
  }

  if (ssl->s3->read_traffic_secret_len > *out_len) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_OVERFLOW);
    return 0;
  }

  OPENSSL_memcpy(secret, ssl->s3->read_traffic_secret,
                 ssl->s3->read_traffic_secret_len);

  *out_len = ssl->s3->read_traffic_secret_len;

  return 1;
}

OPENSSL_EXPORT int SSL_get_write_traffic_secret(
    const SSL *ssl,
    uint8_t *secret, size_t *out_len)  {
  if (SSL_in_init(ssl) || ssl_protocol_version(ssl) < TLS1_3_VERSION) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  GUARD_PTR(out_len);

  if (secret == nullptr) {
    *out_len = ssl->s3->write_traffic_secret_len;
    return 1;
  }

  if (ssl->s3->write_traffic_secret_len > *out_len) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_OVERFLOW);
    return 0;
  }

  OPENSSL_memcpy(secret, ssl->s3->write_traffic_secret,
                 ssl->s3->write_traffic_secret_len);

  *out_len = ssl->s3->write_traffic_secret_len;

  return 1;
}

// No-op function for compatibility with OpenSSL.
int SSL_verify_client_post_handshake(SSL *ssl) {
  return 0;
}
