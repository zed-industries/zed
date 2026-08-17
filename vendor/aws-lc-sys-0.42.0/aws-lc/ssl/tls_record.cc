// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2002 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <assert.h>
#include <string.h>

#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "internal.h"
#include "../crypto/internal.h"


BSSL_NAMESPACE_BEGIN

// kMaxEmptyRecords is the number of consecutive, empty records that will be
// processed. Without this limit an attacker could send empty records at a
// faster rate than we can process and cause record processing to loop
// forever.
static const uint8_t kMaxEmptyRecords = 32;

// kMaxEarlyDataSkipped is the maximum number of rejected early data bytes that
// will be skipped. Without this limit an attacker could send records at a
// faster rate than we can process and cause trial decryption to loop forever.
// This value should be slightly above kMaxEarlyDataAccepted, which is measured
// in plaintext.
static const size_t kMaxEarlyDataSkipped = 16384;

// kMaxWarningAlerts is the number of consecutive warning alerts that will be
// processed.
static const uint8_t kMaxWarningAlerts = 4;

// ssl_needs_record_splitting returns one if |ssl|'s current outgoing cipher
// state needs record-splitting and zero otherwise.
static bool ssl_needs_record_splitting(const SSL *ssl) {
#if !defined(BORINGSSL_UNSAFE_FUZZER_MODE)
  return !ssl->s3->aead_write_ctx->is_null_cipher() &&
         ssl->s3->aead_write_ctx->ProtocolVersion() < TLS1_1_VERSION &&
         (ssl->mode & SSL_MODE_CBC_RECORD_SPLITTING) != 0 &&
         SSL_CIPHER_is_block_cipher(ssl->s3->aead_write_ctx->cipher());
#else
  return false;
#endif
}

bool ssl_record_sequence_update(uint8_t *seq, size_t seq_len) {
  for (size_t i = seq_len - 1; i < seq_len; i--) {
    ++seq[i];
    if (seq[i] != 0) {
      return true;
    }
  }
  OPENSSL_PUT_ERROR(SSL, ERR_R_OVERFLOW);
  return false;
}

size_t ssl_record_prefix_len(const SSL *ssl) {
  size_t header_len;
  if (SSL_is_dtls(ssl)) {
    header_len = DTLS1_RT_HEADER_LENGTH;
  } else {
    header_len = SSL3_RT_HEADER_LENGTH;
  }

  return header_len + ssl->s3->aead_read_ctx->ExplicitNonceLen();
}

size_t ssl_seal_align_prefix_len(const SSL *ssl) {
  if (SSL_is_dtls(ssl)) {
    return DTLS1_RT_HEADER_LENGTH + ssl->s3->aead_write_ctx->ExplicitNonceLen();
  }

  size_t ret =
      SSL3_RT_HEADER_LENGTH + ssl->s3->aead_write_ctx->ExplicitNonceLen();
  if (ssl_needs_record_splitting(ssl)) {
    ret += SSL3_RT_HEADER_LENGTH;
    ret += ssl_cipher_get_record_split_len(ssl->s3->aead_write_ctx->cipher());
  }
  return ret;
}

static ssl_open_record_t skip_early_data(SSL *ssl, uint8_t *out_alert,
                                         size_t consumed) {
  ssl->s3->early_data_skipped += consumed;
  if (ssl->s3->early_data_skipped < consumed) {
    ssl->s3->early_data_skipped = kMaxEarlyDataSkipped + 1;
  }

  if (ssl->s3->early_data_skipped > kMaxEarlyDataSkipped) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_TOO_MUCH_SKIPPED_EARLY_DATA);
    *out_alert = SSL_AD_UNEXPECTED_MESSAGE;
    return ssl_open_record_error;
  }

  return ssl_open_record_discard;
}

