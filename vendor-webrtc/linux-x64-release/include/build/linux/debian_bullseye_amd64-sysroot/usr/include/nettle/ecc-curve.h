/* ecc-curve.h

   Copyright (C) 2013 Niels Möller

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

/* Development of Nettle's ECC support was funded by the .SE Internet Fund. */

#ifndef NETTLE_ECC_CURVE_H_INCLUDED
#define NETTLE_ECC_CURVE_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* The contents of this struct is internal. */
struct ecc_curve;

const struct ecc_curve * _NETTLE_ATTRIBUTE_PURE nettle_get_gost_gc256b(void);
const struct ecc_curve * _NETTLE_ATTRIBUTE_PURE nettle_get_gost_gc512a(void);
const struct ecc_curve * _NETTLE_ATTRIBUTE_PURE nettle_get_secp_192r1(void);
const struct ecc_curve * _NETTLE_ATTRIBUTE_PURE nettle_get_secp_224r1(void);
const struct ecc_curve * _NETTLE_ATTRIBUTE_PURE nettle_get_secp_256r1(void);
const struct ecc_curve * _NETTLE_ATTRIBUTE_PURE nettle_get_secp_384r1(void);
const struct ecc_curve * _NETTLE_ATTRIBUTE_PURE nettle_get_secp_521r1(void);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_ECC_CURVE_H_INCLUDED */
