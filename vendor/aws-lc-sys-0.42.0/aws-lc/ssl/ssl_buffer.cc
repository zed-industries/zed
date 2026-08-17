// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/ssl.h>

#include <assert.h>
#include <limits.h>
#include <stdlib.h>
#include <string.h>

#include <openssl/bio.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../crypto/internal.h"
#include "internal.h"


BSSL_NAMESPACE_BEGIN

// BIO uses int instead of size_t. No lengths will exceed SSLBUFFER_MAX_CAPACITY
// (uint16_t), so this will not overflow.
static_assert(SSLBUFFER_MAX_CAPACITY <= INT_MAX,
              "uint16_t does not fit in int");

static_assert((SSL3_ALIGN_PAYLOAD & (SSL3_ALIGN_PAYLOAD - 1)) == 0,
              "SSL3_ALIGN_PAYLOAD must be a power of 2");

static OPENSSL_INLINE size_t compute_buffer_size(size_t capacity) {
  return capacity + SSL3_ALIGN_PAYLOAD - 1;
}

static OPENSSL_INLINE size_t compute_buffer_offset(size_t header_len,
                                                   uint8_t *buffer) {
  return (0 - header_len - (uintptr_t)buffer) & (SSL3_ALIGN_PAYLOAD - 1);
}

void SSLBuffer::Clear() {
  if (buf_allocated_) {
    free(buf_);  // Allocated with malloc().
  }
  buf_ = nullptr;
  buf_allocated_ = false;
  buf_cap_ = 0;
  buf_size_ = 0;
  header_len_ = 0;
  offset_ = 0;
  size_ = 0;
  cap_ = 0;
  max_serialization_version_ = kSSLBufferMaxSerDeVersion;
}

bool SSLBuffer::EnsureCap(size_t header_len, size_t new_cap) {
  if (new_cap > SSLBUFFER_MAX_CAPACITY) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  if ((size_t)cap_ >= new_cap) {
    return true;
  }

  uint8_t *new_buf;
  bool new_buf_allocated;
  size_t new_offset;
  if (new_cap <= sizeof(inline_buf_)) {
    // This function is called twice per TLS record, first for the five-byte
    // header. To avoid allocating twice, use an inline buffer for short inputs.
    new_buf = inline_buf_;
    new_buf_allocated = false;
    new_offset = 0;
    buf_size_ = sizeof(inline_buf_);
  } else {
    // Add up to |SSL3_ALIGN_PAYLOAD| - 1 bytes of slack for alignment.
    //
    // Since this buffer gets allocated quite frequently and doesn't contain any
    // sensitive data, we allocate with malloc rather than |OPENSSL_malloc| and
    // avoid zeroing on free.
    buf_size_ = compute_buffer_size(new_cap);
    new_buf = (uint8_t *)malloc(buf_size_);
    if (new_buf == NULL) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return false;
    }
    new_buf_allocated = true;
    // Offset the buffer such that the record body is aligned.
    new_offset = compute_buffer_offset(header_len, new_buf);
  }

  // Note if the both old and new buffer are inline, the source and destination
  // may alias.
  OPENSSL_memmove(new_buf + new_offset, buf_ + offset_, size_);

  if (buf_allocated_) {
    free(buf_);  // Allocated with malloc().
  }

  buf_ = new_buf;
  buf_allocated_ = new_buf_allocated;
  header_len_ = header_len;
  offset_ = new_offset;
  buf_cap_ = cap_ = new_cap;
  max_serialization_version_ = kSSLBufferMaxSerDeVersion;
  return true;
}

void SSLBuffer::DidWrite(size_t new_size) {
  if (new_size > cap() - size()) {
    abort();
  }
  size_ += new_size;
}

void SSLBuffer::Consume(size_t len) {
  if (len > (size_t)size_) {
    abort();
  }
  offset_ += (int)len;
  size_ -= (int)len;
  cap_ -= (int)len;
}

void SSLBuffer::DiscardConsumed() {
  if (size_ == 0) {
    Clear();
  }
}

