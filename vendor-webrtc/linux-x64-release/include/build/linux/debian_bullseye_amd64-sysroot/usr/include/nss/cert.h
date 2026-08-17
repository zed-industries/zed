/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * cert.h - public data structures and prototypes for the certificate library
 */

#ifndef _CERT_H_
#define _CERT_H_

#include "utilrename.h"
#include "plarena.h"
#include "plhash.h"
#include "prlong.h"
#include "prlog.h"

#include "seccomon.h"
#include "secdert.h"
#include "secoidt.h"
#include "keythi.h"
#include "certt.h"

SEC_BEGIN_PROTOS

/****************************************************************************
 *
 * RFC1485 ascii to/from X.? RelativeDistinguishedName (aka CERTName)
 *
 ****************************************************************************/

/*
** Convert an ascii RFC1485 encoded name into its CERTName equivalent.
*/
extern CERTName *CERT_AsciiToName(const char *string);

/*
** Convert an CERTName into its RFC1485 encoded equivalent.
** Returns a string that must be freed with PORT_Free().
** This version produces a string for maximum human readability,
** not for strict RFC compliance.
*/
extern char *CERT_NameToAscii(CERTName *name);

/*
** Convert an CERTName into its RFC1485 encoded equivalent.
** Returns a string that must be freed with PORT_Free().
** Caller chooses encoding rules.
*/
extern char *CERT_NameToAsciiInvertible(CERTName *name,
                                        CertStrictnessLevel strict);

extern CERTAVA *CERT_CopyAVA(PLArenaPool *arena, CERTAVA *src);

/* convert an OID to dotted-decimal representation */
/* Returns a string that must be freed with PR_smprintf_free(). */
extern char *CERT_GetOidString(const SECItem *oid);

/*
** Examine an AVA and return the tag that refers to it. The AVA tags are
** defined as SEC_OID_AVA*.
*/
extern SECOidTag CERT_GetAVATag(CERTAVA *ava);

/*
** Compare two AVA's, returning the difference between them.
*/
extern SECComparison CERT_CompareAVA(const CERTAVA *a, const CERTAVA *b);

/*
** Create an RDN (relative-distinguished-name). The argument list is a
** NULL terminated list of AVA's.
*/
extern CERTRDN *CERT_CreateRDN(PLArenaPool *arena, CERTAVA *avas, ...);

/*
** Make a copy of "src" storing it in "dest".
*/
extern SECStatus CERT_CopyRDN(PLArenaPool *arena, CERTRDN *dest, CERTRDN *src);

/*
** Add an AVA to an RDN.
**	"rdn" the RDN to add to
**	"ava" the AVA to add
*/
extern SECStatus CERT_AddAVA(PLArenaPool *arena, CERTRDN *rdn, CERTAVA *ava);

/*
** Compare two RDN's, returning the difference between them.
*/
extern SECComparison CERT_CompareRDN(const CERTRDN *a, const CERTRDN *b);

/*
** Create an X.500 style name using a NULL terminated list of RDN's.
*/
extern CERTName *CERT_CreateName(CERTRDN *rdn, ...);

/*
** Make a copy of "src" storing it in "dest". Memory is allocated in
** "dest" for each of the appropriate sub objects. Memory is not freed in
** "dest" before allocation is done (use CERT_DestroyName(dest, PR_FALSE) to
** do that).
*/
extern SECStatus CERT_CopyName(PLArenaPool *arena, CERTName *dest,
                               const CERTName *src);

/*
** Destroy a Name object.
**	"name" the CERTName to destroy
**	"freeit" if PR_TRUE then free the object as well as its sub-objects
*/
extern void CERT_DestroyName(CERTName *name);

/*
** Add an RDN to a name.
**	"name" the name to add the RDN to
**	"rdn" the RDN to add to name
*/
extern SECStatus CERT_AddRDN(CERTName *name, CERTRDN *rdn);

/*
** Compare two names, returning the difference between them.
*/
extern SECComparison CERT_CompareName(const CERTName *a, const CERTName *b);

/*
** Convert a CERTName into something readable
*/
extern char *CERT_FormatName(CERTName *name);

/*
** Convert a der-encoded integer to a hex printable string form.
** Perhaps this should be a SEC function but it's only used for certs.
*/
extern char *CERT_Hexify(SECItem *i, int do_colon);

/*
** Converts DER string (with explicit length) into zString, if destination
** buffer is big enough to receive it.  Does quoting and/or escaping as
** specified in RFC 1485.  Input string must be single or multi-byte DER
** character set, (ASCII, UTF8, or ISO 8851-x) not a wide character set.
** Returns SECSuccess or SECFailure with error code set. If output buffer
** is too small, sets error code SEC_ERROR_OUTPUT_LEN.
*/
extern SECStatus CERT_RFC1485_EscapeAndQuote(char *dst, int dstlen, char *src,
                                             int srclen);

/******************************************************************************
 *
 * Certificate handling operations
 *
 *****************************************************************************/

/*
** Create a new validity object given two unix time values.
**	"notBefore" the time before which the validity is not valid
**	"notAfter" the time after which the validity is not valid
*/
extern CERTValidity *CERT_CreateValidity(PRTime notBefore, PRTime notAfter);

/*
** Destroy a validity object.
**	"v" the validity to destroy
**	"freeit" if PR_TRUE then free the object as well as its sub-objects
*/
extern void CERT_DestroyValidity(CERTValidity *v);

/*
** Copy the "src" object to "dest". Memory is allocated in "dest" for
** each of the appropriate sub-objects. Memory in "dest" is not freed
** before memory is allocated (use CERT_DestroyValidity(v, PR_FALSE) to do
** that).
*/
extern SECStatus CERT_CopyValidity(PLArenaPool *arena, CERTValidity *dest,
                                   CERTValidity *src);

/*
** The cert lib considers a cert or CRL valid if the "notBefore" time is
** in the not-too-distant future, e.g. within the next 24 hours. This
** prevents freshly issued certificates from being considered invalid
** because the local system's time zone is incorrectly set.
** The amount of "pending slop time" is adjustable by the application.
** Units of SlopTime are seconds.  Default is 86400  (24 hours).
** Negative SlopTime values are not allowed.
*/
PRInt32 CERT_GetSlopTime(void);

SECStatus CERT_SetSlopTime(PRInt32 slop);

/*
** Create a new certificate object. The result must be wrapped with an
** CERTSignedData to create a signed certificate.
**	"serialNumber" the serial number
**	"issuer" the name of the certificate issuer
**	"validity" the validity period of the certificate
**	"req" the certificate request that prompted the certificate issuance
*/
extern CERTCertificate *CERT_CreateCertificate(unsigned long serialNumber,
                                               CERTName *issuer,
                                               CERTValidity *validity,
                                               CERTCertificateRequest *req);

/*
** Destroy a certificate object
**	"cert" the certificate to destroy
** NOTE: certificate's are reference counted. This call decrements the
** reference count, and if the result is zero, then the object is destroyed
** and optionally freed.
*/
extern void CERT_DestroyCertificate(CERTCertificate *cert);

/*
** Make a shallow copy of a certificate "c". Just increments the
** reference count on "c".
*/
extern CERTCertificate *CERT_DupCertificate(CERTCertificate *c);

/* Access the DER of the certificate. This only creates a reference to the DER
 * in the outparam not a copy.  To avoid the pointer becoming invalid, use
 * CERT_DupCertificate() and keep a reference to the duplicate alive.
 */
