// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/bio.h>

#include <assert.h>
#include <errno.h>
#include <limits.h>
#include <string.h>

#include <openssl/asn1.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/thread.h>

#include "../internal.h"

// |callback_fn_wrap_ex| adapts the legacy callback interface |BIO_callback_fn| to the
// extended callback interface |BIO_callback_fn_ex|. This function should only be
// called when |callback_ex| is not available and the legacy callback is set.
//
// The extended interface parameters |len| and |processed| are mapped to the legacy
// interface parameters |argi| and |bio_ret| respectively.
//
// Returns -1 on NULL |BIO| or callback, otherwise returns the result of the legacy
// callback.
static long callback_fn_wrap_ex(BIO *bio, int oper, const char *argp,
                              size_t len, int argi, long argl, int bio_ret,
                              size_t *processed) {
  assert(bio != NULL);
  assert(bio->callback != NULL);
  assert(bio->callback_ex == NULL);

  /* Strip off any BIO_CB_RETURN flag */
  int bareoper = oper & ~BIO_CB_RETURN;

  if (bareoper == BIO_CB_READ || bareoper == BIO_CB_WRITE
    || bareoper == BIO_CB_GETS) {
    /* In this case |len| is set, and should be used instead of |argi| */
    if (len > INT_MAX) {
      return -1;
    }

    argi = (int)len;
  }

  if (bio_ret > 0 && (oper & BIO_CB_RETURN) && bareoper != BIO_CB_CTRL) {
    if (*processed > INT_MAX) {
      return -1;
    }

    bio_ret = *processed;
  }


  long ret = bio->callback(bio, oper, argp, argi, argl, bio_ret);

  if (ret > 0 && (oper & BIO_CB_RETURN) && bareoper != BIO_CB_CTRL) {
    *processed = (size_t)ret;
    ret = 1;
  }

  return ret;
}

// |get_callback| returns the appropriate callback function for a given |BIO|, preferring
// the extended interface |callback_ex| over the legacy interface.
//
// When only the legacy callback is available, it is wrapped in the extended format
// via |callback_fn_wrap_ex| to provide a consistent interface. The extended callback
// provides additional parameters for length and bytes processed tracking.
//
// Returns the |callback_ex| function if available, a wrapped legacy callback if only
// |callback| is set, or NULL if no callbacks are set.
static BIO_callback_fn_ex get_callback(BIO *bio) {
  assert(bio != NULL);

  if (bio->callback_ex != NULL) {
    return bio->callback_ex;
  }
  if (bio->callback != NULL) {
    // Wrap old-style callback in extended format
    return callback_fn_wrap_ex;
  }
  return NULL;
}

// Helper function to handle return values from |BIO_read|, |BIO_write|,
// |BIO_gets|, and |BIO_puts| operations.
static int handle_callback_return(BIO *bio, int oper, const void *buf,
int len, int ret) {

  size_t processed = 0;

  if (ret > 0) {
    if (oper == BIO_CB_READ || oper == BIO_CB_GETS) {
      bio->num_read += ret;
    } else if (oper == BIO_CB_WRITE || oper == BIO_CB_PUTS) {
      bio->num_write += ret;
    }
    // |callback_ex| receives the number of bytes processed via the |processed| parameter,
    // while the legacy callback receives this information through both |argi| and |ret|.
    // When using the legacy callback, the |processed| value will be mapped back to |ret|.
    processed = ret;
    ret = 1;
  }

  BIO_callback_fn_ex cb = get_callback(bio);
  if (cb != NULL) {
    long callback_ret = cb(bio, oper | BIO_CB_RETURN, buf, len, 0, 0L, ret, &processed);
    if (callback_ret > INT_MAX || callback_ret < INT_MIN) {
      return -1;
    }
    ret = (int)callback_ret;
  }


  if (ret > 0) {
    if (processed > INT_MAX) {
      ret = -1; // Value too large to represent as int
    } else {
      ret = (int)processed;
    }
  }

  return ret;
}

static CRYPTO_EX_DATA_CLASS g_ex_data_class =
    CRYPTO_EX_DATA_CLASS_INIT_WITH_APP_DATA;

