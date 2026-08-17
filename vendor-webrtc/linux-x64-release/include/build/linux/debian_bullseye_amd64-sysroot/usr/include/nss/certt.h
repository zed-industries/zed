/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/*
 * certt.h - public data structures for the certificate library
 */
#ifndef _CERTT_H_
#define _CERTT_H_

#include "prclist.h"
#include "pkcs11t.h"
#include "seccomon.h"
#include "secmodt.h"
#include "secoidt.h"
#include "plarena.h"
#include "prcvar.h"
#include "nssilock.h"
#include "prio.h"
#include "prmon.h"

/* Stan data types */
struct NSSCertificateStr;
struct NSSTrustDomainStr;

/* Non-opaque objects */
typedef struct CERTAVAStr CERTAVA;
typedef struct CERTAttributeStr CERTAttribute;
typedef struct CERTAuthInfoAccessStr CERTAuthInfoAccess;
typedef struct CERTAuthKeyIDStr CERTAuthKeyID;
typedef struct CERTBasicConstraintsStr CERTBasicConstraints;
typedef struct NSSTrustDomainStr CERTCertDBHandle;
typedef struct CERTCertExtensionStr CERTCertExtension;
typedef struct CERTCertKeyStr CERTCertKey;
typedef struct CERTCertListStr CERTCertList;
typedef struct CERTCertListNodeStr CERTCertListNode;
typedef struct CERTCertNicknamesStr CERTCertNicknames;
typedef struct CERTCertTrustStr CERTCertTrust;
typedef struct CERTCertDistrustStr CERTCertDistrust;
typedef struct CERTCertificateStr CERTCertificate;
typedef struct CERTCertificateListStr CERTCertificateList;
typedef struct CERTCertificateRequestStr CERTCertificateRequest;
typedef struct CERTCrlStr CERTCrl;
typedef struct CERTCrlDistributionPointsStr CERTCrlDistributionPoints;
typedef struct CERTCrlEntryStr CERTCrlEntry;
typedef struct CERTCrlHeadNodeStr CERTCrlHeadNode;
typedef struct CERTCrlKeyStr CERTCrlKey;
typedef struct CERTCrlNodeStr CERTCrlNode;
typedef struct CERTDERCertsStr CERTDERCerts;
typedef struct CERTDistNamesStr CERTDistNames;
typedef struct CERTGeneralNameStr CERTGeneralName;
typedef struct CERTGeneralNameListStr CERTGeneralNameList;
typedef struct CERTIssuerAndSNStr CERTIssuerAndSN;
typedef struct CERTNameStr CERTName;
typedef struct CERTNameConstraintStr CERTNameConstraint;
typedef struct CERTNameConstraintsStr CERTNameConstraints;
typedef struct CERTOKDomainNameStr CERTOKDomainName;
typedef struct CERTPrivKeyUsagePeriodStr CERTPrivKeyUsagePeriod;
typedef struct CERTPublicKeyAndChallengeStr CERTPublicKeyAndChallenge;
typedef struct CERTRDNStr CERTRDN;
typedef struct CERTSignedCrlStr CERTSignedCrl;
typedef struct CERTSignedDataStr CERTSignedData;
typedef struct CERTStatusConfigStr CERTStatusConfig;
typedef struct CERTSubjectListStr CERTSubjectList;
typedef struct CERTSubjectNodeStr CERTSubjectNode;
typedef struct CERTSubjectPublicKeyInfoStr CERTSubjectPublicKeyInfo;
typedef struct CERTValidityStr CERTValidity;
typedef struct CERTVerifyLogStr CERTVerifyLog;
typedef struct CERTVerifyLogNodeStr CERTVerifyLogNode;
typedef struct CRLDistributionPointStr CRLDistributionPoint;

/* CRL extensions type */
typedef unsigned long CERTCrlNumber;

/*
** An X.500 AVA object
*/
struct CERTAVAStr {
    SECItem type;
    SECItem value;
};

/*
** An X.500 RDN object
*/
struct CERTRDNStr {
    CERTAVA **avas;
};

/*
** An X.500 name object
*/
struct CERTNameStr {
    PLArenaPool *arena;
    CERTRDN **rdns;
};

/*
** An X.509 validity object
*/
struct CERTValidityStr {
    PLArenaPool *arena;
    SECItem notBefore;
    SECItem notAfter;
};

/*
 * A serial number and issuer name, which is used as a database key
 */
struct CERTCertKeyStr {
    SECItem serialNumber;
    SECItem derIssuer;
};

/*
** A signed data object. Used to implement the "signed" macro used
** in the X.500 specs.
*/
struct CERTSignedDataStr {
    SECItem data;
    SECAlgorithmID signatureAlgorithm;
    SECItem signature;
};

/*
** An X.509 subject-public-key-info object
*/
struct CERTSubjectPublicKeyInfoStr {
    PLArenaPool *arena;
    SECAlgorithmID algorithm;
    SECItem subjectPublicKey;
};

struct CERTPublicKeyAndChallengeStr {
    SECItem spki;
    SECItem challenge;
};

struct CERTCertTrustStr {
    unsigned int sslFlags;
    unsigned int emailFlags;
    unsigned int objectSigningFlags;
};

/*
 * Distrust dates for specific certificate usages.
 * These dates are hardcoded in nssckbi/builtins. They are DER encoded to be
 * compatible with the format of certdata.txt, other date fields in certs and
 * existing functions to read these dates. Clients should check the distrust
 * date in certificates to avoid trusting a CA for service they have ceased to
 * support */
struct CERTCertDistrustStr {
    SECItem serverDistrustAfter;
    SECItem emailDistrustAfter;
};

/*
 * defined the types of trust that exist
 */
typedef enum SECTrustTypeEnum {
    trustSSL = 0,
    trustEmail = 1,
    trustObjectSigning = 2,
    trustTypeNone = 3
} SECTrustType;

#define SEC_GET_TRUST_FLAGS(trust, type)                                  \
    (((type) == trustSSL)                                                 \
         ? ((trust)->sslFlags)                                            \
         : (((type) == trustEmail) ? ((trust)->emailFlags)                \
                                   : (((type) == trustObjectSigning)      \
                                          ? ((trust)->objectSigningFlags) \
                                          : 0)))

/*
** An X.509.3 certificate extension
*/
struct CERTCertExtensionStr {
    SECItem id;
    SECItem critical;
    SECItem value;
};

