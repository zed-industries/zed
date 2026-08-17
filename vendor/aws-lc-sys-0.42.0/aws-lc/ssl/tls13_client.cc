// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/ssl.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <utility>

#include <openssl/bytestring.h>
#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/sha.h>
#include <openssl/stack.h>

#include "../crypto/internal.h"
#include "internal.h"


BSSL_NAMESPACE_BEGIN

enum client_hs_state_t {
  state_read_hello_retry_request = 0,
  state_send_second_client_hello,
  state_read_server_hello,
  state_read_encrypted_extensions,
  state_read_certificate_request,
  state_read_server_certificate,
  state_read_server_certificate_verify,
  state_server_certificate_reverify,
  state_read_server_finished,
  state_send_end_of_early_data,
  state_send_client_encrypted_extensions,
  state_send_client_certificate,
  state_send_client_certificate_verify,
  state_complete_second_flight,
  state_done,
};

static const uint8_t kZeroes[EVP_MAX_MD_SIZE] = {0};

// end_of_early_data closes the early data stream for |hs| and switches the
// encryption level to |level|. It returns true on success and false on error.
static bool close_early_data(SSL_HANDSHAKE *hs, ssl_encryption_level_t level) {
  SSL *const ssl = hs->ssl;
  assert(hs->in_early_data);

  // Note |can_early_write| may already be false if |SSL_write| exceeded the
  // early data write limit.
  hs->can_early_write = false;

  // 0-RTT write states on the client differ between TLS 1.3, DTLS 1.3, and
  // QUIC. TLS 1.3 has one write encryption level at a time. 0-RTT write keys
  // overwrite the null cipher and defer handshake write keys. While a
  // HelloRetryRequest can cause us to rewind back to the null cipher, sequence
  // numbers have no effect, so we can install a "new" null cipher.
  //
  // In QUIC and DTLS 1.3, 0-RTT write state cannot override or defer the normal
  // write state. The two ClientHello sequence numbers must align, and handshake
  // write keys must be installed early to ACK the EncryptedExtensions.
  //
  // We do not currently implement DTLS 1.3 and, in QUIC, the caller handles
  // 0-RTT data, so we can skip installing 0-RTT keys and act as if there is one
  // write level. If we implement DTLS 1.3, we'll need to model this better.
  if (ssl->quic_method == nullptr) {
    if (level == ssl_encryption_initial) {
      bssl::UniquePtr<SSLAEADContext> null_ctx =
          SSLAEADContext::CreateNullCipher(SSL_is_dtls(ssl));
      if (!null_ctx ||
          !ssl->method->set_write_state(ssl, ssl_encryption_initial,
                                        std::move(null_ctx),
                                        /*secret_for_quic=*/{})) {
        return false;
      }
      ssl->s3->aead_write_ctx->SetVersionIfNullCipher(ssl->version);
    } else {
      assert(level == ssl_encryption_handshake);
      if (!tls13_set_traffic_key(ssl, ssl_encryption_handshake, evp_aead_seal,
                                 hs->new_session.get(),
                                 hs->client_handshake_secret())) {
        return false;
      }
    }
  }

  assert(ssl->s3->write_level == level);
  return true;
}

static bool parse_server_hello_tls13(const SSL_HANDSHAKE *hs,
                                     ParsedServerHello *out, uint8_t *out_alert,
                                     const SSLMessage &msg) {
  if (!ssl_parse_server_hello(out, out_alert, msg)) {
    return false;
  }
  // The RFC8446 version of the structure fixes some legacy values.
  // Additionally, the session ID must echo the original one.
  if (out->legacy_version != TLS1_2_VERSION ||
      out->compression_method != 0 ||
      !CBS_mem_equal(&out->session_id, hs->session_id, hs->session_id_len) ||
      CBS_len(&out->extensions) == 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    *out_alert = SSL_AD_DECODE_ERROR;
    return false;
  }
  return true;
}

static bool is_hello_retry_request(const ParsedServerHello &server_hello) {
  return Span<const uint8_t>(server_hello.random) == kHelloRetryRequest;
}

