/* cfb.h

   Cipher feedback mode.

   Copyright (C) 2015, 2017 Dmitry Eremin-Solenikov
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

#ifndef NETTLE_CFB_H_INCLUDED
#define NETTLE_CFB_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define cfb_encrypt nettle_cfb_encrypt
#define cfb_decrypt nettle_cfb_decrypt

#define cfb8_encrypt nettle_cfb8_encrypt
#define cfb8_decrypt nettle_cfb8_decrypt

void
cfb_encrypt(const void *ctx, nettle_cipher_func *f,
	    size_t block_size, uint8_t *iv,
	    size_t length, uint8_t *dst,
	    const uint8_t *src);

void
cfb_decrypt(const void *ctx, nettle_cipher_func *f,
	    size_t block_size, uint8_t *iv,
	    size_t length, uint8_t *dst,
	    const uint8_t *src);

void
cfb8_encrypt(const void *ctx, nettle_cipher_func *f,
	     size_t block_size, uint8_t *iv,
	     size_t length, uint8_t *dst,
	     const uint8_t *src);

void
cfb8_decrypt(const void *ctx, nettle_cipher_func *f,
	     size_t block_size, uint8_t *iv,
	     size_t length, uint8_t *dst,
	     const uint8_t *src);


#define CFB_CTX(type, size) \
{ type ctx; uint8_t iv[size]; }

#define CFB_SET_IV(ctx, data) \
memcpy((ctx)->iv, (data), sizeof((ctx)->iv))

#define CFB8_CTX CFB_CTX
#define CFB8_SET_IV CFB_SET_IV

/* NOTE: Avoid using NULL, as we don't include anything defining it. */
#define CFB_ENCRYPT(self, f, length, dst, src)		\
  (0 ? ((f)(&(self)->ctx, ~(size_t) 0,			\
	    (uint8_t *) 0, (const uint8_t *) 0))	\
   : cfb_encrypt((void *) &(self)->ctx,			\
		 (nettle_cipher_func *) (f),		\
		 sizeof((self)->iv), (self)->iv,	\
		 (length), (dst), (src)))

#define CFB_DECRYPT(self, f, length, dst, src)		\
  (0 ? ((f)(&(self)->ctx, ~(size_t) 0,			\
	    (uint8_t *) 0, (const uint8_t *) 0))	\
   : cfb_decrypt((void *) &(self)->ctx,			\
		 (nettle_cipher_func *) (f),		\
		 sizeof((self)->iv), (self)->iv,	\
		 (length), (dst), (src)))

#define CFB8_ENCRYPT(self, f, length, dst, src)		\
  (0 ? ((f)(&(self)->ctx, ~(size_t) 0,			\
	    (uint8_t *) 0, (const uint8_t *) 0))	\
   : cfb8_encrypt((void *) &(self)->ctx,		\
		  (nettle_cipher_func *) (f),		\
		  sizeof((self)->iv), (self)->iv,	\
		  (length), (dst), (src)))

#define CFB8_DECRYPT(self, f, length, dst, src)		\
  (0 ? ((f)(&(self)->ctx, ~(size_t) 0,			\
	    (uint8_t *) 0, (const uint8_t *) 0))	\
   : cfb8_decrypt((void *) &(self)->ctx,		\
		  (nettle_cipher_func *) (f),		\
		  sizeof((self)->iv), (self)->iv,	\
		  (length), (dst), (src)))

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_CFB_H_INCLUDED */
