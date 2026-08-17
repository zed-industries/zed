/* rsa.h

   The RSA publickey algorithm.

   Copyright (C) 2001, 2002 Niels Möller

   This file is part of GNU Nettle.

   GNU Nettle is free software: you can redistribute it and/or
   modify it under the terms of either:

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at your
       option) any later version.

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at your
       option) any later version.

   or both in parallel, as here.

   GNU Nettle is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see http://www.gnu.org/licenses/.
*/
 
#ifndef NETTLE_RSA_H_INCLUDED
#define NETTLE_RSA_H_INCLUDED

#include "nettle-types.h"
#include "bignum.h"

#include "md5.h"
#include "sha1.h"
#include "sha2.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define rsa_public_key_init nettle_rsa_public_key_init
#define rsa_public_key_clear nettle_rsa_public_key_clear
#define rsa_public_key_prepare nettle_rsa_public_key_prepare
#define rsa_private_key_init nettle_rsa_private_key_init
#define rsa_private_key_clear nettle_rsa_private_key_clear
#define rsa_private_key_prepare nettle_rsa_private_key_prepare
#define rsa_pkcs1_verify nettle_rsa_pkcs1_verify
#define rsa_pkcs1_sign nettle_rsa_pkcs1_sign
#define rsa_pkcs1_sign_tr nettle_rsa_pkcs1_sign_tr
#define rsa_md5_sign nettle_rsa_md5_sign
#define rsa_md5_sign_tr nettle_rsa_md5_sign_tr
#define rsa_md5_verify nettle_rsa_md5_verify
#define rsa_sha1_sign nettle_rsa_sha1_sign
#define rsa_sha1_sign_tr nettle_rsa_sha1_sign_tr
#define rsa_sha1_verify nettle_rsa_sha1_verify
#define rsa_sha256_sign nettle_rsa_sha256_sign
#define rsa_sha256_sign_tr nettle_rsa_sha256_sign_tr
#define rsa_sha256_verify nettle_rsa_sha256_verify
#define rsa_sha512_sign nettle_rsa_sha512_sign
#define rsa_sha512_sign_tr nettle_rsa_sha512_sign_tr
#define rsa_sha512_verify nettle_rsa_sha512_verify
#define rsa_md5_sign_digest nettle_rsa_md5_sign_digest
#define rsa_md5_sign_digest_tr nettle_rsa_md5_sign_digest_tr
#define rsa_md5_verify_digest nettle_rsa_md5_verify_digest
#define rsa_sha1_sign_digest nettle_rsa_sha1_sign_digest
#define rsa_sha1_sign_digest_tr nettle_rsa_sha1_sign_digest_tr
#define rsa_sha1_verify_digest nettle_rsa_sha1_verify_digest
#define rsa_sha256_sign_digest nettle_rsa_sha256_sign_digest
#define rsa_sha256_sign_digest_tr nettle_rsa_sha256_sign_digest_tr
#define rsa_sha256_verify_digest nettle_rsa_sha256_verify_digest
#define rsa_sha512_sign_digest nettle_rsa_sha512_sign_digest
#define rsa_sha512_sign_digest_tr nettle_rsa_sha512_sign_digest_tr
#define rsa_sha512_verify_digest nettle_rsa_sha512_verify_digest
#define rsa_pss_sha256_sign_digest_tr nettle_rsa_pss_sha256_sign_digest_tr
#define rsa_pss_sha256_verify_digest nettle_rsa_pss_sha256_verify_digest
#define rsa_pss_sha384_sign_digest_tr nettle_rsa_pss_sha384_sign_digest_tr
#define rsa_pss_sha384_verify_digest nettle_rsa_pss_sha384_verify_digest
#define rsa_pss_sha512_sign_digest_tr nettle_rsa_pss_sha512_sign_digest_tr
#define rsa_pss_sha512_verify_digest nettle_rsa_pss_sha512_verify_digest
#define rsa_encrypt nettle_rsa_encrypt
#define rsa_decrypt nettle_rsa_decrypt
#define rsa_decrypt_tr nettle_rsa_decrypt_tr
#define rsa_sec_decrypt nettle_rsa_sec_decrypt
#define rsa_compute_root nettle_rsa_compute_root
#define rsa_compute_root_tr nettle_rsa_compute_root_tr
#define rsa_generate_keypair nettle_rsa_generate_keypair
#define rsa_keypair_to_sexp nettle_rsa_keypair_to_sexp
#define rsa_keypair_from_sexp_alist nettle_rsa_keypair_from_sexp_alist
#define rsa_keypair_from_sexp nettle_rsa_keypair_from_sexp
#define rsa_public_key_from_der_iterator nettle_rsa_public_key_from_der_iterator
#define rsa_private_key_from_der_iterator nettle_rsa_private_key_from_der_iterator
#define rsa_keypair_from_der nettle_rsa_keypair_from_der
#define rsa_keypair_to_openpgp nettle_rsa_keypair_to_openpgp