extern SECStatus CERT_GetCertificateDer(const CERTCertificate *c, SECItem *der);

/*
** Create a new certificate request. This result must be wrapped with an
** CERTSignedData to create a signed certificate request.
**	"name" the subject name (who the certificate request is from)
**	"spki" describes/defines the public key the certificate is for
**	"attributes" if non-zero, some optional attribute data
*/
extern CERTCertificateRequest *CERT_CreateCertificateRequest(
    CERTName *name, CERTSubjectPublicKeyInfo *spki, SECItem **attributes);

/*
** Destroy a certificate-request object
**	"r" the certificate-request to destroy
**	"freeit" if PR_TRUE then free the object as well as its sub-objects
*/
extern void CERT_DestroyCertificateRequest(CERTCertificateRequest *r);

/*
** Start adding extensions to a certificate request.
*/
void *CERT_StartCertificateRequestAttributes(CERTCertificateRequest *req);

/*
** Reformat the certificate extension list into a CertificateRequest
** attribute list.
*/
SECStatus CERT_FinishCertificateRequestAttributes(CERTCertificateRequest *req);

/*
** Extract the Extension Requests from a DER CertRequest attribute list.
*/
SECStatus CERT_GetCertificateRequestExtensions(CERTCertificateRequest *req,
                                               CERTCertExtension ***exts);

/*
** Extract a public key object from a certificate
*/
extern SECKEYPublicKey *CERT_ExtractPublicKey(CERTCertificate *cert);

/*
** Retrieve the Key Type associated with the cert we're dealing with
*/

extern KeyType CERT_GetCertKeyType(const CERTSubjectPublicKeyInfo *spki);

/*
** Initialize the certificate database.  This is called to create
**  the initial list of certificates in the database.
*/
extern SECStatus CERT_InitCertDB(CERTCertDBHandle *handle);

extern int CERT_GetDBContentVersion(CERTCertDBHandle *handle);

/*
** Default certificate database routines
*/
extern void CERT_SetDefaultCertDB(CERTCertDBHandle *handle);

extern CERTCertDBHandle *CERT_GetDefaultCertDB(void);

extern CERTCertList *CERT_GetCertChainFromCert(CERTCertificate *cert,
                                               PRTime time, SECCertUsage usage);
extern CERTCertificate *CERT_NewTempCertificate(CERTCertDBHandle *handle,
                                                SECItem *derCert,
                                                char *nickname, PRBool isperm,
                                                PRBool copyDER);

/******************************************************************************
 *
 * X.500 Name handling operations
 *
 *****************************************************************************/

/*
** Create an AVA (attribute-value-assertion)
**	"arena" the memory arena to alloc from
**	"kind" is one of SEC_OID_AVA_*
**	"valueType" is one of DER_PRINTABLE_STRING, DER_IA5_STRING, or
**	   DER_T61_STRING
**	"value" is the null terminated string containing the value
*/
extern CERTAVA *CERT_CreateAVA(PLArenaPool *arena, SECOidTag kind,
                               int valueType, char *value);

/*
** Extract the Distinguished Name from a DER encoded certificate
**	"derCert" is the DER encoded certificate
**	"derName" is the SECItem that the name is returned in
*/
extern SECStatus CERT_NameFromDERCert(SECItem *derCert, SECItem *derName);

/*
** Extract the Issuers Distinguished Name from a DER encoded certificate
**	"derCert" is the DER encoded certificate
**	"derName" is the SECItem that the name is returned in
*/
extern SECStatus CERT_IssuerNameFromDERCert(SECItem *derCert, SECItem *derName);

extern SECItem *CERT_EncodeGeneralName(CERTGeneralName *genName, SECItem *dest,
                                       PLArenaPool *arena);

extern CERTGeneralName *CERT_DecodeGeneralName(PLArenaPool *reqArena,
                                               SECItem *encodedName,
                                               CERTGeneralName *genName);

/*
** Generate a database search key for a certificate, based on the
** issuer and serial number.
**	"arena" the memory arena to alloc from
**	"derCert" the DER encoded certificate
**	"key" the returned key
*/
extern SECStatus CERT_KeyFromDERCert(PLArenaPool *reqArena, SECItem *derCert,
                                     SECItem *key);

extern SECStatus CERT_KeyFromIssuerAndSN(PLArenaPool *arena, SECItem *issuer,
                                         SECItem *sn, SECItem *key);

extern SECStatus CERT_SerialNumberFromDERCert(SECItem *derCert,
                                              SECItem *derName);

/*
** Generate a database search key for a crl, based on the
** issuer.
**	"arena" the memory arena to alloc from
**	"derCrl" the DER encoded crl
**	"key" the returned key
*/
extern SECStatus CERT_KeyFromDERCrl(PLArenaPool *arena, SECItem *derCrl,
                                    SECItem *key);

/*
** Open the certificate database.  Use callback to get name of database.
*/
extern SECStatus CERT_OpenCertDB(CERTCertDBHandle *handle, PRBool readOnly,
                                 CERTDBNameFunc namecb, void *cbarg);

/* Open the certificate database.  Use given filename for database. */
extern SECStatus CERT_OpenCertDBFilename(CERTCertDBHandle *handle,
                                         char *certdbname, PRBool readOnly);

/*
** Open and initialize a cert database that is entirely in memory.  This
** can be used when the permanent database can not be opened or created.
*/
extern SECStatus CERT_OpenVolatileCertDB(CERTCertDBHandle *handle);

/*
** Extract the list of host names, host name patters, IP address strings
** this cert is valid for.
** This function does NOT return nicknames.
** Type CERTCertNicknames is being used because it's a convenient
** data structure to carry a list of strings and its count.
*/
extern CERTCertNicknames *CERT_GetValidDNSPatternsFromCert(
    CERTCertificate *cert);

/*
** Check the hostname to make sure that it matches the shexp that
** is given in the common name of the certificate.
*/
extern SECStatus CERT_VerifyCertName(const CERTCertificate *cert,
                                     const char *hostname);

/*
** Add a domain name to the list of names that the user has explicitly
** allowed (despite cert name mismatches) for use with a server cert.
*/
extern SECStatus CERT_AddOKDomainName(CERTCertificate *cert,
                                      const char *hostname);

/*
** Decode a DER encoded certificate into an CERTCertificate structure
**	"derSignedCert" is the DER encoded signed certificate
**	"copyDER" is true if the DER should be copied, false if the
**		existing copy should be referenced
**	"nickname" is the nickname to use in the database.  If it is NULL
**		then a temporary nickname is generated.
*/
extern CERTCertificate *CERT_DecodeDERCertificate(SECItem *derSignedCert,
                                                  PRBool copyDER,
                                                  char *nickname);
/*
** Decode a DER encoded CRL into a CERTSignedCrl structure
**	"derSignedCrl" is the DER encoded signed CRL.
**	"type" must be SEC_CRL_TYPE.
*/
#define SEC_CRL_TYPE 1
#define SEC_KRL_TYPE 0 /* deprecated */

extern CERTSignedCrl *CERT_DecodeDERCrl(PLArenaPool *arena,
                                        SECItem *derSignedCrl, int type);

/*
 * same as CERT_DecodeDERCrl, plus allow options to be passed in
 */

extern CERTSignedCrl *CERT_DecodeDERCrlWithFlags(PLArenaPool *narena,
                                                 SECItem *derSignedCrl,
                                                 int type, PRInt32 options);

