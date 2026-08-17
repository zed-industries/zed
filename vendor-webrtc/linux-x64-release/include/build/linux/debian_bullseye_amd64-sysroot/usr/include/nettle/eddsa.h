/* eddsa.h

   Copyright (C) 2014 Niels Möller

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

#ifndef NETTLE_EDDSA_H
#define NETTLE_EDDSA_H

#include "nettle-types.h"

#include "bignum.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define ed25519_sha512_set_private_key nettle_ed25519_sha512_set_private_key
#define ed25519_sha512_public_key nettle_ed25519_sha512_public_key
#define ed25519_sha512_sign nettle_ed25519_sha512_sign
#define ed25519_sha512_verify nettle_ed25519_sha512_verify
#define ed448_shake256_public_key nettle_ed448_shake256_public_key
#define ed448_shake256_sign nettle_ed448_shake256_sign
#define ed448_shake256_verify nettle_ed448_shake256_verify

#define ED25519_KEY_SIZE 32
#define ED25519_SIGNATURE_SIZE 64

void
ed25519_sha512_public_key (uint8_t *pub, const uint8_t *priv);

void
ed25519_sha512_sign (const uint8_t *pub,
		     const uint8_t *priv,		     
		     size_t length, const uint8_t *msg,
		     uint8_t *signature);

int
ed25519_sha512_verify (const uint8_t *pub,
		       size_t length, const uint8_t *msg,
		       const uint8_t *signature);

#define ED448_KEY_SIZE 57
#define ED448_SIGNATURE_SIZE 114

void
ed448_shake256_public_key (uint8_t *pub, const uint8_t *priv);

void
ed448_shake256_sign (const uint8_t *pub,
		     const uint8_t *priv,
		     size_t length, const uint8_t *msg,
		     uint8_t *signature);

int
ed448_shake256_verify (const uint8_t *pub,
		       size_t length, const uint8_t *msg,
		       const uint8_t *signature);
			   
#ifdef __cplusplus
}
#endif

#endif /* NETTLE_EDDSA_H */
