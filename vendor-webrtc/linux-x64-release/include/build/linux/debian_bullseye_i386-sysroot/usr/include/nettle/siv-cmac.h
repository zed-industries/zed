/* siv-cmac.h

   AES-SIV, RFC5297

   Copyright (C) 2017 Nikos Mavrogiannopoulos

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

#ifndef NETTLE_SIV_H_INCLUDED
#define NETTLE_SIV_H_INCLUDED

#include "nettle-types.h"
#include "nettle-meta.h"
#include "cmac.h"
#include "aes.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define siv_cmac_set_key nettle_siv_cmac_set_key
#define siv_cmac_encrypt_message nettle_siv_cmac_encrypt_message
#define siv_cmac_decrypt_message nettle_siv_cmac_decrypt_message
#define siv_cmac_aes128_set_key nettle_siv_cmac_aes128_set_key
#define siv_cmac_aes128_encrypt_message nettle_siv_cmac_aes128_encrypt_message
#define siv_cmac_aes128_decrypt_message nettle_siv_cmac_aes128_decrypt_message
#define siv_cmac_aes256_set_key nettle_siv_cmac_aes256_set_key
#define siv_cmac_aes256_encrypt_message nettle_siv_cmac_aes256_encrypt_message
#define siv_cmac_aes256_decrypt_message nettle_siv_cmac_aes256_decrypt_message

/* For SIV, the block size of the underlying cipher shall be 128 bits. */
#define SIV_BLOCK_SIZE  16
#define SIV_DIGEST_SIZE 16
#define SIV_MIN_NONCE_SIZE 1

void
siv_cmac_set_key(struct cmac128_key *cmac_key, void *cmac_cipher, void *ctr_cipher,
		 const struct nettle_cipher *nc,
		 const uint8_t *key);

void
siv_cmac_encrypt_message(const struct cmac128_key *cmac_key, const void *cmac_cipher_ctx,
			 const struct nettle_cipher *nc,
			 const void *ctr_ctx,
			 size_t nlength, const uint8_t *nonce,
			 size_t alength, const uint8_t *adata,
			 size_t clength, uint8_t *dst, const uint8_t *src);

int
siv_cmac_decrypt_message(const struct cmac128_key *cmac_key, const void *cmac_cipher,
			 const struct nettle_cipher *nc,
			 const void *ctr_cipher,
			 size_t nlength, const uint8_t *nonce,
			 size_t alength, const uint8_t *adata,
			 size_t mlength, uint8_t *dst, const uint8_t *src);

/*
 * SIV mode requires the aad and plaintext when building the IV, which
 * prevents streaming processing and it incompatible with the AEAD API.
 */

#define SIV_CMAC_CTX(type) { struct cmac128_key cmac_key; type cmac_cipher; type ctr_cipher; }

/* SIV_CMAC_AES128 */
#define SIV_CMAC_AES128_KEY_SIZE 32

struct siv_cmac_aes128_ctx SIV_CMAC_CTX(struct aes128_ctx);

void
siv_cmac_aes128_set_key(struct siv_cmac_aes128_ctx *ctx, const uint8_t *key);

void
siv_cmac_aes128_encrypt_message(const struct siv_cmac_aes128_ctx *ctx,
				size_t nlength, const uint8_t *nonce,
				size_t alength, const uint8_t *adata,
				size_t clength, uint8_t *dst, const uint8_t *src);

int
siv_cmac_aes128_decrypt_message(const struct siv_cmac_aes128_ctx *ctx,
				size_t nlength, const uint8_t *nonce,
				size_t alength, const uint8_t *adata,
				size_t mlength, uint8_t *dst, const uint8_t *src);

/* SIV_CMAC_AES256 */
#define SIV_CMAC_AES256_KEY_SIZE 64

struct siv_cmac_aes256_ctx SIV_CMAC_CTX(struct aes256_ctx);

void
siv_cmac_aes256_set_key(struct siv_cmac_aes256_ctx *ctx, const uint8_t *key);

void
siv_cmac_aes256_encrypt_message(const struct siv_cmac_aes256_ctx *ctx,
				size_t nlength, const uint8_t *nonce,
				size_t alength, const uint8_t *adata,
				size_t clength, uint8_t *dst, const uint8_t *src);

int
siv_cmac_aes256_decrypt_message(const struct siv_cmac_aes256_ctx *ctx,
				size_t nlength, const uint8_t *nonce,
				size_t alength, const uint8_t *adata,
				size_t mlength, uint8_t *dst, const uint8_t *src);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_SIV_H_INCLUDED */
