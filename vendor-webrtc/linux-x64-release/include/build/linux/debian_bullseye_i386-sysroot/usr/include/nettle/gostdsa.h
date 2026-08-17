/* gostdsa.h

   Copyright (C) 2015 Dmity Eremin-Solenikov
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

#ifndef NETTLE_GOSTDSA_H_INCLUDED
#define NETTLE_GOSTDSA_H_INCLUDED

#include "ecc.h"
#include "dsa.h"
#include "ecdsa.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define gostdsa_sign nettle_gostdsa_sign
#define gostdsa_verify nettle_gostdsa_verify
#define gostdsa_vko nettle_gostdsa_vko
#define ecc_gostdsa_sign nettle_ecc_gostdsa_sign
#define ecc_gostdsa_sign_itch nettle_ecc_gostdsa_sign_itch
#define ecc_gostdsa_verify nettle_ecc_gostdsa_verify
#define ecc_gostdsa_verify_itch nettle_ecc_gostdsa_verify_itch

/* Just use ECDSA function for key generation */
#define gostdsa_generate_keypair ecdsa_generate_keypair

/* High level GOST DSA functions.
 *
 * A public key is represented as a struct ecc_point, and a private
 * key as a struct ecc_scalar. FIXME: Introduce some aliases? */
void
gostdsa_sign (const struct ecc_scalar *key,
	      void *random_ctx, nettle_random_func *random,
	      size_t digest_length,
	      const uint8_t *digest,
	      struct dsa_signature *signature);

int
gostdsa_verify (const struct ecc_point *pub,
	        size_t length, const uint8_t *digest,
	        const struct dsa_signature *signature);

void
gostdsa_vko (const struct ecc_scalar *key,
	     const struct ecc_point *pub,
	     size_t ukm_length, const uint8_t *ukm,
	     uint8_t *out);

/* Low-level GOSTDSA functions. */
mp_size_t
ecc_gostdsa_sign_itch (const struct ecc_curve *ecc);

void
ecc_gostdsa_sign (const struct ecc_curve *ecc,
		const mp_limb_t *zp,
		/* Random nonce, must be invertible mod ecc group
		   order. */
		const mp_limb_t *kp,
		size_t length, const uint8_t *digest,
		mp_limb_t *rp, mp_limb_t *sp,
		mp_limb_t *scratch);

mp_size_t
ecc_gostdsa_verify_itch (const struct ecc_curve *ecc);

int
ecc_gostdsa_verify (const struct ecc_curve *ecc,
		  const mp_limb_t *pp, /* Public key */
		  size_t length, const uint8_t *digest,
		  const mp_limb_t *rp, const mp_limb_t *sp,
		  mp_limb_t *scratch);


#ifdef __cplusplus
}
#endif

#endif /* NETTLE_GOSTDSA_H_INCLUDED */
