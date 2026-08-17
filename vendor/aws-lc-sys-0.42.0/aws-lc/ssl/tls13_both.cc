// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/ssl.h>

#include <assert.h>
#include <string.h>

#include <utility>

#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/hkdf.h>
#include <openssl/mem.h>
#include <openssl/stack.h>
#include <openssl/x509.h>

#include "../crypto/internal.h"
#include "internal.h"


BSSL_NAMESPACE_BEGIN

const uint8_t kHelloRetryRequest[SSL3_RANDOM_SIZE] = {
    0xcf, 0x21, 0xad, 0x74, 0xe5, 0x9a, 0x61, 0x11, 0xbe, 0x1d, 0x8c,
    0x02, 0x1e, 0x65, 0xb8, 0x91, 0xc2, 0xa2, 0x11, 0x16, 0x7a, 0xbb,
    0x8c, 0x5e, 0x07, 0x9e, 0x09, 0xe2, 0xc8, 0xa8, 0x33, 0x9c,
};

// See RFC 8446, section 4.1.3.
const uint8_t kTLS12DowngradeRandom[8] = {0x44, 0x4f, 0x57, 0x4e,
                                          0x47, 0x52, 0x44, 0x00};
const uint8_t kTLS13DowngradeRandom[8] = {0x44, 0x4f, 0x57, 0x4e,
                                          0x47, 0x52, 0x44, 0x01};

// This is a non-standard randomly-generated value.
const uint8_t kJDK11DowngradeRandom[8] = {0xed, 0xbf, 0xb4, 0xa8,
                                          0xc2, 0x47, 0x10, 0xff};

bool tls13_get_cert_verify_signature_input(
    SSL_HANDSHAKE *hs, Array<uint8_t> *out,
    enum ssl_cert_verify_context_t cert_verify_context) {
  ScopedCBB cbb;
  if (!CBB_init(cbb.get(), 64 + 33 + 1 + 2 * EVP_MAX_MD_SIZE)) {
    return false;
  }

  for (size_t i = 0; i < 64; i++) {
    if (!CBB_add_u8(cbb.get(), 0x20)) {
      return false;
    }
  }

  Span<const char> context;
  if (cert_verify_context == ssl_cert_verify_server) {
    static const char kContext[] = "TLS 1.3, server CertificateVerify";
    context = kContext;
  } else if (cert_verify_context == ssl_cert_verify_client) {
    static const char kContext[] = "TLS 1.3, client CertificateVerify";
    context = kContext;
  } else if (cert_verify_context == ssl_cert_verify_channel_id) {
    static const char kContext[] = "TLS 1.3, Channel ID";
    context = kContext;
  } else {
    return false;
  }

  // Note |context| includes the NUL byte separator.
  if (!CBB_add_bytes(cbb.get(),
                     reinterpret_cast<const uint8_t *>(context.data()),
                     context.size())) {
    return false;
  }

  uint8_t context_hash[EVP_MAX_MD_SIZE];
  size_t context_hash_len;
  if (!hs->transcript.GetHash(context_hash, &context_hash_len) ||
      !CBB_add_bytes(cbb.get(), context_hash, context_hash_len) ||
      !CBBFinishArray(cbb.get(), out)) {
    return false;
  }

  return true;
}

