/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _CERTDB_H_
#define _CERTDB_H_

/* common flags for all types of certificates */
#define CERTDB_TERMINAL_RECORD (1u << 0)
#define CERTDB_TRUSTED (1u << 1)
#define CERTDB_SEND_WARN (1u << 2)
#define CERTDB_VALID_CA (1u << 3)
#define CERTDB_TRUSTED_CA (1u << 4) /* trusted for issuing server certs */
#define CERTDB_NS_TRUSTED_CA (1u << 5)
#define CERTDB_USER (1u << 6)
#define CERTDB_TRUSTED_CLIENT_CA (1u << 7) /* trusted for issuing client certs */
#define CERTDB_INVISIBLE_CA (1u << 8)      /* don't show in UI */
#define CERTDB_GOVT_APPROVED_CA (1u << 9)  /* can do strong crypto in export ver */

/* old usage, to keep old programs compiling */
/* On Windows, Mac, and Linux (and other gcc platforms), we can give compile
 * time deprecation warnings when applications use the old CERTDB_VALID_PEER
 * define */
#if __GNUC__ > 3
#if (__GNUC__ == 4) && (__GNUC_MINOR__ < 5)
typedef unsigned int __CERTDB_VALID_PEER __attribute__((deprecated));
#else
typedef unsigned int __CERTDB_VALID_PEER __attribute__((
    deprecated("CERTDB_VALID_PEER is now CERTDB_TERMINAL_RECORD")));
#endif
#define CERTDB_VALID_PEER ((__CERTDB_VALID_PEER)CERTDB_TERMINAL_RECORD)
#else
#ifdef _WIN32
#pragma deprecated(CERTDB_VALID_PEER)
#endif
#define CERTDB_VALID_PEER CERTDB_TERMINAL_RECORD
#endif

SEC_BEGIN_PROTOS

CERTSignedCrl *SEC_FindCrlByKey(CERTCertDBHandle *handle, SECItem *crlKey,
                                int type);

CERTSignedCrl *SEC_FindCrlByName(CERTCertDBHandle *handle, SECItem *crlKey,
                                 int type);

CERTSignedCrl *SEC_FindCrlByDERCert(CERTCertDBHandle *handle, SECItem *derCrl,
                                    int type);

PRBool SEC_CertNicknameConflict(const char *nickname, const SECItem *derSubject,
                                CERTCertDBHandle *handle);
CERTSignedCrl *SEC_NewCrl(CERTCertDBHandle *handle, char *url, SECItem *derCrl,
                          int type);

SECStatus SEC_DeletePermCRL(CERTSignedCrl *crl);

SECStatus SEC_LookupCrls(CERTCertDBHandle *handle, CERTCrlHeadNode **nodes,
                         int type);

SECStatus SEC_DestroyCrl(CERTSignedCrl *crl);

CERTSignedCrl *SEC_DupCrl(CERTSignedCrl *acrl);

SECStatus CERT_AddTempCertToPerm(CERTCertificate *cert, char *nickname,
                                 CERTCertTrust *trust);

SECStatus SEC_DeletePermCertificate(CERTCertificate *cert);

PRBool SEC_CrlIsNewer(CERTCrl *inNew, CERTCrl *old);

/*
** Extract the validity times from a CRL
**	"crl" is the CRL
**	"notBefore" is the start of the validity period (last update)
**	"notAfter" is the end of the validity period (next update)
*/
SECStatus SEC_GetCrlTimes(CERTCrl *crl, PRTime *notBefore, PRTime *notAfter);

/*
** Check the validity times of a crl vs. time 't', allowing
** some slop for broken clocks and stuff.
**	"crl" is the certificate to be checked
**	"t" is the time to check against
*/
SECCertTimeValidity SEC_CheckCrlTimes(CERTCrl *crl, PRTime t);

SEC_END_PROTOS

#endif /* _CERTDB_H_ */
