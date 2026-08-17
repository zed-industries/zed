/*
 * Copyright (C) 2011-2012 Free Software Foundation, Inc.
 *
 * Author: Simon Josefsson
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

/* Online Certificate Status Protocol - RFC 2560
 */

#ifndef GNUTLS_OCSP_H
#define GNUTLS_OCSP_H

#include <gnutls/gnutls.h>
#include <gnutls/x509.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

#define GNUTLS_OCSP_NONCE "1.3.6.1.5.5.7.48.1.2"

/**
 * gnutls_ocsp_print_formats_t:
 * @GNUTLS_OCSP_PRINT_FULL: Full information about OCSP request/response.
 * @GNUTLS_OCSP_PRINT_COMPACT: More compact information about OCSP request/response.
 *
 * Enumeration of different OCSP printing variants.
 */
typedef enum gnutls_ocsp_print_formats_t {
	GNUTLS_OCSP_PRINT_FULL = 0,
	GNUTLS_OCSP_PRINT_COMPACT = 1
} gnutls_ocsp_print_formats_t;

/**
 * gnutls_ocsp_resp_status_t:
 * @GNUTLS_OCSP_RESP_SUCCESSFUL: Response has valid confirmations.
 * @GNUTLS_OCSP_RESP_MALFORMEDREQUEST: Illegal confirmation request
 * @GNUTLS_OCSP_RESP_INTERNALERROR: Internal error in issuer
 * @GNUTLS_OCSP_RESP_TRYLATER: Try again later
 * @GNUTLS_OCSP_RESP_SIGREQUIRED: Must sign the request
 * @GNUTLS_OCSP_RESP_UNAUTHORIZED: Request unauthorized
 *
 * Enumeration of different OCSP response status codes.
 */
typedef enum gnutls_ocsp_resp_status_t {
	GNUTLS_OCSP_RESP_SUCCESSFUL = 0,
	GNUTLS_OCSP_RESP_MALFORMEDREQUEST = 1,
	GNUTLS_OCSP_RESP_INTERNALERROR = 2,
	GNUTLS_OCSP_RESP_TRYLATER = 3,
	GNUTLS_OCSP_RESP_SIGREQUIRED = 5,
	GNUTLS_OCSP_RESP_UNAUTHORIZED = 6
} gnutls_ocsp_resp_status_t;

/**
 * gnutls_ocsp_cert_status_t:
 * @GNUTLS_OCSP_CERT_GOOD: Positive response to status inquiry.
 * @GNUTLS_OCSP_CERT_REVOKED: Certificate has been revoked.
 * @GNUTLS_OCSP_CERT_UNKNOWN: The responder doesn't know about the
 *   certificate.
 *
 * Enumeration of different OCSP response certificate status codes.
 */
typedef enum gnutls_ocsp_cert_status_t {
	GNUTLS_OCSP_CERT_GOOD = 0,
	GNUTLS_OCSP_CERT_REVOKED = 1,
	GNUTLS_OCSP_CERT_UNKNOWN = 2
} gnutls_ocsp_cert_status_t;

/**
 * gnutls_x509_crl_reason_t:
 * @GNUTLS_X509_CRLREASON_UNSPECIFIED: Unspecified reason.
 * @GNUTLS_X509_CRLREASON_KEYCOMPROMISE: Private key compromised.
 * @GNUTLS_X509_CRLREASON_CACOMPROMISE: CA compromised.
 * @GNUTLS_X509_CRLREASON_AFFILIATIONCHANGED: Affiliation has changed.
 * @GNUTLS_X509_CRLREASON_SUPERSEDED: Certificate superseded.
 * @GNUTLS_X509_CRLREASON_CESSATIONOFOPERATION: Operation has ceased.
 * @GNUTLS_X509_CRLREASON_CERTIFICATEHOLD: Certificate is on hold.
 * @GNUTLS_X509_CRLREASON_REMOVEFROMCRL: Will be removed from delta CRL.
 * @GNUTLS_X509_CRLREASON_PRIVILEGEWITHDRAWN: Privilege withdrawn.
 * @GNUTLS_X509_CRLREASON_AACOMPROMISE: AA compromised.
 *
 * Enumeration of different reason codes.  Note that this
 * corresponds to the CRLReason ASN.1 enumeration type, and not the
 * ReasonFlags ASN.1 bit string.
 */