/* This limit is somewhat arbitrary. Technically, the smallest modulo
   which makes sense at all is 15 = 3*5, phi(15) = 8, size 4 bits. But
   for ridiculously small keys, not all odd e are possible (e.g., for
   5 bits, the only possible modulo is 3*7 = 21, phi(21) = 12, and e =
   3 don't work). The smallest size that makes sense with pkcs#1, and
   which allows RSA encryption of one byte messages, is 12 octets, 89
   bits. */

#define RSA_MINIMUM_N_OCTETS 12
#define RSA_MINIMUM_N_BITS (8*RSA_MINIMUM_N_OCTETS - 7)

struct rsa_public_key
{
  /* Size of the modulo, in octets. This is also the size of all
   * signatures that are created or verified with this key. */
  size_t size;
  
  /* Modulo */
  mpz_t n;

  /* Public exponent */
  mpz_t e;
};

struct rsa_private_key
{
  size_t size;

  /* d is filled in by the key generation function; otherwise it's
   * completely unused. */
  mpz_t d;
  
  /* The two factors */
  mpz_t p; mpz_t q;

  /* d % (p-1), i.e. a e = 1 (mod (p-1)) */
  mpz_t a;

  /* d % (q-1), i.e. b e = 1 (mod (q-1)) */
  mpz_t b;

  /* modular inverse of q , i.e. c q = 1 (mod p) */
  mpz_t c;
};

/* Signing a message works as follows:
 *
 * Store the private key in a rsa_private_key struct.
 *
 * Call rsa_private_key_prepare. This initializes the size attribute
 * to the length of a signature.
 *
 * Initialize a hashing context, by callling
 *   md5_init
 *
 * Hash the message by calling
 *   md5_update
 *
 * Create the signature by calling
 *   rsa_md5_sign
 *
 * The signature is represented as a mpz_t bignum. This call also
 * resets the hashing context.
 *
 * When done with the key and signature, don't forget to call
 * mpz_clear.
 */
 
/* Calls mpz_init to initialize bignum storage. */
void
rsa_public_key_init(struct rsa_public_key *key);

/* Calls mpz_clear to deallocate bignum storage. */
void
rsa_public_key_clear(struct rsa_public_key *key);

int
rsa_public_key_prepare(struct rsa_public_key *key);

/* Calls mpz_init to initialize bignum storage. */
void
rsa_private_key_init(struct rsa_private_key *key);

/* Calls mpz_clear to deallocate bignum storage. */
void
rsa_private_key_clear(struct rsa_private_key *key);

int
rsa_private_key_prepare(struct rsa_private_key *key);


/* PKCS#1 style signatures */
int
rsa_pkcs1_sign(const struct rsa_private_key *key,
	       size_t length, const uint8_t *digest_info,
	       mpz_t s);

int
rsa_pkcs1_sign_tr(const struct rsa_public_key *pub,
  	          const struct rsa_private_key *key,
	          void *random_ctx, nettle_random_func *random,
	          size_t length, const uint8_t *digest_info,
   	          mpz_t s);
int
rsa_pkcs1_verify(const struct rsa_public_key *key,
		 size_t length, const uint8_t *digest_info,
		 const mpz_t signature);

int
rsa_md5_sign(const struct rsa_private_key *key,
             struct md5_ctx *hash,
             mpz_t signature);

int
rsa_md5_sign_tr(const struct rsa_public_key *pub,
		const struct rsa_private_key *key,
		void *random_ctx, nettle_random_func *random,
		struct md5_ctx *hash, mpz_t s);


int
rsa_md5_verify(const struct rsa_public_key *key,
               struct md5_ctx *hash,
	       const mpz_t signature);

int
rsa_sha1_sign(const struct rsa_private_key *key,
              struct sha1_ctx *hash,
              mpz_t signature);

int
rsa_sha1_sign_tr(const struct rsa_public_key *pub,
		 const struct rsa_private_key *key,
		 void *random_ctx, nettle_random_func *random,
		 struct sha1_ctx *hash,
		 mpz_t s);

int
rsa_sha1_verify(const struct rsa_public_key *key,
                struct sha1_ctx *hash,
		const mpz_t signature);

int
rsa_sha256_sign(const struct rsa_private_key *key,
		struct sha256_ctx *hash,
		mpz_t signature);