// An SSLBuffer is serialized as the following ASN.1 structure:
//
// -- The V2 Serialization
// SSLBuffer ::= SEQUENCE {
//    version                           INTEGER (2),  -- SSLBuffer structure
//    bufAllocated                      BOOLEAN,
//    bufferCapacity                    INTEGER,
//    headerLength                      INTEGER,
//    relativeOffset                    INTEGER,
//    remainingCapacity                 INTEGER,
//    buf                               OCTET STRING,
// }
//
// -- The V1 Serialization
// SSLBuffer ::= SEQUENCE {
//    version                           INTEGER (1),  -- SSLBuffer structure
//    bufAllocated                      BOOLEAN,
//    offset                            INTEGER,
//    size                              INTEGER,
//    cap                               INTEGER,
//    buf                               OCTET STRING,
// }

bool SSLBuffer::DoSerializationV1(CBB &seq) {
  if (!CBB_add_asn1_uint64(&seq, SSL_BUFFER_SERDE_VERSION_ONE) ||
      !CBB_add_asn1_bool(&seq, (buf_allocated_ ? 1 : 0)) ||
      !CBB_add_asn1_uint64(&seq, offset_) ||
      !CBB_add_asn1_uint64(&seq, size_) || !CBB_add_asn1_uint64(&seq, cap_) ||
      (buf_allocated_ && !CBB_add_asn1_octet_string(&seq, buf_, buf_size_)) ||
      (!buf_allocated_ &&
       !CBB_add_asn1_octet_string(&seq, inline_buf_, SSL3_RT_HEADER_LENGTH))) {
    return false;
  }
  return true;
}

bool SSLBuffer::DoSerializationV2(CBB &seq) {
  size_t align_offset = 0;
  if (buf_allocated_) {
    align_offset = compute_buffer_offset(header_len_, buf_);
  }

  size_t rel_offset = offset_ - align_offset;

  if (!CBB_add_asn1_uint64(&seq, SSL_BUFFER_SERDE_VERSION_TWO) ||
      !CBB_add_asn1_bool(&seq, (buf_allocated_ ? 1 : 0)) ||
      !CBB_add_asn1_uint64(&seq, buf_cap_) ||
      !CBB_add_asn1_uint64(&seq, header_len_) ||
      !CBB_add_asn1_uint64(&seq, rel_offset) ||
      !CBB_add_asn1_uint64(&seq, cap_) ||
      (buf_allocated_ && !CBB_add_asn1_octet_string(&seq, buf_ + align_offset,
                                                    rel_offset + size_)) ||
      (!buf_allocated_ &&
       !CBB_add_asn1_octet_string(&seq, inline_buf_ + align_offset,
                                  rel_offset + size_))) {
    return false;
  }

  return true;
}

bool SSLBuffer::DoSerialization(CBB &cbb) {
  CBB seq;

  if (!CBB_add_asn1(&cbb, &seq, CBS_ASN1_SEQUENCE)) {
    return false;
  }

  switch (max_serialization_version_) {
    case SSL_BUFFER_SERDE_VERSION_TWO:
      if (!DoSerializationV2(seq)) {
        return false;
      }
      break;
    case SSL_BUFFER_SERDE_VERSION_ONE:
      if (!DoSerializationV1(seq)) {
        return false;
      }
      break;
    default:
      // Only possible with a programming error
      abort();
  }

  return CBB_flush(&cbb) == 1;
}

bool SSLBuffer::ValidateBuffersState() {
  const uint8_t *start_ptr = buf_;
  const uint8_t *end_ptr = buf_ + buf_size_;
  const uint8_t *data_start_ = buf_ + offset_;
  const uint8_t *data_end_ptr = data_start_ + size_;
  const uint8_t *remaining_ptr = data_end_ptr + (cap_ - size_);
  if (data_start_ > end_ptr || data_start_ < start_ptr ||
      data_end_ptr > end_ptr || data_end_ptr < start_ptr || size_ > cap_ ||
      remaining_ptr > end_ptr || remaining_ptr < start_ptr) {
    return false;
  }
  return true;
}

bool SSLBuffer::DoDeserialization(CBS &cbs) {
  CBS seq;
  uint64_t version = 0;
  if (!CBS_get_asn1(&cbs, &seq, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1_uint64(&seq, &version) ||
      version > kSSLBufferMaxSerDeVersion) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return false;
  }

  switch (version) {
    case SSL_BUFFER_SERDE_VERSION_TWO:
      return DoDeserializationV2(seq);
    case SSL_BUFFER_SERDE_VERSION_ONE:
      return DoDeserializationV1(seq);
    default:
      return false;
  }
}

