/*
 * NSS utility functions
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef __nss_h_
#define __nss_h_

/* The private macro _NSS_CUSTOMIZED is for NSS internal use only. */
#if defined(NSS_ALLOW_UNSUPPORTED_CRITICAL)
#define _NSS_CUSTOMIZED " (Customized build)"
#else
#define _NSS_CUSTOMIZED
#endif

/*
 * NSS's major version, minor version, patch level, build number, and whether
 * this is a beta release.
 *
 * The format of the version string should be
 *     "<major version>.<minor version>[.<patch level>[.<build number>]][ <ECC>][ <Beta>]"
 */
#define NSS_VERSION "3.61" _NSS_CUSTOMIZED
#define NSS_VMAJOR 3
#define NSS_VMINOR 61
#define NSS_VPATCH 0
#define NSS_VBUILD 0
#define NSS_BETA PR_FALSE

#ifndef RC_INVOKED

#include "seccomon.h"

typedef struct NSSInitParametersStr NSSInitParameters;

/*
 * parameters used to initialize softoken. Mostly strings used to
 * internationalize softoken. Memory for the strings are owned by the caller,
 * who is free to free them once NSS_ContextInit returns. If the string
 * parameter is NULL (as opposed to empty, zero length), then the softoken
 * default is used. These are equivalent to the parameters for
 * PK11_ConfigurePKCS11().
 *
 * field names match their equivalent parameter names for softoken strings
 * documented at https://developer.mozilla.org/en/PKCS11_Module_Specs.
 *
 * minPWLen
 *     Minimum password length in bytes.
 * manufacturerID
 *     Override the default manufactureID value for the module returned in
 *     the CK_INFO, CK_SLOT_INFO, and CK_TOKEN_INFO structures with an
 *     internationalize string (UTF8). This value will be truncated at 32
 *     bytes (not including the trailing NULL, partial UTF8 characters will be
 *     dropped).
 * libraryDescription
 *     Override the default libraryDescription value for the module returned in
 *     the CK_INFO structure with an internationalize string (UTF8). This value
 *     will be truncated at 32 bytes(not including the trailing NULL, partial
 *     UTF8 characters will be dropped).
 * cryptoTokenDescription
 *     Override the default label value for the internal crypto token returned
 *     in the CK_TOKEN_INFO structure with an internationalize string (UTF8).
 *     This value will be truncated at 32 bytes (not including the trailing
 *     NULL, partial UTF8 characters will be dropped).
 * dbTokenDescription
 *     Override the default label value for the internal DB token returned in
 *     the CK_TOKEN_INFO structure with an internationalize string (UTF8). This
 *     value will be truncated at 32 bytes (not including the trailing NULL,
 *     partial UTF8 characters will be dropped).
 * FIPSTokenDescription
 *     Override the default label value for the internal FIPS token returned in
 *     the CK_TOKEN_INFO structure with an internationalize string (UTF8). This
 *     value will be truncated at 32 bytes (not including the trailing NULL,
 *     partial UTF8 characters will be dropped).
 * cryptoSlotDescription
 *     Override the default slotDescription value for the internal crypto token
 *     returned in the CK_SLOT_INFO structure with an internationalize string
 *     (UTF8). This value will be truncated at 64 bytes (not including the
 *     trailing NULL, partial UTF8 characters will be dropped).
 * dbSlotDescription
 *     Override the default slotDescription value for the internal DB token
 *     returned in the CK_SLOT_INFO structure with an internationalize string
 *     (UTF8). This value will be truncated at 64 bytes (not including the
 *     trailing NULL, partial UTF8 characters will be dropped).
 * FIPSSlotDescription
 *     Override the default slotDecription value for the internal FIPS token
 *     returned in the CK_SLOT_INFO structure with an internationalize string
 *     (UTF8). This value will be truncated at 64 bytes (not including the
 *     trailing NULL, partial UTF8 characters will be dropped).
 *
 */
struct NSSInitParametersStr {
    unsigned int length; /* allow this structure to grow in the future,
                                * must be set */
    PRBool passwordRequired;
    int minPWLen;
    char *manufactureID;      /* variable names for strings match the */
    char *libraryDescription; /*   parameter name in softoken */
    char *cryptoTokenDescription;
    char *dbTokenDescription;
    char *FIPSTokenDescription;
    char *cryptoSlotDescription;
    char *dbSlotDescription;
    char *FIPSSlotDescription;
};

SEC_BEGIN_PROTOS

/*
 * Return a boolean that indicates whether the underlying library
 * will perform as the caller expects.
 *
 * The only argument is a string, which should be the version
 * identifier of the NSS library. That string will be compared
 * against a string that represents the actual build version of
 * the NSS library.
 */
extern PRBool NSS_VersionCheck(const char *importedVersion);