BIO *BIO_new(const BIO_METHOD *method) {
  BIO *ret = OPENSSL_zalloc(sizeof(BIO));
  if (ret == NULL) {
    return NULL;
  }

  ret->method = method;
  ret->shutdown = 1;
  ret->references = 1;
  ret->callback_ex = NULL;
  ret->callback = NULL;
  CRYPTO_new_ex_data(&ret->ex_data);

  if (method->create != NULL && !method->create(ret)) {
    OPENSSL_free(ret);
    return NULL;
  }

  return ret;
}

int BIO_free(BIO *bio) {
  BIO *next_bio;

  for (; bio != NULL; bio = next_bio) {
    if (!CRYPTO_refcount_dec_and_test_zero(&bio->references)) {
      return 0;
    }

    next_bio = BIO_pop(bio);

    if (bio->method != NULL && bio->method->destroy != NULL) {
      bio->method->destroy(bio);
    }

    BIO_callback_fn_ex cb = get_callback(bio);
    if (cb != NULL) {
      long ret = cb(bio, BIO_CB_FREE, NULL, 0, 0, 0L, 1L, NULL);
      if (ret <= 0) {
        if (ret >= INT_MIN) {
          return (int)ret;
        }
        return INT_MIN;
      }
    }

    CRYPTO_free_ex_data(&g_ex_data_class, bio, &bio->ex_data);
    OPENSSL_free(bio);
  }
  return 1;
}

int BIO_up_ref(BIO *bio) {
  CRYPTO_refcount_inc(&bio->references);
  return 1;
}

void BIO_vfree(BIO *bio) {
  BIO_free(bio);
}

void BIO_free_all(BIO *bio) {
  BIO_free(bio);
}

int BIO_read(BIO *bio, void *buf, int len) {

  if (bio == NULL || bio->method == NULL || bio->method->bread == NULL) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_UNSUPPORTED_METHOD);
    return -2;
  }
  if (len <= 0) {
    return 0;
  }

  BIO_callback_fn_ex cb = get_callback(bio);
  if (cb != NULL) {
    long callback_ret = cb(bio, BIO_CB_READ, buf, len, 0, 0L, 1L, NULL);
    if (callback_ret <= 0) {
      if (callback_ret >= INT_MIN) {
        return (int)callback_ret;
      }
      return INT_MIN;
    }
  }

  if (!bio->init) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_UNINITIALIZED);
    return -2;
  }
  int ret = bio->method->bread(bio, buf, len);

  return handle_callback_return(bio, BIO_CB_READ, buf, len, ret);
}

int BIO_read_ex(BIO *bio, void *data, size_t data_len, size_t *read_bytes) {
  if (bio == NULL || read_bytes == NULL) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_NULL_PARAMETER);
    return 0;
  }

  int read_len = (int)data_len;
  if (data_len > INT_MAX) {
    read_len = INT_MAX;
  }

  int ret = BIO_read(bio, data, read_len);
  if (ret > 0) {
    *read_bytes = ret;
    return 1;
  } else {
    *read_bytes = 0;
    return 0;
  }
}

int BIO_gets(BIO *bio, char *buf, int len) {
  if (bio == NULL || bio->method == NULL || bio->method->bgets == NULL) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_UNSUPPORTED_METHOD);
    return -2;
  }
  if (len <= 0) {
    return 0;
  }

  BIO_callback_fn_ex cb = get_callback(bio);
  if (cb != NULL) {
    long callback_ret = cb(bio, BIO_CB_GETS, buf, len, 0, 0L, 1L, NULL);
    if (callback_ret <= 0) {
      if (callback_ret >= INT_MIN) {
        return (int)callback_ret;
      }
      return INT_MIN;
    }
  }

  if (!bio->init) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_UNINITIALIZED);
    return -2;
  }
  int ret = bio->method->bgets(bio, buf, len);

  return handle_callback_return(bio, BIO_CB_GETS, buf, len, ret);
}