bool SSLBuffer::DoDeserializationV1(CBS &cbs) {
  CBS buf;
  int buf_allocated_int = 0;
  uint64_t offset = 0, size = 0, cap = 0;
  if (!CBS_get_asn1_bool(&cbs, &buf_allocated_int) ||
      !CBS_get_asn1_uint64(&cbs, &offset) ||
      !CBS_get_asn1_uint64(&cbs, &size) || !CBS_get_asn1_uint64(&cbs, &cap) ||
      !CBS_get_asn1(&cbs, &buf, CBS_ASN1_OCTETSTRING) || CBS_len(&cbs) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return false;
  }

  if (offset > INT_MAX || size > INT_MAX || cap > INT_MAX) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return false;
  }

  // In the event that deserialization happens into an existing buffer.
  Clear();

  bool buf_allocated = !!buf_allocated_int;
  if (buf_allocated) {
    // When buf_allocated, CBS_len(&buf) should be larger than
    // sizeof(inline_buf_). This is ensured in |EnsureCap|.
    if (CBS_len(&buf) <= sizeof(inline_buf_)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
      return false;
    }
    buf_allocated_ = true;
    buf_ = (uint8_t *)malloc(CBS_len(&buf));
    if (buf_ == NULL) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return false;
    }
    buf_size_ = CBS_len(&buf);
    OPENSSL_memcpy(buf_, CBS_data(&buf), CBS_len(&buf));
  } else {
    if (CBS_len(&buf) != sizeof(inline_buf_)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
      return false;
    }
    buf_allocated_ = false;
    buf_ = inline_buf_;
    buf_size_ = sizeof(inline_buf_);
    OPENSSL_memcpy(inline_buf_, CBS_data(&buf), CBS_len(&buf));
  }
  buf_cap_ = header_len_ = 0;  // V1 was lossy :(
  offset_ = (int)offset;
  size_ = (int)size;
  cap_ = (int)cap;
  // As we restored from a V1 format we can only serialize as V1 until the next
  // |EnsureCap| call.
  max_serialization_version_ = SSL_BUFFER_SERDE_VERSION_ONE;

  // Final sanity check
  if(!ValidateBuffersState()) {
    Clear();
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return false;
  }

  return true;
}

bool SSLBuffer::DoDeserializationV2(CBS &cbs) {
  CBS buf;
  int buf_allocated_int = 0;
  uint64_t header_len = 0, buf_cap = 0, rel_offset = 0, cap = 0;
  size_t align_offset = 0;
  if (!CBS_get_asn1_bool(&cbs, &buf_allocated_int) ||
      !CBS_get_asn1_uint64(&cbs, &buf_cap) ||
      !CBS_get_asn1_uint64(&cbs, &header_len) ||
      !CBS_get_asn1_uint64(&cbs, &rel_offset) ||
      !CBS_get_asn1_uint64(&cbs, &cap) ||
      !CBS_get_asn1(&cbs, &buf, CBS_ASN1_OCTETSTRING) || CBS_len(&cbs) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return false;
  }

  if (buf_cap > SSLBUFFER_MAX_CAPACITY || rel_offset > INT_MAX ||
      rel_offset > CBS_len(&buf) || cap > INT_MAX) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return false;
  }

  // In the event that deserialization happens into an existing buffer.
  Clear();

  bool buf_allocated = !!buf_allocated_int;
  if (buf_allocated) {
    buf_allocated_ = true;
    buf_size_ = compute_buffer_size(buf_cap);
    buf_ = (uint8_t *)malloc(buf_size_);
    if (buf_ == NULL) {
      Clear();
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return false;
    }
    align_offset = compute_buffer_offset(header_len, buf_);
    if (rel_offset > INT_MAX - align_offset ||
        CBS_len(&buf) - rel_offset > INT_MAX ||
        CBS_len(&buf) > buf_size_ - align_offset) {
      Clear();
      OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
      return false;
    }
    OPENSSL_memcpy(buf_ + align_offset, CBS_data(&buf), CBS_len(&buf));
  } else {
    buf_allocated_ = false;
    buf_ = inline_buf_;
    buf_size_ = sizeof(inline_buf_);
    // We could relax this and just allocate a in-memory buffer with correct
    // alignment. But this value is not configurable (unlike
    // SSL3_ALIGN_PAYLOAD), so we can adjust this in the future if we modify
    // sizeof(inline_buf_);
    if (CBS_len(&buf) > sizeof(inline_buf_) || buf_cap > sizeof(inline_buf_)) {
      Clear();
      OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
      return false;
    }
    OPENSSL_memcpy(buf_, CBS_data(&buf), CBS_len(&buf));
  }
  buf_cap_ = (size_t)buf_cap;
  header_len_ = (size_t)header_len;
  offset_ = (int)(align_offset + rel_offset);
  size_ = (int)(CBS_len(&buf) - rel_offset);
  cap_ = (int)cap;
  max_serialization_version_ = SSL_BUFFER_SERDE_VERSION_TWO;

  // Final sanity check
  if (!ValidateBuffersState()) {
    Clear();
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return false;
  }

  return true;
}