struct CERTSubjectNodeStr {
    struct CERTSubjectNodeStr *next;
    struct CERTSubjectNodeStr *prev;
    SECItem certKey;
    SECItem keyID;
};

struct CERTSubjectListStr {
    PLArenaPool *arena;
    int ncerts;
    char *emailAddr;
    CERTSubjectNode *head;
    CERTSubjectNode *tail; /* do we need tail? */
    void *entry;
};

/*
** An X.509 certificate object (the unsigned form)
*/
struct CERTCertificateStr {
    /* the arena is used to allocate any data structures that have the same
     * lifetime as the cert.  This is all stuff that hangs off of the cert
     * structure, and is all freed at the same time.  It is used when the
     * cert is decoded, destroyed, and at some times when it changes
     * state
     */
    PLArenaPool *arena;

    /* The following fields are static after the cert has been decoded */
    char *subjectName;
    char *issuerName;
    CERTSignedData signatureWrap; /* XXX */
    SECItem derCert;              /* original DER for the cert */
    SECItem derIssuer;            /* DER for issuer name */
    SECItem derSubject;           /* DER for subject name */
    SECItem derPublicKey;         /* DER for the public key */
    SECItem certKey;              /* database key for this cert */
    SECItem version;
    SECItem serialNumber;
    SECAlgorithmID signature;
    CERTName issuer;
    CERTValidity validity;
    CERTName subject;
    CERTSubjectPublicKeyInfo subjectPublicKeyInfo;
    SECItem issuerID;
    SECItem subjectID;
    CERTCertExtension **extensions;
    char *emailAddr;
    CERTCertDBHandle *dbhandle;
    SECItem subjectKeyID;     /* x509v3 subject key identifier */
    PRBool keyIDGenerated;    /* was the keyid generated? */
    unsigned int keyUsage;    /* what uses are allowed for this cert */
    unsigned int rawKeyUsage; /* value of the key usage extension */
    PRBool keyUsagePresent;   /* was the key usage extension present */
    PRUint32 nsCertType;      /* value of the ns cert type extension */
                              /* must be 32-bit for PR_ATOMIC_SET */

    /* these values can be set by the application to bypass certain checks
     * or to keep the cert in memory for an entire session.
     * XXX - need an api to set these
     */
    PRBool keepSession;         /* keep this cert for entire session*/
    PRBool timeOK;              /* is the bad validity time ok? */
    CERTOKDomainName *domainOK; /* these domain names are ok */

    /*
     * these values can change when the cert changes state.  These state
     * changes include transitions from temp to perm or vice-versa, and
     * changes of trust flags
     */
    PRBool isperm;
    PRBool istemp;
    char *nickname;
    char *dbnickname;
    struct NSSCertificateStr *nssCertificate; /* This is Stan stuff. */
    CERTCertTrust *trust;

    /* the reference count is modified whenever someone looks up, dups
     * or destroys a certificate
     */
    int referenceCount;

    /* The subject list is a list of all certs with the same subject name.
     * It can be modified any time a cert is added or deleted from either
     * the in-memory(temporary) or on-disk(permanent) database.
     */
    CERTSubjectList *subjectList;

    /* these belong in the static section, but are here to maintain
     * the structure's integrity
     */
    CERTAuthKeyID *authKeyID; /* x509v3 authority key identifier */
    PRBool isRoot;            /* cert is the end of a chain */

    /* these fields are used by client GUI code to keep track of ssl sockets
     * that are blocked waiting on GUI feedback related to this cert.
     * XXX - these should be moved into some sort of application specific
     *       data structure.  They are only used by the browser right now.
     */
    union {
        void *apointer; /* was struct SECSocketNode* authsocketlist */
        struct {
            unsigned int hasUnsupportedCriticalExt : 1;
            /* add any new option bits needed here */
        } bits;
    } options;
    int series; /* was int authsocketcount; record the series of the pkcs11ID */

    /* This is PKCS #11 stuff. */
    PK11SlotInfo *slot;        /*if this cert came of a token, which is it*/
    CK_OBJECT_HANDLE pkcs11ID; /*and which object on that token is it */
    PRBool ownSlot;            /*true if the cert owns the slot reference */
    /* These fields are used in nssckbi/builtins CAs. */
    CERTCertDistrust *distrust;
};
#define SEC_CERTIFICATE_VERSION_1 0 /* default created */
#define SEC_CERTIFICATE_VERSION_2 1 /* v2 */
#define SEC_CERTIFICATE_VERSION_3 2 /* v3 extensions */

#define SEC_CRL_VERSION_1 0 /* default */
#define SEC_CRL_VERSION_2 1 /* v2 extensions */

/*
 * used to identify class of cert in mime stream code
 */
#define SEC_CERT_CLASS_CA 1
#define SEC_CERT_CLASS_SERVER 2
#define SEC_CERT_CLASS_USER 3
#define SEC_CERT_CLASS_EMAIL 4

struct CERTDERCertsStr {
    PLArenaPool *arena;
    int numcerts;
    SECItem *rawCerts;
};

/*
** A PKCS ? Attribute
** XXX this is duplicated through out the code, it *should* be moved
** to a central location.  Where would be appropriate?
*/
struct CERTAttributeStr {
    SECItem attrType;
    SECItem **attrValue;
};

/*
** A PKCS#10 certificate-request object (the unsigned form)
*/
struct CERTCertificateRequestStr {
    PLArenaPool *arena;
    SECItem version;
    CERTName subject;
    CERTSubjectPublicKeyInfo subjectPublicKeyInfo;
    CERTAttribute **attributes;
};
#define SEC_CERTIFICATE_REQUEST_VERSION 0 /* what we *create* */

/*
** A certificate list object.
*/
struct CERTCertificateListStr {
    SECItem *certs;
    int len; /* number of certs */
    PLArenaPool *arena;
};

struct CERTCertListNodeStr {
    PRCList links;
    CERTCertificate *cert;
    void *appData;
};

struct CERTCertListStr {
    PRCList list;
    PLArenaPool *arena;
};

#define CERT_LIST_HEAD(l) ((CERTCertListNode *)PR_LIST_HEAD(&l->list))
#define CERT_LIST_TAIL(l) ((CERTCertListNode *)PR_LIST_TAIL(&l->list))
#define CERT_LIST_NEXT(n) ((CERTCertListNode *)n->links.next)
#define CERT_LIST_END(n, l) (((void *)n) == ((void *)&l->list))
#define CERT_LIST_EMPTY(l) CERT_LIST_END(CERT_LIST_HEAD(l), l)

