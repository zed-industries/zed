/* pss-mgf1.h

   PKCS#1 mask generation function 1, used in RSA-PSS (RFC-3447).

   Copyright (C) 2017 Daiki Ueno

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

#ifndef NETTLE_PSS_MGF1_H_INCLUDED
#define NETTLE_PSS_MGF1_H_INCLUDED

#include "nettle-meta.h"

#include "sha1.h"
#include "sha2.h"

#ifdef __cplusplus
extern "C"
{
#endif

/* Namespace mangling */
#define pss_mgf1 nettle_pss_mgf1

void
pss_mgf1(const void *seed, const struct nettle_hash *hash,
	 size_t length, uint8_t *mask);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_PSS_MGF1_H_INCLUDED */