ssl_open_record_t tls_open_record(SSL *ssl, uint8_t *out_type,
                                  Span<uint8_t> *out, size_t *out_consumed,
                                  uint8_t *out_alert, Span<uint8_t> in) {
  *out_consumed = 0;
  if (ssl->s3->read_shutdown == ssl_shutdown_close_notify) {
    return ssl_open_record_close_notify;
  }

  // If there is an unprocessed handshake message or we are already buffering
  // too much, stop before decrypting another handshake record.
  if (!tls_can_accept_handshake_data(ssl, out_alert)) {
    return ssl_open_record_error;
  }

  CBS cbs = CBS(in);

  // Decode the record header.
  uint8_t type;
  uint16_t version, ciphertext_len;
  if (!CBS_get_u8(&cbs, &type) ||
      !CBS_get_u16(&cbs, &version) ||
      !CBS_get_u16(&cbs, &ciphertext_len)) {
    *out_consumed = SSL3_RT_HEADER_LENGTH;
    return ssl_open_record_partial;
  }

  bool version_ok;
  if (ssl->s3->aead_read_ctx->is_null_cipher()) {
    // Only check the first byte. Enforcing beyond that can prevent decoding
    // version negotiation failure alerts.
    version_ok = (version >> 8) == SSL3_VERSION_MAJOR;
  } else {
    version_ok = version == ssl->s3->aead_read_ctx->RecordVersion();
  }

  if (!version_ok) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_WRONG_VERSION_NUMBER);
    *out_alert = SSL_AD_PROTOCOL_VERSION;
    return ssl_open_record_error;
  }

  // Check the ciphertext length.
  if (ciphertext_len > SSL3_RT_MAX_ENCRYPTED_LENGTH) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_ENCRYPTED_LENGTH_TOO_LONG);
    *out_alert = SSL_AD_RECORD_OVERFLOW;
    return ssl_open_record_error;
  }

  // Extract the body.
  CBS body;
  if (!CBS_get_bytes(&cbs, &body, ciphertext_len)) {
    *out_consumed = SSL3_RT_HEADER_LENGTH + (size_t)ciphertext_len;
    return ssl_open_record_partial;
  }

  Span<const uint8_t> header = in.subspan(0, SSL3_RT_HEADER_LENGTH);
  ssl_do_msg_callback(ssl, 0 /* read */, SSL3_RT_HEADER, header);

  *out_consumed = in.size() - CBS_len(&cbs);

  if (ssl->s3->have_version &&
      ssl_protocol_version(ssl) >= TLS1_3_VERSION &&
      SSL_in_init(ssl) &&
      type == SSL3_RT_CHANGE_CIPHER_SPEC &&
      ciphertext_len == 1 &&
      CBS_data(&body)[0] == 1) {
    ssl->s3->empty_record_count++;
    if (ssl->s3->empty_record_count > kMaxEmptyRecords) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_TOO_MANY_EMPTY_FRAGMENTS);
      *out_alert = SSL_AD_UNEXPECTED_MESSAGE;
      return ssl_open_record_error;
    }
    return ssl_open_record_discard;
  }

  // Skip early data received when expecting a second ClientHello if we rejected
  // 0RTT.
  if (ssl->s3->skip_early_data &&
      ssl->s3->aead_read_ctx->is_null_cipher() &&
      type == SSL3_RT_APPLICATION_DATA) {
    return skip_early_data(ssl, out_alert, *out_consumed);
  }

  // Decrypt the body in-place.
  if (!ssl->s3->aead_read_ctx->Open(
          out, type, version, ssl->s3->read_sequence, header,
          MakeSpan(const_cast<uint8_t *>(CBS_data(&body)), CBS_len(&body)))) {
    if (ssl->s3->skip_early_data && !ssl->s3->aead_read_ctx->is_null_cipher()) {
      ERR_clear_error();
      return skip_early_data(ssl, out_alert, *out_consumed);
    }

    OPENSSL_PUT_ERROR(SSL, SSL_R_DECRYPTION_FAILED_OR_BAD_RECORD_MAC);
    *out_alert = SSL_AD_BAD_RECORD_MAC;
    return ssl_open_record_error;
  }

  ssl->s3->skip_early_data = false;

  if (!ssl_record_sequence_update(ssl->s3->read_sequence, 8)) {
    *out_alert = SSL_AD_INTERNAL_ERROR;
    return ssl_open_record_error;
  }

  // TLS 1.3 hides the record type inside the encrypted data.
  bool has_padding =
      !ssl->s3->aead_read_ctx->is_null_cipher() &&
      ssl->s3->aead_read_ctx->ProtocolVersion() >= TLS1_3_VERSION;

  // If there is padding, the plaintext limit includes the padding, but includes
  // extra room for the inner content type.
  size_t plaintext_limit =
      has_padding ? SSL3_RT_MAX_PLAIN_LENGTH + 1 : SSL3_RT_MAX_PLAIN_LENGTH;
  if (out->size() > plaintext_limit) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DATA_LENGTH_TOO_LONG);
    *out_alert = SSL_AD_RECORD_OVERFLOW;
    return ssl_open_record_error;
  }

  if (has_padding) {
    // The outer record type is always application_data.
    if (type != SSL3_RT_APPLICATION_DATA) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_INVALID_OUTER_RECORD_TYPE);
      *out_alert = SSL_AD_DECODE_ERROR;
      return ssl_open_record_error;
    }

    do {
      if (out->empty()) {
        OPENSSL_PUT_ERROR(SSL, SSL_R_DECRYPTION_FAILED_OR_BAD_RECORD_MAC);
        *out_alert = SSL_AD_DECRYPT_ERROR;
        return ssl_open_record_error;
      }
      type = out->back();
      *out = out->subspan(0, out->size() - 1);
    } while (type == 0);
  }

  // Limit the number of consecutive empty records.
  if (out->empty()) {
    ssl->s3->empty_record_count++;
    if (ssl->s3->empty_record_count > kMaxEmptyRecords) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_TOO_MANY_EMPTY_FRAGMENTS);
      *out_alert = SSL_AD_UNEXPECTED_MESSAGE;
      return ssl_open_record_error;
    }
    // Apart from the limit, empty records are returned up to the caller. This
    // allows the caller to reject records of the wrong type.
  } else {
    ssl->s3->empty_record_count = 0;
  }

  if (type == SSL3_RT_ALERT) {
    return ssl_process_alert(ssl, out_alert, *out);
  }

  // Handshake messages may not interleave with any other record type.
  if (type != SSL3_RT_HANDSHAKE &&
      tls_has_unprocessed_handshake_data(ssl)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_RECORD);
    *out_alert = SSL_AD_UNEXPECTED_MESSAGE;
    return ssl_open_record_error;
  }

  ssl->s3->warning_alert_count = 0;

  *out_type = type;
  return ssl_open_record_success;
}

