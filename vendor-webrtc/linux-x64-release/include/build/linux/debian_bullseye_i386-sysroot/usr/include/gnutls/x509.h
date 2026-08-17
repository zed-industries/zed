/*
 * Copyright (C) 2003-2016 Free Software Foundation, Inc.
 * Copyright (C) 2015-2016 Red Hat, Inc.
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

#ifndef GNUTLS_X509_H
#define GNUTLS_X509_H

#include <gnutls/gnutls.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

/* Some OIDs usually found in Distinguished names, or
 * in Subject Directory Attribute extensions.
 */
#define GNUTLS_OID_X520_COUNTRY_NAME		"2.5.4.6"
#define GNUTLS_OID_X520_ORGANIZATION_NAME	"2.5.4.10"
#define GNUTLS_OID_X520_ORGANIZATIONAL_UNIT_NAME "2.5.4.11"
#define GNUTLS_OID_X520_COMMON_NAME		"2.5.4.3"
#define GNUTLS_OID_X520_LOCALITY_NAME		"2.5.4.7"
#define GNUTLS_OID_X520_STATE_OR_PROVINCE_NAME	"2.5.4.8"

#define GNUTLS_OID_X520_INITIALS		"2.5.4.43"
#define GNUTLS_OID_X520_GENERATION_QUALIFIER	"2.5.4.44"
#define GNUTLS_OID_X520_SURNAME			"2.5.4.4"
#define GNUTLS_OID_X520_GIVEN_NAME		"2.5.4.42"
#define GNUTLS_OID_X520_TITLE			"2.5.4.12"
#define GNUTLS_OID_X520_DN_QUALIFIER		"2.5.4.46"
#define GNUTLS_OID_X520_PSEUDONYM		"2.5.4.65"
#define GNUTLS_OID_X520_POSTALCODE              "2.5.4.17"
#define GNUTLS_OID_X520_NAME                    "2.5.4.41"

#define GNUTLS_OID_LDAP_DC			"0.9.2342.19200300.100.1.25"
#define GNUTLS_OID_LDAP_UID			"0.9.2342.19200300.100.1.1"

/* The following should not be included in DN.
 */
#define GNUTLS_OID_PKCS9_EMAIL			"1.2.840.113549.1.9.1"

#define GNUTLS_OID_PKIX_DATE_OF_BIRTH		"1.3.6.1.5.5.7.9.1"
#define GNUTLS_OID_PKIX_PLACE_OF_BIRTH		"1.3.6.1.5.5.7.9.2"
#define GNUTLS_OID_PKIX_GENDER			"1.3.6.1.5.5.7.9.3"
#define GNUTLS_OID_PKIX_COUNTRY_OF_CITIZENSHIP	"1.3.6.1.5.5.7.9.4"
#define GNUTLS_OID_PKIX_COUNTRY_OF_RESIDENCE	"1.3.6.1.5.5.7.9.5"

/* Key purpose Object Identifiers.
 */
#define GNUTLS_KP_TLS_WWW_SERVER		"1.3.6.1.5.5.7.3.1"
#define GNUTLS_KP_TLS_WWW_CLIENT                "1.3.6.1.5.5.7.3.2"
#define GNUTLS_KP_CODE_SIGNING			"1.3.6.1.5.5.7.3.3"
#define GNUTLS_KP_MS_SMART_CARD_LOGON		"1.3.6.1.4.1.311.20.2.2"
#define GNUTLS_KP_EMAIL_PROTECTION		"1.3.6.1.5.5.7.3.4"
#define GNUTLS_KP_TIME_STAMPING			"1.3.6.1.5.5.7.3.8"
#define GNUTLS_KP_OCSP_SIGNING			"1.3.6.1.5.5.7.3.9"
#define GNUTLS_KP_IPSEC_IKE			"1.3.6.1.5.5.7.3.17"
#define GNUTLS_KP_ANY				"2.5.29.37.0"

#define GNUTLS_KP_FLAG_DISALLOW_ANY		1

#define GNUTLS_OID_AIA				"1.3.6.1.5.5.7.1.1"
#define GNUTLS_OID_AD_OCSP			"1.3.6.1.5.5.7.48.1"
#define GNUTLS_OID_AD_CAISSUERS			"1.3.6.1.5.5.7.48.2"

#define GNUTLS_FSAN_SET 0
#define GNUTLS_FSAN_APPEND 1
#define GNUTLS_FSAN_ENCODE_OCTET_STRING (1<<1)
#define GNUTLS_FSAN_ENCODE_UTF8_STRING (1<<2)

#define GNUTLS_X509EXT_OID_SUBJECT_KEY_ID "2.5.29.14"
#define GNUTLS_X509EXT_OID_KEY_USAGE "2.5.29.15"
#define GNUTLS_X509EXT_OID_PRIVATE_KEY_USAGE_PERIOD "2.5.29.16"
#define GNUTLS_X509EXT_OID_SAN "2.5.29.17"
#define GNUTLS_X509EXT_OID_IAN "2.5.29.18"
#define GNUTLS_X509EXT_OID_BASIC_CONSTRAINTS "2.5.29.19"
#define GNUTLS_X509EXT_OID_NAME_CONSTRAINTS "2.5.29.30"
#define GNUTLS_X509EXT_OID_CRL_DIST_POINTS "2.5.29.31"
#define GNUTLS_X509EXT_OID_CRT_POLICY "2.5.29.32"
#define GNUTLS_X509EXT_OID_AUTHORITY_KEY_ID "2.5.29.35"
#define GNUTLS_X509EXT_OID_EXTENDED_KEY_USAGE "2.5.29.37"
#define GNUTLS_X509EXT_OID_INHIBIT_ANYPOLICY "2.5.29.52"
#define GNUTLS_X509EXT_OID_AUTHORITY_INFO_ACCESS "1.3.6.1.5.5.7.1.1"
#define GNUTLS_X509EXT_OID_PROXY_CRT_INFO "1.3.6.1.5.5.7.1.14"
#define GNUTLS_X509EXT_OID_TLSFEATURES "1.3.6.1.5.5.7.1.24"

#define GNUTLS_X509_OID_POLICY_ANY "2.5.29.54"

/* Certificate handling functions.
 */

/**
 * gnutls_certificate_import_flags:
 * @GNUTLS_X509_CRT_LIST_IMPORT_FAIL_IF_EXCEED: Fail if the
 *   certificates in the buffer are more than the space allocated for
 *   certificates. The error code will be %GNUTLS_E_SHORT_MEMORY_BUFFER.
 * @GNUTLS_X509_CRT_LIST_FAIL_IF_UNSORTED: Fail if the certificates
 *   in the buffer are not ordered starting from subject to issuer.
 *   The error code will be %GNUTLS_E_CERTIFICATE_LIST_UNSORTED.
 * @GNUTLS_X509_CRT_LIST_SORT: Sort the certificate chain if unsorted.
 *
 * Enumeration of different certificate import flags.
 */
typedef enum gnutls_certificate_import_flags {
	GNUTLS_X509_CRT_LIST_IMPORT_FAIL_IF_EXCEED = 1,
	GNUTLS_X509_CRT_LIST_FAIL_IF_UNSORTED = 1<<1,
	GNUTLS_X509_CRT_LIST_SORT = 1<<2
} gnutls_certificate_import_flags;

int gnutls_x509_crt_init(gnutls_x509_crt_t * cert);
void gnutls_x509_crt_deinit(gnutls_x509_crt_t cert);

/**
 * gnutls_certificate_import_flags:
 * @GNUTLS_X509_CRT_FLAG_IGNORE_SANITY: Ignore any sanity checks at the
 *   import of the certificate; i.e., ignore checks such as version/field
 *   matching and strict time field checks. Intended to be used for debugging.
 *
 * Enumeration of different certificate flags.
 */
typedef enum gnutls_x509_crt_flags {
	GNUTLS_X509_CRT_FLAG_IGNORE_SANITY = 1
} gnutls_x509_crt_flags;
void gnutls_x509_crt_set_flags(gnutls_x509_crt_t cert, unsigned flags);

unsigned gnutls_x509_crt_equals(gnutls_x509_crt_t cert1, gnutls_x509_crt_t cert2);
unsigned gnutls_x509_crt_equals2(gnutls_x509_crt_t cert1, const gnutls_datum_t * der);

int gnutls_x509_crt_import(gnutls_x509_crt_t cert,
			   const gnutls_datum_t * data,
			   gnutls_x509_crt_fmt_t format);
int gnutls_x509_crt_list_import2(gnutls_x509_crt_t ** certs,
				 unsigned int *size,
				 const gnutls_datum_t * data,
				 gnutls_x509_crt_fmt_t format,
				 unsigned int flags);
int gnutls_x509_crt_list_import(gnutls_x509_crt_t * certs,
				unsigned int *cert_max,
				const gnutls_datum_t * data,
				gnutls_x509_crt_fmt_t format,
				unsigned int flags);

int gnutls_x509_crt_import_url(gnutls_x509_crt_t crt,
				      const char *url, unsigned int flags
				      /* GNUTLS_PKCS11_OBJ_FLAG_* */
    );

int
gnutls_x509_crt_list_import_url(gnutls_x509_crt_t **certs,
				unsigned int *size,
				const char *url,
				gnutls_pin_callback_t pin_fn,
				void *pin_fn_userdata,
				unsigned int flags);

int gnutls_x509_crt_export(gnutls_x509_crt_t cert,
			   gnutls_x509_crt_fmt_t format,
			   void *output_data, size_t * output_data_size);
int gnutls_x509_crt_export2(gnutls_x509_crt_t cert,
			    gnutls_x509_crt_fmt_t format,
			    gnutls_datum_t * out);
int gnutls_x509_crt_get_private_key_usage_period(gnutls_x509_crt_t
						 cert,
						 time_t *
						 activation,
						 time_t *
						 expiration, unsigned int
						 *critical);

int gnutls_x509_crt_get_issuer_dn(gnutls_x509_crt_t cert,
				  char *buf, size_t * buf_size);
int gnutls_x509_crt_get_issuer_dn2(gnutls_x509_crt_t cert,
				   gnutls_datum_t * dn);
int gnutls_x509_crt_get_issuer_dn3(gnutls_x509_crt_t cert,
				   gnutls_datum_t * dn, unsigned flags);
int gnutls_x509_crt_get_issuer_dn_oid(gnutls_x509_crt_t cert,
				      unsigned indx, void *oid,
				      size_t * oid_size);
int gnutls_x509_crt_get_issuer_dn_by_oid(gnutls_x509_crt_t cert,
					 const char *oid, unsigned indx,
					 unsigned int raw_flag,
					 void *buf, size_t * buf_size);

