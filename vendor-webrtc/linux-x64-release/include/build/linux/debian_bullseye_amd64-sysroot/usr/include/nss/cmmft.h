/* -*- Mode: C; tab-width: 8 -*-*/
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _CMMFT_H_
#define _CMMFT_H_

#include "secasn1.h"

/*
 * These are the enumerations used to distinguish between the different
 * choices available for the CMMFCertOrEncCert structure.
 */
typedef enum {
    cmmfNoCertOrEncCert = 0,
    cmmfCertificate = 1,
    cmmfEncryptedCert = 2
} CMMFCertOrEncCertChoice;

/*
 * This is the enumeration and the corresponding values used to
 * represent the CMMF type PKIStatus
 */
typedef enum {
    cmmfNoPKIStatus = -1,
    cmmfGranted = 0,
    cmmfGrantedWithMods = 1,
    cmmfRejection = 2,
    cmmfWaiting = 3,
    cmmfRevocationWarning = 4,
    cmmfRevocationNotification = 5,
    cmmfKeyUpdateWarning = 6,
    cmmfNumPKIStatus
} CMMFPKIStatus;

/*
 * These enumerations are used to represent the corresponding values
 * in PKIFailureInfo defined in CMMF.
 */
typedef enum {
    cmmfBadAlg = 0,
    cmmfBadMessageCheck = 1,
    cmmfBadRequest = 2,
    cmmfBadTime = 3,
    cmmfBadCertId = 4,
    cmmfBadDataFormat = 5,
    cmmfWrongAuthority = 6,
    cmmfIncorrectData = 7,
    cmmfMissingTimeStamp = 8,
    cmmfNoFailureInfo = 9
} CMMFPKIFailureInfo;

typedef struct CMMFPKIStatusInfoStr CMMFPKIStatusInfo;
typedef struct CMMFCertOrEncCertStr CMMFCertOrEncCert;
typedef struct CMMFCertifiedKeyPairStr CMMFCertifiedKeyPair;
typedef struct CMMFCertResponseStr CMMFCertResponse;
typedef struct CMMFCertResponseSeqStr CMMFCertResponseSeq;
typedef struct CMMFPOPODecKeyChallContentStr CMMFPOPODecKeyChallContent;
typedef struct CMMFChallengeStr CMMFChallenge;
typedef struct CMMFRandStr CMMFRand;
typedef struct CMMFPOPODecKeyRespContentStr CMMFPOPODecKeyRespContent;
typedef struct CMMFKeyRecRepContentStr CMMFKeyRecRepContent;
typedef struct CMMFCertRepContentStr CMMFCertRepContent;

/* Export this so people can call SEC_ASN1EncodeItem instead of having to
 * write callbacks that are passed in to the high level encode function
 * for CMMFCertRepContent.
 */
extern const SEC_ASN1Template CMMFCertRepContentTemplate[];
extern const SEC_ASN1Template CMMFPOPODecKeyChallContentTemplate[];

#endif /*_CMMFT_H_*/
