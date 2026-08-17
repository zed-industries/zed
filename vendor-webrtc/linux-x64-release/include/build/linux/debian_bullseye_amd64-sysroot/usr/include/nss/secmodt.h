/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#ifndef _SECMODT_H_
#define _SECMODT_H_ 1

#include "nssrwlkt.h"
#include "nssilckt.h"
#include "secoid.h"
#include "secasn1.h"
#include "pkcs11t.h"
#include "utilmodt.h"

SEC_BEGIN_PROTOS

/* find a better home for these... */
extern const SEC_ASN1Template SECKEY_PointerToEncryptedPrivateKeyInfoTemplate[];
SEC_ASN1_CHOOSER_DECLARE(SECKEY_PointerToEncryptedPrivateKeyInfoTemplate)
extern const SEC_ASN1Template SECKEY_EncryptedPrivateKeyInfoTemplate[];
SEC_ASN1_CHOOSER_DECLARE(SECKEY_EncryptedPrivateKeyInfoTemplate)
extern const SEC_ASN1Template SECKEY_PrivateKeyInfoTemplate[];
SEC_ASN1_CHOOSER_DECLARE(SECKEY_PrivateKeyInfoTemplate)
extern const SEC_ASN1Template SECKEY_PointerToPrivateKeyInfoTemplate[];
SEC_ASN1_CHOOSER_DECLARE(SECKEY_PointerToPrivateKeyInfoTemplate)

SEC_END_PROTOS

/* PKCS11 needs to be included */
typedef struct SECMODModuleStr SECMODModule;
typedef struct SECMODModuleListStr SECMODModuleList;
typedef NSSRWLock SECMODListLock;
typedef struct PK11SlotInfoStr PK11SlotInfo;          /* defined in secmodti.h */
typedef struct NSSUTILPreSlotInfoStr PK11PreSlotInfo; /* defined in secmodti.h */
typedef struct PK11SymKeyStr PK11SymKey;              /* defined in secmodti.h */
typedef struct PK11ContextStr PK11Context;            /* defined in secmodti.h */
typedef struct PK11SlotListStr PK11SlotList;
typedef struct PK11SlotListElementStr PK11SlotListElement;
typedef struct PK11RSAGenParamsStr PK11RSAGenParams;
typedef unsigned long SECMODModuleID;
typedef struct PK11DefaultArrayEntryStr PK11DefaultArrayEntry;
typedef struct PK11GenericObjectStr PK11GenericObject;
typedef void (*PK11FreeDataFunc)(void *);

struct SECMODModuleStr {
    PLArenaPool *arena;
    PRBool internal;           /* true of internally linked modules, false
                                * for the loaded modules */
    PRBool loaded;             /* Set to true if module has been loaded */
    PRBool isFIPS;             /* Set to true if module is finst internal */
    char *dllName;             /* name of the shared library which implements
                                * this module */
    char *commonName;          /* name of the module to display to the user */
    void *library;             /* pointer to the library. opaque. used only by
                                * pk11load.c */
    void *functionList;        /* The PKCS #11 function table */
    PZLock *refLock;           /* only used pk11db.c */
    int refCount;              /* Module reference count */
    PK11SlotInfo **slots;      /* array of slot points attached to this mod*/
    int slotCount;             /* count of slot in above array */
    PK11PreSlotInfo *slotInfo; /* special info about slots default settings */
    int slotInfoCount;         /* count */
    SECMODModuleID moduleID;   /* ID so we can find this module again */
    PRBool isThreadSafe;
    unsigned long ssl[2];        /* SSL cipher enable flags */
    char *libraryParams;         /* Module specific parameters */
    void *moduleDBFunc;          /* function to return module configuration data*/
    SECMODModule *parent;        /* module that loaded us */
    PRBool isCritical;           /* This module must load successfully */
    PRBool isModuleDB;           /* this module has lists of PKCS #11 modules */
    PRBool moduleDBOnly;         /* this module only has lists of PKCS #11 modules */
    int trustOrder;              /* order for this module's certificate trust rollup */
    int cipherOrder;             /* order for cipher operations */
    unsigned long evControlMask; /* control the running and shutdown of slot
                                  * events (SECMOD_WaitForAnyTokenEvent) */
    CK_VERSION cryptokiVersion;  /* version of this library */
    CK_FLAGS flags;              /* pkcs11 v3 flags */
};

/* evControlMask flags */
/*
 * These bits tell the current state of a SECMOD_WaitForAnyTokenEvent.
 *
 * SECMOD_WAIT_PKCS11_EVENT - we're waiting in the PKCS #11 module in
 *  C_WaitForSlotEvent().
 * SECMOD_WAIT_SIMULATED_EVENT - we're waiting in the NSS simulation code
 *  which polls for token insertion and removal events.
 * SECMOD_END_WAIT - SECMOD_CancelWait has been called while the module is
 *  waiting in SECMOD_WaitForAnyTokenEvent. SECMOD_WaitForAnyTokenEvent
 *  should return immediately to it's caller.
 */
