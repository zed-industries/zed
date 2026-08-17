/*
 * Copyright (C) 2003-2012 Free Software Foundation, Inc.
 * Copyright (C) 2015 Red Hat, Inc.
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

/* This file contains the types and prototypes for the X.509
 * certificate and CRL handling functions.
 */

#ifndef GNUTLS_PKCS7_H
#define GNUTLS_PKCS7_H

#include <gnutls/gnutls.h>
#include <gnutls/x509.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

/* PKCS7 structures handling
 */
struct gnutls_pkcs7_int;
typedef struct gnutls_pkcs7_int *gnutls_pkcs7_t;

int gnutls_pkcs7_init(gnutls_pkcs7_t * pkcs7);
void gnutls_pkcs7_deinit(gnutls_pkcs7_t pkcs7);
int gnutls_pkcs7_import(gnutls_pkcs7_t pkcs7,
			const gnutls_datum_t * data,
			gnutls_x509_crt_fmt_t format);
int gnutls_pkcs7_export(gnutls_pkcs7_t pkcs7,
			gnutls_x509_crt_fmt_t format,
			void *output_data, size_t * output_data_size);
int gnutls_pkcs7_export2(gnutls_pkcs7_t pkcs7,
			 gnutls_x509_crt_fmt_t format,
			 gnutls_datum_t * out);

int gnutls_pkcs7_get_signature_count(gnutls_pkcs7_t pkcs7);

#define GNUTLS_PKCS7_EDATA_GET_RAW (1<<24)
int gnutls_pkcs7_get_embedded_data(gnutls_pkcs7_t pkcs7, unsigned flags, gnutls_datum_t *data);

const char *
gnutls_pkcs7_get_embedded_data_oid(gnutls_pkcs7_t pkcs7);

int gnutls_pkcs7_get_crt_count(gnutls_pkcs7_t pkcs7);
int gnutls_pkcs7_get_crt_raw(gnutls_pkcs7_t pkcs7, unsigned indx,
			     void *certificate, size_t * certificate_size);

int gnutls_pkcs7_set_crt_raw(gnutls_pkcs7_t pkcs7,
			     const gnutls_datum_t * crt);
int gnutls_pkcs7_set_crt(gnutls_pkcs7_t pkcs7, gnutls_x509_crt_t crt);
int gnutls_pkcs7_delete_crt(gnutls_pkcs7_t pkcs7, int indx);

int gnutls_pkcs7_get_crl_raw(gnutls_pkcs7_t pkcs7,
			     unsigned indx, void *crl, size_t * crl_size);
int gnutls_pkcs7_get_crl_count(gnutls_pkcs7_t pkcs7);

int gnutls_pkcs7_set_crl_raw(gnutls_pkcs7_t pkcs7,
			     const gnutls_datum_t * crl);
int gnutls_pkcs7_set_crl(gnutls_pkcs7_t pkcs7, gnutls_x509_crl_t crl);
int gnutls_pkcs7_delete_crl(gnutls_pkcs7_t pkcs7, int indx);

typedef struct gnutls_pkcs7_attrs_st *gnutls_pkcs7_attrs_t;

typedef struct gnutls_pkcs7_signature_info_st {
	gnutls_sign_algorithm_t algo;
	gnutls_datum_t sig;
	gnutls_datum_t issuer_dn;
	gnutls_datum_t signer_serial;
	gnutls_datum_t issuer_keyid;
	time_t signing_time;
	gnutls_pkcs7_attrs_t signed_attrs;
	gnutls_pkcs7_attrs_t unsigned_attrs;
	char pad[64];
} gnutls_pkcs7_signature_info_st;

void gnutls_pkcs7_signature_info_deinit(gnutls_pkcs7_signature_info_st *info);
int gnutls_pkcs7_get_signature_info(gnutls_pkcs7_t pkcs7, unsigned idx, gnutls_pkcs7_signature_info_st *info);

int gnutls_pkcs7_verify_direct(gnutls_pkcs7_t pkcs7, gnutls_x509_crt_t signer,
			       unsigned idx, const gnutls_datum_t *data, unsigned flags);
int gnutls_pkcs7_verify(gnutls_pkcs7_t pkcs7, gnutls_x509_trust_list_t tl,
			gnutls_typed_vdata_st * vdata, unsigned int vdata_size,
			unsigned idx, const gnutls_datum_t *data, unsigned flags);

#define GNUTLS_PKCS7_ATTR_ENCODE_OCTET_STRING 1
int gnutls_pkcs7_add_attr(gnutls_pkcs7_attrs_t *list, const char *oid, gnutls_datum_t *data, unsigned flags);
void gnutls_pkcs7_attrs_deinit(gnutls_pkcs7_attrs_t list);
int gnutls_pkcs7_get_attr(gnutls_pkcs7_attrs_t list, unsigned idx, char **oid, gnutls_datum_t *data, unsigned flags);

/**
 * gnutls_pkcs7_sign_flags:
 * @GNUTLS_PKCS7_EMBED_DATA: The signed data will be embedded in the structure.
 * @GNUTLS_PKCS7_INCLUDE_TIME: The signing time will be included in the structure.
 * @GNUTLS_PKCS7_INCLUDE_CERT: The signer's certificate will be included in the cert list.
 * @GNUTLS_PKCS7_WRITE_SPKI: Use the signer's key identifier instead of name.
 *
 * Enumeration of the different PKCS #7 signature flags.
 */
typedef enum gnutls_pkcs7_sign_flags {
	GNUTLS_PKCS7_EMBED_DATA = 1,
	GNUTLS_PKCS7_INCLUDE_TIME = (1<<1),
	GNUTLS_PKCS7_INCLUDE_CERT = (1<<2),
	GNUTLS_PKCS7_WRITE_SPKI = (1<<3)
} gnutls_pkcs7_sign_flags;

int gnutls_pkcs7_sign(gnutls_pkcs7_t pkcs7,
		      gnutls_x509_crt_t signer,
		      gnutls_privkey_t signer_key,
		      const gnutls_datum_t *data,
		      gnutls_pkcs7_attrs_t signed_attrs,
		      gnutls_pkcs7_attrs_t unsigned_attrs,
		      gnutls_digest_algorithm_t dig,
		      unsigned flags);

int
gnutls_pkcs7_get_crt_raw2(gnutls_pkcs7_t pkcs7,
			  unsigned indx, gnutls_datum_t *cert);
int
gnutls_pkcs7_get_crl_raw2(gnutls_pkcs7_t pkcs7,
			  unsigned indx, gnutls_datum_t *crl);

int gnutls_pkcs7_print(gnutls_pkcs7_t pkcs7,
		       gnutls_certificate_print_formats_t format,
		       gnutls_datum_t * out);

int gnutls_pkcs7_print_signature_info(gnutls_pkcs7_signature_info_st * info,
				      gnutls_certificate_print_formats_t format,
				      gnutls_datum_t * out);

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_PKCS7_H */
