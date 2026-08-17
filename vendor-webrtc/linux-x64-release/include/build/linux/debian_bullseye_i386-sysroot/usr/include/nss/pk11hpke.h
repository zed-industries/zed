/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _PK11_HPKE_H_
#define _PK11_HPKE_H_ 1

#include "blapit.h"
#include "seccomon.h"

#ifdef NSS_ENABLE_DRAFT_HPKE
#define HPKE_DRAFT_VERSION 5

#define CLEANUP                    \
    PORT_Assert(rv == SECSuccess); \
    cleanup

/* Error code must already be set.  */
#define CHECK_RV(rv)          \
    if ((rv) != SECSuccess) { \
        goto cleanup;         \
    }

/* Error code must already be set.  */
#define CHECK_FAIL(cond) \
    if ((cond)) {        \
        rv = SECFailure; \
        goto cleanup;    \
    }

#define CHECK_FAIL_ERR(cond, err) \
    if ((cond)) {                 \
        PORT_SetError((err));     \
        rv = SECFailure;          \
        goto cleanup;             \
    }

#endif /* NSS_ENABLE_DRAFT_HPKE */

typedef enum {
    HpkeModeBase = 0,
    HpkeModePsk = 1,
} HpkeModeId;

/* https://tools.ietf.org/html/draft-irtf-cfrg-hpke-05#section-7.1 */
typedef enum {
    HpkeDhKemX25519Sha256 = 0x20,
} HpkeKemId;

typedef enum {
    HpkeKdfHkdfSha256 = 1,
} HpkeKdfId;

typedef enum {
    HpkeAeadAes128Gcm = 1,
    HpkeAeadChaCha20Poly1305 = 3,
} HpkeAeadId;

typedef struct hpkeKemParamsStr {
    HpkeKemId id;
    unsigned int Nsk;
    unsigned int Nsecret;
    unsigned int Npk;
    SECOidTag oidTag;
    CK_MECHANISM_TYPE hashMech;
} hpkeKemParams;

typedef struct hpkeKdfParamsStr {
    HpkeKdfId id;
    unsigned int Nh;
    CK_MECHANISM_TYPE mech;
} hpkeKdfParams;

typedef struct hpkeAeadParamsStr {
    HpkeAeadId id;
    unsigned int Nk;
    unsigned int Nn;
    unsigned int tagLen;
    CK_MECHANISM_TYPE mech;
} hpkeAeadParams;

typedef struct HpkeContextStr HpkeContext;

#endif /* _PK11_HPKE_H_ */