struct CERTCrlEntryStr {
    SECItem serialNumber;
    SECItem revocationDate;
    CERTCertExtension **extensions;
};

struct CERTCrlStr {
    PLArenaPool *arena;
    SECItem version;
    SECAlgorithmID signatureAlg;
    SECItem derName;
    CERTName name;
    SECItem lastUpdate;
    SECItem nextUpdate; /* optional for x.509 CRL  */
    CERTCrlEntry **entries;
    CERTCertExtension **extensions;
    /* can't add anything there for binary backwards compatibility reasons */
};

struct CERTCrlKeyStr {
    SECItem derName;
    SECItem dummy; /* The decoder can not skip a primitive,
                      this serves as a place holder for the
                      decoder to finish its task only
                   */
};

struct CERTSignedCrlStr {
    PLArenaPool *arena;
    CERTCrl crl;
    void *reserved1;
    PRBool reserved2;
    PRBool isperm;
    PRBool istemp;
    int referenceCount;
    CERTCertDBHandle *dbhandle;
    CERTSignedData signatureWrap; /* XXX */
    char *url;
    SECItem *derCrl;
    PK11SlotInfo *slot;
    CK_OBJECT_HANDLE pkcs11ID;
    void *opaque; /* do not touch */
};

struct CERTCrlHeadNodeStr {
    PLArenaPool *arena;
    CERTCertDBHandle *dbhandle;
    CERTCrlNode *first;
    CERTCrlNode *last;
};

struct CERTCrlNodeStr {
    CERTCrlNode *next;
    int type;
    CERTSignedCrl *crl;
};

/*
 * Array of X.500 Distinguished Names
 */
struct CERTDistNamesStr {
    PLArenaPool *arena;
    int nnames;
    SECItem *names;
    void *head; /* private */
};

/*
 * NS_CERT_TYPE defines are used in two areas:
 * 1) The old NSS Cert Type Extension, which is a certificate extension in the
 * actual cert. It was created before the x509 Extended Key Usage Extension,
 * which has now taken over it's function. This field is only 8 bits wide
 * 2) The nsCertType entry in the CERTCertificate structure. This field is
 * 32 bits wide.
 * Any entries in this table greater than 0x80 will not be able to be encoded
 * in an NSS Cert Type Extension, but can still be represented internally in
 * the nsCertType field.
 */
#define NS_CERT_TYPE_IPSEC_CA (0x200)         /* outside the NS Cert Type Extenstion */
#define NS_CERT_TYPE_IPSEC (0x100)            /* outside the NS Cert Type Extenstion */
#define NS_CERT_TYPE_SSL_CLIENT (0x80)        /* bit 0 */
#define NS_CERT_TYPE_SSL_SERVER (0x40)        /* bit 1 */
#define NS_CERT_TYPE_EMAIL (0x20)             /* bit 2 */
#define NS_CERT_TYPE_OBJECT_SIGNING (0x10)    /* bit 3 */
#define NS_CERT_TYPE_RESERVED (0x08)          /* bit 4 */
#define NS_CERT_TYPE_SSL_CA (0x04)            /* bit 5 */
#define NS_CERT_TYPE_EMAIL_CA (0x02)          /* bit 6 */
#define NS_CERT_TYPE_OBJECT_SIGNING_CA (0x01) /* bit 7 */

#define EXT_KEY_USAGE_TIME_STAMP (0x8000)
#define EXT_KEY_USAGE_STATUS_RESPONDER (0x4000)

#define NS_CERT_TYPE_APP                                                      \
    (NS_CERT_TYPE_SSL_CLIENT | NS_CERT_TYPE_SSL_SERVER | NS_CERT_TYPE_EMAIL | \
     NS_CERT_TYPE_IPSEC | NS_CERT_TYPE_OBJECT_SIGNING)

#define NS_CERT_TYPE_CA                                                \
    (NS_CERT_TYPE_SSL_CA | NS_CERT_TYPE_EMAIL_CA |                     \
     NS_CERT_TYPE_OBJECT_SIGNING_CA | EXT_KEY_USAGE_STATUS_RESPONDER | \
     NS_CERT_TYPE_IPSEC_CA)
typedef enum SECCertUsageEnum {
    certUsageSSLClient = 0,
    certUsageSSLServer = 1,
    certUsageSSLServerWithStepUp = 2,
    certUsageSSLCA = 3,
    certUsageEmailSigner = 4,
    certUsageEmailRecipient = 5,
    certUsageObjectSigner = 6,
    certUsageUserCertImport = 7,
    certUsageVerifyCA = 8,
    certUsageProtectedObjectSigner = 9,
    certUsageStatusResponder = 10,
    certUsageAnyCA = 11,
    certUsageIPsec = 12
} SECCertUsage;

typedef PRInt64 SECCertificateUsage;

#define certificateUsageCheckAllUsages (0x0000)
#define certificateUsageSSLClient (0x0001)
#define certificateUsageSSLServer (0x0002)
#define certificateUsageSSLServerWithStepUp (0x0004)
#define certificateUsageSSLCA (0x0008)
#define certificateUsageEmailSigner (0x0010)
#define certificateUsageEmailRecipient (0x0020)
#define certificateUsageObjectSigner (0x0040)
#define certificateUsageUserCertImport (0x0080)
#define certificateUsageVerifyCA (0x0100)
#define certificateUsageProtectedObjectSigner (0x0200)
#define certificateUsageStatusResponder (0x0400)
#define certificateUsageAnyCA (0x0800)
#define certificateUsageIPsec (0x1000)

#define certificateUsageHighest certificateUsageIPsec

/*
 * Does the cert belong to the user, a peer, or a CA.
 */
typedef enum CERTCertOwnerEnum {
    certOwnerUser = 0,
    certOwnerPeer = 1,
    certOwnerCA = 2
} CERTCertOwner;

/*
 * This enum represents the state of validity times of a certificate
 */
typedef enum SECCertTimeValidityEnum {
    secCertTimeValid = 0,
    secCertTimeExpired = 1,
    secCertTimeNotValidYet = 2,
    secCertTimeUndetermined = 3 /* validity could not be decoded from the
                                   cert, most likely because it was NULL */
} SECCertTimeValidity;