int BIO_write(BIO *bio, const void *in, int inl) {

  if (bio == NULL || bio->method == NULL || bio->method->bwrite == NULL) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_UNSUPPORTED_METHOD);
    return -2;
  }
  if (inl <= 0) {
    return 0;
  }

  BIO_callback_fn_ex cb = get_callback(bio);
  if (cb != NULL) {
    long callback_ret = cb(bio, BIO_CB_WRITE, in, inl, 0, 0L, 1L, NULL);
    if (callback_ret <= 0) {
      if (callback_ret >= INT_MIN) {
        return (int)callback_ret;
      }
      return INT_MIN;
    }
  }

  if (!bio->init) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_UNINITIALIZED);
    return -2;
  }
  int ret = bio->method->bwrite(bio, in, inl);

  return handle_callback_return(bio, BIO_CB_WRITE, in, inl, ret);
}

int BIO_write_ex(BIO *bio, const void *data, size_t data_len, size_t *written_bytes) {
  if (written_bytes != NULL) {
    *written_bytes = 0;
  }
  if (bio == NULL) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_NULL_PARAMETER);
    return 0;
  }

  int write_len = (int)data_len;
  if (data_len > INT_MAX) {
    write_len = INT_MAX;
  }

  int ret = BIO_write(bio, data, write_len);
  if (ret > 0) {
    if (written_bytes != NULL) {
      *written_bytes = ret;
    }
    return 1;
  } else {
    return 0;
  }
}

int BIO_write_all(BIO *bio, const void *data, size_t len) {
  const uint8_t *data_u8 = data;
  while (len > 0) {
    const int write_len = ((len > INT_MAX) ? INT_MAX : (int)len);
    int ret = BIO_write(bio, data_u8, write_len);
    assert(ret <= write_len);
    if (ret <= 0) {
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_BUFFER_TOO_SMALL);
      return 0;
    }
    data_u8 += ret;
    len -= ret;
  }
  return 1;
}

int BIO_puts(BIO *bio, const char *in) {
  // Check for bwrites here since we use that if bputs is NULL
  if (bio == NULL || bio->method == NULL || (bio->method->bwrite == NULL &&
                                            bio->method->bputs == NULL)) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_UNSUPPORTED_METHOD);
    return -2;
  }

  BIO_callback_fn_ex cb = get_callback(bio);
  if (cb != NULL) {
    long callback_ret = cb(bio, BIO_CB_PUTS, in, 0, 0, 0L, 1L, NULL);
    if (callback_ret <= 0) {
      if (callback_ret >= INT_MIN) {
        return (int)callback_ret;
      }
      return INT_MIN;
    }
  }

  if (!bio->init) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_UNINITIALIZED);
    return -2;
  }
  int ret = 0;

  if (bio->method->bputs != NULL) {
    ret = bio->method->bputs(bio, in);
  } else {
    const size_t len = strlen(in);
    if (len > INT_MAX) {
      // |BIO_write| and the return value both assume the string fits in |int|.
      OPENSSL_PUT_ERROR(BIO, ERR_R_OVERFLOW);
      return -1;
    }
    ret = bio->method->bwrite(bio, in, len);
  }

  return handle_callback_return(bio, BIO_CB_PUTS, in, 0, ret);
}

int BIO_flush(BIO *bio) {
  return (int)BIO_ctrl(bio, BIO_CTRL_FLUSH, 0, NULL);
}

long BIO_ctrl(BIO *bio, int cmd, long larg, void *parg) {
  if (bio == NULL) {
    return 0;
  }

  if (bio->method == NULL || bio->method->ctrl == NULL) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_UNSUPPORTED_METHOD);
    return -2;
  }
  long ret = 0;

  BIO_callback_fn_ex cb = get_callback(bio);
  if (cb != NULL) {
    ret = cb(bio, BIO_CB_CTRL, parg, 0, cmd, larg, 1L, NULL);
    if (ret <= 0) {
      return ret;
    }
  }


  ret = bio->method->ctrl(bio, cmd, larg, parg);

  cb = get_callback(bio);
  if (cb != NULL) {
    ret = cb(bio, BIO_CB_CTRL | BIO_CB_RETURN, parg, 0, cmd, larg,
                         ret, NULL);
  }

  return ret;
}

char *BIO_ptr_ctrl(BIO *b, int cmd, long larg) {
  char *p = NULL;

  if (BIO_ctrl(b, cmd, larg, (void *)&p) <= 0) {
    return NULL;
  }

  return p;
}

