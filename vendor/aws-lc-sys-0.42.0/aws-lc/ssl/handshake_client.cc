// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2007 The OpenSSL Project. All rights reserved.
// Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
// Copyright 2005 Nokia. All rights reserved.
//
// ECC cipher suite support in OpenSSL originally written by
// Vipul Gupta and Sumit Gupta of Sun Microsystems Laboratories.
//
// The Contribution, originally written by Mika Kousa and Pasi Eronen of
// Nokia Corporation, consists of the "PSK" (Pre-Shared Key) ciphersuites
// support (see RFC 4279) to OpenSSL.
//
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <utility>

#include <openssl/aead.h>
#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/ec_key.h>
#include <openssl/ecdsa.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/hpke.h>
#include <openssl/md5.h>
#include <openssl/mem.h>
#include <openssl/rand.h>
#include <openssl/sha.h>

#include "../crypto/internal.h"
#include "internal.h"


BSSL_NAMESPACE_BEGIN

enum ssl_client_hs_state_t {
  state_start_connect = 0,
  state_enter_early_data,
  state_early_reverify_server_certificate,
  state_read_hello_verify_request,
  state_read_server_hello,
  state_tls13,
  state_read_server_certificate,
  state_read_certificate_status,
  state_verify_server_certificate,
  state_reverify_server_certificate,
  state_read_server_key_exchange,
  state_read_certificate_request,
  state_read_server_hello_done,
  state_send_client_certificate,
  state_send_client_key_exchange,
  state_send_client_certificate_verify,
  state_send_client_finished,
  state_finish_flight,
  state_read_session_ticket,
  state_process_change_cipher_spec,
  state_read_server_finished,
  state_finish_client_handshake,
  state_done,
};

// ssl_get_client_disabled sets |*out_mask_a| and |*out_mask_k| to masks of
// disabled algorithms.
static void ssl_get_client_disabled(const SSL_HANDSHAKE *hs,
                                    uint32_t *out_mask_a,
                                    uint32_t *out_mask_k) {
  *out_mask_a = 0;
  *out_mask_k = 0;

  // PSK requires a client callback.
  if (hs->config->psk_client_callback == NULL) {
    *out_mask_a |= SSL_aPSK;
    *out_mask_k |= SSL_kPSK;
  }
}

// collect_cipher_protocol_ids uses |cbb| to collect IANA-assigned numbers
// of |ciphers|. |mask_a|, |mask_k|, |max_version| and |min_version|
// are used to filter the |ciphers|. |any_enabled| will be true if not all
// ciphers are filtered out.
// It returns true when success. It returns false otherwise.
static bool collect_cipher_protocol_ids(STACK_OF(SSL_CIPHER) *ciphers,
  CBB *cbb, uint32_t mask_k, uint32_t mask_a, uint16_t max_version,
  uint16_t min_version, bool *any_enabled) {
  *any_enabled = false;

  for (const SSL_CIPHER *cipher : ciphers) {
    // Skip disabled ciphers
    if ((cipher->algorithm_mkey & mask_k) ||
        (cipher->algorithm_auth & mask_a)) {
      continue;
    }
    if (SSL_CIPHER_get_min_version(cipher) > max_version ||
        SSL_CIPHER_get_max_version(cipher) < min_version) {
      continue;
    }
    *any_enabled = true;
    if (!CBB_add_u16(cbb, SSL_CIPHER_get_protocol_id(cipher))) {
      return false;
    }
  }
  return true;
}

static bool ssl_write_client_cipher_list(const SSL_HANDSHAKE *hs, CBB *out,
                                         ssl_client_hello_type_t type) {
  const SSL *const ssl = hs->ssl;
  uint32_t mask_a, mask_k;
  ssl_get_client_disabled(hs, &mask_a, &mask_k);

  CBB child;
  if (!CBB_add_u16_length_prefixed(out, &child)) {
    return false;
  }

  // Add a fake cipher suite. See RFC 8701.
  if (ssl->ctx->grease_enabled &&
      !CBB_add_u16(&child, ssl_get_grease_value(hs, ssl_grease_cipher))) {
    return false;
  }

  // Add all ciphers unless TLS 1.3 only connection
  if (hs->min_version < TLS1_3_VERSION && type != ssl_client_hello_inner) {
    bool any_enabled = false;
    if (!collect_cipher_protocol_ids(SSL_get_ciphers(ssl), &child, mask_k,
      mask_a, hs->max_version, hs->min_version, &any_enabled)) {
      return false;
    }

    // If all ciphers were disabled, return the error to the caller.
    if (!any_enabled) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_NO_CIPHERS_AVAILABLE);
      return false;
    }
  } else if (hs->max_version >= TLS1_3_VERSION) {
    // Only TLS 1.3 ciphers
    STACK_OF(SSL_CIPHER) *ciphers = (ssl->config && ssl->config->tls13_cipher_list) ?
      ssl->config->tls13_cipher_list->ciphers.get() : ssl->ctx->tls13_cipher_list->ciphers.get();

    bool any_enabled = false;

    if (!collect_cipher_protocol_ids(ciphers, &child, mask_k,
      mask_a, hs->max_version, hs->min_version, &any_enabled)) {
      return false;
    }

    // If all ciphers were disabled, return the error to the caller.
    if (!any_enabled) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_NO_CIPHERS_AVAILABLE);
      return false;
    }
  }

  if (ssl->mode & SSL_MODE_SEND_FALLBACK_SCSV) {
    if (!CBB_add_u16(&child, SSL3_CK_FALLBACK_SCSV & 0xffff)) {
      return false;
    }
  }

  return CBB_flush(out);
}

bool ssl_write_client_hello_without_extensions(const SSL_HANDSHAKE *hs,
                                               CBB *cbb,
                                               ssl_client_hello_type_t type,
                                               bool empty_session_id) {
  const SSL *const ssl = hs->ssl;
  CBB child;
  if (!CBB_add_u16(cbb, hs->client_version) ||
      !CBB_add_bytes(cbb,
                     type == ssl_client_hello_inner ? hs->inner_client_random
                                                    : ssl->s3->client_random,
                     SSL3_RANDOM_SIZE) ||
      !CBB_add_u8_length_prefixed(cbb, &child)) {
    return false;
  }

  // Do not send a session ID on renegotiation.
  if (!ssl->s3->initial_handshake_complete &&
      !empty_session_id &&
      !CBB_add_bytes(&child, hs->session_id, hs->session_id_len)) {
    return false;
  }

  if (SSL_is_dtls(ssl)) {
    if (!CBB_add_u8_length_prefixed(cbb, &child) ||
        !CBB_add_bytes(&child, hs->dtls_cookie.data(),
                       hs->dtls_cookie.size())) {
      return false;
    }
  }

  if (!ssl_write_client_cipher_list(hs, cbb, type) ||
      !CBB_add_u8(cbb, 1 /* one compression method */) ||
      !CBB_add_u8(cbb, 0 /* null compression */)) {
    return false;
  }
  return true;
}

bool ssl_add_client_hello(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  ScopedCBB cbb;
  CBB body;
  ssl_client_hello_type_t type = hs->selected_ech_config
                                     ? ssl_client_hello_outer
                                     : ssl_client_hello_unencrypted;
  bool needs_psk_binder;
  Array<uint8_t> msg;
  if (!ssl->method->init_message(ssl, cbb.get(), &body, SSL3_MT_CLIENT_HELLO) ||
      !ssl_write_client_hello_without_extensions(hs, &body, type,
                                                 /*empty_session_id=*/false) ||
      !ssl_add_clienthello_tlsext(hs, &body, /*out_encoded=*/nullptr,
                                  &needs_psk_binder, type, CBB_len(&body)) ||
      !ssl->method->finish_message(ssl, cbb.get(), &msg)) {
    return false;
  }

  // Now that the length prefixes have been computed, fill in the placeholder
  // PSK binder.
  if (needs_psk_binder) {
    // ClientHelloOuter cannot have a PSK binder. Otherwise the
    // ClientHellOuterAAD computation would break.
    assert(type != ssl_client_hello_outer);
    if (!tls13_write_psk_binder(hs, hs->transcript, MakeSpan(msg),
                                /*out_binder_len=*/0)) {
      return false;
    }
  }

  return ssl->method->add_message(ssl, std::move(msg));
}