int
rsa_sha256_sign_tr(const struct rsa_public_key *pub,
		   const struct rsa_private_key *key,
		   void *random_ctx, nettle_random_func *random,
		   struct sha256_ctx *hash,
		   mpz_t s);

int
rsa_sha256_verify(const struct rsa_public_key *key,
		  struct sha256_ctx *hash,
		  const mpz_t signature);

int
rsa_sha512_sign(const struct rsa_private_key *key,
		struct sha512_ctx *hash,
		mpz_t signature);

int
rsa_sha512_sign_tr(const struct rsa_public_key *pub,
		   const struct rsa_private_key *key,
		   void *random_ctx, nettle_random_func *random,
		   struct sha512_ctx *hash,
		   mpz_t s);

int
rsa_sha512_verify(const struct rsa_public_key *key,
		  struct sha512_ctx *hash,
		  const mpz_t signature);

/* Variants taking the digest as argument. */
int
rsa_md5_sign_digest(const struct rsa_private_key *key,
		    const uint8_t *digest,
		    mpz_t s);

int
rsa_md5_sign_digest_tr(const struct rsa_public_key *pub,
		       const struct rsa_private_key *key,
		       void *random_ctx, nettle_random_func *random,
		       const uint8_t *digest, mpz_t s);

int
rsa_md5_verify_digest(const struct rsa_public_key *key,
		      const uint8_t *digest,
		      const mpz_t signature);

int
rsa_sha1_sign_digest(const struct rsa_private_key *key,
		     const uint8_t *digest,
		     mpz_t s);

int
rsa_sha1_sign_digest_tr(const struct rsa_public_key *pub,
			const struct rsa_private_key *key,
			void *random_ctx, nettle_random_func *random,
			const uint8_t *digest,
			mpz_t s);

int
rsa_sha1_verify_digest(const struct rsa_public_key *key,
		       const uint8_t *digest,
		       const mpz_t signature);

int
rsa_sha256_sign_digest(const struct rsa_private_key *key,
		       const uint8_t *digest,
		       mpz_t s);

int
rsa_sha256_sign_digest_tr(const struct rsa_public_key *pub,
			  const struct rsa_private_key *key,
			  void *random_ctx, nettle_random_func *random,
			  const uint8_t *digest,
			  mpz_t s);

int
rsa_sha256_verify_digest(const struct rsa_public_key *key,
			 const uint8_t *digest,
			 const mpz_t signature);

int
rsa_sha512_sign_digest(const struct rsa_private_key *key,
		       const uint8_t *digest,
		       mpz_t s);

int
rsa_sha512_sign_digest_tr(const struct rsa_public_key *pub,
			  const struct rsa_private_key *key,
			  void *random_ctx, nettle_random_func *random,
			  const uint8_t *digest,
			  mpz_t s);

int
rsa_sha512_verify_digest(const struct rsa_public_key *key,
			 const uint8_t *digest,
			 const mpz_t signature);

/* PSS style signatures */
int
rsa_pss_sha256_sign_digest_tr(const struct rsa_public_key *pub,
			      const struct rsa_private_key *key,
			      void *random_ctx, nettle_random_func *random,
			      size_t salt_length, const uint8_t *salt,
			      const uint8_t *digest,
			      mpz_t s);

int
rsa_pss_sha256_verify_digest(const struct rsa_public_key *key,
			     size_t salt_length,
			     const uint8_t *digest,
			     const mpz_t signature);

int
rsa_pss_sha384_sign_digest_tr(const struct rsa_public_key *pub,
			      const struct rsa_private_key *key,
			      void *random_ctx, nettle_random_func *random,
			      size_t salt_length, const uint8_t *salt,
			      const uint8_t *digest,
			      mpz_t s);

int
rsa_pss_sha384_verify_digest(const struct rsa_public_key *key,
			     size_t salt_length,
			     const uint8_t *digest,
			     const mpz_t signature);

int
rsa_pss_sha512_sign_digest_tr(const struct rsa_public_key *pub,
			      const struct rsa_private_key *key,
			      void *random_ctx, nettle_random_func *random,
			      size_t salt_length, const uint8_t *salt,
			      const uint8_t *digest,
			      mpz_t s);

int
rsa_pss_sha512_verify_digest(const struct rsa_public_key *key,
			     size_t salt_length,
			     const uint8_t *digest,
			     const mpz_t signature);


/* RSA encryption, using PKCS#1 */
/* These functions uses the v1.5 padding. What should the v2 (OAEP)
 * functions be called? */

/* Returns 1 on success, 0 on failure, which happens if the
 * message is too long for the key. */