/*
 * Returns a const string of the NSS library version.
 */
extern const char *NSS_GetVersion(void);

/*
 * Open the Cert, Key, and Security Module databases, read only.
 * Initialize the Random Number Generator.
 * Does not initialize the cipher policies or enables.
 * Default policy settings disallow all ciphers.
 */
extern SECStatus NSS_Init(const char *configdir);

/*
 * Returns whether NSS has already been initialized or not.
 */
extern PRBool NSS_IsInitialized(void);

/*
 * Open the Cert, Key, and Security Module databases, read/write.
 * Initialize the Random Number Generator.
 * Does not initialize the cipher policies or enables.
 * Default policy settings disallow all ciphers.
 */
extern SECStatus NSS_InitReadWrite(const char *configdir);

/*
 * Open the Cert, Key, and Security Module databases, read/write.
 * Initialize the Random Number Generator.
 * Does not initialize the cipher policies or enables.
 * Default policy settings disallow all ciphers.
 *
 * This allows using application defined prefixes for the cert and key db's
 * and an alternate name for the secmod database. NOTE: In future releases,
 * the database prefixes my not necessarily map to database names.
 *
 * configdir - base directory where all the cert, key, and module datbases live.
 * certPrefix - prefix added to the beginning of the cert database example: "
 *                      "https-server1-"
 * keyPrefix - prefix added to the beginning of the key database example: "
 *                      "https-server1-"
 * secmodName - name of the security module database (usually "secmod.db").
 * flags - change the open options of NSS_Initialize as follows:
 *      NSS_INIT_READONLY - Open the databases read only.
 *      NSS_INIT_NOCERTDB - Don't open the cert DB and key DB's, just
 *                      initialize the volatile certdb.
 *      NSS_INIT_NOMODDB  - Don't open the security module DB, just
 *                      initialize the  PKCS #11 module.
 *      NSS_INIT_FORCEOPEN - Continue to force initializations even if the
 *                      databases cannot be opened.
 *      NSS_INIT_NOROOTINIT - Don't try to look for the root certs module
 *                      automatically.
 *      NSS_INIT_OPTIMIZESPACE - Use smaller tables and caches.
 *      NSS_INIT_PK11THREADSAFE - only load PKCS#11 modules that are
 *                      thread-safe, ie. that support locking - either OS
 *                      locking or NSS-provided locks . If a PKCS#11
 *                      module isn't thread-safe, don't serialize its
 *                      calls; just don't load it instead. This is necessary
 *                      if another piece of code is using the same PKCS#11
 *                      modules that NSS is accessing without going through
 *                      NSS, for example the Java SunPKCS11 provider.
 *      NSS_INIT_PK11RELOAD - ignore the CKR_CRYPTOKI_ALREADY_INITIALIZED
 *                      error when loading PKCS#11 modules. This is necessary
 *                      if another piece of code is using the same PKCS#11
 *                      modules that NSS is accessing without going through
 *                      NSS, for example Java SunPKCS11 provider.
 *      NSS_INIT_NOPK11FINALIZE - never call C_Finalize on any
 *                      PKCS#11 module. This may be necessary in order to
 *                      ensure continuous operation and proper shutdown
 *                      sequence if another piece of code is using the same
 *                      PKCS#11 modules that NSS is accessing without going
 *                      through NSS, for example Java SunPKCS11 provider.
 *                      The following limitation applies when this is set :
 *                      SECMOD_WaitForAnyTokenEvent will not use
 *                      C_WaitForSlotEvent, in order to prevent the need for
 *                      C_Finalize. This call will be emulated instead.
 *      NSS_INIT_RESERVED - Currently has no effect, but may be used in the
 *                      future to trigger better cooperation between PKCS#11
 *                      modules used by both NSS and the Java SunPKCS11
 *                      provider. This should occur after a new flag is defined
 *                      for C_Initialize by the PKCS#11 working group.
 *      NSS_INIT_COOPERATE - Sets 4 recommended options for applications that
 *                      use both NSS and the Java SunPKCS11 provider.
 *
 * Also NOTE: This is not the recommended method for initializing NSS.
 * The preferred method is NSS_init().
 */
#define NSS_INIT_READONLY 0x1
#define NSS_INIT_NOCERTDB 0x2
#define NSS_INIT_NOMODDB 0x4
#define NSS_INIT_FORCEOPEN 0x8
#define NSS_INIT_NOROOTINIT 0x10
#define NSS_INIT_OPTIMIZESPACE 0x20
#define NSS_INIT_PK11THREADSAFE 0x40
#define NSS_INIT_PK11RELOAD 0x80
#define NSS_INIT_NOPK11FINALIZE 0x100
#define NSS_INIT_RESERVED 0x200

