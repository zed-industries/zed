// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/bn.h>

#include <assert.h>
#include <ctype.h>
#include <limits.h>
#include <stdio.h>

#include <openssl/bio.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../fipsmodule/bn/internal.h"


int BN_bn2cbb_padded(CBB *out, size_t len, const BIGNUM *in) {
  uint8_t *ptr;
  return CBB_add_space(out, &ptr, len) && BN_bn2bin_padded(ptr, len, in);
}

static const char hextable[] = "0123456789ABCDEF";

char *BN_bn2hex(const BIGNUM *bn) {
  int width = bn_minimal_width(bn);
  char *buf = OPENSSL_zalloc(1 /* leading '-' */ + 1 /* zero is non-empty */ +
                             width * BN_BYTES * 2 + 1 /* trailing NUL */);
  if (buf == NULL) {
    return NULL;
  }

  char *p = buf;
  if (bn->neg) {
    *(p++) = '-';
  }

  if (BN_is_zero(bn)) {
    *(p++) = '0';
  }

  int z = 0;
  for (int i = width - 1; i >= 0; i--) {
    for (int j = BN_BITS2 - 8; j >= 0; j -= 8) {
      // strip leading zeros
      int v = ((int)(bn->d[i] >> (long)j)) & 0xff;
      if (z || v != 0) {
        *(p++) = hextable[v >> 4];
        *(p++) = hextable[v & 0x0f];
        z = 1;
      }
    }
  }
  assert(*p == '\0');

  return buf;
}

// decode_hex decodes |in_len| bytes of hex data from |in| and updates |bn|.
static int decode_hex(BIGNUM *bn, const char *in, int in_len) {
  if (in_len > INT_MAX/4) {
    OPENSSL_PUT_ERROR(BN, BN_R_BIGNUM_TOO_LONG);
    return 0;
  }
  // |in_len| is the number of hex digits.
  if (!bn_expand(bn, in_len * 4)) {
    return 0;
  }

  int i = 0;
  while (in_len > 0) {
    // Decode one |BN_ULONG| at a time.
    int todo = BN_BYTES * 2;
    if (todo > in_len) {
      todo = in_len;
    }

    BN_ULONG word = 0;
    int j;
    for (j = todo; j > 0; j--) {
      uint8_t hex = 0;
      if (!OPENSSL_fromxdigit(&hex, in[in_len - j])) {
        // This shouldn't happen. The caller checks |OPENSSL_isxdigit|.
        assert(0);
      }
      word = (word << 4) | hex;
    }

    bn->d[i++] = word;
    in_len -= todo;
  }
  assert(i <= bn->dmax);
  bn->width = i;
  return 1;
}

// decode_dec decodes |in_len| bytes of decimal data from |in| and updates |bn|.
static int decode_dec(BIGNUM *bn, const char *in, int in_len) {
  int i, j;
  BN_ULONG l = 0;

  // Decode |BN_DEC_NUM| digits at a time.
  j = BN_DEC_NUM - (in_len % BN_DEC_NUM);
  if (j == BN_DEC_NUM) {
    j = 0;
  }
  l = 0;
  for (i = 0; i < in_len; i++) {
    l *= 10;
    l += in[i] - '0';
    if (++j == BN_DEC_NUM) {
      if (!BN_mul_word(bn, BN_DEC_CONV) ||
          !BN_add_word(bn, l)) {
        return 0;
      }
      l = 0;
      j = 0;
    }
  }
  return 1;
}

typedef int (*decode_func) (BIGNUM *bn, const char *in, int in_len);
typedef int (*char_test_func) (int c);

static int bn_x2bn(BIGNUM **outp, const char *in, decode_func decode, char_test_func want_char) {
  BIGNUM *ret = NULL;
  int neg = 0, i;
  int num;

  if (in == NULL || *in == 0) {
    return 0;
  }

  if (*in == '-') {
    neg = 1;
    in++;
  }

  for (i = 0; want_char((unsigned char)in[i]) && i + neg < INT_MAX; i++) {}

  if(i == 0) {
    OPENSSL_PUT_ERROR(BN, BN_R_INVALID_INPUT);
    return 0;
  }

  num = i + neg;
  if (outp == NULL) {
    return num;
  }

  // in is the start of the hex digits, and it is 'i' long
  if (*outp == NULL) {
    ret = BN_new();
    if (ret == NULL) {
      return 0;
    }
  } else {
    ret = *outp;
    BN_zero(ret);
  }

  if (!decode(ret, in, i)) {
    goto err;
  }

  bn_set_minimal_width(ret);
  if (!BN_is_zero(ret)) {
    ret->neg = neg;
  }

  *outp = ret;
  return num;

err:
  if (*outp == NULL) {
    BN_free(ret);
  }

  return 0;
}

int BN_hex2bn(BIGNUM **outp, const char *in) {
  return bn_x2bn(outp, in, decode_hex, OPENSSL_isxdigit);
}

