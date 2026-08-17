/* dsa.h

   The DSA publickey algorithm.

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
 
#ifndef NETTLE_DSA_H_INCLUDED
#define NETTLE_DSA_H_INCLUDED

#include "nettle-types.h"
#include "bignum.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define dsa_params_init nettle_dsa_params_init
#define dsa_params_clear nettle_dsa_params_clear
#define dsa_signature_init nettle_dsa_signature_init
#define dsa_signature_clear nettle_dsa_signature_clear
#define dsa_sign nettle_dsa_sign
#define dsa_verify nettle_dsa_verify
#define dsa_generate_params nettle_dsa_generate_params
#define dsa_generate_keypair nettle_dsa_generate_keypair
#define dsa_signature_from_sexp nettle_dsa_signature_from_sexp
#define dsa_keypair_to_sexp nettle_dsa_keypair_to_sexp
#define dsa_keypair_from_sexp_alist nettle_dsa_keypair_from_sexp_alist
#define dsa_sha1_keypair_from_sexp nettle_dsa_sha1_keypair_from_sexp
#define dsa_sha256_keypair_from_sexp nettle_dsa_sha256_keypair_from_sexp
#define dsa_params_from_der_iterator nettle_dsa_params_from_der_iterator
#define dsa_public_key_from_der_iterator nettle_dsa_public_key_from_der_iterator
#define dsa_openssl_private_key_from_der_iterator nettle_dsa_openssl_private_key_from_der_iterator 
#define dsa_openssl_private_key_from_der nettle_openssl_provate_key_from_der

/* For FIPS approved parameters */
#define DSA_SHA1_MIN_P_BITS 512
#define DSA_SHA1_Q_OCTETS 20
#define DSA_SHA1_Q_BITS 160

#define DSA_SHA256_MIN_P_BITS 1024
#define DSA_SHA256_Q_OCTETS 32
#define DSA_SHA256_Q_BITS 256

struct dsa_params
{  
  /* Modulo */
  mpz_t p;

  /* Group order */
  mpz_t q;

  /* Generator */
  mpz_t g;
};

void
dsa_params_init (struct dsa_params *params);

void
dsa_params_clear (struct dsa_params *params);

struct dsa_signature
{
  mpz_t r;
  mpz_t s;
};

/* Calls mpz_init to initialize bignum storage. */
void
dsa_signature_init(struct dsa_signature *signature);

/* Calls mpz_clear to deallocate bignum storage. */
void
dsa_signature_clear(struct dsa_signature *signature);

int
dsa_sign(const struct dsa_params *params,
	 const mpz_t x,
	 void *random_ctx, nettle_random_func *random,
	 size_t digest_size,
	 const uint8_t *digest,
	 struct dsa_signature *signature);

int
dsa_verify(const struct dsa_params *params,
	   const mpz_t y,
	   size_t digest_size,
	   const uint8_t *digest,
	   const struct dsa_signature *signature);


/* Key generation */

int
dsa_generate_params(struct dsa_params *params,
		    void *random_ctx, nettle_random_func *random,
		    void *progress_ctx, nettle_progress_func *progress,
		    unsigned p_bits, unsigned q_bits);

void
dsa_generate_keypair (const struct dsa_params *params,
		      mpz_t pub, mpz_t key,
		      void *random_ctx, nettle_random_func *random);

/* Keys in sexp form. */

struct nettle_buffer;

/* Generates a public-key expression if PRIV is NULL .*/
int
dsa_keypair_to_sexp(struct nettle_buffer *buffer,
		    const char *algorithm_name, /* NULL means "dsa" */
		    const struct dsa_params *params,
		    const mpz_t pub,
		    const mpz_t priv);

struct sexp_iterator;

int
dsa_signature_from_sexp(struct dsa_signature *rs,
			struct sexp_iterator *i,
			unsigned q_bits);

int
dsa_keypair_from_sexp_alist(struct dsa_params *params,
			    mpz_t pub,
			    mpz_t priv,
			    unsigned p_max_bits,
			    unsigned q_bits,
			    struct sexp_iterator *i);

/* If PRIV is NULL, expect a public-key expression. If PUB is NULL,
 * expect a private key expression and ignore the parts not needed for
 * the public key. */
/* Keys must be initialized before calling this function, as usual. */
int
dsa_sha1_keypair_from_sexp(struct dsa_params *params,
			   mpz_t pub,
			   mpz_t priv,
			   unsigned p_max_bits,
			   size_t length, const uint8_t *expr);

int
dsa_sha256_keypair_from_sexp(struct dsa_params *params,
			     mpz_t pub,
			     mpz_t priv,
			     unsigned p_max_bits,
			     size_t length, const uint8_t *expr);

/* Keys in X.509 andd OpenSSL format. */
struct asn1_der_iterator;

int
dsa_params_from_der_iterator(struct dsa_params *params,
			     unsigned max_bits, unsigned q_bits,
			     struct asn1_der_iterator *i);

int
dsa_public_key_from_der_iterator(const struct dsa_params *params,
				 mpz_t pub,
				 struct asn1_der_iterator *i);

int
dsa_openssl_private_key_from_der_iterator(struct dsa_params *params,
					  mpz_t pub,
					  mpz_t priv,
					  unsigned p_max_bits,
					  struct asn1_der_iterator *i);

int
dsa_openssl_private_key_from_der(struct dsa_params *params,
				 mpz_t pub,
				 mpz_t priv,
				 unsigned p_max_bits,
				 size_t length, const uint8_t *data);


#ifdef __cplusplus
}
#endif

#endif /* NETTLE_DSA_H_INCLUDED */