/* CRL options to pass */

#define CRL_DECODE_DEFAULT_OPTIONS 0x00000000

/* when CRL_DECODE_DONT_COPY_DER is set, the DER is not copied . The
   application must then keep derSignedCrl until it destroys the
   CRL . Ideally, it should allocate derSignedCrl in an arena
   and pass that arena in as the first argument to
   CERT_DecodeDERCrlWithFlags */

#define CRL_DECODE_DONT_COPY_DER 0x00000001
#define CRL_DECODE_SKIP_ENTRIES 0x00000002
#define CRL_DECODE_KEEP_BAD_CRL 0x00000004
#define CRL_DECODE_ADOPT_HEAP_DER 0x00000008

/* complete the decoding of a partially decoded CRL, ie. decode the
   entries. Note that entries is an optional field in a CRL, so the
   "entries" pointer in CERTCrlStr may still be NULL even after
   function returns SECSuccess */

extern SECStatus CERT_CompleteCRLDecodeEntries(CERTSignedCrl *crl);

/* Validate CRL then import it to the dbase.  If there is already a CRL with the
 * same CA in the dbase, it will be replaced if derCRL is more up to date.
 * If the process successes, a CRL will be returned.  Otherwise, a NULL will
 * be returned. The caller should call PORT_GetError() for the exactly error
 * code.
 */
extern CERTSignedCrl *CERT_ImportCRL(CERTCertDBHandle *handle, SECItem *derCRL,
                                     char *url, int type, void *wincx);

extern void CERT_DestroyCrl(CERTSignedCrl *crl);

/* this is a hint to flush the CRL cache. crlKey is the DER subject of
   the issuer (CA). */
void CERT_CRLCacheRefreshIssuer(CERTCertDBHandle *dbhandle, SECItem *crlKey);

/* add the specified DER CRL object to the CRL cache. Doing so will allow
   certificate verification functions (such as CERT_VerifyCertificate)
   to automatically find and make use of this CRL object.
   Once a CRL is added to the CRL cache, the application must hold on to
   the object's memory, because the cache will reference it directly. The
   application can only free the object after it calls CERT_UncacheCRL to
   remove it from the CRL cache.
*/
SECStatus CERT_CacheCRL(CERTCertDBHandle *dbhandle, SECItem *newcrl);

/* remove a previously added CRL object from the CRL cache. It is OK
   for the application to free the memory after a successful removal
*/
SECStatus CERT_UncacheCRL(CERTCertDBHandle *dbhandle, SECItem *oldcrl);

/*
** Find a certificate in the database
**	"key" is the database key to look for
*/
extern CERTCertificate *CERT_FindCertByKey(CERTCertDBHandle *handle,
                                           SECItem *key);

/*
** Find a certificate in the database by name
**	"name" is the distinguished name to look up
*/
extern CERTCertificate *CERT_FindCertByName(CERTCertDBHandle *handle,
                                            SECItem *name);

/*
** Find a certificate in the database by name
**	"name" is the distinguished name to look up (in ascii)
*/
extern CERTCertificate *CERT_FindCertByNameString(CERTCertDBHandle *handle,
                                                  char *name);

/*
** Find a certificate in the database by name and keyid
**	"name" is the distinguished name to look up
**	"keyID" is the value of the subjectKeyID to match
*/
extern CERTCertificate *CERT_FindCertByKeyID(CERTCertDBHandle *handle,
                                             SECItem *name, SECItem *keyID);

/*
** Generate a certificate key from the issuer and serialnumber, then look it
** up in the database.  Return the cert if found.
**	"issuerAndSN" is the issuer and serial number to look for
*/
extern CERTCertificate *CERT_FindCertByIssuerAndSN(
    CERTCertDBHandle *handle, CERTIssuerAndSN *issuerAndSN);
extern CERTCertificate *CERT_FindCertByIssuerAndSNCX(
    CERTCertDBHandle *handle, CERTIssuerAndSN *issuerAndSN, void *wincx);

/*
** Find a certificate in the database by a subject key ID
**	"subjKeyID" is the subject Key ID to look for
*/
extern CERTCertificate *CERT_FindCertBySubjectKeyID(CERTCertDBHandle *handle,
                                                    SECItem *subjKeyID);

/*
** Encode Certificate SKID (Subject Key ID) extension.
**
*/
extern SECStatus CERT_EncodeSubjectKeyID(PLArenaPool *arena,
                                         const SECItem *srcString,
                                         SECItem *encodedValue);

/*
** Find a certificate in the database by a nickname
**	"nickname" is the ascii string nickname to look for
*/
extern CERTCertificate *CERT_FindCertByNickname(CERTCertDBHandle *handle,
                                                const char *nickname);

/*
** Find a certificate in the database by a DER encoded certificate
**	"derCert" is the DER encoded certificate
*/
extern CERTCertificate *CERT_FindCertByDERCert(CERTCertDBHandle *handle,
                                               SECItem *derCert);

/*
** Find a certificate in the database by a email address
**	"emailAddr" is the email address to look up
*/
CERTCertificate *CERT_FindCertByEmailAddr(CERTCertDBHandle *handle,
                                          char *emailAddr);

/*
** Find a certificate in the database by a email address or nickname
**	"name" is the email address or nickname to look up
*/
CERTCertificate *CERT_FindCertByNicknameOrEmailAddr(CERTCertDBHandle *handle,
                                                    const char *name);
CERTCertificate *CERT_FindCertByNicknameOrEmailAddrCX(CERTCertDBHandle *handle,
                                                      const char *name,
                                                      void *wincx);

/*
** Find a certificate in the database by a email address or nickname
** and require it to have the given usage.
**      "name" is the email address or nickname to look up
*/
CERTCertificate *CERT_FindCertByNicknameOrEmailAddrForUsage(
    CERTCertDBHandle *handle, const char *name, SECCertUsage lookingForUsage);
CERTCertificate *CERT_FindCertByNicknameOrEmailAddrForUsageCX(
    CERTCertDBHandle *handle, const char *name, SECCertUsage lookingForUsage,
    void *wincx);

/*
** Find a certificate in the database by a digest of a subject public key
**	"spkDigest" is the digest to look up
*/
extern CERTCertificate *CERT_FindCertBySPKDigest(CERTCertDBHandle *handle,
                                                 SECItem *spkDigest);

/*
 * Find the issuer of a cert
 */
CERTCertificate *CERT_FindCertIssuer(CERTCertificate *cert, PRTime validTime,
                                     SECCertUsage usage);

/*
** Check the validity times of a certificate vs. time 't', allowing
** some slop for broken clocks and stuff.
**	"cert" is the certificate to be checked
**	"t" is the time to check against
**	"allowOverride" if true then check to see if the invalidity has
**		been overridden by the user.
*/
extern SECCertTimeValidity CERT_CheckCertValidTimes(const CERTCertificate *cert,
                                                    PRTime t,
                                                    PRBool allowOverride);

/*
** WARNING - this function is deprecated, and will either go away or have
**		a new API in the near future.
**
** Check the validity times of a certificate vs. the current time, allowing
** some slop for broken clocks and stuff.
**	"cert" is the certificate to be checked
*/
extern SECStatus CERT_CertTimesValid(CERTCertificate *cert);

/*
** Extract the validity times from a certificate
**	"c" is the certificate
**	"notBefore" is the start of the validity period
**	"notAfter" is the end of the validity period
*/
extern SECStatus CERT_GetCertTimes(const CERTCertificate *c, PRTime *notBefore,
                                   PRTime *notAfter);

