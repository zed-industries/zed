// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include "x509_util.h"
#include "test_util.h"

int Verify(X509 *leaf, const std::vector<X509 *> &roots,
           const std::vector<X509 *> &intermediates,
           const std::vector<X509_CRL *> &crls, unsigned long flags,
           std::function<void(X509_STORE_CTX *)> configure_callback) {
  bssl::UniquePtr<STACK_OF(X509)> roots_stack(CertsToStack(roots));
  bssl::UniquePtr<STACK_OF(X509)> intermediates_stack(
      CertsToStack(intermediates));
  bssl::UniquePtr<STACK_OF(X509_CRL)> crls_stack(CRLsToStack(crls));

  if (!roots_stack || !intermediates_stack || !crls_stack) {
    return X509_V_ERR_UNSPECIFIED;
  }

  bssl::UniquePtr<X509_STORE_CTX> ctx(X509_STORE_CTX_new());
  bssl::UniquePtr<X509_STORE> store(X509_STORE_new());
  if (!ctx || !store) {
    return X509_V_ERR_UNSPECIFIED;
  }

  if (!X509_STORE_CTX_init(ctx.get(), store.get(), leaf,
                           intermediates_stack.get())) {
    return X509_V_ERR_UNSPECIFIED;
  }

  X509_STORE_CTX_set0_trusted_stack(ctx.get(), roots_stack.get());
  X509_STORE_CTX_set0_crls(ctx.get(), crls_stack.get());

  X509_VERIFY_PARAM *param = X509_STORE_CTX_get0_param(ctx.get());
  X509_VERIFY_PARAM_set_time_posix(param, kReferenceTime);
  if (configure_callback) {
    configure_callback(ctx.get());
  }
  if (flags) {
    X509_VERIFY_PARAM_set_flags(param, flags);
  }

  ERR_clear_error();
  if (X509_verify_cert(ctx.get()) != 1) {
    return X509_STORE_CTX_get_error(ctx.get());
  }

  return X509_V_OK;
}

bssl::UniquePtr<STACK_OF(X509_CRL)> CRLsToStack(
    const std::vector<X509_CRL *> &crls) {
  bssl::UniquePtr<STACK_OF(X509_CRL)> stack(sk_X509_CRL_new_null());
  if (!stack) {
    return nullptr;
  }
  for (auto crl : crls) {
    if (!bssl::PushToStack(stack.get(), bssl::UpRef(crl))) {
      return nullptr;
    }
  }

  return stack;
}
