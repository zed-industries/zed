// DTLS implementation written by Nagendra Modadugu (nagendra@cs.stanford.edu) for the OpenSSL project 2005.
// Copyright (c) 1998-2005 The OpenSSL Project.  All rights reserved.
// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <assert.h>
#include <string.h>

#include <openssl/bytestring.h>
#include <openssl/err.h>

#include "internal.h"
#include "../crypto/internal.h"


BSSL_NAMESPACE_BEGIN

// to_u64_be treats |in| as a 8-byte big-endian integer and returns the value as
// a |uint64_t|.
static uint64_t to_u64_be(const uint8_t in[8]) {
  uint64_t ret = 0;
  unsigned i;
  for (i = 0; i < 8; i++) {
    ret <<= 8;
    ret |= in[i];
  }
  return ret;
}

// dtls1_bitmap_should_discard returns one if |seq_num| has been seen in
// |bitmap| or is stale. Otherwise it returns zero.
static bool dtls1_bitmap_should_discard(DTLS1_BITMAP *bitmap,
                                        const uint8_t seq_num[8]) {
  const size_t kWindowSize = bitmap->map.size();

  uint64_t seq_num_u = to_u64_be(seq_num);
  if (seq_num_u > bitmap->max_seq_num) {
    return false;
  }
  uint64_t idx = bitmap->max_seq_num - seq_num_u;
  return idx >= kWindowSize || bitmap->map[idx];
}

// dtls1_bitmap_record updates |bitmap| to record receipt of sequence number
// |seq_num|. It slides the window forward if needed. It is an error to call
// this function on a stale sequence number.
static void dtls1_bitmap_record(DTLS1_BITMAP *bitmap,
                                const uint8_t seq_num[8]) {
  const size_t kWindowSize = bitmap->map.size();

  uint64_t seq_num_u = to_u64_be(seq_num);
  // Shift the window if necessary.
  if (seq_num_u > bitmap->max_seq_num) {
    uint64_t shift = seq_num_u - bitmap->max_seq_num;
    if (shift >= kWindowSize) {
      bitmap->map.reset();
    } else {
      bitmap->map <<= shift;
    }
    bitmap->max_seq_num = seq_num_u;
  }

  uint64_t idx = bitmap->max_seq_num - seq_num_u;
  if (idx < kWindowSize) {
    bitmap->map[idx] = true;
  }
}

enum ssl_open_record_t dtls_open_record(SSL *ssl, uint8_t *out_type,
                                        Span<uint8_t> *out,
                                        size_t *out_consumed,
                                        uint8_t *out_alert, Span<uint8_t> in) {
  *out_consumed = 0;
  if (ssl->s3->read_shutdown == ssl_shutdown_close_notify) {
    return ssl_open_record_close_notify;
  }

  if (in.empty()) {
    return ssl_open_record_partial;
  }

  CBS cbs = CBS(in);

  // Decode the record.
  uint8_t type;
  uint16_t version;
  uint8_t sequence[8];
  CBS body;
  if (!CBS_get_u8(&cbs, &type) ||
      !CBS_get_u16(&cbs, &version) ||
      !CBS_copy_bytes(&cbs, sequence, 8) ||
      !CBS_get_u16_length_prefixed(&cbs, &body) ||
      CBS_len(&body) > SSL3_RT_MAX_ENCRYPTED_LENGTH) {
    // The record header was incomplete or malformed. Drop the entire packet.
    *out_consumed = in.size();
    return ssl_open_record_discard;
  }

  bool version_ok;
  if (ssl->s3->aead_read_ctx->is_null_cipher()) {
    // Only check the first byte. Enforcing beyond that can prevent decoding
    // version negotiation failure alerts.
    version_ok = (version >> 8) == DTLS1_VERSION_MAJOR;
  } else {
    version_ok = version == ssl->s3->aead_read_ctx->RecordVersion();
  }

  if (!version_ok) {
    // The record header was incomplete or malformed. Drop the entire packet.
    *out_consumed = in.size();
    return ssl_open_record_discard;
  }

  Span<const uint8_t> header = in.subspan(0, DTLS1_RT_HEADER_LENGTH);
  ssl_do_msg_callback(ssl, 0 /* read */, SSL3_RT_HEADER, header);

  uint16_t epoch = (((uint16_t)sequence[0]) << 8) | sequence[1];
  if (epoch != ssl->d1->r_epoch ||
      dtls1_bitmap_should_discard(&ssl->d1->bitmap, sequence)) {
    // Drop this record. It's from the wrong epoch or is a replay. Note that if
    // |epoch| is the next epoch, the record could be buffered for later. For
    // simplicity, drop it and expect retransmit to handle it later; DTLS must
    // handle packet loss anyway.
    *out_consumed = in.size() - CBS_len(&cbs);
    return ssl_open_record_discard;
  }

  // discard the body in-place.
  if (!ssl->s3->aead_read_ctx->Open(
          out, type, version, sequence, header,
          MakeSpan(const_cast<uint8_t *>(CBS_data(&body)), CBS_len(&body)))) {
    // Bad packets are silently dropped in DTLS. See section 4.2.1 of RFC 6347.
    // Clear the error queue of any errors decryption may have added. Drop the
    // entire packet as it must not have come from the peer.
    //
    // TODO(davidben): This doesn't distinguish malloc failures from encryption
    // failures.
    ERR_clear_error();
    *out_consumed = in.size() - CBS_len(&cbs);
    return ssl_open_record_discard;
  }
  *out_consumed = in.size() - CBS_len(&cbs);

  // Check the plaintext length.
  if (out->size() > SSL3_RT_MAX_PLAIN_LENGTH) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DATA_LENGTH_TOO_LONG);
    *out_alert = SSL_AD_RECORD_OVERFLOW;
    return ssl_open_record_error;
  }

  dtls1_bitmap_record(&ssl->d1->bitmap, sequence);

  // TODO(davidben): Limit the number of empty records as in TLS? This is only
  // useful if we also limit discarded packets.

  if (type == SSL3_RT_ALERT) {
    return ssl_process_alert(ssl, out_alert, *out);
  }

  ssl->s3->warning_alert_count = 0;

  *out_type = type;
  return ssl_open_record_success;
}