int gnutls_x509_crt_get_dn(gnutls_x509_crt_t cert, char *buf,
			   size_t * buf_size);
int gnutls_x509_crt_get_dn2(gnutls_x509_crt_t cert, gnutls_datum_t * dn);
int gnutls_x509_crt_get_dn3(gnutls_x509_crt_t cert, gnutls_datum_t * dn, unsigned flags);

int gnutls_x509_crt_get_dn_oid(gnutls_x509_crt_t cert, unsigned indx,
			       void *oid, size_t * oid_size);
int gnutls_x509_crt_get_dn_by_oid(gnutls_x509_crt_t cert,
				  const char *oid, unsigned indx,
				  unsigned int raw_flag, void *buf,
				  size_t * buf_size);
unsigned gnutls_x509_crt_check_hostname(gnutls_x509_crt_t cert,
				   const char *hostname);
unsigned gnutls_x509_crt_check_hostname2(gnutls_x509_crt_t cert,
					 const char *hostname, unsigned int flags);
unsigned
gnutls_x509_crt_check_email(gnutls_x509_crt_t cert,
			    const char *email, unsigned int flags);

unsigned
gnutls_x509_crt_check_ip(gnutls_x509_crt_t cert,
			 const unsigned char *ip, unsigned int ip_size,
			 unsigned int flags);

int gnutls_x509_crt_get_signature_algorithm(gnutls_x509_crt_t cert);
int gnutls_x509_crt_get_signature(gnutls_x509_crt_t cert,
				  char *sig, size_t * sizeof_sig);
int gnutls_x509_crt_get_version(gnutls_x509_crt_t cert);

int gnutls_x509_crt_get_pk_oid(gnutls_x509_crt_t cert, char *oid, size_t *oid_size);
int gnutls_x509_crt_get_signature_oid(gnutls_x509_crt_t cert, char *oid, size_t *oid_size);

/**
 * gnutls_keyid_flags_t:
 * @GNUTLS_KEYID_USE_SHA1: Use SHA1 as the key ID algorithm (default).
 * @GNUTLS_KEYID_USE_SHA256: Use SHA256 as the key ID algorithm.
 * @GNUTLS_KEYID_USE_SHA512: Use SHA512 as the key ID algorithm.
 * @GNUTLS_KEYID_USE_BEST_KNOWN: Use the best known algorithm to calculate key ID. Using that option will make your program behavior depend on the version of gnutls linked with. That option has a cap of 64-bytes key IDs.
 *
 * Enumeration of different flags for the key ID functions.
 
 */
typedef enum {
	GNUTLS_KEYID_USE_SHA1 = 0,
	GNUTLS_KEYID_USE_SHA256 = (1<<0),
	GNUTLS_KEYID_USE_SHA512 = (1<<1),
	GNUTLS_KEYID_USE_BEST_KNOWN = (1<<30)
} gnutls_keyid_flags_t;
int gnutls_x509_crt_get_key_id(gnutls_x509_crt_t crt,
			       unsigned int flags,
			       unsigned char *output_data,
			       size_t * output_data_size);

int gnutls_x509_crt_set_private_key_usage_period(gnutls_x509_crt_t
						 crt,
						 time_t activation,
						 time_t expiration);
int gnutls_x509_crt_set_authority_key_id(gnutls_x509_crt_t cert,
					 const void *id, size_t id_size);
int gnutls_x509_crt_get_authority_key_id(gnutls_x509_crt_t cert,
					 void *id,
					 size_t * id_size,
					 unsigned int *critical);
int gnutls_x509_crt_get_authority_key_gn_serial(gnutls_x509_crt_t
						cert,
						unsigned int seq,
						void *alt,
						size_t * alt_size,
						unsigned int
						*alt_type,
						void *serial,
						size_t *
						serial_size, unsigned int
						*critical);

int gnutls_x509_crt_get_subject_key_id(gnutls_x509_crt_t cert,
				       void *ret,
				       size_t * ret_size,
				       unsigned int *critical);

int gnutls_x509_crt_get_subject_unique_id(gnutls_x509_crt_t crt,
					  char *buf, size_t * buf_size);

int gnutls_x509_crt_get_issuer_unique_id(gnutls_x509_crt_t crt,
					 char *buf, size_t * buf_size);

void gnutls_x509_crt_set_pin_function(gnutls_x509_crt_t crt,
				      gnutls_pin_callback_t fn,
				      void *userdata);

  /**
   * gnutls_info_access_what_t:
   * @GNUTLS_IA_ACCESSMETHOD_OID: Get accessMethod OID.
   * @GNUTLS_IA_ACCESSLOCATION_GENERALNAME_TYPE: Get accessLocation name type.
   * @GNUTLS_IA_URI: Get accessLocation URI value.
   * @GNUTLS_IA_OCSP_URI: get accessLocation URI value for OCSP.
   * @GNUTLS_IA_CAISSUERS_URI: get accessLocation URI value for caIssuers.
   *
   * Enumeration of types for the @what parameter of
   * gnutls_x509_crt_get_authority_info_access().
   */
typedef enum gnutls_info_access_what_t {
	GNUTLS_IA_ACCESSMETHOD_OID = 1,
	GNUTLS_IA_ACCESSLOCATION_GENERALNAME_TYPE = 2,
	/* use 100-108 for the generalName types, populate as needed */
	GNUTLS_IA_URI = 106,
	/* quick-access variants that match both OID and name type. */
	GNUTLS_IA_UNKNOWN = 10000,
	GNUTLS_IA_OCSP_URI = 10006,
	GNUTLS_IA_CAISSUERS_URI = 10106
} gnutls_info_access_what_t;

int gnutls_x509_crt_get_authority_info_access(gnutls_x509_crt_t
					      crt,
					      unsigned int seq,
					      int what,
					      gnutls_datum_t *
					      data, unsigned int
					      *critical);

typedef struct gnutls_name_constraints_st *gnutls_x509_name_constraints_t;

unsigned gnutls_x509_name_constraints_check(gnutls_x509_name_constraints_t nc,
				       gnutls_x509_subject_alt_name_t type,
				       const gnutls_datum_t * name);
unsigned gnutls_x509_name_constraints_check_crt(gnutls_x509_name_constraints_t nc,
				       gnutls_x509_subject_alt_name_t type,
				       gnutls_x509_crt_t crt);

int gnutls_x509_name_constraints_init(gnutls_x509_name_constraints_t *nc);
void gnutls_x509_name_constraints_deinit(gnutls_x509_name_constraints_t nc);

#define GNUTLS_EXT_FLAG_APPEND 1

#define GNUTLS_NAME_CONSTRAINTS_FLAG_APPEND GNUTLS_EXT_FLAG_APPEND
int gnutls_x509_crt_get_name_constraints(gnutls_x509_crt_t crt,
					 gnutls_x509_name_constraints_t nc,
					 unsigned int flags,
					 unsigned int *critical);
int gnutls_x509_name_constraints_add_permitted(gnutls_x509_name_constraints_t nc,
					       gnutls_x509_subject_alt_name_t type,
					       const gnutls_datum_t * name);
int gnutls_x509_name_constraints_add_excluded(gnutls_x509_name_constraints_t nc,
					      gnutls_x509_subject_alt_name_t type,
					      const gnutls_datum_t * name);
int gnutls_x509_crt_set_name_constraints(gnutls_x509_crt_t crt, 
					 gnutls_x509_name_constraints_t nc,
					 unsigned int critical);
int gnutls_x509_name_constraints_get_permitted(gnutls_x509_name_constraints_t nc,
				     unsigned idx,
				     unsigned *type, gnutls_datum_t * name);
int gnutls_x509_name_constraints_get_excluded(gnutls_x509_name_constraints_t nc,
				     unsigned idx,
				     unsigned *type, gnutls_datum_t * name);
int gnutls_x509_cidr_to_rfc5280(const char *cidr, gnutls_datum_t *cidr_rfc5280);


#define GNUTLS_CRL_REASON_SUPERSEEDED GNUTLS_CRL_REASON_SUPERSEDED,
  /**
   * gnutls_x509_crl_reason_flags_t:
   * @GNUTLS_CRL_REASON_PRIVILEGE_WITHDRAWN: The privileges were withdrawn from the owner.
   * @GNUTLS_CRL_REASON_CERTIFICATE_HOLD: The certificate is on hold.
   * @GNUTLS_CRL_REASON_CESSATION_OF_OPERATION: The end-entity is no longer operating.
   * @GNUTLS_CRL_REASON_SUPERSEDED: There is a newer certificate of the owner.
   * @GNUTLS_CRL_REASON_AFFILIATION_CHANGED: The end-entity affiliation has changed.
   * @GNUTLS_CRL_REASON_CA_COMPROMISE: The CA was compromised.
   * @GNUTLS_CRL_REASON_KEY_COMPROMISE: The certificate's key was compromised.
   * @GNUTLS_CRL_REASON_UNUSED: The key was never used.
   * @GNUTLS_CRL_REASON_AA_COMPROMISE: AA compromised.
   *
   * Enumeration of types for the CRL revocation reasons. 
   */
typedef enum gnutls_x509_crl_reason_flags_t {
	GNUTLS_CRL_REASON_UNSPECIFIED = 0,
	GNUTLS_CRL_REASON_PRIVILEGE_WITHDRAWN = 1,
	GNUTLS_CRL_REASON_CERTIFICATE_HOLD = 2,
	GNUTLS_CRL_REASON_CESSATION_OF_OPERATION = 4,
	GNUTLS_CRL_REASON_SUPERSEDED = 8,
	GNUTLS_CRL_REASON_AFFILIATION_CHANGED = 16,
	GNUTLS_CRL_REASON_CA_COMPROMISE = 32,
	GNUTLS_CRL_REASON_KEY_COMPROMISE = 64,
	GNUTLS_CRL_REASON_UNUSED = 128,
	GNUTLS_CRL_REASON_AA_COMPROMISE = 32768
} gnutls_x509_crl_reason_flags_t;

int gnutls_x509_crt_get_crl_dist_points(gnutls_x509_crt_t cert,
					unsigned int seq,
					void *ret,
					size_t * ret_size,
					unsigned int *reason_flags,
					unsigned int *critical);
int gnutls_x509_crt_set_crl_dist_points2(gnutls_x509_crt_t crt,
					 gnutls_x509_subject_alt_name_t
					 type, const void *data,
					 unsigned int data_size,
					 unsigned int reason_flags);
int gnutls_x509_crt_set_crl_dist_points(gnutls_x509_crt_t crt,
					gnutls_x509_subject_alt_name_t
					type,
					const void *data_string,
					unsigned int reason_flags);
int gnutls_x509_crt_cpy_crl_dist_points(gnutls_x509_crt_t dst,
					gnutls_x509_crt_t src);

