// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2002 The OpenSSL Project.  All rights reserved.
// Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
//
// ECC cipher suite support in OpenSSL originally developed by
// SUN MICROSYSTEMS, INC., and contributed to the OpenSSL project.
//
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <tuple>

#include <openssl/buf.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/md5.h>
#include <openssl/nid.h>
#include <openssl/rand.h>
#include <openssl/sha.h>

#include "../crypto/internal.h"
#include "internal.h"


BSSL_NAMESPACE_BEGIN

static bool add_record_to_flight(SSL *ssl, uint8_t type,
                                 Span<const uint8_t> in) {
  // The caller should have flushed |pending_hs_data| first.
  assert(!ssl->s3->pending_hs_data);
  // We'll never add a flight while in the process of writing it out.
  assert(ssl->s3->pending_flight_offset == 0);

  if (ssl->s3->pending_flight == nullptr) {
    ssl->s3->pending_flight.reset(BUF_MEM_new());
    if (ssl->s3->pending_flight == nullptr) {
      return false;
    }
  }

  size_t max_out = in.size() + SSL_max_seal_overhead(ssl);
  size_t new_cap = ssl->s3->pending_flight->length + max_out;
  if (max_out < in.size() || new_cap < max_out) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_OVERFLOW);
    return false;
  }

  size_t len;
  if (!BUF_MEM_reserve(ssl->s3->pending_flight.get(), new_cap) ||
      !tls_seal_record(ssl,
                       (uint8_t *)ssl->s3->pending_flight->data +
                           ssl->s3->pending_flight->length,
                       &len, max_out, type, in.data(), in.size())) {
    return false;
  }

  ssl->s3->pending_flight->length += len;
  return true;
}

bool tls_init_message(const SSL *ssl, CBB *cbb, CBB *body, uint8_t type) {
  // Pick a modest size hint to save most of the |realloc| calls.
  if (!CBB_init(cbb, 64) ||
      !CBB_add_u8(cbb, type) ||
      !CBB_add_u24_length_prefixed(cbb, body)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    CBB_cleanup(cbb);
    return false;
  }

  return true;
}

bool tls_finish_message(const SSL *ssl, CBB *cbb, Array<uint8_t> *out_msg) {
  return CBBFinishArray(cbb, out_msg);
}

bool tls_add_message(SSL *ssl, Array<uint8_t> msg) {
  // Pack handshake data into the minimal number of records. This avoids
  // unnecessary encryption overhead, notably in TLS 1.3 where we send several
  // encrypted messages in a row. For now, we do not do this for the null
  // cipher. The benefit is smaller and there is a risk of breaking buggy
  // implementations.
  //
  // TODO(davidben): See if we can do this uniformly.
  Span<const uint8_t> rest = msg;
  if (ssl->quic_method == nullptr &&
      ssl->s3->aead_write_ctx->is_null_cipher()) {
    while (!rest.empty()) {
      Span<const uint8_t> chunk = rest.subspan(0, ssl->max_send_fragment);
      rest = rest.subspan(chunk.size());

      if (!add_record_to_flight(ssl, SSL3_RT_HANDSHAKE, chunk)) {
        return false;
      }
    }
  } else {
    while (!rest.empty()) {
      // Flush if |pending_hs_data| is full.
      if (ssl->s3->pending_hs_data &&
          ssl->s3->pending_hs_data->length >= ssl->max_send_fragment &&
          !tls_flush_pending_hs_data(ssl)) {
        return false;
      }

      size_t pending_len =
          ssl->s3->pending_hs_data ? ssl->s3->pending_hs_data->length : 0;
      Span<const uint8_t> chunk =
          rest.subspan(0, ssl->max_send_fragment - pending_len);
      assert(!chunk.empty());
      rest = rest.subspan(chunk.size());

      if (!ssl->s3->pending_hs_data) {
        ssl->s3->pending_hs_data.reset(BUF_MEM_new());
      }
      if (!ssl->s3->pending_hs_data ||
          !BUF_MEM_append(ssl->s3->pending_hs_data.get(), chunk.data(),
                          chunk.size())) {
        return false;
      }
    }
  }

  ssl_do_msg_callback(ssl, 1 /* write */, SSL3_RT_HANDSHAKE, msg);
  // TODO(svaldez): Move this up a layer to fix abstraction for SSLTranscript on
  // hs.
  if (ssl->s3->hs != NULL &&
      !ssl->s3->hs->transcript.Update(msg)) {
    return false;
  }
  return true;
}