static const SSLAEADContext *get_write_aead(const SSL *ssl,
                                            enum dtls1_use_epoch_t use_epoch) {
  if (use_epoch == dtls1_use_previous_epoch) {
    assert(ssl->d1->w_epoch >= 1);
    return ssl->d1->last_aead_write_ctx.get();
  }

  return ssl->s3->aead_write_ctx.get();
}

size_t dtls_max_seal_overhead(const SSL *ssl,
                              enum dtls1_use_epoch_t use_epoch) {
  return DTLS1_RT_HEADER_LENGTH + get_write_aead(ssl, use_epoch)->MaxOverhead();
}

size_t dtls_seal_prefix_len(const SSL *ssl, enum dtls1_use_epoch_t use_epoch) {
  return DTLS1_RT_HEADER_LENGTH +
         get_write_aead(ssl, use_epoch)->ExplicitNonceLen();
}

bool dtls_seal_record(SSL *ssl, uint8_t *out, size_t *out_len, size_t max_out,
                      uint8_t type, const uint8_t *in, size_t in_len,
                      enum dtls1_use_epoch_t use_epoch) {
  const size_t prefix = dtls_seal_prefix_len(ssl, use_epoch);
  if (buffers_alias(in, in_len, out, max_out) &&
      (max_out < prefix || out + prefix != in)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_OUTPUT_ALIASES_INPUT);
    return false;
  }

  // Determine the parameters for the current epoch.
  uint16_t epoch = ssl->d1->w_epoch;
  SSLAEADContext *aead = ssl->s3->aead_write_ctx.get();
  uint8_t *seq = ssl->s3->write_sequence;
  if (use_epoch == dtls1_use_previous_epoch) {
    assert(ssl->d1->w_epoch >= 1);
    epoch = ssl->d1->w_epoch - 1;
    aead = ssl->d1->last_aead_write_ctx.get();
    seq = ssl->d1->last_write_sequence;
  }

  if (max_out < DTLS1_RT_HEADER_LENGTH) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BUFFER_TOO_SMALL);
    return false;
  }

  out[0] = type;

  uint16_t record_version = ssl->s3->aead_write_ctx->RecordVersion();
  out[1] = record_version >> 8;
  out[2] = record_version & 0xff;

  out[3] = epoch >> 8;
  out[4] = epoch & 0xff;
  OPENSSL_memcpy(&out[5], &seq[2], 6);

  size_t ciphertext_len;
  if (!aead->CiphertextLen(&ciphertext_len, in_len, 0)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_RECORD_TOO_LARGE);
    return false;
  }
  out[11] = ciphertext_len >> 8;
  out[12] = ciphertext_len & 0xff;
  Span<const uint8_t> header = MakeConstSpan(out, DTLS1_RT_HEADER_LENGTH);

  size_t len_copy;
  if (!aead->Seal(out + DTLS1_RT_HEADER_LENGTH, &len_copy,
                  max_out - DTLS1_RT_HEADER_LENGTH, type, record_version,
                  &out[3] /* seq */, header, in, in_len) ||
      !ssl_record_sequence_update(&seq[2], 6)) {
    return false;
  }
  assert(ciphertext_len == len_copy);

  *out_len = DTLS1_RT_HEADER_LENGTH + ciphertext_len;
  ssl_do_msg_callback(ssl, 1 /* write */, SSL3_RT_HEADER, header);
  return true;
}

BSSL_NAMESPACE_END