static const unsigned kBufferViewOffsetFromDataPtr =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0;

bool SSLBuffer::SerializeBufferView(CBB &cbb, Span<uint8_t> &view) {
  // View must be a span that points into this buffer.
  if (!buf_ptr() || !view.data() ||
      view.data() <
          buf_ptr() ||  // does the start of the view fall before the buffer
      (view.data() + view.size()) <
          buf_ptr() ||  // does the end of the view fall before the buffer
      (buf_ptr() + buf_size()) <
          view.data() ||  // does the start of the view fall after the end of
                          // the buffer
      (buf_ptr() + buf_size()) <
          (view.data() + view.size())  // does the end of of the view fall after
                                       // the end of the buffer
  ) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return false;
  }

  // Important: The offset should not be relative to
  // |buf_ptr()|, as the alignment can change on restore.
  // It should be relative to |data()|, which can mean it may
  // currently point towards data before the current offset and thus be
  // negative.
  //
  // Ideally for simplicity we would make this relative to read buffer's
  // alignment offset, but we might not know this in the V1 case if the buffer
  // had been restored from a V1 format :\. So this is the better option at this
  // time.
  ptrdiff_t offset = view.data() - data();

  CBB child, child2;
  if (!CBB_add_asn1(&cbb, &child, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&child, &child2, kBufferViewOffsetFromDataPtr) ||
      !CBB_add_asn1_int64(&child2, offset) ||
      !CBB_add_asn1_uint64(&child, view.size())) {
    return false;
  }

  return CBB_flush(&cbb) == 1;
}

static bool deserialize_buffer_view_from_buf_ptr_offset(CBS &cbs,
                                                        SSLBuffer *buffer,
                                                        Span<uint8_t> &view) {
  if (!buffer) {
    abort();
  }
  uint64_t offset = 0, size = 0;
  if (!CBS_get_asn1_uint64(&cbs, &offset) ||
      !CBS_get_asn1_uint64(&cbs, &size)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return 0;
  }
  uint8_t *view_ptr = buffer->buf_ptr() + offset;
  if (view_ptr < buffer->buf_ptr() ||  // does the start of the view fall before
                                       // the buffer
      view_ptr > (buffer->buf_ptr() +
                  buffer->buf_size()) ||  // does the the start of the view fall
                                          // after the end of the buffer
      (view_ptr + size) <
          buffer->buf_ptr() ||  // does the end of the view fall
                                // before the start of the buffer
      (view_ptr + size) > (buffer->buf_ptr() +
                           buffer->buf_size())  // does the end of the view fall
                                                // after the end of the buffer

  ) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return false;
  }
  view = MakeSpan(view_ptr, size);
  return true;
}

static int fits_in_ptrdiff_int64(int64_t x) {
#if PTRDIFF_MAX > INT64_MAX || PTRDIFF_MAX == INT64_MAX
  // ptrdiff_t is equal to or wider than int64_t — all int64_t fit
  (void)x;
  return 1;
#else
  // ptrdiff_t is narrower than int64_t — cast to int64_t for safe compare
  return x >= (int64_t)PTRDIFF_MIN && x <= (int64_t)PTRDIFF_MAX;
#endif
}

static bool deserialize_buffer_view_from_data_offset(CBS &cbs,
                                                     SSLBuffer *buffer,
                                                     Span<uint8_t> &view) {
  if (!buffer) {
    abort();
  }
  CBS child;
  int64_t offset = 0;
  uint64_t size = 0;
  if (!CBS_get_asn1(&cbs, &child, kBufferViewOffsetFromDataPtr) ||
      !CBS_get_asn1_int64(&child, &offset) ||
      !CBS_get_asn1_uint64(&cbs, &size)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return 0;
  }
  if (!fits_in_ptrdiff_int64(offset)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return 0;
  }
  uint8_t *view_ptr = buffer->data() + (ptrdiff_t)offset;
  if (view_ptr <
          buffer->buf_ptr() ||  // does the view start fall before the buffer
      view_ptr >
          (buffer->buf_ptr() +
           buffer->buf_size()) ||  // does the view start fall after the buffer
      (view_ptr + size) < buffer->buf_ptr() ||  // does the end of the view fall
                                                // before the buffer start
      (view_ptr + size) > (buffer->buf_ptr() +
                           buffer->buf_size()  // does the end of the view fall
                                               // after the end of the buffer
                           )) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return 0;
  }
  view = MakeSpan(view_ptr, size);

  return true;
}


