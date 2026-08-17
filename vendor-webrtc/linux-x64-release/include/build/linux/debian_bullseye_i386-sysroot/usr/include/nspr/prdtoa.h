/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prdtoa_h___
#define prdtoa_h___

#include "prtypes.h"

PR_BEGIN_EXTERN_C

/*
** PR_strtod() returns as a double-precision floating-point number
** the  value represented by the character string pointed to by
** s00. The string is scanned up to the first unrecognized
** character.
**a
** If the value of se is not (char **)NULL, a  pointer  to
** the  character terminating the scan is returned in the location pointed
** to by se. If no number can be formed, se is set to s00, and
** zero is returned.
*/
NSPR_API(PRFloat64)
PR_strtod(const char *s00, char **se);

/*
** PR_cnvtf()
** conversion routines for floating point
** prcsn - number of digits of precision to generate floating
** point value.
*/
NSPR_API(void) PR_cnvtf(char *buf, PRIntn bufsz, PRIntn prcsn, PRFloat64 fval);

/*
** PR_dtoa() converts double to a string.
**
** ARGUMENTS:
** If rve is not null, *rve is set to point to the end of the return value.
** If d is +-Infinity or NaN, then *decpt is set to 9999.
**
** mode:
**     0 ==> shortest string that yields d when read in
**           and rounded to nearest.
*/
NSPR_API(PRStatus) PR_dtoa(PRFloat64 d, PRIntn mode, PRIntn ndigits,
                           PRIntn *decpt, PRIntn *sign, char **rve, char *buf, PRSize bufsize);

PR_END_EXTERN_C

#endif /* prdtoa_h___ */
