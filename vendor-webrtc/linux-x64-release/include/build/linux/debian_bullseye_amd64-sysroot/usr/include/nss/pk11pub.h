/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#ifndef _PK11PUB_H_
#define _PK11PUB_H_
#include "plarena.h"
#include "seccomon.h"
#include "secoidt.h"
#include "secdert.h"
#include "keythi.h"
#include "certt.h"
#include "pk11hpke.h"
#include "pkcs11t.h"
#include "secmodt.h"
#include "seccomon.h"
#include "pkcs7t.h"
#include "cmsreclist.h"

/*
 * Exported PK11 wrap functions.
 */

SEC_BEGIN_PROTOS

/************************************************************
 * Generic Slot Lists Management
 ************************************************************/
void PK11_FreeSlotList(PK11SlotList *list);
SECStatus PK11_FreeSlotListElement(PK11SlotList *list, PK11SlotListElement *le);
PK11SlotListElement *PK11_GetFirstSafe(PK11SlotList *list);
PK11SlotListElement *PK11_GetNextSafe(PK11SlotList *list,
                                      PK11SlotListElement *le, PRBool restart);

/************************************************************
 * Generic Slot Management
 ************************************************************/
PK11SlotInfo *PK11_ReferenceSlot(PK11SlotInfo *slot);
void PK11_FreeSlot(PK11SlotInfo *slot);
SECStatus PK11_DestroyObject(PK11SlotInfo *slot, CK_OBJECT_HANDLE object);
SECStatus PK11_DestroyTokenObject(PK11SlotInfo *slot, CK_OBJECT_HANDLE object);
PK11SlotInfo *PK11_GetInternalKeySlot(void);
PK11SlotInfo *PK11_GetInternalSlot(void);
SECStatus PK11_Logout(PK11SlotInfo *slot);
void PK11_LogoutAll(void);

/************************************************************
 *  Slot Password Management
 ************************************************************/
void PK11_SetSlotPWValues(PK11SlotInfo *slot, int askpw, int timeout);
void PK11_GetSlotPWValues(PK11SlotInfo *slot, int *askpw, int *timeout);
SECStatus PK11_CheckSSOPassword(PK11SlotInfo *slot, char *ssopw);
SECStatus PK11_CheckUserPassword(PK11SlotInfo *slot, const char *pw);
PRBool PK11_IsLoggedIn(PK11SlotInfo *slot, void *wincx);
SECStatus PK11_InitPin(PK11SlotInfo *slot, const char *ssopw,
                       const char *pk11_userpwd);
SECStatus PK11_ChangePW(PK11SlotInfo *slot, const char *oldpw,
                        const char *newpw);
void PK11_SetPasswordFunc(PK11PasswordFunc func);
int PK11_GetMinimumPwdLength(PK11SlotInfo *slot);
SECStatus PK11_ResetToken(PK11SlotInfo *slot, char *sso_pwd);
SECStatus PK11_Authenticate(PK11SlotInfo *slot, PRBool loadCerts, void *wincx);
SECStatus PK11_TokenRefresh(PK11SlotInfo *slot);

/******************************************************************
 *           Slot info functions
 ******************************************************************/
PK11SlotInfo *PK11_FindSlotByName(const char *name);
/******************************************************************
 * PK11_FindSlotsByNames searches for a PK11SlotInfo using one or
 * more criteria : dllName, slotName and tokenName . In addition, if
 * presentOnly is set , only slots with a token inserted will be
 * returned.
 ******************************************************************/
PK11SlotList *PK11_FindSlotsByNames(const char *dllName,
                                    const char *slotName, const char *tokenName, PRBool presentOnly);
PRBool PK11_IsReadOnly(PK11SlotInfo *slot);
PRBool PK11_IsInternal(PK11SlotInfo *slot);
PRBool PK11_IsInternalKeySlot(PK11SlotInfo *slot);
char *PK11_GetTokenName(PK11SlotInfo *slot);
char *PK11_GetTokenURI(PK11SlotInfo *slot);
char *PK11_GetSlotName(PK11SlotInfo *slot);
PRBool PK11_NeedLogin(PK11SlotInfo *slot);
PRBool PK11_IsFriendly(PK11SlotInfo *slot);
PRBool PK11_IsHW(PK11SlotInfo *slot);
PRBool PK11_IsRemovable(PK11SlotInfo *slot);
PRBool PK11_NeedUserInit(PK11SlotInfo *slot);
PRBool PK11_ProtectedAuthenticationPath(PK11SlotInfo *slot);
int PK11_GetSlotSeries(PK11SlotInfo *slot);
int PK11_GetCurrentWrapIndex(PK11SlotInfo *slot);
unsigned long PK11_GetDefaultFlags(PK11SlotInfo *slot);
CK_SLOT_ID PK11_GetSlotID(PK11SlotInfo *slot);
SECMODModuleID PK11_GetModuleID(PK11SlotInfo *slot);
SECStatus PK11_GetSlotInfo(PK11SlotInfo *slot, CK_SLOT_INFO *info);
SECStatus PK11_GetTokenInfo(PK11SlotInfo *slot, CK_TOKEN_INFO *info);
PRBool PK11_IsDisabled(PK11SlotInfo *slot);
PRBool PK11_HasRootCerts(PK11SlotInfo *slot);
PK11DisableReasons PK11_GetDisabledReason(PK11SlotInfo *slot);
/* Prevents the slot from being used, and set disable reason to user-disable */
/* NOTE: Mechanisms that were ON continue to stay ON */
/*       Therefore, when the slot is enabled, it will remember */
/*       what mechanisms needs to be turned on */
PRBool PK11_UserDisableSlot(PK11SlotInfo *slot);
/* Allow all mechanisms that are ON before UserDisableSlot() */
/* was called to be available again */
PRBool PK11_UserEnableSlot(PK11SlotInfo *slot);
/*
 * wait for a specific slot event.
 * event is a specific event to wait for. Currently only
 *    PK11TokenChangeOrRemovalEvent and PK11TokenPresentEvents are defined.
 * timeout can be an interval time to wait, PR_INTERVAL_NO_WAIT (meaning only
 * poll once), or PR_INTERVAL_NO_TIMEOUT (meaning block until a change).
 * pollInterval is a suggested pulling interval value. '0' means use the
 *  default. Future implementations that don't poll may ignore this value.
 * series is the current series for the last slot. This should be the series
 *  value for the slot the last time you read persistant information from the
 *  slot. For instance, if you publish a cert from the slot, you should obtain
 *  the slot series at that time. Then PK11_WaitForTokenEvent can detect a
 *  a change in the slot between the time you publish and the time
 *  PK11_WaitForTokenEvent is called, elliminating potential race conditions.
 *
 * The current status that is returned is:
 *   PK11TokenNotRemovable - always returned for any non-removable token.
 *   PK11TokenPresent - returned when the token is present and we are waiting
 *     on a PK11TokenPresentEvent. Then next event to look for is a
 *     PK11TokenChangeOrRemovalEvent.
 *   PK11TokenChanged - returned when the old token has been removed and a new
 *     token ad been inserted, and we are waiting for a
 *     PK11TokenChangeOrRemovalEvent. The next event to look for is another
 *     PK11TokenChangeOrRemovalEvent.
 *   PK11TokenRemoved - returned when the token is not present and we are
 *     waiting for a PK11TokenChangeOrRemovalEvent. The next event to look for
 *     is a PK11TokenPresentEvent.
 */