bool SSLBuffer::DeserializeBufferView(CBS &cbs, Span<uint8_t> &view) {
  CBS app_seq;

  if (!CBS_get_asn1(&cbs, &app_seq, CBS_ASN1_SEQUENCE)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return false;
  }

  if (CBS_peek_asn1_tag(&app_seq, CBS_ASN1_INTEGER)) {
    return deserialize_buffer_view_from_buf_ptr_offset(app_seq, this, view);
  } else if (CBS_peek_asn1_tag(&app_seq, kBufferViewOffsetFromDataPtr)) {
    return deserialize_buffer_view_from_data_offset(app_seq, this, view);
  } else {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_BUFFER);
    return false;
  }

  return true;
}

static int dtls_read_buffer_next_packet(SSL *ssl) {
  SSLBuffer *buf = &ssl->s3->read_buffer;

  if (!buf->empty()) {
    // It is an error to call |dtls_read_buffer_extend| when the read buffer is
    // not empty.
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return -1;
  }

  // Read a single packet from |ssl->rbio|. |buf->cap()| must fit in an int.
  int ret =
      BIO_read(ssl->rbio.get(), buf->data(), static_cast<int>(buf->cap()));
  if (ret <= 0) {
    ssl->s3->rwstate = SSL_ERROR_WANT_READ;
    return ret;
  }
  buf->DidWrite(static_cast<size_t>(ret));
  return 1;
}

static int tls_read_buffer_extend_to(SSL *ssl, size_t len) {
  SSLBuffer *buf = &ssl->s3->read_buffer;

  if (len > buf->cap()) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BUFFER_TOO_SMALL);
    return -1;
  }

  // Read until the target length is reached.
  while (buf->size() < len) {
    // The amount of data to read is bounded by |buf->cap|, which must fit in an
    // int.

    // If enable_read_ahead  we want to attempt to fill the entire buffer, but
    // the while loop will bail once we have at least the requested len amount
    // of data. If not enable_read_ahead, only read as much to get to len bytes,
    // at this point we know len is less than the overall size of the buffer.
    assert(buf->cap() >= buf->size());
    size_t read_amount =
        ssl->enable_read_ahead ? buf->cap() - buf->size() : len - buf->size();
    assert(read_amount <= buf->cap() - buf->size());
    int ret = BIO_read(ssl->rbio.get(), buf->data() + buf->size(),
                       static_cast<int>(read_amount));
    if (ret <= 0) {
      // If the peer closed the transport without a close_notify and the caller
      // enabled SSL_OP_IGNORE_UNEXPECTED_EOF, report a clean shutdown with
      // SSL_ERROR_ZERO_RETURN.
      if (ret == 0  && (ssl->options & SSL_OP_IGNORE_UNEXPECTED_EOF) &&
          ssl->s3->read_shutdown == ssl_shutdown_none &&
          !SSL_in_init(ssl) && BIO_eof(ssl->rbio.get())) {
        ssl->s3->read_shutdown = ssl_shutdown_close_notify;
        ssl->s3->rwstate = SSL_ERROR_ZERO_RETURN;
        return ret;
      }
      
      ssl->s3->rwstate = SSL_ERROR_WANT_READ;
      return ret;
    }
    buf->DidWrite(static_cast<size_t>(ret));
  }

  return 1;
}