#define SECMOD_END_WAIT 0x01
#define SECMOD_WAIT_SIMULATED_EVENT 0x02
#define SECMOD_WAIT_PKCS11_EVENT 0x04

struct SECMODModuleListStr {
    SECMODModuleList *next;
    SECMODModule *module;
};

struct PK11SlotListStr {
    PK11SlotListElement *head;
    PK11SlotListElement *tail;
    PZLock *lock;
};

struct PK11SlotListElementStr {
    PK11SlotListElement *next;
    PK11SlotListElement *prev;
    PK11SlotInfo *slot;
    int refCount;
};

struct PK11RSAGenParamsStr {
    int keySizeInBits;
    unsigned long pe;
};

typedef enum {
    PK11CertListUnique = 0,     /* get one instance of all certs */
    PK11CertListUser = 1,       /* get all instances of user certs */
    PK11CertListRootUnique = 2, /* get one instance of CA certs without a private key.
                                 * deprecated. Use PK11CertListCAUnique
                                 */
    PK11CertListCA = 3,         /* get all instances of CA certs */
    PK11CertListCAUnique = 4,   /* get one instance of CA certs */
    PK11CertListUserUnique = 5, /* get one instance of user certs */
    PK11CertListAll = 6         /* get all instances of all certs */
} PK11CertListType;

/*
 * Entry into the array which lists all the legal bits for the default flags
 * in the slot, their definition, and the PKCS #11 mechanism they represent.
 * Always statically allocated.
 */
struct PK11DefaultArrayEntryStr {
    const char *name;
    unsigned long flag;
    unsigned long mechanism; /* this is a long so we don't include the
                              * whole pkcs 11 world to use this header */
};

/*
 * PK11AttrFlags
 *
 * A 32-bit bitmask of PK11_ATTR_XXX flags
 */
typedef PRUint32 PK11AttrFlags;

/*
 * PK11_ATTR_XXX
 *
 * The following PK11_ATTR_XXX bitflags are used to specify
 * PKCS #11 object attributes that have Boolean values.  Some NSS
 * functions have a "PK11AttrFlags attrFlags" parameter whose value
 * is the logical OR of these bitflags.  NSS use these bitflags on
 * private keys or secret keys.  Some of these bitflags also apply
 * to the public keys associated with the private keys.
 *
 * For each PKCS #11 object attribute, we need two bitflags to
 * specify not only "true" and "false" but also "default".  For
 * example, PK11_ATTR_PRIVATE and PK11_ATTR_PUBLIC control the
 * CKA_PRIVATE attribute.  If PK11_ATTR_PRIVATE is set, we add
 *     { CKA_PRIVATE, &cktrue, sizeof(CK_BBOOL) }
 * to the template.  If PK11_ATTR_PUBLIC is set, we add
 *     { CKA_PRIVATE, &ckfalse, sizeof(CK_BBOOL) }
 * to the template.  If neither flag is set, we don't add any
 * CKA_PRIVATE entry to the template.
 */

/*
 * Attributes for PKCS #11 storage objects, which include not only
 * keys but also certificates and domain parameters.
 */

/*
 * PK11_ATTR_TOKEN
 * PK11_ATTR_SESSION
 *
 * These two flags determine whether the object is a token or
 * session object.
 *
 * These two flags are related and cannot both be set.
 * If the PK11_ATTR_TOKEN flag is set, the object is a token
 * object.  If the PK11_ATTR_SESSION flag is set, the object is
 * a session object.  If neither flag is set, the object is *by
 * default* a session object.
 *
 * These two flags specify the value of the PKCS #11 CKA_TOKEN
 * attribute.
 */
#define PK11_ATTR_TOKEN 0x00000001L
#define PK11_ATTR_SESSION 0x00000002L

/*
 * PK11_ATTR_PRIVATE
 * PK11_ATTR_PUBLIC
 *
 * These two flags determine whether the object is a private or
 * public object.  A user may not access a private object until the
 * user has authenticated to the token.
 *
 * These two flags are related and cannot both be set.
 * If the PK11_ATTR_PRIVATE flag is set, the object is a private
 * object.  If the PK11_ATTR_PUBLIC flag is set, the object is a
 * public object.  If neither flag is set, it is token-specific
 * whether the object is private or public.
 *
 * These two flags specify the value of the PKCS #11 CKA_PRIVATE
 * attribute.  NSS only uses this attribute on private and secret
 * keys, so public keys created by NSS get the token-specific
 * default value of the CKA_PRIVATE attribute.
 */