int gnutls_x509_crl_sign(gnutls_x509_crl_t crl,
			 gnutls_x509_crt_t issuer,
			 gnutls_x509_privkey_t issuer_key);

int gnutls_x509_crl_sign2(gnutls_x509_crl_t crl,
			  gnutls_x509_crt_t issuer,
			  gnutls_x509_privkey_t issuer_key,
			  gnutls_digest_algorithm_t dig,
			  unsigned int flags);

time_t gnutls_x509_crt_get_activation_time(gnutls_x509_crt_t cert);

/* This macro is deprecated and defunc; do not use */
#define GNUTLS_X509_NO_WELL_DEFINED_EXPIRATION ((time_t)4294197631)

time_t gnutls_x509_crt_get_expiration_time(gnutls_x509_crt_t cert);
int gnutls_x509_crt_get_serial(gnutls_x509_crt_t cert,
			       void *result, size_t * result_size);

typedef struct gnutls_x509_spki_st *gnutls_x509_spki_t;

int gnutls_x509_spki_init(gnutls_x509_spki_t *spki);
void gnutls_x509_spki_deinit(gnutls_x509_spki_t spki);

int gnutls_x509_spki_get_rsa_pss_params(gnutls_x509_spki_t spki,
			gnutls_digest_algorithm_t *dig, unsigned int *salt_size);

void gnutls_x509_spki_set_rsa_pss_params(gnutls_x509_spki_t spki,
			gnutls_digest_algorithm_t dig, unsigned int salt_size);

int gnutls_x509_crt_get_pk_algorithm(gnutls_x509_crt_t cert,
				     unsigned int *bits);
int gnutls_x509_crt_set_spki(gnutls_x509_crt_t crt, const gnutls_x509_spki_t spki,
			     unsigned int flags);
int gnutls_x509_crt_get_spki(gnutls_x509_crt_t cert, gnutls_x509_spki_t spki,
			     unsigned int flags);

int gnutls_x509_crt_get_pk_rsa_raw(gnutls_x509_crt_t crt,
				   gnutls_datum_t * m, gnutls_datum_t * e);
int gnutls_x509_crt_get_pk_dsa_raw(gnutls_x509_crt_t crt,
				   gnutls_datum_t * p,
				   gnutls_datum_t * q,
				   gnutls_datum_t * g, gnutls_datum_t * y);
int gnutls_x509_crt_get_pk_ecc_raw(gnutls_x509_crt_t crt,
				   gnutls_ecc_curve_t * curve,
				   gnutls_datum_t * x,
				   gnutls_datum_t * y);
int gnutls_x509_crt_get_pk_gost_raw(gnutls_x509_crt_t crt,
				    gnutls_ecc_curve_t * curve,
				    gnutls_digest_algorithm_t * digest,
				    gnutls_gost_paramset_t *paramset,
				    gnutls_datum_t * x, gnutls_datum_t * y);

int gnutls_x509_crt_get_subject_alt_name(gnutls_x509_crt_t cert,
					 unsigned int seq,
					 void *san,
					 size_t * san_size,
					 unsigned int *critical);
int gnutls_x509_crt_get_subject_alt_name2(gnutls_x509_crt_t cert,
					  unsigned int seq,
					  void *san,
					  size_t * san_size,
					  unsigned int *san_type,
					  unsigned int *critical);

int gnutls_x509_crt_get_subject_alt_othername_oid(gnutls_x509_crt_t
						  cert,
						  unsigned int seq,
						  void *oid,
						  size_t * oid_size);

int gnutls_x509_crt_get_issuer_alt_name(gnutls_x509_crt_t cert,
					unsigned int seq,
					void *ian,
					size_t * ian_size,
					unsigned int *critical);
int gnutls_x509_crt_get_issuer_alt_name2(gnutls_x509_crt_t cert,
					 unsigned int seq,
					 void *ian,
					 size_t * ian_size,
					 unsigned int *ian_type,
					 unsigned int *critical);

int gnutls_x509_crt_get_issuer_alt_othername_oid(gnutls_x509_crt_t
						 cert,
						 unsigned int seq,
						 void *ret,
						 size_t * ret_size);

int gnutls_x509_crt_get_ca_status(gnutls_x509_crt_t cert,
				  unsigned int *critical);
int gnutls_x509_crt_get_basic_constraints(gnutls_x509_crt_t cert,
					  unsigned int *critical,
					  unsigned int *ca, int *pathlen);

/* The key_usage flags are defined in gnutls.h. They are the
 * GNUTLS_KEY_* definitions.
 */
int gnutls_x509_crt_get_key_usage(gnutls_x509_crt_t cert,
				  unsigned int *key_usage,
				  unsigned int *critical);
int gnutls_x509_crt_set_key_usage(gnutls_x509_crt_t crt,
				  unsigned int usage);
int gnutls_x509_crt_set_authority_info_access(gnutls_x509_crt_t
					      crt, int what,
					      gnutls_datum_t * data);

int gnutls_x509_crt_get_inhibit_anypolicy(gnutls_x509_crt_t cert,
				  unsigned int *skipcerts,
				  unsigned int *critical);
int
gnutls_x509_crt_set_inhibit_anypolicy(gnutls_x509_crt_t crt, unsigned int skipcerts);

int gnutls_x509_crt_get_proxy(gnutls_x509_crt_t cert,
			      unsigned int *critical,
			      int *pathlen,
			      char **policyLanguage,
			      char **policy, size_t * sizeof_policy);


typedef struct gnutls_x509_tlsfeatures_st *gnutls_x509_tlsfeatures_t;

int gnutls_x509_tlsfeatures_init(gnutls_x509_tlsfeatures_t *features);
void gnutls_x509_tlsfeatures_deinit(gnutls_x509_tlsfeatures_t);
int gnutls_x509_tlsfeatures_get(gnutls_x509_tlsfeatures_t f, unsigned idx, unsigned int *feature);

int gnutls_x509_crt_set_tlsfeatures(gnutls_x509_crt_t crt,
				    gnutls_x509_tlsfeatures_t features);

int gnutls_x509_crt_get_tlsfeatures(gnutls_x509_crt_t cert,
				    gnutls_x509_tlsfeatures_t features,
				    unsigned int flags,
				    unsigned int *critical);

unsigned gnutls_x509_tlsfeatures_check_crt(gnutls_x509_tlsfeatures_t feat,
				           gnutls_x509_crt_t crt);


#define GNUTLS_MAX_QUALIFIERS 8

  /**
   * gnutls_x509_qualifier_t:
   * @GNUTLS_X509_QUALIFIER_UNKNOWN: Unknown qualifier.
   * @GNUTLS_X509_QUALIFIER_URI: A URL
   * @GNUTLS_X509_QUALIFIER_NOICE: A text notice.
   *
   * Enumeration of types for the X.509 qualifiers, of the certificate policy extension. 
   */
typedef enum gnutls_x509_qualifier_t {
	GNUTLS_X509_QUALIFIER_UNKNOWN = 0, GNUTLS_X509_QUALIFIER_URI,
	GNUTLS_X509_QUALIFIER_NOTICE
} gnutls_x509_qualifier_t;

typedef struct gnutls_x509_policy_st {
	char *oid;
	unsigned int qualifiers;
	struct {
		gnutls_x509_qualifier_t type;
		char *data;
		unsigned int size;
	} qualifier[GNUTLS_MAX_QUALIFIERS];
} gnutls_x509_policy_st;

void gnutls_x509_policy_release(struct gnutls_x509_policy_st
				*policy);
int gnutls_x509_crt_get_policy(gnutls_x509_crt_t crt, unsigned indx, struct gnutls_x509_policy_st
			       *policy, unsigned int *critical);
int gnutls_x509_crt_set_policy(gnutls_x509_crt_t crt, const struct gnutls_x509_policy_st
			       *policy, unsigned int critical);

int gnutls_x509_dn_oid_known(const char *oid);

#define GNUTLS_X509_DN_OID_RETURN_OID 1
const char *gnutls_x509_dn_oid_name(const char *oid, unsigned int flags);

	/* Read extensions by OID. */
int gnutls_x509_crt_get_extension_oid(gnutls_x509_crt_t cert,
				      unsigned indx, void *oid,
				      size_t * oid_size);
int gnutls_x509_crt_get_extension_by_oid(gnutls_x509_crt_t cert,
					 const char *oid, unsigned indx,
					 void *buf,
					 size_t * buf_size,
					 unsigned int *critical);

int gnutls_x509_crq_get_signature_algorithm(gnutls_x509_crq_t crq);
int
gnutls_x509_crq_get_extension_by_oid2(gnutls_x509_crq_t crq,
				     const char *oid, unsigned indx,
				     gnutls_datum_t *output,
				     unsigned int *critical);

	/* Read extensions by sequence number. */
int gnutls_x509_crt_get_extension_info(gnutls_x509_crt_t cert,
				       unsigned indx, void *oid,
				       size_t * oid_size,
				       unsigned int *critical);
int gnutls_x509_crt_get_extension_data(gnutls_x509_crt_t cert,
				       unsigned indx, void *data,
				       size_t * sizeof_data);
int
gnutls_x509_crt_get_extension_data2(gnutls_x509_crt_t cert,
			       unsigned indx, gnutls_datum_t * data);


int gnutls_x509_crt_set_extension_by_oid(gnutls_x509_crt_t crt,
					 const char *oid,
					 const void *buf,
					 size_t sizeof_buf,
					 unsigned int critical);

/* X.509 Certificate writing.
 */
int gnutls_x509_crt_set_dn(gnutls_x509_crt_t crt, const char *dn,
			   const char **err);

int gnutls_x509_crt_set_dn_by_oid(gnutls_x509_crt_t crt,
				  const char *oid,
				  unsigned int raw_flag,
				  const void *name,
				  unsigned int sizeof_name);
int gnutls_x509_crt_set_issuer_dn_by_oid(gnutls_x509_crt_t crt,
					 const char *oid,
					 unsigned int raw_flag,
					 const void *name,
					 unsigned int sizeof_name);
int gnutls_x509_crt_set_issuer_dn(gnutls_x509_crt_t crt,
				  const char *dn, const char **err);

int gnutls_x509_crt_set_version(gnutls_x509_crt_t crt,
				unsigned int version);
int gnutls_x509_crt_set_key(gnutls_x509_crt_t crt,
			    gnutls_x509_privkey_t key);
int gnutls_x509_crt_set_ca_status(gnutls_x509_crt_t crt, unsigned int ca);
int gnutls_x509_crt_set_basic_constraints(gnutls_x509_crt_t crt,
					  unsigned int ca,
					  int pathLenConstraint);

