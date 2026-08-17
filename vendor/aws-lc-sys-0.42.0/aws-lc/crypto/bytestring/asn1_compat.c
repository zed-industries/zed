// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC


#include <openssl/bytestring.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/mem.h>

#include "internal.h"
#include "../internal.h"


int CBB_finish_i2d(CBB *cbb, uint8_t **outp) {
  assert(!cbb->is_child);
  assert(cbb->u.base.can_resize);

  uint8_t *der;
  size_t der_len;
  if (!CBB_finish(cbb, &der, &der_len)) {
    CBB_cleanup(cbb);
    return -1;
  }
  if (der_len > INT_MAX) {
    OPENSSL_free(der);
    return -1;
  }
  if (outp != NULL) {
    if (*outp == NULL) {
      *outp = der;
      der = NULL;
    } else {
      OPENSSL_memcpy(*outp, der, der_len);
      *outp += der_len;
    }
  }
  OPENSSL_free(der);
  return (int)der_len;
}