/*
** Extract the issuer and serial number from a certificate
*/
extern CERTIssuerAndSN *CERT_GetCertIssuerAndSN(PLArenaPool *,
                                                CERTCertificate *);

/*
** verify the signature of a signed data object with a given certificate
**	"sd" the signed data object to be verified
**	"cert" the certificate to use to check the signature
*/
extern SECStatus CERT_VerifySignedData(CERTSignedData *sd,
                                       CERTCertificate *cert, PRTime t,
                                       void *wincx);
/*
** verify the signature of a signed data object with the given DER publickey
*/
extern SECStatus CERT_VerifySignedDataWithPublicKeyInfo(
    CERTSignedData *sd, CERTSubjectPublicKeyInfo *pubKeyInfo, void *wincx);

/*
** verify the signature of a signed data object with a SECKEYPublicKey.
*/
extern SECStatus CERT_VerifySignedDataWithPublicKey(const CERTSignedData *sd,
                                                    SECKEYPublicKey *pubKey,
                                                    void *wincx);

/*
** NEW FUNCTIONS with new bit-field-FIELD SECCertificateUsage - please use
** verify a certificate by checking validity times against a certain time,
** that we trust the issuer, and that the signature on the certificate is
** valid.
**	"cert" the certificate to verify
**	"checkSig" only check signatures if true
*/
extern SECStatus CERT_VerifyCertificate(CERTCertDBHandle *handle,
                                        CERTCertificate *cert, PRBool checkSig,
                                        SECCertificateUsage requiredUsages,
                                        PRTime t, void *wincx,
                                        CERTVerifyLog *log,
                                        SECCertificateUsage *returnedUsages);

/* same as above, but uses current time */
extern SECStatus CERT_VerifyCertificateNow(CERTCertDBHandle *handle,
                                           CERTCertificate *cert,
                                           PRBool checkSig,
                                           SECCertificateUsage requiredUsages,
                                           void *wincx,
                                           SECCertificateUsage *returnedUsages);

/*
** Verify that a CA cert can certify some (unspecified) leaf cert for a given
** purpose. This is used by UI code to help identify where a chain may be
** broken and why. This takes identical parameters to CERT_VerifyCert
*/
extern SECStatus CERT_VerifyCACertForUsage(CERTCertDBHandle *handle,
                                           CERTCertificate *cert,
                                           PRBool checkSig,
                                           SECCertUsage certUsage, PRTime t,
                                           void *wincx, CERTVerifyLog *log);

/*
** OLD OBSOLETE FUNCTIONS with enum SECCertUsage - DO NOT USE FOR NEW CODE
** verify a certificate by checking validity times against a certain time,
** that we trust the issuer, and that the signature on the certificate is
** valid.
**	"cert" the certificate to verify
**	"checkSig" only check signatures if true
*/
extern SECStatus CERT_VerifyCert(CERTCertDBHandle *handle,
                                 CERTCertificate *cert, PRBool checkSig,
                                 SECCertUsage certUsage, PRTime t, void *wincx,
                                 CERTVerifyLog *log);

/* same as above, but uses current time */
extern SECStatus CERT_VerifyCertNow(CERTCertDBHandle *handle,
                                    CERTCertificate *cert, PRBool checkSig,
                                    SECCertUsage certUsage, void *wincx);

SECStatus CERT_VerifyCertChain(CERTCertDBHandle *handle, CERTCertificate *cert,
                               PRBool checkSig, SECCertUsage certUsage,
                               PRTime t, void *wincx, CERTVerifyLog *log);

/*
** Read a base64 ascii encoded DER certificate and convert it to our
** internal format.
**	"certstr" is a null-terminated string containing the certificate
*/
extern CERTCertificate *CERT_ConvertAndDecodeCertificate(char *certstr);

/*
** Read a certificate in some foreign format, and convert it to our
** internal format.
**	"certbuf" is the buffer containing the certificate
**	"certlen" is the length of the buffer
** NOTE - currently supports netscape base64 ascii encoded raw certs
**  and netscape binary DER typed files.
*/
extern CERTCertificate *CERT_DecodeCertFromPackage(char *certbuf, int certlen);

extern SECStatus CERT_ImportCAChain(SECItem *certs, int numcerts,
                                    SECCertUsage certUsage);

extern SECStatus CERT_ImportCAChainTrusted(SECItem *certs, int numcerts,
                                           SECCertUsage certUsage);

/*
** Read a certificate chain in some foreign format, and pass it to a
** callback function.
**	"certbuf" is the buffer containing the certificate
**	"certlen" is the length of the buffer
**	"f" is the callback function
**	"arg" is the callback argument
*/
typedef SECStatus(PR_CALLBACK *CERTImportCertificateFunc)(void *arg,
                                                          SECItem **certs,
                                                          int numcerts);

extern SECStatus CERT_DecodeCertPackage(char *certbuf, int certlen,
                                        CERTImportCertificateFunc f, void *arg);

/*
** Returns the value of an AVA.  This was a formerly static
** function that has been exposed due to the need to decode
** and convert unicode strings to UTF8.
**
** XXX This function resides in certhtml.c, should it be
** moved elsewhere?
*/
extern SECItem *CERT_DecodeAVAValue(const SECItem *derAVAValue);

/*
** extract various element strings from a distinguished name.
**	"name" the distinguished name
*/

extern char *CERT_GetCertificateEmailAddress(CERTCertificate *cert);

extern char *CERT_GetCertEmailAddress(const CERTName *name);

extern const char *CERT_GetFirstEmailAddress(CERTCertificate *cert);

extern const char *CERT_GetNextEmailAddress(CERTCertificate *cert,
                                            const char *prev);

/* The return value must be freed with PORT_Free. */
extern char *CERT_GetCommonName(const CERTName *name);

extern char *CERT_GetCountryName(const CERTName *name);

extern char *CERT_GetLocalityName(const CERTName *name);

extern char *CERT_GetStateName(const CERTName *name);

extern char *CERT_GetOrgName(const CERTName *name);

extern char *CERT_GetOrgUnitName(const CERTName *name);

extern char *CERT_GetDomainComponentName(const CERTName *name);

extern char *CERT_GetCertUid(const CERTName *name);

/* manipulate the trust parameters of a certificate */

extern SECStatus CERT_GetCertTrust(const CERTCertificate *cert,
                                   CERTCertTrust *trust);

extern SECStatus CERT_ChangeCertTrust(CERTCertDBHandle *handle,
                                      CERTCertificate *cert,
                                      CERTCertTrust *trust);

extern SECStatus CERT_ChangeCertTrustByUsage(CERTCertDBHandle *certdb,
                                             CERTCertificate *cert,
                                             SECCertUsage usage);

/*************************************************************************
 *
 * manipulate the extensions of a certificate
 *
 ************************************************************************/

/*
** Set up a cert for adding X509v3 extensions.  Returns an opaque handle
** used by the next two routines.
**	"cert" is the certificate we are adding extensions to
*/
extern void *CERT_StartCertExtensions(CERTCertificate *cert);

/*
** Add an extension to a certificate.
**	"exthandle" is the handle returned by the previous function
**	"idtag" is the integer tag for the OID that should ID this extension
**	"value" is the value of the extension
**	"critical" is the critical extension flag
**	"copyData" is a flag indicating whether the value data should be
**		copied.
*/
extern SECStatus CERT_AddExtension(void *exthandle, int idtag, SECItem *value,
                                   PRBool critical, PRBool copyData);

