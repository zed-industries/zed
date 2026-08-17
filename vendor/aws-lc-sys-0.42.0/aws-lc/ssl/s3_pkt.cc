// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2002 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <algorithm>

#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/rand.h>

#include "../crypto/err/internal.h"
#include "../crypto/internal.h"
#include "internal.h"


BSSL_NAMESPACE_BEGIN

static int do_tls_write(SSL *ssl, size_t *out_bytes_written, uint8_t type,
                        Span<const uint8_t> in);

int tls_write_app_data(SSL *ssl, bool *out_needs_handshake,
                       size_t *out_bytes_written, Span<const uint8_t> in) {
  assert(ssl_can_write(ssl));
  assert(!ssl->s3->aead_write_ctx->is_null_cipher());

  *out_needs_handshake = false;

  if (ssl->s3->write_shutdown != ssl_shutdown_none) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_PROTOCOL_IS_SHUTDOWN);
    return -1;
  }

  size_t total_bytes_written = ssl->s3->unreported_bytes_written;
  if (in.size() < total_bytes_written) {
    // This can happen if the caller disables |SSL_MODE_ENABLE_PARTIAL_WRITE|,
    // asks us to write some input of length N, we successfully encrypt M bytes
    // and write it, but fail to write the rest. We will report
    // |SSL_ERROR_WANT_WRITE|. If the caller then retries with fewer than M
    // bytes, we cannot satisfy that request. The caller is required to always
    // retry with at least as many bytes as the previous attempt.
    OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_LENGTH);
    return -1;
  }

  in = in.subspan(total_bytes_written);

  const bool is_early_data_write =
      !ssl->server && SSL_in_early_data(ssl) && ssl->s3->hs->can_early_write;
  for (;;) {
    size_t max_send_fragment = ssl->max_send_fragment;
    if (is_early_data_write) {
      SSL_HANDSHAKE *hs = ssl->s3->hs.get();
      if (hs->early_data_written >= hs->early_session->ticket_max_early_data) {
        ssl->s3->unreported_bytes_written = total_bytes_written;
        hs->can_early_write = false;
        *out_needs_handshake = true;
        return -1;
      }
      max_send_fragment = std::min(
          max_send_fragment, size_t{hs->early_session->ticket_max_early_data -
                                    hs->early_data_written});
    }

    const size_t to_write = std::min(max_send_fragment, in.size());
    size_t bytes_written;
    int ret = do_tls_write(ssl, &bytes_written, SSL3_RT_APPLICATION_DATA,
                           in.subspan(0, to_write));
    if (ret <= 0) {
      ssl->s3->unreported_bytes_written = total_bytes_written;
      return ret;
    }

    // Note |bytes_written| may be less than |to_write| if there was a pending
    // record from a smaller write attempt.
    assert(bytes_written <= to_write);
    total_bytes_written += bytes_written;
    in = in.subspan(bytes_written);
    if (is_early_data_write) {
      ssl->s3->hs->early_data_written += bytes_written;
    }

    if (in.empty() || (ssl->mode & SSL_MODE_ENABLE_PARTIAL_WRITE)) {
      ssl->s3->unreported_bytes_written = 0;
      *out_bytes_written = total_bytes_written;
      return 1;
    }
  }
}

