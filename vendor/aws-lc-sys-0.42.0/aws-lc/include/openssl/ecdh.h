// Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
//
// The Elliptic Curve Public-Key Crypto Library (ECC Code) included
// herein is developed by SUN MICROSYSTEMS, INC., and is contributed
// to the OpenSSL project.
// 
// The ECDH software is originally written by Douglas Stebila of
// Sun Microsystems Laboratories.
//
// Copyright (c) 2000-2002 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_ECDH_H
#define OPENSSL_HEADER_ECDH_H

#include <openssl/base.h>

#include <openssl/ec_key.h>

#if defined(__cplusplus)
extern "C" {
#endif


// Elliptic curve Diffie-Hellman.


// ECDH_compute_key calculates the shared key between |pub_key| and |priv_key|.
// If |kdf| is not NULL, then it is called with the bytes of the shared key and
// the parameter |out|. When |kdf| returns, the value of |*outlen| becomes the
// return value. Otherwise, as many bytes of the shared key as will fit are
// copied directly to, at most, |outlen| bytes at |out|. It returns the number
// of bytes written to |out|, or -1 on error.
OPENSSL_EXPORT int ECDH_compute_key(
    void *out, size_t outlen, const EC_POINT *pub_key, const EC_KEY *priv_key,
    void *(*kdf)(const void *in, size_t inlen, void *out, size_t *outlen));

// ECDH_compute_key_fips calculates the shared key between |pub_key| and
// |priv_key| and hashes it with the appropriate SHA function for |out_len|. The
// only value values for |out_len| are thus 24 (SHA-224), 32 (SHA-256), 48
// (SHA-384), and 64 (SHA-512). It returns one on success and zero on error.
//
// Note that the return value is different to |ECDH_compute_key|: it returns an
// error flag (as is common for BoringSSL) rather than the number of bytes
// written.
//
// This function allows the FIPS module to compute an ECDH and KDF within the
// module boundary without taking an arbitrary function pointer for the KDF,
// which isn't very FIPSy.
OPENSSL_EXPORT int ECDH_compute_key_fips(uint8_t *out, size_t out_len,
                                         const EC_POINT *pub_key,
                                         const EC_KEY *priv_key);


#if defined(__cplusplus)
}  // extern C
#endif

#define ECDH_R_KDF_FAILED 100
#define ECDH_R_NO_PRIVATE_VALUE 101
#define ECDH_R_POINT_ARITHMETIC_FAILURE 102
#define ECDH_R_UNKNOWN_DIGEST_LENGTH 103

#endif  // OPENSSL_HEADER_ECDH_H
