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

/* This file contains the types and prototypes for the OpenPGP
 * key and private key parsing functions.
 */

#ifndef GNUTLS_OPENPGP_H
#define GNUTLS_OPENPGP_H

#include <gnutls/gnutls.h>
#include <limits.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

/* Openpgp certificate stuff
 */

/**
 * gnutls_openpgp_crt_fmt_t:
 * @GNUTLS_OPENPGP_FMT_RAW: OpenPGP certificate in raw format.
 * @GNUTLS_OPENPGP_FMT_BASE64: OpenPGP certificate in base64 format.
 *
 * Enumeration of different OpenPGP key formats.
 */
typedef enum gnutls_openpgp_crt_fmt {
	GNUTLS_OPENPGP_FMT_RAW,
	GNUTLS_OPENPGP_FMT_BASE64
} gnutls_openpgp_crt_fmt_t;

#define GNUTLS_OPENPGP_KEYID_SIZE 8
#define GNUTLS_OPENPGP_V4_FINGERPRINT_SIZE 20
typedef unsigned char
 gnutls_openpgp_keyid_t[GNUTLS_OPENPGP_KEYID_SIZE];

/* gnutls_openpgp_cert_t should be defined in gnutls.h
 */

	/* initializes the memory for gnutls_openpgp_crt_t struct */
int gnutls_openpgp_crt_init(gnutls_openpgp_crt_t * key) _GNUTLS_GCC_ATTR_DEPRECATED;
	/* frees all memory */
void gnutls_openpgp_crt_deinit(gnutls_openpgp_crt_t key) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_crt_import(gnutls_openpgp_crt_t key,
			      const gnutls_datum_t * data,
			      gnutls_openpgp_crt_fmt_t format) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_crt_export(gnutls_openpgp_crt_t key,
			      gnutls_openpgp_crt_fmt_t format,
			      void *output_data,
			      size_t * output_data_size) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_crt_export2(gnutls_openpgp_crt_t key,
			       gnutls_openpgp_crt_fmt_t format,
			       gnutls_datum_t * out) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_crt_print(gnutls_openpgp_crt_t cert,
			     gnutls_certificate_print_formats_t
			     format, gnutls_datum_t * out) _GNUTLS_GCC_ATTR_DEPRECATED;

/* The key_usage flags are defined in gnutls.h. They are
 * the GNUTLS_KEY_* definitions.
 */
#define GNUTLS_OPENPGP_MASTER_KEYID_IDX INT_MAX

int gnutls_openpgp_crt_get_key_usage(gnutls_openpgp_crt_t key,
				     unsigned int *key_usage) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_crt_get_fingerprint(gnutls_openpgp_crt_t key,
				       void *fpr, size_t * fprlen) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_crt_get_subkey_fingerprint(gnutls_openpgp_crt_t
					      key,
					      unsigned int idx,
					      void *fpr, size_t * fprlen) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_crt_get_name(gnutls_openpgp_crt_t key,
				int idx, char *buf, size_t * sizeof_buf) _GNUTLS_GCC_ATTR_DEPRECATED;

gnutls_pk_algorithm_t
gnutls_openpgp_crt_get_pk_algorithm(gnutls_openpgp_crt_t key,
				    unsigned int *bits) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_crt_get_version(gnutls_openpgp_crt_t key) _GNUTLS_GCC_ATTR_DEPRECATED;

time_t gnutls_openpgp_crt_get_creation_time(gnutls_openpgp_crt_t key) _GNUTLS_GCC_ATTR_DEPRECATED;
time_t gnutls_openpgp_crt_get_expiration_time(gnutls_openpgp_crt_t key) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_crt_get_key_id(gnutls_openpgp_crt_t key,
				  gnutls_openpgp_keyid_t keyid) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_crt_check_hostname(gnutls_openpgp_crt_t key,
				      const char *hostname) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_crt_check_hostname2(gnutls_openpgp_crt_t key,
				      const char *hostname, unsigned int flags) _GNUTLS_GCC_ATTR_DEPRECATED;