// do_tls_write writes an SSL record of the given type. On success, it sets
// |*out_bytes_written| to number of bytes successfully written and returns one.
// On error, it returns a value <= 0 from the underlying |BIO|.
static int do_tls_write(SSL *ssl, size_t *out_bytes_written, uint8_t type,
                        Span<const uint8_t> in) {
  // If there is a pending write, the retry must be consistent.
  if (!ssl->s3->pending_write.empty() &&
      (ssl->s3->pending_write.size() > in.size() ||
       (!(ssl->mode & SSL_MODE_ACCEPT_MOVING_WRITE_BUFFER) &&
        ssl->s3->pending_write.data() != in.data()) ||
       ssl->s3->pending_write_type != type)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_WRITE_RETRY);
    return -1;
  }

  // Flush any unwritten data to the transport. There may be data to flush even
  // if |wpend_tot| is zero.
  int ret = ssl_write_buffer_flush(ssl);
  if (ret <= 0) {
    return ret;
  }

  // If there is a pending write, we just completed it. Report it to the caller.
  if (!ssl->s3->pending_write.empty()) {
    *out_bytes_written = ssl->s3->pending_write.size();
    ssl->s3->pending_write = {};
    return 1;
  }

  SSLBuffer *buf = &ssl->s3->write_buffer;
  if (in.size() > SSL3_RT_MAX_PLAIN_LENGTH || buf->size() > 0) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return -1;
  }

  if (!tls_flush_pending_hs_data(ssl)) {
    return -1;
  }

  // We may have unflushed handshake data that must be written before |in|. This
  // may be a KeyUpdate acknowledgment, 0-RTT key change messages, or a
  // NewSessionTicket.
  Span<const uint8_t> pending_flight;
  if (ssl->s3->pending_flight != nullptr) {
    pending_flight = MakeConstSpan(
        reinterpret_cast<const uint8_t *>(ssl->s3->pending_flight->data),
        ssl->s3->pending_flight->length);
    pending_flight = pending_flight.subspan(ssl->s3->pending_flight_offset);
  }

  size_t max_out = pending_flight.size();
  if (!in.empty()) {
    const size_t max_ciphertext_len = in.size() + SSL_max_seal_overhead(ssl);
    if (max_ciphertext_len < in.size() ||
        max_out + max_ciphertext_len < max_out) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_OVERFLOW);
      return -1;
    }
    max_out += max_ciphertext_len;
  }

  if (max_out == 0) {
    // Nothing to write.
    *out_bytes_written = 0;
    return 1;
  }

  if (!buf->EnsureCap(pending_flight.size() + ssl_seal_align_prefix_len(ssl),
                      max_out)) {
    return -1;
  }

  // Copy |pending_flight| to the output.
  if (!pending_flight.empty()) {
    OPENSSL_memcpy(buf->remaining().data(), pending_flight.data(),
                   pending_flight.size());
    ssl->s3->pending_flight.reset();
    ssl->s3->pending_flight_offset = 0;
    buf->DidWrite(pending_flight.size());
  }

  if (!in.empty()) {
    size_t ciphertext_len;
    if (!tls_seal_record(ssl, buf->remaining().data(), &ciphertext_len,
                         buf->remaining().size(), type, in.data(), in.size())) {
      return -1;
    }
    buf->DidWrite(ciphertext_len);
  }

  // Now that we've made progress on the connection, uncork KeyUpdate
  // acknowledgments.
  ssl->s3->key_update_pending = SSL_KEY_UPDATE_NONE;

  // Flush the write buffer.
  ret = ssl_write_buffer_flush(ssl);
  if (ret <= 0) {
    // Track the unfinished write.
    if (!in.empty()) {
      ssl->s3->pending_write = in;
      ssl->s3->pending_write_type = type;
    }
    return ret;
  }

  *out_bytes_written = in.size();
  return 1;
}

ssl_open_record_t tls_open_app_data(SSL *ssl, Span<uint8_t> *out,
                                    size_t *out_consumed, uint8_t *out_alert,
                                    Span<uint8_t> in) {
  assert(ssl_can_read(ssl));
  assert(!ssl->s3->aead_read_ctx->is_null_cipher());

  uint8_t type;
  Span<uint8_t> body;
  auto ret = tls_open_record(ssl, &type, &body, out_consumed, out_alert, in);
  if (ret != ssl_open_record_success) {
    return ret;
  }

  const bool is_early_data_read = ssl->server && SSL_in_early_data(ssl);

  if (type == SSL3_RT_HANDSHAKE) {
    // Post-handshake data prior to TLS 1.3 is always renegotiation, which we
    // never accept as a server. Otherwise |tls_get_message| will send
    // |SSL_R_EXCESSIVE_MESSAGE_SIZE|.
    if (ssl->server && ssl_protocol_version(ssl) < TLS1_3_VERSION) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_NO_RENEGOTIATION);
      *out_alert = SSL_AD_NO_RENEGOTIATION;
      return ssl_open_record_error;
    }

    if (!tls_append_handshake_data(ssl, body)) {
      *out_alert = SSL_AD_INTERNAL_ERROR;
      return ssl_open_record_error;
    }
    return ssl_open_record_discard;
  }

  if (type != SSL3_RT_APPLICATION_DATA) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_RECORD);
    *out_alert = SSL_AD_UNEXPECTED_MESSAGE;
    return ssl_open_record_error;
  }

  if (is_early_data_read) {
    if (body.size() > kMaxEarlyDataAccepted - ssl->s3->hs->early_data_read) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_TOO_MUCH_READ_EARLY_DATA);
      *out_alert = SSL3_AD_UNEXPECTED_MESSAGE;
      return ssl_open_record_error;
    }

    ssl->s3->hs->early_data_read += body.size();
  }

  if (body.empty()) {
    return ssl_open_record_discard;
  }

  *out = body;
  return ssl_open_record_success;
}