long BIO_int_ctrl(BIO *b, int cmd, long larg, int iarg) {
  int i = iarg;

  return BIO_ctrl(b, cmd, larg, (void *)&i);
}

int BIO_reset(BIO *bio) {
  return (int)BIO_ctrl(bio, BIO_CTRL_RESET, 0, NULL);
}

int BIO_eof(BIO *bio) {
  return (int)BIO_ctrl(bio, BIO_CTRL_EOF, 0, NULL);
}

void BIO_set_flags(BIO *bio, int flags) {
  bio->flags |= flags;
}

int BIO_test_flags(const BIO *bio, int flags) {
  return bio->flags & flags;
}

int BIO_should_read(const BIO *bio) {
  return BIO_test_flags(bio, BIO_FLAGS_READ);
}

int BIO_should_write(const BIO *bio) {
  return BIO_test_flags(bio, BIO_FLAGS_WRITE);
}

int BIO_should_retry(const BIO *bio) {
  return BIO_test_flags(bio, BIO_FLAGS_SHOULD_RETRY);
}

int BIO_should_io_special(const BIO *bio) {
  return BIO_test_flags(bio, BIO_FLAGS_IO_SPECIAL);
}

int BIO_get_retry_reason(const BIO *bio) { return bio->retry_reason; }

void BIO_set_retry_reason(BIO *bio, int reason) { bio->retry_reason = reason; }

void BIO_clear_flags(BIO *bio, int flags) {
  bio->flags &= ~flags;
}

void BIO_set_retry_read(BIO *bio) {
  bio->flags |= BIO_FLAGS_READ | BIO_FLAGS_SHOULD_RETRY;
}

void BIO_set_retry_write(BIO *bio) {
  bio->flags |= BIO_FLAGS_WRITE | BIO_FLAGS_SHOULD_RETRY;
}

static const int kRetryFlags = BIO_FLAGS_RWS | BIO_FLAGS_SHOULD_RETRY;

int BIO_get_retry_flags(BIO *bio) {
  return bio->flags & kRetryFlags;
}

void BIO_clear_retry_flags(BIO *bio) {
  bio->flags &= ~kRetryFlags;
  bio->retry_reason = 0;
}

int BIO_method_type(const BIO *bio) { return bio->method->type; }

const char *BIO_method_name(const BIO *bio) { return bio->method->name; }

void BIO_copy_next_retry(BIO *bio) {
  BIO_clear_retry_flags(bio);
  BIO_set_flags(bio, BIO_get_retry_flags(bio->next_bio));
  bio->retry_reason = bio->next_bio->retry_reason;
}

long BIO_callback_ctrl(BIO *bio, int cmd, bio_info_cb fp) {
  if (bio == NULL) {
    return 0;
  }

  if (bio->method == NULL || bio->method->callback_ctrl == NULL) {
    OPENSSL_PUT_ERROR(BIO, BIO_R_UNSUPPORTED_METHOD);
    return 0;
  }

  return bio->method->callback_ctrl(bio, cmd, fp);
}

size_t BIO_pending(const BIO *bio) {
  const long r = BIO_ctrl((BIO *) bio, BIO_CTRL_PENDING, 0, NULL);
  assert(r >= 0);

  if (r < 0) {
    return 0;
  }
  return r;
}

size_t BIO_ctrl_pending(const BIO *bio) {
  return BIO_pending(bio);
}

size_t BIO_wpending(const BIO *bio) {
  const long r = BIO_ctrl((BIO *) bio, BIO_CTRL_WPENDING, 0, NULL);
  assert(r >= 0);

  if (r < 0) {
    return 0;
  }
  return r;
}

int BIO_set_close(BIO *bio, int close_flag) {
  return (int)BIO_ctrl(bio, BIO_CTRL_SET_CLOSE, close_flag, NULL);
}

int BIO_get_close(BIO *bio) {
  return (int)BIO_ctrl(bio, BIO_CTRL_GET_CLOSE, 0, NULL);
}

OPENSSL_EXPORT uint64_t BIO_number_read(const BIO *bio) {
  return bio->num_read;
}

OPENSSL_EXPORT uint64_t BIO_number_written(const BIO *bio) {
  return bio->num_write;
}