int
gnutls_openpgp_crt_check_email(gnutls_openpgp_crt_t key, const char *email, unsigned flags) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_crt_get_revoked_status(gnutls_openpgp_crt_t key) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_crt_get_subkey_count(gnutls_openpgp_crt_t key) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_crt_get_subkey_idx(gnutls_openpgp_crt_t key,
				      const gnutls_openpgp_keyid_t keyid) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_crt_get_subkey_revoked_status
    (gnutls_openpgp_crt_t key, unsigned int idx) _GNUTLS_GCC_ATTR_DEPRECATED;
gnutls_pk_algorithm_t
gnutls_openpgp_crt_get_subkey_pk_algorithm(gnutls_openpgp_crt_t
					   key,
					   unsigned int idx,
					   unsigned int *bits) _GNUTLS_GCC_ATTR_DEPRECATED;
time_t
    gnutls_openpgp_crt_get_subkey_creation_time
    (gnutls_openpgp_crt_t key, unsigned int idx) _GNUTLS_GCC_ATTR_DEPRECATED;
time_t
    gnutls_openpgp_crt_get_subkey_expiration_time
    (gnutls_openpgp_crt_t key, unsigned int idx) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_crt_get_subkey_id(gnutls_openpgp_crt_t key,
				     unsigned int idx,
				     gnutls_openpgp_keyid_t keyid) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_crt_get_subkey_usage(gnutls_openpgp_crt_t key,
					unsigned int idx,
					unsigned int *key_usage) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_crt_get_subkey_pk_dsa_raw(gnutls_openpgp_crt_t
					     crt, unsigned int idx,
					     gnutls_datum_t * p,
					     gnutls_datum_t * q,
					     gnutls_datum_t * g,
					     gnutls_datum_t * y) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_crt_get_subkey_pk_rsa_raw(gnutls_openpgp_crt_t
					     crt, unsigned int idx,
					     gnutls_datum_t * m,
					     gnutls_datum_t * e) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_crt_get_pk_dsa_raw(gnutls_openpgp_crt_t crt,
				      gnutls_datum_t * p,
				      gnutls_datum_t * q,
				      gnutls_datum_t * g,
				      gnutls_datum_t * y) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_crt_get_pk_rsa_raw(gnutls_openpgp_crt_t crt,
				      gnutls_datum_t * m,
				      gnutls_datum_t * e) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_crt_get_preferred_key_id(gnutls_openpgp_crt_t
					    key,
					    gnutls_openpgp_keyid_t keyid) _GNUTLS_GCC_ATTR_DEPRECATED;
int
gnutls_openpgp_crt_set_preferred_key_id(gnutls_openpgp_crt_t key,
					const
					gnutls_openpgp_keyid_t keyid) _GNUTLS_GCC_ATTR_DEPRECATED;

/* privkey stuff.
 */
int gnutls_openpgp_privkey_init(gnutls_openpgp_privkey_t * key) _GNUTLS_GCC_ATTR_DEPRECATED;
void gnutls_openpgp_privkey_deinit(gnutls_openpgp_privkey_t key) _GNUTLS_GCC_ATTR_DEPRECATED;
gnutls_pk_algorithm_t
    gnutls_openpgp_privkey_get_pk_algorithm
    (gnutls_openpgp_privkey_t key, unsigned int *bits) _GNUTLS_GCC_ATTR_DEPRECATED;

gnutls_sec_param_t
gnutls_openpgp_privkey_sec_param(gnutls_openpgp_privkey_t key) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_privkey_import(gnutls_openpgp_privkey_t key,
				  const gnutls_datum_t * data,
				  gnutls_openpgp_crt_fmt_t format,
				  const char *password,
				  unsigned int flags) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_privkey_get_fingerprint(gnutls_openpgp_privkey_t
					   key, void *fpr,
					   size_t * fprlen) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_privkey_get_subkey_fingerprint
    (gnutls_openpgp_privkey_t key, unsigned int idx, void *fpr,
     size_t * fprlen) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_privkey_get_key_id(gnutls_openpgp_privkey_t key,
				      gnutls_openpgp_keyid_t keyid) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_privkey_get_subkey_count(gnutls_openpgp_privkey_t key) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_privkey_get_subkey_idx(gnutls_openpgp_privkey_t
					  key,
					  const
					  gnutls_openpgp_keyid_t keyid) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_privkey_get_subkey_revoked_status
    (gnutls_openpgp_privkey_t key, unsigned int idx) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_privkey_get_revoked_status
    (gnutls_openpgp_privkey_t key) _GNUTLS_GCC_ATTR_DEPRECATED;

