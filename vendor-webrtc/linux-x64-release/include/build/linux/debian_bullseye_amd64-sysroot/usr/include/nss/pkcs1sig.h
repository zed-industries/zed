/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#ifndef _PKCS1SIG_H_
#define _PKCS1SIG_H_

#include "hasht.h"
#include "seccomon.h"
#include "secoidt.h"

/* SGN_VerifyPKCS1DigestInfo verifies that the length of the digest is correct
 * for the given algorithm, then verifies that the recovered data from the
 * PKCS#1 signature is a properly-formatted DigestInfo that identifies the
 * given digest algorithm, then verifies that the digest in the DigestInfo
 * matches the given digest.
 *
 * dataRecoveredFromSignature must be the result of calling PK11_VerifyRecover
 * or equivalent.
 *
 * If unsafeAllowMissingParameters is true (not recommended), then a DigestInfo
 * without the mandatory ASN.1 NULL parameter will also be accepted.
 */
SECStatus _SGN_VerifyPKCS1DigestInfo(SECOidTag digestAlg,
                                     const SECItem* digest,
                                     const SECItem* dataRecoveredFromSignature,
                                     PRBool unsafeAllowMissingParameters);

#endif /* _PKCS1SIG_H_ */
