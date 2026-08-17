// DTLS implementation written by Nagendra Modadugu (nagendra@cs.stanford.edu) for the OpenSSL project 2005.
// Copyright (c) 1998-2005 The OpenSSL Project.  All rights reserved.
// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <assert.h>
#include <string.h>

#include <openssl/bio.h>
#include <openssl/bytestring.h>
#include <openssl/mem.h>
#include <openssl/evp.h>
#include <openssl/err.h>
#include <openssl/rand.h>

#include "../crypto/internal.h"
#include "internal.h"


BSSL_NAMESPACE_BEGIN

ssl_open_record_t dtls1_open_app_data(SSL *ssl, Span<uint8_t> *out,
                                      size_t *out_consumed, uint8_t *out_alert,
                                      Span<uint8_t> in) {
  assert(!SSL_in_init(ssl));

  uint8_t type;
  Span<uint8_t> record;
  auto ret = dtls_open_record(ssl, &type, &record, out_consumed, out_alert, in);
  if (ret != ssl_open_record_success) {
    return ret;
  }

  if (type == SSL3_RT_HANDSHAKE) {
    // Parse the first fragment header to determine if this is a pre-CCS or
    // post-CCS handshake record. DTLS resets handshake message numbers on each
    // handshake, so renegotiations and retransmissions are ambiguous.
    CBS cbs, body;
    struct hm_header_st msg_hdr;
    CBS_init(&cbs, record.data(), record.size());
    if (!dtls1_parse_fragment(&cbs, &msg_hdr, &body)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_HANDSHAKE_RECORD);
      *out_alert = SSL_AD_DECODE_ERROR;
      return ssl_open_record_error;
    }

    if (msg_hdr.type == SSL3_MT_FINISHED &&
        msg_hdr.seq == ssl->d1->handshake_read_seq - 1) {
      if (msg_hdr.frag_off == 0) {
        // Retransmit our last flight of messages. If the peer sends the second
        // Finished, they may not have received ours. Only do this for the
        // first fragment, in case the Finished was fragmented.
        if (!dtls1_check_timeout_num(ssl)) {
          *out_alert = 0;  // TODO(davidben): Send an alert?
          return ssl_open_record_error;
        }

        dtls1_retransmit_outgoing_messages(ssl);
      }
      return ssl_open_record_discard;
    }

    // Otherwise, this is a pre-CCS handshake message from an unsupported
    // renegotiation attempt. Fall through to the error path.
  }

  if (type != SSL3_RT_APPLICATION_DATA) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_RECORD);
    *out_alert = SSL_AD_UNEXPECTED_MESSAGE;
    return ssl_open_record_error;
  }

  if (record.empty()) {
    return ssl_open_record_discard;
  }

  *out = record;
  return ssl_open_record_success;
}

int dtls1_write_app_data(SSL *ssl, bool *out_needs_handshake,
                         size_t *out_bytes_written, Span<const uint8_t> in) {
  assert(!SSL_in_init(ssl));
  *out_needs_handshake = false;

  if (ssl->s3->write_shutdown != ssl_shutdown_none) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_PROTOCOL_IS_SHUTDOWN);
    return -1;
  }

  // DTLS does not split the input across records.
  if (in.size() > SSL3_RT_MAX_PLAIN_LENGTH) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DTLS_MESSAGE_TOO_BIG);
    return -1;
  }

  if (in.empty()) {
    *out_bytes_written = 0;
    return 1;
  }

  int ret = dtls1_write_record(ssl, SSL3_RT_APPLICATION_DATA, in,
                               dtls1_use_current_epoch);
  if (ret <= 0) {
    return ret;
  }
  *out_bytes_written = in.size();
  return 1;
}

int dtls1_write_record(SSL *ssl, int type, Span<const uint8_t> in,
                       enum dtls1_use_epoch_t use_epoch) {
  SSLBuffer *buf = &ssl->s3->write_buffer;
  assert(in.size() <= SSL3_RT_MAX_PLAIN_LENGTH);
  // There should never be a pending write buffer in DTLS. One can't write half
  // a datagram, so the write buffer is always dropped in
  // |ssl_write_buffer_flush|.
  assert(buf->empty());

  if (in.size() > SSL3_RT_MAX_PLAIN_LENGTH) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return -1;
  }

  size_t ciphertext_len;
  if (!buf->EnsureCap(ssl_seal_align_prefix_len(ssl),
                      in.size() + SSL_max_seal_overhead(ssl)) ||
      !dtls_seal_record(ssl, buf->remaining().data(), &ciphertext_len,
                        buf->remaining().size(), type, in.data(), in.size(),
                        use_epoch)) {
    buf->Clear();
    return -1;
  }
  buf->DidWrite(ciphertext_len);

  int ret = ssl_write_buffer_flush(ssl);
  if (ret <= 0) {
    return ret;
  }
  return 1;
}

int dtls1_dispatch_alert(SSL *ssl) {
  int ret = dtls1_write_record(ssl, SSL3_RT_ALERT, ssl->s3->send_alert,
                               dtls1_use_current_epoch);
  if (ret <= 0) {
    return ret;
  }
  ssl->s3->alert_dispatch = false;

  // If the alert is fatal, flush the BIO now.
  if (ssl->s3->send_alert[0] == SSL3_AL_FATAL) {
    BIO_flush(ssl->wbio.get());
  }

  ssl_do_msg_callback(ssl, 1 /* write */, SSL3_RT_ALERT, ssl->s3->send_alert);

  int alert = (ssl->s3->send_alert[0] << 8) | ssl->s3->send_alert[1];
  ssl_do_info_callback(ssl, SSL_CB_WRITE_ALERT, alert);

  return 1;
}

BSSL_NAMESPACE_END
