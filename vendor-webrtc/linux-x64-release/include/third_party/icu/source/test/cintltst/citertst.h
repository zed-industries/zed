// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT:
 * Copyright (c) 1997-2014, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CITERTST.H
*
* Modification History:
*        Name                     Description            
*     Madhu Katragadda            Converted to C
*********************************************************************************/

/**
 * Collation Iterator tests.
 * (Let me reiterate my position...)
 */

#ifndef _CITERCOLLTST
#define _CITERCOLLTST

#include "unicode/utypes.h"

#if !UCONFIG_NO_COLLATION

#include "unicode/ucol.h"
#include "unicode/ucoleitr.h"
#include "cintltst.h"

#define MAX_TOKEN_LEN 16

/**
* Test for CollationElementIterator previous and next for the whole set of
* unicode characters.
*/
static void TestUnicodeChar(void);
/**
* Test for CollationElementIterator previous and next for the whole set of
* unicode characters with normalization on.
*/
static void TestNormalizedUnicodeChar(void);
/**
* Test incremental normalization
*/
static void TestNormalization(void);
 /**
 * Test for CollationElementIterator.previous()
 *
 * @bug 4108758 - Make sure it works with contracting characters
 * 
 */
static void TestPrevious(void);

/**
 * Test for getOffset() and setOffset()
 */
static void TestOffset(void);
/**
 * Test for setText()
 */
static void TestSetText(void);
/** @bug 4108762
 * Test for getMaxExpansion()
 */
static void TestMaxExpansion(void);
/**
* Test Bug 672, where different locales give a different offset after 
* a previous for the same string at the same position
*/
static void TestBug672(void);

/**
 * Repeat TestBug672 with normalizatin enabled - this test revealed a bug
 *   in incremental normalization.
 */
static void TestBug672Normalize(void);
/**
* Test iterators with an relatively small buffer
*/
static void TestSmallBuffer(void);
/**
* Tests the discontiguous contractions
*/
static void TestDiscontiguos(void);
/**
* TestSearchCollatorElements tests iterator behavior (forwards and backwards) with
* normalization on AND jamo tailoring, among other things.
*/
static void TestSearchCollatorElements(void);

/*------------------------------------------------------------------------
 Internal utilities
 */


static void assertEqual(UCollationElements *i1, UCollationElements *i2);


#endif /* #if !UCONFIG_NO_COLLATION */

#endif