gnutls_pk_algorithm_t
    gnutls_openpgp_privkey_get_subkey_pk_algorithm
    (gnutls_openpgp_privkey_t key, unsigned int idx, unsigned int *bits) _GNUTLS_GCC_ATTR_DEPRECATED;

time_t
    gnutls_openpgp_privkey_get_subkey_expiration_time
    (gnutls_openpgp_privkey_t key, unsigned int idx) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_privkey_get_subkey_id(gnutls_openpgp_privkey_t
					 key, unsigned int idx,
					 gnutls_openpgp_keyid_t keyid) _GNUTLS_GCC_ATTR_DEPRECATED;

time_t
    gnutls_openpgp_privkey_get_subkey_creation_time
    (gnutls_openpgp_privkey_t key, unsigned int idx) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_privkey_export_subkey_dsa_raw
    (gnutls_openpgp_privkey_t pkey, unsigned int idx,
     gnutls_datum_t * p, gnutls_datum_t * q, gnutls_datum_t * g,
     gnutls_datum_t * y, gnutls_datum_t * x) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_privkey_export_subkey_rsa_raw
    (gnutls_openpgp_privkey_t pkey, unsigned int idx,
     gnutls_datum_t * m, gnutls_datum_t * e, gnutls_datum_t * d,
     gnutls_datum_t * p, gnutls_datum_t * q, gnutls_datum_t * u) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_privkey_export_dsa_raw(gnutls_openpgp_privkey_t
					  pkey, gnutls_datum_t * p,
					  gnutls_datum_t * q,
					  gnutls_datum_t * g,
					  gnutls_datum_t * y,
					  gnutls_datum_t * x) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_privkey_export_rsa_raw(gnutls_openpgp_privkey_t
					  pkey, gnutls_datum_t * m,
					  gnutls_datum_t * e,
					  gnutls_datum_t * d,
					  gnutls_datum_t * p,
					  gnutls_datum_t * q,
					  gnutls_datum_t * u) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_privkey_export(gnutls_openpgp_privkey_t key,
				  gnutls_openpgp_crt_fmt_t format,
				  const char *password,
				  unsigned int flags,
				  void *output_data,
				  size_t * output_data_size) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_privkey_export2(gnutls_openpgp_privkey_t key,
				   gnutls_openpgp_crt_fmt_t format,
				   const char *password,
				   unsigned int flags,
				   gnutls_datum_t * out) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_privkey_set_preferred_key_id
    (gnutls_openpgp_privkey_t key, const gnutls_openpgp_keyid_t keyid) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_openpgp_privkey_get_preferred_key_id
    (gnutls_openpgp_privkey_t key, gnutls_openpgp_keyid_t keyid) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_crt_get_auth_subkey(gnutls_openpgp_crt_t crt,
				       gnutls_openpgp_keyid_t
				       keyid, unsigned int flag) _GNUTLS_GCC_ATTR_DEPRECATED;

/* Keyring stuff.
 */

int gnutls_openpgp_keyring_init(gnutls_openpgp_keyring_t * keyring) _GNUTLS_GCC_ATTR_DEPRECATED;
void gnutls_openpgp_keyring_deinit(gnutls_openpgp_keyring_t keyring) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_keyring_import(gnutls_openpgp_keyring_t keyring,
				  const gnutls_datum_t * data,
				  gnutls_openpgp_crt_fmt_t format) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_keyring_check_id(gnutls_openpgp_keyring_t ring,
				    const gnutls_openpgp_keyid_t
				    keyid, unsigned int flags) _GNUTLS_GCC_ATTR_DEPRECATED;


