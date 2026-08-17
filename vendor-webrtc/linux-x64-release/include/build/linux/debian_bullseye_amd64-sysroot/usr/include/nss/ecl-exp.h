/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef __ecl_exp_h_
#define __ecl_exp_h_

/* Curve field type */
typedef enum {
    ECField_GFp,
    ECField_GF2m
} ECField;

/* Hexadecimal encoding of curve parameters */
struct ECCurveParamsStr {
    char *text;
    ECField field;
    unsigned int size;
    char *irr;
    char *curvea;
    char *curveb;
    char *genx;
    char *geny;
    char *order;
    int cofactor;
    int security;
    int pointSize;
    unsigned int usage;
};
typedef struct ECCurveParamsStr ECCurveParams;

/* Named curve parameters */
typedef enum {

    ECCurve_noName = 0,

    /* NIST prime curves */
    ECCurve_NIST_P192, /* not supported */
    ECCurve_NIST_P224, /* not supported */
    ECCurve_NIST_P256,
    ECCurve_NIST_P384,
    ECCurve_NIST_P521,

    /* NIST binary curves */
    ECCurve_NIST_K163, /* not supported */
    ECCurve_NIST_B163, /* not supported */
    ECCurve_NIST_K233, /* not supported */
    ECCurve_NIST_B233, /* not supported */
    ECCurve_NIST_K283, /* not supported */
    ECCurve_NIST_B283, /* not supported */
    ECCurve_NIST_K409, /* not supported */
    ECCurve_NIST_B409, /* not supported */
    ECCurve_NIST_K571, /* not supported */
    ECCurve_NIST_B571, /* not supported */

    /* ANSI X9.62 prime curves */
    /* ECCurve_X9_62_PRIME_192V1 == ECCurve_NIST_P192 */
    ECCurve_X9_62_PRIME_192V2, /* not supported */
    ECCurve_X9_62_PRIME_192V3, /* not supported */
    ECCurve_X9_62_PRIME_239V1, /* not supported */
    ECCurve_X9_62_PRIME_239V2, /* not supported */
    ECCurve_X9_62_PRIME_239V3, /* not supported */
    /* ECCurve_X9_62_PRIME_256V1 == ECCurve_NIST_P256 */

    /* ANSI X9.62 binary curves */
    ECCurve_X9_62_CHAR2_PNB163V1, /* not supported */
    ECCurve_X9_62_CHAR2_PNB163V2, /* not supported */
    ECCurve_X9_62_CHAR2_PNB163V3, /* not supported */
    ECCurve_X9_62_CHAR2_PNB176V1, /* not supported */
    ECCurve_X9_62_CHAR2_TNB191V1, /* not supported */
    ECCurve_X9_62_CHAR2_TNB191V2, /* not supported */
    ECCurve_X9_62_CHAR2_TNB191V3, /* not supported */
    ECCurve_X9_62_CHAR2_PNB208W1, /* not supported */
    ECCurve_X9_62_CHAR2_TNB239V1, /* not supported */
    ECCurve_X9_62_CHAR2_TNB239V2, /* not supported */
    ECCurve_X9_62_CHAR2_TNB239V3, /* not supported */
    ECCurve_X9_62_CHAR2_PNB272W1, /* not supported */
    ECCurve_X9_62_CHAR2_PNB304W1, /* not supported */
    ECCurve_X9_62_CHAR2_TNB359V1, /* not supported */
    ECCurve_X9_62_CHAR2_PNB368W1, /* not supported */
    ECCurve_X9_62_CHAR2_TNB431R1, /* not supported */

    /* SEC2 prime curves */
    ECCurve_SECG_PRIME_112R1, /* not supported */
    ECCurve_SECG_PRIME_112R2, /* not supported */
    ECCurve_SECG_PRIME_128R1, /* not supported */
    ECCurve_SECG_PRIME_128R2, /* not supported */
    ECCurve_SECG_PRIME_160K1, /* not supported */
    ECCurve_SECG_PRIME_160R1, /* not supported */
    ECCurve_SECG_PRIME_160R2, /* not supported */
    ECCurve_SECG_PRIME_192K1, /* not supported */
    /* ECCurve_SECG_PRIME_192R1 == ECCurve_NIST_P192 */
    ECCurve_SECG_PRIME_224K1, /* not supported */
    /* ECCurve_SECG_PRIME_224R1 == ECCurve_NIST_P224 */
    ECCurve_SECG_PRIME_256K1, /* not supported */
    /* ECCurve_SECG_PRIME_256R1 == ECCurve_NIST_P256 */
    /* ECCurve_SECG_PRIME_384R1 == ECCurve_NIST_P384 */
    /* ECCurve_SECG_PRIME_521R1 == ECCurve_NIST_P521 */

    /* SEC2 binary curves */
    ECCurve_SECG_CHAR2_113R1, /* not supported */
    ECCurve_SECG_CHAR2_113R2, /* not supported */
    ECCurve_SECG_CHAR2_131R1, /* not supported */
    ECCurve_SECG_CHAR2_131R2, /* not supported */
    /* ECCurve_SECG_CHAR2_163K1 == ECCurve_NIST_K163 */
    ECCurve_SECG_CHAR2_163R1, /* not supported */
    /* ECCurve_SECG_CHAR2_163R2 == ECCurve_NIST_B163 */
    ECCurve_SECG_CHAR2_193R1, /* not supported */
    ECCurve_SECG_CHAR2_193R2, /* not supported */
    /* ECCurve_SECG_CHAR2_233K1 == ECCurve_NIST_K233 */
    /* ECCurve_SECG_CHAR2_233R1 == ECCurve_NIST_B233 */
    ECCurve_SECG_CHAR2_239K1, /* not supported */
    /* ECCurve_SECG_CHAR2_283K1 == ECCurve_NIST_K283 */
    /* ECCurve_SECG_CHAR2_283R1 == ECCurve_NIST_B283 */
    /* ECCurve_SECG_CHAR2_409K1 == ECCurve_NIST_K409 */
    /* ECCurve_SECG_CHAR2_409R1 == ECCurve_NIST_B409 */
    /* ECCurve_SECG_CHAR2_571K1 == ECCurve_NIST_K571 */
    /* ECCurve_SECG_CHAR2_571R1 == ECCurve_NIST_B571 */

    /* WTLS curves */
    ECCurve_WTLS_1, /* not supported */
    /* there is no WTLS 2 curve */
    /* ECCurve_WTLS_3 == ECCurve_NIST_K163 */
    /* ECCurve_WTLS_4 == ECCurve_SECG_CHAR2_113R1 */
    /* ECCurve_WTLS_5 == ECCurve_X9_62_CHAR2_PNB163V1 */
    /* ECCurve_WTLS_6 == ECCurve_SECG_PRIME_112R1 */
    /* ECCurve_WTLS_7 == ECCurve_SECG_PRIME_160R1 */
    ECCurve_WTLS_8, /* not supported */
    ECCurve_WTLS_9, /* not supported */
    /* ECCurve_WTLS_10 == ECCurve_NIST_K233 */
    /* ECCurve_WTLS_11 == ECCurve_NIST_B233 */
    /* ECCurve_WTLS_12 == ECCurve_NIST_P224 */

    ECCurve25519,

    ECCurve_pastLastCurve
} ECCurveName;

