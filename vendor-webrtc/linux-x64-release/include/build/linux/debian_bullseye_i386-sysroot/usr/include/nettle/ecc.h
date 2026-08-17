/* ecc.h

   Copyright (C) 2013 Niels Möller

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

/* Development of Nettle's ECC support was funded by the .SE Internet Fund. */

#ifndef NETTLE_ECC_H_INCLUDED
#define NETTLE_ECC_H_INCLUDED

#include "nettle-types.h"
#include "bignum.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define ecc_point_init nettle_ecc_point_init
#define ecc_point_clear nettle_ecc_point_clear
#define ecc_point_set nettle_ecc_point_set
#define ecc_point_get nettle_ecc_point_get
#define ecc_point_mul nettle_ecc_point_mul
#define ecc_point_mul_g nettle_ecc_point_mul_g
#define ecc_scalar_init nettle_ecc_scalar_init
#define ecc_scalar_clear nettle_ecc_scalar_clear
#define ecc_scalar_set nettle_ecc_scalar_set
#define ecc_scalar_get nettle_ecc_scalar_get
#define ecc_scalar_random nettle_ecc_scalar_random
#define ecc_point_mul nettle_ecc_point_mul
#define ecc_bit_size nettle_ecc_bit_size
#define ecc_size nettle_ecc_size
#define ecc_size_a nettle_ecc_size_a
#define ecc_size_j nettle_ecc_size_j

struct ecc_curve;

/* High level interface, for ECDSA, DH, etc */

/* Represents a point on the ECC curve */
struct ecc_point
{
  const struct ecc_curve *ecc;
  /* Allocated using the same allocation function as GMP. */
  mp_limb_t *p;
};

/* Represents a non-zero scalar, an element of Z_q^*, where q is the
   group order of the curve. */
struct ecc_scalar
{
  const struct ecc_curve *ecc;
  /* Allocated using the same allocation function as GMP. */
  mp_limb_t *p;
};

void
ecc_point_init (struct ecc_point *p, const struct ecc_curve *ecc);
void
ecc_point_clear (struct ecc_point *p);

/* Fails and returns zero if the point is not on the curve. */
int
ecc_point_set (struct ecc_point *p, const mpz_t x, const mpz_t y);
void
ecc_point_get (const struct ecc_point *p, mpz_t x, mpz_t y);

void
ecc_scalar_init (struct ecc_scalar *s, const struct ecc_curve *ecc);
void
ecc_scalar_clear (struct ecc_scalar *s);

/* Fails and returns zero if the scalar is not in the proper range. */
int
ecc_scalar_set (struct ecc_scalar *s, const mpz_t z);
void
ecc_scalar_get (const struct ecc_scalar *s, mpz_t z);
/* Generates a random scalar, suitable as an ECDSA private key or a
   ECDH exponent. */
void
ecc_scalar_random (struct ecc_scalar *s,
		   void *random_ctx, nettle_random_func *random);

/* Computes r = n p */
void
ecc_point_mul (struct ecc_point *r, const struct ecc_scalar *n,
	       const struct ecc_point *p);

/* Computes r = n g */
void
ecc_point_mul_g (struct ecc_point *r, const struct ecc_scalar *n);


/* Low-level interface */
  
/* Points on a curve are represented as arrays of mp_limb_t, with
   curve-specific representation. For the secp curves, we use Jacobian
   coordinates (possibly in Montgomery form for mod multiplication).
   For curve25519 we use homogeneous coordinates on an equivalent
   Edwards curve. The suffix "_h" denotes this internal
   representation.
   
   Since we use additive notation for the groups, the infinity point
   on the curve is denoted 0. The infinity point can be represented
   with x = y = 0 in affine coordinates, and Z = 0 in Jacobian
   coordinates. However, note that most of the ECC functions do *not*
   support infinity as an input or output.
*/

/* Returns the bit size of a single coordinate (and of the prime p). */
unsigned
ecc_bit_size (const struct ecc_curve *ecc);

/* Returns the size of a single coordinate. */
mp_size_t
ecc_size (const struct ecc_curve *ecc);

/* Size of a point, using affine coordinates x, y. */
mp_size_t
ecc_size_a (const struct ecc_curve *ecc);

/* Size of a point, using jacobian coordinates X, Y and Z. */
mp_size_t
ecc_size_j (const struct ecc_curve *ecc);

/* FIXME: Define a generic ecc_dup, ecc_add, for any type of curve. Do
   they need to handle infinity points? */

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_ECC_H_INCLUDED */