int
rsa_encrypt(const struct rsa_public_key *key,
	    /* For padding */
	    void *random_ctx, nettle_random_func *random,
	    size_t length, const uint8_t *cleartext,
	    mpz_t cipher);

/* Message must point to a buffer of size *LENGTH. KEY->size is enough
 * for all valid messages. On success, *LENGTH is updated to reflect
 * the actual length of the message. Returns 1 on success, 0 on
 * failure, which happens if decryption failed or if the message
 * didn't fit. */
int
rsa_decrypt(const struct rsa_private_key *key,
	    size_t *length, uint8_t *cleartext,
	    const mpz_t ciphertext);

/* Timing-resistant version, using randomized RSA blinding. */
int
rsa_decrypt_tr(const struct rsa_public_key *pub,
	       const struct rsa_private_key *key,
	       void *random_ctx, nettle_random_func *random,	       
	       size_t *length, uint8_t *message,
	       const mpz_t gibberish);

/* like rsa_decrypt_tr but with additional side-channel resistance.
 * NOTE: the length of the final message must be known in advance. */
int
rsa_sec_decrypt(const struct rsa_public_key *pub,
	        const struct rsa_private_key *key,
	        void *random_ctx, nettle_random_func *random,
	        size_t length, uint8_t *message,
	        const mpz_t gibberish);

/* Compute x, the e:th root of m. Calling it with x == m is allowed.
   It is required that 0 <= m < n. */
void
rsa_compute_root(const struct rsa_private_key *key,
		 mpz_t x, const mpz_t m);

/* Safer variant, using RSA blinding, and checking the result after
   CRT. It is required that 0 <= m < n. */
int
rsa_compute_root_tr(const struct rsa_public_key *pub,
		    const struct rsa_private_key *key,
		    void *random_ctx, nettle_random_func *random,
		    mpz_t x, const mpz_t m);

/* Key generation */

/* Note that the key structs must be initialized first. */
int
rsa_generate_keypair(struct rsa_public_key *pub,
		     struct rsa_private_key *key,

		     void *random_ctx, nettle_random_func *random,
		     void *progress_ctx, nettle_progress_func *progress,

		     /* Desired size of modulo, in bits */
		     unsigned n_size,
		     
		     /* Desired size of public exponent, in bits. If
		      * zero, the passed in value pub->e is used. */
		     unsigned e_size);


#define RSA_SIGN(key, algorithm, ctx, length, data, signature) ( \
  algorithm##_update(ctx, length, data), \
  rsa_##algorithm##_sign(key, ctx, signature) \
)

#define RSA_VERIFY(key, algorithm, ctx, length, data, signature) ( \
  algorithm##_update(ctx, length, data), \
  rsa_##algorithm##_verify(key, ctx, signature) \
)


/* Keys in sexp form. */

struct nettle_buffer;

/* Generates a public-key expression if PRIV is NULL .*/
int
rsa_keypair_to_sexp(struct nettle_buffer *buffer,
		    const char *algorithm_name, /* NULL means "rsa" */
		    const struct rsa_public_key *pub,
		    const struct rsa_private_key *priv);

struct sexp_iterator;

int
rsa_keypair_from_sexp_alist(struct rsa_public_key *pub,
			    struct rsa_private_key *priv,
			    unsigned limit,
			    struct sexp_iterator *i);

/* If PRIV is NULL, expect a public-key expression. If PUB is NULL,
 * expect a private key expression and ignore the parts not needed for
 * the public key. */
/* Keys must be initialized before calling this function, as usual. */
int
rsa_keypair_from_sexp(struct rsa_public_key *pub,
		      struct rsa_private_key *priv,
		      unsigned limit,
		      size_t length, const uint8_t *expr);


/* Keys in PKCS#1 format. */
struct asn1_der_iterator;

int
rsa_public_key_from_der_iterator(struct rsa_public_key *pub,
				 unsigned limit,
				 struct asn1_der_iterator *i);

int
rsa_private_key_from_der_iterator(struct rsa_public_key *pub,
				  struct rsa_private_key *priv,
				  unsigned limit,
				  struct asn1_der_iterator *i);

/* For public keys, use PRIV == NULL */ 
int
rsa_keypair_from_der(struct rsa_public_key *pub,
		     struct rsa_private_key *priv,
		     unsigned limit, 
		     size_t length, const uint8_t *data);

/* OpenPGP format. Experimental interface, subject to change. */
int
rsa_keypair_to_openpgp(struct nettle_buffer *buffer,
		       const struct rsa_public_key *pub,
		       const struct rsa_private_key *priv,
		       /* A single user id. NUL-terminated utf8. */
		       const char *userid);


#ifdef __cplusplus
}
#endif

#endif /* NETTLE_RSA_H_INCLUDED */