static bool parse_server_version(const SSL_HANDSHAKE *hs, uint16_t *out_version,
                                 uint8_t *out_alert,
                                 const ParsedServerHello &server_hello) {
  // If the outer version is not TLS 1.2, use it.
  // TODO(davidben): This function doesn't quite match the RFC8446 formulation.
  if (server_hello.legacy_version != TLS1_2_VERSION) {
    *out_version = server_hello.legacy_version;
    return true;
  }

  SSLExtension supported_versions(TLSEXT_TYPE_supported_versions);
  CBS extensions = server_hello.extensions;
  if (!ssl_parse_extensions(&extensions, out_alert, {&supported_versions},
                            /*ignore_unknown=*/true)) {
    return false;
  }

  if (!supported_versions.present) {
    *out_version = server_hello.legacy_version;
    return true;
  }

  if (!CBS_get_u16(&supported_versions.data, out_version) ||
       CBS_len(&supported_versions.data) != 0) {
    *out_alert = SSL_AD_DECODE_ERROR;
    return false;
  }

  return true;
}

// should_offer_early_data returns |ssl_early_data_accepted| if |hs| should
// offer early data, and some other reason code otherwise.
static ssl_early_data_reason_t should_offer_early_data(
    const SSL_HANDSHAKE *hs) {
  const SSL *const ssl = hs->ssl;
  assert(!ssl->server);
  if (!ssl->enable_early_data) {
    return ssl_early_data_disabled;
  }

  if (hs->max_version < TLS1_3_VERSION) {
    // We discard inapplicable sessions, so this is redundant with the session
    // checks below, but reporting that TLS 1.3 was disabled is more useful.
    return ssl_early_data_protocol_version;
  }

  if (ssl->session == nullptr) {
    return ssl_early_data_no_session_offered;
  }

  if (ssl_session_protocol_version(ssl->session.get()) < TLS1_3_VERSION ||
      ssl->session->ticket_max_early_data == 0) {
    return ssl_early_data_unsupported_for_session;
  }

  if (!ssl->session->early_alpn.empty()) {
    if (!ssl_is_alpn_protocol_allowed(hs, ssl->session->early_alpn)) {
      // Avoid reporting a confusing value in |SSL_get0_alpn_selected|.
      return ssl_early_data_alpn_mismatch;
    }

    // If the previous connection negotiated ALPS, only offer 0-RTT when the
    // local are settings are consistent with what we'd offer for this
    // connection.
    if (ssl->session->has_application_settings) {
      Span<const uint8_t> settings;
      if (!ssl_get_local_application_settings(hs, &settings,
                                              ssl->session->early_alpn) ||
          settings != ssl->session->local_application_settings) {
        return ssl_early_data_alps_mismatch;
      }
    }
  }

  // Early data has not yet been accepted, but we use it as a success code.
  return ssl_early_data_accepted;
}

void ssl_done_writing_client_hello(SSL_HANDSHAKE *hs) {
  hs->ech_client_outer.Reset();
  hs->cookie.Reset();
  hs->key_share_bytes.Reset();
}

static enum ssl_hs_wait_t do_start_connect(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;

  ssl_do_info_callback(ssl, SSL_CB_HANDSHAKE_START, 1);
  ssl_update_counter(ssl->session_ctx.get(),
                     ssl->session_ctx->stats.sess_connect, true);
  // |session_reused| must be reset in case this is a renegotiation.
  ssl->s3->session_reused = false;

  // Freeze the version range.
  if (!ssl_get_version_range(hs, &hs->min_version, &hs->max_version)) {
    return ssl_hs_error;
  }

  uint8_t ech_enc[EVP_HPKE_MAX_ENC_LENGTH];
  size_t ech_enc_len;
  if (!ssl_select_ech_config(hs, ech_enc, &ech_enc_len)) {
    return ssl_hs_error;
  }

  // Always advertise the ClientHello version from the original maximum version,
  // even on renegotiation. The static RSA key exchange uses this field, and
  // some servers fail when it changes across handshakes.
  if (SSL_is_dtls(hs->ssl)) {
    hs->client_version =
        hs->max_version >= TLS1_2_VERSION ? DTLS1_2_VERSION : DTLS1_VERSION;
  } else {
    hs->client_version =
        hs->max_version >= TLS1_2_VERSION ? TLS1_2_VERSION : hs->max_version;
  }

  // If the configured session has expired or is not usable, drop it. We also do
  // not offer sessions on renegotiation.
  if (ssl->session != nullptr) {
    if (ssl->session->is_server ||
        !ssl_supports_version(hs, ssl->session->ssl_version) ||
        // Do not offer TLS 1.2 sessions with ECH. ClientHelloInner does not
        // offer TLS 1.2, and the cleartext session ID may leak the server
        // identity.
        (hs->selected_ech_config &&
         ssl_session_protocol_version(ssl->session.get()) < TLS1_3_VERSION) ||
        !SSL_SESSION_is_resumable(ssl->session.get()) ||
        !ssl_session_is_time_valid(ssl, ssl->session.get()) ||
        (ssl->quic_method != nullptr) != ssl->session->is_quic ||
        ssl->s3->initial_handshake_complete) {
      ssl_set_session(ssl, nullptr);
    }
  }

  if (!RAND_bytes(ssl->s3->client_random, sizeof(ssl->s3->client_random))) {
    return ssl_hs_error;
  }
  if (hs->selected_ech_config &&
      !RAND_bytes(hs->inner_client_random, sizeof(hs->inner_client_random))) {
    return ssl_hs_error;
  }

  // Never send a session ID in QUIC. QUIC uses TLS 1.3 at a minimum and
  // disables TLS 1.3 middlebox compatibility mode.
  if (ssl->quic_method == nullptr) {
    const bool has_id_session = ssl->session != nullptr &&
                                ssl->session->session_id_length > 0 &&
                                ssl->session->ticket.empty();
    const bool has_ticket_session =
        ssl->session != nullptr && !ssl->session->ticket.empty();
    if (has_id_session) {
      hs->session_id_len = ssl->session->session_id_length;
      OPENSSL_memcpy(hs->session_id, ssl->session->session_id,
                     hs->session_id_len);
    } else if (has_ticket_session || hs->max_version >= TLS1_3_VERSION) {
      // Send a random session ID. TLS 1.3 always sends one, and TLS 1.2 session
      // tickets require a placeholder value to signal resumption.
      hs->session_id_len = sizeof(hs->session_id);
      if (!RAND_bytes(hs->session_id, hs->session_id_len)) {
        return ssl_hs_error;
      }
    }
  }

  ssl_early_data_reason_t reason = should_offer_early_data(hs);
  if (reason != ssl_early_data_accepted) {
    ssl->s3->early_data_reason = reason;
  } else {
    hs->early_data_offered = true;
  }

  if (!ssl_setup_key_shares(hs, /*override_group_id=*/0) ||
      !ssl_setup_extension_permutation(hs) ||
      !ssl_encrypt_client_hello(hs, MakeConstSpan(ech_enc, ech_enc_len)) ||
      !ssl_add_client_hello(hs)) {
    return ssl_hs_error;
  }

  hs->state = state_enter_early_data;
  return ssl_hs_flush;
}

