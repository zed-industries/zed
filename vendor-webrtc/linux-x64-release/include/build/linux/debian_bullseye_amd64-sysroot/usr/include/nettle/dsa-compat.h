/* dsa-compat.h

   Old DSA publickey interface.

   Copyright (C) 2002, 2013, 2014 Niels Möller

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

#ifndef NETTLE_DSA_COMPAT_H_INCLUDED
#define NETTLE_DSA_COMPAT_H_INCLUDED

#include "dsa.h"

#include "sha1.h"
#include "sha2.h"

/* Name mangling */
#define dsa_public_key_init nettle_dsa_public_key_init
#define dsa_public_key_clear nettle_dsa_public_key_clear
#define dsa_private_key_init nettle_dsa_private_key_init
#define dsa_private_key_clear nettle_dsa_private_key_clear
#define dsa_sha1_sign nettle_dsa_sha1_sign
#define dsa_sha1_verify nettle_dsa_sha1_verify
#define dsa_sha256_sign nettle_dsa_sha256_sign
#define dsa_sha256_verify nettle_dsa_sha256_verify
#define dsa_sha1_sign_digest nettle_dsa_sha1_sign_digest
#define dsa_sha1_verify_digest nettle_dsa_sha1_verify_digest
#define dsa_sha256_sign_digest nettle_dsa_sha256_sign_digest
#define dsa_sha256_verify_digest nettle_dsa_sha256_verify_digest
#define dsa_compat_generate_keypair nettle_dsa_compat_generate_keypair

/* Switch meaning of dsa_generate_keypair */
#undef dsa_generate_keypair
#define dsa_generate_keypair nettle_dsa_compat_generate_keypair

#ifdef __cplusplus
extern "C" {
#endif

struct dsa_public_key
{
  /* Same as struct dsa_params, but can't use that struct here without
     breaking backwards compatibility. Layout must be identical, since
     this is cast to a struct dsa_param pointer for calling _dsa_sign
     and _dsa_verify */
  mpz_t p;
  mpz_t q;
  mpz_t g;

  /* Public value */
  mpz_t y;
};

struct dsa_private_key
{
  /* Unlike an rsa public key, private key operations will need both
   * the private and the public information. */
  mpz_t x;
};

/* Signing a message works as follows:
 *
 * Store the private key in a dsa_private_key struct.
 *
 * Initialize a hashing context, by callling
 *   sha1_init
 *
 * Hash the message by calling
 *   sha1_update
 *
 * Create the signature by calling
 *   dsa_sha1_sign
 *
 * The signature is represented as a struct dsa_signature. This call also
 * resets the hashing context.
 *
 * When done with the key and signature, don't forget to call
 * dsa_signature_clear.
 */

/* Calls mpz_init to initialize bignum storage. */
void
dsa_public_key_init(struct dsa_public_key *key);

/* Calls mpz_clear to deallocate bignum storage. */
void
dsa_public_key_clear(struct dsa_public_key *key);


/* Calls mpz_init to initialize bignum storage. */
void
dsa_private_key_init(struct dsa_private_key *key);

/* Calls mpz_clear to deallocate bignum storage. */
void
dsa_private_key_clear(struct dsa_private_key *key);

int
dsa_sha1_sign(const struct dsa_public_key *pub,
	      const struct dsa_private_key *key,
	      void *random_ctx, nettle_random_func *random,
	      struct sha1_ctx *hash,
	      struct dsa_signature *signature);

int
dsa_sha256_sign(const struct dsa_public_key *pub,
		const struct dsa_private_key *key,
		void *random_ctx, nettle_random_func *random,
		struct sha256_ctx *hash,
		struct dsa_signature *signature);

int
dsa_sha1_verify(const struct dsa_public_key *key,
		struct sha1_ctx *hash,
		const struct dsa_signature *signature);

int
dsa_sha256_verify(const struct dsa_public_key *key,
		  struct sha256_ctx *hash,
		  const struct dsa_signature *signature);

int
dsa_sha1_sign_digest(const struct dsa_public_key *pub,
		     const struct dsa_private_key *key,
		     void *random_ctx, nettle_random_func *random,
		     const uint8_t *digest,
		     struct dsa_signature *signature);
int
dsa_sha256_sign_digest(const struct dsa_public_key *pub,
		       const struct dsa_private_key *key,
		       void *random_ctx, nettle_random_func *random,
		       const uint8_t *digest,
		       struct dsa_signature *signature);

int
dsa_sha1_verify_digest(const struct dsa_public_key *key,
		       const uint8_t *digest,
		       const struct dsa_signature *signature);

int
dsa_sha256_verify_digest(const struct dsa_public_key *key,
			 const uint8_t *digest,
			 const struct dsa_signature *signature);

/* Key generation */
int
dsa_generate_keypair(struct dsa_public_key *pub,
		     struct dsa_private_key *key,

		     void *random_ctx, nettle_random_func *random,
		     void *progress_ctx, nettle_progress_func *progress,
		     unsigned p_bits, unsigned q_bits);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_DSA_COMPAT_H_INCLUDED */
