// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <string.h>

#include <openssl/base64.h>
#include <openssl/buf.h>
#include <openssl/des.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/pem.h>
#include <openssl/rand.h>
#include <openssl/x509.h>

#include "internal.h"
#include "../internal.h"
#include "../console/internal.h"
#include "../fipsmodule/evp/internal.h"


static int load_iv(char **fromp, unsigned char *to, size_t num);
static int check_pem(const char *nm, const char *name);

void PEM_proc_type(char buf[PEM_BUFSIZE], int type) {
  const char *str;

  if (type == PEM_TYPE_ENCRYPTED) {
    str = "ENCRYPTED";
  } else if (type == PEM_TYPE_MIC_CLEAR) {
    str = "MIC-CLEAR";
  } else if (type == PEM_TYPE_MIC_ONLY) {
    str = "MIC-ONLY";
  } else {
    str = "BAD-TYPE";
  }

  OPENSSL_strlcat(buf, "Proc-Type: 4,", PEM_BUFSIZE);
  OPENSSL_strlcat(buf, str, PEM_BUFSIZE);
  OPENSSL_strlcat(buf, "\n", PEM_BUFSIZE);
}

void PEM_dek_info(char buf[PEM_BUFSIZE], const char *type, size_t len,
                         char *str) {
  static const unsigned char map[17] = "0123456789ABCDEF";

  OPENSSL_strlcat(buf, "DEK-Info: ", PEM_BUFSIZE);
  OPENSSL_strlcat(buf, type, PEM_BUFSIZE);
  OPENSSL_strlcat(buf, ",", PEM_BUFSIZE);
  size_t buf_len = strlen(buf);
  // We must write an additional |2 * len + 2| bytes after |buf_len|, including
  // the trailing newline and NUL.
  if (len > (PEM_BUFSIZE - buf_len - 2) / 2) {
    return;
  }
  for (size_t i = 0; i < len; i++) {
    buf[buf_len + i * 2] = map[(str[i] >> 4) & 0x0f];
    buf[buf_len + i * 2 + 1] = map[(str[i]) & 0x0f];
  }
  buf[buf_len + len * 2] = '\n';
  buf[buf_len + len * 2 + 1] = '\0';
}

void *PEM_ASN1_read(d2i_of_void *d2i, const char *name, FILE *fp, void **x,
                    pem_password_cb *cb, void *u) {
  BIO *b = BIO_new_fp(fp, BIO_NOCLOSE);
  if (b == NULL) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_BUF_LIB);
    return NULL;
  }
  void *ret = PEM_ASN1_read_bio(d2i, name, b, x, cb, u);
  BIO_free(b);
  return ret;
}

static int check_pem(const char *nm, const char *name) {
  // Normal matching nm and name
  if (!strcmp(nm, name)) {
    return 1;
  }

  // Make PEM_STRING_EVP_PKEY match any private key

  if (!strcmp(name, PEM_STRING_EVP_PKEY)) {
    return !strcmp(nm, PEM_STRING_PKCS8) || !strcmp(nm, PEM_STRING_PKCS8INF) ||
           !strcmp(nm, PEM_STRING_RSA) || !strcmp(nm, PEM_STRING_EC) ||
           !strcmp(nm, PEM_STRING_DSA);
  }

  // These correspond with the PEM strings that have "PARAMETERS".
  if (!strcmp(name, PEM_STRING_PARAMETERS)) {
    return !strcmp(nm, PEM_STRING_ECPARAMETERS) ||
           !strcmp(nm, PEM_STRING_DSAPARAMS) ||
           !strcmp(nm, PEM_STRING_DHPARAMS);
  }

  // Permit older strings

  if (!strcmp(nm, PEM_STRING_X509_OLD) && !strcmp(name, PEM_STRING_X509)) {
    return 1;
  }

  if (!strcmp(nm, PEM_STRING_X509_REQ_OLD) &&
      !strcmp(name, PEM_STRING_X509_REQ)) {
    return 1;
  }

  // Allow normal certs to be read as trusted certs
  if (!strcmp(nm, PEM_STRING_X509) && !strcmp(name, PEM_STRING_X509_TRUSTED)) {
    return 1;
  }

  if (!strcmp(nm, PEM_STRING_X509_OLD) &&
      !strcmp(name, PEM_STRING_X509_TRUSTED)) {
    return 1;
  }

  // Some CAs use PKCS#7 with CERTIFICATE headers
  if (!strcmp(nm, PEM_STRING_X509) && !strcmp(name, PEM_STRING_PKCS7)) {
    return 1;
  }

  if (!strcmp(nm, PEM_STRING_PKCS7_SIGNED) && !strcmp(name, PEM_STRING_PKCS7)) {
    return 1;
  }

#ifndef OPENSSL_NO_CMS
  if (!strcmp(nm, PEM_STRING_X509) && !strcmp(name, PEM_STRING_CMS)) {
    return 1;
  }
  // Allow CMS to be read from PKCS#7 headers
  if (!strcmp(nm, PEM_STRING_PKCS7) && !strcmp(name, PEM_STRING_CMS)) {
    return 1;
  }
#endif

  return 0;
}

