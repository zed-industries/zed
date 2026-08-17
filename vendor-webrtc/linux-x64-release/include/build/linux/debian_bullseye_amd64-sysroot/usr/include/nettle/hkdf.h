/* hkdf.h

   TLS PRF code (RFC-5246, RFC-2246).

   Copyright (C) 2017 Red Hat, Inc.

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

#ifndef NETTLE_HKDF_H_INCLUDED
#define NETTLE_HKDF_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Namespace mangling */
#define hkdf_extract nettle_hkdf_extract
#define hkdf_expand nettle_hkdf_expand

void
hkdf_extract(void *mac_ctx,
	     nettle_hash_update_func *update,
	     nettle_hash_digest_func *digest,
	     size_t digest_size,
	     size_t secret_size, const uint8_t *secret,
	     uint8_t *dst);

void
hkdf_expand(void *mac_ctx,
	    nettle_hash_update_func *update,
	    nettle_hash_digest_func *digest,
	    size_t digest_size,
	    size_t info_size, const uint8_t *info,
	    size_t length, uint8_t *dst);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_HKDF_H_INCLUDED */