PK11TokenStatus PK11_WaitForTokenEvent(PK11SlotInfo *slot, PK11TokenEvent event,
                                       PRIntervalTime timeout, PRIntervalTime pollInterval, int series);

PRBool PK11_NeedPWInit(void);
PRBool PK11_TokenExists(CK_MECHANISM_TYPE);
SECStatus PK11_GetModInfo(SECMODModule *mod, CK_INFO *info);
char *PK11_GetModuleURI(SECMODModule *mod);
PRBool PK11_IsFIPS(void);
SECMODModule *PK11_GetModule(PK11SlotInfo *slot);

/*********************************************************************
 *            Slot mapping utility functions.
 *********************************************************************/
PRBool PK11_IsPresent(PK11SlotInfo *slot);
PRBool PK11_DoesMechanism(PK11SlotInfo *slot, CK_MECHANISM_TYPE type);
PK11SlotList *PK11_GetAllTokens(CK_MECHANISM_TYPE type, PRBool needRW,
                                PRBool loadCerts, void *wincx);
PK11SlotInfo *PK11_GetBestSlotMultipleWithAttributes(CK_MECHANISM_TYPE *type,
                                                     CK_FLAGS *mechFlag, unsigned int *keySize,
                                                     unsigned int count, void *wincx);
PK11SlotInfo *PK11_GetBestSlotMultiple(CK_MECHANISM_TYPE *type,
                                       unsigned int count, void *wincx);
PK11SlotInfo *PK11_GetBestSlot(CK_MECHANISM_TYPE type, void *wincx);
PK11SlotInfo *PK11_GetBestSlotWithAttributes(CK_MECHANISM_TYPE type,
                                             CK_FLAGS mechFlag, unsigned int keySize, void *wincx);
CK_MECHANISM_TYPE PK11_GetBestWrapMechanism(PK11SlotInfo *slot);
int PK11_GetBestKeyLength(PK11SlotInfo *slot, CK_MECHANISM_TYPE type);

/*
 * Open a new database using the softoken. The caller is responsible for making
 * sure the module spec is correct and usable. The caller should ask for one
 * new database per call if the caller wants to get meaningful information
 * about the new database.
 *
 * moduleSpec is the same data that you would pass to softoken at
 * initialization time under the 'tokens' options. For example, if you were
 * to specify tokens=<0x4=[configdir='./mybackup' tokenDescription='Backup']>
 * You would specify "configdir='./mybackup' tokenDescription='Backup'" as your
 * module spec here. The slot ID will be calculated for you by
 * SECMOD_OpenUserDB().
 *
 * Typical parameters here are configdir, tokenDescription and flags.
 *
 * a Full list is below:
 *
 *
 *  configDir - The location of the databases for this token. If configDir is
 *         not specified, and noCertDB and noKeyDB is not specified, the load
 *         will fail.
 *   certPrefix - Cert prefix for this token.
 *   keyPrefix - Prefix for the key database for this token. (if not specified,
 *         certPrefix will be used).
 *   tokenDescription - The label value for this token returned in the
 *         CK_TOKEN_INFO structure with an internationalize string (UTF8).
 *         This value will be truncated at 32 bytes (no NULL, partial UTF8
 *         characters dropped). You should specify a user friendly name here
 *         as this is the value the token will be referred to in most
 *         application UI's. You should make sure tokenDescription is unique.
 *   slotDescription - The slotDescription value for this token returned
 *         in the CK_SLOT_INFO structure with an internationalize string
 *         (UTF8). This value will be truncated at 64 bytes (no NULL, partial
 *         UTF8 characters dropped). This name will not change after the
 *         database is closed. It should have some number to make this unique.
 *   minPWLen - minimum password length for this token.
 *   flags - comma separated list of flag values, parsed case-insensitive.
 *         Valid flags are:
 *              readOnly - Databases should be opened read only.
 *              noCertDB - Don't try to open a certificate database.
 *              noKeyDB - Don't try to open a key database.
 *              forceOpen - Don't fail to initialize the token if the
 *                databases could not be opened.
 *              passwordRequired - zero length passwords are not acceptable
 *                (valid only if there is a keyDB).
 *              optimizeSpace - allocate smaller hash tables and lock tables.
 *                When this flag is not specified, Softoken will allocate
 *                large tables to prevent lock contention.
 */
PK11SlotInfo *SECMOD_OpenUserDB(const char *moduleSpec);
SECStatus SECMOD_CloseUserDB(PK11SlotInfo *slot);

/*
 * This is exactly the same as OpenUserDB except it can be called on any
 * module that understands softoken style new slot entries. The resulting
 * slot can be closed using SECMOD_CloseUserDB above. Value of moduleSpec
 * is token specific.
 */
PK11SlotInfo *SECMOD_OpenNewSlot(SECMODModule *mod, const char *moduleSpec);

/*
 * merge the permanent objects from on token to another
 */
SECStatus PK11_MergeTokens(PK11SlotInfo *targetSlot, PK11SlotInfo *sourceSlot,
                           PK11MergeLog *log, void *targetPwArg, void *sourcePwArg);

/*
 * create and destroy merge logs needed by PK11_MergeTokens
 */
PK11MergeLog *PK11_CreateMergeLog(void);
void PK11_DestroyMergeLog(PK11MergeLog *log);

/*********************************************************************
 *       Mechanism Mapping functions
 *********************************************************************/
CK_KEY_TYPE PK11_GetKeyType(CK_MECHANISM_TYPE type, unsigned long len);
CK_MECHANISM_TYPE PK11_GetKeyGen(CK_MECHANISM_TYPE type);
int PK11_GetBlockSize(CK_MECHANISM_TYPE type, SECItem *params);
int PK11_GetIVLength(CK_MECHANISM_TYPE type);
SECItem *PK11_ParamFromIV(CK_MECHANISM_TYPE type, SECItem *iv);
unsigned char *PK11_IVFromParam(CK_MECHANISM_TYPE type, SECItem *param, int *len);
SECItem *PK11_BlockData(SECItem *data, unsigned long size);

/* PKCS #11 to DER mapping functions */
SECItem *PK11_ParamFromAlgid(SECAlgorithmID *algid);
SECItem *PK11_GenerateNewParam(CK_MECHANISM_TYPE, PK11SymKey *);
CK_MECHANISM_TYPE PK11_AlgtagToMechanism(SECOidTag algTag);
SECOidTag PK11_MechanismToAlgtag(CK_MECHANISM_TYPE type);
SECOidTag PK11_FortezzaMapSig(SECOidTag algTag);
SECStatus PK11_ParamToAlgid(SECOidTag algtag, SECItem *param,
                            PLArenaPool *arena, SECAlgorithmID *algid);
SECStatus PK11_SeedRandom(PK11SlotInfo *, unsigned char *data, int len);
SECStatus PK11_GenerateRandomOnSlot(PK11SlotInfo *, unsigned char *data, int len);
SECStatus PK11_RandomUpdate(void *data, size_t bytes);
SECStatus PK11_GenerateRandom(unsigned char *data, int len);

/* warning: cannot work with pkcs 5 v2
 * use algorithm ID s instead of pkcs #11 mechanism pointers */
CK_RV PK11_MapPBEMechanismToCryptoMechanism(CK_MECHANISM_PTR pPBEMechanism,
                                            CK_MECHANISM_PTR pCryptoMechanism,
                                            SECItem *pbe_pwd, PRBool bad3DES);
