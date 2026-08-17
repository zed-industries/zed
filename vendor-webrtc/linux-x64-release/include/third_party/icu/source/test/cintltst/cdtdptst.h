// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2014, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CDTDPTST.H
*
* Modification History:
*        Name                     Description            
*     Madhu Katragadda               Creation
*********************************************************************************
*/
/* INDEPTH TEST FOR DATE FORMAT */
#ifndef _CDTFRRGSTST
#define _CDTFRRGSTST

#include "unicode/utypes.h"
#include "unicode/udat.h"

#if !UCONFIG_NO_FORMATTING

#include "cintltst.h"

/**
 * Test the parsing of 2-digit years.
 */
void TestTwoDigitYearDSTParse(void);
/**
 * Verify that strings which contain incomplete specifications are parsed
 * correctly.  In some instances, this means not being parsed at all, and
 * returning an appropriate error.
 */
void TestPartialParse994(void);
/**
 * Verify the behavior of patterns in which digits for different fields run together
 * without intervening separators.
 */
void TestRunTogetherPattern985(void);

/**
 * Verify the handling of Czech June and July, which have the unique attribute that
 * one is a proper prefix substring of the other.
 */
void TestCzechMonths459(void);

/**
 * Test the handling of single quotes in patterns.
 */
void TestQuotePattern161(void);

/*Internal functions used*/
void tryPat994(UDateFormat* format, const char* pat, const char* s, UDate expected);

/*
 * Testing udat_getBooleanAttribute and  unum_setBooleanAttribute()
 */
void TestBooleanAttributes(void);

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
