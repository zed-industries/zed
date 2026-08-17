/*
 * Copyright (C) 2008-2012 Free Software Foundation, Inc.
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

/* Typedefs for more compatibility with older GnuTLS. */

#ifndef GNUTLS_COMPAT_H
#define GNUTLS_COMPAT_H

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

#ifdef __GNUC__

#define _GNUTLS_GCC_VERSION (__GNUC__ * 10000 + __GNUC_MINOR__ * 100 + __GNUC_PATCHLEVEL__)

#if !defined GNUTLS_INTERNAL_BUILD
#if _GNUTLS_GCC_VERSION >= 30100
#define _GNUTLS_GCC_ATTR_DEPRECATED __attribute__ ((__deprecated__))
#endif
#endif

#endif				/* __GNUC__ */

#ifndef _GNUTLS_GCC_ATTR_DEPRECATED
#define _GNUTLS_GCC_ATTR_DEPRECATED
#endif

/* gnutls_connection_end_t was made redundant in 2.99.0 */
typedef unsigned int gnutls_connection_end_t _GNUTLS_GCC_ATTR_DEPRECATED;

/* Stuff deprecated in 2.x */
typedef gnutls_cipher_algorithm_t gnutls_cipher_algorithm
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_kx_algorithm_t gnutls_kx_algorithm
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_mac_algorithm_t gnutls_mac_algorithm
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_digest_algorithm_t gnutls_digest_algorithm
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_compression_method_t gnutls_compression_method
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_connection_end_t gnutls_connection_end
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_x509_crt_fmt_t gnutls_x509_crt_fmt
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_pk_algorithm_t gnutls_pk_algorithm
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_sign_algorithm_t gnutls_sign_algorithm
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_close_request_t gnutls_close_request
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_certificate_request_t gnutls_certificate_request
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_certificate_status_t gnutls_certificate_status
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_session_t gnutls_session _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_alert_level_t gnutls_alert_level
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_alert_description_t gnutls_alert_description
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_x509_subject_alt_name_t gnutls_x509_subject_alt_name
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_openpgp_privkey_t gnutls_openpgp_privkey
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_openpgp_keyring_t gnutls_openpgp_keyring
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_x509_crt_t gnutls_x509_crt _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_x509_privkey_t gnutls_x509_privkey
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_x509_crl_t gnutls_x509_crl _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_x509_crq_t gnutls_x509_crq _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_certificate_credentials_t
    gnutls_certificate_credentials _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_anon_server_credentials_t
    gnutls_anon_server_credentials _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_anon_client_credentials_t
    gnutls_anon_client_credentials _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_srp_client_credentials_t
    gnutls_srp_client_credentials _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_srp_server_credentials_t
    gnutls_srp_server_credentials _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_dh_params_t gnutls_dh_params _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_rsa_params_t gnutls_rsa_params _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_params_type_t gnutls_params_type
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_credentials_type_t gnutls_credentials_type
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_certificate_type_t gnutls_certificate_type
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_datum_t gnutls_datum _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_transport_ptr_t gnutls_transport_ptr
    _GNUTLS_GCC_ATTR_DEPRECATED;

/* Old verification flags */
#define GNUTLS_VERIFY_ALLOW_X509_V1_CA_CRT  (0)

/* Old SRP alerts removed in 2.1.x because the TLS-SRP RFC was
   modified to use the PSK alert. */
#define GNUTLS_A_MISSING_SRP_USERNAME GNUTLS_A_UNKNOWN_PSK_IDENTITY
#define GNUTLS_A_UNKNOWN_SRP_USERNAME GNUTLS_A_UNKNOWN_PSK_IDENTITY

/* OpenPGP stuff renamed in 2.1.x. */
#define GNUTLS_OPENPGP_KEY GNUTLS_OPENPGP_CERT
#define GNUTLS_OPENPGP_KEY_FINGERPRINT GNUTLS_OPENPGP_CERT_FINGERPRINT
#define gnutls_openpgp_send_key gnutls_openpgp_send_cert
typedef gnutls_openpgp_crt_status_t gnutls_openpgp_key_status_t
    _GNUTLS_GCC_ATTR_DEPRECATED;
typedef gnutls_openpgp_crt_t gnutls_openpgp_key_t
    _GNUTLS_GCC_ATTR_DEPRECATED;
#define gnutls_openpgp_key_init gnutls_openpgp_crt_init
#define gnutls_openpgp_key_deinit gnutls_openpgp_crt_deinit
#define gnutls_openpgp_key_import gnutls_openpgp_crt_import
#define gnutls_openpgp_key_export gnutls_openpgp_crt_export
#define gnutls_openpgp_key_get_key_usage gnutls_openpgp_crt_get_key_usage
#define gnutls_openpgp_key_get_fingerprint gnutls_openpgp_crt_get_fingerprint
#define gnutls_openpgp_key_get_pk_algorithm gnutls_openpgp_crt_get_pk_algorithm
#define gnutls_openpgp_key_get_name gnutls_openpgp_crt_get_name
#define gnutls_openpgp_key_get_version gnutls_openpgp_crt_get_version
#define gnutls_openpgp_key_get_creation_time gnutls_openpgp_crt_get_creation_time
#define gnutls_openpgp_key_get_expiration_time gnutls_openpgp_crt_get_expiration_time
#define gnutls_openpgp_key_get_id gnutls_openpgp_crt_get_id
#define gnutls_openpgp_key_check_hostname gnutls_openpgp_crt_check_hostname