static const EVP_CIPHER *cipher_by_name(const char *name) {
  // This is similar to the (deprecated) function |EVP_get_cipherbyname|. Note
  // the PEM code assumes that ciphers have at least 8 bytes of IV, at most 20
  // bytes of overhead and generally behave like CBC mode.
  if (0 == strcmp(name, SN_des_cbc)) {
    return EVP_des_cbc();
  } else if (0 == strcmp(name, SN_des_ede3_cbc)) {
    return EVP_des_ede3_cbc();
  } else if (0 == strcmp(name, SN_aes_128_cbc)) {
    return EVP_aes_128_cbc();
  } else if (0 == strcmp(name, SN_aes_192_cbc)) {
    return EVP_aes_192_cbc();
  } else if (0 == strcmp(name, SN_aes_256_cbc)) {
    return EVP_aes_256_cbc();
  } else {
    return NULL;
  }
}

int PEM_bytes_read_bio(unsigned char **pdata, long *plen, char **pnm,
                       const char *name, BIO *bp, pem_password_cb *cb,
                       void *u) {
  EVP_CIPHER_INFO cipher;
  char *nm = NULL, *header = NULL;
  unsigned char *data = NULL;
  long len;
  int ret = 0;

  for (;;) {
    if (!PEM_read_bio(bp, &nm, &header, &data, &len)) {
      uint32_t error = ERR_peek_error();
      if (ERR_GET_LIB(error) == ERR_LIB_PEM &&
          ERR_GET_REASON(error) == PEM_R_NO_START_LINE) {
        ERR_add_error_data(2, "Expecting: ", name);
      }
      return 0;
    }
    if (check_pem(nm, name)) {
      break;
    }
    OPENSSL_free(nm);
    OPENSSL_free(header);
    OPENSSL_free(data);
  }
  if (!PEM_get_EVP_CIPHER_INFO(header, &cipher)) {
    goto err;
  }
  if (!PEM_do_header(&cipher, data, &len, cb, u)) {
    goto err;
  }

  *pdata = data;
  *plen = len;

  if (pnm) {
    *pnm = nm;
  }

  ret = 1;

err:
  if (!ret || !pnm) {
    OPENSSL_free(nm);
  }
  OPENSSL_free(header);
  if (!ret) {
    OPENSSL_free(data);
  }
  return ret;
}

int PEM_ASN1_write(i2d_of_void *i2d, const char *name, FILE *fp, void *x,
                   const EVP_CIPHER *enc, const unsigned char *pass,
                   int pass_len, pem_password_cb *callback, void *u) {
  BIO *b = BIO_new_fp(fp, BIO_NOCLOSE);
  if (b == NULL) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_BUF_LIB);
    return 0;
  }
  int ret =
      PEM_ASN1_write_bio(i2d, name, b, x, enc, pass, pass_len, callback, u);
  BIO_free(b);
  return ret;
}