/*
 * This is used as return status in functions that compare the validity
 * periods of two certificates A and B, currently only
 * CERT_CompareValidityTimes.
 */

typedef enum CERTCompareValidityStatusEnum {
    certValidityUndetermined = 0, /* the function is unable to select one cert
                                     over another */
    certValidityChooseB = 1,      /* cert B should be preferred */
    certValidityEqual = 2,        /* both certs have the same validity period */
    certValidityChooseA = 3       /* cert A should be preferred */
} CERTCompareValidityStatus;

/*
 * Interface for getting certificate nickname strings out of the database
 */

/* these are values for the what argument below */
#define SEC_CERT_NICKNAMES_ALL 1
#define SEC_CERT_NICKNAMES_USER 2
#define SEC_CERT_NICKNAMES_SERVER 3
#define SEC_CERT_NICKNAMES_CA 4

struct CERTCertNicknamesStr {
    PLArenaPool *arena;
    void *head;
    int numnicknames;
    char **nicknames;
    int what;
    int totallen;
};

struct CERTIssuerAndSNStr {
    SECItem derIssuer;
    CERTName issuer;
    SECItem serialNumber;
};

/* X.509 v3 Key Usage Extension flags */
#define KU_DIGITAL_SIGNATURE (0x80) /* bit 0 */
#define KU_NON_REPUDIATION (0x40)   /* bit 1 */
#define KU_KEY_ENCIPHERMENT (0x20)  /* bit 2 */
#define KU_DATA_ENCIPHERMENT (0x10) /* bit 3 */
#define KU_KEY_AGREEMENT (0x08)     /* bit 4 */
#define KU_KEY_CERT_SIGN (0x04)     /* bit 5 */
#define KU_CRL_SIGN (0x02)          /* bit 6 */
#define KU_ENCIPHER_ONLY (0x01)     /* bit 7 */
#define KU_ALL                                                         \
    (KU_DIGITAL_SIGNATURE | KU_NON_REPUDIATION | KU_KEY_ENCIPHERMENT | \
     KU_DATA_ENCIPHERMENT | KU_KEY_AGREEMENT | KU_KEY_CERT_SIGN |      \
     KU_CRL_SIGN | KU_ENCIPHER_ONLY)

/* This value will not occur in certs.  It is used internally for the case
 * when either digital signature or non-repudiation is the correct value.
 */
#define KU_DIGITAL_SIGNATURE_OR_NON_REPUDIATION (0x2000)

/* This value will not occur in certs.  It is used internally for the case
 * when the key type is not know ahead of time and either key agreement or
 * key encipherment are the correct value based on key type
 */
#define KU_KEY_AGREEMENT_OR_ENCIPHERMENT (0x4000)

/* internal bits that do not match bits in the x509v3 spec, but are used
 * for similar purposes
 */
#define KU_NS_GOVT_APPROVED (0x8000) /*don't make part of KU_ALL!*/
/*
* x.509 v3 Basic Constraints Extension
* If isCA is false, the pathLenConstraint is ignored.
* Otherwise, the following pathLenConstraint values will apply:
*	< 0 - there is no limit to the certificate path
*	0   - CA can issues end-entity certificates only
*	> 0 - the number of certificates in the certificate path is
*	      limited to this number
*/
#define CERT_UNLIMITED_PATH_CONSTRAINT -2

struct CERTBasicConstraintsStr {
    PRBool isCA;           /* on if is CA */
    int pathLenConstraint; /* maximum number of certificates that can be
                              in the cert path.  Only applies to a CA
                              certificate; otherwise, it's ignored.
                            */
};

/* Maximum length of a certificate chain */
#define CERT_MAX_CERT_CHAIN 20

#define CERT_MAX_SERIAL_NUMBER_BYTES 20 /* from RFC 3280 */
#define CERT_MAX_DN_BYTES 4096          /* arbitrary */

/* x.509 v3 Reason Flags, used in CRLDistributionPoint Extension */
#define RF_UNUSED (0x80)                 /* bit 0 */
#define RF_KEY_COMPROMISE (0x40)         /* bit 1 */
#define RF_CA_COMPROMISE (0x20)          /* bit 2 */
#define RF_AFFILIATION_CHANGED (0x10)    /* bit 3 */
#define RF_SUPERSEDED (0x08)             /* bit 4 */
#define RF_CESSATION_OF_OPERATION (0x04) /* bit 5 */
#define RF_CERTIFICATE_HOLD (0x02)       /* bit 6 */

/* enum for CRL Entry Reason Code */
typedef enum CERTCRLEntryReasonCodeEnum {
    crlEntryReasonUnspecified = 0,
    crlEntryReasonKeyCompromise = 1,
    crlEntryReasonCaCompromise = 2,
    crlEntryReasonAffiliationChanged = 3,
    crlEntryReasonSuperseded = 4,
    crlEntryReasonCessationOfOperation = 5,
    crlEntryReasoncertificatedHold = 6,
    crlEntryReasonRemoveFromCRL = 8,
    crlEntryReasonPrivilegeWithdrawn = 9,
    crlEntryReasonAaCompromise = 10
} CERTCRLEntryReasonCode;

/* If we needed to extract the general name field, use this */
/* General Name types */
typedef enum CERTGeneralNameTypeEnum {
    certOtherName = 1,
    certRFC822Name = 2,
    certDNSName = 3,
    certX400Address = 4,
    certDirectoryName = 5,
    certEDIPartyName = 6,
    certURI = 7,
    certIPAddress = 8,
    certRegisterID = 9
} CERTGeneralNameType;

typedef struct OtherNameStr {
    SECItem name;
    SECItem oid;
} OtherName;

struct CERTGeneralNameStr {
    CERTGeneralNameType type; /* name type */
    union {
        CERTName directoryName; /* distinguish name */
        OtherName OthName;      /* Other Name */
        SECItem other;          /* the rest of the name forms */
    } name;
    SECItem derDirectoryName; /* this is saved to simplify directory name
                                 comparison */
    PRCList l;
};

struct CERTGeneralNameListStr {
    PLArenaPool *arena;
    CERTGeneralName *name;
    int refCount;
    int len;
    PZLock *lock;
};

struct CERTNameConstraintStr {
    CERTGeneralName name;
    SECItem DERName;
    SECItem min;
    SECItem max;
    PRCList l;
};

