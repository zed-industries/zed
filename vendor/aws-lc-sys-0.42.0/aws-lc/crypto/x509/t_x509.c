// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <assert.h>

#include <openssl/asn1.h>
#include <openssl/bio.h>
#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "internal.h"


int X509_print_ex_fp(FILE *fp, X509 *x, unsigned long nmflag,
                     unsigned long cflag) {
  BIO *b = BIO_new_fp(fp, BIO_NOCLOSE);
  if (b == NULL) {
    OPENSSL_PUT_ERROR(X509, ERR_R_BUF_LIB);
    return 0;
  }
  int ret = X509_print_ex(b, x, nmflag, cflag);
  BIO_free(b);
  return ret;
}

int X509_print_fp(FILE *fp, X509 *x) {
  return X509_print_ex_fp(fp, x, XN_FLAG_COMPAT, X509_FLAG_COMPAT);
}

int X509_print(BIO *bp, X509 *x) {
  return X509_print_ex(bp, x, XN_FLAG_COMPAT, X509_FLAG_COMPAT);
}

int X509_print_ex(BIO *bp, X509 *x, unsigned long nmflags,
                  unsigned long cflag) {
  char mlch = ' ';
  int nmindent = 0;
  if ((nmflags & XN_FLAG_SEP_MASK) == XN_FLAG_SEP_MULTILINE) {
    mlch = '\n';
    nmindent = 12;
  }

  if (nmflags == X509_FLAG_COMPAT) {
    nmindent = 16;
  }

  const X509_CINF *ci = x->cert_info;
  if (!(cflag & X509_FLAG_NO_HEADER)) {
    if (BIO_write(bp, "Certificate:\n", 13) <= 0) {
      return 0;
    }
    if (BIO_write(bp, "    Data:\n", 10) <= 0) {
      return 0;
    }
  }
  if (!(cflag & X509_FLAG_NO_VERSION)) {
    long l = X509_get_version(x);
    assert(X509_VERSION_1 <= l && l <= X509_VERSION_3);
    if (BIO_printf(bp, "%8sVersion: %ld (0x%lx)\n", "", l + 1,
                   (unsigned long)l) <= 0) {
      return 0;
    }
  }
  if (!(cflag & X509_FLAG_NO_SERIAL)) {
    if (BIO_write(bp, "        Serial Number:", 22) <= 0) {
      return 0;
    }

    const ASN1_INTEGER *serial = X509_get0_serialNumber(x);
    uint64_t serial_u64;
    if (ASN1_INTEGER_get_uint64(&serial_u64, serial)) {
      assert(serial->type != V_ASN1_NEG_INTEGER);
      if (BIO_printf(bp, " %" PRIu64 " (0x%" PRIx64 ")\n", serial_u64,
                     serial_u64) <= 0) {
        return 0;
      }
    } else {
      ERR_clear_error();  // Clear |ASN1_INTEGER_get_uint64|'s error.
      const char *neg =
          (serial->type == V_ASN1_NEG_INTEGER) ? " (Negative)" : "";
      if (BIO_printf(bp, "\n%12s%s", "", neg) <= 0) {
        return 0;
      }

      for (int i = 0; i < serial->length; i++) {
        if (BIO_printf(bp, "%02x%c", serial->data[i],
                       ((i + 1 == serial->length) ? '\n' : ':')) <= 0) {
          return 0;
        }
      }
    }
  }

  if (!(cflag & X509_FLAG_NO_SIGNAME)) {
    if (X509_signature_print(bp, ci->signature, NULL) <= 0) {
      return 0;
    }
  }

  if (!(cflag & X509_FLAG_NO_ISSUER)) {
    if (BIO_printf(bp, "        Issuer:%c", mlch) <= 0) {
      return 0;
    }
    if (X509_NAME_print_ex(bp, X509_get_issuer_name(x), nmindent, nmflags) <
        0) {
      return 0;
    }
    if (BIO_write(bp, "\n", 1) <= 0) {
      return 0;
    }
  }
  if (!(cflag & X509_FLAG_NO_VALIDITY)) {
    if (BIO_write(bp, "        Validity\n", 17) <= 0) {
      return 0;
    }
    if (BIO_write(bp, "            Not Before: ", 24) <= 0) {
      return 0;
    }
    if (!ASN1_TIME_print(bp, X509_get_notBefore(x))) {
      return 0;
    }
    if (BIO_write(bp, "\n            Not After : ", 25) <= 0) {
      return 0;
    }
    if (!ASN1_TIME_print(bp, X509_get_notAfter(x))) {
      return 0;
    }
    if (BIO_write(bp, "\n", 1) <= 0) {
      return 0;
    }
  }
  if (!(cflag & X509_FLAG_NO_SUBJECT)) {
    if (BIO_printf(bp, "        Subject:%c", mlch) <= 0) {
      return 0;
    }
    if (X509_NAME_print_ex(bp, X509_get_subject_name(x), nmindent, nmflags) <
        0) {
      return 0;
    }
    if (BIO_write(bp, "\n", 1) <= 0) {
      return 0;
    }
  }
  if (!(cflag & X509_FLAG_NO_PUBKEY)) {
    if (BIO_write(bp, "        Subject Public Key Info:\n", 33) <= 0) {
      return 0;
    }
    if (BIO_printf(bp, "%12sPublic Key Algorithm: ", "") <= 0) {
      return 0;
    }
    if (i2a_ASN1_OBJECT(bp, ci->key->algor->algorithm) <= 0) {
      return 0;
    }
    if (BIO_puts(bp, "\n") <= 0) {
      return 0;
    }

    const EVP_PKEY *pkey = X509_get0_pubkey(x);
    if (pkey == NULL) {
      BIO_printf(bp, "%12sUnable to load Public Key\n", "");
      ERR_print_errors(bp);
    } else {
      EVP_PKEY_print_public(bp, pkey, 16, NULL);
    }
  }

  if (!(cflag & X509_FLAG_NO_IDS)) {
    if (ci->issuerUID) {
      if (BIO_printf(bp, "%8sIssuer Unique ID: ", "") <= 0) {
        return 0;
      }
      if (!X509_signature_dump(bp, ci->issuerUID, 12)) {
        return 0;
      }
    }
    if (ci->subjectUID) {
      if (BIO_printf(bp, "%8sSubject Unique ID: ", "") <= 0) {
        return 0;
      }
      if (!X509_signature_dump(bp, ci->subjectUID, 12)) {
        return 0;
      }
    }
  }

  if (!(cflag & X509_FLAG_NO_EXTENSIONS)) {
    X509V3_extensions_print(bp, "X509v3 extensions", ci->extensions, cflag, 8);
  }

  if (!(cflag & X509_FLAG_NO_SIGDUMP)) {
    if (X509_signature_print(bp, x->sig_alg, x->signature) <= 0) {
      return 0;
    }
  }
  if (!(cflag & X509_FLAG_NO_AUX)) {
    if (!X509_CERT_AUX_print(bp, x->aux, 0)) {
      return 0;
    }
  }

  return 1;
}