/* Aliased named curves */

#define ECCurve_X9_62_PRIME_192V1 ECCurve_NIST_P192 /* not supported */
#define ECCurve_X9_62_PRIME_256V1 ECCurve_NIST_P256
#define ECCurve_SECG_PRIME_192R1 ECCurve_NIST_P192 /* not supported */
#define ECCurve_SECG_PRIME_224R1 ECCurve_NIST_P224 /* not supported */
#define ECCurve_SECG_PRIME_256R1 ECCurve_NIST_P256
#define ECCurve_SECG_PRIME_384R1 ECCurve_NIST_P384
#define ECCurve_SECG_PRIME_521R1 ECCurve_NIST_P521
#define ECCurve_SECG_CHAR2_163K1 ECCurve_NIST_K163  /* not supported */
#define ECCurve_SECG_CHAR2_163R2 ECCurve_NIST_B163  /* not supported */
#define ECCurve_SECG_CHAR2_233K1 ECCurve_NIST_K233  /* not supported */
#define ECCurve_SECG_CHAR2_233R1 ECCurve_NIST_B233  /* not supported */
#define ECCurve_SECG_CHAR2_283K1 ECCurve_NIST_K283  /* not supported */
#define ECCurve_SECG_CHAR2_283R1 ECCurve_NIST_B283  /* not supported */
#define ECCurve_SECG_CHAR2_409K1 ECCurve_NIST_K409  /* not supported */
#define ECCurve_SECG_CHAR2_409R1 ECCurve_NIST_B409  /* not supported */
#define ECCurve_SECG_CHAR2_571K1 ECCurve_NIST_K571  /* not supported */
#define ECCurve_SECG_CHAR2_571R1 ECCurve_NIST_B571  /* not supported */
#define ECCurve_WTLS_3 ECCurve_NIST_K163            /* not supported */
#define ECCurve_WTLS_4 ECCurve_SECG_CHAR2_113R1     /* not supported */
#define ECCurve_WTLS_5 ECCurve_X9_62_CHAR2_PNB163V1 /* not supported */
#define ECCurve_WTLS_6 ECCurve_SECG_PRIME_112R1     /* not supported */
#define ECCurve_WTLS_7 ECCurve_SECG_PRIME_160R1     /* not supported */
#define ECCurve_WTLS_10 ECCurve_NIST_K233           /* not supported */
#define ECCurve_WTLS_11 ECCurve_NIST_B233           /* not supported */
#define ECCurve_WTLS_12 ECCurve_NIST_P224           /* not supported */

#endif /* __ecl_exp_h_ */
