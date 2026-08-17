// Copyright 2015-2021 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "openssl/ocsp.h"
#include "openssl/x509.h"

#if defined(__cplusplus)
extern "C" {
#endif

// CRLReason does not have a status assigned to the value 7.
//
// See Reason Code RFC: https://www.rfc-editor.org/rfc/rfc5280#section-5.3.1.
#define OCSP_UNASSIGNED_REVOKED_STATUS 7

// OCSPResponseStatus does not have a status assigned to the value 4.
//
// See Reason Code RFC:
// https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1
#define OCSP_UNASSIGNED_RESPONSE_STATUS 4

// OCSP Request ASN.1 specification:
// https://datatracker.ietf.org/doc/html/rfc6960#section-4.1.1
//
// OCSP Response ASN.1 specification:
// https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1

//   CertID ::= SEQUENCE {
//       hashAlgorithm    AlgorithmIdentifier,
//       issuerNameHash   OCTET STRING,  --Hash of Issuer's DN
//       issuerKeyHash    OCTET STRING,  --Hash of Issuers public key (excluding
//                                         the tag & length fields)
//       serialNumber     CertificateSerialNumber }
//
struct ocsp_cert_id_st {
  X509_ALGOR *hashAlgorithm;
  ASN1_OCTET_STRING *issuerNameHash;
  ASN1_OCTET_STRING *issuerKeyHash;
  ASN1_INTEGER *serialNumber;
};

//   Request ::=     SEQUENCE {
//       reqCert                    CertID,
//       singleRequestExtensions    [0] EXPLICIT Extensions OPTIONAL }
//
struct ocsp_one_request_st {
  OCSP_CERTID *reqCert;
  STACK_OF(X509_EXTENSION) *singleRequestExtensions;
};

//   TBSRequest      ::=     SEQUENCE {
//       version             [0] EXPLICIT Version DEFAULT v1,
//       requestorName       [1] EXPLICIT GeneralName OPTIONAL,
//       requestList             SEQUENCE OF Request,
//       requestExtensions   [2] EXPLICIT Extensions OPTIONAL }
//
struct ocsp_req_info_st {
  ASN1_INTEGER *version;
  GENERAL_NAME *requestorName;
  STACK_OF(OCSP_ONEREQ) *requestList;
  STACK_OF(X509_EXTENSION) *requestExtensions;
};

//   Signature ::= SEQUENCE {
//       signatureAlgorithm   AlgorithmIdentifier,
//       signature            BIT STRING,
//       certs                [0] EXPLICIT SEQUENCE OF Certificate OPTIONAL }
//
struct ocsp_signature_st {
  X509_ALGOR *signatureAlgorithm;
  ASN1_BIT_STRING *signature;
  STACK_OF(X509) *certs;
};

//   OCSPRequest     ::=     SEQUENCE {
//       tbsRequest                  TBSRequest,
//       optionalSignature   [0]     EXPLICIT Signature OPTIONAL }
//
struct ocsp_request_st {
  OCSP_REQINFO *tbsRequest;
  OCSP_SIGNATURE *optionalSignature;
};

// Opaque OCSP request status structure
struct ocsp_req_ctx_st {
  int state;                   // Current I/O state
  unsigned char *iobuf;        // Line buffer. Should only be modified during
                               // http exchange in OCSP_REQ_CTX_nbio.
  int iobuflen;                // Line buffer length
  BIO *io;                     // BIO to perform I/O with
  BIO *mem;                    // Memory BIO response is built into
  unsigned long asn1_len;      // ASN1 length of response
  unsigned long max_resp_len;  // Maximum length of response
};

//   OCSPResponseStatus ::= ENUMERATED {
//       successful         (0),  --Response has valid confirmations
//       malformedRequest   (1),  --Illegal confirmation request
//       internalError      (2),  --Internal error in issuer
//       tryLater           (3),  --Try again later
//                                --(4) is not used
//       sigRequired        (5),  --Must sign the request
//       unauthorized       (6)   --Request unauthorized
//   }
//

//   ResponseBytes ::= SEQUENCE {
//       responseType   OBJECT IDENTIFIER,
//       response       OCTET STRING }
//
struct ocsp_resp_bytes_st {
  ASN1_OBJECT *responseType;
  ASN1_OCTET_STRING *response;
};

//   OCSPResponse ::= SEQUENCE {
//      responseStatus   OCSPResponseStatus,
//      responseBytes    [0] EXPLICIT ResponseBytes OPTIONAL }
//
struct ocsp_response_st {
  ASN1_ENUMERATED *responseStatus;
  OCSP_RESPBYTES *responseBytes;
};

//  ResponderID ::= CHOICE {
//      byName   [1]   Name,
//      byKey    [2]   KeyHash }
//
//   KeyHash ::= OCTET STRING --SHA-1 hash of responder's public key
//                            --(excluding the tag and length fields)
//
// The RFC requires that the KeyHash value be of a SHA-1 hash. Even though this
// is not being used cryptographically, there is the possibility of a response
// being returned with a forced Responder KeyHash when using SHA-1 (assuming a
// preimage attack, which is beyond the scope of how SHA-1 is currently
// vulnerable). However, our hand are tied with what the RFC mandates.
//
// RFC 6960: https://datatracker.ietf.org/doc/html/rfc6960#appendix-B.2
struct ocsp_responder_id_st {
  int type;
  union {
    X509_NAME *byName;
    ASN1_OCTET_STRING *byKey;
  } value;
};

//   RevokedInfo ::= SEQUENCE {
//     revocationTime     GeneralizedTime,
//     revocationReason   [0] EXPLICIT CRLReason OPTIONAL }
//
struct ocsp_revoked_info_st {
  ASN1_GENERALIZEDTIME *revocationTime;
  ASN1_ENUMERATED *revocationReason;
};

//   CertStatus ::= CHOICE {
//       good      [0] IMPLICIT NULL,
//       revoked   [1] IMPLICIT RevokedInfo,
//       unknown   [2] IMPLICIT UnknownInfo }
//
struct ocsp_cert_status_st {
  int type;
  union {
    ASN1_NULL *good;
    OCSP_REVOKEDINFO *revoked;
    ASN1_NULL *unknown;
  } value;
};

//   SingleResponse ::= SEQUENCE {
//      certID             CertID,
//      certStatus         CertStatus,
//      thisUpdate         GeneralizedTime,
//      nextUpdate         [0] EXPLICIT GeneralizedTime OPTIONAL,
//      singleExtensions   [1] EXPLICIT Extensions OPTIONAL }
//
struct ocsp_single_response_st {
  OCSP_CERTID *certId;
  OCSP_CERTSTATUS *certStatus;
  ASN1_GENERALIZEDTIME *thisUpdate;
  ASN1_GENERALIZEDTIME *nextUpdate;
  STACK_OF(X509_EXTENSION) *singleExtensions;
};

//   ResponseData ::= SEQUENCE {
//      version              [0] EXPLICIT Version DEFAULT v1,
//      responderID          ResponderID,
//      producedAt           GeneralizedTime,
//      responses            SEQUENCE OF SingleResponse,
//      responseExtensions   [1] EXPLICIT Extensions OPTIONAL }
//
struct ocsp_response_data_st {
  ASN1_INTEGER *version;
  OCSP_RESPID *responderId;
  ASN1_GENERALIZEDTIME *producedAt;
  STACK_OF(OCSP_SINGLERESP) *responses;
  STACK_OF(X509_EXTENSION) *responseExtensions;
};

//   BasicOCSPResponse ::= SEQUENCE {
//      tbsResponseData      ResponseData,
//      signatureAlgorithm   AlgorithmIdentifier,
//      signature            BIT STRING,
//      certs                [0] EXPLICIT SEQUENCE OF Certificate OPTIONAL }
//
//
// Note 1: The value for "signature" is specified in the OCSP rfc2560 as
// follows: "The value for the signature SHALL be computed on the hash of
// the DER encoding ResponseData." This means that you must hash the
// DER-encoded tbsResponseData, and then run it through a crypto-signing
// function, which will (at least w/RSA) do a hash-'n'-private-encrypt
// operation.  This seems a bit odd, but that's the spec.  Also note that
// the data structures do not leave anywhere to independently specify the
// algorithm used for the initial hash. So, we look at the
// signature-specification algorithm, and try to do something intelligent.
// -- Kathy Weinhold, CertCo
//
// Note 2: It seems that the mentioned passage from RFC 2560 (section
// 4.2.1) is open for interpretation.  I've done tests against another
// responder, and found that it doesn't do the double hashing that the RFC
// seems to say one should.  Therefore, all relevant functions take a flag
// saying which variant should be used.  -- Richard Levitte, OpenSSL team
// and CeloCom
struct ocsp_basic_response_st {
  OCSP_RESPDATA *tbsResponseData;
  X509_ALGOR *signatureAlgorithm;
  ASN1_BIT_STRING *signature;
  STACK_OF(X509) *certs;
};

DECLARE_ASN1_FUNCTIONS(OCSP_RESPDATA)
DECLARE_ASN1_FUNCTIONS(OCSP_REQINFO)
DECLARE_ASN1_FUNCTIONS(OCSP_SIGNATURE)
DECLARE_ASN1_FUNCTIONS(OCSP_RESPBYTES)
DECLARE_ASN1_FUNCTIONS(OCSP_REVOKEDINFO)

// OCSP_get_default_digest sets the default digest according to |signer|.
// This exists because OpenSSL sets the default to |EVP_sha256| when passing
// NULL for |type| in |EVP_DigestSignInit| when using certain key types. We wish
// to avoid this general behavior for all |EVP_DigestSign*| operations, so we
// only set the default digest from the OCSP layer. |dgst| represents the user's
// self-defined digest type, if it's non-NULL, |dgst| is directly returned.
const EVP_MD *OCSP_get_default_digest(const EVP_MD *dgst, EVP_PKEY *signer);

// Try exchanging request and response via HTTP on (non-)blocking BIO in rctx.
OPENSSL_EXPORT int OCSP_REQ_CTX_nbio(OCSP_REQ_CTX *rctx);

// Tries to exchange the request and response with |OCSP_REQ_CTX_nbio|, but on
// success, it additionally parses the response, which must be a
// DER-encoded ASN.1 structure.
int OCSP_REQ_CTX_nbio_d2i(OCSP_REQ_CTX *rctx, ASN1_VALUE **pval,
                          const ASN1_ITEM *it);

// Returns the internal memory BIO of the |OCSP_REQ_CTX|. For AWS-LC, this is
// only used for testing if contents of |OCSP_REQ_CTX| have been written
// correctly.
OPENSSL_EXPORT BIO *OCSP_REQ_CTX_get0_mem_bio(OCSP_REQ_CTX *rctx);


// OCSP extension functions

// OCSP_REQUEST_get_ext_by_NID returns the index of an extension from an
// |OCSP_REQUEST| by its NID. Returns -1 if not found.
OPENSSL_EXPORT int OCSP_REQUEST_get_ext_by_NID(OCSP_REQUEST *req, int nid,
                                               int lastpos);

// OCSP_REQUEST_get_ext retrieves an |X509_EXTENSION| from an |OCSP_REQUEST|
// by its position in the extension list.
OPENSSL_EXPORT X509_EXTENSION *OCSP_REQUEST_get_ext(OCSP_REQUEST *req, int loc);

// OCSP_BASICRESP_add_ext adds a copy of |ex| to the extension list in
// |*bs|. It returns 1 on success and 0 on error. The new extension is
// inserted at index |loc|, shifting extensions to the right. If |loc| is -1 or
// out of bounds, the new extension is appended to the list.
int OCSP_BASICRESP_add_ext(OCSP_BASICRESP *bs, X509_EXTENSION *ex, int loc);


#define IS_OCSP_FLAG_SET(flags, query) (flags & query)
#define OCSP_MAX_RESP_LENGTH (100 * 1024)

#if defined(__cplusplus)
}  // extern C
#endif