extern SECStatus CERT_AddExtensionByOID(void *exthandle, SECItem *oid,
                                        SECItem *value, PRBool critical,
                                        PRBool copyData);

extern SECStatus CERT_EncodeAndAddExtension(void *exthandle, int idtag,
                                            void *value, PRBool critical,
                                            const SEC_ASN1Template *atemplate);

extern SECStatus CERT_EncodeAndAddBitStrExtension(void *exthandle, int idtag,
                                                  SECItem *value,
                                                  PRBool critical);

extern SECStatus CERT_EncodeAltNameExtension(PLArenaPool *arena,
                                             CERTGeneralName *value,
                                             SECItem *encodedValue);

/*
** Finish adding cert extensions.  Does final processing on extension
** data, putting it in the right format, and freeing any temporary
** storage.
**	"exthandle" is the handle used to add extensions to a certificate
*/
extern SECStatus CERT_FinishExtensions(void *exthandle);

/*
** Merge an external list of extensions into a cert's extension list, adding one
** only when its OID matches none of the cert's existing extensions. Call this
** immediately before calling CERT_FinishExtensions().
*/
SECStatus CERT_MergeExtensions(void *exthandle, CERTCertExtension **exts);

/* If the extension is found, return its criticality and value.
** This allocate storage for the returning extension value.
*/
extern SECStatus CERT_GetExtenCriticality(CERTCertExtension **extensions,
                                          int tag, PRBool *isCritical);

extern void CERT_DestroyOidSequence(CERTOidSequence *oidSeq);

/****************************************************************************
 *
 * DER encode and decode extension values
 *
 ****************************************************************************/

/* Encode the value of the basicConstraint extension.
**	arena - where to allocate memory for the encoded value.
**	value - extension value to encode
**	encodedValue - output encoded value
*/
extern SECStatus CERT_EncodeBasicConstraintValue(PLArenaPool *arena,
                                                 CERTBasicConstraints *value,
                                                 SECItem *encodedValue);

/*
** Encode the value of the authorityKeyIdentifier extension.
*/
extern SECStatus CERT_EncodeAuthKeyID(PLArenaPool *arena, CERTAuthKeyID *value,
                                      SECItem *encodedValue);

/*
** Encode the value of the crlDistributionPoints extension.
*/
extern SECStatus CERT_EncodeCRLDistributionPoints(
    PLArenaPool *arena, CERTCrlDistributionPoints *value, SECItem *derValue);

/*
** Decodes a DER encoded basicConstaint extension value into a readable format
**	value - decoded value
**	encodedValue - value to decoded
*/
extern SECStatus CERT_DecodeBasicConstraintValue(CERTBasicConstraints *value,
                                                 const SECItem *encodedValue);

/* Decodes a DER encoded authorityKeyIdentifier extension value into a
** readable format.
**	arena - where to allocate memory for the decoded value
**	encodedValue - value to be decoded
**	Returns a CERTAuthKeyID structure which contains the decoded value
*/
extern CERTAuthKeyID *CERT_DecodeAuthKeyID(PLArenaPool *arena,
                                           const SECItem *encodedValue);

/* Decodes a DER encoded crlDistributionPoints extension value into a
** readable format.
**	arena - where to allocate memory for the decoded value
**	der - value to be decoded
**	Returns a CERTCrlDistributionPoints structure which contains the
**          decoded value
*/
extern CERTCrlDistributionPoints *CERT_DecodeCRLDistributionPoints(
    PLArenaPool *arena, SECItem *der);

/* Extract certain name type from a generalName */
extern void *CERT_GetGeneralNameByType(CERTGeneralName *genNames,
                                       CERTGeneralNameType type,
                                       PRBool derFormat);

extern CERTOidSequence *CERT_DecodeOidSequence(const SECItem *seqItem);

/****************************************************************************
 *
 * Find extension values of a certificate
 *
 ***************************************************************************/

extern SECStatus CERT_FindCertExtension(const CERTCertificate *cert, int tag,
                                        SECItem *value);

extern SECStatus CERT_FindNSCertTypeExtension(CERTCertificate *cert,
                                              SECItem *value);

extern char *CERT_FindNSStringExtension(CERTCertificate *cert, int oidtag);

extern SECStatus CERT_FindCertExtensionByOID(CERTCertificate *cert,
                                             SECItem *oid, SECItem *value);

/* Returns the decoded value of the authKeyID extension.
**   Note that this uses passed in the arena to allocate storage for the result
*/
extern CERTAuthKeyID *CERT_FindAuthKeyIDExten(PLArenaPool *arena,
                                              CERTCertificate *cert);

/* Returns the decoded value of the basicConstraint extension.
 */
extern SECStatus CERT_FindBasicConstraintExten(CERTCertificate *cert,
                                               CERTBasicConstraints *value);

/* Returns the decoded value of the crlDistributionPoints extension.
**  Note that the arena in cert is used to allocate storage for the result
*/
extern CERTCrlDistributionPoints *CERT_FindCRLDistributionPoints(
    CERTCertificate *cert);

/* Returns value of the keyUsage extension.  This uses PR_Alloc to allocate
** buffer for the decoded value. The caller should free up the storage
** allocated in value->data.
*/
extern SECStatus CERT_FindKeyUsageExtension(CERTCertificate *cert,
                                            SECItem *value);

/* Return the decoded value of the subjectKeyID extension. The caller should
** free up the storage allocated in retItem->data.
*/
extern SECStatus CERT_FindSubjectKeyIDExtension(CERTCertificate *cert,
                                                SECItem *retItem);

/*
** If cert is a v3 certificate, and a critical keyUsage extension is included,
** then check the usage against the extension value.  If a non-critical
** keyUsage extension is included, this will return SECSuccess without
** checking, since the extension is an advisory field, not a restriction.
** If cert is not a v3 certificate, this will return SECSuccess.
**	cert - certificate
**	usage - one of the x.509 v3 the Key Usage Extension flags
*/
extern SECStatus CERT_CheckCertUsage(CERTCertificate *cert,
                                     unsigned char usage);

/****************************************************************************
 *
 *  CRL v2 Extensions supported routines
 *
 ****************************************************************************/

extern SECStatus CERT_FindCRLExtensionByOID(CERTCrl *crl, SECItem *oid,
                                            SECItem *value);

extern SECStatus CERT_FindCRLExtension(CERTCrl *crl, int tag, SECItem *value);

extern SECStatus CERT_FindInvalidDateExten(CERTCrl *crl, PRTime *value);

/*
** Set up a crl for adding X509v3 extensions.  Returns an opaque handle
** used by routines that take an exthandle (void*) argument .
**	"crl" is the CRL we are adding extensions to
*/
extern void *CERT_StartCRLExtensions(CERTCrl *crl);

/*
** Set up a crl entry for adding X509v3 extensions.  Returns an opaque handle
** used by routines that take an exthandle (void*) argument .
**	"crl" is the crl we are adding certs entries to
**      "entry" is the crl entry we are adding extensions to
*/
extern void *CERT_StartCRLEntryExtensions(CERTCrl *crl, CERTCrlEntry *entry);

extern CERTCertNicknames *CERT_GetCertNicknames(CERTCertDBHandle *handle,
                                                int what, void *wincx);