int PEM_ASN1_write_bio(i2d_of_void *i2d, const char *name, BIO *bp, void *x,
                       const EVP_CIPHER *enc, const unsigned char *pass,
                       int pass_len, pem_password_cb *callback, void *u) {
  EVP_CIPHER_CTX ctx;
  int i, j, ret = 0;
  unsigned char *p, *data = NULL;
  const char *objstr = NULL;
  char buf[PEM_BUFSIZE];
  unsigned char key[EVP_MAX_KEY_LENGTH];
  unsigned char iv[EVP_MAX_IV_LENGTH];

  if (enc != NULL) {
    objstr = OBJ_nid2sn(EVP_CIPHER_nid(enc));
    if (objstr == NULL || cipher_by_name(objstr) == NULL ||
        EVP_CIPHER_iv_length(enc) < 8) {
      OPENSSL_PUT_ERROR(PEM, PEM_R_UNSUPPORTED_CIPHER);
      goto err;
    }
  }

  int dsize = i2d(x, NULL);
  if (dsize < 0) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_ASN1_LIB);
    OPENSSL_cleanse(&dsize, sizeof(dsize));
    goto err;
  }
  // dzise + 8 bytes are needed
  // actually it needs the cipher block size extra...
  data = (unsigned char *)OPENSSL_malloc((unsigned int)dsize + 20);
  if (data == NULL) {
    goto err;
  }
  p = data;
  i = i2d(x, &p);

  if (enc != NULL) {
    const unsigned iv_len = EVP_CIPHER_iv_length(enc);

    if (pass == NULL) {
      if (!callback) {
        callback = PEM_def_callback;
      }
      pass_len = (*callback)(buf, PEM_BUFSIZE, 1, u);
      pass = (const unsigned char *)buf;
    }
    if (pass_len < 0) {
      OPENSSL_PUT_ERROR(PEM, PEM_R_READ_KEY);
      goto err;
    }
    assert(iv_len <= sizeof(iv));
    AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(iv, iv_len));  // Generate a salt

    // The 'iv' is used as the iv and as a salt.  It is NOT taken from
    // the BytesToKey function
    if (!EVP_BytesToKey(enc, EVP_md5(), iv, pass, pass_len, 1, key, NULL)) {
      goto err;
    }

    if (pass == (const unsigned char *)buf) {
      OPENSSL_cleanse(buf, PEM_BUFSIZE);
    }

    assert(strlen(objstr) + 23 + 2 * iv_len + 13 <= sizeof(buf));

    buf[0] = '\0';
    PEM_proc_type(buf, PEM_TYPE_ENCRYPTED);
    PEM_dek_info(buf, objstr, iv_len, (char *)iv);
    // k=strlen(buf);

    EVP_CIPHER_CTX_init(&ctx);
    ret = 1;
    if (!EVP_EncryptInit_ex(&ctx, enc, NULL, key, iv) ||
        !EVP_EncryptUpdate(&ctx, data, &j, data, i) ||
        !EVP_EncryptFinal_ex(&ctx, &(data[j]), &i)) {
      ret = 0;
    } else {
      i += j;
    }
    EVP_CIPHER_CTX_cleanup(&ctx);
    if (ret == 0) {
      goto err;
    }
  } else {
    ret = 1;
    buf[0] = '\0';
  }
  i = PEM_write_bio(bp, name, buf, data, i);
  if (i <= 0) {
    ret = 0;
  }
err:
  OPENSSL_cleanse(key, sizeof(key));
  OPENSSL_cleanse(iv, sizeof(iv));
  OPENSSL_cleanse((char *)&ctx, sizeof(ctx));
  OPENSSL_cleanse(buf, PEM_BUFSIZE);
  OPENSSL_free(data);
  return ret;
}

int PEM_do_header(EVP_CIPHER_INFO *cipher, unsigned char *data, long *plen,
                  pem_password_cb *callback, void *u) {
  int i = 0, j, o, pass_len;
  long len;
  EVP_CIPHER_CTX ctx;
  unsigned char key[EVP_MAX_KEY_LENGTH];
  char buf[PEM_BUFSIZE];

  len = *plen;

  if (cipher->cipher == NULL) {
    return 1;
  }

  if (!callback) {
    callback = PEM_def_callback;
  }
  pass_len = callback(buf, PEM_BUFSIZE, 0, u);
  if (pass_len < 0) {
    OPENSSL_PUT_ERROR(PEM, PEM_R_BAD_PASSWORD_READ);
    return 0;
  }

  if (!EVP_BytesToKey(cipher->cipher, EVP_md5(), &(cipher->iv[0]),
                      (unsigned char *)buf, pass_len, 1, key, NULL)) {
    return 0;
  }

  j = (int)len;
  EVP_CIPHER_CTX_init(&ctx);
  o = EVP_DecryptInit_ex(&ctx, cipher->cipher, NULL, key, &(cipher->iv[0]));
  if (o) {
    o = EVP_DecryptUpdate(&ctx, data, &i, data, j);
  }
  if (o) {
    o = EVP_DecryptFinal_ex(&ctx, &(data[i]), &j);
  }
  EVP_CIPHER_CTX_cleanup(&ctx);
  OPENSSL_cleanse((char *)buf, sizeof(buf));
  OPENSSL_cleanse((char *)key, sizeof(key));
  if (!o) {
    OPENSSL_PUT_ERROR(PEM, PEM_R_BAD_DECRYPT);
    return 0;
  }
  j += i;
  *plen = j;
  return 1;
}