static bool do_seal_record(SSL *ssl, uint8_t *out_prefix, uint8_t *out,
                           uint8_t *out_suffix, uint8_t type, const uint8_t *in,
                           const size_t in_len) {
  SSLAEADContext *aead = ssl->s3->aead_write_ctx.get();
  uint8_t *extra_in = NULL;
  size_t extra_in_len = 0;
  if (!aead->is_null_cipher() &&
      aead->ProtocolVersion() >= TLS1_3_VERSION) {
    // TLS 1.3 hides the actual record type inside the encrypted data.
    extra_in = &type;
    extra_in_len = 1;
  }

  size_t suffix_len, ciphertext_len;
  if (!aead->SuffixLen(&suffix_len, in_len, extra_in_len) ||
      !aead->CiphertextLen(&ciphertext_len, in_len, extra_in_len)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_RECORD_TOO_LARGE);
    return false;
  }

  assert(in == out || !buffers_alias(in, in_len, out, in_len));
  assert(!buffers_alias(in, in_len, out_prefix, ssl_record_prefix_len(ssl)));
  assert(!buffers_alias(in, in_len, out_suffix, suffix_len));

  if (extra_in_len) {
    out_prefix[0] = SSL3_RT_APPLICATION_DATA;
  } else {
    out_prefix[0] = type;
  }

  uint16_t record_version = aead->RecordVersion();

  out_prefix[1] = record_version >> 8;
  out_prefix[2] = record_version & 0xff;
  out_prefix[3] = ciphertext_len >> 8;
  out_prefix[4] = ciphertext_len & 0xff;
  Span<const uint8_t> header = MakeSpan(out_prefix, SSL3_RT_HEADER_LENGTH);

  if (!aead->SealScatter(out_prefix + SSL3_RT_HEADER_LENGTH, out, out_suffix,
                         out_prefix[0], record_version, ssl->s3->write_sequence,
                         header, in, in_len, extra_in, extra_in_len) ||
      !ssl_record_sequence_update(ssl->s3->write_sequence, 8)) {
    return false;
  }

  ssl_do_msg_callback(ssl, 1 /* write */, SSL3_RT_HEADER, header);
  return true;
}

