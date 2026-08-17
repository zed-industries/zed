/*
 ***********************************************************************
 * © 2016 and later: Unicode, Inc. and others.
 * License & terms of use: http://www.unicode.org/copyright.html
 ***********************************************************************
 ***********************************************************************
 * Copyright (c) 2011-2012,International Business Machines
 * Corporation and others.  All Rights Reserved.
 ***********************************************************************
 */

#ifndef SIEVE_H
#define SIEVE_H

#ifndef U_LOTS_OF_TIMES
#define U_LOTS_OF_TIMES 1000000
#endif

#include "unicode/utypes.h"
/**
 * Calculate the standardized sieve time (1 run)
 */
U_CAPI double uprv_calcSieveTime(void);

/**
 * Calculate the mean time, with margin of error
 * @param times array of times (modified/sorted)
 * @param timeCount length of array - on return, how many remain after throwing out outliers
 * @param marginOfError out parameter: gives +/- margin of err at 95% confidence
 * @return the mean time, or negative if error/imprecision.
 */
U_CAPI double uprv_getMeanTime(double *times, uint32_t *timeCount, double *marginOfError);

/**
 * Get the standardized sieve time. (Doesn't recalculate if already computed.
 * @param marginOfError out parameter: gives +/- margin of error at 95% confidence.
 * @return the mean time, or negative if error/imprecision.
 */
U_CAPI double uprv_getSieveTime(double *marginOfError);

#endif