int
gnutls_x509_crt_set_subject_unique_id(gnutls_x509_crt_t cert, const void *id,
			   size_t id_size);
int
gnutls_x509_crt_set_issuer_unique_id(gnutls_x509_crt_t cert, const void *id,
			   size_t id_size);

int gnutls_x509_crt_set_subject_alternative_name(gnutls_x509_crt_t
						 crt,
						 gnutls_x509_subject_alt_name_t
						 type, const char
						 *data_string);
int gnutls_x509_crt_set_subject_alt_name(gnutls_x509_crt_t crt,
					 gnutls_x509_subject_alt_name_t
					 type, const void *data,
					 unsigned int data_size,
					 unsigned int flags);

int
gnutls_x509_crt_set_subject_alt_othername(gnutls_x509_crt_t crt,
				     const char *oid,
				     const void *data,
				     unsigned int data_size,
				     unsigned int flags);

int gnutls_x509_crt_set_issuer_alt_name(gnutls_x509_crt_t crt,
					 gnutls_x509_subject_alt_name_t
					 type, const void *data,
					 unsigned int data_size,
					 unsigned int flags);

int
gnutls_x509_crt_set_issuer_alt_othername(gnutls_x509_crt_t crt,
				     const char *oid,
				     const void *data,
				     unsigned int data_size,
				     unsigned int flags);

int gnutls_x509_crt_sign(gnutls_x509_crt_t crt,
			 gnutls_x509_crt_t issuer,
			 gnutls_x509_privkey_t issuer_key);
int gnutls_x509_crt_sign2(gnutls_x509_crt_t crt,
			  gnutls_x509_crt_t issuer,
			  gnutls_x509_privkey_t issuer_key,
			  gnutls_digest_algorithm_t dig,
			  unsigned int flags);
int gnutls_x509_crt_set_activation_time(gnutls_x509_crt_t cert,
					time_t act_time);
int gnutls_x509_crt_set_expiration_time(gnutls_x509_crt_t cert,
					time_t exp_time);
int gnutls_x509_crt_set_serial(gnutls_x509_crt_t cert,
			       const void *serial, size_t serial_size);

int gnutls_x509_crt_set_subject_key_id(gnutls_x509_crt_t cert,
				       const void *id, size_t id_size);

int gnutls_x509_crt_set_proxy_dn(gnutls_x509_crt_t crt,
				 gnutls_x509_crt_t eecrt,
				 unsigned int raw_flag,
				 const void *name,
				 unsigned int sizeof_name);
int gnutls_x509_crt_set_proxy(gnutls_x509_crt_t crt,
			      int pathLenConstraint,
			      const char *policyLanguage,
			      const char *policy, size_t sizeof_policy);

int gnutls_x509_crt_print(gnutls_x509_crt_t cert,
			  gnutls_certificate_print_formats_t
			  format, gnutls_datum_t * out);
int gnutls_x509_crl_print(gnutls_x509_crl_t crl,
			  gnutls_certificate_print_formats_t
			  format, gnutls_datum_t * out);

	/* Access to internal Certificate fields.
	 */
int gnutls_x509_crt_get_raw_issuer_dn(gnutls_x509_crt_t cert,
				      gnutls_datum_t * start);
int gnutls_x509_crt_get_raw_dn(gnutls_x509_crt_t cert,
			       gnutls_datum_t * start);

/* RDN handling.
 */
int gnutls_x509_rdn_get(const gnutls_datum_t * idn,
			char *buf, size_t * sizeof_buf);
int
gnutls_x509_rdn_get2(const gnutls_datum_t * idn,
                     gnutls_datum_t *str, unsigned flags);

int gnutls_x509_rdn_get_oid(const gnutls_datum_t * idn,
			    unsigned indx, void *buf, size_t * sizeof_buf);

int gnutls_x509_rdn_get_by_oid(const gnutls_datum_t * idn,
			       const char *oid, unsigned indx,
			       unsigned int raw_flag, void *buf,
			       size_t * sizeof_buf);

typedef struct gnutls_x509_dn_st *gnutls_x509_dn_t;

typedef struct gnutls_x509_ava_st {
	gnutls_datum_t oid;
	gnutls_datum_t value;
	unsigned long value_tag;
} gnutls_x509_ava_st;

int gnutls_x509_crt_get_subject(gnutls_x509_crt_t cert,
				gnutls_x509_dn_t * dn);
int gnutls_x509_crt_get_issuer(gnutls_x509_crt_t cert,
			       gnutls_x509_dn_t * dn);
int gnutls_x509_dn_get_rdn_ava(gnutls_x509_dn_t dn, int irdn,
			       int iava, gnutls_x509_ava_st * ava);

int gnutls_x509_dn_get_str(gnutls_x509_dn_t dn, gnutls_datum_t *str);

#define GNUTLS_X509_DN_FLAG_COMPAT 1
int gnutls_x509_dn_get_str2(gnutls_x509_dn_t dn, gnutls_datum_t *str, unsigned flags);

int
gnutls_x509_dn_set_str(gnutls_x509_dn_t dn, const char *str, const char **err);

int gnutls_x509_dn_init(gnutls_x509_dn_t * dn);

int gnutls_x509_dn_import(gnutls_x509_dn_t dn,
			  const gnutls_datum_t * data);

int gnutls_x509_dn_export(gnutls_x509_dn_t dn,
			  gnutls_x509_crt_fmt_t format,
			  void *output_data, size_t * output_data_size);
int gnutls_x509_dn_export2(gnutls_x509_dn_t dn,
			   gnutls_x509_crt_fmt_t format,
			   gnutls_datum_t * out);

void gnutls_x509_dn_deinit(gnutls_x509_dn_t dn);


/* CRL handling functions.
 */
int gnutls_x509_crl_init(gnutls_x509_crl_t * crl);
void gnutls_x509_crl_deinit(gnutls_x509_crl_t crl);

int gnutls_x509_crl_import(gnutls_x509_crl_t crl,
			   const gnutls_datum_t * data,
			   gnutls_x509_crt_fmt_t format);
int gnutls_x509_crl_export(gnutls_x509_crl_t crl,
			   gnutls_x509_crt_fmt_t format,
			   void *output_data, size_t * output_data_size);
int gnutls_x509_crl_export2(gnutls_x509_crl_t crl,
			    gnutls_x509_crt_fmt_t format,
			    gnutls_datum_t * out);

int
gnutls_x509_crl_get_raw_issuer_dn(gnutls_x509_crl_t crl,
				  gnutls_datum_t * dn);

int gnutls_x509_crl_get_issuer_dn(gnutls_x509_crl_t crl,
				  char *buf, size_t * sizeof_buf);
int gnutls_x509_crl_get_issuer_dn2(gnutls_x509_crl_t crl,
				   gnutls_datum_t * dn);
int gnutls_x509_crl_get_issuer_dn3(gnutls_x509_crl_t crl,
				   gnutls_datum_t * dn, unsigned flags);

int gnutls_x509_crl_get_issuer_dn_by_oid(gnutls_x509_crl_t crl,
					 const char *oid, unsigned indx,
					 unsigned int raw_flag,
					 void *buf, size_t * sizeof_buf);
int gnutls_x509_crl_get_dn_oid(gnutls_x509_crl_t crl, unsigned indx,
			       void *oid, size_t * sizeof_oid);

int gnutls_x509_crl_get_signature_algorithm(gnutls_x509_crl_t crl);
int gnutls_x509_crl_get_signature(gnutls_x509_crl_t crl,
				  char *sig, size_t * sizeof_sig);
int gnutls_x509_crl_get_version(gnutls_x509_crl_t crl);

int gnutls_x509_crl_get_signature_oid(gnutls_x509_crl_t crl, char *oid, size_t *oid_size);

time_t gnutls_x509_crl_get_this_update(gnutls_x509_crl_t crl);
time_t gnutls_x509_crl_get_next_update(gnutls_x509_crl_t crl);

int gnutls_x509_crl_get_crt_count(gnutls_x509_crl_t crl);
int gnutls_x509_crl_get_crt_serial(gnutls_x509_crl_t crl, unsigned indx,
				   unsigned char *serial,
				   size_t * serial_size, time_t * t);

typedef struct gnutls_x509_crl_iter * gnutls_x509_crl_iter_t;

int gnutls_x509_crl_iter_crt_serial(gnutls_x509_crl_t crl,
				    gnutls_x509_crl_iter_t *,
				    unsigned char *serial,
				    size_t * serial_size, time_t * t);

void gnutls_x509_crl_iter_deinit(gnutls_x509_crl_iter_t);

#define gnutls_x509_crl_get_certificate_count gnutls_x509_crl_get_crt_count
#define gnutls_x509_crl_get_certificate gnutls_x509_crl_get_crt_serial

unsigned gnutls_x509_crl_check_issuer(gnutls_x509_crl_t crl,
				 gnutls_x509_crt_t issuer);

int gnutls_x509_crl_list_import2(gnutls_x509_crl_t ** crls,
				 unsigned int *size,
				 const gnutls_datum_t * data,
				 gnutls_x509_crt_fmt_t format,
				 unsigned int flags);

int gnutls_x509_crl_list_import(gnutls_x509_crl_t * crls,
				unsigned int *crl_max,
				const gnutls_datum_t * data,
				gnutls_x509_crt_fmt_t format,
				unsigned int flags);
/* CRL writing.
 */
int gnutls_x509_crl_set_version(gnutls_x509_crl_t crl,
				unsigned int version);
int gnutls_x509_crl_set_this_update(gnutls_x509_crl_t crl,
				    time_t act_time);
int gnutls_x509_crl_set_next_update(gnutls_x509_crl_t crl,
				    time_t exp_time);
int gnutls_x509_crl_set_crt_serial(gnutls_x509_crl_t crl,
				   const void *serial,
				   size_t serial_size,
				   time_t revocation_time);
int gnutls_x509_crl_set_crt(gnutls_x509_crl_t crl,
			    gnutls_x509_crt_t crt, time_t revocation_time);

int gnutls_x509_crl_get_authority_key_id(gnutls_x509_crl_t crl,
					 void *id,
					 size_t * id_size,
					 unsigned int *critical);
int gnutls_x509_crl_get_authority_key_gn_serial(gnutls_x509_crl_t
						crl,
						unsigned int seq,
						void *alt,
						size_t * alt_size,
						unsigned int
						*alt_type,
						void *serial,
						size_t *
						serial_size, unsigned int
						*critical);

int gnutls_x509_crl_get_number(gnutls_x509_crl_t crl, void *ret,
			       size_t * ret_size, unsigned int *critical);

int gnutls_x509_crl_get_extension_oid(gnutls_x509_crl_t crl,
				      unsigned indx, void *oid,
				      size_t * sizeof_oid);

