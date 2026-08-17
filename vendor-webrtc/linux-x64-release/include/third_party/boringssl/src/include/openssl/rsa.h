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

#ifndef OPENSSL_HEADER_RSA_H
#define OPENSSL_HEADER_RSA_H

#include <openssl/base.h>   // IWYU pragma: export

#include <openssl/engine.h>
#include <openssl/ex_data.h>
#include <openssl/thread.h>

#if defined(__cplusplus)
extern "C" {
#endif


// rsa.h contains functions for handling encryption and signature using RSA.


// Allocation and destruction.
//
// An |RSA| object represents a public or private RSA key. A given object may be
// used concurrently on multiple threads by non-mutating functions, provided no
// other thread is concurrently calling a mutating function. Unless otherwise
// documented, functions which take a |const| pointer are non-mutating and
// functions which take a non-|const| pointer are mutating.

// RSA_new_public_key returns a new |RSA| object containing a public key with
// the specified parameters, or NULL on error or invalid input.
OPENSSL_EXPORT RSA *RSA_new_public_key(const BIGNUM *n, const BIGNUM *e);

// RSA_new_private_key returns a new |RSA| object containing a private key with
// the specified parameters, or NULL on error or invalid input. All parameters
// are mandatory and may not be NULL.
//
// This function creates standard RSA private keys with CRT parameters.
OPENSSL_EXPORT RSA *RSA_new_private_key(const BIGNUM *n, const BIGNUM *e,
                                        const BIGNUM *d, const BIGNUM *p,
                                        const BIGNUM *q, const BIGNUM *dmp1,
                                        const BIGNUM *dmq1, const BIGNUM *iqmp);

// RSA_new returns a new, empty |RSA| object or NULL on error. Prefer using
// |RSA_new_public_key| or |RSA_new_private_key| to import an RSA key.
OPENSSL_EXPORT RSA *RSA_new(void);

// RSA_new_method acts the same as |RSA_new| but takes an explicit |ENGINE|.
OPENSSL_EXPORT RSA *RSA_new_method(const ENGINE *engine);

// RSA_free decrements the reference count of |rsa| and frees it if the
// reference count drops to zero.
OPENSSL_EXPORT void RSA_free(RSA *rsa);

// RSA_up_ref increments the reference count of |rsa| and returns one. It does
// not mutate |rsa| for thread-safety purposes and may be used concurrently.
OPENSSL_EXPORT int RSA_up_ref(RSA *rsa);


// Properties.

// OPENSSL_RSA_MAX_MODULUS_BITS is the maximum supported RSA modulus, in bits.
//
// TODO(crbug.com/402677800): Reduce this to 8192.
#define OPENSSL_RSA_MAX_MODULUS_BITS 16384

// RSA_bits returns the size of |rsa|, in bits.
OPENSSL_EXPORT unsigned RSA_bits(const RSA *rsa);

// RSA_get0_n returns |rsa|'s public modulus.
OPENSSL_EXPORT const BIGNUM *RSA_get0_n(const RSA *rsa);

// RSA_get0_e returns |rsa|'s public exponent.
OPENSSL_EXPORT const BIGNUM *RSA_get0_e(const RSA *rsa);

// RSA_get0_d returns |rsa|'s private exponent. If |rsa| is a public key, this
// value will be NULL.
OPENSSL_EXPORT const BIGNUM *RSA_get0_d(const RSA *rsa);

// RSA_get0_p returns |rsa|'s first private prime factor. If |rsa| is a public
// key or lacks its prime factors, this value will be NULL.
OPENSSL_EXPORT const BIGNUM *RSA_get0_p(const RSA *rsa);

// RSA_get0_q returns |rsa|'s second private prime factor. If |rsa| is a public
// key or lacks its prime factors, this value will be NULL.
OPENSSL_EXPORT const BIGNUM *RSA_get0_q(const RSA *rsa);

// RSA_get0_dmp1 returns d (mod p-1) for |rsa|. If |rsa| is a public key or
// lacks CRT parameters, this value will be NULL.
OPENSSL_EXPORT const BIGNUM *RSA_get0_dmp1(const RSA *rsa);

// RSA_get0_dmq1 returns d (mod q-1) for |rsa|. If |rsa| is a public key or
// lacks CRT parameters, this value will be NULL.
OPENSSL_EXPORT const BIGNUM *RSA_get0_dmq1(const RSA *rsa);

// RSA_get0_iqmp returns q^-1 (mod p). If |rsa| is a public key or lacks CRT
// parameters, this value will be NULL.
OPENSSL_EXPORT const BIGNUM *RSA_get0_iqmp(const RSA *rsa);

// RSA_get0_key sets |*out_n|, |*out_e|, and |*out_d|, if non-NULL, to |rsa|'s
// modulus, public exponent, and private exponent, respectively. If |rsa| is a
// public key, the private exponent will be set to NULL.
OPENSSL_EXPORT void RSA_get0_key(const RSA *rsa, const BIGNUM **out_n,
                                 const BIGNUM **out_e, const BIGNUM **out_d);

// RSA_get0_factors sets |*out_p| and |*out_q|, if non-NULL, to |rsa|'s prime
// factors. If |rsa| is a public key, they will be set to NULL.
OPENSSL_EXPORT void RSA_get0_factors(const RSA *rsa, const BIGNUM **out_p,
                                     const BIGNUM **out_q);

// RSA_get0_crt_params sets |*out_dmp1|, |*out_dmq1|, and |*out_iqmp|, if
// non-NULL, to |rsa|'s CRT parameters. These are d (mod p-1), d (mod q-1) and
// q^-1 (mod p), respectively. If |rsa| is a public key, each parameter will be
// set to NULL.
OPENSSL_EXPORT void RSA_get0_crt_params(const RSA *rsa, const BIGNUM **out_dmp1,
                                        const BIGNUM **out_dmq1,
                                        const BIGNUM **out_iqmp);


// Setting individual properties.
//
// These functions allow setting individual properties of an |RSA| object. This
// is typically used with |RSA_new| to construct an RSA key field by field.
// Prefer instead to use |RSA_new_public_key| and |RSA_new_private_key|. These
// functions defer some initialization to the first use of an |RSA| object. This
// means invalid inputs may be caught late.
//
// TODO(crbug.com/boringssl/316): This deferred initialization also causes
// performance problems in multi-threaded applications. The preferred APIs
// currently have the same issues, but they will initialize eagerly in the
// future.

// RSA_set0_key sets |rsa|'s modulus, public exponent, and private exponent to
// |n|, |e|, and |d| respectively, if non-NULL. On success, it takes ownership
// of each argument and returns one. Otherwise, it returns zero.
//
// |d| may be NULL, but |n| and |e| must either be non-NULL or already
// configured on |rsa|.
//
// It is an error to call this function after |rsa| has been used for a
// cryptographic operation. Construct a new |RSA| object instead.
OPENSSL_EXPORT int RSA_set0_key(RSA *rsa, BIGNUM *n, BIGNUM *e, BIGNUM *d);

// RSA_set0_factors sets |rsa|'s prime factors to |p| and |q|, if non-NULL, and
// takes ownership of them. On success, it takes ownership of each argument and
// returns one. Otherwise, it returns zero.
//
// Each argument must either be non-NULL or already configured on |rsa|.
//
// It is an error to call this function after |rsa| has been used for a
// cryptographic operation. Construct a new |RSA| object instead.
OPENSSL_EXPORT int RSA_set0_factors(RSA *rsa, BIGNUM *p, BIGNUM *q);

// RSA_set0_crt_params sets |rsa|'s CRT parameters to |dmp1|, |dmq1|, and
// |iqmp|, if non-NULL, and takes ownership of them. On success, it takes
// ownership of its parameters and returns one. Otherwise, it returns zero.
//
// Each argument must either be non-NULL or already configured on |rsa|.
//
// It is an error to call this function after |rsa| has been used for a
// cryptographic operation. Construct a new |RSA| object instead.
OPENSSL_EXPORT int RSA_set0_crt_params(RSA *rsa, BIGNUM *dmp1, BIGNUM *dmq1,
                                       BIGNUM *iqmp);


// Key generation.

// RSA_generate_key_ex generates a new RSA key where the modulus has size
// |bits| and the public exponent is |e|. If unsure, |RSA_F4| is a good value
// for |e|. If |cb| is not NULL then it is called during the key generation
// process. In addition to the calls documented for |BN_generate_prime_ex|, it
// is called with event=2 when the n'th prime is rejected as unsuitable and
// with event=3 when a suitable value for |p| is found.
//
// It returns one on success or zero on error.
OPENSSL_EXPORT int RSA_generate_key_ex(RSA *rsa, int bits, const BIGNUM *e,
                                       BN_GENCB *cb);

// RSA_generate_key_fips behaves like |RSA_generate_key_ex| but performs
// additional checks for FIPS compliance. The public exponent is always 65537
// and |bits| must be either 2048 or 3072.
OPENSSL_EXPORT int RSA_generate_key_fips(RSA *rsa, int bits, BN_GENCB *cb);


// Encryption / Decryption
//
// These functions are considered non-mutating for thread-safety purposes and
// may be used concurrently.

// RSA_PKCS1_PADDING denotes PKCS#1 v1.5 padding. When used with encryption,
// this is RSAES-PKCS1-v1_5. When used with signing, this is RSASSA-PKCS1-v1_5.
//
// WARNING: The RSAES-PKCS1-v1_5 encryption scheme is vulnerable to a
// chosen-ciphertext attack. Decrypting attacker-supplied ciphertext with
// RSAES-PKCS1-v1_5 may give the attacker control over your private key. This
// does not impact the RSASSA-PKCS1-v1_5 signature scheme. See "Chosen
// Ciphertext Attacks Against Protocols Based on the RSA Encryption Standard
// PKCS #1", Daniel Bleichenbacher, Advances in Cryptology (Crypto '98).
#define RSA_PKCS1_PADDING 1

// RSA_NO_PADDING denotes a raw RSA operation.
#define RSA_NO_PADDING 3

// RSA_PKCS1_OAEP_PADDING denotes the RSAES-OAEP encryption scheme.
#define RSA_PKCS1_OAEP_PADDING 4

// RSA_PKCS1_PSS_PADDING denotes the RSASSA-PSS signature scheme. This value may
// not be passed into |RSA_sign_raw|, only |EVP_PKEY_CTX_set_rsa_padding|. See
// also |RSA_sign_pss_mgf1| and |RSA_verify_pss_mgf1|.
#define RSA_PKCS1_PSS_PADDING 6

// RSA_encrypt encrypts |in_len| bytes from |in| to the public key from |rsa|
// and writes, at most, |max_out| bytes of encrypted data to |out|. The
// |max_out| argument must be, at least, |RSA_size| in order to ensure success.
//
// It returns 1 on success or zero on error.
//
// The |padding| argument must be one of the |RSA_*_PADDING| values. If in
// doubt, use |RSA_PKCS1_OAEP_PADDING| for new protocols.
OPENSSL_EXPORT int RSA_encrypt(RSA *rsa, size_t *out_len, uint8_t *out,
                               size_t max_out, const uint8_t *in, size_t in_len,
                               int padding);

// RSA_decrypt decrypts |in_len| bytes from |in| with the private key from
// |rsa| and writes, at most, |max_out| bytes of plaintext to |out|. The
// |max_out| argument must be, at least, |RSA_size| in order to ensure success.
//
// It returns 1 on success or zero on error.
//
// The |padding| argument must be one of the |RSA_*_PADDING| values. If in
// doubt, use |RSA_PKCS1_OAEP_PADDING| for new protocols.
//
// WARNING: Passing |RSA_PKCS1_PADDING| into this function is deprecated and
// insecure. RSAES-PKCS1-v1_5 is vulnerable to a chosen-ciphertext attack.
// Decrypting attacker-supplied ciphertext with RSAES-PKCS1-v1_5 may give the
// attacker control over your private key. See "Chosen Ciphertext Attacks
// Against Protocols Based on the RSA Encryption Standard PKCS #1", Daniel
// Bleichenbacher, Advances in Cryptology (Crypto '98).
//
// In some limited cases, such as TLS RSA key exchange, it is possible to
// mitigate this flaw with custom, protocol-specific padding logic. This
// should be implemented with |RSA_NO_PADDING|, not |RSA_PKCS1_PADDING|.
OPENSSL_EXPORT int RSA_decrypt(RSA *rsa, size_t *out_len, uint8_t *out,
                               size_t max_out, const uint8_t *in, size_t in_len,
                               int padding);

// RSA_public_encrypt encrypts |flen| bytes from |from| to the public key in
// |rsa| and writes the encrypted data to |to|. The |to| buffer must have at
// least |RSA_size| bytes of space. It returns the number of bytes written, or
// -1 on error. The |padding| argument must be one of the |RSA_*_PADDING|
// values. If in doubt, use |RSA_PKCS1_OAEP_PADDING| for new protocols.
//
// WARNING: this function is dangerous because it breaks the usual return value
// convention. Use |RSA_encrypt| instead.
OPENSSL_EXPORT int RSA_public_encrypt(size_t flen, const uint8_t *from,
                                      uint8_t *to, RSA *rsa, int padding);

// RSA_private_decrypt decrypts |flen| bytes from |from| with the public key in
// |rsa| and writes the plaintext to |to|. The |to| buffer must have at least
// |RSA_size| bytes of space. It returns the number of bytes written, or -1 on
// error. The |padding| argument must be one of the |RSA_*_PADDING| values. If
// in doubt, use |RSA_PKCS1_OAEP_PADDING| for new protocols. Passing
// |RSA_PKCS1_PADDING| into this function is deprecated and insecure. See
// |RSA_decrypt|.
//
// WARNING: this function is dangerous because it breaks the usual return value
// convention. Use |RSA_decrypt| instead.
OPENSSL_EXPORT int RSA_private_decrypt(size_t flen, const uint8_t *from,
                                       uint8_t *to, RSA *rsa, int padding);


// Signing / Verification
//
// These functions are considered non-mutating for thread-safety purposes and
// may be used concurrently.

// RSA_sign signs |digest_len| bytes of digest from |digest| with |rsa| using
// RSASSA-PKCS1-v1_5. It writes, at most, |RSA_size(rsa)| bytes to |out|. On
// successful return, the actual number of bytes written is written to
// |*out_len|.
//
// The |hash_nid| argument identifies the hash function used to calculate
// |digest| and is embedded in the resulting signature. For example, it might be
// |NID_sha256|.
//
// It returns 1 on success and zero on error.
//
// WARNING: |digest| must be the result of hashing the data to be signed with
// |hash_nid|. Passing unhashed inputs will not result in a secure signature
// scheme.
OPENSSL_EXPORT int RSA_sign(int hash_nid, const uint8_t *digest,
                            size_t digest_len, uint8_t *out, unsigned *out_len,
                            RSA *rsa);

// RSA_sign_pss_mgf1 signs |digest_len| bytes from |digest| with the public key
// from |rsa| using RSASSA-PSS with MGF1 as the mask generation function. It
// writes, at most, |max_out| bytes of signature data to |out|. The |max_out|
// argument must be, at least, |RSA_size| in order to ensure success. It returns
// 1 on success or zero on error.
//
// The |md| and |mgf1_md| arguments identify the hash used to calculate |digest|
// and the MGF1 hash, respectively. If |mgf1_md| is NULL, |md| is
// used.
//
// |salt_len| specifies the expected salt length in bytes. If |salt_len| is -1,
// then the salt length is the same as the hash length. If -2, then the salt
// length is maximal given the size of |rsa|. If unsure, use -1.
//
// WARNING: |digest| must be the result of hashing the data to be signed with
// |md|. Passing unhashed inputs will not result in a secure signature scheme.
OPENSSL_EXPORT int RSA_sign_pss_mgf1(RSA *rsa, size_t *out_len, uint8_t *out,
                                     size_t max_out, const uint8_t *digest,
                                     size_t digest_len, const EVP_MD *md,
                                     const EVP_MD *mgf1_md, int salt_len);

// RSA_sign_raw performs the private key portion of computing a signature with
// |rsa|. It writes, at most, |max_out| bytes of signature data to |out|. The
// |max_out| argument must be, at least, |RSA_size| in order to ensure the
// output fits. It returns 1 on success or zero on error.
//
// If |padding| is |RSA_PKCS1_PADDING|, this function wraps |in| with the
// padding portion of RSASSA-PKCS1-v1_5 and then performs the raw private key
// operation. The caller is responsible for hashing the input and wrapping it in
// a DigestInfo structure.
//
// If |padding| is |RSA_NO_PADDING|, this function only performs the raw private
// key operation, interpreting |in| as a integer modulo n. The caller is
// responsible for hashing the input and encoding it for the signature scheme
// being implemented.
//
// WARNING: This function is a building block for a signature scheme, not a
// complete one. |in| must be the result of hashing and encoding the data as
// needed for the scheme being implemented. Passing in arbitrary inputs will not
// result in a secure signature scheme.
OPENSSL_EXPORT int RSA_sign_raw(RSA *rsa, size_t *out_len, uint8_t *out,
                                size_t max_out, const uint8_t *in,
                                size_t in_len, int padding);

// RSA_verify verifies that |sig_len| bytes from |sig| are a valid,
// RSASSA-PKCS1-v1_5 signature of |digest_len| bytes at |digest| by |rsa|.
//
// The |hash_nid| argument identifies the hash function used to calculate
// |digest| and is embedded in the resulting signature in order to prevent hash
// confusion attacks. For example, it might be |NID_sha256|.
//
// It returns one if the signature is valid and zero otherwise.
//
// WARNING: this differs from the original, OpenSSL function which additionally
// returned -1 on error.
//
// WARNING: |digest| must be the result of hashing the data to be verified with
// |hash_nid|. Passing unhashed input will not result in a secure signature
// scheme.
OPENSSL_EXPORT int RSA_verify(int hash_nid, const uint8_t *digest,
                              size_t digest_len, const uint8_t *sig,
                              size_t sig_len, RSA *rsa);

// RSA_verify_pss_mgf1 verifies that |sig_len| bytes from |sig| are a valid,
// RSASSA-PSS signature of |digest_len| bytes at |digest| by |rsa|. It returns
// one if the signature is valid and zero otherwise. MGF1 is used as the mask
// generation function.
//
// The |md| and |mgf1_md| arguments identify the hash used to calculate |digest|
// and the MGF1 hash, respectively. If |mgf1_md| is NULL, |md| is
// used. |salt_len| specifies the expected salt length in bytes.
//
// If |salt_len| is -1, then the salt length is the same as the hash length. If
// -2, then the salt length is recovered and all values accepted. If unsure, use
// -1.
//
// WARNING: |digest| must be the result of hashing the data to be verified with
// |md|. Passing unhashed input will not result in a secure signature scheme.
OPENSSL_EXPORT int RSA_verify_pss_mgf1(RSA *rsa, const uint8_t *digest,
                                       size_t digest_len, const EVP_MD *md,
                                       const EVP_MD *mgf1_md, int salt_len,
                                       const uint8_t *sig, size_t sig_len);

// RSA_verify_raw performs the public key portion of verifying |in_len| bytes of
// signature from |in| using the public key from |rsa|. On success, it returns
// one and writes, at most, |max_out| bytes of output to |out|. The |max_out|
// argument must be, at least, |RSA_size| in order to ensure the output fits. On
// failure or invalid input, it returns zero.
//
// If |padding| is |RSA_PKCS1_PADDING|, this function checks the padding portion
// of RSASSA-PKCS1-v1_5 and outputs the remainder of the encoded digest. The
// caller is responsible for checking the output is a DigestInfo-wrapped digest
// of the message.
//
// If |padding| is |RSA_NO_PADDING|, this function only performs the raw public
// key operation. The caller is responsible for checking the output is a valid
// result for the signature scheme being implemented.
//
// WARNING: This function is a building block for a signature scheme, not a
// complete one. Checking for arbitrary strings in |out| will not result in a
// secure signature scheme.
OPENSSL_EXPORT int RSA_verify_raw(RSA *rsa, size_t *out_len, uint8_t *out,
                                  size_t max_out, const uint8_t *in,
                                  size_t in_len, int padding);

// RSA_private_encrypt performs the private key portion of computing a signature
// with |rsa|. It takes |flen| bytes from |from| as input and writes the result
// to |to|. The |to| buffer must have at least |RSA_size| bytes of space. It
// returns the number of bytes written, or -1 on error.
//
// For the interpretation of |padding| and the input, see |RSA_sign_raw|.
//
// WARNING: This function is a building block for a signature scheme, not a
// complete one. See |RSA_sign_raw| for details.
//
// WARNING: This function is dangerous because it breaks the usual return value
// convention. Use |RSA_sign_raw| instead.
OPENSSL_EXPORT int RSA_private_encrypt(size_t flen, const uint8_t *from,
                                       uint8_t *to, RSA *rsa, int padding);

// RSA_public_decrypt performs the public key portion of verifying |flen| bytes
// of signature from |from| using the public key from |rsa|. It writes the
// result to |to|, which must have at least |RSA_size| bytes of space. It
// returns the number of bytes written, or -1 on error.
//
// For the interpretation of |padding| and the result, see |RSA_verify_raw|.
//
// WARNING: This function is a building block for a signature scheme, not a
// complete one. See |RSA_verify_raw| for details.
//
// WARNING: This function is dangerous because it breaks the usual return value
// convention. Use |RSA_verify_raw| instead.
OPENSSL_EXPORT int RSA_public_decrypt(size_t flen, const uint8_t *from,
                                      uint8_t *to, RSA *rsa, int padding);


// Utility functions.

// RSA_size returns the number of bytes in the modulus, which is also the size
// of a signature or encrypted value using |rsa|.
OPENSSL_EXPORT unsigned RSA_size(const RSA *rsa);

// RSA_is_opaque returns one if |rsa| is opaque and doesn't expose its key
// material. Otherwise it returns zero.
OPENSSL_EXPORT int RSA_is_opaque(const RSA *rsa);

// RSAPublicKey_dup allocates a fresh |RSA| and copies the public key from
// |rsa| into it. It returns the fresh |RSA| object, or NULL on error.
OPENSSL_EXPORT RSA *RSAPublicKey_dup(const RSA *rsa);

// RSAPrivateKey_dup allocates a fresh |RSA| and copies the private key from
// |rsa| into it. It returns the fresh |RSA| object, or NULL on error.
OPENSSL_EXPORT RSA *RSAPrivateKey_dup(const RSA *rsa);

// RSA_check_key performs basic validity tests on |rsa|. It returns one if
// they pass and zero otherwise. Opaque keys and public keys always pass. If it
// returns zero then a more detailed error is available on the error queue.
OPENSSL_EXPORT int RSA_check_key(const RSA *rsa);

// RSA_check_fips performs public key validity tests on |key|. It returns one if
// they pass and zero otherwise. Opaque keys always fail. This function does not
// mutate |rsa| for thread-safety purposes and may be used concurrently.
OPENSSL_EXPORT int RSA_check_fips(RSA *key);

// RSA_verify_PKCS1_PSS_mgf1 verifies that |EM| is a correct PSS padding of
// |mHash|, where |mHash| is a digest produced by |Hash|. |EM| must point to
// exactly |RSA_size(rsa)| bytes of data. The |mgf1Hash| argument specifies the
// hash function for generating the mask. If NULL, |Hash| is used. The |sLen|
// argument specifies the expected salt length in bytes. If |sLen| is -1 then
// the salt length is the same as the hash length. If -2, then the salt length
// is recovered and all values accepted.
//
// If unsure, use -1.
//
// It returns one on success or zero on error.
//
// This function implements only the low-level padding logic. Use
// |RSA_verify_pss_mgf1| instead.
OPENSSL_EXPORT int RSA_verify_PKCS1_PSS_mgf1(const RSA *rsa,
                                             const uint8_t *mHash,
                                             const EVP_MD *Hash,
                                             const EVP_MD *mgf1Hash,
                                             const uint8_t *EM, int sLen);

// RSA_padding_add_PKCS1_PSS_mgf1 writes a PSS padding of |mHash| to |EM|,
// where |mHash| is a digest produced by |Hash|. |RSA_size(rsa)| bytes of
// output will be written to |EM|. The |mgf1Hash| argument specifies the hash
// function for generating the mask. If NULL, |Hash| is used. The |sLen|
// argument specifies the expected salt length in bytes. If |sLen| is -1 then
// the salt length is the same as the hash length. If -2, then the salt length
// is maximal given the space in |EM|.
//
// It returns one on success or zero on error.
//
// This function implements only the low-level padding logic. Use
// |RSA_sign_pss_mgf1| instead.
OPENSSL_EXPORT int RSA_padding_add_PKCS1_PSS_mgf1(const RSA *rsa, uint8_t *EM,
                                                  const uint8_t *mHash,
                                                  const EVP_MD *Hash,
                                                  const EVP_MD *mgf1Hash,
                                                  int sLen);

// RSA_padding_add_PKCS1_OAEP_mgf1 writes an OAEP padding of |from| to |to|
// with the given parameters and hash functions. If |md| is NULL then SHA-1 is
// used. If |mgf1md| is NULL then the value of |md| is used (which means SHA-1
// if that, in turn, is NULL).
//
// It returns one on success or zero on error.
OPENSSL_EXPORT int RSA_padding_add_PKCS1_OAEP_mgf1(
    uint8_t *to, size_t to_len, const uint8_t *from, size_t from_len,
    const uint8_t *param, size_t param_len, const EVP_MD *md,
    const EVP_MD *mgf1md);

// RSA_add_pkcs1_prefix builds a version of |digest| prefixed with the
// DigestInfo header for the given hash function and sets |out_msg| to point to
// it. On successful return, if |*is_alloced| is one, the caller must release
// |*out_msg| with |OPENSSL_free|.
OPENSSL_EXPORT int RSA_add_pkcs1_prefix(uint8_t **out_msg, size_t *out_msg_len,
                                        int *is_alloced, int hash_nid,
                                        const uint8_t *digest,
                                        size_t digest_len);


// ASN.1 functions.

// RSA_parse_public_key parses a DER-encoded RSAPublicKey structure (RFC 8017)
// from |cbs| and advances |cbs|. It returns a newly-allocated |RSA| or NULL on
// error.
OPENSSL_EXPORT RSA *RSA_parse_public_key(CBS *cbs);

// RSA_public_key_from_bytes parses |in| as a DER-encoded RSAPublicKey structure
// (RFC 8017). It returns a newly-allocated |RSA| or NULL on error.
OPENSSL_EXPORT RSA *RSA_public_key_from_bytes(const uint8_t *in, size_t in_len);

// RSA_marshal_public_key marshals |rsa| as a DER-encoded RSAPublicKey structure
// (RFC 8017) and appends the result to |cbb|. It returns one on success and
// zero on failure.
OPENSSL_EXPORT int RSA_marshal_public_key(CBB *cbb, const RSA *rsa);

// RSA_public_key_to_bytes marshals |rsa| as a DER-encoded RSAPublicKey
// structure (RFC 8017) and, on success, sets |*out_bytes| to a newly allocated
// buffer containing the result and returns one. Otherwise, it returns zero. The
// result should be freed with |OPENSSL_free|.
OPENSSL_EXPORT int RSA_public_key_to_bytes(uint8_t **out_bytes, size_t *out_len,
                                           const RSA *rsa);

// RSA_parse_private_key parses a DER-encoded RSAPrivateKey structure (RFC 8017)
// from |cbs| and advances |cbs|. It returns a newly-allocated |RSA| or NULL on
// error.
OPENSSL_EXPORT RSA *RSA_parse_private_key(CBS *cbs);

// RSA_private_key_from_bytes parses |in| as a DER-encoded RSAPrivateKey
// structure (RFC 8017). It returns a newly-allocated |RSA| or NULL on error.
OPENSSL_EXPORT RSA *RSA_private_key_from_bytes(const uint8_t *in,
                                               size_t in_len);

// RSA_marshal_private_key marshals |rsa| as a DER-encoded RSAPrivateKey
// structure (RFC 8017) and appends the result to |cbb|. It returns one on
// success and zero on failure.
OPENSSL_EXPORT int RSA_marshal_private_key(CBB *cbb, const RSA *rsa);

// RSA_private_key_to_bytes marshals |rsa| as a DER-encoded RSAPrivateKey
// structure (RFC 8017) and, on success, sets |*out_bytes| to a newly allocated
// buffer containing the result and returns one. Otherwise, it returns zero. The
// result should be freed with |OPENSSL_free|.
OPENSSL_EXPORT int RSA_private_key_to_bytes(uint8_t **out_bytes,
                                            size_t *out_len, const RSA *rsa);


// Obscure RSA variants.
//
// These functions allow creating RSA keys with obscure combinations of
// parameters.

// RSA_new_private_key_no_crt behaves like |RSA_new_private_key| but constructs
// an RSA key without CRT coefficients.
//
// Keys created by this function will be less performant and cannot be
// serialized.
OPENSSL_EXPORT RSA *RSA_new_private_key_no_crt(const BIGNUM *n, const BIGNUM *e,
                                               const BIGNUM *d);

// RSA_new_private_key_no_e behaves like |RSA_new_private_key| but constructs an
// RSA key without CRT parameters or public exponent.
//
// Keys created by this function will be less performant, cannot be serialized,
// and lack hardening measures that protect against side channels and fault
// attacks.
OPENSSL_EXPORT RSA *RSA_new_private_key_no_e(const BIGNUM *n, const BIGNUM *d);

// RSA_new_public_key_large_e behaves like |RSA_new_public_key| but allows any
// |e| up to |n|.
//
// BoringSSL typically bounds public exponents as a denial-of-service
// mitigation. Keys created by this function may perform worse than those
// created by |RSA_new_public_key|.
OPENSSL_EXPORT RSA *RSA_new_public_key_large_e(const BIGNUM *n,
                                               const BIGNUM *e);

// RSA_new_private_key_large_e behaves like |RSA_new_private_key| but allows any
// |e| up to |n|.
//
// BoringSSL typically bounds public exponents as a denial-of-service
// mitigation. Keys created by this function may perform worse than those
// created by |RSA_new_private_key|.
OPENSSL_EXPORT RSA *RSA_new_private_key_large_e(
    const BIGNUM *n, const BIGNUM *e, const BIGNUM *d, const BIGNUM *p,
    const BIGNUM *q, const BIGNUM *dmp1, const BIGNUM *dmq1,
    const BIGNUM *iqmp);


// ex_data functions.
//
// See |ex_data.h| for details.

OPENSSL_EXPORT int RSA_get_ex_new_index(long argl, void *argp,
                                        CRYPTO_EX_unused *unused,
                                        CRYPTO_EX_dup *dup_unused,
                                        CRYPTO_EX_free *free_func);
OPENSSL_EXPORT int RSA_set_ex_data(RSA *rsa, int idx, void *arg);
OPENSSL_EXPORT void *RSA_get_ex_data(const RSA *rsa, int idx);


// Flags.

// RSA_FLAG_OPAQUE specifies that this RSA_METHOD does not expose its key
// material. This may be set if, for instance, it is wrapping some other crypto
// API, like a platform key store.
#define RSA_FLAG_OPAQUE 1

// RSA_FLAG_NO_BLINDING disables blinding of private operations, which is a
// dangerous thing to do. This flag is set internally as part of self-tests but
// is otherwise impossible to set externally.
#define RSA_FLAG_NO_BLINDING 8

// RSA_FLAG_EXT_PKEY is deprecated and ignored.
#define RSA_FLAG_EXT_PKEY 0x20

// RSA_FLAG_NO_PUBLIC_EXPONENT indicates that private keys without a public
// exponent are allowed. This is an internal constant. Use
// |RSA_new_private_key_no_e| to construct such keys.
#define RSA_FLAG_NO_PUBLIC_EXPONENT 0x40

// RSA_FLAG_LARGE_PUBLIC_EXPONENT indicates that keys with a large public
// exponent are allowed. This is an internal constant. Use
// |RSA_new_public_key_large_e| and |RSA_new_private_key_large_e| to construct
// such keys.
#define RSA_FLAG_LARGE_PUBLIC_EXPONENT 0x80


// RSA public exponent values.

#define RSA_3 0x3
#define RSA_F4 0x10001


// Deprecated functions.

#define RSA_METHOD_FLAG_NO_CHECK RSA_FLAG_OPAQUE

// RSA_flags returns the flags for |rsa|. These are a bitwise OR of |RSA_FLAG_*|
// constants.
OPENSSL_EXPORT int RSA_flags(const RSA *rsa);

// RSA_test_flags returns the subset of flags in |flags| which are set in |rsa|.
OPENSSL_EXPORT int RSA_test_flags(const RSA *rsa, int flags);

// RSA_blinding_on returns one.
OPENSSL_EXPORT int RSA_blinding_on(RSA *rsa, BN_CTX *ctx);

// RSA_blinding_off does nothing.
OPENSSL_EXPORT void RSA_blinding_off(RSA *rsa);

// RSA_generate_key behaves like |RSA_generate_key_ex|, which is what you
// should use instead. It returns NULL on error, or a newly-allocated |RSA| on
// success. This function is provided for compatibility only. The |callback|
// and |cb_arg| parameters must be NULL.
OPENSSL_EXPORT RSA *RSA_generate_key(int bits, uint64_t e, void *callback,
                                     void *cb_arg);

// d2i_RSAPublicKey parses a DER-encoded RSAPublicKey structure (RFC 8017) from
// |len| bytes at |*inp|, as described in |d2i_SAMPLE|.
//
// Use |RSA_parse_public_key| instead.
OPENSSL_EXPORT RSA *d2i_RSAPublicKey(RSA **out, const uint8_t **inp, long len);

// i2d_RSAPublicKey marshals |in| to a DER-encoded RSAPublicKey structure (RFC
// 8017), as described in |i2d_SAMPLE|.
//
// Use |RSA_marshal_public_key| instead.
OPENSSL_EXPORT int i2d_RSAPublicKey(const RSA *in, uint8_t **outp);

// d2i_RSAPrivateKey parses a DER-encoded RSAPrivateKey structure (RFC 8017)
// from |len| bytes at |*inp|, as described in |d2i_SAMPLE|.
//
// Use |RSA_parse_private_key| instead.
OPENSSL_EXPORT RSA *d2i_RSAPrivateKey(RSA **out, const uint8_t **inp, long len);

// i2d_RSAPrivateKey marshals |in| to a DER-encoded RSAPrivateKey structure (RFC
// 8017), as described in |i2d_SAMPLE|.
//
// Use |RSA_marshal_private_key| instead.
OPENSSL_EXPORT int i2d_RSAPrivateKey(const RSA *in, uint8_t **outp);

// RSA_padding_add_PKCS1_PSS acts like |RSA_padding_add_PKCS1_PSS_mgf1| but the
// |mgf1Hash| parameter of the latter is implicitly set to |Hash|.
//
// This function implements only the low-level padding logic. Use
// |RSA_sign_pss_mgf1| instead.
OPENSSL_EXPORT int RSA_padding_add_PKCS1_PSS(const RSA *rsa, uint8_t *EM,
                                             const uint8_t *mHash,
                                             const EVP_MD *Hash, int sLen);

// RSA_verify_PKCS1_PSS acts like |RSA_verify_PKCS1_PSS_mgf1| but the
// |mgf1Hash| parameter of the latter is implicitly set to |Hash|.
//
// This function implements only the low-level padding logic. Use
// |RSA_verify_pss_mgf1| instead.
OPENSSL_EXPORT int RSA_verify_PKCS1_PSS(const RSA *rsa, const uint8_t *mHash,
                                        const EVP_MD *Hash, const uint8_t *EM,
                                        int sLen);

// RSA_padding_add_PKCS1_OAEP acts like |RSA_padding_add_PKCS1_OAEP_mgf1| but
// the |md| and |mgf1md| parameters of the latter are implicitly set to NULL,
// which means SHA-1.
OPENSSL_EXPORT int RSA_padding_add_PKCS1_OAEP(uint8_t *to, size_t to_len,
                                              const uint8_t *from,
                                              size_t from_len,
                                              const uint8_t *param,
                                              size_t param_len);

// RSA_print prints a textual representation of |rsa| to |bio|. It returns one
// on success or zero otherwise.
OPENSSL_EXPORT int RSA_print(BIO *bio, const RSA *rsa, int indent);

// RSA_get0_pss_params returns NULL. In OpenSSL, this function retries RSA-PSS
// parameters associated with |RSA| objects, but BoringSSL does not support
// the id-RSASSA-PSS key encoding.
OPENSSL_EXPORT const RSA_PSS_PARAMS *RSA_get0_pss_params(const RSA *rsa);

// RSA_new_method_no_e returns a newly-allocated |RSA| object backed by
// |engine|, with a public modulus of |n| and no known public exponent.
//
// Do not use this function. It exists only to support Conscrypt, whose use
// should be replaced with a more sound mechanism. See
// https://crbug.com/boringssl/602.
OPENSSL_EXPORT RSA *RSA_new_method_no_e(const ENGINE *engine, const BIGNUM *n);


struct rsa_meth_st {
  struct openssl_method_common_st common;

  void *app_data;

  int (*init)(RSA *rsa);
  int (*finish)(RSA *rsa);

  int (*sign)(int type, const uint8_t *m, unsigned int m_length,
              uint8_t *sigret, unsigned int *siglen, const RSA *rsa);

  // These functions mirror the |RSA_*| functions of the same name.
  int (*sign_raw)(RSA *rsa, size_t *out_len, uint8_t *out, size_t max_out,
                  const uint8_t *in, size_t in_len, int padding);
  int (*decrypt)(RSA *rsa, size_t *out_len, uint8_t *out, size_t max_out,
                 const uint8_t *in, size_t in_len, int padding);

  // private_transform takes a big-endian integer from |in|, calculates the
  // d'th power of it, modulo the RSA modulus and writes the result as a
  // big-endian integer to |out|. Both |in| and |out| are |len| bytes long and
  // |len| is always equal to |RSA_size(rsa)|. If the result of the transform
  // can be represented in fewer than |len| bytes, then |out| must be zero
  // padded on the left.
  //
  // It returns one on success and zero otherwise.
  //
  // RSA decrypt and sign operations will call this, thus an ENGINE might wish
  // to override it in order to avoid having to implement the padding
  // functionality demanded by those, higher level, operations.
  int (*private_transform)(RSA *rsa, uint8_t *out, const uint8_t *in,
                           size_t len);

  int flags;
};


#if defined(__cplusplus)
}  // extern C