static size_t tls_seal_scatter_prefix_len(const SSL *ssl, uint8_t type,
                                          size_t in_len) {
  size_t ret = SSL3_RT_HEADER_LENGTH;
  if (type == SSL3_RT_APPLICATION_DATA && in_len > 1 &&
      ssl_needs_record_splitting(ssl)) {
    // In the case of record splitting, the 1-byte record (of the 1/n-1 split)
    // will be placed in the prefix, as will four of the five bytes of the
    // record header for the main record. The final byte will replace the first
    // byte of the plaintext that was used in the small record.
    ret += ssl_cipher_get_record_split_len(ssl->s3->aead_write_ctx->cipher());
    ret += SSL3_RT_HEADER_LENGTH - 1;
  } else {
    ret += ssl->s3->aead_write_ctx->ExplicitNonceLen();
  }
  return ret;
}

static bool tls_seal_scatter_suffix_len(const SSL *ssl, size_t *out_suffix_len,
                                        uint8_t type, size_t in_len) {
  size_t extra_in_len = 0;
  if (!ssl->s3->aead_write_ctx->is_null_cipher() &&
      ssl->s3->aead_write_ctx->ProtocolVersion() >= TLS1_3_VERSION) {
    // TLS 1.3 adds an extra byte for encrypted record type.
    extra_in_len = 1;
  }
  // clang-format off
  if (type == SSL3_RT_APPLICATION_DATA &&
      in_len > 1 &&
      ssl_needs_record_splitting(ssl)) {
    // With record splitting enabled, the first byte gets sealed into a separate
    // record which is written into the prefix.
    in_len -= 1;
  }
  // clang-format on
  return ssl->s3->aead_write_ctx->SuffixLen(out_suffix_len, in_len, extra_in_len);
}

// tls_seal_scatter_record seals a new record of type |type| and body |in| and
// splits it between |out_prefix|, |out|, and |out_suffix|. Exactly
// |tls_seal_scatter_prefix_len| bytes are written to |out_prefix|, |in_len|
// bytes to |out|, and |tls_seal_scatter_suffix_len| bytes to |out_suffix|. It
// returns one on success and zero on error. If enabled,
// |tls_seal_scatter_record| implements TLS 1.0 CBC 1/n-1 record splitting and
// may write two records concatenated.
static bool tls_seal_scatter_record(SSL *ssl, uint8_t *out_prefix, uint8_t *out,
                                    uint8_t *out_suffix, uint8_t type,
                                    const uint8_t *in, size_t in_len) {
  if (type == SSL3_RT_APPLICATION_DATA && in_len > 1 &&
      ssl_needs_record_splitting(ssl)) {
    assert(ssl->s3->aead_write_ctx->ExplicitNonceLen() == 0);
    const size_t prefix_len = SSL3_RT_HEADER_LENGTH;

    // Write the 1-byte fragment into |out_prefix|.
    uint8_t *split_body = out_prefix + prefix_len;
    uint8_t *split_suffix = split_body + 1;

    if (!do_seal_record(ssl, out_prefix, split_body, split_suffix, type, in,
                        1)) {
      return false;
    }

    size_t split_record_suffix_len;
    if (!ssl->s3->aead_write_ctx->SuffixLen(&split_record_suffix_len, 1, 0)) {
      assert(false);
      return false;
    }
    const size_t split_record_len = prefix_len + 1 + split_record_suffix_len;
    assert(SSL3_RT_HEADER_LENGTH + ssl_cipher_get_record_split_len(
                                       ssl->s3->aead_write_ctx->cipher()) ==
           split_record_len);

    // Write the n-1-byte fragment. The header gets split between |out_prefix|
    // (header[:-1]) and |out| (header[-1:]).
    uint8_t tmp_prefix[SSL3_RT_HEADER_LENGTH];
    if (!do_seal_record(ssl, tmp_prefix, out + 1, out_suffix, type, in + 1,
                        in_len - 1)) {
      return false;
    }
    assert(tls_seal_scatter_prefix_len(ssl, type, in_len) ==
           split_record_len + SSL3_RT_HEADER_LENGTH - 1);
    OPENSSL_memcpy(out_prefix + split_record_len, tmp_prefix,
                   SSL3_RT_HEADER_LENGTH - 1);
    OPENSSL_memcpy(out, tmp_prefix + SSL3_RT_HEADER_LENGTH - 1, 1);
    return true;
  }

  return do_seal_record(ssl, out_prefix, out, out_suffix, type, in, in_len);
}