struct CERTNameConstraintsStr {
    CERTNameConstraint *permited;
    CERTNameConstraint *excluded;
    SECItem **DERPermited;
    SECItem **DERExcluded;
};

/* Private Key Usage Period extension struct. */
struct CERTPrivKeyUsagePeriodStr {
    SECItem notBefore;
    SECItem notAfter;
    PLArenaPool *arena;
};

/* X.509 v3 Authority Key Identifier extension.  For the authority certificate
   issuer field, we only support URI now.
 */
struct CERTAuthKeyIDStr {
    SECItem keyID;                   /* unique key identifier */
    CERTGeneralName *authCertIssuer; /* CA's issuer name.  End with a NULL */
    SECItem authCertSerialNumber;    /* CA's certificate serial number */
    SECItem **DERAuthCertIssuer;     /* This holds the DER encoded format of
                                        the authCertIssuer field. It is used
                                        by the encoding engine. It should be
                                        used as a read only field by the caller.
                                     */
};

/* x.509 v3 CRL Distributeion Point */

/*
 * defined the types of CRL Distribution points
 */
typedef enum DistributionPointTypesEnum {
    generalName = 1, /* only support this for now */
    relativeDistinguishedName = 2
} DistributionPointTypes;

struct CRLDistributionPointStr {
    DistributionPointTypes distPointType;
    union {
        CERTGeneralName *fullName;
        CERTRDN relativeName;
    } distPoint;
    SECItem reasons;
    CERTGeneralName *crlIssuer;

    /* Reserved for internal use only*/
    SECItem derDistPoint;
    SECItem derRelativeName;
    SECItem **derCrlIssuer;
    SECItem **derFullName;
    SECItem bitsmap;
};

struct CERTCrlDistributionPointsStr {
    CRLDistributionPoint **distPoints;
};

/*
 * This structure is used to keep a log of errors when verifying
 * a cert chain.  This allows multiple errors to be reported all at
 * once.
 */
struct CERTVerifyLogNodeStr {
    CERTCertificate *cert;             /* what cert had the error */
    long error;                        /* what error was it? */
    unsigned int depth;                /* how far up the chain are we */
    void *arg;                         /* error specific argument */
    struct CERTVerifyLogNodeStr *next; /* next in the list */
    struct CERTVerifyLogNodeStr *prev; /* next in the list */
};

struct CERTVerifyLogStr {
    PLArenaPool *arena;
    unsigned int count;
    struct CERTVerifyLogNodeStr *head;
    struct CERTVerifyLogNodeStr *tail;
};

struct CERTOKDomainNameStr {
    CERTOKDomainName *next;
    char *name;
};

typedef SECStatus(PR_CALLBACK *CERTStatusChecker)(CERTCertDBHandle *handle,
                                                  CERTCertificate *cert,
                                                  PRTime time, void *pwArg);

typedef SECStatus(PR_CALLBACK *CERTStatusDestroy)(CERTStatusConfig *handle);

struct CERTStatusConfigStr {
    CERTStatusChecker statusChecker; /* NULL means no checking enabled */
    CERTStatusDestroy statusDestroy; /* enabled or no, will clean up */
    void *statusContext;             /* cx specific to checking protocol */
};

struct CERTAuthInfoAccessStr {
    SECItem method;
    SECItem derLocation;
    CERTGeneralName *location; /* decoded location */
};

/* This is the typedef for the callback passed to CERT_OpenCertDB() */
/* callback to return database name based on version number */
typedef char *(*CERTDBNameFunc)(void *arg, int dbVersion);

/*
 * types of cert packages that we can decode
 */
typedef enum CERTPackageTypeEnum {
    certPackageNone = 0,
    certPackageCert = 1,
    certPackagePKCS7 = 2,
    certPackageNSCertSeq = 3,
    certPackageNSCertWrap = 4
} CERTPackageType;

/*
 * these types are for the PKIX Certificate Policies extension
 */
typedef struct {
    SECOidTag oid;
    SECItem qualifierID;
    SECItem qualifierValue;
} CERTPolicyQualifier;

typedef struct {
    SECOidTag oid;
    SECItem policyID;
    CERTPolicyQualifier **policyQualifiers;
} CERTPolicyInfo;

typedef struct {
    PLArenaPool *arena;
    CERTPolicyInfo **policyInfos;
} CERTCertificatePolicies;

typedef struct {
    SECItem organization;
    SECItem **noticeNumbers;
} CERTNoticeReference;

typedef struct {
    PLArenaPool *arena;
    CERTNoticeReference noticeReference;
    SECItem derNoticeReference;
    SECItem displayText;
} CERTUserNotice;

typedef struct {
    PLArenaPool *arena;
    SECItem **oids;
} CERTOidSequence;

/*
 * these types are for the PKIX Policy Mappings extension
 */
typedef struct {
    SECItem issuerDomainPolicy;
    SECItem subjectDomainPolicy;
} CERTPolicyMap;

typedef struct {
    PLArenaPool *arena;
    CERTPolicyMap **policyMaps;
} CERTCertificatePolicyMappings;

/*
 * these types are for the PKIX inhibitAnyPolicy extension
 */
typedef struct {
    SECItem inhibitAnySkipCerts;
} CERTCertificateInhibitAny;

/*
 * these types are for the PKIX Policy Constraints extension
 */
typedef struct {
    SECItem explicitPolicySkipCerts;
    SECItem inhibitMappingSkipCerts;
} CERTCertificatePolicyConstraints;

/*
 * These types are for the validate chain callback param.
 *
 * CERTChainVerifyCallback is an application-supplied callback that can be used
 * to augment libpkix's certificate chain validation with additional
 * application-specific checks. It may be called multiple times if there are
 * multiple potentially-valid paths for the certificate being validated. This
 * callback is called before revocation checking is done on the certificates in
 * the given chain.
 *
 * - isValidChainArg contains the application-provided opaque argument
 * - currentChain is the currently validated chain. It is ordered with the leaf
 *   certificate at the head and the trust anchor at the tail.
 *
 * The callback should set *chainOK = PR_TRUE and return SECSuccess if the
 * certificate chain is acceptable. It should set *chainOK = PR_FALSE and
 * return SECSuccess if the chain is unacceptable, to indicate that the given
 * chain is bad and path building should continue. It should return SECFailure
 * to indicate an fatal error that will cause path validation to fail
 * immediately.
 */