char *BN_bn2dec(const BIGNUM *a) {
  // It is easier to print strings little-endian, so we assemble it in reverse
  // and fix at the end.
  BIGNUM *copy = NULL;
  CBB cbb;
  if (!CBB_init(&cbb, 16) ||
      !CBB_add_u8(&cbb, 0 /* trailing NUL */)) {
    goto err;
  }

  if (BN_is_zero(a)) {
    if (!CBB_add_u8(&cbb, '0')) {
      goto err;
    }
  } else {
    copy = BN_dup(a);
    if (copy == NULL) {
      goto err;
    }

    while (!BN_is_zero(copy)) {
      BN_ULONG word = BN_div_word(copy, BN_DEC_CONV);
      if (word == (BN_ULONG)-1) {
        goto err;
      }

      const int add_leading_zeros = !BN_is_zero(copy);
      for (int i = 0; i < BN_DEC_NUM && (add_leading_zeros || word != 0); i++) {
        if (!CBB_add_u8(&cbb, '0' + word % 10)) {
          goto err;
        }
        word /= 10;
      }
      assert(word == 0);
    }
  }

  if (BN_is_negative(a) &&
      !CBB_add_u8(&cbb, '-')) {
    goto err;
  }

  uint8_t *data;
  size_t len;
  if (!CBB_finish(&cbb, &data, &len)) {
    goto err;
  }

  // Reverse the buffer.
  for (size_t i = 0; i < len/2; i++) {
    uint8_t tmp = data[i];
    data[i] = data[len - 1 - i];
    data[len - 1 - i] = tmp;
  }

  BN_free(copy);
  return (char *)data;

err:
  BN_free(copy);
  CBB_cleanup(&cbb);
  return NULL;
}

int BN_dec2bn(BIGNUM **outp, const char *in) {
  return bn_x2bn(outp, in, decode_dec, OPENSSL_isdigit);
}

int BN_asc2bn(BIGNUM **outp, const char *in) {
  const char *const orig_in = in;
  if (*in == '-') {
    in++;
  }

  if (in[0] == '0' && (in[1] == 'X' || in[1] == 'x')) {
    if (!BN_hex2bn(outp, in+2)) {
      return 0;
    }
  } else {
    if (!BN_dec2bn(outp, in)) {
      return 0;
    }
  }

  if (*orig_in == '-' && !BN_is_zero(*outp)) {
    (*outp)->neg = 1;
  }

  return 1;
}

int BN_print(BIO *bp, const BIGNUM *a) {
  int i, j, v, z = 0;
  int ret = 0;

  if (a->neg && BIO_write(bp, "-", 1) != 1) {
    goto end;
  }

  if (BN_is_zero(a) && BIO_write(bp, "0", 1) != 1) {
    goto end;
  }

  for (i = bn_minimal_width(a) - 1; i >= 0; i--) {
    for (j = BN_BITS2 - 4; j >= 0; j -= 4) {
      // strip leading zeros
      v = ((int)(a->d[i] >> (long)j)) & 0x0f;
      if (z || v != 0) {
        if (BIO_write(bp, &hextable[v], 1) != 1) {
          goto end;
        }
        z = 1;
      }
    }
  }
  ret = 1;

end:
  return ret;
}

int BN_print_fp(FILE *fp, const BIGNUM *a) {
  BIO *b = BIO_new_fp(fp, BIO_NOCLOSE);
  if (b == NULL) {
    return 0;
  }

  int ret = BN_print(b, a);
  BIO_free(b);
  return ret;
}


size_t BN_bn2mpi(const BIGNUM *in, uint8_t *out) {
  const size_t bits = BN_num_bits(in);
  const size_t bytes = (bits + 7) / 8;
  // If the number of bits is a multiple of 8, i.e. if the MSB is set,
  // prefix with a zero byte.
  int extend = 0;
  if (bytes != 0 && (bits & 0x07) == 0) {
    extend = 1;
  }

  const size_t len = bytes + extend;
  if (len < bytes ||
      4 + len < len ||
      (len & 0xffffffff) != len) {
    // If we cannot represent the number then we emit zero as the interface
    // doesn't allow an error to be signalled.
    if (out) {
      OPENSSL_memset(out, 0, 4);
    }
    return 4;
  }

  if (out == NULL) {
    return 4 + len;
  }

  out[0] = len >> 24;
  out[1] = len >> 16;
  out[2] = len >> 8;
  out[3] = len;
  if (extend) {
    out[4] = 0;
  }
  BN_bn2bin(in, out + 4 + extend);
  if (in->neg && len > 0) {
    out[4] |= 0x80;
  }
  return len + 4;
}

BIGNUM *BN_mpi2bn(const uint8_t *in, size_t len, BIGNUM *out) {
  if (len < 4) {
    OPENSSL_PUT_ERROR(BN, BN_R_BAD_ENCODING);
    return NULL;
  }
  const size_t in_len = ((size_t)in[0] << 24) |
                        ((size_t)in[1] << 16) |
                        ((size_t)in[2] << 8) |
                        ((size_t)in[3]);
  if (in_len != len - 4) {
    OPENSSL_PUT_ERROR(BN, BN_R_BAD_ENCODING);
    return NULL;
  }

  int out_is_alloced = 0;
  if (out == NULL) {
    out = BN_new();
    if (out == NULL) {
      return NULL;
    }
    out_is_alloced = 1;
  }

  if (in_len == 0) {
    BN_zero(out);
    return out;
  }

  in += 4;
  if (BN_bin2bn(in, in_len, out) == NULL) {
    if (out_is_alloced) {
      BN_free(out);
    }
    return NULL;
  }
  out->neg = ((*in) & 0x80) != 0;
  if (out->neg) {
    unsigned num_bits = BN_num_bits(out);
    if (num_bits >= INT_MAX) {
      if (out_is_alloced) {
        BN_free(out);
      }
      return NULL;
    }
    BN_clear_bit(out, (int)num_bits - 1);
  }
  return out;
}

int BN_bn2binpad(const BIGNUM *in, uint8_t *out, int len) {
  if (len < 0 ||
      !BN_bn2bin_padded(out, (size_t)len, in)) {
    return -1;
  }
  return len;
}