static enum ssl_hs_wait_t do_enter_early_data(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;

  if (SSL_is_dtls(ssl)) {
    hs->state = state_read_hello_verify_request;
    return ssl_hs_ok;
  }

  if (!hs->early_data_offered) {
    hs->state = state_read_server_hello;
    return ssl_hs_ok;
  }

  ssl->s3->aead_write_ctx->SetVersionIfNullCipher(ssl->session->ssl_version);
  if (!ssl->method->add_change_cipher_spec(ssl)) {
    return ssl_hs_error;
  }

  if (!tls13_init_early_key_schedule(hs, ssl->session.get()) ||
      !tls13_derive_early_secret(hs)) {
    return ssl_hs_error;
  }

  // Stash the early data session, so connection properties may be queried out
  // of it.
  hs->early_session = UpRef(ssl->session);
  hs->state = state_early_reverify_server_certificate;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_early_reverify_server_certificate(SSL_HANDSHAKE *hs) {
  if (hs->ssl->ctx->reverify_on_resume) {
    // Don't send an alert on error. The alert be in early data, which the
    // server may not accept anyway. It would also be a mismatch between QUIC
    // and TCP because the QUIC early keys are deferred below.
    //
    // TODO(davidben): The client behavior should be to verify the certificate
    // before deciding whether to offer the session and, if invalid, decline to
    // send the session.
    switch (ssl_reverify_peer_cert(hs, /*send_alert=*/false)) {
      case ssl_verify_ok:
        break;
      case ssl_verify_invalid:
        return ssl_hs_error;
      case ssl_verify_retry:
        hs->state = state_early_reverify_server_certificate;
        return ssl_hs_certificate_verify;
    }
  }

  // Defer releasing the 0-RTT key to after certificate reverification, so the
  // QUIC implementation does not accidentally write data too early.
  if (!tls13_set_traffic_key(hs->ssl, ssl_encryption_early_data, evp_aead_seal,
                             hs->early_session.get(),
                             hs->early_traffic_secret())) {
    return ssl_hs_error;
  }

  hs->in_early_data = true;
  hs->can_early_write = true;
  hs->state = state_read_server_hello;
  return ssl_hs_early_return;
}

static enum ssl_hs_wait_t do_read_hello_verify_request(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;

  assert(SSL_is_dtls(ssl));

  // When implementing DTLS 1.3, we need to handle the interactions between
  // HelloVerifyRequest, DTLS 1.3's HelloVerifyRequest removal, and ECH.
  assert(hs->max_version < TLS1_3_VERSION);

  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }

  if (msg.type != DTLS1_MT_HELLO_VERIFY_REQUEST) {
    hs->state = state_read_server_hello;
    return ssl_hs_ok;
  }

  CBS hello_verify_request = msg.body, cookie;
  uint16_t server_version;
  if (!CBS_get_u16(&hello_verify_request, &server_version) ||
      !CBS_get_u8_length_prefixed(&hello_verify_request, &cookie) ||
      CBS_len(&hello_verify_request) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    return ssl_hs_error;
  }

  if (!hs->dtls_cookie.CopyFrom(cookie)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
    return ssl_hs_error;
  }

  ssl->method->next_message(ssl);

  // DTLS resets the handshake buffer after HelloVerifyRequest.
  if (!hs->transcript.Init()) {
    return ssl_hs_error;
  }

  if (!ssl_add_client_hello(hs)) {
    return ssl_hs_error;
  }

  hs->state = state_read_server_hello;
  return ssl_hs_flush;
}

bool ssl_parse_server_hello(ParsedServerHello *out, uint8_t *out_alert,
                            const SSLMessage &msg) {
  if (msg.type != SSL3_MT_SERVER_HELLO) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_MESSAGE);
    *out_alert = SSL_AD_UNEXPECTED_MESSAGE;
    return false;
  }
  out->raw = msg.raw;
  CBS body = msg.body;
  if (!CBS_get_u16(&body, &out->legacy_version) ||
      !CBS_get_bytes(&body, &out->random, SSL3_RANDOM_SIZE) ||
      !CBS_get_u8_length_prefixed(&body, &out->session_id) ||
      CBS_len(&out->session_id) > SSL3_SESSION_ID_SIZE ||
      !CBS_get_u16(&body, &out->cipher_suite) ||
      !CBS_get_u8(&body, &out->compression_method)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    *out_alert = SSL_AD_DECODE_ERROR;
    return false;
  }
  // In TLS 1.2 and below, empty extensions blocks may be omitted. In TLS 1.3,
  // ServerHellos always have extensions, so this can be applied generically.
  CBS_init(&out->extensions, nullptr, 0);
  if ((CBS_len(&body) != 0 &&
       !CBS_get_u16_length_prefixed(&body, &out->extensions)) ||
      CBS_len(&body) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    *out_alert = SSL_AD_DECODE_ERROR;
    return false;
  }
  return true;
}

static enum ssl_hs_wait_t do_read_server_hello(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_server_hello;
  }

  ParsedServerHello server_hello;
  uint16_t server_version;
  uint8_t alert = SSL_AD_DECODE_ERROR;
  if (!ssl_parse_server_hello(&server_hello, &alert, msg) ||
      !parse_server_version(hs, &server_version, &alert, server_hello)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
    return ssl_hs_error;
  }

  if (!ssl_supports_version(hs, server_version)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNSUPPORTED_PROTOCOL);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_PROTOCOL_VERSION);
    return ssl_hs_error;
  }

  assert(ssl->s3->have_version == ssl->s3->initial_handshake_complete);
  if (!ssl->s3->have_version) {
    ssl->version = server_version;
    // At this point, the connection's version is known and ssl->version is
    // fixed. Begin enforcing the record-layer version.
    ssl->s3->have_version = true;
    ssl->s3->aead_write_ctx->SetVersionIfNullCipher(ssl->version);
  } else if (server_version != ssl->version) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_SSL_VERSION);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_PROTOCOL_VERSION);
    return ssl_hs_error;
  }

  if (ssl_protocol_version(ssl) >= TLS1_3_VERSION) {
    hs->state = state_tls13;
    return ssl_hs_ok;
  }

  // Clear some TLS 1.3 state that no longer needs to be retained.
  hs->key_shares[0].reset();
  hs->key_shares[1].reset();
  ssl_done_writing_client_hello(hs);

  // A TLS 1.2 server would not know to skip the early data we offered. Report
  // an error code sooner. The caller may use this error code to implement the
  // fallback described in RFC 8446 appendix D.3.
  if (hs->early_data_offered) {
    // Disconnect early writes. This ensures subsequent |SSL_write| calls query
    // the handshake which, in turn, will replay the error code rather than fail
    // at the |write_shutdown| check. See https://crbug.com/1078515.
    // TODO(davidben): Should all handshake errors do this? What about record
    // decryption failures?
    hs->can_early_write = false;
    OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_VERSION_ON_EARLY_DATA);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_PROTOCOL_VERSION);
    return ssl_hs_error;
  }

  // TLS 1.2 handshakes cannot accept ECH.
  if (hs->selected_ech_config) {
    ssl->s3->ech_status = ssl_ech_rejected;
  }

  // Copy over the server random.
  OPENSSL_memcpy(ssl->s3->server_random, CBS_data(&server_hello.random),
                 SSL3_RANDOM_SIZE);

  // Enforce the TLS 1.3 anti-downgrade feature.
  if (!ssl->s3->initial_handshake_complete &&
      ssl_supports_version(hs, TLS1_3_VERSION)) {
    static_assert(
        sizeof(kTLS12DowngradeRandom) == sizeof(kTLS13DowngradeRandom),
        "downgrade signals have different size");
    static_assert(
        sizeof(kJDK11DowngradeRandom) == sizeof(kTLS13DowngradeRandom),
        "downgrade signals have different size");
    auto suffix =
        MakeConstSpan(ssl->s3->server_random, sizeof(ssl->s3->server_random))
            .subspan(SSL3_RANDOM_SIZE - sizeof(kTLS13DowngradeRandom));
    if (suffix == kTLS12DowngradeRandom || suffix == kTLS13DowngradeRandom ||
        suffix == kJDK11DowngradeRandom) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_TLS13_DOWNGRADE);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      return ssl_hs_error;
    }
  }

  // The cipher must be allowed in the selected version and enabled.
  const SSL_CIPHER *cipher = SSL_get_cipher_by_value(server_hello.cipher_suite);
  uint32_t mask_a, mask_k;
  ssl_get_client_disabled(hs, &mask_a, &mask_k);
  if (cipher == nullptr ||
      (cipher->algorithm_mkey & mask_k) ||
      (cipher->algorithm_auth & mask_a) ||
      SSL_CIPHER_get_min_version(cipher) > ssl_protocol_version(ssl) ||
      SSL_CIPHER_get_max_version(cipher) < ssl_protocol_version(ssl) ||
      !sk_SSL_CIPHER_find_awslc(SSL_get_ciphers(ssl), nullptr, cipher)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_CIPHER_RETURNED);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
    return ssl_hs_error;
  }

  hs->new_cipher = cipher;

  if (hs->session_id_len != 0 &&
      CBS_mem_equal(&server_hello.session_id, hs->session_id,
                    hs->session_id_len)) {
    // Echoing the ClientHello session ID in TLS 1.2, whether from the session
    // or a synthetic one, indicates resumption. If there was no session (or if
    // the session was only offered in ECH ClientHelloInner), this was the
    // TLS 1.3 compatibility mode session ID. As we know this is not a session
    // the server knows about, any server resuming it is in error. Reject the
    // first connection deterministicly, rather than installing an invalid
    // session into the session cache. https://crbug.com/796910
    if (ssl->session == nullptr || ssl->s3->ech_status == ssl_ech_rejected) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_SERVER_ECHOED_INVALID_SESSION_ID);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      return ssl_hs_error;
    }
    if (ssl->session->ssl_version != ssl->version) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_OLD_SESSION_VERSION_NOT_RETURNED);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      return ssl_hs_error;
    }
    if (ssl->session->cipher != hs->new_cipher) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_OLD_SESSION_CIPHER_NOT_RETURNED);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      return ssl_hs_error;
    }
    if (!ssl_session_is_context_valid(hs, ssl->session.get())) {
      // This is actually a client application bug.
      OPENSSL_PUT_ERROR(SSL,
                        SSL_R_ATTEMPT_TO_REUSE_SESSION_IN_DIFFERENT_CONTEXT);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      return ssl_hs_error;
    }
    // We never offer sessions on renegotiation.
    assert(!ssl->s3->initial_handshake_complete);
    ssl->s3->session_reused = true;
  } else {
    // The session wasn't resumed. Create a fresh SSL_SESSION to fill out.
    ssl_set_session(ssl, NULL);
    if (!ssl_get_new_session(hs)) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
      return ssl_hs_error;
    }

    // Save the session ID from the server. This may be empty if the session
    // isn't resumable, or if we'll receive a session ticket later.
    assert(CBS_len(&server_hello.session_id) <= SSL3_SESSION_ID_SIZE);
    static_assert(SSL3_SESSION_ID_SIZE <= UINT8_MAX,
                  "max session ID is too large");
    hs->new_session->session_id_length =
        static_cast<uint8_t>(CBS_len(&server_hello.session_id));
    OPENSSL_memcpy(hs->new_session->session_id,
                   CBS_data(&server_hello.session_id),
                   CBS_len(&server_hello.session_id));

    hs->new_session->cipher = hs->new_cipher;
  }

  // Now that the cipher is known, initialize the handshake hash and hash the
  // ServerHello.
  if (!hs->transcript.InitHash(ssl_protocol_version(ssl), hs->new_cipher) ||
      !ssl_hash_message(hs, msg)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
    return ssl_hs_error;
  }

  // If doing a full handshake, the server may request a client certificate
  // which requires hashing the handshake transcript. Otherwise, the handshake
  // buffer may be released.
  if (ssl->session != NULL ||
      !ssl_cipher_uses_certificate_auth(hs->new_cipher)) {
    hs->transcript.FreeBuffer();
  }

  // Only the NULL compression algorithm is supported.
  if (server_hello.compression_method != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNSUPPORTED_COMPRESSION_ALGORITHM);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
    return ssl_hs_error;
  }

  if (!ssl_parse_serverhello_tlsext(hs, &server_hello.extensions)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_PARSE_TLSEXT);
    return ssl_hs_error;
  }

  if (ssl->session != NULL &&
      hs->extended_master_secret != ssl->session->extended_master_secret) {
    if (ssl->session->extended_master_secret) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_RESUMED_EMS_SESSION_WITHOUT_EMS_EXTENSION);
    } else {
      OPENSSL_PUT_ERROR(SSL, SSL_R_RESUMED_NON_EMS_SESSION_WITH_EMS_EXTENSION);
    }
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_HANDSHAKE_FAILURE);
    return ssl_hs_error;
  }

  ssl->method->next_message(ssl);

  if (ssl->session != NULL) {
    if (ssl->ctx->reverify_on_resume &&
        ssl_cipher_uses_certificate_auth(hs->new_cipher)) {
      hs->state = state_reverify_server_certificate;
    } else {
      hs->state = state_read_session_ticket;
    }
    return ssl_hs_ok;
  }

  hs->state = state_read_server_certificate;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_tls13(SSL_HANDSHAKE *hs) {
  enum ssl_hs_wait_t wait = tls13_client_handshake(hs);
  if (wait == ssl_hs_ok) {
    hs->state = state_finish_client_handshake;
    return ssl_hs_ok;
  }

  return wait;
}

