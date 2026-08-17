/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _CMSRECLIST_H
#define _CMSRECLIST_H

struct NSSCMSRecipientStr {
    int riIndex;  /* this recipient's index in recipientInfo array */
    int subIndex; /* index into recipientEncryptedKeys */
                  /* (only in NSSCMSKeyAgreeRecipientInfoStr) */
    enum { RLIssuerSN = 0,
           RLSubjKeyID = 1 } kind; /* for conversion recipientinfos -> recipientlist */
    union {
        CERTIssuerAndSN* issuerAndSN;
        SECItem* subjectKeyID;
    } id;

    /* result data (filled out for each recipient that's us) */
    CERTCertificate* cert;
    SECKEYPrivateKey* privkey;
    PK11SlotInfo* slot;
};

typedef struct NSSCMSRecipientStr NSSCMSRecipient;

#endif /* _CMSRECLIST_H */