extern "C++" {

BSSL_NAMESPACE_BEGIN

BORINGSSL_MAKE_DELETER(RSA, RSA_free)
BORINGSSL_MAKE_UP_REF(RSA, RSA_up_ref)

BSSL_NAMESPACE_END

}  // extern C++

#endif

#define RSA_R_BAD_ENCODING 100
#define RSA_R_BAD_E_VALUE 101
#define RSA_R_BAD_FIXED_HEADER_DECRYPT 102
#define RSA_R_BAD_PAD_BYTE_COUNT 103
#define RSA_R_BAD_RSA_PARAMETERS 104
#define RSA_R_BAD_SIGNATURE 105
#define RSA_R_BAD_VERSION 106
#define RSA_R_BLOCK_TYPE_IS_NOT_01 107
#define RSA_R_BN_NOT_INITIALIZED 108
#define RSA_R_CANNOT_RECOVER_MULTI_PRIME_KEY 109
#define RSA_R_CRT_PARAMS_ALREADY_GIVEN 110
#define RSA_R_CRT_VALUES_INCORRECT 111
#define RSA_R_DATA_LEN_NOT_EQUAL_TO_MOD_LEN 112
#define RSA_R_DATA_TOO_LARGE 113
#define RSA_R_DATA_TOO_LARGE_FOR_KEY_SIZE 114
#define RSA_R_DATA_TOO_LARGE_FOR_MODULUS 115
#define RSA_R_DATA_TOO_SMALL 116
#define RSA_R_DATA_TOO_SMALL_FOR_KEY_SIZE 117
#define RSA_R_DIGEST_TOO_BIG_FOR_RSA_KEY 118
#define RSA_R_D_E_NOT_CONGRUENT_TO_1 119
#define RSA_R_EMPTY_PUBLIC_KEY 120
#define RSA_R_ENCODE_ERROR 121
#define RSA_R_FIRST_OCTET_INVALID 122
#define RSA_R_INCONSISTENT_SET_OF_CRT_VALUES 123
#define RSA_R_INTERNAL_ERROR 124
#define RSA_R_INVALID_MESSAGE_LENGTH 125
#define RSA_R_KEY_SIZE_TOO_SMALL 126
#define RSA_R_LAST_OCTET_INVALID 127
#define RSA_R_MODULUS_TOO_LARGE 128
#define RSA_R_MUST_HAVE_AT_LEAST_TWO_PRIMES 129
#define RSA_R_NO_PUBLIC_EXPONENT 130
#define RSA_R_NULL_BEFORE_BLOCK_MISSING 131
#define RSA_R_N_NOT_EQUAL_P_Q 132
#define RSA_R_OAEP_DECODING_ERROR 133
#define RSA_R_ONLY_ONE_OF_P_Q_GIVEN 134
#define RSA_R_OUTPUT_BUFFER_TOO_SMALL 135
#define RSA_R_PADDING_CHECK_FAILED 136
#define RSA_R_PKCS_DECODING_ERROR 137
#define RSA_R_SLEN_CHECK_FAILED 138
#define RSA_R_SLEN_RECOVERY_FAILED 139
#define RSA_R_TOO_LONG 140
#define RSA_R_TOO_MANY_ITERATIONS 141
#define RSA_R_UNKNOWN_ALGORITHM_TYPE 142
#define RSA_R_UNKNOWN_PADDING_TYPE 143
#define RSA_R_VALUE_MISSING 144
#define RSA_R_WRONG_SIGNATURE_LENGTH 145
#define RSA_R_PUBLIC_KEY_VALIDATION_FAILED 146
#define RSA_R_D_OUT_OF_RANGE 147
#define RSA_R_BLOCK_TYPE_IS_NOT_02 148

#endif  // OPENSSL_HEADER_RSA_H
