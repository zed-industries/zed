/*
 * Copyright (C) 2014 Red Hat, Inc.
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

#ifndef GNUTLS_URLS_H
#define GNUTLS_URLS_H

#include <gnutls/gnutls.h>
#include <gnutls/x509.h>
#include <gnutls/abstract.h>

/* This API allows to register application specific URLs for
 * keys and certificates.
 */

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

typedef int (*gnutls_privkey_import_url_func)(gnutls_privkey_t pkey,
					       const char *url, unsigned flags);

typedef int (*gnutls_x509_crt_import_url_func)(gnutls_x509_crt_t pkey,
					        const char *url, unsigned flags);

/* The following callbacks are optional */

/* This is to enable gnutls_pubkey_import_url() */
typedef int (*gnutls_pubkey_import_url_func)(gnutls_pubkey_t pkey,
					     const char *url, unsigned flags);

/* This is to allow constructing a certificate chain. It will be provided
 * the initial certificate URL and the certificate to find its issuer, and must
 * return zero and the DER encoding of the issuer's certificate. If not available,
 * it should return GNUTLS_E_REQUESTED_DATA_NOT_AVAILABLE. */
typedef int (*gnutls_get_raw_issuer_func)(const char *url, gnutls_x509_crt_t crt,
					  gnutls_datum_t *issuer_der, unsigned flags);

typedef struct gnutls_custom_url_st {
	const char *name;
	unsigned name_size;
	gnutls_privkey_import_url_func import_key;
	gnutls_x509_crt_import_url_func import_crt;
	gnutls_pubkey_import_url_func import_pubkey;
	gnutls_get_raw_issuer_func get_issuer;
	void *future1; /* replace in a future extension */
	void *future2; /* replace in a future extension */
} gnutls_custom_url_st;

int gnutls_register_custom_url(const gnutls_custom_url_st *st);

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_URLS_H */