int X509_signature_print(BIO *bp, const X509_ALGOR *sigalg,
                         const ASN1_STRING *sig) {
  if (BIO_puts(bp, "    Signature Algorithm: ") <= 0) {
    return 0;
  }
  if (i2a_ASN1_OBJECT(bp, sigalg->algorithm) <= 0) {
    return 0;
  }

  // RSA-PSS signatures have parameters to print.
  int sig_nid = OBJ_obj2nid(sigalg->algorithm);
  if (sig_nid == NID_rsassaPss &&
      !x509_print_rsa_pss_params(bp, sigalg, 9, 0)) {
    return 0;
  }

  if (sig) {
    return X509_signature_dump(bp, sig, 9);
  } else if (BIO_puts(bp, "\n") <= 0) {
    return 0;
  }
  return 1;
}

int X509_NAME_print(BIO *bp, const X509_NAME *name, int obase) {
  char *s, *c, *b;
  int ret = 0, i;

  b = X509_NAME_oneline(name, NULL, 0);
  if (!b) {
    return 0;
  }
  if (!*b) {
    OPENSSL_free(b);
    return 1;
  }
  s = b + 1;  // skip the first slash

  c = s;
  for (;;) {
    if (((*s == '/') && ((s[1] >= 'A') && (s[1] <= 'Z') &&
                         ((s[2] == '=') || ((s[2] >= 'A') && (s[2] <= 'Z') &&
                                            (s[3] == '='))))) ||
        (*s == '\0')) {
      i = s - c;
      if (BIO_write(bp, c, i) != i) {
        goto err;
      }
      c = s + 1;  // skip following slash
      if (*s != '\0') {
        if (BIO_write(bp, ", ", 2) != 2) {
          goto err;
        }
      }
    }
    if (*s == '\0') {
      break;
    }
    s++;
  }

  ret = 1;
  if (0) {
  err:
    OPENSSL_PUT_ERROR(X509, ERR_R_BUF_LIB);
  }
  OPENSSL_free(b);
  return ret;
}