BIO *BIO_push(BIO *bio, BIO *appended_bio) {
  BIO *last_bio;

  if (bio == NULL) {
    return bio;
  }

  last_bio = bio;
  while (last_bio->next_bio != NULL) {
    last_bio = last_bio->next_bio;
  }

  last_bio->next_bio = appended_bio;
  return bio;
}

BIO *BIO_pop(BIO *bio) {
  BIO *ret;

  if (bio == NULL) {
    return NULL;
  }
  ret = bio->next_bio;
  bio->next_bio = NULL;
  return ret;
}

BIO *BIO_next(BIO *bio) {
  if (!bio) {
    return NULL;
  }
  return bio->next_bio;
}

BIO *BIO_find_type(BIO *bio, int type) {
  int method_type, mask;

  if (!bio) {
    return NULL;
  }
  mask = type & 0xff;

  do {
    if (bio->method != NULL) {
      method_type = bio->method->type;

      if (!mask) {
        if (method_type & type) {
          return bio;
        }
      } else if (method_type == type) {
        return bio;
      }
    }
    bio = bio->next_bio;
  } while (bio != NULL);

  return NULL;
}

int BIO_indent(BIO *bio, unsigned indent, unsigned max_indent) {
  if (indent > max_indent) {
    indent = max_indent;
  }

  while (indent--) {
    if (BIO_puts(bio, " ") != 1) {
      return 0;
    }
  }
  return 1;
}

static int print_bio(const char *str, size_t len, void *bio) {
  return BIO_write_all((BIO *)bio, str, len);
}

void ERR_print_errors(BIO *bio) {
  ERR_print_errors_cb(print_bio, bio);
}

// bio_read_all reads everything from |bio| and prepends |prefix| to it. On
// success, |*out| is set to an allocated buffer (which should be freed with
// |OPENSSL_free|), |*out_len| is set to its length and one is returned. The
// buffer will contain |prefix| followed by the contents of |bio|. On failure,
// zero is returned.
//
// The function will fail if the size of the output would equal or exceed
// |max_len|.
static int bio_read_all(BIO *bio, uint8_t **out, size_t *out_len,
                        const uint8_t *prefix, size_t prefix_len,
                        size_t max_len) {
  static const size_t kChunkSize = 4096;

  size_t len = prefix_len + kChunkSize;
  if (len > max_len) {
    len = max_len;
  }
  if (len < prefix_len) {
    return 0;
  }
  *out = OPENSSL_malloc(len);
  if (*out == NULL) {
    return 0;
  }
  OPENSSL_memcpy(*out, prefix, prefix_len);
  size_t done = prefix_len;

  for (;;) {
    if (done == len) {
      OPENSSL_free(*out);
      return 0;
    }
    size_t todo = len - done;
    if (todo > INT_MAX) {
      todo = INT_MAX;
    }
    const int n = BIO_read(bio, *out + done, (int)todo);
    if (n == 0) {
      *out_len = done;
      return 1;
    } else if (n == -1) {
      OPENSSL_free(*out);
      return 0;
    }

    done += n;
    if (len < max_len && len - done < kChunkSize / 2) {
      len += kChunkSize;
      if (len < kChunkSize || len > max_len) {
        len = max_len;
      }
      uint8_t *new_buf = OPENSSL_realloc(*out, len);
      if (new_buf == NULL) {
        OPENSSL_free(*out);
        return 0;
      }
      *out = new_buf;
    }
  }
}

// bio_read_full reads |len| bytes |bio| and writes them into |out|. It
// tolerates partial reads from |bio| and returns one on success or zero if a
// read fails before |len| bytes are read. On failure, it additionally sets
// |*out_eof_on_first_read| to whether the error was due to |bio| returning zero
// on the first read. |out_eof_on_first_read| may be NULL to discard the value.
static int bio_read_full(BIO *bio, uint8_t *out, int *out_eof_on_first_read,
                         size_t len) {
  int first_read = 1;
  while (len > 0) {
    int todo = len <= INT_MAX ? (int)len : INT_MAX;
    int ret = BIO_read(bio, out, todo);
    if (ret <= 0) {
      if (out_eof_on_first_read != NULL) {
        *out_eof_on_first_read = first_read && ret == 0;
      }
      return 0;
    }
    out += ret;
    len -= (size_t)ret;
    first_read = 0;
  }

  return 1;
}