typedef SECStatus (*CERTChainVerifyCallbackFunc)(
    void *isChainValidArg, const CERTCertList *currentChain, PRBool *chainOK);

/*
 * Note: If extending this structure, it will be necessary to change the
 * associated CERTValParamInType
 */
typedef struct {
    CERTChainVerifyCallbackFunc isChainValid;
    void *isChainValidArg;
} CERTChainVerifyCallback;

/*
 * these types are for the CERT_PKIX* Verification functions
 * These are all optional parameters.
 */

typedef enum {
    cert_pi_end = 0,              /* SPECIAL: signifies end of array of
                              * CERTValParam* */
    cert_pi_nbioContext = 1,      /* specify a non-blocking IO context used to
                              * resume a session. If this argument is
                              * specified, no other arguments should be.
                              * Specified in value.pointer.p. If the
                              * operation completes the context will be
                              * freed. */
    cert_pi_nbioAbort = 2,        /* specify a non-blocking IO context for an
                              * existing operation which the caller wants
                              * to abort. If this argument is
                              * specified, no other arguments should be.
                              * Specified in value.pointer.p. If the
                              * operation succeeds the context will be
                              * freed. */
    cert_pi_certList = 3,         /* specify the chain to validate against. If
                              * this value is given, then the path
                              * construction step in the validation is
                              * skipped. Specified in value.pointer.chain */
    cert_pi_policyOID = 4,        /* validate certificate for policy OID.
                              * Specified in value.array.oids. Cert must
                              * be good for at least one OID in order
                              * to validate. Default is that the user is not
                              * concerned about certificate policy. */
    cert_pi_policyFlags = 5,      /* flags for each policy specified in policyOID.
                              * Specified in value.scalar.ul. Policy flags
                              * apply to all specified oids.
                              * Use CERT_POLICY_FLAG_* macros below. If not
                              * specified policy flags default to 0 */
    cert_pi_keyusage = 6,         /* specify what the keyusages the certificate
                              * will be evaluated against, specified in
                              * value.scalar.ui. The cert must validate for
                              * at least one of the specified key usages.
                              * Values match the KU_  bit flags defined
                              * in this file. Default is derived from
                              * the 'usages' function argument */
    cert_pi_extendedKeyusage = 7, /* specify what the required extended key
                                   * usage of the certificate. Specified as
                                   * an array of oidTags in value.array.oids.
                                   * The cert must validate for at least one
                                   * of the specified extended key usages.
                                   * If not specified, no extended key usages
                                   * will be checked. */
    cert_pi_date = 8,             /* validate certificate is valid as of date
                                   * specified in value.scalar.time. A special
                                   * value '0' indicates 'now'. default is '0' */
    cert_pi_revocationFlags = 9,  /* Specify what revocation checking to do.
                                   * See CERT_REV_FLAG_* macros below
                                   * Set in value.pointer.revocation */
    cert_pi_certStores = 10,      /* Bitmask of Cert Store flags (see below)
                                   * Set in value.scalar.ui */
    cert_pi_trustAnchors =
        11,                       /* Specify the list of trusted roots to
                                   * validate against.
                                   * The default set of trusted roots, these are
                                   * root CA certs from libnssckbi.so or CA
                                   * certs trusted by user, are used in any of
                                   * the following cases:
                                   *      * when the parameter is not set.
                                   *      * when the list of trust anchors is
                                   *        empty.
                                   * Note that this handling can be further
                                   * altered by altering the
                                   * cert_pi_useOnlyTrustAnchors flag
                                   * Specified in value.pointer.chain */
    cert_pi_useAIACertFetch = 12, /* Enables cert fetching using AIA extension.
                                  * In NSS 3.12.1 or later. Default is off.
                                  * Value is in value.scalar.b */
    cert_pi_chainVerifyCallback = 13,
    /* The callback container for doing extra
     * validation on the currently calculated chain.
     * Value is in value.pointer.chainVerifyCallback */
    cert_pi_useOnlyTrustAnchors = 14,
    /* If true, disables trusting any
        * certificates other than the ones passed in via cert_pi_trustAnchors.
        * If false, then the certificates specified via cert_pi_trustAnchors
        * will be combined with the pre-existing trusted roots, but only
        * for the certificate validation being performed.
        * If no value has been supplied via cert_pi_trustAnchors, this has
        * no effect.
        * The default value is true, meaning if this is not supplied, only
        * trust anchors supplied via cert_pi_trustAnchors are trusted.
        * Specified in value.scalar.b */
    cert_pi_max /* SPECIAL: signifies maximum allowed value,
                 *  can increase in future releases */
} CERTValParamInType;

/*
 * for all out parameters:
 *  out parameters are only returned if the caller asks for them in
 *  the CERTValOutParam array. Caller is responsible for the CERTValOutParam
 *  array itself. The pkix verify function will allocate and other arrays
 *  pointers, or objects. The Caller is responsible for freeing those results.
 * If SECWouldBlock is returned, only cert_pi_nbioContext is returned.
 */
typedef enum {
    cert_po_end = 0,              /* SPECIAL: signifies end of array of
                                   * CERTValParam* */
    cert_po_nbioContext = 1,      /* Return a nonblocking context. If no
                                   * non-blocking context is specified, then
                                   * blocking IO will be used.
                                   * Returned in value.pointer.p. The context is
                                   * freed after an abort or a complete operation.
                                   * This value is only returned on SECWouldBlock.
                                   */
    cert_po_trustAnchor = 2,      /* Return the trust anchor for the chain that
                                   * was validated. Returned in
                                   * value.pointer.cert, this value is only
                                   * returned on SECSuccess. */
    cert_po_certList = 3,         /* Return the entire chain that was validated.
                                   * Returned in value.pointer.certList. If no
                                   * chain could be constructed, this value
                                   * would be NULL. */
    cert_po_policyOID = 4,        /* Return the policies that were found to be
                                   * valid. Returned in value.array.oids as an
                                   * array. This is only returned on
                                   * SECSuccess. */
    cert_po_errorLog = 5,         /* Return a log of problems with the chain.
                                   * Returned in value.pointer.log  */
    cert_po_usages = 6,           /* Return what usages the certificate is valid
                                     for. Returned in value.scalar.usages */
    cert_po_keyUsage = 7,         /* Return what key usages the certificate
                                   * is valid for.
                                   * Returned in value.scalar.usage */
    cert_po_extendedKeyusage = 8, /* Return what extended key usages the
                                   * certificate is valid for.
                                   * Returned in value.array.oids */
    cert_po_max                   /* SPECIAL: signifies maximum allowed value,
                                   *  can increase in future releases */

} CERTValParamOutType;