int PEM_get_EVP_CIPHER_INFO(char *header, EVP_CIPHER_INFO *cipher) {
  const EVP_CIPHER *enc = NULL;
  char *p, c;
  char **header_pp = &header;

  cipher->cipher = NULL;
  OPENSSL_memset(cipher->iv, 0, sizeof(cipher->iv));
  if ((header == NULL) || (*header == '\0') || (*header == '\n')) {
    return 1;
  }
  if (strncmp(header, "Proc-Type: ", 11) != 0) {
    OPENSSL_PUT_ERROR(PEM, PEM_R_NOT_PROC_TYPE);
    return 0;
  }
  header += 11;
  if (*header != '4') {
    return 0;
  }
  header++;
  if (*header != ',') {
    return 0;
  }
  header++;
  if (strncmp(header, "ENCRYPTED", 9) != 0) {
    OPENSSL_PUT_ERROR(PEM, PEM_R_NOT_ENCRYPTED);
    return 0;
  }
  for (; (*header != '\n') && (*header != '\0'); header++) {
    ;
  }
  if (*header == '\0') {
    OPENSSL_PUT_ERROR(PEM, PEM_R_SHORT_HEADER);
    return 0;
  }
  header++;
  if (strncmp(header, "DEK-Info: ", 10) != 0) {
    OPENSSL_PUT_ERROR(PEM, PEM_R_NOT_DEK_INFO);
    return 0;
  }
  header += 10;

  p = header;
  for (;;) {
    c = *header;
    if (!((c >= 'A' && c <= 'Z') || c == '-' ||
          OPENSSL_isdigit(c))) {
      break;
    }
    header++;
  }
  *header = '\0';
  cipher->cipher = enc = cipher_by_name(p);
  *header = c;
  header++;

  if (enc == NULL) {
    OPENSSL_PUT_ERROR(PEM, PEM_R_UNSUPPORTED_ENCRYPTION);
    return 0;
  }
  // The IV parameter must be at least 8 bytes long to be used as the salt in
  // the KDF. (This should not happen given |cipher_by_name|.)
  if (EVP_CIPHER_iv_length(enc) < 8) {
    assert(0);
    OPENSSL_PUT_ERROR(PEM, PEM_R_UNSUPPORTED_ENCRYPTION);
    return 0;
  }
  if (!load_iv(header_pp, &(cipher->iv[0]), EVP_CIPHER_iv_length(enc))) {
    return 0;
  }

  return 1;
}

static int load_iv(char **fromp, unsigned char *to, size_t num) {
  uint8_t v;
  char *from;

  from = *fromp;
  for (size_t i = 0; i < num; i++) {
    to[i] = 0;
  }
  num *= 2;
  for (size_t i = 0; i < num; i++) {
    if (!OPENSSL_fromxdigit(&v, *from)) {
      OPENSSL_PUT_ERROR(PEM, PEM_R_BAD_IV_CHARS);
      return 0;
    }
    from++;
    to[i / 2] |= v << (!(i & 1)) * 4;
  }

  *fromp = from;
  return 1;
}

int PEM_write(FILE *fp, const char *name, const char *header,
              const unsigned char *data, long len) {
  BIO *b = BIO_new_fp(fp, BIO_NOCLOSE);
  if (b == NULL) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_BUF_LIB);
    return 0;
  }
  int ret = PEM_write_bio(b, name, header, data, len);
  BIO_free(b);
  return ret;
}