// For compatibility with existing |d2i_*_bio| callers, |BIO_read_asn1| uses
// |ERR_LIB_ASN1| errors.
OPENSSL_DECLARE_ERROR_REASON(ASN1, ASN1_R_DECODE_ERROR)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ASN1_R_HEADER_TOO_LONG)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ASN1_R_NOT_ENOUGH_DATA)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ASN1_R_TOO_LONG)

int BIO_read_asn1(BIO *bio, uint8_t **out, size_t *out_len, size_t max_len) {
  uint8_t header[6];

  static const size_t kInitialHeaderLen = 2;
  int eof_on_first_read;
  if (!bio_read_full(bio, header, &eof_on_first_read, kInitialHeaderLen)) {
    if (eof_on_first_read) {
      // Historically, OpenSSL returned |ASN1_R_HEADER_TOO_LONG| when
      // |d2i_*_bio| could not read anything. CPython conditions on this to
      // determine if |bio| was empty.
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_HEADER_TOO_LONG);
    } else {
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_NOT_ENOUGH_DATA);
    }
    return 0;
  }

  const uint8_t tag = header[0];
  const uint8_t length_byte = header[1];

  if ((tag & 0x1f) == 0x1f) {
    // Long form tags are not supported.
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_DECODE_ERROR);
    return 0;
  }

  size_t len, header_len;
  if ((length_byte & 0x80) == 0) {
    // Short form length.
    len = length_byte;
    header_len = kInitialHeaderLen;
  } else {
    const size_t num_bytes = length_byte & 0x7f;

    if ((tag & 0x20 /* constructed */) != 0 && num_bytes == 0) {
      // indefinite length.
      if (!bio_read_all(bio, out, out_len, header, kInitialHeaderLen,
                        max_len)) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_NOT_ENOUGH_DATA);
        return 0;
      }
      return 1;
    }

    if (num_bytes == 0 || num_bytes > 4) {
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_DECODE_ERROR);
      return 0;
    }

    if (!bio_read_full(bio, header + kInitialHeaderLen, NULL, num_bytes)) {
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_NOT_ENOUGH_DATA);
      return 0;
    }
    header_len = kInitialHeaderLen + num_bytes;

    uint32_t len32 = 0;
    for (unsigned i = 0; i < num_bytes; i++) {
      len32 <<= 8;
      len32 |= header[kInitialHeaderLen + i];
    }

    if (len32 < 128) {
      // Length should have used short-form encoding.
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_DECODE_ERROR);
      return 0;
    }

    if ((len32 >> ((num_bytes-1)*8)) == 0) {
      // Length should have been at least one byte shorter.
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_DECODE_ERROR);
      return 0;
    }

    len = len32;
  }

  if (len + header_len < len ||
      len + header_len > max_len ||
      len > INT_MAX) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_TOO_LONG);
    return 0;
  }
  len += header_len;
  *out_len = len;

  *out = OPENSSL_malloc(len);
  if (*out == NULL) {
    return 0;
  }
  OPENSSL_memcpy(*out, header, header_len);
  if (!bio_read_full(bio, (*out) + header_len, NULL, len - header_len)) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_NOT_ENOUGH_DATA);
    OPENSSL_free(*out);
    return 0;
  }

  return 1;
}

void BIO_set_retry_special(BIO *bio) {
  bio->flags |= BIO_FLAGS_READ | BIO_FLAGS_IO_SPECIAL;
}

int BIO_set_write_buffer_size(BIO *bio, int buffer_size) { return 0; }

static struct CRYPTO_STATIC_MUTEX g_index_lock = CRYPTO_STATIC_MUTEX_INIT;
static int g_index = BIO_TYPE_START;

int BIO_get_new_index(void) {
  CRYPTO_STATIC_MUTEX_lock_write(&g_index_lock);
  // If |g_index| exceeds 255, it will collide with the flags bits.
  int ret = g_index > 255 ? -1 : g_index++;
  CRYPTO_STATIC_MUTEX_unlock_write(&g_index_lock);
  return ret;
}