int gnutls_x509_crl_get_extension_info(gnutls_x509_crl_t crl,
				       unsigned indx, void *oid,
				       size_t * sizeof_oid,
				       unsigned int *critical);

int gnutls_x509_crl_get_extension_data(gnutls_x509_crl_t crl,
				       unsigned indx, void *data,
				       size_t * sizeof_data);
int
gnutls_x509_crl_get_extension_data2(gnutls_x509_crl_t crl,
			       unsigned indx, gnutls_datum_t * data);

int gnutls_x509_crl_set_authority_key_id(gnutls_x509_crl_t crl,
					 const void *id, size_t id_size);

int gnutls_x509_crl_set_number(gnutls_x509_crl_t crl,
			       const void *nr, size_t nr_size);


/* X.509 Certificate verification functions.
 */

/**
 * gnutls_certificate_verify_flags:
 * @GNUTLS_VERIFY_DISABLE_CA_SIGN: If set a signer does not have to be
 *   a certificate authority. This flag should normally be disabled,
 *   unless you know what this means.
 * @GNUTLS_VERIFY_DISABLE_TRUSTED_TIME_CHECKS: If set a signer in the trusted
 *   list is never checked for expiration or activation.
 * @GNUTLS_VERIFY_DO_NOT_ALLOW_X509_V1_CA_CRT: Do not allow trusted CA
 *   certificates that have version 1.  This option is to be used
 *   to deprecate all certificates of version 1.
 * @GNUTLS_VERIFY_DO_NOT_ALLOW_SAME: If a certificate is not signed by
 *   anyone trusted but exists in the trusted CA list do not treat it
 *   as trusted.
 * @GNUTLS_VERIFY_ALLOW_UNSORTED_CHAIN: A certificate chain is tolerated
 *   if unsorted (the case with many TLS servers out there). This is the
 *   default since GnuTLS 3.1.4.
 * @GNUTLS_VERIFY_DO_NOT_ALLOW_UNSORTED_CHAIN: Do not tolerate an unsorted
 *   certificate chain.
 * @GNUTLS_VERIFY_ALLOW_ANY_X509_V1_CA_CRT: Allow CA certificates that
 *   have version 1 (both root and intermediate). This might be
 *   dangerous since those haven't the basicConstraints
 *   extension. 
 * @GNUTLS_VERIFY_ALLOW_SIGN_RSA_MD2: Allow certificates to be signed
 *   using the broken MD2 algorithm.
 * @GNUTLS_VERIFY_ALLOW_SIGN_RSA_MD5: Allow certificates to be signed
 *   using the broken MD5 algorithm.
 * @GNUTLS_VERIFY_ALLOW_SIGN_WITH_SHA1: Allow certificates to be signed
 *   using the broken SHA1 hash algorithm.
 * @GNUTLS_VERIFY_ALLOW_BROKEN: Allow certificates to be signed
 *   using any broken algorithm.
 * @GNUTLS_VERIFY_DISABLE_TIME_CHECKS: Disable checking of activation
 *   and expiration validity periods of certificate chains. Don't set
 *   this unless you understand the security implications.
 * @GNUTLS_VERIFY_DISABLE_CRL_CHECKS: Disable checking for validity
 *   using certificate revocation lists or the available OCSP data.
 * @GNUTLS_VERIFY_DO_NOT_ALLOW_WILDCARDS: When including a hostname
 *   check in the verification, do not consider any wildcards.
 * @GNUTLS_VERIFY_DO_NOT_ALLOW_IP_MATCHES: When verifying a hostname
 *   prevent textual IP addresses from matching IP addresses in the
 *   certificate. Treat the input only as a DNS name.
 * @GNUTLS_VERIFY_USE_TLS1_RSA: This indicates that a (raw) RSA signature is provided
 *   as in the TLS 1.0 protocol. Not all functions accept this flag.
 * @GNUTLS_VERIFY_IGNORE_UNKNOWN_CRIT_EXTENSIONS: This signals the verification
 *   process, not to fail on unknown critical extensions.
 *
 * Enumeration of different certificate verify flags. Additional
 * verification profiles can be set using GNUTLS_PROFILE_TO_VFLAGS()
 * and %gnutls_certificate_verification_profiles_t.
 */
typedef enum gnutls_certificate_verify_flags {
	GNUTLS_VERIFY_DISABLE_CA_SIGN = 1 << 0,
	GNUTLS_VERIFY_DO_NOT_ALLOW_IP_MATCHES = 1<<1,
	GNUTLS_VERIFY_DO_NOT_ALLOW_SAME = 1 << 2,
	GNUTLS_VERIFY_ALLOW_ANY_X509_V1_CA_CRT = 1 << 3,
	GNUTLS_VERIFY_ALLOW_SIGN_RSA_MD2 = 1 << 4,
	GNUTLS_VERIFY_ALLOW_SIGN_RSA_MD5 = 1 << 5,
	GNUTLS_VERIFY_DISABLE_TIME_CHECKS = 1 << 6,
	GNUTLS_VERIFY_DISABLE_TRUSTED_TIME_CHECKS = 1 << 7,
	GNUTLS_VERIFY_DO_NOT_ALLOW_X509_V1_CA_CRT = 1 << 8,
	GNUTLS_VERIFY_DISABLE_CRL_CHECKS = 1 << 9,
	GNUTLS_VERIFY_ALLOW_UNSORTED_CHAIN = 1 << 10,
	GNUTLS_VERIFY_DO_NOT_ALLOW_UNSORTED_CHAIN = 1 << 11,
	GNUTLS_VERIFY_DO_NOT_ALLOW_WILDCARDS = 1 << 12,
	GNUTLS_VERIFY_USE_TLS1_RSA = 1 << 13,
	GNUTLS_VERIFY_IGNORE_UNKNOWN_CRIT_EXTENSIONS = 1 << 14,
	GNUTLS_VERIFY_ALLOW_SIGN_WITH_SHA1 = 1 << 15
	/* cannot exceed 2^24 due to GNUTLS_PROFILE_TO_VFLAGS() */
} gnutls_certificate_verify_flags;

#define GNUTLS_VERIFY_ALLOW_BROKEN (GNUTLS_VERIFY_ALLOW_SIGN_RSA_MD2|GNUTLS_VERIFY_ALLOW_SIGN_RSA_MD5)

/**
 * gnutls_certificate_verification_profiles_t:
 * @GNUTLS_PROFILE_UNKNOWN: An invalid/unknown profile.
 * @GNUTLS_PROFILE_VERY_WEAK: A verification profile that
 *  corresponds to @GNUTLS_SEC_PARAM_VERY_WEAK (64 bits)
 * @GNUTLS_PROFILE_LOW: A verification profile that
 *  corresponds to @GNUTLS_SEC_PARAM_LOW (80 bits)
 * @GNUTLS_PROFILE_LEGACY: A verification profile that
 *  corresponds to @GNUTLS_SEC_PARAM_LEGACY (96 bits)
 * @GNUTLS_PROFILE_MEDIUM: A verification profile that
 *  corresponds to @GNUTLS_SEC_PARAM_MEDIUM (112 bits)
 * @GNUTLS_PROFILE_HIGH: A verification profile that
 *  corresponds to @GNUTLS_SEC_PARAM_HIGH (128 bits)
 * @GNUTLS_PROFILE_ULTRA: A verification profile that
 *  corresponds to @GNUTLS_SEC_PARAM_ULTRA (192 bits)
 * @GNUTLS_PROFILE_FUTURE: A verification profile that
 *  corresponds to @GNUTLS_SEC_PARAM_FUTURE (256 bits)
 * @GNUTLS_PROFILE_SUITEB128: A verification profile that
 *  applies the SUITEB128 rules
 * @GNUTLS_PROFILE_SUITEB192: A verification profile that
 *  applies the SUITEB192 rules
 *
 * Enumeration of different certificate verification profiles.
 */
typedef enum gnutls_certificate_verification_profiles_t {
	GNUTLS_PROFILE_UNKNOWN = 0,
	GNUTLS_PROFILE_VERY_WEAK = 1,
	GNUTLS_PROFILE_LOW = 2,
	GNUTLS_PROFILE_LEGACY = 4,
	GNUTLS_PROFILE_MEDIUM = 5,
	GNUTLS_PROFILE_HIGH = 6,
	GNUTLS_PROFILE_ULTRA = 7,
	GNUTLS_PROFILE_FUTURE = 8,
	
	GNUTLS_PROFILE_SUITEB128=32,
	GNUTLS_PROFILE_SUITEB192=33
	/*GNUTLS_PROFILE_MAX=255*/
} gnutls_certificate_verification_profiles_t;

#define GNUTLS_PROFILE_TO_VFLAGS(x) \
	(((unsigned)x)<<24)

#define GNUTLS_VFLAGS_PROFILE_MASK (0xff000000)

#define GNUTLS_VFLAGS_TO_PROFILE(x) \
	((((unsigned)x)>>24)&0xff)

const char *
	gnutls_certificate_verification_profile_get_name(gnutls_certificate_verification_profiles_t id) __GNUTLS_CONST__;
gnutls_certificate_verification_profiles_t gnutls_certificate_verification_profile_get_id(const char *name) __GNUTLS_CONST__;

unsigned gnutls_x509_crt_check_issuer(gnutls_x509_crt_t cert,
				 gnutls_x509_crt_t issuer);

int gnutls_x509_crt_list_verify(const gnutls_x509_crt_t *
				cert_list, unsigned cert_list_length,
				const gnutls_x509_crt_t * CA_list,
				unsigned CA_list_length,
				const gnutls_x509_crl_t * CRL_list,
				unsigned CRL_list_length,
				unsigned int flags, unsigned int *verify);

int gnutls_x509_crt_verify(gnutls_x509_crt_t cert,
			   const gnutls_x509_crt_t * CA_list,
			   unsigned CA_list_length, unsigned int flags,
			   unsigned int *verify);
int gnutls_x509_crl_verify(gnutls_x509_crl_t crl,
			   const gnutls_x509_crt_t * CA_list,
			   unsigned CA_list_length, unsigned int flags,
			   unsigned int *verify);

int
gnutls_x509_crt_verify_data2(gnutls_x509_crt_t crt,
			   gnutls_sign_algorithm_t algo,
			   unsigned int flags,
			   const gnutls_datum_t * data,
			   const gnutls_datum_t * signature);

int gnutls_x509_crt_check_revocation(gnutls_x509_crt_t cert,
				     const gnutls_x509_crl_t *
				     crl_list, unsigned crl_list_length);

int gnutls_x509_crt_get_fingerprint(gnutls_x509_crt_t cert,
				    gnutls_digest_algorithm_t algo,
				    void *buf, size_t * buf_size);

