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

#ifndef GNUTLS_SYSTEM_KEYS_H
#define GNUTLS_SYSTEM_KEYS_H

#include <gnutls/gnutls.h>
#include <gnutls/x509.h>

/* This API allows to access user key and certificate pairs that are
 * available in the current system. If any passwords are required,
 * they will be requested through the pin callbacks.
 */

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

struct system_key_iter_st;
typedef struct system_key_iter_st *gnutls_system_key_iter_t;

void gnutls_system_key_iter_deinit(gnutls_system_key_iter_t iter);
int
gnutls_system_key_iter_get_info(gnutls_system_key_iter_t *iter,
			       unsigned cert_type /* gnutls_certificate_type_t */,
			       char **cert_url,
			       char **key_url,
			       char **label,
			       gnutls_datum_t *der,
			       unsigned int flags);

int gnutls_system_key_delete(const char *cert_url, const char *key_url);

int gnutls_system_key_add_x509(gnutls_x509_crt_t crt, gnutls_x509_privkey_t privkey,
				const char *label, char **cert_url, char **key_url);

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_SYSTEM_KEYS_H */