static enum ssl_hs_wait_t do_read_server_certificate(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;

  if (!ssl_cipher_uses_certificate_auth(hs->new_cipher)) {
    hs->state = state_read_certificate_status;
    return ssl_hs_ok;
  }

  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }

  if (!ssl_check_message_type(ssl, msg, SSL3_MT_CERTIFICATE) ||
      !ssl_hash_message(hs, msg)) {
    return ssl_hs_error;
  }

  CBS body = msg.body;
  uint8_t alert = SSL_AD_DECODE_ERROR;
  if (!ssl_parse_cert_chain(&alert, &hs->new_session->certs, &hs->peer_pubkey,
                            NULL, &body, ssl->ctx->pool)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
    return ssl_hs_error;
  }

  if (sk_CRYPTO_BUFFER_num(hs->new_session->certs.get()) == 0 ||
      CBS_len(&body) != 0 ||
      !ssl->ctx->x509_method->session_cache_objects(hs->new_session.get())) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    return ssl_hs_error;
  }

  if (!ssl_check_leaf_certificate(
          hs, hs->peer_pubkey.get(),
          sk_CRYPTO_BUFFER_value(hs->new_session->certs.get(), 0))) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
    return ssl_hs_error;
  }

  ssl->method->next_message(ssl);

  hs->state = state_read_certificate_status;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_read_certificate_status(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;

  if (!hs->certificate_status_expected) {
    hs->state = state_verify_server_certificate;
    return ssl_hs_ok;
  }

  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }

  if (msg.type != SSL3_MT_CERTIFICATE_STATUS) {
    // A server may send status_request in ServerHello and then change its mind
    // about sending CertificateStatus.
    hs->state = state_verify_server_certificate;
    return ssl_hs_ok;
  }

  if (!ssl_hash_message(hs, msg)) {
    return ssl_hs_error;
  }

  CBS certificate_status = msg.body, ocsp_response;
  uint8_t status_type;
  if (!CBS_get_u8(&certificate_status, &status_type) ||
      status_type != TLSEXT_STATUSTYPE_ocsp ||
      !CBS_get_u24_length_prefixed(&certificate_status, &ocsp_response) ||
      CBS_len(&ocsp_response) == 0 ||
      CBS_len(&certificate_status) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    return ssl_hs_error;
  }

  hs->new_session->ocsp_response.reset(
      CRYPTO_BUFFER_new_from_CBS(&ocsp_response, ssl->ctx->pool));
  if (hs->new_session->ocsp_response == nullptr) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
    return ssl_hs_error;
  }

  ssl->method->next_message(ssl);

  hs->state = state_verify_server_certificate;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_verify_server_certificate(SSL_HANDSHAKE *hs) {
  if (!ssl_cipher_uses_certificate_auth(hs->new_cipher)) {
    hs->state = state_read_server_key_exchange;
    return ssl_hs_ok;
  }

  switch (ssl_verify_peer_cert(hs)) {
    case ssl_verify_ok:
      break;
    case ssl_verify_invalid:
      return ssl_hs_error;
    case ssl_verify_retry:
      hs->state = state_verify_server_certificate;
      return ssl_hs_certificate_verify;
  }

  hs->state = state_read_server_key_exchange;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_reverify_server_certificate(SSL_HANDSHAKE *hs) {
  assert(hs->ssl->ctx->reverify_on_resume);

  switch (ssl_reverify_peer_cert(hs, /*send_alert=*/true)) {
    case ssl_verify_ok:
      break;
    case ssl_verify_invalid:
      return ssl_hs_error;
    case ssl_verify_retry:
      hs->state = state_reverify_server_certificate;
      return ssl_hs_certificate_verify;
  }

  hs->state = state_read_session_ticket;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_read_server_key_exchange(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }

  if (msg.type != SSL3_MT_SERVER_KEY_EXCHANGE) {
    // Some ciphers (pure PSK) have an optional ServerKeyExchange message.
    if (ssl_cipher_requires_server_key_exchange(hs->new_cipher)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_MESSAGE);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_UNEXPECTED_MESSAGE);
      return ssl_hs_error;
    }

    hs->state = state_read_certificate_request;
    return ssl_hs_ok;
  }

  if (!ssl_hash_message(hs, msg)) {
    return ssl_hs_error;
  }

  uint32_t alg_k = hs->new_cipher->algorithm_mkey;
  uint32_t alg_a = hs->new_cipher->algorithm_auth;
  CBS server_key_exchange = msg.body;
  if (alg_a & SSL_aPSK) {
    CBS psk_identity_hint;

    // Each of the PSK key exchanges begins with a psk_identity_hint.
    if (!CBS_get_u16_length_prefixed(&server_key_exchange,
                                     &psk_identity_hint)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
      return ssl_hs_error;
    }

    // Store the PSK identity hint for the ClientKeyExchange. Assume that the
    // maximum length of a PSK identity hint can be as long as the maximum
    // length of a PSK identity. Also do not allow NULL characters; identities
    // are saved as C strings.
    //
    // TODO(davidben): Should invalid hints be ignored? It's a hint rather than
    // a specific identity.
    if (CBS_len(&psk_identity_hint) > PSK_MAX_IDENTITY_LEN ||
        CBS_contains_zero_byte(&psk_identity_hint)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_DATA_LENGTH_TOO_LONG);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_HANDSHAKE_FAILURE);
      return ssl_hs_error;
    }

    // Save non-empty identity hints as a C string. Empty identity hints we
    // treat as missing. Plain PSK makes it possible to send either no hint
    // (omit ServerKeyExchange) or an empty hint, while ECDHE_PSK can only spell
    // empty hint. Having different capabilities is odd, so we interpret empty
    // and missing as identical.
    char *raw = nullptr;
    if (CBS_len(&psk_identity_hint) != 0 &&
        !CBS_strdup(&psk_identity_hint, &raw)) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
      return ssl_hs_error;
    }
    hs->peer_psk_identity_hint.reset(raw);
  }

  if (alg_k & SSL_kECDHE) {
    // Parse the server parameters.
    uint8_t group_type;
    uint16_t group_id;
    CBS point;
    if (!CBS_get_u8(&server_key_exchange, &group_type) ||
        group_type != NAMED_CURVE_TYPE ||
        !CBS_get_u16(&server_key_exchange, &group_id) ||
        !CBS_get_u8_length_prefixed(&server_key_exchange, &point)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
      return ssl_hs_error;
    }

    // Ensure the group is consistent with preferences.
    if (!tls1_check_group_id(hs, group_id)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_CURVE);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      return ssl_hs_error;
    }

    // Save the group and peer public key for later.
    hs->new_session->group_id = group_id;
    if (!ssl->s3->peer_key.CopyFrom(point)) {
      return ssl_hs_error;
    }
  } else if (!(alg_k & SSL_kPSK)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_MESSAGE);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_UNEXPECTED_MESSAGE);
    return ssl_hs_error;
  }

  // At this point, |server_key_exchange| contains the signature, if any, while
  // |msg.body| contains the entire message. From that, derive a CBS containing
  // just the parameter.
  CBS parameter;
  CBS_init(&parameter, CBS_data(&msg.body),
           CBS_len(&msg.body) - CBS_len(&server_key_exchange));

  // ServerKeyExchange should be signed by the server's public key.
  if (ssl_cipher_uses_certificate_auth(hs->new_cipher)) {
    uint16_t signature_algorithm = 0;
    if (ssl_protocol_version(ssl) >= TLS1_2_VERSION) {
      if (!CBS_get_u16(&server_key_exchange, &signature_algorithm)) {
        OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
        ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
        return ssl_hs_error;
      }
      uint8_t alert = SSL_AD_DECODE_ERROR;
      if (!tls12_check_peer_sigalg(hs, &alert, signature_algorithm)) {
        ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
        return ssl_hs_error;
      }
      hs->new_session->peer_signature_algorithm = signature_algorithm;
    } else if (!tls1_get_legacy_signature_algorithm(&signature_algorithm,
                                                    hs->peer_pubkey.get())) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_PEER_ERROR_UNSUPPORTED_CERTIFICATE_TYPE);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_UNSUPPORTED_CERTIFICATE);
      return ssl_hs_error;
    }

    // The last field in |server_key_exchange| is the signature.
    CBS signature;
    if (!CBS_get_u16_length_prefixed(&server_key_exchange, &signature) ||
        CBS_len(&server_key_exchange) != 0) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
      return ssl_hs_error;
    }

    ScopedCBB transcript;
    Array<uint8_t> transcript_data;
    if (!CBB_init(transcript.get(),
                  2 * SSL3_RANDOM_SIZE + CBS_len(&parameter)) ||
        !CBB_add_bytes(transcript.get(), ssl->s3->client_random,
                       SSL3_RANDOM_SIZE) ||
        !CBB_add_bytes(transcript.get(), ssl->s3->server_random,
                       SSL3_RANDOM_SIZE) ||
        !CBB_add_bytes(transcript.get(), CBS_data(&parameter),
                       CBS_len(&parameter)) ||
        !CBBFinishArray(transcript.get(), &transcript_data)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
      return ssl_hs_error;
    }

    if (!ssl_public_key_verify(ssl, signature, signature_algorithm,
                               hs->peer_pubkey.get(), transcript_data)) {
      // bad signature
      OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_SIGNATURE);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECRYPT_ERROR);
      return ssl_hs_error;
    }
  } else {
    // PSK ciphers are the only supported certificate-less ciphers.
    assert(alg_a == SSL_aPSK);

    if (CBS_len(&server_key_exchange) > 0) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_EXTRA_DATA_IN_MESSAGE);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
      return ssl_hs_error;
    }
  }

  ssl->method->next_message(ssl);
  hs->state = state_read_certificate_request;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_read_certificate_request(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;

  if (!ssl_cipher_uses_certificate_auth(hs->new_cipher)) {
    hs->state = state_read_server_hello_done;
    return ssl_hs_ok;
  }

  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }

  if (msg.type == SSL3_MT_SERVER_HELLO_DONE) {
    // If we get here we don't need the handshake buffer as we won't be doing
    // client auth.
    hs->transcript.FreeBuffer();
    hs->state = state_read_server_hello_done;
    return ssl_hs_ok;
  }

  if (!ssl_check_message_type(ssl, msg, SSL3_MT_CERTIFICATE_REQUEST) ||
      !ssl_hash_message(hs, msg)) {
    return ssl_hs_error;
  }

  // Get the certificate types.
  CBS body = msg.body, certificate_types;
  if (!CBS_get_u8_length_prefixed(&body, &certificate_types)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return ssl_hs_error;
  }

  if (!hs->certificate_types.CopyFrom(certificate_types)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
    return ssl_hs_error;
  }

  if (ssl_protocol_version(ssl) >= TLS1_2_VERSION) {
    CBS supported_signature_algorithms;
    if (!CBS_get_u16_length_prefixed(&body, &supported_signature_algorithms) ||
        !tls1_parse_peer_sigalgs(hs, &supported_signature_algorithms)) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
      OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
      return ssl_hs_error;
    }
  }

  uint8_t alert = SSL_AD_DECODE_ERROR;
  UniquePtr<STACK_OF(CRYPTO_BUFFER)> ca_names =
      ssl_parse_client_CA_list(ssl, &alert, &body);
  if (!ca_names) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
    return ssl_hs_error;
  }

  if (CBS_len(&body) != 0) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return ssl_hs_error;
  }

  hs->cert_request = true;
  hs->ca_names = std::move(ca_names);
  ssl->ctx->x509_method->hs_flush_cached_ca_names(hs);

  ssl->method->next_message(ssl);
  hs->state = state_read_server_hello_done;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_read_server_hello_done(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }

  if (!ssl_check_message_type(ssl, msg, SSL3_MT_SERVER_HELLO_DONE) ||
      !ssl_hash_message(hs, msg)) {
    return ssl_hs_error;
  }

  // ServerHelloDone is empty.
  if (CBS_len(&msg.body) != 0) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return ssl_hs_error;
  }

  // ServerHelloDone should be the end of the flight.
  if (ssl->method->has_unprocessed_handshake_data(ssl)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_UNEXPECTED_MESSAGE);
    OPENSSL_PUT_ERROR(SSL, SSL_R_EXCESS_HANDSHAKE_DATA);
    return ssl_hs_error;
  }

  ssl->method->next_message(ssl);
  hs->state = state_send_client_certificate;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_send_client_certificate(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;

  // The peer didn't request a certificate.
  if (!hs->cert_request) {
    hs->state = state_send_client_key_exchange;
    return ssl_hs_ok;
  }

  if (ssl->s3->ech_status == ssl_ech_rejected) {
    // Do not send client certificates on ECH reject. We have not authenticated
    // the server for the name that can learn the certificate.
    SSL_certs_clear(ssl);
  } else if (hs->config->cert->cert_cb != nullptr) {
    // Call cert_cb to update the certificate.
    int rv = hs->config->cert->cert_cb(ssl, hs->config->cert->cert_cb_arg);
    if (rv == 0) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
      OPENSSL_PUT_ERROR(SSL, SSL_R_CERT_CB_ERROR);
      return ssl_hs_error;
    }
    if (rv < 0) {
      hs->state = state_send_client_certificate;
      return ssl_hs_x509_lookup;
    }
  }

  if (!ssl_on_certificate_selected(hs)) {
    return ssl_hs_error;
  }

  if (ssl_has_certificate(hs)) {
    if (hs->config->check_client_certificate_type) {
      // Check the certificate types advertised by the peer.
      uint8_t cert_type;
      switch (EVP_PKEY_id(hs->local_pubkey.get())) {
        case EVP_PKEY_RSA:
          cert_type = SSL3_CT_RSA_SIGN;
          break;
        case EVP_PKEY_EC:
        case EVP_PKEY_ED25519:
          cert_type = TLS_CT_ECDSA_SIGN;
          break;
        default:
          OPENSSL_PUT_ERROR(SSL, SSL_R_UNKNOWN_CERTIFICATE_TYPE);
          return ssl_hs_error;
      }
      if (std::find(hs->certificate_types.begin(), hs->certificate_types.end(),
                    cert_type) == hs->certificate_types.end()) {
        OPENSSL_PUT_ERROR(SSL, SSL_R_UNKNOWN_CERTIFICATE_TYPE);
        return ssl_hs_error;
      }
    }
  } else {
    // Without a client certificate, the handshake buffer may be released.
    hs->transcript.FreeBuffer();
  }

  if (!ssl_output_cert_chain(hs)) {
    return ssl_hs_error;
  }

  hs->state = state_send_client_key_exchange;
  return ssl_hs_ok;
}