bool tls_flush_pending_hs_data(SSL *ssl) {
  if (!ssl->s3->pending_hs_data || ssl->s3->pending_hs_data->length == 0) {
    return true;
  }

  UniquePtr<BUF_MEM> pending_hs_data = std::move(ssl->s3->pending_hs_data);
  auto data =
      MakeConstSpan(reinterpret_cast<const uint8_t *>(pending_hs_data->data),
                    pending_hs_data->length);
  if (ssl->quic_method) {
    if ((ssl->s3->hs == nullptr || !ssl->s3->hs->hints_requested) &&
        !ssl->quic_method->add_handshake_data(ssl, ssl->s3->write_level,
                                              data.data(), data.size())) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_QUIC_INTERNAL_ERROR);
      return false;
    }
    return true;
  }

  return add_record_to_flight(ssl, SSL3_RT_HANDSHAKE, data);
}

bool tls_add_change_cipher_spec(SSL *ssl) {
  static const uint8_t kChangeCipherSpec[1] = {SSL3_MT_CCS};

  if (!tls_flush_pending_hs_data(ssl)) {
    return false;
  }

  if (!ssl->quic_method &&
      !add_record_to_flight(ssl, SSL3_RT_CHANGE_CIPHER_SPEC,
                            kChangeCipherSpec)) {
    return false;
  }

  ssl_do_msg_callback(ssl, 1 /* write */, SSL3_RT_CHANGE_CIPHER_SPEC,
                      kChangeCipherSpec);
  return true;
}

int tls_flush_flight(SSL *ssl) {
  if (!tls_flush_pending_hs_data(ssl)) {
    return -1;
  }

  if (ssl->quic_method) {
    if (ssl->s3->write_shutdown != ssl_shutdown_none) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_PROTOCOL_IS_SHUTDOWN);
      return -1;
    }

    if (!ssl->quic_method->flush_flight(ssl)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_QUIC_INTERNAL_ERROR);
      return -1;
    }
  }

  if (ssl->s3->pending_flight == nullptr) {
    return 1;
  }

  if (ssl->s3->write_shutdown != ssl_shutdown_none) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_PROTOCOL_IS_SHUTDOWN);
    return -1;
  }

  static_assert(INT_MAX <= 0xffffffff, "int is larger than 32 bits");
  if (ssl->s3->pending_flight->length > INT_MAX) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return -1;
  }

  // If there is pending data in the write buffer, it must be flushed out before
  // any new data in pending_flight.
  if (!ssl->s3->write_buffer.empty()) {
    int ret = ssl_write_buffer_flush(ssl);
    if (ret <= 0) {
      ssl->s3->rwstate = SSL_ERROR_WANT_WRITE;
      return ret;
    }
  }

  if (ssl->wbio == nullptr) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BIO_NOT_SET);
    return -1;
  }

  // Write the pending flight.
  while (ssl->s3->pending_flight_offset < ssl->s3->pending_flight->length) {
    int ret = BIO_write(
        ssl->wbio.get(),
        ssl->s3->pending_flight->data + ssl->s3->pending_flight_offset,
        ssl->s3->pending_flight->length - ssl->s3->pending_flight_offset);
    if (ret <= 0) {
      ssl->s3->rwstate = SSL_ERROR_WANT_WRITE;
      return ret;
    }

    ssl->s3->pending_flight_offset += ret;
  }

  if (BIO_flush(ssl->wbio.get()) <= 0) {
    ssl->s3->rwstate = SSL_ERROR_WANT_WRITE;
    return -1;
  }

  ssl->s3->pending_flight.reset();
  ssl->s3->pending_flight_offset = 0;
  return 1;
}