CK_MECHANISM_TYPE PK11_GetPadMechanism(CK_MECHANISM_TYPE);
CK_MECHANISM_TYPE PK11_MapSignKeyType(KeyType keyType);

/**********************************************************************
 *                   Symmetric, Public, and Private Keys
 **********************************************************************/
void PK11_FreeSymKey(PK11SymKey *key);
PK11SymKey *PK11_ReferenceSymKey(PK11SymKey *symKey);
PK11SymKey *PK11_ImportDataKey(PK11SlotInfo *slot, CK_MECHANISM_TYPE type, PK11Origin origin,
                               CK_ATTRIBUTE_TYPE operation, SECItem *key, void *wincx);
PK11SymKey *PK11_ImportSymKey(PK11SlotInfo *slot, CK_MECHANISM_TYPE type,
                              PK11Origin origin, CK_ATTRIBUTE_TYPE operation, SECItem *key, void *wincx);
PK11SymKey *PK11_ImportSymKeyWithFlags(PK11SlotInfo *slot,
                                       CK_MECHANISM_TYPE type, PK11Origin origin, CK_ATTRIBUTE_TYPE operation,
                                       SECItem *key, CK_FLAGS flags, PRBool isPerm, void *wincx);
PK11SymKey *PK11_SymKeyFromHandle(PK11SlotInfo *slot, PK11SymKey *parent,
                                  PK11Origin origin, CK_MECHANISM_TYPE type, CK_OBJECT_HANDLE keyID,
                                  PRBool owner, void *wincx);
/* PK11_GetWrapKey and PK11_SetWrapKey are not thread safe. */
PK11SymKey *PK11_GetWrapKey(PK11SlotInfo *slot, int wrap,
                            CK_MECHANISM_TYPE type, int series, void *wincx);
void PK11_SetWrapKey(PK11SlotInfo *slot, int wrap, PK11SymKey *wrapKey);
CK_MECHANISM_TYPE PK11_GetMechanism(PK11SymKey *symKey);
/*
 * import a public key into the desired slot
 *
 * This function takes a public key structure and creates a public key in a
 * given slot. If isToken is set, then a persistant public key is created.
 *
 * Note: it is possible for this function to return a handle for a key which
 * is persistant, even if isToken is not set.
 */
CK_OBJECT_HANDLE PK11_ImportPublicKey(PK11SlotInfo *slot,
                                      SECKEYPublicKey *pubKey, PRBool isToken);
PK11SymKey *PK11_KeyGen(PK11SlotInfo *slot, CK_MECHANISM_TYPE type,
                        SECItem *param, int keySize, void *wincx);
PK11SymKey *PK11_TokenKeyGen(PK11SlotInfo *slot, CK_MECHANISM_TYPE type,
                             SECItem *param, int keySize, SECItem *keyid,
                             PRBool isToken, void *wincx);
PK11SymKey *PK11_TokenKeyGenWithFlags(PK11SlotInfo *slot,
                                      CK_MECHANISM_TYPE type, SECItem *param,
                                      int keySize, SECItem *keyid, CK_FLAGS opFlags,
                                      PK11AttrFlags attrFlags, void *wincx);
/* Generates a key using the exact template supplied by the caller. The other
 * PK11_[Token]KeyGen mechanisms should be used instead of this one whenever
 * they work because they include/exclude the CKA_VALUE_LEN template value
 * based on the mechanism type as required by many tokens.
 *
 * keyGenType should be PK11_GetKeyGenWithSize(type, <key size>) or it should
 * be equal to type if PK11_GetKeyGenWithSize cannot be used (e.g. because
 * pk11wrap does not know about the mechanisms).
 */
PK11SymKey *PK11_KeyGenWithTemplate(PK11SlotInfo *slot, CK_MECHANISM_TYPE type,
                                    CK_MECHANISM_TYPE keyGenType,
                                    SECItem *param, CK_ATTRIBUTE *attrs,
                                    unsigned int attrsCount, void *wincx);
PK11SymKey *PK11_ListFixedKeysInSlot(PK11SlotInfo *slot, char *nickname,
                                     void *wincx);
PK11SymKey *PK11_GetNextSymKey(PK11SymKey *symKey);
CK_KEY_TYPE PK11_GetSymKeyType(PK11SymKey *key);
CK_OBJECT_HANDLE PK11_GetSymKeyHandle(PK11SymKey *symKey);

/*
 * PK11_SetSymKeyUserData
 *   sets generic user data on keys (usually a pointer to a data structure)
 * that can later be retrieved by PK11_GetSymKeyUserData().
 *    symKey - key where data will be set.
 *    data - data to be set.
 *    freefunc - function used to free the data.
 * Setting user data on symKeys with existing user data already set will cause
 * the existing user data to be freed before the new user data is set.
 * Freeing user data is done by calling the user specified freefunc.
 * If freefunc is NULL, the user data is assumed to be global or static an
 * not freed. Passing NULL for user data to PK11_SetSymKeyUserData has the
 * effect of freeing any existing user data, and clearing the user data
 * pointer. If user data exists when the symKey is finally freed, that
 * data will be freed with freefunc.
 *
 * Applications should only use this function on keys which the application
 * has created directly, as there is only one user data value per key.
 */
void PK11_SetSymKeyUserData(PK11SymKey *symKey, void *data,
                            PK11FreeDataFunc freefunc);
/* PK11_GetSymKeyUserData
 *   retrieves generic user data which was set on a key by
 * PK11_SetSymKeyUserData.
 *    symKey - key with data to be fetched
 *
 * If no data exists, or the data has been cleared, PK11_GetSymKeyUserData
 * will return NULL. Returned data is still owned and managed by the SymKey,
 * the caller should not free the data.
 *
 */
void *PK11_GetSymKeyUserData(PK11SymKey *symKey);

SECStatus PK11_PubWrapSymKey(CK_MECHANISM_TYPE type, SECKEYPublicKey *pubKey,
                             PK11SymKey *symKey, SECItem *wrappedKey);
SECStatus PK11_PubWrapSymKeyWithMechanism(SECKEYPublicKey *pubKey,
                                          CK_MECHANISM_TYPE mechType,
                                          SECItem *param,
                                          PK11SymKey *symKey,
                                          SECItem *wrappedKey);
SECStatus PK11_WrapSymKey(CK_MECHANISM_TYPE type, SECItem *params,
                          PK11SymKey *wrappingKey, PK11SymKey *symKey, SECItem *wrappedKey);
/* move a key to 'slot' optionally set the key attributes according to either
 * operation or the  flags and making the key permanent at the same time.
 * If the key is moved to the same slot, operation and flags values are
 * currently ignored */
PK11SymKey *PK11_MoveSymKey(PK11SlotInfo *slot, CK_ATTRIBUTE_TYPE operation,
                            CK_FLAGS flags, PRBool perm, PK11SymKey *symKey);
/*
 * To do joint operations, we often need two keys in the same slot.
 * Usually the PKCS #11 wrappers handle this correctly (like for PK11_WrapKey),
 * but sometimes the wrappers don't know about mechanism specific keys in
 * the Mechanism params. This function makes sure the two keys are in the
 * same slot by copying one or both of the keys into a common slot. This
 * functions makes sure the slot can handle the target mechanism. If the copy
 * is warranted, this function will prefer to move the movingKey first, then
 * the preferedKey. If the keys are moved, the new keys are returned in
 * newMovingKey and/or newPreferedKey. The application is responsible
 * for freeing those keys one the operation is complete.
 */
