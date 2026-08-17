// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_DIGEST_H
#define OPENSSL_HEADER_DIGEST_H

#include <openssl/base.h>   // IWYU pragma: export

#if defined(__cplusplus)
extern "C" {
#endif


// Digest functions.
//
// An EVP_MD abstracts the details of a specific hash function allowing code to
// deal with the concept of a "hash function" without needing to know exactly
// which hash function it is.


// Hash algorithms.
//
// The following functions return |EVP_MD| objects that implement the named hash
// function.

OPENSSL_EXPORT const EVP_MD *EVP_md4(void);
OPENSSL_EXPORT const EVP_MD *EVP_md5(void);
OPENSSL_EXPORT const EVP_MD *EVP_sha1(void);
OPENSSL_EXPORT const EVP_MD *EVP_sha224(void);
OPENSSL_EXPORT const EVP_MD *EVP_sha256(void);
OPENSSL_EXPORT const EVP_MD *EVP_sha384(void);
OPENSSL_EXPORT const EVP_MD *EVP_sha512(void);
OPENSSL_EXPORT const EVP_MD *EVP_sha512_256(void);
OPENSSL_EXPORT const EVP_MD *EVP_blake2b256(void);

// EVP_md5_sha1 is a TLS-specific |EVP_MD| which computes the concatenation of
// MD5 and SHA-1, as used in TLS 1.1 and below.
OPENSSL_EXPORT const EVP_MD *EVP_md5_sha1(void);

// EVP_get_digestbynid returns an |EVP_MD| for the given NID, or NULL if no
// such digest is known.
OPENSSL_EXPORT const EVP_MD *EVP_get_digestbynid(int nid);

// EVP_get_digestbyobj returns an |EVP_MD| for the given |ASN1_OBJECT|, or NULL
// if no such digest is known.
OPENSSL_EXPORT const EVP_MD *EVP_get_digestbyobj(const ASN1_OBJECT *obj);


// Digest contexts.
//
// An EVP_MD_CTX represents the state of a specific digest operation in
// progress.

// EVP_MD_CTX_init initialises an, already allocated, |EVP_MD_CTX|. This is the
// same as setting the structure to zero.
OPENSSL_EXPORT void EVP_MD_CTX_init(EVP_MD_CTX *ctx);

// EVP_MD_CTX_new allocates and initialises a fresh |EVP_MD_CTX| and returns
// it, or NULL on allocation failure. The caller must use |EVP_MD_CTX_free| to
// release the resulting object.
OPENSSL_EXPORT EVP_MD_CTX *EVP_MD_CTX_new(void);

// EVP_MD_CTX_cleanup frees any resources owned by |ctx| and resets it to a
// freshly initialised state. It does not free |ctx| itself. It returns one.
OPENSSL_EXPORT int EVP_MD_CTX_cleanup(EVP_MD_CTX *ctx);

// EVP_MD_CTX_cleanse zeros the digest state in |ctx| and then performs the
// actions of |EVP_MD_CTX_cleanup|. Note that some |EVP_MD_CTX| objects contain
// more than just a digest (e.g. those resulting from |EVP_DigestSignInit|) but
// this function does not zero out more than just the digest state even in that
// case.
OPENSSL_EXPORT void EVP_MD_CTX_cleanse(EVP_MD_CTX *ctx);

// EVP_MD_CTX_free calls |EVP_MD_CTX_cleanup| and then frees |ctx| itself.
OPENSSL_EXPORT void EVP_MD_CTX_free(EVP_MD_CTX *ctx);

// EVP_MD_CTX_copy_ex sets |out|, which must already be initialised, to be a
// copy of |in|. It returns one on success and zero on allocation failure.
OPENSSL_EXPORT int EVP_MD_CTX_copy_ex(EVP_MD_CTX *out, const EVP_MD_CTX *in);

// EVP_MD_CTX_move sets |out|, which must already be initialised, to the hash
// state in |in|. |in| is mutated and left in an empty state.
OPENSSL_EXPORT void EVP_MD_CTX_move(EVP_MD_CTX *out, EVP_MD_CTX *in);

// EVP_MD_CTX_reset calls |EVP_MD_CTX_cleanup| followed by |EVP_MD_CTX_init|. It
// returns one.
OPENSSL_EXPORT int EVP_MD_CTX_reset(EVP_MD_CTX *ctx);


// Digest operations.

// EVP_DigestInit_ex configures |ctx|, which must already have been
// initialised, for a fresh hashing operation using |type|. It returns one on
// success and zero on allocation failure.
OPENSSL_EXPORT int EVP_DigestInit_ex(EVP_MD_CTX *ctx, const EVP_MD *type,
                                     ENGINE *engine);

// EVP_DigestInit acts like |EVP_DigestInit_ex| except that |ctx| is
// initialised before use.
OPENSSL_EXPORT int EVP_DigestInit(EVP_MD_CTX *ctx, const EVP_MD *type);

// EVP_DigestUpdate hashes |len| bytes from |data| into the hashing operation
// in |ctx|. It returns one.
OPENSSL_EXPORT int EVP_DigestUpdate(EVP_MD_CTX *ctx, const void *data,
                                    size_t len);

// EVP_MAX_MD_SIZE is the largest digest size supported, in bytes.
// Functions that output a digest generally require the buffer have
// at least this much space.
#define EVP_MAX_MD_SIZE 64  // SHA-512 is the longest so far.

// EVP_MAX_MD_BLOCK_SIZE is the largest digest block size supported, in
// bytes.
#define EVP_MAX_MD_BLOCK_SIZE 128  // SHA-512 is the longest so far.

// EVP_DigestFinal_ex finishes the digest in |ctx| and writes the output to
// |md_out|. |EVP_MD_CTX_size| bytes are written, which is at most
// |EVP_MAX_MD_SIZE|. If |out_size| is not NULL then |*out_size| is set to the
// number of bytes written. It returns one. After this call, the hash cannot be
// updated or finished again until |EVP_DigestInit_ex| is called to start
// another hashing operation.
OPENSSL_EXPORT int EVP_DigestFinal_ex(EVP_MD_CTX *ctx, uint8_t *md_out,
                                      unsigned int *out_size);

// EVP_DigestFinal acts like |EVP_DigestFinal_ex| except that
// |EVP_MD_CTX_cleanup| is called on |ctx| before returning.
OPENSSL_EXPORT int EVP_DigestFinal(EVP_MD_CTX *ctx, uint8_t *md_out,
                                   unsigned int *out_size);

// EVP_Digest performs a complete hashing operation in one call. It hashes |len|
// bytes from |data| and writes the digest to |md_out|. |EVP_MD_CTX_size| bytes
// are written, which is at most |EVP_MAX_MD_SIZE|. If |out_size| is not NULL
// then |*out_size| is set to the number of bytes written. It returns one on
// success and zero otherwise.
OPENSSL_EXPORT int EVP_Digest(const void *data, size_t len, uint8_t *md_out,
                              unsigned int *md_out_size, const EVP_MD *type,
                              ENGINE *impl);


// Digest function accessors.
//
// These functions allow code to learn details about an abstract hash
// function.

// EVP_MD_type returns a NID identifying |md|. (For example, |NID_sha256|.)
OPENSSL_EXPORT int EVP_MD_type(const EVP_MD *md);

// EVP_MD_flags returns the flags for |md|, which is a set of |EVP_MD_FLAG_*|
// values, ORed together.
OPENSSL_EXPORT uint32_t EVP_MD_flags(const EVP_MD *md);

// EVP_MD_size returns the digest size of |md|, in bytes.
OPENSSL_EXPORT size_t EVP_MD_size(const EVP_MD *md);

// EVP_MD_block_size returns the native block-size of |md|, in bytes.
OPENSSL_EXPORT size_t EVP_MD_block_size(const EVP_MD *md);

// EVP_MD_FLAG_DIGALGID_ABSENT indicates that the parameter type in an X.509
// DigestAlgorithmIdentifier representing this digest function should be
// undefined rather than NULL.
#define EVP_MD_FLAG_DIGALGID_ABSENT 2

// EVP_MD_FLAG_XOF indicates that the digest is an extensible-output function
// (XOF). This flag is defined for compatibility and will never be set in any
// |EVP_MD| in BoringSSL.
#define EVP_MD_FLAG_XOF 4


// Digest operation accessors.

// EVP_MD_CTX_get0_md returns the underlying digest function, or NULL if one has
// not been set.
OPENSSL_EXPORT const EVP_MD *EVP_MD_CTX_get0_md(const EVP_MD_CTX *ctx);

// EVP_MD_CTX_md returns the underlying digest function, or NULL if one has not
// been set. (This is the same as |EVP_MD_CTX_get0_md| but OpenSSL has
// deprecated this spelling.)
OPENSSL_EXPORT const EVP_MD *EVP_MD_CTX_md(const EVP_MD_CTX *ctx);

// EVP_MD_CTX_size returns the digest size of |ctx|, in bytes. It
// will crash if a digest hasn't been set on |ctx|.
OPENSSL_EXPORT size_t EVP_MD_CTX_size(const EVP_MD_CTX *ctx);

// EVP_MD_CTX_block_size returns the block size of the digest function used by
// |ctx|, in bytes. It will crash if a digest hasn't been set on |ctx|.
OPENSSL_EXPORT size_t EVP_MD_CTX_block_size(const EVP_MD_CTX *ctx);

// EVP_MD_CTX_type returns a NID describing the digest function used by |ctx|.
// (For example, |NID_sha256|.) It will crash if a digest hasn't been set on
// |ctx|.
OPENSSL_EXPORT int EVP_MD_CTX_type(const EVP_MD_CTX *ctx);


// ASN.1 functions.
//
// These functions allow code to parse and serialize AlgorithmIdentifiers for
// hash functions.

// EVP_parse_digest_algorithm parses an AlgorithmIdentifier structure containing
// a hash function OID (for example, 2.16.840.1.101.3.4.2.1 is SHA-256) and
// advances |cbs|. The parameters field may either be omitted or a NULL. It
// returns the digest function or NULL on error.
OPENSSL_EXPORT const EVP_MD *EVP_parse_digest_algorithm(CBS *cbs);

// EVP_marshal_digest_algorithm marshals |md| as an AlgorithmIdentifier
// structure and appends the result to |cbb|. It returns one on success and zero
// on error. It sets the parameters field to NULL. Use
// |EVP_marshal_digest_algorithm_no_params| to omit the parameters instead.
//
// In general, the parameters should be omitted for digest algorithms, but the
// following specifications require a NULL parameter instead.
//
// - Hash algorithms and MGF-1 hash algorithms used in RSASSA-PSS and RSAES-OAEP
//   (see RFC 4055, Section 2.1)
// - The hash algorithm in the DigestInfo structure of RSASSA-PKCS1-v1_5 (see
//   RFC 8017, Appendix A.2.4)
//
// Some existing software also uses NULL parameters in other contexts. In
// practice, digest algorithms are encoded wildly inconsistently.
OPENSSL_EXPORT int EVP_marshal_digest_algorithm(CBB *cbb, const EVP_MD *md);

// EVP_marshal_digest_algorithm_no_params behaves like
// |EVP_marshal_digest_algorithm| but omits the parameters field.
OPENSSL_EXPORT int EVP_marshal_digest_algorithm_no_params(CBB *cbb,
                                                          const EVP_MD *md);


// Deprecated functions.

// EVP_MD_CTX_copy sets |out|, which must /not/ be initialised, to be a copy of
// |in|. It returns one on success and zero on error.
OPENSSL_EXPORT int EVP_MD_CTX_copy(EVP_MD_CTX *out, const EVP_MD_CTX *in);

// EVP_add_digest does nothing and returns one. It exists only for
// compatibility with OpenSSL.
OPENSSL_EXPORT int EVP_add_digest(const EVP_MD *digest);

// EVP_get_digestbyname returns an |EVP_MD| given a human readable name in
// |name|, or NULL if the name is unknown.
OPENSSL_EXPORT const EVP_MD *EVP_get_digestbyname(const char *);

// EVP_dss1 returns the value of EVP_sha1(). This was provided by OpenSSL to
// specifiy the original DSA signatures, which were fixed to use SHA-1. Note,
// however, that attempting to sign or verify DSA signatures with the EVP
// interface will always fail.
OPENSSL_EXPORT const EVP_MD *EVP_dss1(void);

// EVP_MD_CTX_create calls |EVP_MD_CTX_new|.
OPENSSL_EXPORT EVP_MD_CTX *EVP_MD_CTX_create(void);

// EVP_MD_CTX_destroy calls |EVP_MD_CTX_free|.
OPENSSL_EXPORT void EVP_MD_CTX_destroy(EVP_MD_CTX *ctx);

// EVP_DigestFinalXOF returns zero and adds an error to the error queue.
// BoringSSL does not support any XOF digests.
OPENSSL_EXPORT int EVP_DigestFinalXOF(EVP_MD_CTX *ctx, uint8_t *out,
                                      size_t len);

// EVP_MD_meth_get_flags calls |EVP_MD_flags|.
OPENSSL_EXPORT uint32_t EVP_MD_meth_get_flags(const EVP_MD *md);

// EVP_MD_CTX_set_flags does nothing.
OPENSSL_EXPORT void EVP_MD_CTX_set_flags(EVP_MD_CTX *ctx, int flags);

// EVP_MD_CTX_FLAG_NON_FIPS_ALLOW is meaningless. In OpenSSL it permits non-FIPS
// algorithms in FIPS mode. But BoringSSL FIPS mode doesn't prohibit algorithms
// (it's up the the caller to use the FIPS module in a fashion compliant with
// their needs). Thus this exists only to allow code to compile.
#define EVP_MD_CTX_FLAG_NON_FIPS_ALLOW 0

// EVP_MD_nid calls |EVP_MD_type|.
OPENSSL_EXPORT int EVP_MD_nid(const EVP_MD *md);


struct evp_md_pctx_ops;

struct env_md_ctx_st {
  // digest is the underlying digest function, or NULL if not set.
  const EVP_MD *digest;
  // md_data points to a block of memory that contains the hash-specific
  // context.
  void *md_data;

  // pctx is an opaque (at this layer) pointer to additional context that
  // EVP_PKEY functions may store in this object.
  EVP_PKEY_CTX *pctx;

  // pctx_ops, if not NULL, points to a vtable that contains functions to
  // manipulate |pctx|.
  const struct evp_md_pctx_ops *pctx_ops;
} /* EVP_MD_CTX */;


#if defined(__cplusplus)
}  // extern C

#if !defined(BORINGSSL_NO_CXX)
extern "C++" {

BSSL_NAMESPACE_BEGIN

BORINGSSL_MAKE_DELETER(EVP_MD_CTX, EVP_MD_CTX_free)

using ScopedEVP_MD_CTX =
    internal::StackAllocatedMovable<EVP_MD_CTX, int, EVP_MD_CTX_init,
                                    EVP_MD_CTX_cleanup, EVP_MD_CTX_move>;

BSSL_NAMESPACE_END

}  // extern C++
#endif

#endif

#define DIGEST_R_INPUT_NOT_INITIALIZED 100
#define DIGEST_R_DECODE_ERROR 101
#define DIGEST_R_UNKNOWN_HASH 102

#endif  // OPENSSL_HEADER_DIGEST_H