static ssl_open_record_t read_v2_client_hello(SSL *ssl, size_t *out_consumed,
                                              Span<const uint8_t> in) {
  *out_consumed = 0;
  assert(in.size() >= SSL3_RT_HEADER_LENGTH);
  // Determine the length of the V2ClientHello.
  size_t msg_length = ((in[0] & 0x7f) << 8) | in[1];
  if (msg_length > (1024 * 4)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_RECORD_TOO_LARGE);
    return ssl_open_record_error;
  }
  if (msg_length < SSL3_RT_HEADER_LENGTH - 2) {
    // Reject lengths that are too short early. We have already read
    // |SSL3_RT_HEADER_LENGTH| bytes, so we should not attempt to process an
    // (invalid) V2ClientHello which would be shorter than that.
    OPENSSL_PUT_ERROR(SSL, SSL_R_RECORD_LENGTH_MISMATCH);
    return ssl_open_record_error;
  }

  // Ask for the remainder of the V2ClientHello.
  if (in.size() < 2 + msg_length) {
    *out_consumed = 2 + msg_length;
    return ssl_open_record_partial;
  }

  CBS v2_client_hello = CBS(ssl->s3->read_buffer.span().subspan(2, msg_length));
  // The V2ClientHello without the length is incorporated into the handshake
  // hash. This is only ever called at the start of the handshake, so hs is
  // guaranteed to be non-NULL.
  if (!ssl->s3->hs->transcript.Update(v2_client_hello)) {
    return ssl_open_record_error;
  }

  ssl_do_msg_callback(ssl, 0 /* read */, 0 /* V2ClientHello */,
                      v2_client_hello);

  uint8_t msg_type;
  uint16_t version, cipher_spec_length, session_id_length, challenge_length;
  CBS cipher_specs, session_id, challenge;
  if (!CBS_get_u8(&v2_client_hello, &msg_type) ||
      !CBS_get_u16(&v2_client_hello, &version) ||
      !CBS_get_u16(&v2_client_hello, &cipher_spec_length) ||
      !CBS_get_u16(&v2_client_hello, &session_id_length) ||
      !CBS_get_u16(&v2_client_hello, &challenge_length) ||
      !CBS_get_bytes(&v2_client_hello, &cipher_specs, cipher_spec_length) ||
      !CBS_get_bytes(&v2_client_hello, &session_id, session_id_length) ||
      !CBS_get_bytes(&v2_client_hello, &challenge, challenge_length) ||
      CBS_len(&v2_client_hello) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
    return ssl_open_record_error;
  }

  // msg_type has already been checked.
  assert(msg_type == SSL2_MT_CLIENT_HELLO);

  // The client_random is the V2ClientHello challenge. Truncate or left-pad with
  // zeros as needed.
  size_t rand_len = CBS_len(&challenge);
  if (rand_len > SSL3_RANDOM_SIZE) {
    rand_len = SSL3_RANDOM_SIZE;
  }
  uint8_t random[SSL3_RANDOM_SIZE];
  OPENSSL_memset(random, 0, SSL3_RANDOM_SIZE);
  OPENSSL_memcpy(random + (SSL3_RANDOM_SIZE - rand_len), CBS_data(&challenge),
                 rand_len);

  // Write out an equivalent TLS ClientHello directly to the handshake buffer.
  size_t max_v3_client_hello = SSL3_HM_HEADER_LENGTH + 2 /* version */ +
                               SSL3_RANDOM_SIZE + 1 /* session ID length */ +
                               2 /* cipher list length */ +
                               CBS_len(&cipher_specs) / 3 * 2 +
                               1 /* compression length */ + 1 /* compression */;
  ScopedCBB client_hello;
  CBB hello_body, cipher_suites;
  if (!ssl->s3->hs_buf) {
    ssl->s3->hs_buf.reset(BUF_MEM_new());
  }
  if (!ssl->s3->hs_buf ||
      !BUF_MEM_reserve(ssl->s3->hs_buf.get(), max_v3_client_hello) ||
      !CBB_init_fixed(client_hello.get(), (uint8_t *)ssl->s3->hs_buf->data,
                      ssl->s3->hs_buf->max) ||
      !CBB_add_u8(client_hello.get(), SSL3_MT_CLIENT_HELLO) ||
      !CBB_add_u24_length_prefixed(client_hello.get(), &hello_body) ||
      !CBB_add_u16(&hello_body, version) ||
      !CBB_add_bytes(&hello_body, random, SSL3_RANDOM_SIZE) ||
      // No session id.
      !CBB_add_u8(&hello_body, 0) ||
      !CBB_add_u16_length_prefixed(&hello_body, &cipher_suites)) {
    return ssl_open_record_error;
  }

  // Copy the cipher suites.
  while (CBS_len(&cipher_specs) > 0) {
    uint32_t cipher_spec;
    if (!CBS_get_u24(&cipher_specs, &cipher_spec)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
      return ssl_open_record_error;
    }

    // Skip SSLv2 ciphers.
    if ((cipher_spec & 0xff0000) != 0) {
      continue;
    }
    if (!CBB_add_u16(&cipher_suites, cipher_spec)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return ssl_open_record_error;
    }
  }

  // Add the null compression scheme and finish.
  if (!CBB_add_u8(&hello_body, 1) ||
      !CBB_add_u8(&hello_body, 0) ||
      !CBB_finish(client_hello.get(), NULL, &ssl->s3->hs_buf->length)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return ssl_open_record_error;
  }

  *out_consumed = 2 + msg_length;
  ssl->s3->is_v2_hello = true;
  return ssl_open_record_success;
}