ssl_open_record_t tls_open_change_cipher_spec(SSL *ssl, size_t *out_consumed,
                                              uint8_t *out_alert,
                                              Span<uint8_t> in) {
  uint8_t type;
  Span<uint8_t> body;
  auto ret = tls_open_record(ssl, &type, &body, out_consumed, out_alert, in);
  if (ret != ssl_open_record_success) {
    return ret;
  }

  if (type != SSL3_RT_CHANGE_CIPHER_SPEC) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_RECORD);
    *out_alert = SSL_AD_UNEXPECTED_MESSAGE;
    return ssl_open_record_error;
  }

  if (body.size() != 1 || body[0] != SSL3_MT_CCS) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_CHANGE_CIPHER_SPEC);
    *out_alert = SSL_AD_ILLEGAL_PARAMETER;
    return ssl_open_record_error;
  }

  ssl_do_msg_callback(ssl, 0 /* read */, SSL3_RT_CHANGE_CIPHER_SPEC, body);
  return ssl_open_record_success;
}

void ssl_send_alert(SSL *ssl, int level, int desc) {
  // This function is called in response to a fatal error from the peer. Ignore
  // any failures writing the alert and report only the original error. In
  // particular, if the transport uses |SSL_write|, our existing error will be
  // clobbered so we must save and restore the error queue. See
  // https://crbug.com/959305.
  //
  // TODO(davidben): Return the alert out of the handshake, rather than calling
  // this function internally everywhere.
  //
  // TODO(davidben): This does not allow retrying if the alert hit EAGAIN. See
  // https://crbug.com/boringssl/130.
  UniquePtr<ERR_SAVE_STATE> err_state(ERR_save_state());
  ssl_send_alert_impl(ssl, level, desc);
  ERR_restore_state(err_state.get());
}

int ssl_send_alert_impl(SSL *ssl, int level, int desc) {
  // It is illegal to send an alert when we've already sent a closing one.
  if (ssl->s3->write_shutdown != ssl_shutdown_none) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_PROTOCOL_IS_SHUTDOWN);
    return -1;
  }

  if (level == SSL3_AL_WARNING && desc == SSL_AD_CLOSE_NOTIFY) {
    ssl->s3->write_shutdown = ssl_shutdown_close_notify;
  } else {
    assert(level == SSL3_AL_FATAL);
    assert(desc != SSL_AD_CLOSE_NOTIFY);
    ssl->s3->write_shutdown = ssl_shutdown_error;
  }

  ssl->s3->alert_dispatch = true;
  ssl->s3->send_alert[0] = level;
  ssl->s3->send_alert[1] = desc;
  if (ssl->s3->write_buffer.empty()) {
    // Nothing is being written out, so the alert may be dispatched
    // immediately.
    return ssl->method->dispatch_alert(ssl);
  }

  // The alert will be dispatched later.
  return -1;
}

int tls_dispatch_alert(SSL *ssl) {
  if (ssl->quic_method) {
    if (!ssl->quic_method->send_alert(ssl, ssl->s3->write_level,
                                      ssl->s3->send_alert[1])) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_QUIC_INTERNAL_ERROR);
      return 0;
    }
  } else {
    size_t bytes_written;
    int ret =
        do_tls_write(ssl, &bytes_written, SSL3_RT_ALERT, ssl->s3->send_alert);
    if (ret <= 0) {
      return ret;
    }
    assert(bytes_written == 2);
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