int gnutls_x509_crt_get_key_purpose_oid(gnutls_x509_crt_t cert,
					unsigned indx, void *oid,
					size_t * oid_size,
					unsigned int *critical);
int gnutls_x509_crt_set_key_purpose_oid(gnutls_x509_crt_t cert,
					const void *oid,
					unsigned int critical);

unsigned gnutls_x509_crt_check_key_purpose(gnutls_x509_crt_t cert,
		const char *purpose, unsigned flags);

/* Private key handling.
 */

/* Flags for the gnutls_x509_privkey_export_pkcs8() function.
 */

#define GNUTLS_PKCS8_PLAIN GNUTLS_PKCS_PLAIN
#define GNUTLS_PKCS8_USE_PKCS12_3DES GNUTLS_PKCS_PKCS12_3DES
#define GNUTLS_PKCS8_USE_PKCS12_ARCFOUR GNUTLS_PKCS_PKCS12_ARCFOUR
#define GNUTLS_PKCS8_USE_PKCS12_RC2_40 GNUTLS_PKCS_PKCS12_RC2_40

/**
 * gnutls_pkcs_encrypt_flags_t:
 * @GNUTLS_PKCS_PLAIN: Unencrypted private key.
 * @GNUTLS_PKCS_NULL_PASSWORD: Some schemas distinguish between an empty and a NULL password.
 * @GNUTLS_PKCS_PKCS12_3DES: PKCS-12 3DES.
 * @GNUTLS_PKCS_PKCS12_ARCFOUR: PKCS-12 ARCFOUR.
 * @GNUTLS_PKCS_PKCS12_RC2_40: PKCS-12 RC2-40.
 * @GNUTLS_PKCS_PBES2_3DES: PBES2 3DES.
 * @GNUTLS_PKCS_PBES2_AES_128: PBES2 AES-128.
 * @GNUTLS_PKCS_PBES2_AES_192: PBES2 AES-192.
 * @GNUTLS_PKCS_PBES2_AES_256: PBES2 AES-256.
 * @GNUTLS_PKCS_PBES2_DES: PBES2 single DES.
 * @GNUTLS_PKCS_PBES1_DES_MD5: PBES1 with single DES; for compatibility with openssl only.
 * @GNUTLS_PKCS_PBES2_GOST_TC26Z: PBES2 GOST 28147-89 CFB with TC26-Z S-box.
 * @GNUTLS_PKCS_PBES2_GOST_CPA: PBES2 GOST 28147-89 CFB with CryptoPro-A S-box.
 * @GNUTLS_PKCS_PBES2_GOST_CPB: PBES2 GOST 28147-89 CFB with CryptoPro-B S-box.
 * @GNUTLS_PKCS_PBES2_GOST_CPC: PBES2 GOST 28147-89 CFB with CryptoPro-C S-box.
 * @GNUTLS_PKCS_PBES2_GOST_CPD: PBES2 GOST 28147-89 CFB with CryptoPro-D S-box.
 *
 * Enumeration of different PKCS encryption flags.
 */
typedef enum gnutls_pkcs_encrypt_flags_t {
	GNUTLS_PKCS_PLAIN = 1,
	GNUTLS_PKCS_PKCS12_3DES = 1<<1,
	GNUTLS_PKCS_PKCS12_ARCFOUR = 1<<2,
	GNUTLS_PKCS_PKCS12_RC2_40 = 1<<3,
	GNUTLS_PKCS_PBES2_3DES = 1<<4,
	GNUTLS_PKCS_PBES2_AES_128 = 1<<5,
	GNUTLS_PKCS_PBES2_AES_192 = 1<<6,
	GNUTLS_PKCS_PBES2_AES_256 = 1<<7,
	GNUTLS_PKCS_NULL_PASSWORD = 1<<8,
	GNUTLS_PKCS_PBES2_DES = 1<<9,
	GNUTLS_PKCS_PBES1_DES_MD5 = 1<<10,
	GNUTLS_PKCS_PBES2_GOST_TC26Z = 1<<11,
	GNUTLS_PKCS_PBES2_GOST_CPA = 1<<12,
	GNUTLS_PKCS_PBES2_GOST_CPB = 1<<13,
	GNUTLS_PKCS_PBES2_GOST_CPC = 1<<14,
	GNUTLS_PKCS_PBES2_GOST_CPD = 1<<15
} gnutls_pkcs_encrypt_flags_t;

#define GNUTLS_PKCS_CIPHER_MASK(x) ((x)&(~(GNUTLS_PKCS_NULL_PASSWORD)))

#define GNUTLS_PKCS_USE_PKCS12_3DES GNUTLS_PKCS_PKCS12_3DES
#define GNUTLS_PKCS_USE_PKCS12_ARCFOUR GNUTLS_PKCS_PKCS12_ARCFOUR
#define GNUTLS_PKCS_USE_PKCS12_RC2_40 GNUTLS_PKCS_PKCS12_RC2_40
#define GNUTLS_PKCS_USE_PBES2_3DES GNUTLS_PKCS_PBES2_3DES
#define GNUTLS_PKCS_USE_PBES2_AES_128 GNUTLS_PKCS_PBES2_AES_128
#define GNUTLS_PKCS_USE_PBES2_AES_192 GNUTLS_PKCS_PBES2_AES_192
#define GNUTLS_PKCS_USE_PBES2_AES_256 GNUTLS_PKCS_PBES2_AES_256
#define GNUTLS_PKCS_USE_PBES2_GOST_TC26Z GNUTLS_PKCS_PBES2_GOST_TC26Z
#define GNUTLS_PKCS_USE_PBES2_GOST_CPA GNUTLS_PKCS_PBES2_GOST_CPA
#define GNUTLS_PKCS_USE_PBES2_GOST_CPB GNUTLS_PKCS_PBES2_GOST_CPB
#define GNUTLS_PKCS_USE_PBES2_GOST_CPC GNUTLS_PKCS_PBES2_GOST_CPC
#define GNUTLS_PKCS_USE_PBES2_GOST_CPD GNUTLS_PKCS_PBES2_GOST_CPD

const char *gnutls_pkcs_schema_get_name(unsigned int schema);
const char *gnutls_pkcs_schema_get_oid(unsigned int schema);

int gnutls_x509_privkey_init(gnutls_x509_privkey_t * key);
void gnutls_x509_privkey_deinit(gnutls_x509_privkey_t key);
gnutls_sec_param_t
gnutls_x509_privkey_sec_param(gnutls_x509_privkey_t key);

void gnutls_x509_privkey_set_pin_function(gnutls_x509_privkey_t key,
				      gnutls_pin_callback_t fn,
				      void *userdata);

int gnutls_x509_privkey_cpy(gnutls_x509_privkey_t dst,
			    gnutls_x509_privkey_t src);
int gnutls_x509_privkey_import(gnutls_x509_privkey_t key,
			       const gnutls_datum_t * data,
			       gnutls_x509_crt_fmt_t format);
int gnutls_x509_privkey_import_pkcs8(gnutls_x509_privkey_t key,
				     const gnutls_datum_t * data,
				     gnutls_x509_crt_fmt_t format,
				     const char *password,
				     unsigned int flags);
int gnutls_x509_privkey_import_openssl(gnutls_x509_privkey_t key,
				       const gnutls_datum_t * data,
				       const char *password);

int
gnutls_pkcs8_info(const gnutls_datum_t * data, gnutls_x509_crt_fmt_t format,
		  unsigned int *schema, unsigned int *cipher,
		  void *salt, unsigned int *salt_size,
		  unsigned int *iter_count, char **oid);

int gnutls_x509_privkey_import2(gnutls_x509_privkey_t key,
				const gnutls_datum_t * data,
				gnutls_x509_crt_fmt_t format,
				const char *password, unsigned int flags);

int gnutls_x509_privkey_import_rsa_raw(gnutls_x509_privkey_t key,
				       const gnutls_datum_t * m,
				       const gnutls_datum_t * e,
				       const gnutls_datum_t * d,
				       const gnutls_datum_t * p,
				       const gnutls_datum_t * q,
				       const gnutls_datum_t * u);
int gnutls_x509_privkey_import_rsa_raw2(gnutls_x509_privkey_t key,
					const gnutls_datum_t * m,
					const gnutls_datum_t * e,
					const gnutls_datum_t * d,
					const gnutls_datum_t * p,
					const gnutls_datum_t * q,
					const gnutls_datum_t * u,
					const gnutls_datum_t * e1,
					const gnutls_datum_t * e2);
int gnutls_x509_privkey_import_ecc_raw(gnutls_x509_privkey_t key,
				       gnutls_ecc_curve_t curve,
				       const gnutls_datum_t * x,
				       const gnutls_datum_t * y,
				       const gnutls_datum_t * k);
int gnutls_x509_privkey_import_gost_raw(gnutls_x509_privkey_t key,
				       gnutls_ecc_curve_t curve,
				       gnutls_digest_algorithm_t digest,
				       gnutls_gost_paramset_t paramset,
				       const gnutls_datum_t * x,
				       const gnutls_datum_t * y,
				       const gnutls_datum_t * k);

int gnutls_x509_privkey_fix(gnutls_x509_privkey_t key);

int gnutls_x509_privkey_export_dsa_raw(gnutls_x509_privkey_t key,
				       gnutls_datum_t * p,
				       gnutls_datum_t * q,
				       gnutls_datum_t * g,
				       gnutls_datum_t * y,
				       gnutls_datum_t * x);
int gnutls_x509_privkey_import_dsa_raw(gnutls_x509_privkey_t key,
				       const gnutls_datum_t * p,
				       const gnutls_datum_t * q,
				       const gnutls_datum_t * g,
				       const gnutls_datum_t * y,
				       const gnutls_datum_t * x);

int gnutls_x509_privkey_get_pk_algorithm(gnutls_x509_privkey_t key);
int gnutls_x509_privkey_get_pk_algorithm2(gnutls_x509_privkey_t
					  key, unsigned int *bits);
int gnutls_x509_privkey_get_spki(gnutls_x509_privkey_t key,
				 gnutls_x509_spki_t spki,
				 unsigned int flags);
int
gnutls_x509_privkey_set_spki(gnutls_x509_privkey_t key,
			     const gnutls_x509_spki_t spki,
			     unsigned int flags);

int gnutls_x509_privkey_get_key_id(gnutls_x509_privkey_t key,
				   unsigned int flags,
				   unsigned char *output_data,
				   size_t * output_data_size);

int gnutls_x509_privkey_generate(gnutls_x509_privkey_t key,
				 gnutls_pk_algorithm_t algo,
				 unsigned int bits, unsigned int flags);