bool tls13_process_certificate(SSL_HANDSHAKE *hs, const SSLMessage &msg,
                               bool allow_anonymous) {
  SSL *const ssl = hs->ssl;
  CBS body = msg.body;
  bssl::UniquePtr<CRYPTO_BUFFER> decompressed;

  if (msg.type == SSL3_MT_COMPRESSED_CERTIFICATE) {
    CBS compressed;
    uint16_t alg_id;
    uint32_t uncompressed_len;

    if (!CBS_get_u16(&body, &alg_id) ||
        !CBS_get_u24(&body, &uncompressed_len) ||
        !CBS_get_u24_length_prefixed(&body, &compressed) ||
        CBS_len(&body) != 0) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
      OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
      return false;
    }

    if (uncompressed_len > ssl->max_cert_list) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      OPENSSL_PUT_ERROR(SSL, SSL_R_UNCOMPRESSED_CERT_TOO_LARGE);
      ERR_add_error_dataf("requested=%u",
                          static_cast<unsigned>(uncompressed_len));
      return false;
    }

    ssl_cert_decompression_func_t decompress = nullptr;
    for (const auto &alg : ssl->ctx->cert_compression_algs) {
      if (alg.alg_id == alg_id) {
        decompress = alg.decompress;
        break;
      }
    }

    if (decompress == nullptr) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
      OPENSSL_PUT_ERROR(SSL, SSL_R_UNKNOWN_CERT_COMPRESSION_ALG);
      ERR_add_error_dataf("alg=%d", static_cast<int>(alg_id));
      return false;
    }

    CRYPTO_BUFFER *decompressed_ptr = nullptr;
    if (!decompress(ssl, &decompressed_ptr, uncompressed_len,
                    CBS_data(&compressed), CBS_len(&compressed))) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
      OPENSSL_PUT_ERROR(SSL, SSL_R_CERT_DECOMPRESSION_FAILED);
      ERR_add_error_dataf("alg=%d", static_cast<int>(alg_id));
      return false;
    }
    decompressed.reset(decompressed_ptr);

    if (CRYPTO_BUFFER_len(decompressed_ptr) != uncompressed_len) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
      OPENSSL_PUT_ERROR(SSL, SSL_R_CERT_DECOMPRESSION_FAILED);
      ERR_add_error_dataf(
          "alg=%d got=%u expected=%u", static_cast<int>(alg_id),
          static_cast<unsigned>(CRYPTO_BUFFER_len(decompressed_ptr)),
          static_cast<unsigned>(uncompressed_len));
      return false;
    }

    CBS_init(&body, CRYPTO_BUFFER_data(decompressed_ptr),
             CRYPTO_BUFFER_len(decompressed_ptr));
  } else {
    assert(msg.type == SSL3_MT_CERTIFICATE);
  }

  CBS context, certificate_list;
  if (!CBS_get_u8_length_prefixed(&body, &context) ||
      CBS_len(&context) != 0 ||
      !CBS_get_u24_length_prefixed(&body, &certificate_list) ||
      CBS_len(&body) != 0) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return false;
  }

  UniquePtr<STACK_OF(CRYPTO_BUFFER)> certs(sk_CRYPTO_BUFFER_new_null());
  if (!certs) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
    return false;
  }

  const bool retain_sha256 =
      ssl->server && hs->config->retain_only_sha256_of_client_certs;
  UniquePtr<EVP_PKEY> pkey;
  while (CBS_len(&certificate_list) > 0) {
    CBS certificate, extensions;
    if (!CBS_get_u24_length_prefixed(&certificate_list, &certificate) ||
        !CBS_get_u16_length_prefixed(&certificate_list, &extensions) ||
        CBS_len(&certificate) == 0) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
      OPENSSL_PUT_ERROR(SSL, SSL_R_CERT_LENGTH_MISMATCH);
      return false;
    }

    if (sk_CRYPTO_BUFFER_num(certs.get()) == 0) {
      pkey = ssl_cert_parse_pubkey(&certificate);
      if (!pkey) {
        ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
        OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
        return false;
      }
      // TLS 1.3 always uses certificate keys for signing thus the correct
      // keyUsage is enforced.
      if (!ssl_cert_check_key_usage(&certificate,
                                    key_usage_digital_signature)) {
        ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_ILLEGAL_PARAMETER);
        return false;
      }

      if (retain_sha256) {
        // Retain the hash of the leaf certificate if requested.
        SHA256(CBS_data(&certificate), CBS_len(&certificate),
               hs->new_session->peer_sha256);
      }
    }

    UniquePtr<CRYPTO_BUFFER> buf(
        CRYPTO_BUFFER_new_from_CBS(&certificate, ssl->ctx->pool));
    if (!buf ||
        !PushToStack(certs.get(), std::move(buf))) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
      return false;
    }

    // Parse out the extensions.
    SSLExtension status_request(
        TLSEXT_TYPE_status_request,
        !ssl->server && hs->config->ocsp_stapling_enabled);
    SSLExtension sct(
        TLSEXT_TYPE_certificate_timestamp,
        !ssl->server && hs->config->signed_cert_timestamps_enabled);
    uint8_t alert = SSL_AD_DECODE_ERROR;
    if (!ssl_parse_extensions(&extensions, &alert, {&status_request, &sct},
                              /*ignore_unknown=*/false)) {
      ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
      return false;
    }

    // All Certificate extensions are parsed, but only the leaf extensions are
    // stored.
    if (status_request.present) {
      uint8_t status_type;
      CBS ocsp_response;
      if (!CBS_get_u8(&status_request.data, &status_type) ||
          status_type != TLSEXT_STATUSTYPE_ocsp ||
          !CBS_get_u24_length_prefixed(&status_request.data, &ocsp_response) ||
          CBS_len(&ocsp_response) == 0 ||
          CBS_len(&status_request.data) != 0) {
        ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
        return false;
      }

      if (sk_CRYPTO_BUFFER_num(certs.get()) == 1) {
        hs->new_session->ocsp_response.reset(
            CRYPTO_BUFFER_new_from_CBS(&ocsp_response, ssl->ctx->pool));
        if (hs->new_session->ocsp_response == nullptr) {
          ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
          return false;
        }
      }
    }

    if (sct.present) {
      if (!ssl_is_sct_list_valid(&sct.data)) {
        OPENSSL_PUT_ERROR(SSL, SSL_R_ERROR_PARSING_EXTENSION);
        ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
        return false;
      }

      if (sk_CRYPTO_BUFFER_num(certs.get()) == 1) {
        hs->new_session->signed_cert_timestamp_list.reset(
            CRYPTO_BUFFER_new_from_CBS(&sct.data, ssl->ctx->pool));
        if (hs->new_session->signed_cert_timestamp_list == nullptr) {
          ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
          return false;
        }
      }
    }
  }

  // Store a null certificate list rather than an empty one if the peer didn't
  // send certificates.
  if (sk_CRYPTO_BUFFER_num(certs.get()) == 0) {
    certs.reset();
  }

  hs->peer_pubkey = std::move(pkey);
  hs->new_session->certs = std::move(certs);

  if (!ssl->ctx->x509_method->session_cache_objects(hs->new_session.get())) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    return false;
  }

  if (sk_CRYPTO_BUFFER_num(hs->new_session->certs.get()) == 0) {
    if (!allow_anonymous) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_PEER_DID_NOT_RETURN_A_CERTIFICATE);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_CERTIFICATE_REQUIRED);
      return false;
    }

    // OpenSSL returns X509_V_OK when no certificates are requested. This is
    // classed by them as a bug, but it's assumed by at least NGINX.
    hs->new_session->verify_result = X509_V_OK;
    ssl->verify_result = hs->new_session->verify_result;

    // No certificate, so nothing more to do.
    return true;
  }

  hs->new_session->peer_sha256_valid = retain_sha256;
  return true;
}

