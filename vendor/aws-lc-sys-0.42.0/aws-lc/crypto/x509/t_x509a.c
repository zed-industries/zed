// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>
#include <openssl/bio.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "internal.h"


// X509_CERT_AUX and string set routines

int X509_CERT_AUX_print(BIO *out, X509_CERT_AUX *aux, int indent) {
  char oidstr[80], first;
  size_t i;
  int j;
  if (!aux) {
    return 1;
  }
  if (aux->trust) {
    first = 1;
    BIO_printf(out, "%*sTrusted Uses:\n%*s", indent, "", indent + 2, "");
    for (i = 0; i < sk_ASN1_OBJECT_num(aux->trust); i++) {
      if (!first) {
        BIO_puts(out, ", ");
      } else {
        first = 0;
      }
      OBJ_obj2txt(oidstr, sizeof oidstr, sk_ASN1_OBJECT_value(aux->trust, i),
                  0);
      BIO_puts(out, oidstr);
    }
    BIO_puts(out, "\n");
  } else {
    BIO_printf(out, "%*sNo Trusted Uses.\n", indent, "");
  }
  if (aux->reject) {
    first = 1;
    BIO_printf(out, "%*sRejected Uses:\n%*s", indent, "", indent + 2, "");
    for (i = 0; i < sk_ASN1_OBJECT_num(aux->reject); i++) {
      if (!first) {
        BIO_puts(out, ", ");
      } else {
        first = 0;
      }
      OBJ_obj2txt(oidstr, sizeof oidstr, sk_ASN1_OBJECT_value(aux->reject, i),
                  0);
      BIO_puts(out, oidstr);
    }
    BIO_puts(out, "\n");
  } else {
    BIO_printf(out, "%*sNo Rejected Uses.\n", indent, "");
  }
  if (aux->alias) {
    BIO_printf(out, "%*sAlias: %.*s\n", indent, "", aux->alias->length,
               aux->alias->data);
  }
  if (aux->keyid) {
    BIO_printf(out, "%*sKey Id: ", indent, "");
    for (j = 0; j < aux->keyid->length; j++) {
      BIO_printf(out, "%s%02X", j ? ":" : "", aux->keyid->data[j]);
    }
    BIO_write(out, "\n", 1);
  }
  return 1;
}