#define PK11_ATTR_PRIVATE 0x00000004L
#define PK11_ATTR_PUBLIC 0x00000008L

/*
 * PK11_ATTR_MODIFIABLE
 * PK11_ATTR_UNMODIFIABLE
 *
 * These two flags determine whether the object is modifiable or
 * read-only.
 *
 * These two flags are related and cannot both be set.
 * If the PK11_ATTR_MODIFIABLE flag is set, the object can be
 * modified.  If the PK11_ATTR_UNMODIFIABLE flag is set, the object
 * is read-only.  If neither flag is set, the object is *by default*
 * modifiable.
 *
 * These two flags specify the value of the PKCS #11 CKA_MODIFIABLE
 * attribute.
 */
#define PK11_ATTR_MODIFIABLE 0x00000010L
#define PK11_ATTR_UNMODIFIABLE 0x00000020L

/* Attributes for PKCS #11 key objects. */

/*
 * PK11_ATTR_SENSITIVE
 * PK11_ATTR_INSENSITIVE
 *
 * These two flags are related and cannot both be set.
 * If the PK11_ATTR_SENSITIVE flag is set, the key is sensitive.
 * If the PK11_ATTR_INSENSITIVE flag is set, the key is not
 * sensitive.  If neither flag is set, it is token-specific whether
 * the key is sensitive or not.
 *
 * If a key is sensitive, certain attributes of the key cannot be
 * revealed in plaintext outside the token.
 *
 * This flag specifies the value of the PKCS #11 CKA_SENSITIVE
 * attribute.  Although the default value of the CKA_SENSITIVE
 * attribute for secret keys is CK_FALSE per PKCS #11, some FIPS
 * tokens set the default value to CK_TRUE because only CK_TRUE
 * is allowed.  So in practice the default value of this attribute
 * is token-specific, hence the need for two bitflags.
 */
#define PK11_ATTR_SENSITIVE 0x00000040L
#define PK11_ATTR_INSENSITIVE 0x00000080L

/*
 * PK11_ATTR_EXTRACTABLE
 * PK11_ATTR_UNEXTRACTABLE
 *
 * These two flags are related and cannot both be set.
 * If the PK11_ATTR_EXTRACTABLE flag is set, the key is extractable
 * and can be wrapped.  If the PK11_ATTR_UNEXTRACTABLE flag is set,
 * the key is not extractable, and certain attributes of the key
 * cannot be revealed in plaintext outside the token (just like a
 * sensitive key).  If neither flag is set, it is token-specific
 * whether the key is extractable or not.
 *
 * These two flags specify the value of the PKCS #11 CKA_EXTRACTABLE
 * attribute.
 */
#define PK11_ATTR_EXTRACTABLE 0x00000100L
#define PK11_ATTR_UNEXTRACTABLE 0x00000200L

/* Cryptographic module types */
#define SECMOD_EXTERNAL 0 /* external module */
#define SECMOD_INTERNAL 1 /* internal default module */
#define SECMOD_FIPS 2     /* internal fips module */

/* default module configuration strings */
#define SECMOD_SLOT_FLAGS "slotFlags=[RSA,DSA,DH,RC2,RC4,DES,RANDOM,SHA1,MD5,MD2,SSL,TLS,AES,Camellia,SEED,SHA256,SHA512]"

#define SECMOD_MAKE_NSS_FLAGS(fips, slot) \
    "Flags=internal,critical" fips " slotparams=(" #slot "={" SECMOD_SLOT_FLAGS "})"

#define SECMOD_INT_NAME "NSS Internal PKCS #11 Module"
#define SECMOD_INT_FLAGS SECMOD_MAKE_NSS_FLAGS("", 1)
#define SECMOD_FIPS_NAME "NSS Internal FIPS PKCS #11 Module"
#define SECMOD_FIPS_FLAGS SECMOD_MAKE_NSS_FLAGS(",fips", 3)

/*
 * What is the origin of a given Key. Normally this doesn't matter, but
 * the fortezza code needs to know if it needs to invoke the SSL3 fortezza
 * hack.
 */
typedef enum {
    PK11_OriginNULL = 0,         /* There is not key, it's a null SymKey */
    PK11_OriginDerive = 1,       /* Key was derived from some other key */
    PK11_OriginGenerated = 2,    /* Key was generated (also PBE keys) */
    PK11_OriginFortezzaHack = 3, /* Key was marked for fortezza hack */
    PK11_OriginUnwrap = 4        /* Key was unwrapped or decrypted */
} PK11Origin;

/* PKCS #11 disable reasons */
typedef enum {
    PK11_DIS_NONE = 0,
    PK11_DIS_USER_SELECTED = 1,
    PK11_DIS_COULD_NOT_INIT_TOKEN = 2,
    PK11_DIS_TOKEN_VERIFY_FAILED = 3,
    PK11_DIS_TOKEN_NOT_PRESENT = 4
} PK11DisableReasons;