int PEM_write_bio(BIO *bp, const char *name, const char *header,
                  const unsigned char *data, long len) {
  int nlen, n, outl;
  long i, j;
  unsigned char *buf = NULL;
  EVP_ENCODE_CTX ctx;
  int reason = ERR_R_BUF_LIB;

  EVP_EncodeInit(&ctx);
  nlen = strlen(name);

  if ((BIO_write(bp, "-----BEGIN ", 11) != 11) ||
      (BIO_write(bp, name, nlen) != nlen) ||
      (BIO_write(bp, "-----\n", 6) != 6)) {
    goto err;
  }

  i = (header != NULL) ? strlen(header) : 0;
  if (i > 0) {
    if ((BIO_write(bp, header, i) != i) || (BIO_write(bp, "\n", 1) != 1)) {
      goto err;
    }
  }

  buf = OPENSSL_malloc(PEM_BUFSIZE * 8);
  if (buf == NULL) {
    goto err;
  }

  i = j = 0;
  while (len > 0) {
    n = (int)((len > (PEM_BUFSIZE * 5)) ? (PEM_BUFSIZE * 5) : len);
    if(!EVP_EncodeUpdate(&ctx, buf, &outl, &(data[j]), n)) {
      goto err;
    }
    if ((outl) && (BIO_write(bp, (char *)buf, outl) != outl)) {
      goto err;
    }
    i += outl;
    len -= n;
    j += n;
  }
  EVP_EncodeFinal(&ctx, buf, &outl);
  if ((outl > 0) && (BIO_write(bp, (char *)buf, outl) != outl)) {
    goto err;
  }
  OPENSSL_free(buf);
  buf = NULL;
  if ((BIO_write(bp, "-----END ", 9) != 9) ||
      (BIO_write(bp, name, nlen) != nlen) ||
      (BIO_write(bp, "-----\n", 6) != 6)) {
    goto err;
  }
  if (i + outl > INT_MAX) {
    reason = ERR_R_OVERFLOW;
    goto err;
  }
  return (int)(i + outl);
err:
  if (buf) {
    OPENSSL_free(buf);
  }
  OPENSSL_PUT_ERROR(PEM, reason);
  return 0;
}

int PEM_read(FILE *fp, char **name, char **header, unsigned char **data,
             long *len) {
  BIO *b = BIO_new_fp(fp, BIO_NOCLOSE);
  if (b == NULL) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_BUF_LIB);
    return 0;
  }
  int ret = PEM_read_bio(b, name, header, data, len);
  BIO_free(b);
  return ret;
}