static_assert(sizeof(size_t) >= sizeof(unsigned),
              "size_t is smaller than unsigned");

static enum ssl_hs_wait_t do_send_client_key_exchange(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  ScopedCBB cbb;
  CBB body;
  if (!ssl->method->init_message(ssl, cbb.get(), &body,
                                 SSL3_MT_CLIENT_KEY_EXCHANGE)) {
    return ssl_hs_error;
  }

  Array<uint8_t> pms;
  uint32_t alg_k = hs->new_cipher->algorithm_mkey;
  uint32_t alg_a = hs->new_cipher->algorithm_auth;
  if (ssl_cipher_uses_certificate_auth(hs->new_cipher)) {
    const CRYPTO_BUFFER *leaf =
        sk_CRYPTO_BUFFER_value(hs->new_session->certs.get(), 0);
    CBS leaf_cbs;
    CRYPTO_BUFFER_init_CBS(leaf, &leaf_cbs);

    // Check the key usage matches the cipher suite. We do this unconditionally
    // for non-RSA certificates. In particular, it's needed to distinguish ECDH
    // certificates, which we do not support, from ECDSA certificates.
    // Historically, we have not checked RSA key usages, so it is controlled by
    // a flag for now. See https://crbug.com/795089.
    ssl_key_usage_t intended_use = (alg_k & SSL_kRSA)
                                       ? key_usage_encipherment
                                       : key_usage_digital_signature;
    if (!ssl_cert_check_key_usage(&leaf_cbs, intended_use)) {
      if (hs->config->enforce_rsa_key_usage ||
          EVP_PKEY_id(hs->peer_pubkey.get()) != EVP_PKEY_RSA) {
        return ssl_hs_error;
      }
      ERR_clear_error();
      ssl->s3->was_key_usage_invalid = true;
    }
  }

  // If using a PSK key exchange, prepare the pre-shared key.
  unsigned psk_len = 0;
  uint8_t psk[PSK_MAX_PSK_LEN];
  if (alg_a & SSL_aPSK) {
    if (hs->config->psk_client_callback == NULL) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_PSK_NO_CLIENT_CB);
      return ssl_hs_error;
    }

    char identity[PSK_MAX_IDENTITY_LEN + 1];
    OPENSSL_memset(identity, 0, sizeof(identity));
    psk_len = hs->config->psk_client_callback(
        ssl, hs->peer_psk_identity_hint.get(), identity, sizeof(identity), psk,
        sizeof(psk));
    if (psk_len == 0) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_PSK_IDENTITY_NOT_FOUND);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_HANDSHAKE_FAILURE);
      return ssl_hs_error;
    }
    assert(psk_len <= PSK_MAX_PSK_LEN);

    hs->new_session->psk_identity.reset(OPENSSL_strdup(identity));
    if (hs->new_session->psk_identity == nullptr) {
      return ssl_hs_error;
    }

    // Write out psk_identity.
    CBB child;
    if (!CBB_add_u16_length_prefixed(&body, &child) ||
        !CBB_add_bytes(&child, (const uint8_t *)identity,
                       OPENSSL_strnlen(identity, sizeof(identity))) ||
        !CBB_flush(&body)) {
      return ssl_hs_error;
    }
  }

  // Depending on the key exchange method, compute |pms|.
  if (alg_k & SSL_kRSA) {
    if (!pms.Init(SSL_MAX_MASTER_KEY_LENGTH)) {
      return ssl_hs_error;
    }

    RSA *rsa = EVP_PKEY_get0_RSA(hs->peer_pubkey.get());
    if (rsa == NULL) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return ssl_hs_error;
    }

    pms[0] = hs->client_version >> 8;
    pms[1] = hs->client_version & 0xff;
    if (!RAND_bytes(&pms[2], SSL_MAX_MASTER_KEY_LENGTH - 2)) {
      return ssl_hs_error;
    }

    CBB enc_pms;
    uint8_t *ptr;
    size_t enc_pms_len;
    if (!CBB_add_u16_length_prefixed(&body, &enc_pms) ||
        !CBB_reserve(&enc_pms, &ptr, RSA_size(rsa)) ||
        !RSA_encrypt(rsa, &enc_pms_len, ptr, RSA_size(rsa), pms.data(),
                     pms.size(), RSA_PKCS1_PADDING) ||
        !CBB_did_write(&enc_pms, enc_pms_len) ||
        !CBB_flush(&body)) {
      return ssl_hs_error;
    }
  } else if (alg_k & SSL_kECDHE) {
    CBB child;
    if (!CBB_add_u8_length_prefixed(&body, &child)) {
      return ssl_hs_error;
    }

    // Generate the premaster secret.
    bssl::UniquePtr<SSLKeyShare> key_share =
        SSLKeyShare::Create(hs->new_session->group_id);
    uint8_t alert = SSL_AD_DECODE_ERROR;
    if (!key_share ||
        !key_share->Accept(&child, &pms, &alert, ssl->s3->peer_key)) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
      return ssl_hs_error;
    }
    if (!CBB_flush(&body)) {
      return ssl_hs_error;
    }

    // The peer key could be discarded, but we preserve it since OpenSSL
    // allows the user to observe it with |SSL_get_peer_tmp_key|.
  } else if (alg_k & SSL_kPSK) {
    // For plain PSK, other_secret is a block of 0s with the same length as
    // the pre-shared key.
    if (!pms.Init(psk_len)) {
      return ssl_hs_error;
    }
    OPENSSL_memset(pms.data(), 0, pms.size());
  } else {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_HANDSHAKE_FAILURE);
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return ssl_hs_error;
  }

  // For a PSK cipher suite, other_secret is combined with the pre-shared
  // key.
  if (alg_a & SSL_aPSK) {
    ScopedCBB pms_cbb;
    CBB child;
    if (!CBB_init(pms_cbb.get(), 2 + psk_len + 2 + pms.size()) ||
        !CBB_add_u16_length_prefixed(pms_cbb.get(), &child) ||
        !CBB_add_bytes(&child, pms.data(), pms.size()) ||
        !CBB_add_u16_length_prefixed(pms_cbb.get(), &child) ||
        !CBB_add_bytes(&child, psk, psk_len) ||
        !CBBFinishArray(pms_cbb.get(), &pms)) {
      return ssl_hs_error;
    }
  }

  // The message must be added to the finished hash before calculating the
  // master secret.
  if (!ssl_add_message_cbb(ssl, cbb.get())) {
    return ssl_hs_error;
  }

  hs->new_session->secret_length =
      tls1_generate_master_secret(hs, hs->new_session->secret, pms);
  if (hs->new_session->secret_length == 0) {
    return ssl_hs_error;
  }
  hs->new_session->extended_master_secret = hs->extended_master_secret;

  hs->state = state_send_client_certificate_verify;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_send_client_certificate_verify(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;

  if (!hs->cert_request || !ssl_has_certificate(hs)) {
    hs->state = state_send_client_finished;
    return ssl_hs_ok;
  }

  assert(ssl_has_private_key(hs));
  ScopedCBB cbb;
  CBB body, child;
  if (!ssl->method->init_message(ssl, cbb.get(), &body,
                                 SSL3_MT_CERTIFICATE_VERIFY)) {
    return ssl_hs_error;
  }

  if (!tls1_choose_signature_algorithm(hs, &hs->signature_algorithm)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_HANDSHAKE_FAILURE);
    return ssl_hs_error;
  }
  if (ssl_protocol_version(ssl) >= TLS1_2_VERSION) {
    // Write out the digest type in TLS 1.2.
    if (!CBB_add_u16(&body, hs->signature_algorithm)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return ssl_hs_error;
    }
  }

  // Set aside space for the signature.
  const size_t max_sig_len = EVP_PKEY_size(hs->local_pubkey.get());
  uint8_t *ptr;
  if (!CBB_add_u16_length_prefixed(&body, &child) ||
      !CBB_reserve(&child, &ptr, max_sig_len)) {
    return ssl_hs_error;
  }

  size_t sig_len = max_sig_len;
  switch (ssl_private_key_sign(hs, ptr, &sig_len, max_sig_len,
                               hs->signature_algorithm,
                               hs->transcript.buffer())) {
    case ssl_private_key_success:
      break;
    case ssl_private_key_failure:
      return ssl_hs_error;
    case ssl_private_key_retry:
      hs->state = state_send_client_certificate_verify;
      return ssl_hs_private_key_operation;
  }

  if (!CBB_did_write(&child, sig_len) ||
      !ssl_add_message_cbb(ssl, cbb.get())) {
    return ssl_hs_error;
  }

  // The handshake buffer is no longer necessary.
  hs->transcript.FreeBuffer();

  hs->state = state_send_client_finished;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_send_client_finished(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  hs->can_release_private_key = true;
  if (!ssl->method->add_change_cipher_spec(ssl) ||
      !tls1_change_cipher_state(hs, evp_aead_seal)) {
    return ssl_hs_error;
  }

  if (hs->next_proto_neg_seen) {
    static const uint8_t kZero[32] = {0};
    size_t padding_len =
        32 - ((ssl->s3->next_proto_negotiated.size() + 2) % 32);

    ScopedCBB cbb;
    CBB body, child;
    if (!ssl->method->init_message(ssl, cbb.get(), &body, SSL3_MT_NEXT_PROTO) ||
        !CBB_add_u8_length_prefixed(&body, &child) ||
        !CBB_add_bytes(&child, ssl->s3->next_proto_negotiated.data(),
                       ssl->s3->next_proto_negotiated.size()) ||
        !CBB_add_u8_length_prefixed(&body, &child) ||
        !CBB_add_bytes(&child, kZero, padding_len) ||
        !ssl_add_message_cbb(ssl, cbb.get())) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return ssl_hs_error;
    }
  }

  if (hs->channel_id_negotiated) {
    ScopedCBB cbb;
    CBB body;
    if (!ssl->method->init_message(ssl, cbb.get(), &body, SSL3_MT_CHANNEL_ID) ||
        !tls1_write_channel_id(hs, &body) ||
        !ssl_add_message_cbb(ssl, cbb.get())) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return ssl_hs_error;
    }
  }

  if (!ssl_send_finished(hs)) {
    return ssl_hs_error;
  }

  hs->state = state_finish_flight;
  return ssl_hs_flush;
}

