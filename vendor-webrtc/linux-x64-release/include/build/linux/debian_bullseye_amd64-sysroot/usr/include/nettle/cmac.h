/* cmac.h

   CMAC mode, as specified in RFC4493

   Copyright (C) 2017 Red Hat, Inc.

   Contributed by Nikos Mavrogiannopoulos

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

#ifndef NETTLE_CMAC_H_INCLUDED
#define NETTLE_CMAC_H_INCLUDED

#include "aes.h"
#include "des.h"
#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

#define CMAC128_DIGEST_SIZE 16
#define CMAC64_DIGEST_SIZE 8

#define cmac128_set_key nettle_cmac128_set_key
#define cmac128_init nettle_cmac128_init
#define cmac128_update nettle_cmac128_update
#define cmac128_digest nettle_cmac128_digest
#define cmac_aes128_set_key nettle_cmac_aes128_set_key
#define cmac_aes128_update nettle_cmac_aes128_update
#define cmac_aes128_digest nettle_cmac_aes128_digest
#define cmac_aes256_set_key nettle_cmac_aes256_set_key
#define cmac_aes256_update nettle_cmac_aes256_update
#define cmac_aes256_digest nettle_cmac_aes256_digest

#define cmac64_set_key nettle_cmac64_set_key
#define cmac64_init nettle_cmac64_init
#define cmac64_update nettle_cmac64_update
#define cmac64_digest nettle_cmac64_digest
#define cmac_des3_set_key nettle_cmac_des3_set_key
#define cmac_des3_update nettle_cmac_des3_update
#define cmac_des3_digest nettle_cmac_des3_digest

struct cmac128_key
{
  union nettle_block16 K1;
  union nettle_block16 K2;
};

struct cmac128_ctx
{
  /* MAC state */
  union nettle_block16 X;

  /* Block buffer */
  union nettle_block16 block;
  size_t index;
};

struct cmac64_key
{
  union nettle_block8 K1;
  union nettle_block8 K2;
};

struct cmac64_ctx
{
  /* MAC state */
  union nettle_block8 X;

  /* Block buffer */
  union nettle_block8 block;
  size_t index;
};

void
cmac128_set_key(struct cmac128_key *key, const void *cipher,
		nettle_cipher_func *encrypt);

void
cmac128_init(struct cmac128_ctx *ctx);

void
cmac128_update(struct cmac128_ctx *ctx, const void *cipher,
	       nettle_cipher_func *encrypt,
	       size_t msg_len, const uint8_t *msg);
void
cmac128_digest(struct cmac128_ctx *ctx, const struct cmac128_key *key,
	       const void *cipher, nettle_cipher_func *encrypt,
	       unsigned length, uint8_t *digest);


#define CMAC128_CTX(type) \
  { struct cmac128_key key; struct cmac128_ctx ctx; type cipher; }

/* NOTE: Avoid using NULL, as we don't include anything defining it. */
#define CMAC128_SET_KEY(self, set_key, encrypt, cmac_key)	\
  do {								\
    (set_key)(&(self)->cipher, (cmac_key));			\
    if (0) (encrypt)(&(self)->cipher, ~(size_t) 0,		\
		     (uint8_t *) 0, (const uint8_t *) 0);	\
    cmac128_set_key(&(self)->key, &(self)->cipher,		\
		    (nettle_cipher_func *) (encrypt));		\
    cmac128_init(&(self)->ctx);					\
  } while (0)

#define CMAC128_UPDATE(self, encrypt, length, src)		\
  (0 ? (encrypt)(&(self)->cipher, ~(size_t) 0,			\
		 (uint8_t *) 0, (const uint8_t *) 0)		\
     : cmac128_update(&(self)->ctx, &(self)->cipher,		\
		      (nettle_cipher_func *)encrypt,		\
		      (length), (src)))

#define CMAC128_DIGEST(self, encrypt, length, digest)		\
  (0 ? (encrypt)(&(self)->cipher, ~(size_t) 0,			\
		 (uint8_t *) 0, (const uint8_t *) 0)		\
     : cmac128_digest(&(self)->ctx, &(self)->key,		\
		      &(self)->cipher,				\
		      (nettle_cipher_func *) (encrypt),		\
		      (length), (digest)))

void
cmac64_set_key(struct cmac64_key *key, const void *cipher,
		nettle_cipher_func *encrypt);

void
cmac64_init(struct cmac64_ctx *ctx);

void
cmac64_update(struct cmac64_ctx *ctx, const void *cipher,
	       nettle_cipher_func *encrypt,
	       size_t msg_len, const uint8_t *msg);

void
cmac64_digest(struct cmac64_ctx *ctx, const struct cmac64_key *key,
	       const void *cipher, nettle_cipher_func *encrypt,
	       unsigned length, uint8_t *digest);


#define CMAC64_CTX(type) \
  { struct cmac64_key key; struct cmac64_ctx ctx; type cipher; }

/* NOTE: Avoid using NULL, as we don't include anything defining it. */
#define CMAC64_SET_KEY(self, set_key, encrypt, cmac_key)	\
  do {								\
    (set_key)(&(self)->cipher, (cmac_key));			\
    if (0) (encrypt)(&(self)->cipher, ~(size_t) 0,		\
		     (uint8_t *) 0, (const uint8_t *) 0);	\
    cmac64_set_key(&(self)->key, &(self)->cipher,		\
		    (nettle_cipher_func *) (encrypt));		\
    cmac64_init(&(self)->ctx);					\
  } while (0)

#define CMAC64_UPDATE(self, encrypt, length, src)		\
  (0 ? (encrypt)(&(self)->cipher, ~(size_t) 0,			\
		 (uint8_t *) 0, (const uint8_t *) 0)		\
     : cmac64_update(&(self)->ctx, &(self)->cipher,		\
		      (nettle_cipher_func *)encrypt,		\
		      (length), (src)))

#define CMAC64_DIGEST(self, encrypt, length, digest)		\
  (0 ? (encrypt)(&(self)->cipher, ~(size_t) 0,			\
		 (uint8_t *) 0, (const uint8_t *) 0)		\
     : cmac64_digest(&(self)->ctx, &(self)->key,		\
		      &(self)->cipher,				\
		      (nettle_cipher_func *) (encrypt),		\
		      (length), (digest)))

struct cmac_aes128_ctx CMAC128_CTX(struct aes128_ctx);

void
cmac_aes128_set_key(struct cmac_aes128_ctx *ctx, const uint8_t *key);

void
cmac_aes128_update(struct cmac_aes128_ctx *ctx,
		   size_t length, const uint8_t *data);

void
cmac_aes128_digest(struct cmac_aes128_ctx *ctx,
		   size_t length, uint8_t *digest);

struct cmac_aes256_ctx CMAC128_CTX(struct aes256_ctx);

void
cmac_aes256_set_key(struct cmac_aes256_ctx *ctx, const uint8_t *key);

void
cmac_aes256_update(struct cmac_aes256_ctx *ctx,
		   size_t length, const uint8_t *data);

void
cmac_aes256_digest(struct cmac_aes256_ctx *ctx,
		   size_t length, uint8_t *digest);

struct cmac_des3_ctx CMAC64_CTX(struct des3_ctx);

void
cmac_des3_set_key(struct cmac_des3_ctx *ctx, const uint8_t *key);

void
cmac_des3_update(struct cmac_des3_ctx *ctx,
		 size_t length, const uint8_t *data);

void
cmac_des3_digest(struct cmac_des3_ctx *ctx,
		 size_t length, uint8_t *digest);

#ifdef __cplusplus
}
#endif

#endif /* CMAC_H_INCLUDED */