SECStatus PK11_SymKeysToSameSlot(CK_MECHANISM_TYPE mech,
                                 CK_ATTRIBUTE_TYPE preferedOperation,
                                 CK_ATTRIBUTE_TYPE movingOperation,
                                 PK11SymKey *preferedKey, PK11SymKey *movingKey,
                                 PK11SymKey **newPreferedKey,
                                 PK11SymKey **newMovingKey);

/*
 * derive a new key from the base key.
 *  PK11_Derive returns a key which can do exactly one operation, and is
 * ephemeral (session key).
 *  PK11_DeriveWithFlags is the same as PK11_Derive, except you can use
 * CKF_ flags to enable more than one operation.
 *  PK11_DeriveWithFlagsPerm is the same as PK11_DeriveWithFlags except you can
 *  (optionally) make the key permanent (token key).
 */
PK11SymKey *PK11_Derive(PK11SymKey *baseKey, CK_MECHANISM_TYPE mechanism,
                        SECItem *param, CK_MECHANISM_TYPE target,
                        CK_ATTRIBUTE_TYPE operation, int keySize);
PK11SymKey *PK11_DeriveWithFlags(PK11SymKey *baseKey,
                                 CK_MECHANISM_TYPE derive, SECItem *param, CK_MECHANISM_TYPE target,
                                 CK_ATTRIBUTE_TYPE operation, int keySize, CK_FLAGS flags);
PK11SymKey *PK11_DeriveWithFlagsPerm(PK11SymKey *baseKey,
                                     CK_MECHANISM_TYPE derive,
                                     SECItem *param, CK_MECHANISM_TYPE target, CK_ATTRIBUTE_TYPE operation,
                                     int keySize, CK_FLAGS flags, PRBool isPerm);
PK11SymKey *
PK11_DeriveWithTemplate(PK11SymKey *baseKey, CK_MECHANISM_TYPE derive,
                        SECItem *param, CK_MECHANISM_TYPE target, CK_ATTRIBUTE_TYPE operation,
                        int keySize, CK_ATTRIBUTE *userAttr, unsigned int numAttrs,
                        PRBool isPerm);

PK11SymKey *PK11_PubDerive(SECKEYPrivateKey *privKey,
                           SECKEYPublicKey *pubKey, PRBool isSender, SECItem *randomA, SECItem *randomB,
                           CK_MECHANISM_TYPE derive, CK_MECHANISM_TYPE target,
                           CK_ATTRIBUTE_TYPE operation, int keySize, void *wincx);
PK11SymKey *PK11_PubDeriveWithKDF(SECKEYPrivateKey *privKey,
                                  SECKEYPublicKey *pubKey, PRBool isSender, SECItem *randomA, SECItem *randomB,
                                  CK_MECHANISM_TYPE derive, CK_MECHANISM_TYPE target,
                                  CK_ATTRIBUTE_TYPE operation, int keySize,
                                  CK_ULONG kdf, SECItem *sharedData, void *wincx);

/*
 * unwrap a new key with a symetric key.
 *  PK11_Unwrap returns a key which can do exactly one operation, and is
 * ephemeral (session key).
 *  PK11_UnwrapWithFlags is the same as PK11_Unwrap, except you can use
 * CKF_ flags to enable more than one operation.
 *  PK11_UnwrapWithFlagsPerm is the same as PK11_UnwrapWithFlags except you can
 *  (optionally) make the key permanent (token key).
 */
PK11SymKey *PK11_UnwrapSymKey(PK11SymKey *key,
                              CK_MECHANISM_TYPE wraptype, SECItem *param, SECItem *wrapppedKey,
                              CK_MECHANISM_TYPE target, CK_ATTRIBUTE_TYPE operation, int keySize);
PK11SymKey *PK11_UnwrapSymKeyWithFlags(PK11SymKey *wrappingKey,
                                       CK_MECHANISM_TYPE wrapType, SECItem *param, SECItem *wrappedKey,
                                       CK_MECHANISM_TYPE target, CK_ATTRIBUTE_TYPE operation, int keySize,
                                       CK_FLAGS flags);
PK11SymKey *PK11_UnwrapSymKeyWithFlagsPerm(PK11SymKey *wrappingKey,
                                           CK_MECHANISM_TYPE wrapType,
                                           SECItem *param, SECItem *wrappedKey,
                                           CK_MECHANISM_TYPE target, CK_ATTRIBUTE_TYPE operation,
                                           int keySize, CK_FLAGS flags, PRBool isPerm);

/*
 * unwrap a new key with a private key.
 *  PK11_PubUnwrap returns a key which can do exactly one operation, and is
 * ephemeral (session key).
 *  PK11_PubUnwrapWithFlagsPerm is the same as PK11_PubUnwrap except you can
 * use * CKF_ flags to enable more than one operation, and optionally make
 * the key permanent (token key).
 */
PK11SymKey *PK11_PubUnwrapSymKey(SECKEYPrivateKey *key, SECItem *wrapppedKey,
                                 CK_MECHANISM_TYPE target, CK_ATTRIBUTE_TYPE operation, int keySize);
PK11SymKey *PK11_PubUnwrapSymKeyWithMechanism(SECKEYPrivateKey *key,
                                              CK_MECHANISM_TYPE mechType,
                                              SECItem *param,
                                              SECItem *wrapppedKey,
                                              CK_MECHANISM_TYPE target,
                                              CK_ATTRIBUTE_TYPE operation,
                                              int keySize);
PK11SymKey *PK11_PubUnwrapSymKeyWithFlagsPerm(SECKEYPrivateKey *wrappingKey,
                                              SECItem *wrappedKey, CK_MECHANISM_TYPE target,
                                              CK_ATTRIBUTE_TYPE operation, int keySize,
                                              CK_FLAGS flags, PRBool isPerm);
PK11SymKey *PK11_FindFixedKey(PK11SlotInfo *slot, CK_MECHANISM_TYPE type,
                              SECItem *keyID, void *wincx);
SECStatus PK11_DeleteTokenPrivateKey(SECKEYPrivateKey *privKey, PRBool force);
SECStatus PK11_DeleteTokenPublicKey(SECKEYPublicKey *pubKey);
SECStatus PK11_DeleteTokenSymKey(PK11SymKey *symKey);
SECStatus PK11_DeleteTokenCertAndKey(CERTCertificate *cert, void *wincx);
SECKEYPrivateKey *PK11_LoadPrivKey(PK11SlotInfo *slot,
                                   SECKEYPrivateKey *privKey, SECKEYPublicKey *pubKey,
                                   PRBool token, PRBool sensitive);
char *PK11_GetSymKeyNickname(PK11SymKey *symKey);
char *PK11_GetPrivateKeyNickname(SECKEYPrivateKey *privKey);
char *PK11_GetPublicKeyNickname(SECKEYPublicKey *pubKey);
SECStatus PK11_SetSymKeyNickname(PK11SymKey *symKey, const char *nickname);
SECStatus PK11_SetPrivateKeyNickname(SECKEYPrivateKey *privKey,
                                     const char *nickname);
SECStatus PK11_SetPublicKeyNickname(SECKEYPublicKey *pubKey,
                                    const char *nickname);