static bool check_ech_confirmation(const SSL_HANDSHAKE *hs, bool *out_accepted,
                                   uint8_t *out_alert,
                                   const ParsedServerHello &server_hello) {
  const bool is_hrr = is_hello_retry_request(server_hello);
  size_t offset;
  if (is_hrr) {
    // We check for an unsolicited extension when parsing all of them.
    SSLExtension ech(TLSEXT_TYPE_encrypted_client_hello);
    if (!ssl_parse_extensions(&server_hello.extensions, out_alert, {&ech},
                              /*ignore_unknown=*/true)) {
      return false;
    }
    if (!ech.present) {
      *out_accepted = false;
      return true;
    }
    if (CBS_len(&ech.data) != ECH_CONFIRMATION_SIGNAL_LEN) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
      *out_alert = SSL_AD_DECODE_ERROR;
      return false;
    }
    offset = CBS_data(&ech.data) - CBS_data(&server_hello.raw);
  } else {
    offset = ssl_ech_confirmation_signal_hello_offset(hs->ssl);
  }

  if (!hs->selected_ech_config) {
    *out_accepted = false;
    return true;
  }

  uint8_t expected[ECH_CONFIRMATION_SIGNAL_LEN];
  if (!ssl_ech_accept_confirmation(hs, expected, hs->inner_client_random,
                                   hs->inner_transcript, is_hrr,
                                   server_hello.raw, offset)) {
    *out_alert = SSL_AD_INTERNAL_ERROR;
    return false;
  }

  *out_accepted = CRYPTO_memcmp(CBS_data(&server_hello.raw) + offset, expected,
                                sizeof(expected)) == 0;
  return true;
}

static enum ssl_hs_wait_t do_read_hello_retry_request(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  assert(ssl->s3->have_version);
  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }

  // Queue up a ChangeCipherSpec for whenever we next send something. This
  // will be before the second ClientHello. If we offered early data, this was
  // already done.
  if (!hs->early_data_offered &&
      !ssl->method->add_change_cipher_spec(ssl)) {
    return ssl_hs_error;
  }

  ParsedServerHello server_hello;
  uint8_t alert = SSL_AD_DECODE_ERROR;
  if (!parse_server_hello_tls13(hs, &server_hello, &alert, msg)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
    return ssl_hs_error;
  }

  // The cipher suite must be one we offered. We currently offer all supported
  // TLS 1.3 ciphers, so check the version.
  const SSL_CIPHER *cipher = SSL_get_cipher_by_value(server_hello.cipher_suite);
  if (cipher == nullptr ||
      SSL_CIPHER_get_min_version(cipher) > ssl_protocol_version(ssl) ||
      SSL_CIPHER_get_max_version(cipher) < ssl_protocol_version(ssl)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_CIPHER_RETURNED);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
    return ssl_hs_error;
  }

  hs->new_cipher = cipher;

  const bool is_hrr = is_hello_retry_request(server_hello);
  if (!hs->transcript.InitHash(ssl_protocol_version(ssl), hs->new_cipher) ||
      (is_hrr && !hs->transcript.UpdateForHelloRetryRequest())) {
    return ssl_hs_error;
  }
  if (hs->selected_ech_config) {
    if (!hs->inner_transcript.InitHash(ssl_protocol_version(ssl),
                                       hs->new_cipher) ||
        (is_hrr && !hs->inner_transcript.UpdateForHelloRetryRequest())) {
      return ssl_hs_error;
    }
  }

  // Determine which ClientHello the server is responding to. Run
  // |check_ech_confirmation| unconditionally, so we validate the extension
  // contents.
  bool ech_accepted;
  if (!check_ech_confirmation(hs, &ech_accepted, &alert, server_hello)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
    return ssl_hs_error;
  }
  if (hs->selected_ech_config) {
    ssl->s3->ech_status = ech_accepted ? ssl_ech_accepted : ssl_ech_rejected;
  }

  if (!is_hrr) {
    hs->tls13_state = state_read_server_hello;
    return ssl_hs_ok;
  }

  // The ECH extension, if present, was already parsed by
  // |check_ech_confirmation|.
  SSLExtension cookie(TLSEXT_TYPE_cookie), key_share(TLSEXT_TYPE_key_share),
      supported_versions(TLSEXT_TYPE_supported_versions),
      ech_unused(TLSEXT_TYPE_encrypted_client_hello,
                 hs->selected_ech_config || hs->config->ech_grease_enabled);
  if (!ssl_parse_extensions(
          &server_hello.extensions, &alert,
          {&cookie, &key_share, &supported_versions, &ech_unused},
          /*ignore_unknown=*/false)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
    return ssl_hs_error;
  }

  if (!cookie.present && !key_share.present) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_EMPTY_HELLO_RETRY_REQUEST);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
    return ssl_hs_error;
  }
  if (cookie.present) {
    CBS cookie_value;
    if (!CBS_get_u16_length_prefixed(&cookie.data, &cookie_value) ||
        CBS_len(&cookie_value) == 0 ||
        CBS_len(&cookie.data) != 0) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
      return ssl_hs_error;
    }

    if (!hs->cookie.CopyFrom(cookie_value)) {
      return ssl_hs_error;
    }
  }

  if (key_share.present) {
    uint16_t group_id;
    if (!CBS_get_u16(&key_share.data, &group_id) ||
        CBS_len(&key_share.data) != 0) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
      return ssl_hs_error;
    }

    // The group must be supported.
    if (!tls1_check_group_id(hs, group_id)) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_CURVE);
      return ssl_hs_error;
    }

    // Check that the HelloRetryRequest does not request a key share that was
    // provided in the initial ClientHello.
    if (hs->key_shares[0]->GroupID() == group_id ||
        (hs->key_shares[1] && hs->key_shares[1]->GroupID() == group_id)) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_CURVE);
      return ssl_hs_error;
    }

    if (!ssl_setup_key_shares(hs, group_id)) {
      return ssl_hs_error;
    }
  }

  // Although we now know whether ClientHelloInner was used, we currently
  // maintain both transcripts up to ServerHello. We could swap transcripts
  // early, but then ClientHello construction and |check_ech_confirmation|
  // become more complex.
  if (!ssl_hash_message(hs, msg)) {
    return ssl_hs_error;
  }
  if (ssl->s3->ech_status == ssl_ech_accepted &&
      !hs->inner_transcript.Update(msg.raw)) {
    return ssl_hs_error;
  }

  // HelloRetryRequest should be the end of the flight.
  if (ssl->method->has_unprocessed_handshake_data(ssl)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_UNEXPECTED_MESSAGE);
    OPENSSL_PUT_ERROR(SSL, SSL_R_EXCESS_HANDSHAKE_DATA);
    return ssl_hs_error;
  }

  ssl->method->next_message(ssl);
  ssl->s3->used_hello_retry_request = true;
  hs->tls13_state = state_send_second_client_hello;
  // 0-RTT is rejected if we receive a HelloRetryRequest.
  if (hs->in_early_data) {
    ssl->s3->early_data_reason = ssl_early_data_hello_retry_request;
    if (!close_early_data(hs, ssl_encryption_initial)) {
      return ssl_hs_error;
    }
    return ssl_hs_early_data_rejected;
  }
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_send_second_client_hello(SSL_HANDSHAKE *hs) {
  // Any 0-RTT keys must have been discarded.
  assert(hs->ssl->s3->write_level == ssl_encryption_initial);

  // Build the second ClientHelloInner, if applicable. The second ClientHello
  // uses an empty string for |enc|.
  if (hs->ssl->s3->ech_status == ssl_ech_accepted &&
      !ssl_encrypt_client_hello(hs, {})) {
    return ssl_hs_error;
  }

  if (!ssl_add_client_hello(hs)) {
    return ssl_hs_error;
  }

  ssl_done_writing_client_hello(hs);
  hs->tls13_state = state_read_server_hello;
  return ssl_hs_flush;
}

