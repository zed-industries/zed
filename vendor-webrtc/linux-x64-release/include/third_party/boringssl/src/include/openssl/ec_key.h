// Copyright 2000-2016 The OpenSSL Project Authors. All Rights Reserved.
// Copyright (c) 2002, Oracle and/or its affiliates. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_EC_KEY_H
#define OPENSSL_HEADER_EC_KEY_H

#include <openssl/base.h>   // IWYU pragma: export

#include <openssl/ec.h>
#include <openssl/engine.h>
#include <openssl/ex_data.h>

#if defined(__cplusplus)
extern "C" {
#endif


// ec_key.h contains functions that handle elliptic-curve points that are
// public/private keys.


// EC key objects.
//
// An |EC_KEY| object represents a public or private EC key. A given object may
// be used concurrently on multiple threads by non-mutating functions, provided
// no other thread is concurrently calling a mutating function. Unless otherwise
// documented, functions which take a |const| pointer are non-mutating and
// functions which take a non-|const| pointer are mutating.

// EC_KEY_new returns a fresh |EC_KEY| object or NULL on error.
OPENSSL_EXPORT EC_KEY *EC_KEY_new(void);

// EC_KEY_new_method acts the same as |EC_KEY_new|, but takes an explicit
// |ENGINE|.
OPENSSL_EXPORT EC_KEY *EC_KEY_new_method(const ENGINE *engine);

// EC_KEY_new_by_curve_name returns a fresh EC_KEY for group specified by |nid|
// or NULL on error.
OPENSSL_EXPORT EC_KEY *EC_KEY_new_by_curve_name(int nid);

// EC_KEY_free frees all the data owned by |key| and |key| itself.
OPENSSL_EXPORT void EC_KEY_free(EC_KEY *key);

// EC_KEY_dup returns a fresh copy of |src| or NULL on error.
OPENSSL_EXPORT EC_KEY *EC_KEY_dup(const EC_KEY *src);

// EC_KEY_up_ref increases the reference count of |key| and returns one. It does
// not mutate |key| for thread-safety purposes and may be used concurrently.
OPENSSL_EXPORT int EC_KEY_up_ref(EC_KEY *key);

// EC_KEY_is_opaque returns one if |key| is opaque and doesn't expose its key
// material. Otherwise it return zero.
OPENSSL_EXPORT int EC_KEY_is_opaque(const EC_KEY *key);

// EC_KEY_get0_group returns a pointer to the |EC_GROUP| object inside |key|.
OPENSSL_EXPORT const EC_GROUP *EC_KEY_get0_group(const EC_KEY *key);

// EC_KEY_set_group sets the |EC_GROUP| object that |key| will use to |group|.
// It returns one on success and zero if |key| is already configured with a
// different group.
OPENSSL_EXPORT int EC_KEY_set_group(EC_KEY *key, const EC_GROUP *group);

// EC_KEY_get0_private_key returns a pointer to the private key inside |key|.
OPENSSL_EXPORT const BIGNUM *EC_KEY_get0_private_key(const EC_KEY *key);

// EC_KEY_set_private_key sets the private key of |key| to |priv|. It returns
// one on success and zero otherwise. |key| must already have had a group
// configured (see |EC_KEY_set_group| and |EC_KEY_new_by_curve_name|).
OPENSSL_EXPORT int EC_KEY_set_private_key(EC_KEY *key, const BIGNUM *priv);

// EC_KEY_get0_public_key returns a pointer to the public key point inside
// |key|.
OPENSSL_EXPORT const EC_POINT *EC_KEY_get0_public_key(const EC_KEY *key);

// EC_KEY_set_public_key sets the public key of |key| to |pub|, by copying it.
// It returns one on success and zero otherwise. |key| must already have had a
// group configured (see |EC_KEY_set_group| and |EC_KEY_new_by_curve_name|), and
// |pub| must also belong to that group.
OPENSSL_EXPORT int EC_KEY_set_public_key(EC_KEY *key, const EC_POINT *pub);

#define EC_PKEY_NO_PARAMETERS 0x001
#define EC_PKEY_NO_PUBKEY 0x002

// EC_KEY_get_enc_flags returns the encoding flags for |key|, which is a
// bitwise-OR of |EC_PKEY_*| values.
OPENSSL_EXPORT unsigned EC_KEY_get_enc_flags(const EC_KEY *key);

// EC_KEY_set_enc_flags sets the encoding flags for |key|, which is a
// bitwise-OR of |EC_PKEY_*| values.
OPENSSL_EXPORT void EC_KEY_set_enc_flags(EC_KEY *key, unsigned flags);

// EC_KEY_get_conv_form returns the conversation form that will be used by
// |key|.
OPENSSL_EXPORT point_conversion_form_t EC_KEY_get_conv_form(const EC_KEY *key);

// EC_KEY_set_conv_form sets the conversion form to be used by |key|.
OPENSSL_EXPORT void EC_KEY_set_conv_form(EC_KEY *key,
                                         point_conversion_form_t cform);

// EC_KEY_check_key performs several checks on |key| (possibly including an
// expensive check that the public key is in the primary subgroup). It returns
// one if all checks pass and zero otherwise. If it returns zero then detail
// about the problem can be found on the error stack.
OPENSSL_EXPORT int EC_KEY_check_key(const EC_KEY *key);

// EC_KEY_check_fips performs both a signing pairwise consistency test
// (FIPS 140-2 4.9.2) and the consistency test from SP 800-56Ar3 section
// 5.6.2.1.4. It returns one if it passes and zero otherwise.
OPENSSL_EXPORT int EC_KEY_check_fips(const EC_KEY *key);

// EC_KEY_set_public_key_affine_coordinates sets the public key in |key| to
// (|x|, |y|). It returns one on success and zero on error. It's considered an
// error if |x| and |y| do not represent a point on |key|'s curve.
OPENSSL_EXPORT int EC_KEY_set_public_key_affine_coordinates(EC_KEY *key,
                                                            const BIGNUM *x,
                                                            const BIGNUM *y);

// EC_KEY_oct2key decodes |len| bytes from |in| as an EC public key in X9.62
// form. |key| must already have a group configured. On success, it sets the
// public key in |key| to the result and returns one. Otherwise, it returns
// zero.
OPENSSL_EXPORT int EC_KEY_oct2key(EC_KEY *key, const uint8_t *in, size_t len,
                                  BN_CTX *ctx);

// EC_KEY_key2buf behaves like |EC_POINT_point2buf|, except it encodes the
// public key in |key|.
OPENSSL_EXPORT size_t EC_KEY_key2buf(const EC_KEY *key,
                                     point_conversion_form_t form,
                                     uint8_t **out_buf, BN_CTX *ctx);

// EC_KEY_oct2priv decodes a big-endian, zero-padded integer from |len| bytes
// from |in| and sets |key|'s private key to the result. It returns one on
// success and zero on error. The input must be padded to the size of |key|'s
// group order.
OPENSSL_EXPORT int EC_KEY_oct2priv(EC_KEY *key, const uint8_t *in, size_t len);

// EC_KEY_priv2oct serializes |key|'s private key as a big-endian integer,
// zero-padded to the size of |key|'s group order and writes the result to at
// most |max_out| bytes of |out|. It returns the number of bytes written on
// success and zero on error. If |out| is NULL, it returns the number of bytes
// needed without writing anything.
OPENSSL_EXPORT size_t EC_KEY_priv2oct(const EC_KEY *key, uint8_t *out,
                                      size_t max_out);

// EC_KEY_priv2buf behaves like |EC_KEY_priv2oct| but sets |*out_buf| to a
// newly-allocated buffer containing the result. It returns the size of the
// result on success and zero on error. The caller must release |*out_buf| with
// |OPENSSL_free| when done.
OPENSSL_EXPORT size_t EC_KEY_priv2buf(const EC_KEY *key, uint8_t **out_buf);


// Key generation.

// EC_KEY_generate_key generates a random, private key, calculates the
// corresponding public key and stores both in |key|. It returns one on success
// or zero otherwise.
OPENSSL_EXPORT int EC_KEY_generate_key(EC_KEY *key);

// EC_KEY_generate_key_fips behaves like |EC_KEY_generate_key| but performs
// additional checks for FIPS compliance. This function is applicable when
// generating keys for either signing/verification or key agreement because
// both types of consistency check (PCT) are performed.
OPENSSL_EXPORT int EC_KEY_generate_key_fips(EC_KEY *key);

// EC_KEY_derive_from_secret deterministically derives a private key for |group|
// from an input secret using HKDF-SHA256. It returns a newly-allocated |EC_KEY|
// on success or NULL on error. |secret| must not be used in any other
// algorithm. If using a base secret for multiple operations, derive separate
// values with a KDF such as HKDF first.
//
// Note this function implements an arbitrary derivation scheme, rather than any
// particular standard one. New protocols are recommended to use X25519 and
// Ed25519, which have standard byte import functions. See
// |X25519_public_from_private| and |ED25519_keypair_from_seed|.
OPENSSL_EXPORT EC_KEY *EC_KEY_derive_from_secret(const EC_GROUP *group,
                                                 const uint8_t *secret,
                                                 size_t secret_len);


// Serialisation.

// EC_KEY_parse_private_key parses a DER-encoded ECPrivateKey structure (RFC
// 5915) from |cbs| and advances |cbs|. It returns a newly-allocated |EC_KEY| or
// NULL on error. If |group| is non-null, the parameters field of the
// ECPrivateKey may be omitted (but must match |group| if present). Otherwise,
// the parameters field is required.
OPENSSL_EXPORT EC_KEY *EC_KEY_parse_private_key(CBS *cbs,
                                                const EC_GROUP *group);

// EC_KEY_marshal_private_key marshals |key| as a DER-encoded ECPrivateKey
// structure (RFC 5915) and appends the result to |cbb|. It returns one on
// success and zero on failure. |enc_flags| is a combination of |EC_PKEY_*|
// values and controls whether corresponding fields are omitted.
OPENSSL_EXPORT int EC_KEY_marshal_private_key(CBB *cbb, const EC_KEY *key,
                                              unsigned enc_flags);

// EC_KEY_parse_curve_name parses a DER-encoded OBJECT IDENTIFIER as a curve
// name from |cbs| and advances |cbs|. It returns the decoded |EC_GROUP| or NULL
// on error.
//
// This function returns a non-const pointer which may be passed to
// |EC_GROUP_free|. However, the resulting object is actually static and calling
// |EC_GROUP_free| is optional.
//
// TODO(davidben): Make this return a const pointer, if it does not break too
// many callers.
OPENSSL_EXPORT EC_GROUP *EC_KEY_parse_curve_name(CBS *cbs);

// EC_KEY_marshal_curve_name marshals |group| as a DER-encoded OBJECT IDENTIFIER
// and appends the result to |cbb|. It returns one on success and zero on
// failure.
OPENSSL_EXPORT int EC_KEY_marshal_curve_name(CBB *cbb, const EC_GROUP *group);

// EC_KEY_parse_parameters parses a DER-encoded ECParameters structure (RFC
// 5480) from |cbs| and advances |cbs|. It returns the resulting |EC_GROUP| or
// NULL on error. It supports the namedCurve and specifiedCurve options, but use
// of specifiedCurve is deprecated. Use |EC_KEY_parse_curve_name| instead.
//
// This function returns a non-const pointer which may be passed to
// |EC_GROUP_free|. However, the resulting object is actually static and calling
// |EC_GROUP_free| is optional.
//
// TODO(davidben): Make this return a const pointer, if it does not break too
// many callers.
OPENSSL_EXPORT EC_GROUP *EC_KEY_parse_parameters(CBS *cbs);


// ex_data functions.
//
// These functions are wrappers. See |ex_data.h| for details.

OPENSSL_EXPORT int EC_KEY_get_ex_new_index(long argl, void *argp,
                                           CRYPTO_EX_unused *unused,
                                           CRYPTO_EX_dup *dup_unused,
                                           CRYPTO_EX_free *free_func);
OPENSSL_EXPORT int EC_KEY_set_ex_data(EC_KEY *r, int idx, void *arg);
OPENSSL_EXPORT void *EC_KEY_get_ex_data(const EC_KEY *r, int idx);


// ECDSA method.

// ECDSA_FLAG_OPAQUE specifies that this ECDSA_METHOD does not expose its key
// material. This may be set if, for instance, it is wrapping some other crypto
// API, like a platform key store.
#define ECDSA_FLAG_OPAQUE 1

// ecdsa_method_st is a structure of function pointers for implementing ECDSA.
// See engine.h.
struct ecdsa_method_st {
  struct openssl_method_common_st common;

  void *app_data;

  int (*init)(EC_KEY *key);
  int (*finish)(EC_KEY *key);

  // sign matches the arguments and behaviour of |ECDSA_sign|.
  int (*sign)(const uint8_t *digest, size_t digest_len, uint8_t *sig,
              unsigned int *sig_len, EC_KEY *eckey);

  int flags;
};


// Deprecated functions.

// EC_KEY_set_asn1_flag does nothing.
OPENSSL_EXPORT void EC_KEY_set_asn1_flag(EC_KEY *key, int flag);

// d2i_ECPrivateKey parses a DER-encoded ECPrivateKey structure (RFC 5915) from
// |len| bytes at |*inp|, as described in |d2i_SAMPLE|. On input, if |*out_key|
// is non-NULL and has a group configured, the parameters field may be omitted
// but must match that group if present.
//
// Use |EC_KEY_parse_private_key| instead.
OPENSSL_EXPORT EC_KEY *d2i_ECPrivateKey(EC_KEY **out_key, const uint8_t **inp,
                                        long len);

// i2d_ECPrivateKey marshals |key| as a DER-encoded ECPrivateKey structure (RFC
// 5915), as described in |i2d_SAMPLE|.
//
// Use |EC_KEY_marshal_private_key| instead.
OPENSSL_EXPORT int i2d_ECPrivateKey(const EC_KEY *key, uint8_t **outp);

// d2i_ECPKParameters parses a DER-encoded ECParameters structure (RFC 5480)
// from |len| bytes at |*inp|, as described in |d2i_SAMPLE|. For legacy reasons,
// it recognizes the specifiedCurve form, but only for curves that are already
// supported as named curves.
//
// Use |EC_KEY_parse_parameters| or |EC_KEY_parse_curve_name| instead.
OPENSSL_EXPORT EC_GROUP *d2i_ECPKParameters(EC_GROUP **out, const uint8_t **inp,
                                            long len);

// i2d_ECPKParameters marshals |group| as a DER-encoded ECParameters structure
// (RFC 5480), as described in |i2d_SAMPLE|.
//
// Use |EC_KEY_marshal_curve_name| instead.
OPENSSL_EXPORT int i2d_ECPKParameters(const EC_GROUP *group, uint8_t **outp);

// d2i_ECParameters parses a DER-encoded ECParameters structure (RFC 5480) from
// |len| bytes at |*inp|, as described in |d2i_SAMPLE|. It returns the result as
// an |EC_KEY| with parameters, but no key, configured.
//
// Use |EC_KEY_parse_parameters| or |EC_KEY_parse_curve_name| instead.
OPENSSL_EXPORT EC_KEY *d2i_ECParameters(EC_KEY **out_key, const uint8_t **inp,
                                        long len);

// i2d_ECParameters marshals |key|'s parameters as a DER-encoded OBJECT
// IDENTIFIER, as described in |i2d_SAMPLE|.
//
// Use |EC_KEY_marshal_curve_name| instead.
OPENSSL_EXPORT int i2d_ECParameters(const EC_KEY *key, uint8_t **outp);

// o2i_ECPublicKey parses an EC point from |len| bytes at |*inp| into
// |*out_key|. Note that this differs from the d2i format in that |*out_key|
// must be non-NULL with a group set. On successful exit, |*inp| is advanced by
// |len| bytes. It returns |*out_key| or NULL on error.
//
// Use |EC_POINT_oct2point| instead.
OPENSSL_EXPORT EC_KEY *o2i_ECPublicKey(EC_KEY **out_key, const uint8_t **inp,
                                       long len);

// i2o_ECPublicKey marshals an EC point from |key|, as described in
// |i2d_SAMPLE|, except it returns zero on error instead of a negative value.
//
// Use |EC_POINT_point2cbb| instead.
OPENSSL_EXPORT int i2o_ECPublicKey(const EC_KEY *key, unsigned char **outp);


#if defined(__cplusplus)
}  // extern C

extern "C++" {

BSSL_NAMESPACE_BEGIN

BORINGSSL_MAKE_DELETER(EC_KEY, EC_KEY_free)
BORINGSSL_MAKE_UP_REF(EC_KEY, EC_KEY_up_ref)

BSSL_NAMESPACE_END

}  // extern C++

#endif

#endif  // OPENSSL_HEADER_EC_KEY_H