/*
 * Using __PK11_SetCertificateNickname is *DANGEROUS*.
 *
 * The API will update the NSS database, but it *will NOT* update the in-memory data.
 * As a result, after calling this API, there will be INCONSISTENCY between
 * in-memory data and the database.
 *
 * Use of the API should be limited to short-lived tools, which will exit immediately
 * after using this API.
 *
 * If you ignore this warning, your process is TAINTED and will most likely misbehave.
 */
SECStatus __PK11_SetCertificateNickname(CERTCertificate *cert,
                                        const char *nickname);

/* size to hold key in bytes */
unsigned int PK11_GetKeyLength(PK11SymKey *key);
/* size of actual secret parts of key in bits */
/* algid is because RC4 strength is determined by the effective bits as well
 * as the key bits */
unsigned int PK11_GetKeyStrength(PK11SymKey *key, SECAlgorithmID *algid);
SECStatus PK11_ExtractKeyValue(PK11SymKey *symKey);
SECItem *PK11_GetKeyData(PK11SymKey *symKey);
PK11SlotInfo *PK11_GetSlotFromKey(PK11SymKey *symKey);
void *PK11_GetWindow(PK11SymKey *symKey);

/*
 * Explicitly set the key usage for the generated private key.
 *
 * This allows us to specify single use EC and RSA keys whose usage
 * can be regulated by the underlying token.
 *
 * The underlying key usage is set using opFlags. opFlagsMask specifies
 * which operations are specified by opFlags. For instance to turn encrypt
 * on and signing off, opFlags would be CKF_ENCRYPT|CKF_DECRYPT and
 * opFlagsMask would be CKF_ENCRYPT|CKF_DECRYPT|CKF_SIGN|CKF_VERIFY. You
 * need to specify both the public and private key flags,
 * PK11_GenerateKeyPairWithOpFlags will sort out the correct flag to the
 * correct key type. Flags not specified in opFlagMask will be defaulted
 * according to mechanism type and token capabilities.
 */
SECKEYPrivateKey *PK11_GenerateKeyPairWithOpFlags(PK11SlotInfo *slot,
                                                  CK_MECHANISM_TYPE type, void *param, SECKEYPublicKey **pubk,
                                                  PK11AttrFlags attrFlags, CK_FLAGS opFlags, CK_FLAGS opFlagsMask,
                                                  void *wincx);
/*
 * The attrFlags is the logical OR of the PK11_ATTR_XXX bitflags.
 * These flags apply to the private key.  The PK11_ATTR_TOKEN,
 * PK11_ATTR_SESSION, PK11_ATTR_MODIFIABLE, and PK11_ATTR_UNMODIFIABLE
 * flags also apply to the public key.
 */
SECKEYPrivateKey *PK11_GenerateKeyPairWithFlags(PK11SlotInfo *slot,
                                                CK_MECHANISM_TYPE type, void *param, SECKEYPublicKey **pubk,
                                                PK11AttrFlags attrFlags, void *wincx);
SECKEYPrivateKey *PK11_GenerateKeyPair(PK11SlotInfo *slot,
                                       CK_MECHANISM_TYPE type, void *param, SECKEYPublicKey **pubk,
                                       PRBool isPerm, PRBool isSensitive, void *wincx);
SECKEYPrivateKey *PK11_FindPrivateKeyFromCert(PK11SlotInfo *slot,
                                              CERTCertificate *cert, void *wincx);
SECKEYPrivateKey *PK11_FindKeyByAnyCert(CERTCertificate *cert, void *wincx);
SECKEYPrivateKey *PK11_FindKeyByKeyID(PK11SlotInfo *slot, SECItem *keyID,
                                      void *wincx);
int PK11_GetPrivateModulusLen(SECKEYPrivateKey *key);

SECStatus PK11_Decrypt(PK11SymKey *symkey,
                       CK_MECHANISM_TYPE mechanism, SECItem *param,
                       unsigned char *out, unsigned int *outLen,
                       unsigned int maxLen,
                       const unsigned char *enc, unsigned int encLen);
SECStatus PK11_Encrypt(PK11SymKey *symKey,
                       CK_MECHANISM_TYPE mechanism, SECItem *param,
                       unsigned char *out, unsigned int *outLen,
                       unsigned int maxLen,
                       const unsigned char *data, unsigned int dataLen);

/* note: despite the name, this function takes a private key. */
SECStatus PK11_PubDecryptRaw(SECKEYPrivateKey *key,
                             unsigned char *data, unsigned *outLen,
                             unsigned int maxLen,
                             const unsigned char *enc, unsigned encLen);
#define PK11_PrivDecryptRaw PK11_PubDecryptRaw
/* The encrypt function that complements the above decrypt function. */
SECStatus PK11_PubEncryptRaw(SECKEYPublicKey *key,
                             unsigned char *enc,
                             const unsigned char *data, unsigned dataLen,
                             void *wincx);

SECStatus PK11_PrivDecryptPKCS1(SECKEYPrivateKey *key,
                                unsigned char *data, unsigned *outLen,
                                unsigned int maxLen,
                                const unsigned char *enc, unsigned encLen);
/* The encrypt function that complements the above decrypt function. */
SECStatus PK11_PubEncryptPKCS1(SECKEYPublicKey *key,
                               unsigned char *enc,
                               const unsigned char *data, unsigned dataLen,
                               void *wincx);

SECStatus PK11_PrivDecrypt(SECKEYPrivateKey *key,
                           CK_MECHANISM_TYPE mechanism, SECItem *param,
                           unsigned char *out, unsigned int *outLen,
                           unsigned int maxLen,
                           const unsigned char *enc, unsigned int encLen);
SECStatus PK11_PubEncrypt(SECKEYPublicKey *key,
                          CK_MECHANISM_TYPE mechanism, SECItem *param,
                          unsigned char *out, unsigned int *outLen,
                          unsigned int maxLen,
                          const unsigned char *data, unsigned int dataLen,
                          void *wincx);

SECStatus PK11_ImportPrivateKeyInfo(PK11SlotInfo *slot,
                                    SECKEYPrivateKeyInfo *pki, SECItem *nickname,
                                    SECItem *publicValue, PRBool isPerm, PRBool isPrivate,
                                    unsigned int usage, void *wincx);
SECStatus PK11_ImportPrivateKeyInfoAndReturnKey(PK11SlotInfo *slot,
                                                SECKEYPrivateKeyInfo *pki, SECItem *nickname,
                                                SECItem *publicValue, PRBool isPerm, PRBool isPrivate,
                                                unsigned int usage, SECKEYPrivateKey **privk, void *wincx);
SECStatus PK11_ImportDERPrivateKeyInfo(PK11SlotInfo *slot,
                                       SECItem *derPKI, SECItem *nickname,
                                       SECItem *publicValue, PRBool isPerm, PRBool isPrivate,
                                       unsigned int usage, void *wincx);
SECStatus PK11_ImportDERPrivateKeyInfoAndReturnKey(PK11SlotInfo *slot,
                                                   SECItem *derPKI, SECItem *nickname,
                                                   SECItem *publicValue, PRBool isPerm, PRBool isPrivate,
                                                   unsigned int usage, SECKEYPrivateKey **privk, void *wincx);
SECStatus PK11_ImportEncryptedPrivateKeyInfo(PK11SlotInfo *slot,
                                             SECKEYEncryptedPrivateKeyInfo *epki, SECItem *pwitem,
                                             SECItem *nickname, SECItem *publicValue, PRBool isPerm,
                                             PRBool isPrivate, KeyType type,
                                             unsigned int usage, void *wincx);