static bool parse_message(const SSL *ssl, SSLMessage *out,
                          size_t *out_bytes_needed) {
  if (!ssl->s3->hs_buf) {
    *out_bytes_needed = 4;
    return false;
  }

  CBS cbs;
  uint32_t len;
  CBS_init(&cbs, reinterpret_cast<const uint8_t *>(ssl->s3->hs_buf->data),
           ssl->s3->hs_buf->length);
  if (!CBS_get_u8(&cbs, &out->type) ||
      !CBS_get_u24(&cbs, &len)) {
    *out_bytes_needed = 4;
    return false;
  }

  if (!CBS_get_bytes(&cbs, &out->body, len)) {
    *out_bytes_needed = 4 + len;
    return false;
  }

  CBS_init(&out->raw, reinterpret_cast<const uint8_t *>(ssl->s3->hs_buf->data),
           4 + len);
  out->is_v2_hello = ssl->s3->is_v2_hello;
  return true;
}

bool tls_get_message(const SSL *ssl, SSLMessage *out) {
  size_t unused;
  if (!parse_message(ssl, out, &unused)) {
    return false;
  }
  if (!ssl->s3->has_message) {
    if (!out->is_v2_hello) {
      ssl_do_msg_callback(ssl, 0 /* read */, SSL3_RT_HANDSHAKE, out->raw);
    }
    ssl->s3->has_message = true;
  }
  return true;
}

