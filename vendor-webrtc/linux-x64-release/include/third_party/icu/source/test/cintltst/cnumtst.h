// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2014, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CNUMTST.H
*
* Modification History:
*        Name                     Description            
*     Madhu Katragadda              Creation
*********************************************************************************
*/
/* C API TEST FOR NUMBER FORMAT */
#ifndef _CNUMFRMTST
#define _CNUMFRMTST

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "cintltst.h"


/**
 * The function used to test the Number format API
 **/
static void TestNumberFormat(void);

/**
 * The function used to test parsing of numbers in UNUM_SPELLOUT style
 **/
static void TestSpelloutNumberParse(void);

/**
 * The function used to test significant digits in the Number format API
 **/
static void TestSignificantDigits(void);

/**
 * The function used to test Number format API rounding with significant digits
 **/
static void TestSigDigRounding(void);

/**
 * The function used to test the Number format API with padding
 **/
static void TestNumberFormatPadding(void);

/**
 * The function used to test the Number format API with padding
 **/
static void TestInt64Format(void);

static void TestNonExistentCurrency(void);

/**
 * Test RBNF access through unumfmt APIs.
 **/
static void TestRBNFFormat(void);

/**
 * Test some Currency stuff
 **/
static void TestCurrencyRegression(void);

/**
 * Test strict parsing of "0"
 **/
static void TestParseZero(void);

/**
 * Test cloning formatter with RBNF
 **/
static void TestCloneWithRBNF(void);

/**
 * Test the Currency Usage Implementations
 **/
static void TestCurrencyUsage(void);
#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
