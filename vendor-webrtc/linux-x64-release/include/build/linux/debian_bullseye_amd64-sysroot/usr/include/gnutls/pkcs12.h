/*
 * Copyright (C) 2003-2012 Free Software Foundation, Inc.
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

#ifndef GNUTLS_PKCS12_H
#define GNUTLS_PKCS12_H

#include <gnutls/x509.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

	/* PKCS12 structures handling
	 */
struct gnutls_pkcs12_int;
typedef struct gnutls_pkcs12_int *gnutls_pkcs12_t;

struct gnutls_pkcs12_bag_int;
typedef struct gnutls_pkcs12_bag_int *gnutls_pkcs12_bag_t;

int gnutls_pkcs12_init(gnutls_pkcs12_t * pkcs12);
void gnutls_pkcs12_deinit(gnutls_pkcs12_t pkcs12);
int gnutls_pkcs12_import(gnutls_pkcs12_t pkcs12,
			 const gnutls_datum_t * data,
			 gnutls_x509_crt_fmt_t format, unsigned int flags);
int gnutls_pkcs12_export(gnutls_pkcs12_t pkcs12,
			 gnutls_x509_crt_fmt_t format,
			 void *output_data, size_t * output_data_size);
int gnutls_pkcs12_export2(gnutls_pkcs12_t pkcs12,
			  gnutls_x509_crt_fmt_t format,
			  gnutls_datum_t * out);

int gnutls_pkcs12_get_bag(gnutls_pkcs12_t pkcs12,
			  int indx, gnutls_pkcs12_bag_t bag);
int gnutls_pkcs12_set_bag(gnutls_pkcs12_t pkcs12, gnutls_pkcs12_bag_t bag);

int gnutls_pkcs12_generate_mac(gnutls_pkcs12_t pkcs12, const char *pass);
int gnutls_pkcs12_generate_mac2(gnutls_pkcs12_t pkcs12, gnutls_mac_algorithm_t mac, const char *pass);
int gnutls_pkcs12_verify_mac(gnutls_pkcs12_t pkcs12, const char *pass);

int gnutls_pkcs12_bag_decrypt(gnutls_pkcs12_bag_t bag, const char *pass);
int gnutls_pkcs12_bag_encrypt(gnutls_pkcs12_bag_t bag,
			      const char *pass, unsigned int flags);

int
gnutls_pkcs12_bag_enc_info(gnutls_pkcs12_bag_t bag, unsigned int *schema, unsigned int *cipher,
	void *salt, unsigned int *salt_size, unsigned int *iter_count, char **oid);
int
gnutls_pkcs12_mac_info(gnutls_pkcs12_t pkcs12, unsigned int *mac,
	void *salt, unsigned int *salt_size, unsigned int *iter_count, char **oid);

#define GNUTLS_PKCS12_SP_INCLUDE_SELF_SIGNED 1
int gnutls_pkcs12_simple_parse(gnutls_pkcs12_t p12,
			       const char *password,
			       gnutls_x509_privkey_t * key,
			       gnutls_x509_crt_t ** chain,
			       unsigned int *chain_len,
			       gnutls_x509_crt_t ** extra_certs,
			       unsigned int *extra_certs_len,
			       gnutls_x509_crl_t * crl,
			       unsigned int flags);

/**
 * gnutls_pkcs12_bag_type_t:
 * @GNUTLS_BAG_EMPTY: Empty PKCS-12 bag.
 * @GNUTLS_BAG_PKCS8_ENCRYPTED_KEY: PKCS-12 bag with PKCS-8 encrypted key.
 * @GNUTLS_BAG_PKCS8_KEY: PKCS-12 bag with PKCS-8 key.
 * @GNUTLS_BAG_CERTIFICATE: PKCS-12 bag with certificate.
 * @GNUTLS_BAG_CRL: PKCS-12 bag with CRL.
 * @GNUTLS_BAG_SECRET: PKCS-12 bag with secret PKCS-9 keys.
 * @GNUTLS_BAG_ENCRYPTED: Encrypted PKCS-12 bag.
 * @GNUTLS_BAG_UNKNOWN: Unknown PKCS-12 bag.
 *
 * Enumeration of different PKCS 12 bag types.
 */
typedef enum gnutls_pkcs12_bag_type_t {
	GNUTLS_BAG_EMPTY = 0,
	GNUTLS_BAG_PKCS8_ENCRYPTED_KEY = 1,
	GNUTLS_BAG_PKCS8_KEY = 2,
	GNUTLS_BAG_CERTIFICATE = 3,
	GNUTLS_BAG_CRL = 4,
	GNUTLS_BAG_SECRET = 5,	/* Secret data. Underspecified in pkcs-12,
				 * gnutls extension. We use the PKCS-9
				 * random nonce ID 1.2.840.113549.1.9.25.3
				 * to store randomly generated keys.
				 */
	GNUTLS_BAG_ENCRYPTED = 10,
	GNUTLS_BAG_UNKNOWN = 20
} gnutls_pkcs12_bag_type_t;

int
gnutls_pkcs12_bag_get_type(gnutls_pkcs12_bag_t bag, unsigned indx);
int gnutls_pkcs12_bag_get_data(gnutls_pkcs12_bag_t bag, unsigned indx,
			       gnutls_datum_t * data);
int gnutls_pkcs12_bag_set_data(gnutls_pkcs12_bag_t bag,
			       gnutls_pkcs12_bag_type_t type,
			       const gnutls_datum_t * data);
int gnutls_pkcs12_bag_set_crl(gnutls_pkcs12_bag_t bag,
			      gnutls_x509_crl_t crl);
int gnutls_pkcs12_bag_set_crt(gnutls_pkcs12_bag_t bag,
			      gnutls_x509_crt_t crt);

int
gnutls_pkcs12_bag_set_privkey(gnutls_pkcs12_bag_t bag,
			      gnutls_x509_privkey_t privkey,
			      const char *password, unsigned flags);

int gnutls_pkcs12_bag_init(gnutls_pkcs12_bag_t * bag);
void gnutls_pkcs12_bag_deinit(gnutls_pkcs12_bag_t bag);
int gnutls_pkcs12_bag_get_count(gnutls_pkcs12_bag_t bag);

int gnutls_pkcs12_bag_get_key_id(gnutls_pkcs12_bag_t bag, unsigned indx,
				 gnutls_datum_t * id);
int gnutls_pkcs12_bag_set_key_id(gnutls_pkcs12_bag_t bag, unsigned indx,
				 const gnutls_datum_t * id);

int gnutls_pkcs12_bag_get_friendly_name(gnutls_pkcs12_bag_t bag,
					unsigned indx, char **name);
int gnutls_pkcs12_bag_set_friendly_name(gnutls_pkcs12_bag_t bag,
					unsigned indx, const char *name);

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_PKCS12_H */