typedef enum {
    cert_revocation_method_crl = 0,
    cert_revocation_method_ocsp,
    cert_revocation_method_count
} CERTRevocationMethodIndex;

/*
 * The following flags are supposed to be used to control bits in
 * each integer contained in the array pointed to be:
 *     CERTRevocationTests.cert_rev_flags_per_method
 * All Flags are prefixed by CERT_REV_M_, where _M_ indicates
 * this is a method dependent flag.
 */

/*
 * Whether or not to use a method for revocation testing.
 * If set to "do not test", then all other flags are ignored.
 */
#define CERT_REV_M_DO_NOT_TEST_USING_THIS_METHOD 0UL
#define CERT_REV_M_TEST_USING_THIS_METHOD 1UL

/*
 * Whether or not NSS is allowed to attempt to fetch fresh information
 *         from the network.
 * (Although fetching will never happen if fresh information for the
 *           method is already locally available.)
 */
#define CERT_REV_M_ALLOW_NETWORK_FETCHING 0UL
#define CERT_REV_M_FORBID_NETWORK_FETCHING 2UL

/*
 * Example for an implicit default source:
 *         The globally configured default OCSP responder.
 * IGNORE means:
 *        ignore the implicit default source, whether it's configured or not.
 * ALLOW means:
 *       if an implicit default source is configured,
 *          then it overrides any available or missing source in the cert.
 *       if no implicit default source is configured,
 *          then we continue to use what's available (or not available)
 *          in the certs.
 */
#define CERT_REV_M_ALLOW_IMPLICIT_DEFAULT_SOURCE 0UL
#define CERT_REV_M_IGNORE_IMPLICIT_DEFAULT_SOURCE 4UL

/*
 * Defines the behavior if no fresh information is available,
 *   fetching from the network is allowed, but the source of revocation
 *   information is unknown (even after considering implicit sources,
 *   if allowed by other flags).
 * SKIPT_TEST means:
 *          We ignore that no fresh information is available and
 *          skip this test.
 * REQUIRE_INFO means:
 *          We still require that fresh information is available.
 *          Other flags define what happens on missing fresh info.
 */
#define CERT_REV_M_SKIP_TEST_ON_MISSING_SOURCE 0UL
#define CERT_REV_M_REQUIRE_INFO_ON_MISSING_SOURCE 8UL

/*
 * Defines the behavior if we are unable to obtain fresh information.
 * INGORE means:
 *      Return "cert status unknown"
 * FAIL means:
 *      Return "cert revoked".
 */
#define CERT_REV_M_IGNORE_MISSING_FRESH_INFO 0UL
#define CERT_REV_M_FAIL_ON_MISSING_FRESH_INFO 16UL

/*
 * What should happen if we were able to find fresh information using
 * this method, and the data indicated the cert is good?
 * STOP_TESTING means:
 *              Our success is sufficient, do not continue testing
 *              other methods.
 * CONTINUE_TESTING means:
 *                  We will continue and test the next allowed
 *                  specified method.
 */
#define CERT_REV_M_STOP_TESTING_ON_FRESH_INFO 0UL
#define CERT_REV_M_CONTINUE_TESTING_ON_FRESH_INFO 32UL

/* When this flag is used, libpkix will never attempt to use the GET HTTP
 * method for OCSP requests; it will always use POST.
 */
#define CERT_REV_M_FORCE_POST_METHOD_FOR_OCSP 64UL

/*
 * The following flags are supposed to be used to control bits in
 *     CERTRevocationTests.cert_rev_method_independent_flags
 * All Flags are prefixed by CERT_REV_M_, where _M_ indicates
 * this is a method independent flag.
 */

/*
 * This defines the order to checking.
 * EACH_METHOD_SEPARATELY means:
 *      Do all tests related to a particular allowed method
 *      (both local information and network fetching) in a single step.
 *      Only after testing for a particular method is done,
 *      then switching to the next method will happen.
 * ALL_LOCAL_INFORMATION_FIRST means:
 *      Start by testing the information for all allowed methods
 *      which are already locally available. Only after that is done
 *      consider to fetch from the network (as allowed by other flags).
 */
#define CERT_REV_MI_TEST_EACH_METHOD_SEPARATELY 0UL
#define CERT_REV_MI_TEST_ALL_LOCAL_INFORMATION_FIRST 1UL

/*
 * Use this flag to specify that it's necessary that fresh information
 * is available for at least one of the allowed methods, but it's
 * irrelevant which of the mechanisms succeeded.
 * NO_OVERALL_INFO_REQUIREMENT means:
 *     We strictly follow the requirements for each individual method.
 * REQUIRE_SOME_FRESH_INFO_AVAILABLE means:
 *     After the individual tests have been executed, we must have
 *     been able to find fresh information using at least one method.
 *     If we were unable to find fresh info, it's a failure.
 *     This setting overrides the CERT_REV_M_FAIL_ON_MISSING_FRESH_INFO
 *     flag on all methods.
 */
#define CERT_REV_MI_NO_OVERALL_INFO_REQUIREMENT 0UL
#define CERT_REV_MI_REQUIRE_SOME_FRESH_INFO_AVAILABLE 2UL

typedef struct {
    /*
     * The size of the array that cert_rev_flags_per_method points to,
     * meaning, the number of methods that are known and defined
     * by the caller.
     */
    PRUint32 number_of_defined_methods;

    /*
     * A pointer to an array of integers.
     * Each integer defines revocation checking for a single method,
     *      by having individual CERT_REV_M_* bits set or not set.
     * The meaning of index numbers into this array are defined by
     *     enum CERTRevocationMethodIndex
     * The size of the array must be specified by the caller in the separate
     *     variable number_of_defined_methods.
     * The size of the array may be smaller than
     *     cert_revocation_method_count, it can happen if a caller
     *     is not yet aware of the latest revocation methods
     *     (or does not want to use them).
     */
    PRUint64 *cert_rev_flags_per_method;

    /*
     * How many preferred methods are specified?
     * This is equivalent to the size of the array that
     *      preferred_methods points to.
     * It's allowed to set this value to zero,
     *      then NSS will decide which methods to prefer.
     */
    PRUint32 number_of_preferred_methods;

    /* Array that may specify an optional order of preferred methods.
     * Each array entry shall contain a method identifier as defined
     *   by CERTRevocationMethodIndex.
     * The entry at index [0] specifies the method with highest preference.
     * These methods will be tested first for locally available information.
     * Methods allowed for downloading will be attempted in the same order.
     */
    CERTRevocationMethodIndex *preferred_methods;

    /*
     * An integer which defines certain aspects of revocation checking
     * (independent of individual methods) by having individual
     * CERT_REV_MI_* bits set or not set.
     */
    PRUint64 cert_rev_method_independent_flags;
} CERTRevocationTests;