bool tls13_process_certificate_verify(SSL_HANDSHAKE *hs, const SSLMessage &msg) {
  SSL *const ssl = hs->ssl;
  if (hs->peer_pubkey == NULL) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  CBS body = msg.body, signature;
  uint16_t signature_algorithm;
  if (!CBS_get_u16(&body, &signature_algorithm) ||
      !CBS_get_u16_length_prefixed(&body, &signature) ||
      CBS_len(&body) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    return false;
  }

  uint8_t alert = SSL_AD_DECODE_ERROR;
  if (!tls12_check_peer_sigalg(hs, &alert, signature_algorithm)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
    return false;
  }
  hs->new_session->peer_signature_algorithm = signature_algorithm;

  Array<uint8_t> input;
  if (!tls13_get_cert_verify_signature_input(
          hs, &input,
          ssl->server ? ssl_cert_verify_client : ssl_cert_verify_server)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
    return false;
  }

  if (!ssl_public_key_verify(ssl, signature, signature_algorithm,
                             hs->peer_pubkey.get(), input)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_SIGNATURE);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECRYPT_ERROR);
    return false;
  }

  return true;
}

bool tls13_process_finished(SSL_HANDSHAKE *hs, const SSLMessage &msg,
                            bool use_saved_value) {
  SSL *const ssl = hs->ssl;
  uint8_t verify_data_buf[EVP_MAX_MD_SIZE];
  Span<const uint8_t> verify_data;
  if (use_saved_value) {
    assert(ssl->server);
    verify_data = hs->expected_client_finished();
  } else {
    size_t len;
    if (!tls13_finished_mac(hs, verify_data_buf, &len, !ssl->server)) {
      return false;
    }
    verify_data = MakeConstSpan(verify_data_buf, len);
  }

  bool finished_ok =
      CBS_mem_equal(&msg.body, verify_data.data(), verify_data.size());
#if defined(BORINGSSL_UNSAFE_FUZZER_MODE)
  finished_ok = true;
#endif
  if (!finished_ok) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECRYPT_ERROR);
    OPENSSL_PUT_ERROR(SSL, SSL_R_DIGEST_CHECK_FAILED);
    return false;
  }

  if (verify_data.size() > sizeof(ssl->s3->previous_client_finished) ||
      verify_data.size() > sizeof(ssl->s3->previous_server_finished)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  if (ssl->server) {
    OPENSSL_memcpy(ssl->s3->previous_client_finished, verify_data.data(),
                   verify_data.size());
    ssl->s3->previous_client_finished_len = verify_data.size();
  } else {
    OPENSSL_memcpy(ssl->s3->previous_server_finished, verify_data.data(),
                   verify_data.size());
    ssl->s3->previous_server_finished_len = verify_data.size();
  }

  return true;
}