#define NSS_INIT_COOPERATE NSS_INIT_PK11THREADSAFE |     \
                               NSS_INIT_PK11RELOAD |     \
                               NSS_INIT_NOPK11FINALIZE | \
                               NSS_INIT_RESERVED

#define SECMOD_DB "secmod.db"

typedef struct NSSInitContextStr NSSInitContext;

extern SECStatus NSS_Initialize(const char *configdir,
                                const char *certPrefix, const char *keyPrefix,
                                const char *secmodName, PRUint32 flags);

extern NSSInitContext *NSS_InitContext(const char *configdir,
                                       const char *certPrefix, const char *keyPrefix,
                                       const char *secmodName, NSSInitParameters *initParams, PRUint32 flags);

extern SECStatus NSS_ShutdownContext(NSSInitContext *);

/*
 * same as NSS_Init, but checks to see if we need to merge an
 * old database in.
 *   updatedir is the directory where the old database lives.
 *   updCertPrefix is the certPrefix for the old database.
 *   updKeyPrefix is the keyPrefix for the old database.
 *   updateID is a unique identifier chosen by the application for
 *      the specific database.
 *   updatName is the name the user will be prompted for when
 *      asking to authenticate to the old database  */
extern SECStatus NSS_InitWithMerge(const char *configdir,
                                   const char *certPrefix, const char *keyPrefix, const char *secmodName,
                                   const char *updatedir, const char *updCertPrefix,
                                   const char *updKeyPrefix, const char *updateID,
                                   const char *updateName, PRUint32 flags);
/*
 * initialize NSS without a creating cert db's, key db's, or secmod db's.
 */
SECStatus NSS_NoDB_Init(const char *configdir);

/*
 * Allow applications and libraries to register with NSS so that they are called
 * when NSS shuts down.
 *
 * void *appData application specific data passed in by the application at
 * NSS_RegisterShutdown() time.
 * void *nssData is NULL in this release, but is reserved for future versions of
 * NSS to pass some future status information * back to the shutdown function.
 *
 * If the shutdown function returns SECFailure,
 * Shutdown will still complete, but NSS_Shutdown() will return SECFailure.
 */
typedef SECStatus (*NSS_ShutdownFunc)(void *appData, void *nssData);

/*
 * Register a shutdown function.
 */
SECStatus NSS_RegisterShutdown(NSS_ShutdownFunc sFunc, void *appData);

/*
 * Remove an existing shutdown function (you may do this if your library is
 * complete and going away, but NSS is still running).
 */
SECStatus NSS_UnregisterShutdown(NSS_ShutdownFunc sFunc, void *appData);

/* Available options for NSS_OptionSet() and NSS_OptionGet().
 */
#define NSS_RSA_MIN_KEY_SIZE 0x001
#define NSS_DH_MIN_KEY_SIZE 0x002
#define NSS_DSA_MIN_KEY_SIZE 0x004
#define NSS_TLS_VERSION_MIN_POLICY 0x008
#define NSS_TLS_VERSION_MAX_POLICY 0x009
#define NSS_DTLS_VERSION_MIN_POLICY 0x00a
#define NSS_DTLS_VERSION_MAX_POLICY 0x00b

/* Until NSS 3.30, the PKCS#12 implementation used BMPString encoding
 * for all passwords.  This changed to use UTF-8 for non-PKCS#12 PBEs
 * in NSS 3.31.
 *
 * For backward compatibility, this option reverts the behavior to the
 * old NSS versions.  This option might be removed in the future NSS
 * releases; don't rely on it. */
#define __NSS_PKCS12_DECODE_FORCE_UNICODE 0x00c
#define NSS_DEFAULT_LOCKS 0x00d /* lock default values */
#define NSS_DEFAULT_SSL_LOCK 1  /* lock the ssl default values */

/*
 * Set and get global options for the NSS library.
 */
SECStatus NSS_OptionSet(PRInt32 which, PRInt32 value);
SECStatus NSS_OptionGet(PRInt32 which, PRInt32 *value);

/*
 * Close the Cert, Key databases.
 */
extern SECStatus NSS_Shutdown(void);

/*
 * set the PKCS #11 strings for the internal token.
 */
void PK11_ConfigurePKCS11(const char *man, const char *libdesc,
                          const char *tokdesc, const char *ptokdesc, const char *slotdesc,
                          const char *pslotdesc, const char *fslotdesc, const char *fpslotdesc,
                          int minPwd, int pwRequired);

/*
 * Dump the contents of the certificate cache and the temporary cert store.
 * Use to detect leaked references of certs at shutdown time.
 */
void nss_DumpCertificateCacheInfo(void);

SEC_END_PROTOS

#endif /* RC_INVOKED */
#endif /* __nss_h_ */
