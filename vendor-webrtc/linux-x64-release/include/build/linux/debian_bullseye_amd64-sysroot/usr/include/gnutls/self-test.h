/*
 * Copyright (C) 2014 Free Software Foundation, Inc.
 *
 * Author: Nikos Mavrogiannopoulos
 *
 * This file is part of GnuTLS.
 *
 * The GnuTLS is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>
 *
 */

#ifndef GNUTLS_SELF_TEST_H
#define GNUTLS_SELF_TEST_H

#include <gnutls/gnutls.h>

 /* Self checking functions */
 
#define GNUTLS_SELF_TEST_FLAG_ALL 1
#define GNUTLS_SELF_TEST_FLAG_NO_COMPAT (1<<1)

int gnutls_cipher_self_test(unsigned flags, gnutls_cipher_algorithm_t cipher);
int gnutls_mac_self_test(unsigned flags, gnutls_mac_algorithm_t mac);
int gnutls_digest_self_test(unsigned flags, gnutls_digest_algorithm_t digest);
int gnutls_pk_self_test(unsigned flags, gnutls_pk_algorithm_t pk);
int gnutls_hkdf_self_test(unsigned flags, gnutls_mac_algorithm_t mac);
int gnutls_pbkdf2_self_test(unsigned flags, gnutls_mac_algorithm_t mac);
int gnutls_tlsprf_self_test(unsigned flags, gnutls_mac_algorithm_t mac);

#endif /* GNUTLS_SELF_TEST_H */