bool tls_seal_record(SSL *ssl, uint8_t *out, size_t *out_len,
                     size_t max_out_len, uint8_t type, const uint8_t *in,
                     size_t in_len) {
  if (buffers_alias(in, in_len, out, max_out_len)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_OUTPUT_ALIASES_INPUT);
    return false;
  }

  const size_t prefix_len = tls_seal_scatter_prefix_len(ssl, type, in_len);
  size_t suffix_len;
  if (!tls_seal_scatter_suffix_len(ssl, &suffix_len, type, in_len)) {
    return false;
  }
  if (in_len + prefix_len < in_len ||
      prefix_len + in_len + suffix_len < prefix_len + in_len) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_RECORD_TOO_LARGE);
    return false;
  }
  if (max_out_len < in_len + prefix_len + suffix_len) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BUFFER_TOO_SMALL);
    return false;
  }

  uint8_t *prefix = out;
  uint8_t *body = out + prefix_len;
  uint8_t *suffix = body + in_len;
  if (!tls_seal_scatter_record(ssl, prefix, body, suffix, type, in, in_len)) {
    return false;
  }

  *out_len = prefix_len + in_len + suffix_len;
  return true;
}

enum ssl_open_record_t ssl_process_alert(SSL *ssl, uint8_t *out_alert,
                                         Span<const uint8_t> in) {
  // Alerts records may not contain fragmented or multiple alerts.
  if (in.size() != 2) {
    *out_alert = SSL_AD_DECODE_ERROR;
    OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_ALERT);
    return ssl_open_record_error;
  }

  ssl_do_msg_callback(ssl, 0 /* read */, SSL3_RT_ALERT, in);

  const uint8_t alert_level = in[0];
  const uint8_t alert_descr = in[1];

  uint16_t alert = (alert_level << 8) | alert_descr;
  ssl_do_info_callback(ssl, SSL_CB_READ_ALERT, alert);

  if (alert_level == SSL3_AL_WARNING) {
    if (alert_descr == SSL_AD_CLOSE_NOTIFY) {
      ssl->s3->read_shutdown = ssl_shutdown_close_notify;
      return ssl_open_record_close_notify;
    }

    // Warning alerts do not exist in TLS 1.3, but RFC 8446 section 6.1
    // continues to define user_canceled as a signal to cancel the handshake,
    // without specifying how to handle it. JDK11 misuses it to signal
    // full-duplex connection close after the handshake. As a workaround, skip
    // user_canceled as in TLS 1.2. This matches NSS and OpenSSL.
    if (ssl->s3->have_version &&
        ssl_protocol_version(ssl) >= TLS1_3_VERSION &&
        alert_descr != SSL_AD_USER_CANCELLED) {
      *out_alert = SSL_AD_DECODE_ERROR;
      OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_ALERT);
      return ssl_open_record_error;
    }

    ssl->s3->warning_alert_count++;
    if (ssl->s3->warning_alert_count > kMaxWarningAlerts) {
      *out_alert = SSL_AD_UNEXPECTED_MESSAGE;
      OPENSSL_PUT_ERROR(SSL, SSL_R_TOO_MANY_WARNING_ALERTS);
      return ssl_open_record_error;
    }
    return ssl_open_record_discard;
  }

  if (alert_level == SSL3_AL_FATAL) {
    OPENSSL_PUT_ERROR(SSL, SSL_AD_REASON_OFFSET + alert_descr);
    ERR_add_error_dataf("SSL alert number %d", alert_descr);
    *out_alert = 0;  // No alert to send back to the peer.
    return ssl_open_record_error;
  }

  *out_alert = SSL_AD_ILLEGAL_PARAMETER;
  OPENSSL_PUT_ERROR(SSL, SSL_R_UNKNOWN_ALERT_TYPE);
  return ssl_open_record_error;
}

BSSL_NAMESPACE_END

using namespace bssl;

size_t SSL_max_seal_overhead(const SSL *ssl) {
  if (SSL_is_dtls(ssl)) {
    return dtls_max_seal_overhead(ssl, dtls1_use_current_epoch);
  }

  size_t ret = SSL3_RT_HEADER_LENGTH;
  ret += ssl->s3->aead_write_ctx->MaxOverhead();
  // TLS 1.3 needs an extra byte for the encrypted record type.
  if (!ssl->s3->aead_write_ctx->is_null_cipher() &&
      ssl->s3->aead_write_ctx->ProtocolVersion() >= TLS1_3_VERSION) {
    ret += 1;
  }
  if (ssl_needs_record_splitting(ssl)) {
    ret *= 2;
  }
  return ret;
}
