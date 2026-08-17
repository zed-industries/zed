// Copyright (c) 1998-2002 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0
 
#include <openssl/bn.h>
#include <openssl/dh.h>


struct wrapped_callback {
  void (*callback)(int, int, void *);
  void *arg;
};

// callback_wrapper converts an “old” style generation callback to the newer
// |BN_GENCB| form.
static int callback_wrapper(int event, int n, BN_GENCB *gencb) {
  struct wrapped_callback *wrapped = (struct wrapped_callback *) gencb->arg;
  wrapped->callback(event, n, wrapped->arg);
  return 1;
}

DH *DH_generate_parameters(int prime_len, int generator,
                           void (*callback)(int, int, void *), void *cb_arg) {
  if (prime_len < 0 || generator < 0) {
      return NULL;
  }

  DH *ret = DH_new();
  if (ret == NULL) {
      return NULL;
  }

  BN_GENCB gencb_storage;
  BN_GENCB *cb = NULL;

  struct wrapped_callback wrapped;

  if (callback != NULL) {
    wrapped.callback = callback;
    wrapped.arg = cb_arg;

    cb = &gencb_storage;
    BN_GENCB_set(cb, callback_wrapper, &wrapped);
  }

  if (!DH_generate_parameters_ex(ret, prime_len, generator, cb)) {
    goto err;
  }

  return ret;

err:
  DH_free(ret);
  return NULL;
}