/*
** Finds the crlNumber extension and decodes its value into 'value'
*/
extern SECStatus CERT_FindCRLNumberExten(PLArenaPool *arena, CERTCrl *crl,
                                         SECItem *value);

extern SECStatus CERT_FindCRLEntryReasonExten(CERTCrlEntry *crlEntry,
                                              CERTCRLEntryReasonCode *value);

extern void CERT_FreeNicknames(CERTCertNicknames *nicknames);

extern PRBool CERT_CompareCerts(const CERTCertificate *c1,
                                const CERTCertificate *c2);

extern PRBool CERT_CompareCertsForRedirection(CERTCertificate *c1,
                                              CERTCertificate *c2);

/*
** Generate an array of the Distinguished Names that the given cert database
** "trusts"
*/
extern CERTDistNames *CERT_GetSSLCACerts(CERTCertDBHandle *handle);

extern void CERT_FreeDistNames(CERTDistNames *names);

/* Duplicate distinguished name array */
extern CERTDistNames *CERT_DupDistNames(CERTDistNames *orig);

/*
** Generate an array of Distinguished names from an array of nicknames
*/
extern CERTDistNames *CERT_DistNamesFromNicknames(CERTCertDBHandle *handle,
                                                  char **nicknames, int nnames);

/*
** Generate an array of Distinguished names from a list of certs.
*/
extern CERTDistNames *CERT_DistNamesFromCertList(CERTCertList *list);

/*
** Generate a certificate chain from a certificate.
*/
extern CERTCertificateList *CERT_CertChainFromCert(CERTCertificate *cert,
                                                   SECCertUsage usage,
                                                   PRBool includeRoot);

extern CERTCertificateList *CERT_CertListFromCert(CERTCertificate *cert);

extern CERTCertificateList *CERT_DupCertList(
    const CERTCertificateList *oldList);

extern void CERT_DestroyCertificateList(CERTCertificateList *list);

/*
** is cert a user cert? i.e. does it have CERTDB_USER trust,
** i.e. a private key?
*/
PRBool CERT_IsUserCert(CERTCertificate *cert);

/* is cert a newer than cert b? */
PRBool CERT_IsNewer(CERTCertificate *certa, CERTCertificate *certb);

/* currently a stub for address book */
PRBool CERT_IsCertRevoked(CERTCertificate *cert);

void CERT_DestroyCertArray(CERTCertificate **certs, unsigned int ncerts);

/* convert an email address to lower case */
char *CERT_FixupEmailAddr(const char *emailAddr);

/* decode string representation of trust flags into trust struct */
SECStatus CERT_DecodeTrustString(CERTCertTrust *trust, const char *trusts);

/* encode trust struct into string representation of trust flags */
char *CERT_EncodeTrustString(CERTCertTrust *trust);

/* find the next or prev cert in a subject list */
CERTCertificate *CERT_PrevSubjectCert(CERTCertificate *cert);
CERTCertificate *CERT_NextSubjectCert(CERTCertificate *cert);

/*
 * import a collection of certs into the temporary or permanent cert
 * database
 */
SECStatus CERT_ImportCerts(CERTCertDBHandle *certdb, SECCertUsage usage,
                           unsigned int ncerts, SECItem **derCerts,
                           CERTCertificate ***retCerts, PRBool keepCerts,
                           PRBool caOnly, char *nickname);

char *CERT_MakeCANickname(CERTCertificate *cert);

PRBool CERT_IsCACert(CERTCertificate *cert, unsigned int *rettype);

PRBool CERT_IsCADERCert(SECItem *derCert, unsigned int *rettype);

PRBool CERT_IsRootDERCert(SECItem *derCert);

SECStatus CERT_SaveSMimeProfile(CERTCertificate *cert, SECItem *emailProfile,
                                SECItem *profileTime);

/*
 * find the smime symmetric capabilities profile for a given cert
 */
SECItem *CERT_FindSMimeProfile(CERTCertificate *cert);

SECStatus CERT_AddNewCerts(CERTCertDBHandle *handle);

CERTCertificatePolicies *CERT_DecodeCertificatePoliciesExtension(
    const SECItem *extnValue);

void CERT_DestroyCertificatePoliciesExtension(
    CERTCertificatePolicies *policies);

CERTCertificatePolicyMappings *CERT_DecodePolicyMappingsExtension(
    SECItem *encodedCertPolicyMaps);

SECStatus CERT_DestroyPolicyMappingsExtension(
    CERTCertificatePolicyMappings *mappings);

SECStatus CERT_DecodePolicyConstraintsExtension(
    CERTCertificatePolicyConstraints *decodedValue,
    const SECItem *encodedValue);

SECStatus CERT_DecodeInhibitAnyExtension(
    CERTCertificateInhibitAny *decodedValue, SECItem *extnValue);

CERTUserNotice *CERT_DecodeUserNotice(SECItem *noticeItem);

extern CERTGeneralName *CERT_DecodeAltNameExtension(PLArenaPool *reqArena,
                                                    SECItem *EncodedAltName);

extern CERTNameConstraints *CERT_DecodeNameConstraintsExtension(
    PLArenaPool *arena, const SECItem *encodedConstraints);

/* returns addr of a NULL termainated array of pointers to CERTAuthInfoAccess */
extern CERTAuthInfoAccess **CERT_DecodeAuthInfoAccessExtension(
    PLArenaPool *reqArena, const SECItem *encodedExtension);

extern CERTPrivKeyUsagePeriod *CERT_DecodePrivKeyUsagePeriodExtension(
    PLArenaPool *arena, SECItem *extnValue);

extern CERTGeneralName *CERT_GetNextGeneralName(CERTGeneralName *current);

extern CERTGeneralName *CERT_GetPrevGeneralName(CERTGeneralName *current);

/*
 * Look up name constraints for some certs that do not include name constraints
 * (Most importantly, root certificates)
 *
 * If a matching subject is found, |extensions| will be populated with a copy of
 * the
 * DER-encoded name constraints extension. The data in |extensions| will point
 * to
 * memory that the caller owns.
 *
 * There is no mechanism to configure imposed name constraints right now.  All
 * imposed name constraints are built into NSS.
 */
SECStatus CERT_GetImposedNameConstraints(const SECItem *derSubject,
                                         SECItem *extensions);

CERTNameConstraint *CERT_GetNextNameConstraint(CERTNameConstraint *current);

CERTNameConstraint *CERT_GetPrevNameConstraint(CERTNameConstraint *current);

void CERT_DestroyUserNotice(CERTUserNotice *userNotice);

typedef char *(*CERTPolicyStringCallback)(char *org, unsigned long noticeNumber,
                                          void *arg);
void CERT_SetCAPolicyStringCallback(CERTPolicyStringCallback cb, void *cbarg);

char *CERT_GetCertCommentString(CERTCertificate *cert);

PRBool CERT_GovtApprovedBitSet(CERTCertificate *cert);

SECStatus CERT_AddPermNickname(CERTCertificate *cert, char *nickname);

CERTCertList *CERT_MatchUserCert(CERTCertDBHandle *handle, SECCertUsage usage,
                                 int nCANames, char **caNames, void *proto_win);

CERTCertList *CERT_NewCertList(void);

/* free the cert list and all the certs in the list */
void CERT_DestroyCertList(CERTCertList *certs);

/* remove the node and free the cert */
void CERT_RemoveCertListNode(CERTCertListNode *node);

/* equivalent to CERT_AddCertToListTailWithData(certs, cert, NULL) */
SECStatus CERT_AddCertToListTail(CERTCertList *certs, CERTCertificate *cert);

