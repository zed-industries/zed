/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#ifndef _SECMOD_H_
#define _SECMOD_H_
#include "seccomon.h"
#include "secmodt.h"
#include "prinrval.h"

/* These mechanisms flags are visible to all other libraries. */
/* They must be converted to internal SECMOD_*_FLAG */
/* if used inside the functions of the security library */
#define PUBLIC_MECH_RSA_FLAG 0x00000001ul
#define PUBLIC_MECH_DSA_FLAG 0x00000002ul
#define PUBLIC_MECH_RC2_FLAG 0x00000004ul
#define PUBLIC_MECH_RC4_FLAG 0x00000008ul
#define PUBLIC_MECH_DES_FLAG 0x00000010ul
#define PUBLIC_MECH_DH_FLAG 0x00000020ul
#define PUBLIC_MECH_FORTEZZA_FLAG 0x00000040ul
#define PUBLIC_MECH_RC5_FLAG 0x00000080ul
#define PUBLIC_MECH_SHA1_FLAG 0x00000100ul
#define PUBLIC_MECH_MD5_FLAG 0x00000200ul
#define PUBLIC_MECH_MD2_FLAG 0x00000400ul
#define PUBLIC_MECH_SSL_FLAG 0x00000800ul
#define PUBLIC_MECH_TLS_FLAG 0x00001000ul
#define PUBLIC_MECH_AES_FLAG 0x00002000ul
#define PUBLIC_MECH_SHA256_FLAG 0x00004000ul
#define PUBLIC_MECH_SHA512_FLAG 0x00008000ul
#define PUBLIC_MECH_CAMELLIA_FLAG 0x00010000ul
#define PUBLIC_MECH_SEED_FLAG 0x00020000ul
#define PUBLIC_MECH_ECC_FLAG 0x00040000ul

#define PUBLIC_MECH_RANDOM_FLAG 0x08000000ul
#define PUBLIC_MECH_FRIENDLY_FLAG 0x10000000ul
#define PUBLIC_OWN_PW_DEFAULTS 0X20000000ul
#define PUBLIC_DISABLE_FLAG 0x40000000ul

/* warning: reserved means reserved */
#define PUBLIC_MECH_RESERVED_FLAGS 0x87FF0000ul

/* These cipher flags are visible to all other libraries, */
/* But they must be converted before used in functions */
/* withing the security module */
#define PUBLIC_CIPHER_FORTEZZA_FLAG 0x00000001ul

/* warning: reserved means reserved */
#define PUBLIC_CIPHER_RESERVED_FLAGS 0xFFFFFFFEul

SEC_BEGIN_PROTOS

/*
 * the following functions are going to be deprecated in NSS 4.0 in
 * favor of the new stan functions.
 */

/* Initialization */
extern SECMODModule *SECMOD_LoadModule(char *moduleSpec, SECMODModule *parent,
                                       PRBool recurse);

extern SECMODModule *SECMOD_LoadUserModule(char *moduleSpec, SECMODModule *parent,
                                           PRBool recurse);

SECStatus SECMOD_UnloadUserModule(SECMODModule *mod);

SECMODModule *SECMOD_CreateModule(const char *lib, const char *name,
                                  const char *param, const char *nss);
SECMODModule *SECMOD_CreateModuleEx(const char *lib, const char *name,
                                    const char *param, const char *nss,
                                    const char *config);
/*
 * After a fork(), PKCS #11 says we need to call C_Initialize again in
 * the child before we can use the module. This function causes this
 * reinitialization.
 * NOTE: Any outstanding handles will become invalid, which means your
 * keys and contexts will fail, but new ones can be created.
 *
 * Setting 'force' to true means to do the reinitialization even if the
 * PKCS #11 module does not seem to need it. This allows software modules
 * which ignore fork to preserve their keys across the fork().
 */
SECStatus SECMOD_RestartModules(PRBool force);

/* Module Management */
char **SECMOD_GetModuleSpecList(SECMODModule *module);
SECStatus SECMOD_FreeModuleSpecList(SECMODModule *module, char **moduleSpecList);