static bool can_false_start(const SSL_HANDSHAKE *hs) {
  const SSL *const ssl = hs->ssl;

  // False Start bypasses the Finished check's downgrade protection. This can
  // enable attacks where we send data under weaker settings than supported
  // (e.g. the Logjam attack). Thus we require TLS 1.2 with an ECDHE+AEAD
  // cipher, our strongest settings before TLS 1.3.
  //
  // Now that TLS 1.3 exists, we would like to avoid similar attacks between
  // TLS 1.2 and TLS 1.3, but there are too many TLS 1.2 deployments to
  // sacrifice False Start on them. Instead, we rely on the ServerHello.random
  // downgrade signal, which we unconditionally enforce.
  if (SSL_is_dtls(ssl) ||
      SSL_version(ssl) != TLS1_2_VERSION ||
      hs->new_cipher->algorithm_mkey != SSL_kECDHE ||
      hs->new_cipher->algorithm_mac != SSL_AEAD) {
    return false;
  }

  // If ECH was rejected, disable False Start. We run the handshake to
  // completion, including the Finished downgrade check, to authenticate the
  // recovery flow.
  if (ssl->s3->ech_status == ssl_ech_rejected) {
    return false;
  }

  // Additionally require ALPN or NPN by default.
  //
  // TODO(davidben): Can this constraint be relaxed globally now that cipher
  // suite requirements have been tightened?
  if (!ssl->ctx->false_start_allowed_without_alpn &&
      ssl->s3->alpn_selected.empty() &&
      ssl->s3->next_proto_negotiated.empty()) {
    return false;
  }

  return true;
}