int gnutls_openpgp_crt_verify_ring(gnutls_openpgp_crt_t key,
				   gnutls_openpgp_keyring_t
				   keyring, unsigned int flags,
				   unsigned int *verify
				   /* the output of the verification */
    ) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_crt_verify_self(gnutls_openpgp_crt_t key,
				   unsigned int flags,
				   unsigned int *verify) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_keyring_get_crt(gnutls_openpgp_keyring_t ring,
				   unsigned int idx,
				   gnutls_openpgp_crt_t * cert) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_keyring_get_crt_count(gnutls_openpgp_keyring_t ring) _GNUTLS_GCC_ATTR_DEPRECATED;



/**
 * gnutls_openpgp_recv_key_func:
 * @session: a TLS session
 * @keyfpr: key fingerprint
 * @keyfpr_length: length of key fingerprint
 * @key: output key.
 *
 * A callback of this type is used to retrieve OpenPGP keys.  Only
 * useful on the server, and will only be used if the peer send a key
 * fingerprint instead of a full key.  See also
 * gnutls_openpgp_set_recv_key_function().
 *
 * The variable @key must be allocated using gnutls_malloc().
 *
 * Returns: On success, %GNUTLS_E_SUCCESS (zero) is returned,
 *   otherwise an error code is returned.
 */
typedef int (*gnutls_openpgp_recv_key_func) (gnutls_session_t
					     session,
					     const unsigned char
					     *keyfpr,
					     unsigned int
					     keyfpr_length,
					     gnutls_datum_t * key);

void
gnutls_openpgp_set_recv_key_function(gnutls_session_t session,
				     gnutls_openpgp_recv_key_func func) _GNUTLS_GCC_ATTR_DEPRECATED;



/* certificate authentication stuff.
 */
int gnutls_certificate_set_openpgp_key
    (gnutls_certificate_credentials_t res,
     gnutls_openpgp_crt_t crt, gnutls_openpgp_privkey_t pkey) _GNUTLS_GCC_ATTR_DEPRECATED;

int
gnutls_certificate_get_openpgp_key(gnutls_certificate_credentials_t res,
                                   unsigned index,
                                   gnutls_openpgp_privkey_t *key) _GNUTLS_GCC_ATTR_DEPRECATED;
int
gnutls_certificate_get_openpgp_crt(gnutls_certificate_credentials_t res,
                                   unsigned index,
                                   gnutls_openpgp_crt_t **crt_list,
                                   unsigned *crt_list_size) _GNUTLS_GCC_ATTR_DEPRECATED;

int
 gnutls_certificate_set_openpgp_key_file
    (gnutls_certificate_credentials_t res, const char *certfile,
     const char *keyfile, gnutls_openpgp_crt_fmt_t format) _GNUTLS_GCC_ATTR_DEPRECATED;
int gnutls_certificate_set_openpgp_key_mem
    (gnutls_certificate_credentials_t res,
     const gnutls_datum_t * cert, const gnutls_datum_t * key,
     gnutls_openpgp_crt_fmt_t format) _GNUTLS_GCC_ATTR_DEPRECATED;

int
 gnutls_certificate_set_openpgp_key_file2
    (gnutls_certificate_credentials_t res, const char *certfile,
     const char *keyfile, const char *subkey_id,
     gnutls_openpgp_crt_fmt_t format) _GNUTLS_GCC_ATTR_DEPRECATED;
int
 gnutls_certificate_set_openpgp_key_mem2
    (gnutls_certificate_credentials_t res,
     const gnutls_datum_t * cert, const gnutls_datum_t * key,
     const char *subkey_id, gnutls_openpgp_crt_fmt_t format) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_certificate_set_openpgp_keyring_mem
    (gnutls_certificate_credentials_t c, const unsigned char *data,
     size_t dlen, gnutls_openpgp_crt_fmt_t format) _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_certificate_set_openpgp_keyring_file
    (gnutls_certificate_credentials_t c, const char *file,
     gnutls_openpgp_crt_fmt_t format) _GNUTLS_GCC_ATTR_DEPRECATED;

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_OPENPGP_H */
