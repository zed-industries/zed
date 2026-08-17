/*
 * Copyright (C) 2010-2012 Free Software Foundation, Inc.
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

#ifndef GNUTLS_TPM_H
#define GNUTLS_TPM_H

#include <gnutls/gnutls.h>
#include <gnutls/x509.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

struct tpm_key_list_st;
typedef struct tpm_key_list_st *gnutls_tpm_key_list_t;

#define GNUTLS_TPM_KEY_SIGNING (1<<1)
#define GNUTLS_TPM_REGISTER_KEY (1<<2)
#define GNUTLS_TPM_KEY_USER (1<<3)

/**
 * gnutls_tpmkey_fmt_t:
 * @GNUTLS_TPMKEY_FMT_RAW: The portable data format.
 * @GNUTLS_TPMKEY_FMT_DER: An alias for the raw format.
 * @GNUTLS_TPMKEY_FMT_CTK_PEM: A custom data format used by some TPM tools.
 *
 * Enumeration of different certificate encoding formats.
 */
typedef enum {
	GNUTLS_TPMKEY_FMT_RAW = 0,
	GNUTLS_TPMKEY_FMT_DER = GNUTLS_TPMKEY_FMT_RAW,
	GNUTLS_TPMKEY_FMT_CTK_PEM = 1
} gnutls_tpmkey_fmt_t;

int
gnutls_tpm_privkey_generate(gnutls_pk_algorithm_t pk,
			    unsigned int bits,
			    const char *srk_password,
			    const char *key_password,
			    gnutls_tpmkey_fmt_t format,
			    gnutls_x509_crt_fmt_t pub_format,
			    gnutls_datum_t * privkey,
			    gnutls_datum_t * pubkey, unsigned int flags);

void gnutls_tpm_key_list_deinit(gnutls_tpm_key_list_t list);
int gnutls_tpm_key_list_get_url(gnutls_tpm_key_list_t list,
				unsigned int idx, char **url,
				unsigned int flags);
int gnutls_tpm_get_registered(gnutls_tpm_key_list_t * list);
int gnutls_tpm_privkey_delete(const char *url, const char *srk_password);


/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_TPM_H */
