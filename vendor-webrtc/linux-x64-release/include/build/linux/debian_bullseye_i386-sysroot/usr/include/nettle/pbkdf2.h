/* pbkdf2.h

   PKCS #5 password-based key derivation function PBKDF2, see RFC 2898.

   Copyright (C) 2012 Simon Josefsson

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

#ifndef NETTLE_PBKDF2_H_INCLUDED
#define NETTLE_PBKDF2_H_INCLUDED

#include "nettle-meta.h"

#ifdef __cplusplus
extern "C"
{
#endif

/* Namespace mangling */
#define pbkdf2 nettle_pbkdf2
#define pbkdf2_hmac_sha1 nettle_pbkdf2_hmac_sha1
#define pbkdf2_hmac_sha256 nettle_pbkdf2_hmac_sha256
#define pbkdf2_hmac_sha384 nettle_pbkdf2_hmac_sha384
#define pbkdf2_hmac_sha512 nettle_pbkdf2_hmac_sha512
#define pbkdf2_hmac_gosthash94cp nettle_pbkdf2_hmac_gosthash94cp

void
pbkdf2 (void *mac_ctx,
	nettle_hash_update_func *update,
	nettle_hash_digest_func *digest,
	size_t digest_size, unsigned iterations,
	size_t salt_length, const uint8_t *salt,
	size_t length, uint8_t *dst);

#define PBKDF2(ctx, update, digest, digest_size,			\
	       iterations, salt_length, salt, length, dst)		\
  (0 ? ((update)((ctx), 0, (uint8_t *) 0),				\
	(digest)((ctx), 0, (uint8_t *) 0))				\
   : pbkdf2 ((ctx),							\
	     (nettle_hash_update_func *)(update),			\
	     (nettle_hash_digest_func *)(digest),			\
	     (digest_size), (iterations),				\
	     (salt_length), (salt), (length), (dst)))

/* PBKDF2 with specific PRFs. */

void
pbkdf2_hmac_sha1 (size_t key_length, const uint8_t *key,
		  unsigned iterations,
		  size_t salt_length, const uint8_t *salt,
		  size_t length, uint8_t *dst);

void
pbkdf2_hmac_sha256 (size_t key_length, const uint8_t *key,
		    unsigned iterations,
		    size_t salt_length, const uint8_t *salt,
		    size_t length, uint8_t *dst);

void
pbkdf2_hmac_sha384 (size_t key_length, const uint8_t *key,
		    unsigned iterations,
		    size_t salt_length, const uint8_t *salt,
		    size_t length, uint8_t *dst);

void
pbkdf2_hmac_sha512 (size_t key_length, const uint8_t *key,
		    unsigned iterations,
		    size_t salt_length, const uint8_t *salt,
		    size_t length, uint8_t *dst);

void
pbkdf2_hmac_gosthash94cp (size_t key_length, const uint8_t *key,
			  unsigned iterations,
			  size_t salt_length, const uint8_t *salt,
			  size_t length, uint8_t *dst);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_PBKDF2_H_INCLUDED */