SECStatus PK11_ImportEncryptedPrivateKeyInfoAndReturnKey(PK11SlotInfo *slot,
                                                         SECKEYEncryptedPrivateKeyInfo *epki, SECItem *pwitem,
                                                         SECItem *nickname, SECItem *publicValue, PRBool isPerm,
                                                         PRBool isPrivate, KeyType type,
                                                         unsigned int usage, SECKEYPrivateKey **privk, void *wincx);
SECItem *PK11_ExportDERPrivateKeyInfo(SECKEYPrivateKey *pk, void *wincx);
SECKEYPrivateKeyInfo *PK11_ExportPrivKeyInfo(
    SECKEYPrivateKey *pk, void *wincx);
SECKEYPrivateKeyInfo *PK11_ExportPrivateKeyInfo(
    CERTCertificate *cert, void *wincx);
SECKEYEncryptedPrivateKeyInfo *PK11_ExportEncryptedPrivKeyInfo(
    PK11SlotInfo *slot, SECOidTag algTag, SECItem *pwitem,
    SECKEYPrivateKey *pk, int iteration, void *wincx);
SECKEYEncryptedPrivateKeyInfo *PK11_ExportEncryptedPrivateKeyInfo(
    PK11SlotInfo *slot, SECOidTag algTag, SECItem *pwitem,
    CERTCertificate *cert, int iteration, void *wincx);
SECKEYPrivateKey *PK11_FindKeyByDERCert(PK11SlotInfo *slot,
                                        CERTCertificate *cert, void *wincx);
SECKEYPublicKey *PK11_MakeKEAPubKey(unsigned char *data, int length);
SECStatus PK11_DigestKey(PK11Context *context, PK11SymKey *key);
PRBool PK11_VerifyKeyOK(PK11SymKey *key);
SECKEYPrivateKey *PK11_UnwrapPrivKey(PK11SlotInfo *slot,
                                     PK11SymKey *wrappingKey, CK_MECHANISM_TYPE wrapType,
                                     SECItem *param, SECItem *wrappedKey, SECItem *label,
                                     SECItem *publicValue, PRBool token, PRBool sensitive,
                                     CK_KEY_TYPE keyType, CK_ATTRIBUTE_TYPE *usage, int usageCount,
                                     void *wincx);
SECStatus PK11_WrapPrivKey(PK11SlotInfo *slot, PK11SymKey *wrappingKey,
                           SECKEYPrivateKey *privKey, CK_MECHANISM_TYPE wrapType,
                           SECItem *param, SECItem *wrappedKey, void *wincx);
/*
 * The caller of PK11_DEREncodePublicKey should free the returned SECItem with
 * a SECITEM_FreeItem(..., PR_TRUE) call.
 */
SECItem *PK11_DEREncodePublicKey(const SECKEYPublicKey *pubk);
PK11SymKey *PK11_CopySymKeyForSigning(PK11SymKey *originalKey,
                                      CK_MECHANISM_TYPE mech);
SECKEYPrivateKeyList *PK11_ListPrivKeysInSlot(PK11SlotInfo *slot,
                                              char *nickname, void *wincx);
SECKEYPublicKeyList *PK11_ListPublicKeysInSlot(PK11SlotInfo *slot,
                                               char *nickname);
SECKEYPQGParams *PK11_GetPQGParamsFromPrivateKey(SECKEYPrivateKey *privKey);
/* deprecated */
SECKEYPrivateKeyList *PK11_ListPrivateKeysInSlot(PK11SlotInfo *slot);

PK11SymKey *PK11_ConvertSessionSymKeyToTokenSymKey(PK11SymKey *symk,
                                                   void *wincx);
SECKEYPrivateKey *PK11_ConvertSessionPrivKeyToTokenPrivKey(
    SECKEYPrivateKey *privk, void *wincx);
SECKEYPrivateKey *PK11_CopyTokenPrivKeyToSessionPrivKey(PK11SlotInfo *destSlot,
                                                        SECKEYPrivateKey *privKey);

/**********************************************************************
 *                   Certs
 **********************************************************************/
SECItem *PK11_MakeIDFromPubKey(SECItem *pubKeyData);
SECStatus PK11_TraverseSlotCerts(
    SECStatus (*callback)(CERTCertificate *, SECItem *, void *),
    void *arg, void *wincx);
CERTCertificate *PK11_FindCertFromNickname(const char *nickname, void *wincx);
CERTCertificate *PK11_FindCertFromURI(const char *uri, void *wincx);
CERTCertList *PK11_FindCertsFromURI(const char *uri, void *wincx);
CERTCertList *PK11_FindCertsFromEmailAddress(const char *email, void *wincx);
CERTCertList *PK11_FindCertsFromNickname(const char *nickname, void *wincx);
CERTCertificate *PK11_GetCertFromPrivateKey(SECKEYPrivateKey *privKey);
SECStatus PK11_ImportCert(PK11SlotInfo *slot, CERTCertificate *cert,
                          CK_OBJECT_HANDLE key, const char *nickname,
                          PRBool includeTrust);
SECStatus PK11_ImportDERCert(PK11SlotInfo *slot, SECItem *derCert,
                             CK_OBJECT_HANDLE key, char *nickname, PRBool includeTrust);
PK11SlotInfo *PK11_ImportCertForKey(CERTCertificate *cert,
                                    const char *nickname, void *wincx);
PK11SlotInfo *PK11_ImportDERCertForKey(SECItem *derCert, char *nickname,
                                       void *wincx);
PK11SlotInfo *PK11_KeyForCertExists(CERTCertificate *cert,
                                    CK_OBJECT_HANDLE *keyPtr, void *wincx);
PK11SlotInfo *PK11_KeyForDERCertExists(SECItem *derCert,
                                       CK_OBJECT_HANDLE *keyPtr, void *wincx);
CERTCertificate *PK11_FindCertByIssuerAndSN(PK11SlotInfo **slot,
                                            CERTIssuerAndSN *sn, void *wincx);
CERTCertificate *PK11_FindCertAndKeyByRecipientList(PK11SlotInfo **slot,
                                                    SEC_PKCS7RecipientInfo **array, SEC_PKCS7RecipientInfo **rip,
                                                    SECKEYPrivateKey **privKey, void *wincx);
int PK11_FindCertAndKeyByRecipientListNew(NSSCMSRecipient **recipientlist,
                                          void *wincx);
SECStatus PK11_TraverseCertsForSubjectInSlot(CERTCertificate *cert,
                                             PK11SlotInfo *slot, SECStatus (*callback)(CERTCertificate *, void *),
                                             void *arg);
CERTCertificate *PK11_FindCertFromDERCert(PK11SlotInfo *slot,
                                          CERTCertificate *cert, void *wincx);
CERTCertificate *PK11_FindCertFromDERCertItem(PK11SlotInfo *slot,
                                              const SECItem *derCert, void *wincx);
SECStatus PK11_ImportCertForKeyToSlot(PK11SlotInfo *slot, CERTCertificate *cert,
                                      char *nickname, PRBool addUsage,
                                      void *wincx);
CERTCertificate *PK11_FindBestKEAMatch(CERTCertificate *serverCert, void *wincx);
PRBool PK11_FortezzaHasKEA(CERTCertificate *cert);
CK_OBJECT_HANDLE PK11_FindEncodedCertInSlot(PK11SlotInfo *slot, SECItem *derCert, void *wincx);
CK_OBJECT_HANDLE PK11_FindCertInSlot(PK11SlotInfo *slot, CERTCertificate *cert,
                                     void *wincx);