/* equivalent to CERT_AddCertToListHeadWithData(certs, cert, NULL) */
SECStatus CERT_AddCertToListHead(CERTCertList *certs, CERTCertificate *cert);

/*
 * The new cert list node takes ownership of "cert". "cert" is freed
 * when the list node is removed.
 */
SECStatus CERT_AddCertToListTailWithData(CERTCertList *certs,
                                         CERTCertificate *cert, void *appData);

/*
 * The new cert list node takes ownership of "cert". "cert" is freed
 * when the list node is removed.
 */
SECStatus CERT_AddCertToListHeadWithData(CERTCertList *certs,
                                         CERTCertificate *cert, void *appData);

typedef PRBool (*CERTSortCallback)(CERTCertificate *certa,
                                   CERTCertificate *certb, void *arg);
SECStatus CERT_AddCertToListSorted(CERTCertList *certs, CERTCertificate *cert,
                                   CERTSortCallback f, void *arg);

/* callback for CERT_AddCertToListSorted that sorts based on validity
 * period and a given time.
 */
PRBool CERT_SortCBValidity(CERTCertificate *certa, CERTCertificate *certb,
                           void *arg);

SECStatus CERT_CheckForEvilCert(CERTCertificate *cert);

CERTGeneralName *CERT_GetCertificateNames(CERTCertificate *cert,
                                          PLArenaPool *arena);

CERTGeneralName *CERT_GetConstrainedCertificateNames(
    const CERTCertificate *cert, PLArenaPool *arena,
    PRBool includeSubjectCommonName);

/*
 * Creates or adds to a list of all certs with a give subject name, sorted by
 * validity time, newest first.  Invalid certs are considered older than
 * valid certs. If validOnly is set, do not include invalid certs on list.
 */
CERTCertList *CERT_CreateSubjectCertList(CERTCertList *certList,
                                         CERTCertDBHandle *handle,
                                         const SECItem *name, PRTime sorttime,
                                         PRBool validOnly);

/*
 * remove certs from a list that don't have keyUsage and certType
 * that match the given usage.
 */
SECStatus CERT_FilterCertListByUsage(CERTCertList *certList, SECCertUsage usage,
                                     PRBool ca);

/*
 * check the key usage of a cert against a set of required values
 */
SECStatus CERT_CheckKeyUsage(CERTCertificate *cert, unsigned int requiredUsage);

/*
 * return required key usage and cert type based on cert usage
 */
SECStatus CERT_KeyUsageAndTypeForCertUsage(SECCertUsage usage, PRBool ca,
                                           unsigned int *retKeyUsage,
                                           unsigned int *retCertType);
/*
 * return required trust flags for various cert usages for CAs
 */
SECStatus CERT_TrustFlagsForCACertUsage(SECCertUsage usage,
                                        unsigned int *retFlags,
                                        SECTrustType *retTrustType);

/*
 * Find all user certificates that match the given criteria.
 *
 *	"handle" - database to search
 *	"usage" - certificate usage to match
 *	"oneCertPerName" - if set then only return the "best" cert per
 *			name
 *	"validOnly" - only return certs that are curently valid
 *	"proto_win" - window handle passed to pkcs11
 */
CERTCertList *CERT_FindUserCertsByUsage(CERTCertDBHandle *handle,
                                        SECCertUsage usage,
                                        PRBool oneCertPerName, PRBool validOnly,
                                        void *proto_win);

/*
 * Find a user certificate that matchs the given criteria.
 *
 *	"handle" - database to search
 *	"nickname" - nickname to match
 *	"usage" - certificate usage to match
 *	"validOnly" - only return certs that are curently valid
 *	"proto_win" - window handle passed to pkcs11
 */
CERTCertificate *CERT_FindUserCertByUsage(CERTCertDBHandle *handle,
                                          const char *nickname,
                                          SECCertUsage usage, PRBool validOnly,
                                          void *proto_win);

/*
 * Filter a list of certificates, removing those certs that do not have
 * one of the named CA certs somewhere in their cert chain.
 *
 *	"certList" - the list of certificates to filter
 *	"nCANames" - number of CA names
 *	"caNames" - array of CA names in string(rfc 1485) form
 *	"usage" - what use the certs are for, this is used when
 *		selecting CA certs
 */
SECStatus CERT_FilterCertListByCANames(CERTCertList *certList, int nCANames,
                                       char **caNames, SECCertUsage usage);

/*
 * Filter a list of certificates, removing those certs that aren't user certs
 */
SECStatus CERT_FilterCertListForUserCerts(CERTCertList *certList);

/*
 * Collect the nicknames from all certs in a CertList.  If the cert is not
 * valid, append a string to that nickname.
 *
 * "certList" - the list of certificates
 * "expiredString" - the string to append to the nickname of any expired cert
 * "notYetGoodString" - the string to append to the nickname of any cert
 *		that is not yet valid
 */
CERTCertNicknames *CERT_NicknameStringsFromCertList(CERTCertList *certList,
                                                    char *expiredString,
                                                    char *notYetGoodString);

/*
 * Extract the nickname from a nickmake string that may have either
 * expiredString or notYetGoodString appended.
 *
 * Args:
 *	"namestring" - the string containing the nickname, and possibly
 *		one of the validity label strings
 *	"expiredString" - the expired validity label string
 *	"notYetGoodString" - the not yet good validity label string
 *
 * Returns the raw nickname
 */
char *CERT_ExtractNicknameString(char *namestring, char *expiredString,
                                 char *notYetGoodString);

/*
 * Given a certificate, return a string containing the nickname, and possibly
 * one of the validity strings, based on the current validity state of the
 * certificate.
 *
 * "arena" - arena to allocate returned string from.  If NULL, then heap
 *	is used.
 * "cert" - the cert to get nickname from
 * "expiredString" - the string to append to the nickname if the cert is
 *		expired.
 * "notYetGoodString" - the string to append to the nickname if the cert is
 *		not yet good.
 */
char *CERT_GetCertNicknameWithValidity(PLArenaPool *arena,
                                       CERTCertificate *cert,
                                       char *expiredString,
                                       char *notYetGoodString);

/*
 * Return the string representation of a DER encoded distinguished name
 * "dername" - The DER encoded name to convert
 */
char *CERT_DerNameToAscii(SECItem *dername);

/*
 * Supported usage values and types:
 *	certUsageSSLClient
 *	certUsageSSLServer
 *	certUsageSSLServerWithStepUp
 *	certUsageEmailSigner
 *	certUsageEmailRecipient
 *	certUsageObjectSigner
 */

CERTCertificate *CERT_FindMatchingCert(CERTCertDBHandle *handle,
                                       SECItem *derName, CERTCertOwner owner,
                                       SECCertUsage usage, PRBool preferTrusted,
                                       PRTime validTime, PRBool validOnly);

/*
 * Acquire the global lock on the cert database.
 * This lock is currently used for the following operations:
 *	adding or deleting a cert to either the temp or perm databases
 *	converting a temp to perm or perm to temp
 *	changing(maybe just adding?) the trust of a cert
 *	adjusting the reference count of a cert
 */
void CERT_LockDB(CERTCertDBHandle *handle);

/*
 * Free the global cert database lock.
 */
void CERT_UnlockDB(CERTCertDBHandle *handle);

/*
 * Get the certificate status checking configuratino data for
 * the certificate database
 */
