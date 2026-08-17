/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* Thse functions are stub functions which will get replaced with calls through
 * PKCS #11.
 */

#ifndef _PK11PQG_H_
#define _PK11PQG_H_ 1

#include "blapit.h"

SEC_BEGIN_PROTOS

/* Generate PQGParams and PQGVerify structs.
 * Length of seed and length of h both equal length of P.
 * All lengths are specified by "j", according to the table above.
 */
extern SECStatus PK11_PQG_ParamGen(unsigned int j, PQGParams **pParams,
                                   PQGVerify **pVfy);

/* Generate PQGParams and PQGVerify structs.
 * Length of P specified by j.  Length of h will match length of P.
 * Length of SEED in bytes specified in seedBytes.
 * seedBbytes must be in the range [20..255] or an error will result.
 */
extern SECStatus PK11_PQG_ParamGenSeedLen(unsigned int j,
                                          unsigned int seedBytes, PQGParams **pParams, PQGVerify **pVfy);

/* Generate PQGParams and PQGVerify structs.
 * Length of P specified by L.
 *   if L is greater than 1024 then the resulting verify parameters will be
 *   DSA2.
 * Length of Q specified by N. If zero, The PKCS #11 module will
 *   pick an appropriately sized Q for L. If N is specified and L = 1024, then
 *   the resulting verify parameters will be DSA2, Otherwise DSA1 parameters
 *   will be returned.
 * Length of SEED in bytes specified in seedBytes.
 *
 * The underlying PKCS #11 module will check the values for L, N,
 * and seedBytes. The rules for softoken are:
 *
 * If L <= 1024, then L must be between 512 and 1024 in increments of 64 bits.
 * If L <= 1024, then N must be 0 or 160.
 * If L >= 1024, then L and N must match the following table:
 *   L=1024   N=0 or 160
 *   L=2048   N=0 or 224
 *   L=2048   N=256
 *   L=3072   N=0 or 256
 * if L <= 1024
 *   seedBbytes must be in the range [20..256].
 * if L >= 1024
 *   seedBbytes must be in the range [20..L/16].
 */
extern SECStatus
PK11_PQG_ParamGenV2(unsigned int L, unsigned int N, unsigned int seedBytes,
                    PQGParams **pParams, PQGVerify **pVfy);

/*  Test PQGParams for validity as DSS PQG values.
 *  If vfy is non-NULL, test PQGParams to make sure they were generated
 *       using the specified seed, counter, and h values.
 *
 *  Return value indicates whether Verification operation ran successfully
 *  to completion, but does not indicate if PQGParams are valid or not.
 *  If return value is SECSuccess, then *pResult has these meanings:
 *       SECSuccess: PQGParams are valid.
 *       SECFailure: PQGParams are invalid.
 *
 * Verify the following 12 facts about PQG counter SEED g and h
 * These tests are specified in FIPS 186-3 Appendix A.1.1.1, A.1.1.3, and A.2.2
 * PQG_VerifyParams in softoken/freebl will automatically choose the
 * appropriate test.
 */
extern SECStatus PK11_PQG_VerifyParams(const PQGParams *params,
                                       const PQGVerify *vfy, SECStatus *result);
extern void PK11_PQG_DestroyParams(PQGParams *params);
extern void PK11_PQG_DestroyVerify(PQGVerify *vfy);

/**************************************************************************
 *  Return a pointer to a new PQGParams struct that is constructed from   *
 *  copies of the arguments passed in.                                    *
 *  Return NULL on failure.                                               *
 **************************************************************************/
extern PQGParams *PK11_PQG_NewParams(const SECItem *prime, const SECItem *subPrime, const SECItem *base);

/**************************************************************************
 * Fills in caller's "prime" SECItem with the prime value in params.
 * Contents can be freed by calling SECITEM_FreeItem(prime, PR_FALSE);
 **************************************************************************/
extern SECStatus PK11_PQG_GetPrimeFromParams(const PQGParams *params,
                                             SECItem *prime);

/**************************************************************************
 * Fills in caller's "subPrime" SECItem with the prime value in params.
 * Contents can be freed by calling SECITEM_FreeItem(subPrime, PR_FALSE);
 **************************************************************************/
extern SECStatus PK11_PQG_GetSubPrimeFromParams(const PQGParams *params,
                                                SECItem *subPrime);

/**************************************************************************
 * Fills in caller's "base" SECItem with the base value in params.
 * Contents can be freed by calling SECITEM_FreeItem(base, PR_FALSE);
 **************************************************************************/
extern SECStatus PK11_PQG_GetBaseFromParams(const PQGParams *params,
                                            SECItem *base);

/**************************************************************************
 *  Return a pointer to a new PQGVerify struct that is constructed from   *
 *  copies of the arguments passed in.                                    *
 *  Return NULL on failure.                                               *
 **************************************************************************/
extern PQGVerify *PK11_PQG_NewVerify(unsigned int counter,
                                     const SECItem *seed, const SECItem *h);

/**************************************************************************
 * Returns "counter" value from the PQGVerify.
 **************************************************************************/
extern unsigned int PK11_PQG_GetCounterFromVerify(const PQGVerify *verify);

/**************************************************************************
 * Fills in caller's "seed" SECItem with the seed value in verify.
 * Contents can be freed by calling SECITEM_FreeItem(seed, PR_FALSE);
 **************************************************************************/
extern SECStatus PK11_PQG_GetSeedFromVerify(const PQGVerify *verify,
                                            SECItem *seed);

/**************************************************************************
 * Fills in caller's "h" SECItem with the h value in verify.
 * Contents can be freed by calling SECITEM_FreeItem(h, PR_FALSE);
 **************************************************************************/
extern SECStatus PK11_PQG_GetHFromVerify(const PQGVerify *verify, SECItem *h);

SEC_END_PROTOS

#endif