bool tls13_add_certificate(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  CERT *const cert = hs->config->cert.get();
  DC *const dc = cert->dc.get();

  ScopedCBB cbb;
  CBB *body, body_storage, certificate_list;

  if (hs->cert_compression_negotiated) {
    if (!CBB_init(cbb.get(), 1024)) {
      return false;
    }
    body = cbb.get();
  } else {
    body = &body_storage;
    if (!ssl->method->init_message(ssl, cbb.get(), body, SSL3_MT_CERTIFICATE)) {
      return false;
    }
  }

  if (  // The request context is always empty in the handshake.
      !CBB_add_u8(body, 0) ||
      !CBB_add_u24_length_prefixed(body, &certificate_list)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  if (!ssl_has_certificate(hs)) {
    return ssl_add_message_cbb(ssl, cbb.get());
  }
  // |cert_private_keys| already checked above in |ssl_has_certificate|.
  UniquePtr<STACK_OF(CRYPTO_BUFFER)> &chain =
      cert->cert_private_keys[cert->cert_private_key_idx].chain;
  CRYPTO_BUFFER *leaf_buf = sk_CRYPTO_BUFFER_value(chain.get(), 0);
  CBB leaf, extensions;
  if (!CBB_add_u24_length_prefixed(&certificate_list, &leaf) ||
      !CBB_add_bytes(&leaf, CRYPTO_BUFFER_data(leaf_buf),
                     CRYPTO_BUFFER_len(leaf_buf)) ||
      !CBB_add_u16_length_prefixed(&certificate_list, &extensions)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  if (hs->scts_requested && cert->signed_cert_timestamp_list != nullptr) {
    CBB contents;
    if (!CBB_add_u16(&extensions, TLSEXT_TYPE_certificate_timestamp) ||
        !CBB_add_u16_length_prefixed(&extensions, &contents) ||
        !CBB_add_bytes(
            &contents,
            CRYPTO_BUFFER_data(cert->signed_cert_timestamp_list.get()),
            CRYPTO_BUFFER_len(cert->signed_cert_timestamp_list.get())) ||
        !CBB_flush(&extensions)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }
  }

  if (hs->ocsp_stapling_requested && cert->ocsp_response != NULL) {
    CBB contents, ocsp_response;
    if (!CBB_add_u16(&extensions, TLSEXT_TYPE_status_request) ||
        !CBB_add_u16_length_prefixed(&extensions, &contents) ||
        !CBB_add_u8(&contents, TLSEXT_STATUSTYPE_ocsp) ||
        !CBB_add_u24_length_prefixed(&contents, &ocsp_response) ||
        !CBB_add_bytes(&ocsp_response,
                       CRYPTO_BUFFER_data(cert->ocsp_response.get()),
                       CRYPTO_BUFFER_len(cert->ocsp_response.get())) ||
        !CBB_flush(&extensions)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }
  }

  if (ssl_signing_with_dc(hs)) {
    const CRYPTO_BUFFER *raw = dc->raw.get();
    CBB child;
    if (!CBB_add_u16(&extensions, TLSEXT_TYPE_delegated_credential) ||
        !CBB_add_u16_length_prefixed(&extensions, &child) ||
        !CBB_add_bytes(&child, CRYPTO_BUFFER_data(raw),
                       CRYPTO_BUFFER_len(raw)) ||
        !CBB_flush(&extensions)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }
    ssl->s3->delegated_credential_used = true;
  }

  for (size_t i = 1; i < sk_CRYPTO_BUFFER_num(chain.get()); i++) {
    CRYPTO_BUFFER *cert_buf = sk_CRYPTO_BUFFER_value(chain.get(), i);
    CBB child;
    if (!CBB_add_u24_length_prefixed(&certificate_list, &child) ||
        !CBB_add_bytes(&child, CRYPTO_BUFFER_data(cert_buf),
                       CRYPTO_BUFFER_len(cert_buf)) ||
        !CBB_add_u16(&certificate_list, 0 /* no extensions */)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }
  }

  if (!hs->cert_compression_negotiated) {
    return ssl_add_message_cbb(ssl, cbb.get());
  }

  Array<uint8_t> msg;
  if (!CBBFinishArray(cbb.get(), &msg)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  const CertCompressionAlg *alg = nullptr;
  for (const auto &candidate : ssl->ctx->cert_compression_algs) {
    if (candidate.alg_id == hs->cert_compression_alg_id) {
      alg = &candidate;
      break;
    }
  }

  if (alg == nullptr || alg->compress == nullptr) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  CBB compressed;
  body = &body_storage;
  if (!ssl->method->init_message(ssl, cbb.get(), body,
                                 SSL3_MT_COMPRESSED_CERTIFICATE) ||
      !CBB_add_u16(body, hs->cert_compression_alg_id) ||
      msg.size() > (1u << 24) - 1 ||  //
      !CBB_add_u24(body, static_cast<uint32_t>(msg.size())) ||
      !CBB_add_u24_length_prefixed(body, &compressed)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  SSL_HANDSHAKE_HINTS *const hints = hs->hints.get();
  if (hints && !hs->hints_requested &&
      hints->cert_compression_alg_id == hs->cert_compression_alg_id &&
      hints->cert_compression_input == MakeConstSpan(msg) &&
      !hints->cert_compression_output.empty()) {
    if (!CBB_add_bytes(&compressed, hints->cert_compression_output.data(),
                       hints->cert_compression_output.size())) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }
  } else {
    if (!alg->compress(ssl, &compressed, msg.data(), msg.size())) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }
    if (hints && hs->hints_requested) {
      hints->cert_compression_alg_id = hs->cert_compression_alg_id;
      if (!hints->cert_compression_input.CopyFrom(msg) ||
          !hints->cert_compression_output.CopyFrom(
              MakeConstSpan(CBB_data(&compressed), CBB_len(&compressed)))) {
        return false;
      }
    }
  }

  if (!ssl_add_message_cbb(ssl, cbb.get())) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  return true;
}

enum ssl_private_key_result_t tls13_add_certificate_verify(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;

  ScopedCBB cbb;
  CBB body;
  if (!ssl->method->init_message(ssl, cbb.get(), &body,
                                 SSL3_MT_CERTIFICATE_VERIFY) ||
      !CBB_add_u16(&body, hs->signature_algorithm)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return ssl_private_key_failure;
  }

  CBB child;
  const size_t max_sig_len = EVP_PKEY_size(hs->local_pubkey.get());
  uint8_t *sig;
  size_t sig_len;
  if (!CBB_add_u16_length_prefixed(&body, &child) ||
      !CBB_reserve(&child, &sig, max_sig_len)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
    return ssl_private_key_failure;
  }

  Array<uint8_t> msg;
  if (!tls13_get_cert_verify_signature_input(
          hs, &msg,
          ssl->server ? ssl_cert_verify_server : ssl_cert_verify_client)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
    return ssl_private_key_failure;
  }

  enum ssl_private_key_result_t sign_result = ssl_private_key_sign(
      hs, sig, &sig_len, max_sig_len, hs->signature_algorithm, msg);
  if (sign_result != ssl_private_key_success) {
    return sign_result;
  }

  if (!CBB_did_write(&child, sig_len) ||
      !ssl_add_message_cbb(ssl, cbb.get())) {
    return ssl_private_key_failure;
  }

  return ssl_private_key_success;
}

bool tls13_add_finished(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  size_t verify_data_len;
  uint8_t verify_data[EVP_MAX_MD_SIZE];

  if (!tls13_finished_mac(hs, verify_data, &verify_data_len, ssl->server)) {
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_INTERNAL_ERROR);
    OPENSSL_PUT_ERROR(SSL, SSL_R_DIGEST_CHECK_FAILED);
    return false;
  }

  if (verify_data_len > sizeof(ssl->s3->previous_client_finished) ||
      verify_data_len > sizeof(ssl->s3->previous_server_finished)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  if (ssl->server) {
    OPENSSL_memcpy(ssl->s3->previous_server_finished, verify_data,
                   verify_data_len);
    ssl->s3->previous_server_finished_len = verify_data_len;
  } else {
    OPENSSL_memcpy(ssl->s3->previous_client_finished, verify_data,
                   verify_data_len);
    ssl->s3->previous_client_finished_len = verify_data_len;
  }

  ScopedCBB cbb;
  CBB body;
  if (!ssl->method->init_message(ssl, cbb.get(), &body, SSL3_MT_FINISHED) ||
      !CBB_add_bytes(&body, verify_data, verify_data_len) ||
      !ssl_add_message_cbb(ssl, cbb.get())) {
    return false;
  }

  return true;
}

bool tls13_add_key_update(SSL *ssl, int update_requested) {
  ScopedCBB cbb;
  CBB body_cbb;
  if (!ssl->method->init_message(ssl, cbb.get(), &body_cbb,
                                 SSL3_MT_KEY_UPDATE) ||
      (update_requested != SSL_KEY_UPDATE_NOT_REQUESTED &&
       update_requested != SSL_KEY_UPDATE_REQUESTED) ||
      !CBB_add_u8(&body_cbb, update_requested) ||
      !ssl_add_message_cbb(ssl, cbb.get()) ||
      !tls13_rotate_traffic_key(ssl, evp_aead_seal)) {
    return false;
  }

  // Suppress KeyUpdate acknowledgments until this change is written to the
  // wire. This prevents us from accumulating write obligations when read and
  // write progress at different rates. See RFC 8446, section 4.6.3.
  ssl->s3->key_update_pending = update_requested;

  return true;
}

static bool tls13_receive_key_update(SSL *ssl, const SSLMessage &msg) {
  CBS body = msg.body;
  uint8_t key_update_request;
  if (!CBS_get_u8(&body, &key_update_request) ||
      CBS_len(&body) != 0 ||
      (key_update_request != SSL_KEY_UPDATE_NOT_REQUESTED &&
       key_update_request != SSL_KEY_UPDATE_REQUESTED)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_DECODE_ERROR);
    return false;
  }

  if (!tls13_rotate_traffic_key(ssl, evp_aead_open)) {
    return false;
  }

  // Acknowledge the KeyUpdate
  if (key_update_request == SSL_KEY_UPDATE_REQUESTED &&
      ssl->s3->key_update_pending == SSL_KEY_UPDATE_NONE &&
      !tls13_add_key_update(ssl, SSL_KEY_UPDATE_NOT_REQUESTED)) {
    return false;
  }

  return true;
}

bool tls13_post_handshake(SSL *ssl, const SSLMessage &msg) {
  if (msg.type == SSL3_MT_KEY_UPDATE) {
    if (ssl->quic_method != nullptr) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
      ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_UNEXPECTED_MESSAGE);
      return false;
    }

    return tls13_receive_key_update(ssl, msg);
  }

  if (msg.type == SSL3_MT_NEW_SESSION_TICKET && !ssl->server) {
    return tls13_process_new_session_ticket(ssl, msg);
  }

  ssl_send_alert(ssl, SSL3_AL_FATAL, SSL_AD_UNEXPECTED_MESSAGE);
  OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_MESSAGE);
  return false;
}

BSSL_NAMESPACE_END
