// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef OPENSSL_HEADER_CRYPTO_TEST_X509_UTIL_H
#define OPENSSL_HEADER_CRYPTO_TEST_X509_UTIL_H

#include <functional>
#include <vector>

#include <openssl/x509.h>

int Verify(
    X509 *leaf, const std::vector<X509 *> &roots,
    const std::vector<X509 *> &intermediates,
    const std::vector<X509_CRL *> &crls, unsigned long flags = 0,
    std::function<void(X509_STORE_CTX *)> configure_callback = nullptr);

// CRLsToStack converts a vector of |X509_CRL*| to an OpenSSL
// STACK_OF(X509_CRL), bumping the reference counts for each CRL in question.
bssl::UniquePtr<STACK_OF(X509_CRL)> CRLsToStack(
    const std::vector<X509_CRL *> &crls);

#endif