CERTStatusConfig *CERT_GetStatusConfig(CERTCertDBHandle *handle);

/*
 * Set the certificate status checking information for the
 * database.  The input structure becomes part of the certificate
 * database and will be freed by calling the 'Destroy' function in
 * the configuration object.
 */
void CERT_SetStatusConfig(CERTCertDBHandle *handle, CERTStatusConfig *config);

/*
 * Acquire the cert reference count lock
 * There is currently one global lock for all certs, but I'm putting a cert
 * arg here so that it will be easy to make it per-cert in the future if
 * that turns out to be necessary.
 */
void CERT_LockCertRefCount(CERTCertificate *cert);

/*
 * Release the cert reference count lock
 */
void CERT_UnlockCertRefCount(CERTCertificate *cert);

/*
 * Digest the cert's subject public key using the specified algorithm.
 * NOTE: this digests the value of the BIT STRING subjectPublicKey (excluding
 * the tag, length, and number of unused bits) rather than the whole
 * subjectPublicKeyInfo field.
 *
 * The necessary storage for the digest data is allocated.  If "fill" is
 * non-null, the data is put there, otherwise a SECItem is allocated.
 * Allocation from "arena" if it is non-null, heap otherwise.  Any problem
 * results in a NULL being returned (and an appropriate error set).
 */
extern SECItem *CERT_GetSubjectPublicKeyDigest(PLArenaPool *arena,
                                               const CERTCertificate *cert,
                                               SECOidTag digestAlg,
                                               SECItem *fill);

/*
 * Digest the cert's subject name using the specified algorithm.
 */
extern SECItem *CERT_GetSubjectNameDigest(PLArenaPool *arena,
                                          const CERTCertificate *cert,
                                          SECOidTag digestAlg, SECItem *fill);

SECStatus CERT_CheckCRL(CERTCertificate *cert, CERTCertificate *issuer,
                        const SECItem *dp, PRTime t, void *wincx);

/*
 * Add a CERTNameConstraint to the CERTNameConstraint list
 */
extern CERTNameConstraint *CERT_AddNameConstraint(
    CERTNameConstraint *list, CERTNameConstraint *constraint);

/*
 * Allocate space and copy CERTNameConstraint from src to dest.
 * Arena is used to allocate result(if dest eq NULL) and its members
 * SECItem data.
 */
extern CERTNameConstraint *CERT_CopyNameConstraint(PLArenaPool *arena,
                                                   CERTNameConstraint *dest,
                                                   CERTNameConstraint *src);

/*
 * Verify name against all the constraints relevant to that type of
 * the name.
 */
extern SECStatus CERT_CheckNameSpace(PLArenaPool *arena,
                                     const CERTNameConstraints *constraints,
                                     const CERTGeneralName *currentName);

/*
 * Extract and allocate the name constraints extension from the CA cert.
 * If the certificate contains no name constraints extension, but
 * CERT_GetImposedNameConstraints returns a name constraints extension
 * for the subject of the certificate, then that extension will be returned.
 */
extern SECStatus CERT_FindNameConstraintsExten(
    PLArenaPool *arena, CERTCertificate *cert,
    CERTNameConstraints **constraints);

/*
 * Initialize a new GERTGeneralName fields (link)
 */
extern CERTGeneralName *CERT_NewGeneralName(PLArenaPool *arena,
                                            CERTGeneralNameType type);

/*
 * Lookup a CERTGeneralNameType constant by its human readable string.
 */
extern CERTGeneralNameType CERT_GetGeneralNameTypeFromString(
    const char *string);

/*
 * PKIX extension encoding routines
 */
extern SECStatus CERT_EncodePolicyConstraintsExtension(
    PLArenaPool *arena, CERTCertificatePolicyConstraints *constr,
    SECItem *dest);
extern SECStatus CERT_EncodeInhibitAnyExtension(
    PLArenaPool *arena, CERTCertificateInhibitAny *inhibitAny, SECItem *dest);
extern SECStatus CERT_EncodePolicyMappingExtension(
    PLArenaPool *arena, CERTCertificatePolicyMappings *maps, SECItem *dest);

extern SECStatus CERT_EncodeInfoAccessExtension(PLArenaPool *arena,
                                                CERTAuthInfoAccess **info,
                                                SECItem *dest);
extern SECStatus CERT_EncodeUserNotice(PLArenaPool *arena,
                                       CERTUserNotice *notice, SECItem *dest);

extern SECStatus CERT_EncodeDisplayText(PLArenaPool *arena, SECItem *text,
                                        SECItem *dest);

extern SECStatus CERT_EncodeCertPoliciesExtension(PLArenaPool *arena,
                                                  CERTPolicyInfo **info,
                                                  SECItem *dest);
extern SECStatus CERT_EncodeNoticeReference(PLArenaPool *arena,
                                            CERTNoticeReference *reference,
                                            SECItem *dest);

/*
 * Returns a pointer to a static structure.
 */
extern const CERTRevocationFlags *CERT_GetPKIXVerifyNistRevocationPolicy(void);

/*
 * Returns a pointer to a static structure.
 */
extern const CERTRevocationFlags *CERT_GetClassicOCSPEnabledSoftFailurePolicy(
    void);

/*
 * Returns a pointer to a static structure.
 */
extern const CERTRevocationFlags *CERT_GetClassicOCSPEnabledHardFailurePolicy(
    void);

/*
 * Returns a pointer to a static structure.
 */
extern const CERTRevocationFlags *CERT_GetClassicOCSPDisabledPolicy(void);

/*
 * Verify a Cert with libpkix
 *  paramsIn control the verification options. If a value isn't specified
 *   in paramsIn, it reverts to the application default.
 *  paramsOut specifies the parameters the caller would like to get back.
 *   the caller may pass NULL, in which case no parameters are returned.
 */
extern SECStatus CERT_PKIXVerifyCert(CERTCertificate *cert,
                                     SECCertificateUsage usages,
                                     CERTValInParam *paramsIn,
                                     CERTValOutParam *paramsOut, void *wincx);

/* Makes old cert validation APIs(CERT_VerifyCert, CERT_VerifyCertificate)
 * to use libpkix validation engine. The function should be called ones at
 * application initialization time.
 * Function is not thread safe.*/
extern SECStatus CERT_SetUsePKIXForValidation(PRBool enable);

/* The function return PR_TRUE if cert validation should use
 * libpkix cert validation engine. */
extern PRBool CERT_GetUsePKIXForValidation(void);

/*
 * Allocate a parameter container of type CERTRevocationFlags,
 * and allocate the inner arrays of the given sizes.
 * To cleanup call CERT_DestroyCERTRevocationFlags.
 */
extern CERTRevocationFlags *CERT_AllocCERTRevocationFlags(
    PRUint32 number_leaf_methods, PRUint32 number_leaf_pref_methods,
    PRUint32 number_chain_methods, PRUint32 number_chain_pref_methods);

/*
 * Destroy the arrays inside flags,
 * and destroy the object pointed to by flags, too.
 */
extern void CERT_DestroyCERTRevocationFlags(CERTRevocationFlags *flags);

/*
 * Get istemp and isperm fields from a cert in a thread safe way.
 */
extern SECStatus CERT_GetCertIsTemp(const CERTCertificate *cert, PRBool *istemp);
extern SECStatus CERT_GetCertIsPerm(const CERTCertificate *cert, PRBool *isperm);

SEC_END_PROTOS

#endif /* _CERT_H_ */
