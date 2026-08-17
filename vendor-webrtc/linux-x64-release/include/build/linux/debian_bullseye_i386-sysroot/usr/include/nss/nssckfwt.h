/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef NSSCKFWT_H
#define NSSCKFWT_H

/*
 * nssckfwt.h
 *
 * This file declares the public types used by the NSS Cryptoki Framework.
 */

/*
 * NSSCKFWInstance
 *
 */

struct NSSCKFWInstanceStr;
typedef struct NSSCKFWInstanceStr NSSCKFWInstance;

/*
 * NSSCKFWSlot
 *
 */

struct NSSCKFWSlotStr;
typedef struct NSSCKFWSlotStr NSSCKFWSlot;

/*
 * NSSCKFWToken
 *
 */

struct NSSCKFWTokenStr;
typedef struct NSSCKFWTokenStr NSSCKFWToken;

/*
 * NSSCKFWMechanism
 *
 */

struct NSSCKFWMechanismStr;
typedef struct NSSCKFWMechanismStr NSSCKFWMechanism;

/*
 * NSSCKFWCryptoOperation
 *
 */

struct NSSCKFWCryptoOperationStr;
typedef struct NSSCKFWCryptoOperationStr NSSCKFWCryptoOperation;

/*
 * NSSCKFWSession
 *
 */

struct NSSCKFWSessionStr;
typedef struct NSSCKFWSessionStr NSSCKFWSession;

/*
 * NSSCKFWObject
 *
 */

struct NSSCKFWObjectStr;
typedef struct NSSCKFWObjectStr NSSCKFWObject;

/*
 * NSSCKFWFindObjects
 *
 */

struct NSSCKFWFindObjectsStr;
typedef struct NSSCKFWFindObjectsStr NSSCKFWFindObjects;

/*
 * NSSCKFWMutex
 *
 */

struct NSSCKFWMutexStr;
typedef struct NSSCKFWMutexStr NSSCKFWMutex;

typedef enum {
    SingleThreaded,
    MultiThreaded
} CryptokiLockingState;

/* used as an index into an array, make sure it starts at '0' */
typedef enum {
    NSSCKFWCryptoOperationState_EncryptDecrypt = 0,
    NSSCKFWCryptoOperationState_SignVerify,
    NSSCKFWCryptoOperationState_Digest,
    NSSCKFWCryptoOperationState_Max
} NSSCKFWCryptoOperationState;

typedef enum {
    NSSCKFWCryptoOperationType_Encrypt,
    NSSCKFWCryptoOperationType_Decrypt,
    NSSCKFWCryptoOperationType_Digest,
    NSSCKFWCryptoOperationType_Sign,
    NSSCKFWCryptoOperationType_Verify,
    NSSCKFWCryptoOperationType_SignRecover,
    NSSCKFWCryptoOperationType_VerifyRecover
} NSSCKFWCryptoOperationType;

#endif /* NSSCKFWT_H */