SECStatus PK11_TraverseCertsForNicknameInSlot(SECItem *nickname,
                                              PK11SlotInfo *slot, SECStatus (*callback)(CERTCertificate *, void *),
                                              void *arg);
CERTCertList *PK11_ListCerts(PK11CertListType type, void *pwarg);
CERTCertList *PK11_ListCertsInSlot(PK11SlotInfo *slot);
CERTSignedCrl *PK11_ImportCRL(PK11SlotInfo *slot, SECItem *derCRL, char *url,
                              int type, void *wincx, PRInt32 importOptions, PLArenaPool *arena, PRInt32 decodeOptions);
CK_BBOOL PK11_HasAttributeSet(PK11SlotInfo *slot,
                              CK_OBJECT_HANDLE id,
                              CK_ATTRIBUTE_TYPE type,
                              PRBool haslock /* must be set to PR_FALSE */);

/**********************************************************************
 *                   Hybrid Public Key Encryption  (draft-05)
 **********************************************************************/
/*
 * NOTE: All HPKE functions will fail with SEC_ERROR_INVALID_ALGORITHM
 * unless NSS is compiled with NSS_ENABLE_DRAFT_HPKE while spec (and
 * implementation) is in draft. The eventual RFC number is an input to
 * the key schedule, so applications opting into this MUST be prepared for
 * outputs to change when the implementation is updated or finalized. */

/* Some of the various HPKE arguments would ideally be const, but the
 * underlying PK11 functions take them as non-const. To avoid lying to
 * the application with a cast, this idiosyncrasy is exposed. */
SECStatus PK11_HPKE_ValidateParameters(HpkeKemId kemId, HpkeKdfId kdfId, HpkeAeadId aeadId);
HpkeContext *PK11_HPKE_NewContext(HpkeKemId kemId, HpkeKdfId kdfId, HpkeAeadId aeadId,
                                  PK11SymKey *psk, const SECItem *pskId);
SECStatus PK11_HPKE_Deserialize(const HpkeContext *cx, const PRUint8 *enc,
                                unsigned int encLen, SECKEYPublicKey **outPubKey);
void PK11_HPKE_DestroyContext(HpkeContext *cx, PRBool freeit);
const SECItem *PK11_HPKE_GetEncapPubKey(const HpkeContext *cx);
SECStatus PK11_HPKE_ExportSecret(const HpkeContext *cx, const SECItem *info, unsigned int L,
                                 PK11SymKey **outKey);
SECStatus PK11_HPKE_Open(HpkeContext *cx, const SECItem *aad, const SECItem *ct, SECItem **outPt);
SECStatus PK11_HPKE_Seal(HpkeContext *cx, const SECItem *aad, const SECItem *pt, SECItem **outCt);
SECStatus PK11_HPKE_Serialize(const SECKEYPublicKey *pk, PRUint8 *buf, unsigned int *len, unsigned int maxLen);
SECStatus PK11_HPKE_SetupS(HpkeContext *cx, const SECKEYPublicKey *pkE, SECKEYPrivateKey *skE,
                           SECKEYPublicKey *pkR, const SECItem *info);
SECStatus PK11_HPKE_SetupR(HpkeContext *cx, const SECKEYPublicKey *pkR, SECKEYPrivateKey *skR,
                           const SECItem *enc, const SECItem *info);

/**********************************************************************
 *                   Sign/Verify
 **********************************************************************/

/*
 * Return the length in bytes of a signature generated with the
 * private key.
 *
 * Return 0 or -1 on failure.  (XXX Should we fix it to always return
 * -1 on failure?)
 */
int PK11_SignatureLen(SECKEYPrivateKey *key);
PK11SlotInfo *PK11_GetSlotFromPrivateKey(SECKEYPrivateKey *key);
SECStatus PK11_Sign(SECKEYPrivateKey *key, SECItem *sig,
                    const SECItem *hash);
SECStatus PK11_SignWithMechanism(SECKEYPrivateKey *key,
                                 CK_MECHANISM_TYPE mechanism,
                                 const SECItem *param, SECItem *sig,
                                 const SECItem *hash);
SECStatus PK11_SignWithSymKey(PK11SymKey *symKey, CK_MECHANISM_TYPE mechanism,
                              SECItem *param, SECItem *sig, const SECItem *data);
SECStatus PK11_VerifyRecover(SECKEYPublicKey *key, const SECItem *sig,
                             SECItem *dsig, void *wincx);
SECStatus PK11_Verify(SECKEYPublicKey *key, const SECItem *sig,
                      const SECItem *hash, void *wincx);
SECStatus PK11_VerifyWithMechanism(SECKEYPublicKey *key,
                                   CK_MECHANISM_TYPE mechanism,
                                   const SECItem *param, const SECItem *sig,
                                   const SECItem *hash, void *wincx);

/**********************************************************************
 *                   Crypto Contexts
 **********************************************************************/
void PK11_DestroyContext(PK11Context *context, PRBool freeit);
PK11Context *PK11_CreateContextBySymKey(CK_MECHANISM_TYPE type,
                                        CK_ATTRIBUTE_TYPE operation, PK11SymKey *symKey, SECItem *param);
PK11Context *PK11_CreateDigestContext(SECOidTag hashAlg);
PK11Context *PK11_CloneContext(PK11Context *old);
SECStatus PK11_DigestBegin(PK11Context *cx);
/*
 * The output buffer 'out' must be big enough to hold the output of
 * the hash algorithm 'hashAlg'.
 */
SECStatus PK11_HashBuf(SECOidTag hashAlg, unsigned char *out,
                       const unsigned char *in, PRInt32 len);
SECStatus PK11_DigestOp(PK11Context *context, const unsigned char *in,
                        unsigned len);
SECStatus PK11_CipherOp(PK11Context *context, unsigned char *out, int *outlen,
                        int maxout, const unsigned char *in, int inlen);
/* application builds the mechanism specific params */
SECStatus PK11_AEADRawOp(PK11Context *context, void *params, int paramslen,
                         const unsigned char *aad, int aadlen,
                         unsigned char *out, int *outlen,
                         int maxout, const unsigned char *in, int inlen);
/* NSS builds the mechanism specific params */
SECStatus PK11_AEADOp(PK11Context *context, CK_GENERATOR_FUNCTION ivGen,
                      int fixedbits, unsigned char *iv, int ivlen,
                      const unsigned char *aad, int aadlen,
                      unsigned char *out, int *outlen,
                      int maxout, unsigned char *tag, int taglen,
                      const unsigned char *in, int inlen);

SECStatus PK11_Finalize(PK11Context *context);
SECStatus PK11_DigestFinal(PK11Context *context, unsigned char *data,
                           unsigned int *outLen, unsigned int length);
#define PK11_CipherFinal PK11_DigestFinal
SECStatus PK11_SaveContext(PK11Context *cx, unsigned char *save,
                           int *len, int saveLength);

/* Save the context's state, with possible allocation.
 * The caller may supply an already allocated buffer in preAllocBuf,
 * with length pabLen.  If the buffer is large enough for the context's
 * state, it will receive the state.
 * If the buffer is not large enough (or NULL), then a new buffer will
 * be allocated with PORT_Alloc.
 * In either case, the state will be returned as a buffer, and the length
 * of the state will be given in *stateLen.
 */
