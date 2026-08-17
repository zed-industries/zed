/* pss.h

   PKCS#1 RSA-PSS (RFC-3447).

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

#ifndef NETTLE_PSS_H_INCLUDED
#define NETTLE_PSS_H_INCLUDED

#include "nettle-meta.h"
#include "bignum.h"

#ifdef __cplusplus
extern "C"
{
#endif

/* Namespace mangling */
#define pss_encode_mgf1 nettle_pss_encode_mgf1
#define pss_verify_mgf1 nettle_pss_verify_mgf1

int
pss_encode_mgf1(mpz_t m, size_t bits,
		const struct nettle_hash *hash,
		size_t salt_length, const uint8_t *salt,
		const uint8_t *digest);

int
pss_verify_mgf1(const mpz_t m, size_t bits,
		const struct nettle_hash *hash,
		size_t salt_length,
		const uint8_t *digest);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_PSS_H_INCLUDED */