static enum ssl_hs_wait_t do_finish_flight(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  if (ssl->session != NULL) {
    hs->state = state_finish_client_handshake;
    return ssl_hs_ok;
  }

  // This is a full handshake. If it involves ChannelID, then record the
  // handshake hashes at this point in the session so that any resumption of
  // this session with ChannelID can sign those hashes.
  if (!tls1_record_handshake_hashes_for_channel_id(hs)) {
    return ssl_hs_error;
  }

  hs->state = state_read_session_ticket;

  if ((SSL_get_mode(ssl) & SSL_MODE_ENABLE_FALSE_START) &&
      can_false_start(hs) &&
      // No False Start on renegotiation (would complicate the state machine).
      !ssl->s3->initial_handshake_complete) {
    hs->in_false_start = true;
    hs->can_early_write = true;
    return ssl_hs_early_return;
  }

  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_read_session_ticket(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;

  if (!hs->ticket_expected) {
    hs->state = state_process_change_cipher_spec;
    return ssl_hs_read_change_cipher_spec;
  }

  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }

  if (!ssl_check_message_type(ssl, msg, SSL3_MT_NEW_SESSION_TICKET) ||
      !ssl_hash_message(hs, msg)) {
    return ssl_hs_error;
  }

  CBS new_session_ticket = msg.body, ticket;
  uint32_t ticket_lifetime_hint;
  if (!CBS_get_u32(&new_session_ticket, &ticket_lifetime_hint) ||
      !CBS_get_u16_length_prefixed(&new_session_ticket, &ticket) ||
      CBS_len(&new_session_ticket) != 0) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return ssl_hs_error;
  }

  if (CBS_len(&ticket) == 0) {
    // RFC 5077 allows a server to change its mind and send no ticket after
    // negotiating the extension. The value of |ticket_expected| is checked in
    // |ssl_update_cache| so is cleared here to avoid an unnecessary update.
    hs->ticket_expected = false;
    ssl->method->next_message(ssl);
    hs->state = state_process_change_cipher_spec;
    return ssl_hs_read_change_cipher_spec;
  }

  if (ssl->session != nullptr) {
    // The server is sending a new ticket for an existing session. Sessions are
    // immutable once established, so duplicate all but the ticket of the
    // existing session.
    assert(!hs->new_session);
    hs->new_session =
        SSL_SESSION_dup(ssl->session.get(), SSL_SESSION_INCLUDE_NONAUTH);
    if (!hs->new_session) {
      return ssl_hs_error;
    }
  }

  ssl->verify_result = hs->new_session->verify_result;

  // |ticket_lifetime_hint| is measured from when the ticket was issued.
  ssl_session_rebase_time(ssl, hs->new_session.get());

  if (!hs->new_session->ticket.CopyFrom(ticket)) {
    return ssl_hs_error;
  }
  hs->new_session->ticket_lifetime_hint = ticket_lifetime_hint;

  // Historically, OpenSSL filled in fake session IDs for ticket-based sessions.
  // TODO(davidben): Are external callers relying on this? Try removing this.
  SHA256(CBS_data(&ticket), CBS_len(&ticket), hs->new_session->session_id);
  hs->new_session->session_id_length = SHA256_DIGEST_LENGTH;

  ssl->method->next_message(ssl);
  hs->state = state_process_change_cipher_spec;
  return ssl_hs_read_change_cipher_spec;
}