void gnutls_x509_privkey_set_flags(gnutls_x509_privkey_t key, unsigned int flags);

/**
 * gnutls_keygen_types_t:
 * @GNUTLS_KEYGEN_SEED: Specifies the seed to be used in key generation.
 * @GNUTLS_KEYGEN_DIGEST: The size field specifies the hash algorithm to be used in key generation.
 * @GNUTLS_KEYGEN_SPKI: data points to a %gnutls_x509_spki_t structure; it is not used after the key generation call.
 *
 * Enumeration of different key generation data options.
 */
typedef enum {
	GNUTLS_KEYGEN_SEED = 1,
	GNUTLS_KEYGEN_DIGEST = 2,
	GNUTLS_KEYGEN_SPKI = 3
} gnutls_keygen_types_t;

typedef struct {
	gnutls_keygen_types_t type;
	unsigned char *data;
	unsigned int size;
} gnutls_keygen_data_st;

int
gnutls_x509_privkey_generate2(gnutls_x509_privkey_t key,
			      gnutls_pk_algorithm_t algo, unsigned int bits,
			      unsigned int flags, const gnutls_keygen_data_st *data, unsigned data_size);

int gnutls_x509_privkey_verify_seed(gnutls_x509_privkey_t key, gnutls_digest_algorithm_t, const void *seed, size_t seed_size);
int gnutls_x509_privkey_get_seed(gnutls_x509_privkey_t key, gnutls_digest_algorithm_t*, void *seed, size_t *seed_size);

int gnutls_x509_privkey_verify_params(gnutls_x509_privkey_t key);

int gnutls_x509_privkey_export(gnutls_x509_privkey_t key,
			       gnutls_x509_crt_fmt_t format,
			       void *output_data,
			       size_t * output_data_size);
int gnutls_x509_privkey_export2(gnutls_x509_privkey_t key,
				gnutls_x509_crt_fmt_t format,
				gnutls_datum_t * out);
int gnutls_x509_privkey_export_pkcs8(gnutls_x509_privkey_t key,
				     gnutls_x509_crt_fmt_t format,
				     const char *password,
				     unsigned int flags,
				     void *output_data,
				     size_t * output_data_size);
int gnutls_x509_privkey_export2_pkcs8(gnutls_x509_privkey_t key,
				      gnutls_x509_crt_fmt_t format,
				      const char *password,
				      unsigned int flags,
				      gnutls_datum_t * out);
int gnutls_x509_privkey_export_rsa_raw2(gnutls_x509_privkey_t key,
					gnutls_datum_t * m,
					gnutls_datum_t * e,
					gnutls_datum_t * d,
					gnutls_datum_t * p,
					gnutls_datum_t * q,
					gnutls_datum_t * u,
					gnutls_datum_t * e1,
					gnutls_datum_t * e2);
int gnutls_x509_privkey_export_rsa_raw(gnutls_x509_privkey_t key,
				       gnutls_datum_t * m,
				       gnutls_datum_t * e,
				       gnutls_datum_t * d,
				       gnutls_datum_t * p,
				       gnutls_datum_t * q,
				       gnutls_datum_t * u);
int gnutls_x509_privkey_export_ecc_raw(gnutls_x509_privkey_t key,
				       gnutls_ecc_curve_t * curve,
				       gnutls_datum_t * x,
				       gnutls_datum_t * y,
				       gnutls_datum_t * k);
int gnutls_x509_privkey_export_gost_raw(gnutls_x509_privkey_t key,
				       gnutls_ecc_curve_t * curve,
				       gnutls_digest_algorithm_t * digest,
				       gnutls_gost_paramset_t * paramset,
				       gnutls_datum_t * x,
				       gnutls_datum_t * y,
				       gnutls_datum_t * k);

int gnutls_x509_privkey_sign_data(gnutls_x509_privkey_t key,
				  gnutls_digest_algorithm_t digest,
				  unsigned int flags,
				  const gnutls_datum_t * data,
				  void *signature,
				  size_t * signature_size);

/* Certificate request stuff.
 */
int gnutls_x509_crq_sign(gnutls_x509_crq_t crq,
			 gnutls_x509_privkey_t key);

int gnutls_x509_crq_sign2(gnutls_x509_crq_t crq,
			  gnutls_x509_privkey_t key,
			  gnutls_digest_algorithm_t dig,
			  unsigned int flags);

int gnutls_x509_crq_print(gnutls_x509_crq_t crq,
			  gnutls_certificate_print_formats_t
			  format, gnutls_datum_t * out);

int gnutls_x509_crq_verify(gnutls_x509_crq_t crq, unsigned int flags);

int gnutls_x509_crq_init(gnutls_x509_crq_t * crq);
void gnutls_x509_crq_deinit(gnutls_x509_crq_t crq);
int gnutls_x509_crq_import(gnutls_x509_crq_t crq,
			   const gnutls_datum_t * data,
			   gnutls_x509_crt_fmt_t format);

int gnutls_x509_crq_get_private_key_usage_period(gnutls_x509_crq_t
						 cert,
						 time_t *
						 activation,
						 time_t *
						 expiration, unsigned int
						 *critical);

int gnutls_x509_crq_get_dn(gnutls_x509_crq_t crq, char *buf,
			   size_t * sizeof_buf);
int gnutls_x509_crq_get_dn2(gnutls_x509_crq_t crq, gnutls_datum_t * dn);
int gnutls_x509_crq_get_dn3(gnutls_x509_crq_t crq, gnutls_datum_t * dn, unsigned flags);
int gnutls_x509_crq_get_dn_oid(gnutls_x509_crq_t crq, unsigned indx,
			       void *oid, size_t * sizeof_oid);
int gnutls_x509_crq_get_dn_by_oid(gnutls_x509_crq_t crq,
				  const char *oid, unsigned indx,
				  unsigned int raw_flag, void *buf,
				  size_t * sizeof_buf);
int gnutls_x509_crq_set_dn(gnutls_x509_crq_t crq, const char *dn,
			   const char **err);
int gnutls_x509_crq_set_dn_by_oid(gnutls_x509_crq_t crq,
				  const char *oid,
				  unsigned int raw_flag,
				  const void *data,
				  unsigned int sizeof_data);
int gnutls_x509_crq_set_version(gnutls_x509_crq_t crq,
				unsigned int version);
int gnutls_x509_crq_get_version(gnutls_x509_crq_t crq);
int gnutls_x509_crq_set_key(gnutls_x509_crq_t crq,
			    gnutls_x509_privkey_t key);

int
gnutls_x509_crq_set_extension_by_oid(gnutls_x509_crq_t crq,
				     const char *oid, const void *buf,
				     size_t sizeof_buf,
				     unsigned int critical);

int gnutls_x509_crq_set_challenge_password(gnutls_x509_crq_t crq,
					   const char *pass);
int gnutls_x509_crq_get_challenge_password(gnutls_x509_crq_t crq,
					   char *pass,
					   size_t * sizeof_pass);

int gnutls_x509_crq_set_attribute_by_oid(gnutls_x509_crq_t crq,
					 const char *oid,
					 void *buf, size_t sizeof_buf);
int gnutls_x509_crq_get_attribute_by_oid(gnutls_x509_crq_t crq,
					 const char *oid, unsigned indx,
					 void *buf, size_t * sizeof_buf);

int gnutls_x509_crq_export(gnutls_x509_crq_t crq,
			   gnutls_x509_crt_fmt_t format,
			   void *output_data, size_t * output_data_size);
int gnutls_x509_crq_export2(gnutls_x509_crq_t crq,
			    gnutls_x509_crt_fmt_t format,
			    gnutls_datum_t * out);

int gnutls_x509_crt_set_crq(gnutls_x509_crt_t crt, gnutls_x509_crq_t crq);
int gnutls_x509_crt_set_crq_extensions(gnutls_x509_crt_t crt,
				       gnutls_x509_crq_t crq);

int
gnutls_x509_crt_set_crq_extension_by_oid(gnutls_x509_crt_t crt,
				         gnutls_x509_crq_t crq, const char *oid,
				         unsigned flags);

int gnutls_x509_crq_set_private_key_usage_period(gnutls_x509_crq_t
						 crq,
						 time_t activation,
						 time_t expiration);
int gnutls_x509_crq_set_key_rsa_raw(gnutls_x509_crq_t crq,
				    const gnutls_datum_t * m,
				    const gnutls_datum_t * e);
int gnutls_x509_crq_set_subject_alt_name(gnutls_x509_crq_t crq,
					 gnutls_x509_subject_alt_name_t
					 nt, const void *data,
					 unsigned int data_size,
					 unsigned int flags);

int
gnutls_x509_crq_set_subject_alt_othername(gnutls_x509_crq_t crq,
				     const char *oid,
				     const void *data,
				     unsigned int data_size,
				     unsigned int flags);

int gnutls_x509_crq_set_key_usage(gnutls_x509_crq_t crq,
				  unsigned int usage);
int gnutls_x509_crq_set_basic_constraints(gnutls_x509_crq_t crq,
					  unsigned int ca,
					  int pathLenConstraint);
int gnutls_x509_crq_set_key_purpose_oid(gnutls_x509_crq_t crq,
					const void *oid,
					unsigned int critical);
int gnutls_x509_crq_get_key_purpose_oid(gnutls_x509_crq_t crq,
					unsigned indx, void *oid,
					size_t * sizeof_oid,
					unsigned int *critical);

int gnutls_x509_crq_get_extension_data(gnutls_x509_crq_t crq,
				       unsigned indx, void *data,
				       size_t * sizeof_data);
int
gnutls_x509_crq_get_extension_data2(gnutls_x509_crq_t crq,
			       unsigned indx,
			       gnutls_datum_t * data);
int gnutls_x509_crq_get_extension_info(gnutls_x509_crq_t crq,
				       unsigned indx, void *oid,
				       size_t * sizeof_oid,
				       unsigned int *critical);
int gnutls_x509_crq_get_attribute_data(gnutls_x509_crq_t crq,
				       unsigned indx, void *data,
				       size_t * sizeof_data);
int gnutls_x509_crq_get_attribute_info(gnutls_x509_crq_t crq,
				       unsigned indx, void *oid,
				       size_t * sizeof_oid);
int gnutls_x509_crq_get_pk_algorithm(gnutls_x509_crq_t crq,
				     unsigned int *bits);
int gnutls_x509_crq_get_spki(gnutls_x509_crq_t crq, gnutls_x509_spki_t spki,
			     unsigned int flags);

int gnutls_x509_crq_set_spki(gnutls_x509_crq_t crq, const gnutls_x509_spki_t spki,
			     unsigned int flags);