/* types of PKCS #11 objects
 * used to identify which NSS data structure is
 * passed to the PK11_Raw* functions. Types map as follows:
 *   PK11_TypeGeneric            PK11GenericObject *
 *   PK11_TypePrivKey            SECKEYPrivateKey *
 *   PK11_TypePubKey             SECKEYPublicKey *
 *   PK11_TypeSymKey             PK11SymKey *
 *   PK11_TypeCert               CERTCertificate * (currently not used).
 */
typedef enum {
    PK11_TypeGeneric = 0,
    PK11_TypePrivKey = 1,
    PK11_TypePubKey = 2,
    PK11_TypeCert = 3,
    PK11_TypeSymKey = 4
} PK11ObjectType;

/* function pointer type for password callback function.
 * This type is passed in to PK11_SetPasswordFunc()
 */
typedef char *(PR_CALLBACK *PK11PasswordFunc)(PK11SlotInfo *slot, PRBool retry, void *arg);
typedef PRBool(PR_CALLBACK *PK11VerifyPasswordFunc)(PK11SlotInfo *slot, void *arg);
typedef PRBool(PR_CALLBACK *PK11IsLoggedInFunc)(PK11SlotInfo *slot, void *arg);

/*
 * Special strings the password callback function can return only if
 * the slot is an protected auth path slot.
 */
#define PK11_PW_RETRY "RETRY"        /* an failed attempt to authenticate \
                                      * has already been made, just retry \
                                      * the operation */
#define PK11_PW_AUTHENTICATED "AUTH" /* a successful attempt to authenticate \
                                      * has completed. Continue without      \
                                      * another call to C_Login */
/* All other non-null values mean that that NSS could call C_Login to force
 * the authentication. The following define is to aid applications in
 * documenting that is what it's trying to do */
#define PK11_PW_TRY "TRY" /* Default: a prompt has been presented \
                           * to the user, initiate a C_Login      \
                           * to authenticate the token */

/*
 * PKCS #11 key structures
 */

/*
** Attributes
*/
struct SECKEYAttributeStr {
    SECItem attrType;
    SECItem **attrValue;
};
typedef struct SECKEYAttributeStr SECKEYAttribute;

/*
** A PKCS#8 private key info object
*/
struct SECKEYPrivateKeyInfoStr {
    PLArenaPool *arena;
    SECItem version;
    SECAlgorithmID algorithm;
    SECItem privateKey;
    SECKEYAttribute **attributes;
};
typedef struct SECKEYPrivateKeyInfoStr SECKEYPrivateKeyInfo;

/*
** A PKCS#8 private key info object
*/
struct SECKEYEncryptedPrivateKeyInfoStr {
    PLArenaPool *arena;
    SECAlgorithmID algorithm;
    SECItem encryptedData;
};
typedef struct SECKEYEncryptedPrivateKeyInfoStr SECKEYEncryptedPrivateKeyInfo;

/*
 * token removal detection
 */
typedef enum {
    PK11TokenNotRemovable = 0,
    PK11TokenPresent = 1,
    PK11TokenChanged = 2,
    PK11TokenRemoved = 3
} PK11TokenStatus;

typedef enum {
    PK11TokenRemovedOrChangedEvent = 0,
    PK11TokenPresentEvent = 1
} PK11TokenEvent;

/*
 * CRL Import Flags
 */
#define CRL_IMPORT_DEFAULT_OPTIONS 0x00000000
#define CRL_IMPORT_BYPASS_CHECKS 0x00000001

/*
 * Merge Error Log
 */
typedef struct PK11MergeLogStr PK11MergeLog;
typedef struct PK11MergeLogNodeStr PK11MergeLogNode;

/* These need to be global, leave some open fields so we can 'expand'
 * these without breaking binary compatibility */
struct PK11MergeLogNodeStr {
    PK11MergeLogNode *next;    /* next entry in the list */
    PK11MergeLogNode *prev;    /* last entry in the list */
    PK11GenericObject *object; /* object that failed */
    int error;                 /* what the error was */
    CK_RV reserved1;
    unsigned long reserved2; /* future flags */
    unsigned long reserved3; /* future scalar */
    void *reserved4;         /* future pointer */
    void *reserved5;         /* future expansion pointer */
};

struct PK11MergeLogStr {
    PK11MergeLogNode *head;
    PK11MergeLogNode *tail;
    PLArenaPool *arena;
    int version;
    unsigned long reserved1;
    unsigned long reserved2;
    unsigned long reserved3;
    void *reserverd4;
    void *reserverd5;
};

#endif /*_SECMODT_H_ */