unsigned char *
PK11_SaveContextAlloc(PK11Context *cx,
                      unsigned char *preAllocBuf, unsigned int pabLen,
                      unsigned int *stateLen);

SECStatus PK11_RestoreContext(PK11Context *cx, unsigned char *save, int len);
SECStatus PK11_GenerateFortezzaIV(PK11SymKey *symKey, unsigned char *iv, int len);
void PK11_SetFortezzaHack(PK11SymKey *symKey);

/**********************************************************************
 *                   PBE functions
 **********************************************************************/

/* This function creates PBE parameters from the given inputs.  The result
 * can be used to create a password integrity key for PKCS#12, by sending
 * the return value to PK11_KeyGen along with the appropriate mechanism.
 */
SECItem *
PK11_CreatePBEParams(SECItem *salt, SECItem *pwd, unsigned int iterations);

/* free params created above (can be called after keygen is done */
void PK11_DestroyPBEParams(SECItem *params);

SECAlgorithmID *
PK11_CreatePBEAlgorithmID(SECOidTag algorithm, int iteration, SECItem *salt);

/* use to create PKCS5 V2 algorithms with finder control than that provided
 * by PK11_CreatePBEAlgorithmID. */
SECAlgorithmID *
PK11_CreatePBEV2AlgorithmID(SECOidTag pbeAlgTag, SECOidTag cipherAlgTag,
                            SECOidTag prfAlgTag, int keyLength, int iteration,
                            SECItem *salt);
PK11SymKey *
PK11_PBEKeyGen(PK11SlotInfo *slot, SECAlgorithmID *algid, SECItem *pwitem,
               PRBool faulty3DES, void *wincx);

/* warning: cannot work with PKCS 5 v2 use PK11_PBEKeyGen instead */
PK11SymKey *
PK11_RawPBEKeyGen(PK11SlotInfo *slot, CK_MECHANISM_TYPE type, SECItem *params,
                  SECItem *pwitem, PRBool faulty3DES, void *wincx);
SECItem *
PK11_GetPBEIV(SECAlgorithmID *algid, SECItem *pwitem);
/*
 * Get the Mechanism and parameter of the base encryption or mac scheme from
 * a PBE algorithm ID.
 *  Caller is responsible for freeing the return parameter (param).
 */
CK_MECHANISM_TYPE
PK11_GetPBECryptoMechanism(SECAlgorithmID *algid,
                           SECItem **param, SECItem *pwd);

/**********************************************************************
 * Functions to manage secmod flags
 **********************************************************************/
const PK11DefaultArrayEntry *PK11_GetDefaultArray(int *size);
SECStatus PK11_UpdateSlotAttribute(PK11SlotInfo *slot,
                                   const PK11DefaultArrayEntry *entry,
                                   PRBool add);

/**********************************************************************
 * Functions to look at PKCS #11 dependent data
 **********************************************************************/
PK11GenericObject *PK11_FindGenericObjects(PK11SlotInfo *slot,
                                           CK_OBJECT_CLASS objClass);
PK11GenericObject *PK11_GetNextGenericObject(PK11GenericObject *object);
PK11GenericObject *PK11_GetPrevGenericObject(PK11GenericObject *object);
SECStatus PK11_UnlinkGenericObject(PK11GenericObject *object);
SECStatus PK11_LinkGenericObject(PK11GenericObject *list,
                                 PK11GenericObject *object);
SECStatus PK11_DestroyGenericObjects(PK11GenericObject *object);
SECStatus PK11_DestroyGenericObject(PK11GenericObject *object);
PK11GenericObject *PK11_CreateManagedGenericObject(PK11SlotInfo *slot,
                                                   const CK_ATTRIBUTE *pTemplate,
                                                   int count, PRBool token);
/* deprecated */
PK11GenericObject *PK11_CreateGenericObject(PK11SlotInfo *slot,
                                            const CK_ATTRIBUTE *pTemplate,
                                            int count, PRBool token);

/*
 * PK11_ReadRawAttribute and PK11_WriteRawAttribute are generic
 * functions to read and modify the actual PKCS #11 attributes of
 * the underlying pkcs #11 object.
 *
 * object is a pointer to an NSS object that represents the underlying
 *  PKCS #11 object. It's type must match the type of PK11ObjectType
 *  as follows:
 *
 *     type                           object
 *   PK11_TypeGeneric            PK11GenericObject *
 *   PK11_TypePrivKey            SECKEYPrivateKey *
 *   PK11_TypePubKey             SECKEYPublicKey *
 *   PK11_TypeSymKey             PK11SymKey *
 *
 *  All other types are considered invalid. If type does not match the object
 *  passed, unpredictable results will occur.
 *
 * PK11_ReadRawAttribute allocates the buffer for returning the attribute
 * value.  The caller of PK11_ReadRawAttribute should free the data buffer
 * pointed to by item using a SECITEM_FreeItem(item, PR_FALSE) or
 * PORT_Free(item->data) call.
 */
SECStatus PK11_ReadRawAttribute(PK11ObjectType type, void *object,
                                CK_ATTRIBUTE_TYPE attr, SECItem *item);
SECStatus PK11_ReadRawAttributes(PLArenaPool *arena, PK11ObjectType type, void *object,
                                 CK_ATTRIBUTE *pTemplate, unsigned int count);
SECStatus PK11_WriteRawAttribute(PK11ObjectType type, void *object,
                                 CK_ATTRIBUTE_TYPE attr, SECItem *item);
/* get the PKCS #11 handle and slot for a generic object */
CK_OBJECT_HANDLE PK11_GetObjectHandle(PK11ObjectType objType, void *objSpec,
                                      PK11SlotInfo **slotp);

/*
 * PK11_GetAllSlotsForCert returns all the slots that a given certificate
 * exists on, since it's possible for a cert to exist on more than one
 * PKCS#11 token.
 */
PK11SlotList *
PK11_GetAllSlotsForCert(CERTCertificate *cert, void *arg);

/*
 * Finds all certificates on the given slot with the given subject distinguished
 * name and returns them as DER bytes. If no such certificates can be found,
 * returns SECSuccess and sets *results to NULL. If a failure is encountered
 * while fetching any of the matching certificates, SECFailure is returned and
 * *results will be NULL.
 */
SECStatus
PK11_FindRawCertsWithSubject(PK11SlotInfo *slot, SECItem *derSubject,
                             CERTCertificateList **results);

/*
 * Finds and returns all certificates with a public key that matches the given
 * private key. May return an empty list if no certificates match. Returns NULL
 * if a failure is encountered.
 */
CERTCertList *
PK11_GetCertsMatchingPrivateKey(SECKEYPrivateKey *privKey);

/**********************************************************************
 * New functions which are already deprecated....
 **********************************************************************/
SECItem *
PK11_GetLowLevelKeyIDForCert(PK11SlotInfo *slot,
                             CERTCertificate *cert, void *pwarg);
SECItem *
PK11_GetLowLevelKeyIDForPrivateKey(SECKEYPrivateKey *key);

PRBool SECMOD_HasRootCerts(void);

/**********************************************************************
 * Other Utilities
 **********************************************************************/
/* 
 * Get the state of the system FIPS mode -
 *  NSS uses this to force FIPS mode if the system bit is on. This returns
 *  the system state independent of the database state and can be called
 *  before NSS initializes.
 */
int SECMOD_GetSystemFIPSEnabled(void);

SEC_END_PROTOS

#endif