typedef enum gnutls_x509_crl_reason_t {
	GNUTLS_X509_CRLREASON_UNSPECIFIED = 0,
	GNUTLS_X509_CRLREASON_KEYCOMPROMISE = 1,
	GNUTLS_X509_CRLREASON_CACOMPROMISE = 2,
	GNUTLS_X509_CRLREASON_AFFILIATIONCHANGED = 3,
	GNUTLS_X509_CRLREASON_SUPERSEDED = 4,
	GNUTLS_X509_CRLREASON_CESSATIONOFOPERATION = 5,
	GNUTLS_X509_CRLREASON_CERTIFICATEHOLD = 6,
	GNUTLS_X509_CRLREASON_REMOVEFROMCRL = 8,
	GNUTLS_X509_CRLREASON_PRIVILEGEWITHDRAWN = 9,
	GNUTLS_X509_CRLREASON_AACOMPROMISE = 10
} gnutls_x509_crl_reason_t;

/* When adding a verify failure reason update:
 * _gnutls_ocsp_verify_status_to_str()
 */
/**
 * gnutls_ocsp_verify_reason_t:
 * @GNUTLS_OCSP_VERIFY_SIGNER_NOT_FOUND: Signer cert not found.
 * @GNUTLS_OCSP_VERIFY_SIGNER_KEYUSAGE_ERROR: Signer keyusage bits incorrect.
 * @GNUTLS_OCSP_VERIFY_UNTRUSTED_SIGNER: Signer is not trusted.
 * @GNUTLS_OCSP_VERIFY_INSECURE_ALGORITHM: Signature using insecure algorithm.
 * @GNUTLS_OCSP_VERIFY_SIGNATURE_FAILURE: Signature mismatch.
 * @GNUTLS_OCSP_VERIFY_CERT_NOT_ACTIVATED: Signer cert is not yet activated.
 * @GNUTLS_OCSP_VERIFY_CERT_EXPIRED: Signer cert has expired.
 *
 * Enumeration of OCSP verify status codes, used by
 * gnutls_ocsp_resp_verify() and gnutls_ocsp_resp_verify_direct().
 */
typedef enum gnutls_ocsp_verify_reason_t {
	GNUTLS_OCSP_VERIFY_SIGNER_NOT_FOUND = 1,
	GNUTLS_OCSP_VERIFY_SIGNER_KEYUSAGE_ERROR = 2,
	GNUTLS_OCSP_VERIFY_UNTRUSTED_SIGNER = 4,
	GNUTLS_OCSP_VERIFY_INSECURE_ALGORITHM = 8,
	GNUTLS_OCSP_VERIFY_SIGNATURE_FAILURE = 16,
	GNUTLS_OCSP_VERIFY_CERT_NOT_ACTIVATED = 32,
	GNUTLS_OCSP_VERIFY_CERT_EXPIRED = 64
} gnutls_ocsp_verify_reason_t;

struct gnutls_ocsp_req_int;
typedef struct gnutls_ocsp_req_int *gnutls_ocsp_req_t;
typedef const struct gnutls_ocsp_req_int *gnutls_ocsp_req_const_t;

int gnutls_ocsp_req_init(gnutls_ocsp_req_t * req);
void gnutls_ocsp_req_deinit(gnutls_ocsp_req_t req);

int gnutls_ocsp_req_import(gnutls_ocsp_req_t req,
			   const gnutls_datum_t * data);
int gnutls_ocsp_req_export(gnutls_ocsp_req_const_t req, gnutls_datum_t * data);
int gnutls_ocsp_req_print(gnutls_ocsp_req_const_t req,
			  gnutls_ocsp_print_formats_t format,
			  gnutls_datum_t * out);

int gnutls_ocsp_req_get_version(gnutls_ocsp_req_const_t req);

int gnutls_ocsp_req_get_cert_id(gnutls_ocsp_req_const_t req,
				unsigned indx,
				gnutls_digest_algorithm_t * digest,
				gnutls_datum_t * issuer_name_hash,
				gnutls_datum_t * issuer_key_hash,
				gnutls_datum_t * serial_number);
int gnutls_ocsp_req_add_cert_id(gnutls_ocsp_req_t req,
				gnutls_digest_algorithm_t digest,
				const gnutls_datum_t *
				issuer_name_hash,
				const gnutls_datum_t *
				issuer_key_hash,
				const gnutls_datum_t * serial_number);
int gnutls_ocsp_req_add_cert(gnutls_ocsp_req_t req,
			     gnutls_digest_algorithm_t digest,
			     gnutls_x509_crt_t issuer,
			     gnutls_x509_crt_t cert);

int gnutls_ocsp_req_get_extension(gnutls_ocsp_req_const_t req,
				  unsigned indx,
				  gnutls_datum_t * oid,
				  unsigned int *critical,
				  gnutls_datum_t * data);
int gnutls_ocsp_req_set_extension(gnutls_ocsp_req_t req,
				  const char *oid,
				  unsigned int critical,
				  const gnutls_datum_t * data);

int gnutls_ocsp_req_get_nonce(gnutls_ocsp_req_const_t req,
			      unsigned int *critical,
			      gnutls_datum_t * nonce);
