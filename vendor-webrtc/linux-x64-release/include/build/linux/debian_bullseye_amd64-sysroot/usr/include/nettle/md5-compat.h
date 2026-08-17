/* md5-compat.h

   The md5 hash function, RFC 1321-style interface.

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

#ifndef NETTLE_MD5_COMPAT_H_INCLUDED
#define NETTLE_MD5_COMPAT_H_INCLUDED

#include "md5.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define MD5Init nettle_MD5Init
#define MD5Update nettle_MD5Update
#define MD5Final nettle_MD5Final

typedef struct md5_ctx MD5_CTX;

void MD5Init(MD5_CTX *ctx);
void MD5Update(MD5_CTX *ctx, const unsigned char *data, unsigned int length);
void MD5Final(unsigned char *out, MD5_CTX *ctx);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_MD5_COMPAT_H_INCLUDED */