static enum ssl_hs_wait_t do_read_server_hello(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }
  ParsedServerHello server_hello;
  uint8_t alert = SSL_AD_DECODE_ERROR;
  if (!parse_server_hello_tls13(hs, &server_hello, &alert, msg)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
    return ssl_hs_error;
  }

  // Forbid a second HelloRetryRequest.
  if (is_hello_retry_request(server_hello)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_UNEXPECTED_MESSAGE);
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_MESSAGE);
    return ssl_hs_error;
  }

  // Check the cipher suite, in case this is after HelloRetryRequest.
  if (SSL_CIPHER_get_protocol_id(hs->new_cipher) != server_hello.cipher_suite) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_CIPHER_RETURNED);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
    return ssl_hs_error;
  }

  if (ssl->s3->ech_status == ssl_ech_accepted) {
    if (ssl->s3->used_hello_retry_request) {
      // HelloRetryRequest and ServerHello must accept ECH consistently.
      bool ech_accepted;
      if (!check_ech_confirmation(hs, &ech_accepted, &alert, server_hello)) {
        ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
        return ssl_hs_error;
      }
      if (!ech_accepted) {
        OPENSSL_PUT_ERROR(SSL, SSL_R_INCONSISTENT_ECH_NEGOTIATION);
        ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
        return ssl_hs_error;
      }
    }

    hs->transcript = std::move(hs->inner_transcript);
    hs->extensions.sent = hs->inner_extensions_sent;
    // Report the inner random value through |SSL_get_client_random|.
    OPENSSL_memcpy(ssl->s3->client_random, hs->inner_client_random,
                   SSL3_RANDOM_SIZE);
  }

  OPENSSL_memcpy(ssl->s3->server_random, CBS_data(&server_hello.random),
                 SSL3_RANDOM_SIZE);

  // When offering ECH, |ssl->session| is only offered in ClientHelloInner.
  const bool pre_shared_key_allowed =
      ssl->session != nullptr && ssl->s3->ech_status != ssl_ech_rejected;
  SSLExtension key_share(TLSEXT_TYPE_key_share),
      pre_shared_key(TLSEXT_TYPE_pre_shared_key, pre_shared_key_allowed),
      supported_versions(TLSEXT_TYPE_supported_versions);
  if (!ssl_parse_extensions(&server_hello.extensions, &alert,
                            {&key_share, &pre_shared_key, &supported_versions},
                            /*ignore_unknown=*/false)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
    return ssl_hs_error;
  }

  // Recheck supported_versions, in case this is after HelloRetryRequest.
  uint16_t version;
  if (!supported_versions.present ||
      !CBS_get_u16(&supported_versions.data, &version) ||
      CBS_len(&supported_versions.data) != 0 || version != ssl->version) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SECOND_SERVERHELLO_VERSION_MISMATCH);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
    return ssl_hs_error;
  }

  alert = SSL_AD_DECODE_ERROR;
  if (pre_shared_key.present) {
    if (!ssl_ext_pre_shared_key_parse_serverhello(hs, &alert,
                                                  &pre_shared_key.data)) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
      return ssl_hs_error;
    }

    if (ssl->session->ssl_version != ssl->version) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_OLD_SESSION_VERSION_NOT_RETURNED);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      return ssl_hs_error;
    }

    if (ssl->session->cipher->algorithm_prf != hs->new_cipher->algorithm_prf) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_OLD_SESSION_PRF_HASH_MISMATCH);
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

    ssl->s3->session_reused = true;
    hs->can_release_private_key = true;
    // Only authentication information carries over in TLS 1.3.
    hs->new_session =
        SSL_SESSION_dup(ssl->session.get(), SSL_SESSION_DUP_AUTH_ONLY);
    if (!hs->new_session) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
      return ssl_hs_error;
    }
    ssl_set_session(ssl, NULL);

    // Resumption incorporates fresh key material, so refresh the timeout.
    ssl_session_renew_timeout(ssl, hs->new_session.get(),
                              ssl->session_ctx->session_psk_dhe_timeout);
  } else if (!ssl_get_new_session(hs)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
    return ssl_hs_error;
  }

  hs->new_session->cipher = hs->new_cipher;

  // Set up the key schedule and incorporate the PSK into the running secret.
  size_t hash_len = EVP_MD_size(
      ssl_get_handshake_digest(ssl_protocol_version(ssl), hs->new_cipher));
  if (!tls13_init_key_schedule(
          hs, ssl->s3->session_reused
                  ? MakeConstSpan(hs->new_session->secret,
                                  hs->new_session->secret_length)
                  : MakeConstSpan(kZeroes, hash_len))) {
    return ssl_hs_error;
  }

  if (!key_share.present) {
    // We do not support psk_ke and thus always require a key share.
    OPENSSL_PUT_ERROR(SSL, SSL_R_MISSING_KEY_SHARE);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_MISSING_EXTENSION);
    return ssl_hs_error;
  }

  // Resolve ECDHE and incorporate it into the secret.
  Array<uint8_t> dhe_secret;
  alert = SSL_AD_DECODE_ERROR;
  if (!ssl_ext_key_share_parse_serverhello(hs, &dhe_secret, &ssl->s3->peer_key,
                                           &alert, &key_share.data)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
    return ssl_hs_error;
  }

  if (!tls13_advance_key_schedule(hs, dhe_secret) ||
      !ssl_hash_message(hs, msg) || !tls13_derive_handshake_secrets(hs)) {
    return ssl_hs_error;
  }

  // If currently sending early data over TCP, we defer installing client
  // traffic keys to when the early data stream is closed. See
  // |close_early_data|. Note if the server has already rejected 0-RTT via
  // HelloRetryRequest, |in_early_data| is already false.
  if (!hs->in_early_data || ssl->quic_method != nullptr) {
    if (!tls13_set_traffic_key(ssl, ssl_encryption_handshake, evp_aead_seal,
                               hs->new_session.get(),
                               hs->client_handshake_secret())) {
      return ssl_hs_error;
    }
  }

  if (!tls13_set_traffic_key(ssl, ssl_encryption_handshake, evp_aead_open,
                             hs->new_session.get(),
                             hs->server_handshake_secret())) {
    return ssl_hs_error;
  }

  ssl->method->next_message(ssl);
  hs->tls13_state = state_read_encrypted_extensions;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_read_encrypted_extensions(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }
  if (!ssl_check_message_type(ssl, msg, SSL3_MT_ENCRYPTED_EXTENSIONS)) {
    return ssl_hs_error;
  }

  CBS body = msg.body, extensions;
  if (!CBS_get_u16_length_prefixed(&body, &extensions) ||
      CBS_len(&body) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    return ssl_hs_error;
  }

  if (!ssl_parse_serverhello_tlsext(hs, &extensions)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_PARSE_TLSEXT);
    return ssl_hs_error;
  }

  if (ssl->s3->early_data_accepted) {
    // The extension parser checks the server resumed the session.
    assert(ssl->s3->session_reused);
    // If offering ECH, the server may not accept early data with
    // ClientHelloOuter. We do not offer sessions with ClientHelloOuter, so this
    // this should be implied by checking |session_reused|.
    assert(ssl->s3->ech_status != ssl_ech_rejected);

    if (hs->early_session->cipher != hs->new_session->cipher) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_CIPHER_MISMATCH_ON_EARLY_DATA);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      return ssl_hs_error;
    }
    if (MakeConstSpan(hs->early_session->early_alpn) !=
        ssl->s3->alpn_selected) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_ALPN_MISMATCH_ON_EARLY_DATA);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      return ssl_hs_error;
    }
    // Channel ID is incompatible with 0-RTT. The ALPS extension should be
    // negotiated implicitly.
    if (hs->channel_id_negotiated ||
        hs->new_session->has_application_settings) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_EXTENSION_ON_EARLY_DATA);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      return ssl_hs_error;
    }
    hs->new_session->has_application_settings =
        hs->early_session->has_application_settings;
    if (!hs->new_session->local_application_settings.CopyFrom(
            hs->early_session->local_application_settings) ||
        !hs->new_session->peer_application_settings.CopyFrom(
            hs->early_session->peer_application_settings)) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
      return ssl_hs_error;
    }
    if (hs->received_custom_extension) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_EXTENSION_ON_EARLY_DATA);
      return ssl_hs_error;
    }
  }

  // Store the negotiated ALPN in the session.
  if (!hs->new_session->early_alpn.CopyFrom(ssl->s3->alpn_selected)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
    return ssl_hs_error;
  }

  if (!ssl_hash_message(hs, msg)) {
    return ssl_hs_error;
  }

  ssl->method->next_message(ssl);
  hs->tls13_state = state_read_certificate_request;
  if (hs->in_early_data && !ssl->s3->early_data_accepted) {
    if (!close_early_data(hs, ssl_encryption_handshake)) {
      return ssl_hs_error;
    }
    return ssl_hs_early_data_rejected;
  }
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_read_certificate_request(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  // CertificateRequest may only be sent in non-resumption handshakes.
  if (ssl->s3->session_reused) {
    if (ssl->ctx->reverify_on_resume && !ssl->s3->early_data_accepted) {
      hs->tls13_state = state_server_certificate_reverify;
      return ssl_hs_ok;
    }
    hs->tls13_state = state_read_server_finished;
    return ssl_hs_ok;
  }

  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }

  // CertificateRequest is optional.
  if (msg.type != SSL3_MT_CERTIFICATE_REQUEST) {
    hs->tls13_state = state_read_server_certificate;
    return ssl_hs_ok;
  }


  SSLExtension sigalgs(TLSEXT_TYPE_signature_algorithms),
      ca(TLSEXT_TYPE_certificate_authorities);
  CBS body = msg.body, context, extensions, supported_signature_algorithms;
  uint8_t alert = SSL_AD_DECODE_ERROR;
  if (!CBS_get_u8_length_prefixed(&body, &context) ||
      // The request context is always empty during the handshake.
      CBS_len(&context) != 0 ||
      !CBS_get_u16_length_prefixed(&body, &extensions) ||  //
      CBS_len(&body) != 0 ||
      !ssl_parse_extensions(&extensions, &alert, {&sigalgs, &ca},
                            /*ignore_unknown=*/true) ||
      !sigalgs.present ||
      !CBS_get_u16_length_prefixed(&sigalgs.data,
                                   &supported_signature_algorithms) ||
      !tls1_parse_peer_sigalgs(hs, &supported_signature_algorithms)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return ssl_hs_error;
  }

  if (ca.present) {
    hs->ca_names = ssl_parse_client_CA_list(ssl, &alert, &ca.data);
    if (!hs->ca_names) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
      return ssl_hs_error;
    }
  } else {
    hs->ca_names.reset(sk_CRYPTO_BUFFER_new_null());
    if (!hs->ca_names) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
      return ssl_hs_error;
    }
  }

  hs->cert_request = true;
  ssl->ctx->x509_method->hs_flush_cached_ca_names(hs);

  if (!ssl_hash_message(hs, msg)) {
    return ssl_hs_error;
  }

  ssl->method->next_message(ssl);
  hs->tls13_state = state_read_server_certificate;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_read_server_certificate(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }

  if (msg.type != SSL3_MT_COMPRESSED_CERTIFICATE &&
      !ssl_check_message_type(ssl, msg, SSL3_MT_CERTIFICATE)) {
    return ssl_hs_error;
  }

  if (!tls13_process_certificate(hs, msg, false /* certificate required */) ||
      !ssl_hash_message(hs, msg)) {
    return ssl_hs_error;
  }

  ssl->method->next_message(ssl);
  hs->tls13_state = state_read_server_certificate_verify;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_read_server_certificate_verify(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }
  switch (ssl_verify_peer_cert(hs)) {
    case ssl_verify_ok:
      break;
    case ssl_verify_invalid:
      return ssl_hs_error;
    case ssl_verify_retry:
      hs->tls13_state = state_read_server_certificate_verify;
      return ssl_hs_certificate_verify;
  }

  if (!ssl_check_message_type(ssl, msg, SSL3_MT_CERTIFICATE_VERIFY) ||
      !tls13_process_certificate_verify(hs, msg) ||
      !ssl_hash_message(hs, msg)) {
    return ssl_hs_error;
  }

  ssl->method->next_message(ssl);
  hs->tls13_state = state_read_server_finished;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_server_certificate_reverify(SSL_HANDSHAKE *hs) {
  switch (ssl_reverify_peer_cert(hs, /*send_alert=*/true)) {
    case ssl_verify_ok:
      break;
    case ssl_verify_invalid:
      return ssl_hs_error;
    case ssl_verify_retry:
      hs->tls13_state = state_server_certificate_reverify;
      return ssl_hs_certificate_verify;
  }
  hs->tls13_state = state_read_server_finished;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_read_server_finished(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  SSLMessage msg;
  if (!ssl->method->get_message(ssl, &msg)) {
    return ssl_hs_read_message;
  }
  if (!ssl_check_message_type(ssl, msg, SSL3_MT_FINISHED) ||
      !tls13_process_finished(hs, msg, false /* don't use saved value */) ||
      !ssl_hash_message(hs, msg) ||
      // Update the secret to the master secret and derive traffic keys.
      !tls13_advance_key_schedule(
          hs, MakeConstSpan(kZeroes, hs->transcript.DigestLen())) ||
      !tls13_derive_application_secrets(hs)) {
    return ssl_hs_error;
  }

  // Finished should be the end of the flight.
  if (ssl->method->has_unprocessed_handshake_data(ssl)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_UNEXPECTED_MESSAGE);
    OPENSSL_PUT_ERROR(SSL, SSL_R_EXCESS_HANDSHAKE_DATA);
    return ssl_hs_error;
  }

  ssl->method->next_message(ssl);
  hs->tls13_state = state_send_end_of_early_data;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_send_end_of_early_data(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;

  if (ssl->s3->early_data_accepted) {
    // QUIC omits the EndOfEarlyData message. See RFC 9001, section 8.3.
    if (ssl->quic_method == nullptr) {
      ScopedCBB cbb;
      CBB body;
      if (!ssl->method->init_message(ssl, cbb.get(), &body,
                                     SSL3_MT_END_OF_EARLY_DATA) ||
          !ssl_add_message_cbb(ssl, cbb.get())) {
        return ssl_hs_error;
      }
    }

    if (!close_early_data(hs, ssl_encryption_handshake)) {
      return ssl_hs_error;
    }
  }

  hs->tls13_state = state_send_client_encrypted_extensions;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_send_client_encrypted_extensions(
    SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  // For now, only one extension uses client EncryptedExtensions. This function
  // may be generalized if others use it in the future.
  if (hs->new_session->has_application_settings &&
      !ssl->s3->early_data_accepted) {
    ScopedCBB cbb;
    CBB body, extensions, extension;
    uint16_t extension_type = TLSEXT_TYPE_application_settings_old;
    if (hs->config->alps_use_new_codepoint) {
      extension_type = TLSEXT_TYPE_application_settings;
    }
    if (!ssl->method->init_message(ssl, cbb.get(), &body,
                                   SSL3_MT_ENCRYPTED_EXTENSIONS) ||
        !CBB_add_u16_length_prefixed(&body, &extensions) ||
        !CBB_add_u16(&extensions, extension_type) ||
        !CBB_add_u16_length_prefixed(&extensions, &extension) ||
        !CBB_add_bytes(&extension,
                       hs->new_session->local_application_settings.data(),
                       hs->new_session->local_application_settings.size()) ||
        !ssl_add_message_cbb(ssl, cbb.get())) {
      return ssl_hs_error;
    }
  }

  hs->tls13_state = state_send_client_certificate;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_send_client_certificate(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;

  // The peer didn't request a certificate.
  if (!hs->cert_request) {
    hs->tls13_state = state_complete_second_flight;
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
      hs->tls13_state = state_send_client_certificate;
      return ssl_hs_x509_lookup;
    }
  }

  if (!ssl_on_certificate_selected(hs) ||
      !tls13_add_certificate(hs)) {
    return ssl_hs_error;
  }

  hs->tls13_state = state_send_client_certificate_verify;
  return ssl_hs_ok;
}

static enum ssl_hs_wait_t do_send_client_certificate_verify(SSL_HANDSHAKE *hs) {
  // Don't send CertificateVerify if there is no certificate.
  if (!ssl_has_certificate(hs)) {
    hs->tls13_state = state_complete_second_flight;
    return ssl_hs_ok;
  }

  if (!tls1_choose_signature_algorithm(hs, &hs->signature_algorithm)) {
    ssl_send_alert(hs->ssl, SSL3_AL_FATAL, SSL_AD_HANDSHAKE_FAILURE);
    return ssl_hs_error;
  }

  switch (tls13_add_certificate_verify(hs)) {
    case ssl_private_key_success:
      hs->tls13_state = state_complete_second_flight;
      return ssl_hs_ok;

    case ssl_private_key_retry:
      hs->tls13_state = state_send_client_certificate_verify;
      return ssl_hs_private_key_operation;

    case ssl_private_key_failure:
      return ssl_hs_error;
  }

  assert(0);
  return ssl_hs_error;
}

static enum ssl_hs_wait_t do_complete_second_flight(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  hs->can_release_private_key = true;

  // Send a Channel ID assertion if necessary.
  if (hs->channel_id_negotiated) {
    ScopedCBB cbb;
    CBB body;
    if (!ssl->method->init_message(ssl, cbb.get(), &body, SSL3_MT_CHANNEL_ID) ||
        !tls1_write_channel_id(hs, &body) ||
        !ssl_add_message_cbb(ssl, cbb.get())) {
      return ssl_hs_error;
    }
  }

  // Send a Finished message.
  if (!tls13_add_finished(hs)) {
    return ssl_hs_error;
  }

  // Derive the final keys and enable them.
  if (!tls13_set_traffic_key(ssl, ssl_encryption_application, evp_aead_seal,
                             hs->new_session.get(),
                             hs->client_traffic_secret_0()) ||
      !tls13_set_traffic_key(ssl, ssl_encryption_application, evp_aead_open,
                             hs->new_session.get(),
                             hs->server_traffic_secret_0()) ||
      !tls13_derive_resumption_secret(hs)) {
    return ssl_hs_error;
  }

  hs->tls13_state = state_done;
  return ssl_hs_flush;
}

enum ssl_hs_wait_t tls13_client_handshake(SSL_HANDSHAKE *hs) {
  while (hs->tls13_state != state_done) {
    enum ssl_hs_wait_t ret = ssl_hs_error;
    enum client_hs_state_t state =
        static_cast<enum client_hs_state_t>(hs->tls13_state);
    switch (state) {
      case state_read_hello_retry_request:
        ret = do_read_hello_retry_request(hs);
        break;
      case state_send_second_client_hello:
        ret = do_send_second_client_hello(hs);
        break;
      case state_read_server_hello:
        ret = do_read_server_hello(hs);
        break;
      case state_read_encrypted_extensions:
        ret = do_read_encrypted_extensions(hs);
        break;
      case state_read_certificate_request:
        ret = do_read_certificate_request(hs);
        break;
      case state_read_server_certificate:
        ret = do_read_server_certificate(hs);
        break;
      case state_read_server_certificate_verify:
        ret = do_read_server_certificate_verify(hs);
        break;
      case state_server_certificate_reverify:
        ret = do_server_certificate_reverify(hs);
        break;
      case state_read_server_finished:
        ret = do_read_server_finished(hs);
        break;
      case state_send_end_of_early_data:
        ret = do_send_end_of_early_data(hs);
        break;
      case state_send_client_certificate:
        ret = do_send_client_certificate(hs);
        break;
      case state_send_client_encrypted_extensions:
        ret = do_send_client_encrypted_extensions(hs);
        break;
      case state_send_client_certificate_verify:
        ret = do_send_client_certificate_verify(hs);
        break;
      case state_complete_second_flight:
        ret = do_complete_second_flight(hs);
        break;
      case state_done:
        ret = ssl_hs_ok;
        break;
    }

    if (hs->tls13_state != state) {
      ssl_do_info_callback(hs->ssl, SSL_CB_CONNECT_LOOP, 1);
    }

    if (ret != ssl_hs_ok) {
      return ret;
    }
  }

  return ssl_hs_ok;
}

const char *tls13_client_handshake_state(SSL_HANDSHAKE *hs) {
  enum client_hs_state_t state =
      static_cast<enum client_hs_state_t>(hs->tls13_state);
  switch (state) {
    case state_read_hello_retry_request:
      return "TLS 1.3 client read_hello_retry_request";
    case state_send_second_client_hello:
      return "TLS 1.3 client send_second_client_hello";
    case state_read_server_hello:
      return "TLS 1.3 client read_server_hello";
    case state_read_encrypted_extensions:
      return "TLS 1.3 client read_encrypted_extensions";
    case state_read_certificate_request:
      return "TLS 1.3 client read_certificate_request";
    case state_read_server_certificate:
      return "TLS 1.3 client read_server_certificate";
    case state_read_server_certificate_verify:
      return "TLS 1.3 client read_server_certificate_verify";
    case state_server_certificate_reverify:
      return "TLS 1.3 client server_certificate_reverify";
    case state_read_server_finished:
      return "TLS 1.3 client read_server_finished";
    case state_send_end_of_early_data:
      return "TLS 1.3 client send_end_of_early_data";
    case state_send_client_encrypted_extensions:
      return "TLS 1.3 client send_client_encrypted_extensions";
    case state_send_client_certificate:
      return "TLS 1.3 client send_client_certificate";
    case state_send_client_certificate_verify:
      return "TLS 1.3 client send_client_certificate_verify";
    case state_complete_second_flight:
      return "TLS 1.3 client complete_second_flight";
    case state_done:
      return "TLS 1.3 client done";
  }

  return "TLS 1.3 client unknown";
}

bool tls13_process_new_session_ticket(SSL *ssl, const SSLMessage &msg) {
  if (ssl->s3->write_shutdown != ssl_shutdown_none) {
    // Ignore tickets on shutdown. Callers tend to indiscriminately call
    // |SSL_shutdown| before destroying an |SSL|, at which point calling the new
    // session callback may be confusing.
    return true;
  }

  CBS body = msg.body;
  UniquePtr<SSL_SESSION> session = tls13_create_session_with_ticket(ssl, &body);
  if (!session) {
    return false;
  }

  if ((ssl->session_ctx->session_cache_mode & SSL_SESS_CACHE_CLIENT) &&
      ssl->session_ctx->new_session_cb != NULL &&
      ssl->session_ctx->new_session_cb(ssl, session.get())) {
    // |new_session_cb|'s return value signals that it took ownership.
    session.release();
  }

  return true;
}

UniquePtr<SSL_SESSION> tls13_create_session_with_ticket(SSL *ssl, CBS *body) {
  UniquePtr<SSL_SESSION> session = SSL_SESSION_dup(
      ssl->s3->established_session.get(), SSL_SESSION_INCLUDE_NONAUTH);
  if (!session) {
    return nullptr;
  }

  ssl_session_rebase_time(ssl, session.get());

  uint32_t server_timeout;
  CBS ticket_nonce, ticket, extensions;
  if (!CBS_get_u32(body, &server_timeout) ||
      !CBS_get_u32(body, &session->ticket_age_add) ||
      !CBS_get_u8_length_prefixed(body, &ticket_nonce) ||
      !CBS_get_u16_length_prefixed(body, &ticket) ||
      CBS_len(&ticket) == 0 ||  //
      !session->ticket.CopyFrom(ticket) ||
      !CBS_get_u16_length_prefixed(body, &extensions) ||  //
      CBS_len(body) != 0) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return nullptr;
  }

  // Cap the renewable lifetime by the server advertised value. This avoids
  // wasting bandwidth on 0-RTT when we know the server will reject it.
  if (session->timeout > server_timeout) {
    session->timeout = server_timeout;
  }

  if (!tls13_derive_session_psk(session.get(), ticket_nonce)) {
    return nullptr;
  }

  SSLExtension early_data(TLSEXT_TYPE_early_data);
  uint8_t alert = SSL_AD_DECODE_ERROR;
  if (!ssl_parse_extensions(&extensions, &alert, {&early_data},
                            /*ignore_unknown=*/true)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
    return nullptr;
  }

  if (early_data.present) {
    if (!CBS_get_u32(&early_data.data, &session->ticket_max_early_data) ||
        CBS_len(&early_data.data) != 0) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
      OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
      return nullptr;
    }

    // QUIC does not use the max_early_data_size parameter and always sets it to
    // a fixed value. See RFC 9001, section 4.6.1.
    if (ssl->quic_method != nullptr &&
        session->ticket_max_early_data != 0xffffffff) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
      return nullptr;
    }
  }

  // Historically, OpenSSL filled in fake session IDs for ticket-based sessions.
  // Envoy's tests depend on this, although perhaps they shouldn't.
  SHA256(CBS_data(&ticket), CBS_len(&ticket), session->session_id);
  session->session_id_length = SHA256_DIGEST_LENGTH;

  session->ticket_age_add_valid = true;
  session->not_resumable = false;

  return session;
}

BSSL_NAMESPACE_END