bool tls_can_accept_handshake_data(const SSL *ssl, uint8_t *out_alert) {
  // If there is a complete message, the caller must have consumed it first.
  SSLMessage msg;
  size_t bytes_needed;
  if (parse_message(ssl, &msg, &bytes_needed)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    *out_alert = SSL_AD_INTERNAL_ERROR;
    return false;
  }

  // Enforce the limit so the peer cannot force us to buffer 16MB.
  if (bytes_needed > 4 + ssl_max_handshake_message_len(ssl)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_EXCESSIVE_MESSAGE_SIZE);
    *out_alert = SSL_AD_ILLEGAL_PARAMETER;
    return false;
  }

  return true;
}

bool tls_has_unprocessed_handshake_data(const SSL *ssl) {
  size_t msg_len = 0;
  if (ssl->s3->has_message) {
    SSLMessage msg;
    size_t unused;
    if (parse_message(ssl, &msg, &unused)) {
      msg_len = CBS_len(&msg.raw);
    }
  }

  return ssl->s3->hs_buf && ssl->s3->hs_buf->length > msg_len;
}

bool tls_append_handshake_data(SSL *ssl, Span<const uint8_t> data) {
  // Re-create the handshake buffer if needed.
  if (!ssl->s3->hs_buf) {
    ssl->s3->hs_buf.reset(BUF_MEM_new());
  }
  return ssl->s3->hs_buf &&
         BUF_MEM_append(ssl->s3->hs_buf.get(), data.data(), data.size());
}

ssl_open_record_t tls_open_handshake(SSL *ssl, size_t *out_consumed,
                                     uint8_t *out_alert, Span<uint8_t> in) {
  *out_consumed = 0;
  // Bypass the record layer for the first message to handle V2ClientHello.
  if (ssl->server && !ssl->s3->v2_hello_done) {
    // Ask for the first 5 bytes, the size of the TLS record header. This is
    // sufficient to detect a V2ClientHello and ensures that we never read
    // beyond the first record.
    if (in.size() < SSL3_RT_HEADER_LENGTH) {
      *out_consumed = SSL3_RT_HEADER_LENGTH;
      return ssl_open_record_partial;
    }

    // Some dedicated error codes for protocol mixups should the application
    // wish to interpret them differently. (These do not overlap with
    // ClientHello or V2ClientHello.)
    const char *str = reinterpret_cast<const char*>(in.data());
    if (strncmp("GET ", str, 4) == 0 ||
        strncmp("POST ", str, 5) == 0 ||
        strncmp("HEAD ", str, 5) == 0 ||
        strncmp("PUT ", str, 4) == 0) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_HTTP_REQUEST);
      *out_alert = 0;
      return ssl_open_record_error;
    }
    if (strncmp("CONNE", str, 5) == 0) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_HTTPS_PROXY_REQUEST);
      *out_alert = 0;
      return ssl_open_record_error;
    }

    // Check for a V2ClientHello.
    if ((in[0] & 0x80) != 0 && in[2] == SSL2_MT_CLIENT_HELLO &&
        in[3] == SSL3_VERSION_MAJOR) {
      auto ret = read_v2_client_hello(ssl, out_consumed, in);
      if (ret == ssl_open_record_error) {
        *out_alert = 0;
      } else if (ret == ssl_open_record_success) {
        ssl->s3->v2_hello_done = true;
      }
      return ret;
    }

    ssl->s3->v2_hello_done = true;
  }

  uint8_t type;
  Span<uint8_t> body;
  auto ret = tls_open_record(ssl, &type, &body, out_consumed, out_alert, in);
  if (ret != ssl_open_record_success) {
    return ret;
  }

  // WatchGuard's TLS 1.3 interference bug is very distinctive: they drop the
  // ServerHello and send the remaining encrypted application data records
  // as-is. This manifests as an application data record when we expect
  // handshake. Report a dedicated error code for this case.
  if (!ssl->server && type == SSL3_RT_APPLICATION_DATA &&
      ssl->s3->aead_read_ctx->is_null_cipher()) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_APPLICATION_DATA_INSTEAD_OF_HANDSHAKE);
    *out_alert = SSL_AD_UNEXPECTED_MESSAGE;
    return ssl_open_record_error;
  }

  if (type != SSL3_RT_HANDSHAKE) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_RECORD);
    *out_alert = SSL_AD_UNEXPECTED_MESSAGE;
    return ssl_open_record_error;
  }

  // Append the entire handshake record to the buffer.
  if (!tls_append_handshake_data(ssl, body)) {
    *out_alert = SSL_AD_INTERNAL_ERROR;
    return ssl_open_record_error;
  }

  return ssl_open_record_success;
}