/* protoypes */
/* Get a list of active PKCS #11 modules */
extern SECMODModuleList *SECMOD_GetDefaultModuleList(void);
/* Get a list of defined but not loaded PKCS #11 modules */
extern SECMODModuleList *SECMOD_GetDeadModuleList(void);
/* Get a list of Modules which define PKCS #11 modules to load */
extern SECMODModuleList *SECMOD_GetDBModuleList(void);

/* lock to protect all three module lists above */
extern SECMODListLock *SECMOD_GetDefaultModuleListLock(void);

extern SECStatus SECMOD_UpdateModule(SECMODModule *module);

/* lock management */
extern void SECMOD_GetReadLock(SECMODListLock *);
extern void SECMOD_ReleaseReadLock(SECMODListLock *);

/* Operate on modules by name */
extern SECMODModule *SECMOD_FindModule(const char *name);
extern SECStatus SECMOD_DeleteModule(const char *name, int *type);
extern SECStatus SECMOD_DeleteModuleEx(const char *name,
                                       SECMODModule *mod,
                                       int *type,
                                       PRBool permdb);
extern SECStatus SECMOD_DeleteInternalModule(const char *name);
extern PRBool SECMOD_CanDeleteInternalModule(void);
extern SECStatus SECMOD_AddNewModule(const char *moduleName,
                                     const char *dllPath,
                                     unsigned long defaultMechanismFlags,
                                     unsigned long cipherEnableFlags);
extern SECStatus SECMOD_AddNewModuleEx(const char *moduleName,
                                       const char *dllPath,
                                       unsigned long defaultMechanismFlags,
                                       unsigned long cipherEnableFlags,
                                       char *modparms,
                                       char *nssparms);

/* database/memory management */
extern SECMODModule *SECMOD_GetInternalModule(void);
extern SECMODModule *SECMOD_ReferenceModule(SECMODModule *module);
extern void SECMOD_DestroyModule(SECMODModule *module);
extern PK11SlotInfo *SECMOD_LookupSlot(SECMODModuleID module,
                                       unsigned long slotID);
extern PK11SlotInfo *SECMOD_FindSlot(SECMODModule *module, const char *name);

/* Funtion reports true if at least one of the modules */
/* of modType has been installed */
PRBool SECMOD_IsModulePresent(unsigned long int pubCipherEnableFlags);

/* accessors */
PRBool SECMOD_GetSkipFirstFlag(SECMODModule *mod);
PRBool SECMOD_GetDefaultModDBFlag(SECMODModule *mod);

/* Functions used to convert between internal & public representation
 * of Mechanism Flags and Cipher Enable Flags */
extern unsigned long SECMOD_PubMechFlagstoInternal(unsigned long publicFlags);
extern unsigned long SECMOD_InternaltoPubMechFlags(unsigned long internalFlags);
extern unsigned long SECMOD_PubCipherFlagstoInternal(unsigned long publicFlags);

PRBool SECMOD_HasRemovableSlots(SECMODModule *mod);
PK11SlotInfo *SECMOD_WaitForAnyTokenEvent(SECMODModule *mod,
                                          unsigned long flags, PRIntervalTime latency);
/*
 * Warning: the SECMOD_CancelWait function is highly destructive, potentially
 * finalizing  the module 'mod' (causing inprogress operations to fail,
 * and session key material to disappear). It should only be called when
 * shutting down  the module.
 */
SECStatus SECMOD_CancelWait(SECMODModule *mod);
/*
 * check to see if the module has added new slots. PKCS 11 v2.20 allows for
 * modules to add new slots, but never remove them. Slots not be added between
 * a call to C_GetSlotLlist(Flag, NULL, &count) and the corresponding
 * C_GetSlotList(flag, &data, &count) so that the array doesn't accidently
 * grow on the caller. It is permissible for the slots to increase between
 * corresponding calls with NULL to get the size.
 */
SECStatus SECMOD_UpdateSlotList(SECMODModule *mod);
SEC_END_PROTOS

#endif