int gnutls_ocsp_req_set_nonce(gnutls_ocsp_req_t req,
			      unsigned int critical,
			      const gnutls_datum_t * nonce);
int gnutls_ocsp_req_randomize_nonce(gnutls_ocsp_req_t req);

struct gnutls_ocsp_resp_int;
typedef struct gnutls_ocsp_resp_int *gnutls_ocsp_resp_t;
typedef const struct gnutls_ocsp_resp_int *gnutls_ocsp_resp_const_t;

int gnutls_ocsp_resp_init(gnutls_ocsp_resp_t * resp);
void gnutls_ocsp_resp_deinit(gnutls_ocsp_resp_t resp);

int gnutls_ocsp_resp_import(gnutls_ocsp_resp_t resp,
			    const gnutls_datum_t * data);
int gnutls_ocsp_resp_import2(gnutls_ocsp_resp_t resp,
			     const gnutls_datum_t * data,
			     gnutls_x509_crt_fmt_t fmt);
int gnutls_ocsp_resp_export(gnutls_ocsp_resp_const_t resp,
			    gnutls_datum_t * data);
int gnutls_ocsp_resp_export2(gnutls_ocsp_resp_const_t resp,
			     gnutls_datum_t * data,
			     gnutls_x509_crt_fmt_t fmt);
int gnutls_ocsp_resp_print(gnutls_ocsp_resp_const_t resp,
			   gnutls_ocsp_print_formats_t format,
			   gnutls_datum_t * out);

int gnutls_ocsp_resp_get_status(gnutls_ocsp_resp_const_t resp);
int gnutls_ocsp_resp_get_response(gnutls_ocsp_resp_const_t resp,
				  gnutls_datum_t *
				  response_type_oid,
				  gnutls_datum_t * response);

int gnutls_ocsp_resp_get_version(gnutls_ocsp_resp_const_t resp);
int gnutls_ocsp_resp_get_responder(gnutls_ocsp_resp_const_t resp,
				   gnutls_datum_t * dn);
int gnutls_ocsp_resp_get_responder2(gnutls_ocsp_resp_const_t resp,
				    gnutls_datum_t * dn,
				    unsigned flags);

/* the raw key ID of the responder */
#define GNUTLS_OCSP_RESP_ID_KEY 1
/* the raw DN of the responder */
#define GNUTLS_OCSP_RESP_ID_DN 2
int
gnutls_ocsp_resp_get_responder_raw_id(gnutls_ocsp_resp_const_t resp,
				      unsigned type,
				      gnutls_datum_t * raw);

time_t gnutls_ocsp_resp_get_produced(gnutls_ocsp_resp_const_t resp);
int gnutls_ocsp_resp_get_single(gnutls_ocsp_resp_const_t resp,
				unsigned indx,
				gnutls_digest_algorithm_t * digest,
				gnutls_datum_t * issuer_name_hash,
				gnutls_datum_t * issuer_key_hash,
				gnutls_datum_t * serial_number,
				unsigned int *cert_status,
				time_t * this_update,
				time_t * next_update,
				time_t * revocation_time,
				unsigned int *revocation_reason);
int gnutls_ocsp_resp_get_extension(gnutls_ocsp_resp_const_t resp,
				   unsigned indx,
				   gnutls_datum_t * oid,
				   unsigned int *critical,
				   gnutls_datum_t * data);
int gnutls_ocsp_resp_get_nonce(gnutls_ocsp_resp_const_t resp,
			       unsigned int *critical,
			       gnutls_datum_t * nonce);
int gnutls_ocsp_resp_get_signature_algorithm(gnutls_ocsp_resp_const_t resp);
int gnutls_ocsp_resp_get_signature(gnutls_ocsp_resp_const_t resp,
				   gnutls_datum_t * sig);
int gnutls_ocsp_resp_get_certs(gnutls_ocsp_resp_const_t resp,
			       gnutls_x509_crt_t ** certs,
			       size_t * ncerts);

int gnutls_ocsp_resp_verify_direct(gnutls_ocsp_resp_const_t resp,
				   gnutls_x509_crt_t issuer,
				   unsigned int *verify,
				   unsigned int flags);
int gnutls_ocsp_resp_verify(gnutls_ocsp_resp_const_t resp,
			    gnutls_x509_trust_list_t trustlist,
			    unsigned int *verify, unsigned int flags);

int gnutls_ocsp_resp_check_crt(gnutls_ocsp_resp_const_t resp,
			       unsigned int indx, gnutls_x509_crt_t crt);

int
gnutls_ocsp_resp_list_import2(gnutls_ocsp_resp_t **ocsps,
			     unsigned int *size,
			     const gnutls_datum_t *resp_data,
			     gnutls_x509_crt_fmt_t format,
			     unsigned int flags);

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_OCSP_H */