void tls_next_message(SSL *ssl) {
  SSLMessage msg;
  if (!tls_get_message(ssl, &msg) ||
      !ssl->s3->hs_buf ||
      ssl->s3->hs_buf->length < CBS_len(&msg.raw)) {
    assert(0);
    return;
  }

  OPENSSL_memmove(ssl->s3->hs_buf->data,
                  ssl->s3->hs_buf->data + CBS_len(&msg.raw),
                  ssl->s3->hs_buf->length - CBS_len(&msg.raw));
  ssl->s3->hs_buf->length -= CBS_len(&msg.raw);
  ssl->s3->is_v2_hello = false;
  ssl->s3->has_message = false;

  // Post-handshake messages are rare, so release the buffer after every
  // message. During the handshake, |on_handshake_complete| will release it.
  if (!SSL_in_init(ssl) && ssl->s3->hs_buf->length == 0) {
    ssl->s3->hs_buf.reset();
  }
}

// CipherScorer produces a "score" for each possible cipher suite offered by
// the client.
class CipherScorer {
 public:
  CipherScorer(bool has_aes_hw) : aes_is_fine_(has_aes_hw) {}

  typedef std::tuple<bool, bool> Score;

  // MinScore returns a |Score| that will compare less than the score of all
  // cipher suites.
  Score MinScore() const {
    return Score(false, false);
  }

  Score Evaluate(const SSL_CIPHER *a) const {
    return Score(
        // Something is always preferable to nothing.
        true,
        // Either AES is fine, or else ChaCha20 is preferred.
        aes_is_fine_ || a->algorithm_enc == SSL_CHACHA20POLY1305);
  }

 private:
  const bool aes_is_fine_;
};

const SSL_CIPHER *ssl_choose_tls13_cipher(
    const STACK_OF(SSL_CIPHER) *client_cipher_suites, bool has_aes_hw, uint16_t version,
    const STACK_OF(SSL_CIPHER) *tls13_ciphers) {

  const SSL_CIPHER *best = nullptr;
  CipherScorer scorer(has_aes_hw);
  CipherScorer::Score best_score = scorer.MinScore();

  for (size_t i = 0; i < sk_SSL_CIPHER_num(client_cipher_suites); i++) {
    const SSL_CIPHER *client_cipher = sk_SSL_CIPHER_value(client_cipher_suites, i);
    const SSL_CIPHER *candidate = nullptr;
    if (tls13_ciphers != nullptr) {
      // Limit to configured TLS 1.3 ciphers
      for (size_t j = 0; j < sk_SSL_CIPHER_num(tls13_ciphers); j++) {
        const SSL_CIPHER *cipher = sk_SSL_CIPHER_value(tls13_ciphers, j);
        if (cipher != nullptr && cipher->id == client_cipher->id) {
          candidate = (SSL_CIPHER *)cipher;
          break;
        }
      }
    }
    if (candidate == nullptr ||
        SSL_CIPHER_get_min_version(candidate) > version ||
        SSL_CIPHER_get_max_version(candidate) < version) {
      continue;
    }

    const CipherScorer::Score candidate_score = scorer.Evaluate(candidate);
    // |candidate_score| must be larger to displace the current choice. That way
    // the client's order controls between ciphers with an equal score.
    if (candidate_score > best_score) {
      best = candidate;
      best_score = candidate_score;
    }
  }

  return best;
}

BSSL_NAMESPACE_END