int ssl_read_buffer_extend_to(SSL *ssl, size_t len) {
  // |ssl_read_buffer_extend_to| implicitly discards any consumed data.
  ssl->s3->read_buffer.DiscardConsumed();
  size_t buffer_size = len;
  if (SSL_is_dtls(ssl)) {
    static_assert(DTLS1_RT_HEADER_LENGTH + SSL3_RT_MAX_ENCRYPTED_LENGTH <=
                      SSLBUFFER_MAX_CAPACITY,
                  "DTLS read buffer is too large");

    // The |len| parameter is ignored in DTLS.
    len = DTLS1_RT_HEADER_LENGTH + SSL3_RT_MAX_ENCRYPTED_LENGTH;
    buffer_size = len;
  } else {
    if (ssl->ctx.get()->enable_read_ahead) {
      // If we're reading ahead allocate read_ahead_buffer size or the requested
      // len whichever is bigger
      buffer_size = std::max(len, ssl->read_ahead_buffer_size);
    }
  }

  if (!ssl->s3->read_buffer.EnsureCap(ssl_record_prefix_len(ssl),
                                      buffer_size)) {
    return -1;
  }

  if (ssl->rbio == nullptr) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BIO_NOT_SET);
    return -1;
  }

  int ret;
  if (SSL_is_dtls(ssl)) {
    // |len| is ignored for a datagram transport.
    ret = dtls_read_buffer_next_packet(ssl);
  } else {
    ret = tls_read_buffer_extend_to(ssl, len);
  }

  if (ret <= 0) {
    // If the buffer was empty originally and remained empty after attempting to
    // extend it, release the buffer until the next attempt.
    ssl->s3->read_buffer.DiscardConsumed();
  }
  return ret;
}

int ssl_handle_open_record(SSL *ssl, bool *out_retry, ssl_open_record_t ret,
                           size_t consumed, uint8_t alert) {
  *out_retry = false;
  if (ret != ssl_open_record_partial) {
    ssl->s3->read_buffer.Consume(consumed);
  }
  if (ret != ssl_open_record_success) {
    // Nothing was returned to the caller, so discard anything marked consumed.
    ssl->s3->read_buffer.DiscardConsumed();
  }
  switch (ret) {
    case ssl_open_record_success:
      return 1;

    case ssl_open_record_partial: {
      int read_ret = ssl_read_buffer_extend_to(ssl, consumed);
      if (read_ret <= 0) {
        return read_ret;
      }
      *out_retry = true;
      return 1;
    }

    case ssl_open_record_discard:
      *out_retry = true;
      return 1;

    case ssl_open_record_close_notify:
      ssl->s3->rwstate = SSL_ERROR_ZERO_RETURN;
      return 0;

    case ssl_open_record_error:
      if (alert != 0) {
        ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
      }
      return -1;
  }
  assert(0);
  return -1;
}


static_assert(SSL3_RT_HEADER_LENGTH * 2 +
                      SSL3_RT_SEND_MAX_ENCRYPTED_OVERHEAD * 2 +
                      SSL3_RT_MAX_PLAIN_LENGTH <=
                  SSLBUFFER_MAX_CAPACITY,
              "maximum TLS write buffer is too large");

static_assert(DTLS1_RT_HEADER_LENGTH + SSL3_RT_SEND_MAX_ENCRYPTED_OVERHEAD +
                      SSL3_RT_MAX_PLAIN_LENGTH <=
                  SSLBUFFER_MAX_CAPACITY,
              "maximum DTLS write buffer is too large");

static int tls_write_buffer_flush(SSL *ssl) {
  SSLBuffer *buf = &ssl->s3->write_buffer;

  while (!buf->empty()) {
    int ret = BIO_write(ssl->wbio.get(), buf->data(), buf->size());
    if (ret <= 0) {
      ssl->s3->rwstate = SSL_ERROR_WANT_WRITE;
      return ret;
    }
    buf->Consume(static_cast<size_t>(ret));
  }
  buf->Clear();
  return 1;
}

static int dtls_write_buffer_flush(SSL *ssl) {
  SSLBuffer *buf = &ssl->s3->write_buffer;
  if (buf->empty()) {
    return 1;
  }

  int ret = BIO_write(ssl->wbio.get(), buf->data(), buf->size());
  if (ret <= 0) {
    ssl->s3->rwstate = SSL_ERROR_WANT_WRITE;
    // If the write failed, drop the write buffer anyway. Datagram transports
    // can't write half a packet, so the caller is expected to retry from the
    // top.
    buf->Clear();
    return ret;
  }
  buf->Clear();
  return 1;
}

int ssl_write_buffer_flush(SSL *ssl) {
  if (ssl->wbio == nullptr) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BIO_NOT_SET);
    return -1;
  }

  if (SSL_is_dtls(ssl)) {
    return dtls_write_buffer_flush(ssl);
  } else {
    return tls_write_buffer_flush(ssl);
  }
}

BSSL_NAMESPACE_END