typedef struct {
    CERTRevocationTests leafTests;
    CERTRevocationTests chainTests;
} CERTRevocationFlags;

typedef struct CERTValParamInValueStr {
    union {
        PRBool b;
        PRInt32 i;
        PRUint32 ui;
        PRInt64 l;
        PRUint64 ul;
        PRTime time;
    } scalar;
    union {
        const void *p;
        const char *s;
        const CERTCertificate *cert;
        const CERTCertList *chain;
        const CERTRevocationFlags *revocation;
        const CERTChainVerifyCallback *chainVerifyCallback;
    } pointer;
    union {
        const PRInt32 *pi;
        const PRUint32 *pui;
        const PRInt64 *pl;
        const PRUint64 *pul;
        const SECOidTag *oids;
    } array;
    int arraySize;
} CERTValParamInValue;

typedef struct CERTValParamOutValueStr {
    union {
        PRBool b;
        PRInt32 i;
        PRUint32 ui;
        PRInt64 l;
        PRUint64 ul;
        SECCertificateUsage usages;
    } scalar;
    union {
        void *p;
        char *s;
        CERTVerifyLog *log;
        CERTCertificate *cert;
        CERTCertList *chain;
    } pointer;
    union {
        void *p;
        SECOidTag *oids;
    } array;
    int arraySize;
} CERTValParamOutValue;

typedef struct {
    CERTValParamInType type;
    CERTValParamInValue value;
} CERTValInParam;

typedef struct {
    CERTValParamOutType type;
    CERTValParamOutValue value;
} CERTValOutParam;

/*
 * Levels of standards conformance strictness for CERT_NameToAsciiInvertible
 */
typedef enum CertStrictnessLevels {
    CERT_N2A_READABLE = 0,   /* maximum human readability */
    CERT_N2A_STRICT = 10,    /* strict RFC compliance    */
    CERT_N2A_INVERTIBLE = 20 /* maximum invertibility,
                                all DirectoryStrings encoded in hex */
} CertStrictnessLevel;

/*
 * policy flag defines
 */
#define CERT_POLICY_FLAG_NO_MAPPING 1
#define CERT_POLICY_FLAG_EXPLICIT 2
#define CERT_POLICY_FLAG_NO_ANY 4

/*
 * CertStore flags
 */
#define CERT_ENABLE_LDAP_FETCH 1
#define CERT_ENABLE_HTTP_FETCH 2

/* This functin pointer type may be used for any function that takes
 * a CERTCertificate * and returns an allocated string, which must be
 * freed by a call to PORT_Free.
 */
typedef char *(*CERT_StringFromCertFcn)(CERTCertificate *cert);

/* XXX Lisa thinks the template declarations belong in cert.h, not here? */

#include "secasn1t.h" /* way down here because I expect template stuff to
                       * move out of here anyway */

SEC_BEGIN_PROTOS

extern const SEC_ASN1Template CERT_CertificateRequestTemplate[];
extern const SEC_ASN1Template CERT_CertificateTemplate[];
extern const SEC_ASN1Template SEC_SignedCertificateTemplate[];
extern const SEC_ASN1Template CERT_CertExtensionTemplate[];
extern const SEC_ASN1Template CERT_SequenceOfCertExtensionTemplate[];
extern const SEC_ASN1Template SECKEY_PublicKeyTemplate[];
extern const SEC_ASN1Template CERT_SubjectPublicKeyInfoTemplate[];
extern const SEC_ASN1Template CERT_TimeChoiceTemplate[];
extern const SEC_ASN1Template CERT_ValidityTemplate[];
extern const SEC_ASN1Template CERT_PublicKeyAndChallengeTemplate[];
extern const SEC_ASN1Template SEC_CertSequenceTemplate[];

extern const SEC_ASN1Template CERT_IssuerAndSNTemplate[];
extern const SEC_ASN1Template CERT_NameTemplate[];
extern const SEC_ASN1Template CERT_SetOfSignedCrlTemplate[];
extern const SEC_ASN1Template CERT_RDNTemplate[];
extern const SEC_ASN1Template CERT_SignedDataTemplate[];
extern const SEC_ASN1Template CERT_CrlTemplate[];
extern const SEC_ASN1Template CERT_SignedCrlTemplate[];

/*
** XXX should the attribute stuff be centralized for all of ns/security?
*/
extern const SEC_ASN1Template CERT_AttributeTemplate[];
extern const SEC_ASN1Template CERT_SetOfAttributeTemplate[];

/* These functions simply return the address of the above-declared templates.
** This is necessary for Windows DLLs.  Sigh.
*/
SEC_ASN1_CHOOSER_DECLARE(CERT_CertificateRequestTemplate)
SEC_ASN1_CHOOSER_DECLARE(CERT_CertificateTemplate)
SEC_ASN1_CHOOSER_DECLARE(CERT_CrlTemplate)
SEC_ASN1_CHOOSER_DECLARE(CERT_IssuerAndSNTemplate)
SEC_ASN1_CHOOSER_DECLARE(CERT_NameTemplate)
SEC_ASN1_CHOOSER_DECLARE(CERT_SequenceOfCertExtensionTemplate)
SEC_ASN1_CHOOSER_DECLARE(CERT_SetOfSignedCrlTemplate)
SEC_ASN1_CHOOSER_DECLARE(CERT_SignedDataTemplate)
SEC_ASN1_CHOOSER_DECLARE(CERT_SubjectPublicKeyInfoTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_SignedCertificateTemplate)
SEC_ASN1_CHOOSER_DECLARE(CERT_SignedCrlTemplate)
SEC_ASN1_CHOOSER_DECLARE(CERT_TimeChoiceTemplate)

SEC_END_PROTOS

#endif /* _CERTT_H_ */