/* OpenPGP stuff renamed in 2.3.x. */
#define gnutls_openpgp_crt_get_id gnutls_openpgp_crt_get_key_id

/* New better names renamed in 2.3.x, add these for backwards
   compatibility with old poor names.*/
#define GNUTLS_X509_CRT_FULL GNUTLS_CRT_PRINT_FULL
#define GNUTLS_X509_CRT_ONELINE GNUTLS_CRT_PRINT_ONELINE
#define GNUTLS_X509_CRT_UNSIGNED_FULL GNUTLS_CRT_PRINT_UNSIGNED_FULL

/* Namespace problems. */
#define LIBGNUTLS_VERSION GNUTLS_VERSION
#define LIBGNUTLS_VERSION_MAJOR GNUTLS_VERSION_MAJOR
#define LIBGNUTLS_VERSION_MINOR GNUTLS_VERSION_MINOR
#define LIBGNUTLS_VERSION_PATCH GNUTLS_VERSION_PATCH
#define LIBGNUTLS_VERSION_NUMBER GNUTLS_VERSION_NUMBER
#define LIBGNUTLS_EXTRA_VERSION GNUTLS_VERSION

/* This is a very dangerous and error-prone function.
 * Use gnutls_privkey_sign_hash() instead.
 */
int gnutls_x509_privkey_sign_hash(gnutls_x509_privkey_t key,
				  const gnutls_datum_t * hash,
				  gnutls_datum_t * signature)
    _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_openpgp_privkey_sign_hash(gnutls_openpgp_privkey_t key,
				     const gnutls_datum_t * hash,
				     gnutls_datum_t * signature)
    _GNUTLS_GCC_ATTR_DEPRECATED;

	/* gnutls_pubkey_get_preferred_hash_algorithm() */
int gnutls_x509_crt_get_preferred_hash_algorithm(gnutls_x509_crt_t
						 crt,
						 gnutls_digest_algorithm_t
						 * hash, unsigned int
						 *mand)
    _GNUTLS_GCC_ATTR_DEPRECATED;

	/* use gnutls_privkey_sign_hash() with the GNUTLS_PRIVKEY_SIGN_FLAG_TLS1_RSA flag */

#ifdef _ISOC99_SOURCE
/* we provide older functions for compatibility as inline functions that
 * depend on gnutls_session_get_random. */

static inline const void
*gnutls_session_get_server_random(gnutls_session_t session)
    _GNUTLS_GCC_ATTR_DEPRECATED;
static inline const void
*gnutls_session_get_server_random(gnutls_session_t session)
{
	gnutls_datum_t rnd;
	gnutls_session_get_random(session, NULL, &rnd);	/*doc-skip */
	return rnd.data;
}

static inline const void
*gnutls_session_get_client_random(gnutls_session_t session)
    _GNUTLS_GCC_ATTR_DEPRECATED;
static inline const void
*gnutls_session_get_client_random(gnutls_session_t session)
{
	gnutls_datum_t rnd;
	gnutls_session_get_random(session, &rnd, NULL);	/*doc-skip */
	return rnd.data;
}
#endif

void
gnutls_global_set_mem_functions(gnutls_alloc_function alloc_func,
				gnutls_alloc_function secure_alloc_func,
				gnutls_is_secure_function is_secure_func,
				gnutls_realloc_function realloc_func,
				gnutls_free_function free_func) _GNUTLS_GCC_ATTR_DEPRECATED;

/* defined in old headers - unused nevertheless */
#define GNUTLS_SUPPLEMENTAL_USER_MAPPING_DATA 0

/* old compression related functions */
gnutls_compression_method_t
gnutls_compression_get(gnutls_session_t session) _GNUTLS_GCC_ATTR_DEPRECATED;

const char *
gnutls_compression_get_name(gnutls_compression_method_t
			    algorithm) __GNUTLS_CONST__ _GNUTLS_GCC_ATTR_DEPRECATED;

gnutls_compression_method_t
	gnutls_compression_get_id(const char *name) __GNUTLS_CONST__ _GNUTLS_GCC_ATTR_DEPRECATED;

const gnutls_compression_method_t *
	gnutls_compression_list(void) __GNUTLS_PURE__ _GNUTLS_GCC_ATTR_DEPRECATED;

int gnutls_priority_compression_list(gnutls_priority_t pcache,
				     const unsigned int **list) _GNUTLS_GCC_ATTR_DEPRECATED;

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_COMPAT_H */