BIO_METHOD *BIO_meth_new(int type, const char *name) {
  BIO_METHOD *method = OPENSSL_zalloc(sizeof(BIO_METHOD));
  if (method == NULL) {
    return NULL;
  }
  method->type = type;
  method->name = name;
  return method;
}

void BIO_meth_free(BIO_METHOD *method) {
  OPENSSL_free(method);
}

int BIO_meth_set_create(BIO_METHOD *method,
                        int (*create)(BIO *)) {
  method->create = create;
  return 1;
}

int (*BIO_meth_get_create(const BIO_METHOD *method)) (BIO *) {
  return method->create;
}

int BIO_meth_set_destroy(BIO_METHOD *method,
                         int (*destroy)(BIO *)) {
  method->destroy = destroy;
  return 1;
}

int (*BIO_meth_get_destroy(const BIO_METHOD *method)) (BIO *) {
  return method->destroy;
}

int BIO_meth_set_write(BIO_METHOD *method,
                       int (*write)(BIO *, const char *, int)) {
  method->bwrite = write;
  return 1;
}

int BIO_meth_set_read(BIO_METHOD *method,
                      int (*read)(BIO *, char *, int)) {
  method->bread = read;
  return 1;
}

int BIO_meth_set_gets(BIO_METHOD *method,
                      int (*gets)(BIO *, char *, int)) {
  method->bgets = gets;
  return 1;
}

int (*BIO_meth_get_gets(const BIO_METHOD *method)) (BIO *, char *, int) {
  return method->bgets;
}

int BIO_meth_set_ctrl(BIO_METHOD *method,
                      long (*ctrl)(BIO *, int, long, void *)) {
  method->ctrl = ctrl;
  return 1;
}

long (*BIO_meth_get_ctrl(const BIO_METHOD *method)) (BIO *, int, long, void *) {
  return method->ctrl;
}

int BIO_meth_set_callback_ctrl(BIO_METHOD *method,
                               long (*callback_ctrl)(BIO *, int, bio_info_cb)) {
  method->callback_ctrl = callback_ctrl;
  return 1;
}

long (*BIO_meth_get_callback_ctrl(const BIO_METHOD *method)) (BIO *, int, bio_info_cb) {
  return method->callback_ctrl;
}

void BIO_set_data(BIO *bio, void *ptr) { bio->ptr = ptr; }

void *BIO_get_data(BIO *bio) { return bio->ptr; }

void BIO_set_init(BIO *bio, int init) { bio->init = init; }

int BIO_get_init(BIO *bio) { return bio->init; }

void BIO_set_shutdown(BIO *bio, int shutdown) { bio->shutdown = shutdown; }

int BIO_get_shutdown(BIO *bio) { return bio->shutdown; }

int BIO_meth_set_puts(BIO_METHOD *method, int (*puts)(BIO *, const char *)) {
  method->bputs = puts;
  return 1;
}

int (*BIO_meth_get_puts(const BIO_METHOD *method)) (BIO *, const char *) {
  return method->bputs;
}

void BIO_set_callback_ex(BIO *bio, BIO_callback_fn_ex callback) {
  bio->callback_ex = callback;
}

void BIO_set_callback(BIO *bio, BIO_callback_fn callback) {
  bio->callback = callback;
}

void BIO_set_callback_arg(BIO *bio, char *arg) {
  bio->cb_arg = arg;
}

char *BIO_get_callback_arg(const BIO *bio) {
  return bio->cb_arg;
}

int BIO_get_ex_new_index(long argl, void *argp,
                                    CRYPTO_EX_unused *unused,
                                    CRYPTO_EX_dup *dup_unused,
                                    CRYPTO_EX_free *free_func) {
  int index;
  if (!CRYPTO_get_ex_new_index(&g_ex_data_class, &index, argl, argp,
                               free_func)) {
    return -1;
  }
  return index;
}

int BIO_set_ex_data(BIO *bio, int idx, void *data) {
  return CRYPTO_set_ex_data(&bio->ex_data, idx, data);
}

void *BIO_get_ex_data(const BIO *bio, int idx) {
  return CRYPTO_get_ex_data(&bio->ex_data, idx);
}