int PEM_read_bio(BIO *bp, char **name, char **header, unsigned char **data,
                 long *len) {
  EVP_ENCODE_CTX ctx;
  int end = 0, i, k, bl = 0, hl = 0, nohead = 0;
  char buf[256];
  BUF_MEM *nameB;
  BUF_MEM *headerB;
  BUF_MEM *dataB, *tmpB;

  nameB = BUF_MEM_new();
  headerB = BUF_MEM_new();
  dataB = BUF_MEM_new();
  if ((nameB == NULL) || (headerB == NULL) || (dataB == NULL)) {
    BUF_MEM_free(nameB);
    BUF_MEM_free(headerB);
    BUF_MEM_free(dataB);
    return 0;
  }

  buf[254] = '\0';
  for (;;) {
    i = BIO_gets(bp, buf, 254);

    if (i <= 0) {
      OPENSSL_PUT_ERROR(PEM, PEM_R_NO_START_LINE);
      goto err;
    }

    while ((i >= 0) && (buf[i] <= ' ')) {
      i--;
    }
    buf[++i] = '\n';
    buf[++i] = '\0';

    if (strncmp(buf, "-----BEGIN ", 11) == 0) {
      i = strlen(&(buf[11]));

      if (strncmp(&(buf[11 + i - 6]), "-----\n", 6) != 0) {
        continue;
      }
      if (!BUF_MEM_grow(nameB, i + 9)) {
        goto err;
      }
      OPENSSL_memcpy(nameB->data, &(buf[11]), i - 6);
      nameB->data[i - 6] = '\0';
      break;
    }
  }
  hl = 0;
  if (!BUF_MEM_grow(headerB, 256)) {
    goto err;
  }
  headerB->data[0] = '\0';
  for (;;) {
    i = BIO_gets(bp, buf, 254);
    if (i <= 0) {
      break;
    }

    while ((i >= 0) && (buf[i] <= ' ')) {
      i--;
    }
    buf[++i] = '\n';
    buf[++i] = '\0';

    if (buf[0] == '\n') {
      break;
    }
    if (!BUF_MEM_grow(headerB, hl + i + 9)) {
      goto err;
    }
    // To resolve following error:
    // /home/runner/work/aws-lc-rs/aws-lc-rs/aws-lc-sys/aws-lc/crypto/pem/pem_lib.c:707:11: error: 'strncmp' of strings of length 1 and 9 and bound of 9 evaluates to nonzero [-Werror=string-compare]
    //       707 |       if (strncmp(buf, "-----END ", 9) == 0) {
    //           |           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
    if (CRYPTO_memcmp(buf, "-----END ", 9) == 0) {
      nohead = 1;
      break;
    }
    OPENSSL_memcpy(&(headerB->data[hl]), buf, i);
    headerB->data[hl + i] = '\0';
    hl += i;
  }

  bl = 0;
  if (!BUF_MEM_grow(dataB, 1024)) {
    goto err;
  }
  dataB->data[0] = '\0';
  if (!nohead) {
    for (;;) {
      i = BIO_gets(bp, buf, 254);
      if (i <= 0) {
        break;
      }

      while ((i >= 0) && (buf[i] <= ' ')) {
        i--;
      }
      buf[++i] = '\n';
      buf[++i] = '\0';

      if (i != 65) {
        end = 1;
      }
      if (CRYPTO_memcmp(buf, "-----END ", 9) == 0) {
        break;
      }
      if (i > 65) {
        break;
      }
      if (!BUF_MEM_grow_clean(dataB, i + bl + 9)) {
        goto err;
      }
      OPENSSL_memcpy(&(dataB->data[bl]), buf, i);
      dataB->data[bl + i] = '\0';
      bl += i;
      if (end) {
        buf[0] = '\0';
        i = BIO_gets(bp, buf, 254);
        if (i <= 0) {
          break;
        }

        while ((i >= 0) && (buf[i] <= ' ')) {
          i--;
        }
        buf[++i] = '\n';
        buf[++i] = '\0';

        break;
      }
    }
  } else {
    tmpB = headerB;
    headerB = dataB;
    dataB = tmpB;
    bl = hl;
  }
  i = strlen(nameB->data);
  if ((CRYPTO_memcmp(buf, "-----END ", 9) != 0) ||
      (strncmp(nameB->data, &(buf[9]), i) != 0) ||
      (strncmp(&(buf[9 + i]), "-----\n", 6) != 0)) {
    OPENSSL_PUT_ERROR(PEM, PEM_R_BAD_END_LINE);
    goto err;
  }

  EVP_DecodeInit(&ctx);
  i = EVP_DecodeUpdate(&ctx, (unsigned char *)dataB->data, &bl,
                       (unsigned char *)dataB->data, bl);
  if (i < 0) {
    OPENSSL_PUT_ERROR(PEM, PEM_R_BAD_BASE64_DECODE);
    goto err;
  }
  i = EVP_DecodeFinal(&ctx, (unsigned char *)&(dataB->data[bl]), &k);
  if (i < 0) {
    OPENSSL_PUT_ERROR(PEM, PEM_R_BAD_BASE64_DECODE);
    goto err;
  }
  bl += k;

  if (bl == 0) {
    goto err;
  }
  *name = nameB->data;
  *header = headerB->data;
  *data = (unsigned char *)dataB->data;
  *len = bl;
  OPENSSL_cleanse(buf, sizeof(buf));
  OPENSSL_free(nameB);
  OPENSSL_free(headerB);
  OPENSSL_free(dataB);
  return 1;
err:
  OPENSSL_cleanse(buf, sizeof(buf));
  BUF_MEM_free(nameB);
  BUF_MEM_free(headerB);
  BUF_MEM_free(dataB);
  return 0;
}

int PEM_def_callback(char *buf, int size, int rwflag, void *userdata) {
  if (!buf || size <= 0) {
    return 0;
  }

  // Proactively zeroize |buf|
  OPENSSL_cleanse(buf, size);

  if (userdata) {
    size_t len =  strlen((char *)userdata);
    if (len >= (size_t)size) {
      return 0;
    }
    OPENSSL_strlcpy(buf, userdata, (size_t)size);
    return (int)len;
  }

  const char *prompt = EVP_get_pw_prompt();
  if (prompt == NULL) {
    prompt = "Enter PEM pass phrase:";
  }

  /*
     * rwflag == 0 means decryption
     * rwflag == 1 means encryption
     *
     * We assume that for encryption, we want a minimum length, while for
     * decryption, we cannot know any minimum length, so we assume zero.
     */
  int min_len = rwflag ? MIN_LENGTH : 0;

  int ret = EVP_read_pw_string_min(buf, min_len, size, prompt, rwflag);
  if (ret != 0) {
    return 0;
  }

  return (int)OPENSSL_strnlen(buf, size);
}
