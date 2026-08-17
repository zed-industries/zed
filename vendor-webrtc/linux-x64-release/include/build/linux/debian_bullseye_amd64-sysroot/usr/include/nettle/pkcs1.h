/* pkcs1.h

   PKCS1 embedding.

   Copyright (C) 2003 Niels Möller

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

#ifndef NETTLE_PKCS1_H_INCLUDED
#define NETTLE_PKCS1_H_INCLUDED

#include "nettle-types.h"
#include "bignum.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define pkcs1_rsa_digest_encode nettle_pkcs1_rsa_digest_encode
#define pkcs1_rsa_md5_encode nettle_pkcs1_rsa_md5_encode
#define pkcs1_rsa_md5_encode_digest nettle_pkcs1_rsa_md5_encode_digest
#define pkcs1_rsa_sha1_encode nettle_pkcs1_rsa_sha1_encode
#define pkcs1_rsa_sha1_encode_digest nettle_pkcs1_rsa_sha1_encode_digest
#define pkcs1_rsa_sha256_encode nettle_pkcs1_rsa_sha256_encode
#define pkcs1_rsa_sha256_encode_digest nettle_pkcs1_rsa_sha256_encode_digest
#define pkcs1_rsa_sha512_encode nettle_pkcs1_rsa_sha512_encode
#define pkcs1_rsa_sha512_encode_digest nettle_pkcs1_rsa_sha512_encode_digest
#define pkcs1_encrypt nettle_pkcs1_encrypt
#define pkcs1_decrypt nettle_pkcs1_decrypt

struct md5_ctx;
struct sha1_ctx;
struct sha256_ctx;
struct sha512_ctx;

int
pkcs1_encrypt (size_t key_size,
	       /* For padding */
	       void *random_ctx, nettle_random_func *random,
	       size_t length, const uint8_t *message,
	       mpz_t m);

int
pkcs1_decrypt (size_t key_size,
	       const mpz_t m,
	       size_t *length, uint8_t *message);

int
pkcs1_rsa_digest_encode(mpz_t m, size_t key_size,
			size_t di_length, const uint8_t *digest_info);

int
pkcs1_rsa_md5_encode(mpz_t m, size_t length, struct md5_ctx *hash);

int
pkcs1_rsa_md5_encode_digest(mpz_t m, size_t length, const uint8_t *digest);

int
pkcs1_rsa_sha1_encode(mpz_t m, size_t length, struct sha1_ctx *hash);

int
pkcs1_rsa_sha1_encode_digest(mpz_t m, size_t length, const uint8_t *digest);

int
pkcs1_rsa_sha256_encode(mpz_t m, size_t length, struct sha256_ctx *hash);

int
pkcs1_rsa_sha256_encode_digest(mpz_t m, size_t length, const uint8_t *digest);

int
pkcs1_rsa_sha512_encode(mpz_t m, size_t length, struct sha512_ctx *hash);

int
pkcs1_rsa_sha512_encode_digest(mpz_t m, size_t length, const uint8_t *digest);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_PKCS1_H_INCLUDED */
