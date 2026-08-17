/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#ifndef _P12PLCY_H_
#define _P12PLCY_H_

#include "secoid.h"
#include "ciferfam.h"

SEC_BEGIN_PROTOS

/* for the algid specified, can we decrypt it ? */
extern PRBool SEC_PKCS12DecryptionAllowed(SECAlgorithmID *algid);

/* is encryption allowed? */
extern PRBool SEC_PKCS12IsEncryptionAllowed(void);

/* enable a cipher for encryption/decryption */
extern SECStatus SEC_PKCS12EnableCipher(long which, int on);

/* return the preferred cipher for encryption */
extern SECStatus SEC_PKCS12SetPreferredCipher(long which, int on);

SEC_END_PROTOS
#endif
