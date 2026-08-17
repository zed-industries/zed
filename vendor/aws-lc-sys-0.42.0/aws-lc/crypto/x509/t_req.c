// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <assert.h>
#include <stdio.h>

#include <openssl/bn.h>
#include <openssl/buffer.h>
#include <openssl/err.h>
#include <openssl/objects.h>
#include <openssl/x509.h>

#include "internal.h"


int X509_REQ_print_fp(FILE *fp, X509_REQ *x) {
  BIO *bio = BIO_new_fp(fp, BIO_NOCLOSE);
  if (bio == NULL) {
    OPENSSL_PUT_ERROR(X509, ERR_R_BUF_LIB);
    return 0;
  }
  int ret = X509_REQ_print(bio, x);
  BIO_free(bio);
  return ret;
}

int X509_REQ_print_ex(BIO *bio, X509_REQ *x, unsigned long nmflags,
                      unsigned long cflag) {
  long l;
  STACK_OF(X509_ATTRIBUTE) *sk;
  char mlch = ' ';

  int nmindent = 0;

  if ((nmflags & XN_FLAG_SEP_MASK) == XN_FLAG_SEP_MULTILINE) {
    mlch = '\n';
    nmindent = 12;
  }

  if (nmflags == X509_FLAG_COMPAT) {
    nmindent = 16;
  }

  X509_REQ_INFO *ri = x->req_info;
  if (!(cflag & X509_FLAG_NO_HEADER)) {
    if (BIO_write(bio, "Certificate Request:\n", 21) <= 0 ||
        BIO_write(bio, "    Data:\n", 10) <= 0) {
      goto err;
    }
  }
  if (!(cflag & X509_FLAG_NO_VERSION)) {
    l = X509_REQ_get_version(x);
    // Only zero, |X509_REQ_VERSION_1|, is valid but our parser accepts some
    // invalid values for compatibility.
    assert(0 <= l && l <= 2);
    if (BIO_printf(bio, "%8sVersion: %ld (0x%lx)\n", "", l + 1,
                   (unsigned long)l) <= 0) {
      goto err;
    }
  }
  if (!(cflag & X509_FLAG_NO_SUBJECT)) {
    if (BIO_printf(bio, "        Subject:%c", mlch) <= 0 ||
        X509_NAME_print_ex(bio, ri->subject, nmindent, nmflags) < 0 ||
        BIO_write(bio, "\n", 1) <= 0) {
      goto err;
    }
  }
  if (!(cflag & X509_FLAG_NO_PUBKEY)) {
    if (BIO_write(bio, "        Subject Public Key Info:\n", 33) <= 0 ||
        BIO_printf(bio, "%12sPublic Key Algorithm: ", "") <= 0 ||
        i2a_ASN1_OBJECT(bio, ri->pubkey->algor->algorithm) <= 0 ||
        BIO_puts(bio, "\n") <= 0) {
      goto err;
    }

    const EVP_PKEY *pkey = X509_REQ_get0_pubkey(x);
    if (pkey == NULL) {
      BIO_printf(bio, "%12sUnable to load Public Key\n", "");
      ERR_print_errors(bio);
    } else {
      EVP_PKEY_print_public(bio, pkey, 16, NULL);
    }
  }

  if (!(cflag & X509_FLAG_NO_ATTRIBUTES)) {
    if (BIO_printf(bio, "%8sAttributes:\n", "") <= 0) {
      goto err;
    }

    sk = x->req_info->attributes;
    if (sk_X509_ATTRIBUTE_num(sk) == 0) {
      if (BIO_printf(bio, "%12sa0:00\n", "") <= 0) {
        goto err;
      }
    } else {
      size_t i;
      for (i = 0; i < sk_X509_ATTRIBUTE_num(sk); i++) {
        X509_ATTRIBUTE *a = sk_X509_ATTRIBUTE_value(sk, i);
        ASN1_OBJECT *aobj = X509_ATTRIBUTE_get0_object(a);

        if (X509_REQ_extension_nid(OBJ_obj2nid(aobj))) {
          continue;
        }

        if (BIO_printf(bio, "%12s", "") <= 0) {
          goto err;
        }

        const int num_attrs = X509_ATTRIBUTE_count(a);
        const int obj_str_len = i2a_ASN1_OBJECT(bio, aobj);
        if (obj_str_len <= 0) {
          if (BIO_puts(bio, "(Unable to print attribute ID.)\n") < 0) {
            goto err;
          } else {
            continue;
          }
        }

        int j;
        for (j = 0; j < num_attrs; j++) {
          const ASN1_TYPE *at = X509_ATTRIBUTE_get0_type(a, j);
          const int type = at->type;
          ASN1_BIT_STRING *bs = at->value.asn1_string;

          int k;
          for (k = 25 - obj_str_len; k > 0; k--) {
            if (BIO_write(bio, " ", 1) != 1) {
              goto err;
            }
          }

          if (BIO_puts(bio, ":") <= 0) {
            goto err;
          }

          if (type == V_ASN1_PRINTABLESTRING || type == V_ASN1_UTF8STRING ||
              type == V_ASN1_IA5STRING || type == V_ASN1_T61STRING) {
            if (BIO_write(bio, (char *)bs->data, bs->length) != bs->length) {
              goto err;
            }
            BIO_puts(bio, "\n");
          } else {
            BIO_puts(bio, "unable to print attribute\n");
          }
        }
      }
    }
  }

  if (!(cflag & X509_FLAG_NO_EXTENSIONS)) {
    STACK_OF(X509_EXTENSION) *exts = X509_REQ_get_extensions(x);
    if (exts) {
      BIO_printf(bio, "%8sRequested Extensions:\n", "");

      for (size_t i = 0; i < sk_X509_EXTENSION_num(exts); i++) {
        const X509_EXTENSION *ex = sk_X509_EXTENSION_value(exts, i);
        if (BIO_printf(bio, "%12s", "") <= 0) {
          goto err;
        }
        const ASN1_OBJECT *obj = X509_EXTENSION_get_object(ex);
        i2a_ASN1_OBJECT(bio, obj);
        const int is_critical = X509_EXTENSION_get_critical(ex);
        if (BIO_printf(bio, ": %s\n", is_critical ? "critical" : "") <= 0) {
          goto err;
        }
        if (!X509V3_EXT_print(bio, ex, cflag, 16)) {
          BIO_printf(bio, "%16s", "");
          ASN1_STRING_print(bio, X509_EXTENSION_get_data(ex));
        }
        if (BIO_write(bio, "\n", 1) <= 0) {
          goto err;
        }
      }
      sk_X509_EXTENSION_pop_free(exts, X509_EXTENSION_free);
    }
  }

  if (!(cflag & X509_FLAG_NO_SIGDUMP) &&
      !X509_signature_print(bio, x->sig_alg, x->signature)) {
    goto err;
  }

  return 1;

err:
  OPENSSL_PUT_ERROR(X509, ERR_R_BUF_LIB);
  return 0;
}

int X509_REQ_print(BIO *bio, X509_REQ *req) {
  return X509_REQ_print_ex(bio, req, XN_FLAG_COMPAT, X509_FLAG_COMPAT);
}