static enum ssl_hs_wait_t do_process_change_cipher_spec(SSL_HANDSHAKE *hs) {
  if (!tls1_change_cipher_state(hs, evp_aead_open)) {
    return ssl_hs_error;
  }

  hs->state = state_read_server_finished;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_read_server_finished(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  enum ssl_hs_wait_t wait = ssl_get_finished(hs);
  if (wait != ssl_hs_ok) {
    return wait;
  }

  if (ssl->session != NULL) {
    hs->state = state_send_client_finished;
    return ssl_hs_ok;
  }

  hs->state = state_finish_client_handshake;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_finish_client_handshake(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  if (ssl->s3->ech_status == ssl_ech_rejected) {
    // Release the retry configs.
    hs->ech_authenticated_reject = true;
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ECH_REQUIRED);
    OPENSSL_PUT_ERROR(SSL, SSL_R_ECH_REJECTED);
    return ssl_hs_error;
  }

  ssl->method->on_handshake_complete(ssl);

  // Note TLS 1.2 resumptions with ticket renewal have both |ssl->session| (the
  // resumed session) and |hs->new_session| (the session with the new ticket).
  bool has_new_session = hs->new_session != nullptr;
  if (has_new_session) {
    // When False Start is enabled, the handshake reports completion early. The
    // caller may then have passed the (then unresuable) |hs->new_session| to
    // another thread via |SSL_get0_session| for resumption. To avoid potential
    // race conditions in such callers, we duplicate the session before
    // clearing |not_resumable|.
    ssl->s3->established_session =
        SSL_SESSION_dup(hs->new_session.get(), SSL_SESSION_DUP_ALL);
    if (!ssl->s3->established_session) {
      return ssl_hs_error;
    }
    // Renegotiations do not participate in session resumption.
    if (!ssl->s3->initial_handshake_complete) {
      ssl->s3->established_session->not_resumable = false;
    }

    hs->new_session.reset();
  } else {
    assert(ssl->session != nullptr);
    ssl->s3->established_session = UpRef(ssl->session);
  }

  hs->handshake_finalized = true;
  ssl->s3->initial_handshake_complete = true;
  if (has_new_session) {
    ssl_update_cache(ssl);
  }

  if (ssl->s3->session_reused) {
    // Update client side session hit counter on successfully reused sessions.
    ssl_update_counter(ssl->session_ctx.get(), ssl->session_ctx->stats.sess_hit,
                       true);
  }

  hs->state = state_done;
  return ssl_hs_ok;
}

enum ssl_hs_wait_t ssl_client_handshake(SSL_HANDSHAKE *hs) {
  while (hs->state != state_done) {
    enum ssl_hs_wait_t ret = ssl_hs_error;
    enum ssl_client_hs_state_t state =
        static_cast<enum ssl_client_hs_state_t>(hs->state);
    switch (state) {
      case state_start_connect:
        ret = do_start_connect(hs);
        break;
      case state_enter_early_data:
        ret = do_enter_early_data(hs);
        break;
      case state_early_reverify_server_certificate:
        ret = do_early_reverify_server_certificate(hs);
        break;
      case state_read_hello_verify_request:
        ret = do_read_hello_verify_request(hs);
        break;
      case state_read_server_hello:
        ret = do_read_server_hello(hs);
        break;
      case state_tls13:
        ret = do_tls13(hs);
        break;
      case state_read_server_certificate:
        ret = do_read_server_certificate(hs);
        break;
      case state_read_certificate_status:
        ret = do_read_certificate_status(hs);
        break;
      case state_verify_server_certificate:
        ret = do_verify_server_certificate(hs);
        break;
      case state_reverify_server_certificate:
        ret = do_reverify_server_certificate(hs);
        break;
      case state_read_server_key_exchange:
        ret = do_read_server_key_exchange(hs);
        break;
      case state_read_certificate_request:
        ret = do_read_certificate_request(hs);
        break;
      case state_read_server_hello_done:
        ret = do_read_server_hello_done(hs);
        break;
      case state_send_client_certificate:
        ret = do_send_client_certificate(hs);
        break;
      case state_send_client_key_exchange:
        ret = do_send_client_key_exchange(hs);
        break;
      case state_send_client_certificate_verify:
        ret = do_send_client_certificate_verify(hs);
        break;
      case state_send_client_finished:
        ret = do_send_client_finished(hs);
        break;
      case state_finish_flight:
        ret = do_finish_flight(hs);
        break;
      case state_read_session_ticket:
        ret = do_read_session_ticket(hs);
        break;
      case state_process_change_cipher_spec:
        ret = do_process_change_cipher_spec(hs);
        break;
      case state_read_server_finished:
        ret = do_read_server_finished(hs);
        break;
      case state_finish_client_handshake:
        ret = do_finish_client_handshake(hs);
        break;
      case state_done:
        ret = ssl_hs_ok;
        break;
    }

    if (hs->state != state) {
      ssl_do_info_callback(hs->ssl, SSL_CB_CONNECT_LOOP, 1);
    }

    if (ret != ssl_hs_ok) {
      return ret;
    }
  }

  ssl_update_counter(hs->ssl->session_ctx.get(),
                     hs->ssl->session_ctx->stats.sess_connect_good, true);
  ssl_do_info_callback(hs->ssl, SSL_CB_HANDSHAKE_DONE, 1);
  return ssl_hs_ok;
}

const char *ssl_client_handshake_state(SSL_HANDSHAKE *hs) {
  enum ssl_client_hs_state_t state =
      static_cast<enum ssl_client_hs_state_t>(hs->state);
  switch (state) {
    case state_start_connect:
      return "TLS client start_connect";
    case state_enter_early_data:
      return "TLS client enter_early_data";
    case state_early_reverify_server_certificate:
      return "TLS client early_reverify_server_certificate";
    case state_read_hello_verify_request:
      return "TLS client read_hello_verify_request";
    case state_read_server_hello:
      return "TLS client read_server_hello";
    case state_tls13:
      return tls13_client_handshake_state(hs);
    case state_read_server_certificate:
      return "TLS client read_server_certificate";
    case state_read_certificate_status:
      return "TLS client read_certificate_status";
    case state_verify_server_certificate:
      return "TLS client verify_server_certificate";
    case state_reverify_server_certificate:
      return "TLS client reverify_server_certificate";
    case state_read_server_key_exchange:
      return "TLS client read_server_key_exchange";
    case state_read_certificate_request:
      return "TLS client read_certificate_request";
    case state_read_server_hello_done:
      return "TLS client read_server_hello_done";
    case state_send_client_certificate:
      return "TLS client send_client_certificate";
    case state_send_client_key_exchange:
      return "TLS client send_client_key_exchange";
    case state_send_client_certificate_verify:
      return "TLS client send_client_certificate_verify";
    case state_send_client_finished:
      return "TLS client send_client_finished";
    case state_finish_flight:
      return "TLS client finish_flight";
    case state_read_session_ticket:
      return "TLS client read_session_ticket";
    case state_process_change_cipher_spec:
      return "TLS client process_change_cipher_spec";
    case state_read_server_finished:
      return "TLS client read_server_finished";
    case state_finish_client_handshake:
      return "TLS client finish_client_handshake";
    case state_done:
      return "TLS client done";
  }

  return "TLS client unknown";
}

BSSL_NAMESPACE_END
