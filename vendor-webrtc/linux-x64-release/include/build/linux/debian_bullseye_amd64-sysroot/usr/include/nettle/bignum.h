/* bignum.h

   Bignum operations that are missing from gmp.

   Copyright (C) 2001 Niels Möller

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
 
#ifndef NETTLE_BIGNUM_H_INCLUDED
#define NETTLE_BIGNUM_H_INCLUDED

#include "nettle-types.h"

/* For NETTLE_USE_MINI_GMP */
#include "version.h"

#if NETTLE_USE_MINI_GMP
# include "mini-gmp.h"

# define GMP_NUMB_MASK (~(mp_limb_t) 0)

/* Side-channel silent powm not available in mini-gmp. */
# define mpz_powm_sec mpz_powm
#else
# include <gmp.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

/* Size needed for signed encoding, including extra sign byte if
 * necessary. */
size_t
nettle_mpz_sizeinbase_256_s(const mpz_t x);

/* Size needed for unsigned encoding */
size_t
nettle_mpz_sizeinbase_256_u(const mpz_t x);

/* Writes an integer as length octets, using big endian byte order,
 * and two's complement for negative numbers. */
void
nettle_mpz_get_str_256(size_t length, uint8_t *s, const mpz_t x);

/* Reads a big endian, two's complement, integer. */
void
nettle_mpz_set_str_256_s(mpz_t x,
			 size_t length, const uint8_t *s);

void
nettle_mpz_init_set_str_256_s(mpz_t x,
			      size_t length, const uint8_t *s);

/* Similar, but for unsigned format. These function don't interpret
 * the most significant bit as the sign. */
void
nettle_mpz_set_str_256_u(mpz_t x,
			 size_t length, const uint8_t *s);

void
nettle_mpz_init_set_str_256_u(mpz_t x,
			      size_t length, const uint8_t *s);

/* Returns a uniformly distributed random number 0 <= x < 2^n */
void
nettle_mpz_random_size(mpz_t x,
		       void *ctx, nettle_random_func *random,
		       unsigned bits);

/* Returns a number x, almost uniformly random in the range
 * 0 <= x < n. */
void
nettle_mpz_random(mpz_t x, 
		  void *ctx, nettle_random_func *random,
		  const mpz_t n);

void
nettle_random_prime(mpz_t p, unsigned bits, int top_bits_set,
		    void *ctx, nettle_random_func *random,
		    void *progress_ctx, nettle_progress_func *progress);

  
/* sexp parsing */
struct sexp_iterator;

/* If LIMIT is non-zero, the number must be at most LIMIT bits.
 * Implies sexp_iterator_next. */
int
nettle_mpz_set_sexp(mpz_t x, unsigned limit, struct sexp_iterator *i);


/* der parsing */
struct asn1_der_iterator;

int
nettle_asn1_der_get_bignum(struct asn1_der_iterator *iterator,
			   mpz_t x, unsigned max_bits);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_BIGNUM_H_INCLUDED */