int gnutls_x509_crq_get_signature_oid(gnutls_x509_crq_t crq, char *oid, size_t *oid_size);
int gnutls_x509_crq_get_pk_oid(gnutls_x509_crq_t crq, char *oid, size_t *oid_size);

int gnutls_x509_crq_get_key_id(gnutls_x509_crq_t crq,
			       unsigned int flags,
			       unsigned char *output_data,
			       size_t * output_data_size);
int gnutls_x509_crq_get_key_rsa_raw(gnutls_x509_crq_t crq,
				    gnutls_datum_t * m,
				    gnutls_datum_t * e);

int gnutls_x509_crq_get_key_usage(gnutls_x509_crq_t crq,
				  unsigned int *key_usage,
				  unsigned int *critical);
int gnutls_x509_crq_get_basic_constraints(gnutls_x509_crq_t crq,
					  unsigned int *critical,
					  unsigned int *ca, int *pathlen);
int gnutls_x509_crq_get_subject_alt_name(gnutls_x509_crq_t crq,
					 unsigned int seq,
					 void *ret,
					 size_t * ret_size,
					 unsigned int *ret_type,
					 unsigned int *critical);
int gnutls_x509_crq_get_subject_alt_othername_oid(gnutls_x509_crq_t
						  crq,
						  unsigned int seq,
						  void *ret,
						  size_t * ret_size);

int gnutls_x509_crq_get_extension_by_oid(gnutls_x509_crq_t crq,
					 const char *oid, unsigned indx,
					 void *buf,
					 size_t * sizeof_buf,
					 unsigned int *critical);

int gnutls_x509_crq_get_tlsfeatures(gnutls_x509_crq_t crq,
				    gnutls_x509_tlsfeatures_t features,
				    unsigned flags,
				    unsigned int *critical);
int gnutls_x509_crq_set_tlsfeatures(gnutls_x509_crq_t crq,
				    gnutls_x509_tlsfeatures_t features);

int
gnutls_x509_crt_get_extension_by_oid2(gnutls_x509_crt_t cert,
				     const char *oid, unsigned indx,
				     gnutls_datum_t *output,
				     unsigned int *critical);

typedef struct gnutls_x509_trust_list_st *gnutls_x509_trust_list_t;
typedef struct gnutls_x509_trust_list_iter *gnutls_x509_trust_list_iter_t;

int
gnutls_x509_trust_list_init(gnutls_x509_trust_list_t * list,
			    unsigned int size);

void
gnutls_x509_trust_list_deinit(gnutls_x509_trust_list_t list,
			      unsigned int all);

int gnutls_x509_trust_list_get_issuer(gnutls_x509_trust_list_t
				      list, gnutls_x509_crt_t cert,
				      gnutls_x509_crt_t * issuer,
				      unsigned int flags);

int gnutls_x509_trust_list_get_issuer_by_dn(gnutls_x509_trust_list_t list,
				      const gnutls_datum_t *dn,
				      gnutls_x509_crt_t *issuer,
				      unsigned int flags);

int gnutls_x509_trust_list_get_issuer_by_subject_key_id(gnutls_x509_trust_list_t list,
				      const gnutls_datum_t *dn,
				      const gnutls_datum_t *spki,
				      gnutls_x509_crt_t *issuer,
				      unsigned int flags);
/**
 * gnutls_trust_list_flags_t:
 * @GNUTLS_TL_VERIFY_CRL: If any CRLs are provided they will be verified for validity
 *   prior to be added. The CA certificates that will be used for verification are the
 *   ones already added in the trusted list.
 * @GNUTLS_TL_USE_IN_TLS: Internal flag used by GnuTLS. If provided the trust list
 *   structure will cache a copy of CA DNs to be used in the certificate request
 *   TLS message.
 * @GNUTLS_TL_NO_DUPLICATES: If this flag is specified, a function adding certificates
 *   will check and eliminate any duplicates.
 * @GNUTLS_TL_NO_DUPLICATE_KEY: If this flag is specified, a certificate sharing the
 *   same key as a previously added on will not be added.
 * @GNUTLS_TL_GET_COPY: The semantics of this flag are documented to the functions which
 *   are applicable. In general, on returned value, the function will provide a copy
 *   if this flag is provided, rather than a pointer to internal data.
 * @GNUTLS_TL_FAIL_ON_INVALID_CRL: If an CRL is added which cannot be validated return
 *   an error instead of ignoring (must be used with %GNUTLS_TL_VERIFY_CRL).
 *
 * Enumeration of different certificate trust list flags.
 */
typedef enum gnutls_trust_list_flags_t {
	GNUTLS_TL_VERIFY_CRL = 1,
#define GNUTLS_TL_VERIFY_CRL 1
	GNUTLS_TL_USE_IN_TLS = (1<<1),
#define GNUTLS_TL_USE_IN_TLS (1<<1)
	GNUTLS_TL_NO_DUPLICATES = (1<<2),
#define GNUTLS_TL_NO_DUPLICATES (1<<2)
	GNUTLS_TL_NO_DUPLICATE_KEY = (1<<3),
#define GNUTLS_TL_NO_DUPLICATE_KEY (1<<3)
	GNUTLS_TL_GET_COPY = (1<<4),
#define GNUTLS_TL_GET_COPY (1<<4)
	GNUTLS_TL_FAIL_ON_INVALID_CRL = (1<<5)
#define GNUTLS_TL_FAIL_ON_INVALID_CRL (1<<5)
} gnutls_trust_list_flags_t;

int
gnutls_x509_trust_list_add_cas(gnutls_x509_trust_list_t list,
			       const gnutls_x509_crt_t * clist,
			       unsigned clist_size, unsigned int flags);
int gnutls_x509_trust_list_remove_cas(gnutls_x509_trust_list_t
				      list,
				      const gnutls_x509_crt_t *
				      clist, unsigned clist_size);

int gnutls_x509_trust_list_add_named_crt(gnutls_x509_trust_list_t
					 list,
					 gnutls_x509_crt_t cert,
					 const void *name,
					 size_t name_size,
					 unsigned int flags);

int
gnutls_x509_trust_list_add_crls(gnutls_x509_trust_list_t list,
				const gnutls_x509_crl_t *
				crl_list, unsigned crl_size,
				unsigned int flags,
				unsigned int verification_flags);


int
gnutls_x509_trust_list_iter_get_ca(gnutls_x509_trust_list_t list,
                                   gnutls_x509_trust_list_iter_t *iter,
                                   gnutls_x509_crt_t *crt);

void gnutls_x509_trust_list_iter_deinit(gnutls_x509_trust_list_iter_t iter);

typedef int gnutls_verify_output_function(gnutls_x509_crt_t cert, gnutls_x509_crt_t issuer,
												 /* The issuer if verification failed
												 * because of him. might be null.
												 */
					  gnutls_x509_crl_t crl,	/* The CRL that caused verification failure 
									 * if any. Might be null.
									 */
					  unsigned int
					  verification_output);

void gnutls_session_set_verify_output_function(gnutls_session_t session,
		gnutls_verify_output_function * func);

int gnutls_x509_trust_list_verify_named_crt
    (gnutls_x509_trust_list_t list, gnutls_x509_crt_t cert,
     const void *name, size_t name_size, unsigned int flags,
     unsigned int *verify, gnutls_verify_output_function func);

int
gnutls_x509_trust_list_verify_crt2(gnutls_x509_trust_list_t list,
				  gnutls_x509_crt_t * cert_list,
				  unsigned int cert_list_size,
				  gnutls_typed_vdata_st * data,
				  unsigned int elements,
				  unsigned int flags,
				  unsigned int *voutput,
				  gnutls_verify_output_function func);

int
gnutls_x509_trust_list_verify_crt(gnutls_x509_trust_list_t list,
				  gnutls_x509_crt_t * cert_list,
				  unsigned int cert_list_size,
				  unsigned int flags,
				  unsigned int *verify,
				  gnutls_verify_output_function func);

	/* trust list convenience functions */
int
gnutls_x509_trust_list_add_trust_mem(gnutls_x509_trust_list_t
				     list,
				     const gnutls_datum_t * cas,
				     const gnutls_datum_t * crls,
				     gnutls_x509_crt_fmt_t type,
				     unsigned int tl_flags,
				     unsigned int tl_vflags);

int
gnutls_x509_trust_list_add_trust_file(gnutls_x509_trust_list_t
				      list, const char *ca_file,
				      const char *crl_file,
				      gnutls_x509_crt_fmt_t type,
				      unsigned int tl_flags,
				      unsigned int tl_vflags);

int
gnutls_x509_trust_list_add_trust_dir(gnutls_x509_trust_list_t list,
				      const char *ca_dir,
				      const char *crl_dir,
				      gnutls_x509_crt_fmt_t type,
				      unsigned int tl_flags,
				      unsigned int tl_vflags);

int
gnutls_x509_trust_list_remove_trust_file(gnutls_x509_trust_list_t
					 list,
					 const char *ca_file,
					 gnutls_x509_crt_fmt_t type);

int
gnutls_x509_trust_list_remove_trust_mem(gnutls_x509_trust_list_t
					list,
					const gnutls_datum_t *
					cas, gnutls_x509_crt_fmt_t type);

int
gnutls_x509_trust_list_add_system_trust(gnutls_x509_trust_list_t
					list,
					unsigned int tl_flags,
					unsigned int tl_vflags);

typedef int gnutls_x509_trust_list_getissuer_function(gnutls_x509_trust_list_t list,
						      const gnutls_x509_crt_t cert,
						      gnutls_x509_crt_t **issuers,
						      unsigned int *issuers_size);

void gnutls_x509_trust_list_set_getissuer_function(gnutls_x509_trust_list_t tlist,
				gnutls_x509_trust_list_getissuer_function *func);

void gnutls_x509_trust_list_set_ptr(gnutls_x509_trust_list_t tlist, void *ptr);

void *gnutls_x509_trust_list_get_ptr(gnutls_x509_trust_list_t tlist);

void gnutls_certificate_set_trust_list
    (gnutls_certificate_credentials_t res,
     gnutls_x509_trust_list_t tlist, unsigned flags);
void gnutls_certificate_get_trust_list
    (gnutls_certificate_credentials_t res,
     gnutls_x509_trust_list_t *tlist);

typedef struct gnutls_x509_ext_st {
	char *oid;
	unsigned int critical;
	gnutls_datum_t data;
} gnutls_x509_ext_st;

void gnutls_x509_ext_deinit(gnutls_x509_ext_st *ext);

int
gnutls_x509_ext_print(gnutls_x509_ext_st *exts, unsigned int exts_size,
		      gnutls_certificate_print_formats_t format,
		      gnutls_datum_t * out);

#include <gnutls/pkcs7.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_X509_H */
